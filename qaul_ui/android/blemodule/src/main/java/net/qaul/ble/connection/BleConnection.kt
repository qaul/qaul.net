package net.qaul.ble.test.ble.connection

import android.annotation.SuppressLint
import android.bluetooth.BluetoothDevice
import android.bluetooth.BluetoothGatt
import android.bluetooth.BluetoothSocket
import android.os.Build
import android.util.Log
import net.qaul.ble.BleConstants
import net.qaul.ble.test.ble.l2cap.L2capChannel
import net.qaul.ble.test.ble.metrics.BleMetrics
import net.qaul.ble.test.ble.queue.BleTaskScheduler
import net.qaul.ble.test.ble.queue.NeighbourUpdate
import net.qaul.ble.test.ble.queue.OpLane
import net.qaul.ble.test.ble.queue.ReceiveQueue
import net.qaul.ble.test.ble.queue.SendQueue
import net.qaul.ble.test.ble.util.toHexString
import java.io.IOException
import java.util.UUID

class BleConnection(
    val device: BluetoothDevice,
    val role: BleRole,
    private val connectPhy: Int = BluetoothDevice.PHY_LE_1M_MASK
) {
    /** Populated after the central reads READ_CHAR during connection setup. */
    var remoteQaulId: ByteArray? = null

    /** Negotiated link PHY label ("1M" / "2M" / "Coded"), updated from onPhyUpdate. "?" until the PHY
     *  update completes. For the debug overlay so we can see which links are long-range Coded. */
    @Volatile var phyLabel: String = "?"

    /** Live connection RSSI (dBm), refreshed periodically via readRemoteRssi. CENTRAL connections only can read
     * Field-test telemetry. */
    @Volatile var rssi: Int? = null

    val createdAt: Long = System.currentTimeMillis()

    /**
     * Is this link on the long-range Coded PHY? Drives per connection timeout scaling.
     *
     * Uses the negotiated [phyLabel] once known, falling back to the PHY we opened on ,which is
     * what covers the fragile setup window of an outbound Coded connect, before any onPhyUpdate has
     * landed. Problem?: an peripheral Coded link can't be identified until its PHY update
     * arrives
     */
    val isCoded: Boolean
        get() = phyLabel == "Coded" ||
                (phyLabel == "?" && connectPhy == BluetoothDevice.PHY_LE_CODED_MASK)

    /** [base] scaled for this link's PHY  */
    fun scaleTimeout(base: Long): Long =
        if (isCoded) base * BleConstants.CODED_TIMEOUT_MULTIPLIER else base

    /**
     * Fired the first time we learn the remote's qaul ID from the data stream (SEND_ID FLC).
     * ConnectionPool sets this to detect duplicates across MAC addresses.
     */
    var onQaulIdResolved: ((device: BluetoothDevice, qaulId: ByteArray) -> Unit)? = null

    /**
     * Fired when a sent message reaches an outcome, the remote's FLC ACK arrived or, an error.
     *.[messageId] is libqaul's message_id, carried through as a
     * hex key. [success] is the remote's ACK result. Forwardeding this up to libqaul as a
     * BleDirectSendResult.
     */
    var onMessageResult: ((messageId: String, success: Boolean) -> Unit)? = null

    /** Fired when the remote sends its neighbour list (SEND_NEIGHBOURS FLC). Each entry is a
     *  QAUL_ID_ADVERT_BYTES prefix. ConnectionPool uses it for 2 hop topology awareness. */
    var onNeighboursReceived: ((device: BluetoothDevice, neighbourUpdate: NeighbourUpdate) -> Unit)? = null

    private val TAG = "BleConnection"

    private val sendQueue = SendQueue(BleConstants.LOCAL_QAUL_ID)
    private val receiveQueue = ReceiveQueue()

    // High-bandwidth L2CAP data channel. Owned for BOTH roles: the CENTRAL opens it after
    // reading the peripheral's PSM, the PERIPHERAL has its end handed in by GattServer.
    // All access is guarded by l2capLock. `closed` prevents a late L2CAP connect, one that
    // completes on its background thread after disconnect() ran, from orphaning a channel and leaking its threads.
    private val l2capLock = Any()
    @Volatile private var closed = false

    @Volatile var lastActivityAt = System.currentTimeMillis()
        private set
    fun updateActivity() { lastActivityAt = System.currentTimeMillis() }
    private var l2capChannel: L2capChannel? = null

    fun connect() {
        when (role) {
            BleRole.CENTRAL -> {
                BleTaskScheduler.connect(device, connectPhy)   // open on the discovered PHY (1M or Coded)
                // Connection interval. High (7.5-15ms) on every link saturates the controller radio schedule once several links are up
                // Balanced (~30-50ms) is the idle default, allows for at least 4 connections. We could raise to high while a bulk transfer is in flight.
                BleTaskScheduler.requestConnectionPriority(device, BleConstants.IDLE_CONNECTION_PRIORITY)
                // Order matters: the scheduler is serial, do identity first, so both ends
                // resolve as early as possible, READ_CHAR gives us the peer's qaul ID, and notifications open the path the peer needs to receive ours.
                BleTaskScheduler.discoverServices(device)
                BleTaskScheduler.enableNotifications(device, BleConstants.MSG_CHAR)
                BleTaskScheduler.readCharacteristic(device, BleConstants.READ_CHAR) // Gets qaul id
                BleTaskScheduler.requestMtu(device, BleConstants.TARGET_MTU)
                BleTaskScheduler.readCharacteristic(device, BleConstants.PSM_CHAR)  // Gets L2CAP PSM
                // Keep the connection on the PHY we opened it on. This is a request, the controller negotiates down if unsupported and
                // onPhyUpdate logs what is actually agreed
                if (connectPhy == BluetoothDevice.PHY_LE_CODED_MASK) {
                    BleTaskScheduler.setPreferredPhy(
                        device,
                        BluetoothDevice.PHY_LE_CODED_MASK,
                        BluetoothDevice.PHY_LE_CODED_MASK,
                        BluetoothDevice.PHY_OPTION_S8
                    )
                } else {
                    val phy = if (BleConstants.PREFER_2M) BluetoothDevice.PHY_LE_2M_MASK
                              else BluetoothDevice.PHY_LE_1M_MASK
                    BleTaskScheduler.setPreferredPhy(
                        device,
                        phy,
                        phy,
                        BluetoothDevice.PHY_OPTION_NO_PREFERRED
                    )
                }
            }
            BleRole.PERIPHERAL -> {
                // If peripheral then we are connected TO so nothing should happen here
            }
        }

    }

    fun disconnect() {
        synchronized(l2capLock) {
            closed = true
            l2capChannel?.close()
            l2capChannel = null
        }
        when (role) {
            BleRole.CENTRAL -> BleTaskScheduler.disconnect(device)
            BleRole.PERIPHERAL -> {
                // If peripheral then we cant disconnect so nothing should happen here
            }
        }
    }

    /**
     * CENTRAL: open the L2CAP channel to the peripheral using the PSM read from PSM_CHAR.
     * connect() is blocking, so it runs on a background thread; on success the channel is
     * installed via setupChannel (which discards it if we were disconnected in the meantime).
     */
    @SuppressLint("MissingPermission")
    fun connectL2cap(psm: Int) {
        if (role != BleRole.CENTRAL) return
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.Q) {
            Log.w(TAG, "L2CAP requires API 29+; staying on GATT for ${device.address}")
            return
        }
        if (psm <= 0) {
            Log.w(TAG, "Peripheral ${device.address} reported no L2CAP (PSM=$psm); using GATT")
            return
        }
        Thread({
            try {
                val socket = device.createInsecureL2capChannel(psm)
                socket.connect()                              // blocking
                Log.i(TAG, "L2CAP connected to ${device.address} (PSM=$psm)")
                setupChannel(socket)
            } catch (e: IOException) {
                Log.e(TAG, "L2CAP connect failed to ${device.address}: ${e.message} — staying on GATT")
            }
        }, "l2cap-connect-${device.address}").start()
    }

    /**
     * PERIPHERAL: GattServer hands us our end of an accepted L2CAP channel.
     */
    fun attachL2capSocket(socket: BluetoothSocket) {
        Log.i(TAG, "L2CAP socket accepted for ${device.address}")
        setupChannel(socket)
    }

    /**
     * Install a connected L2CAP socket as this connection's channel (both roles funnel here).
     * Guarded by l2capLock and the `closed` flag so a socket that finishes connecting/arriving
     * after disconnect() is discarded rather than leaked. Received messages are delivered through
     * the same path as assembled GATT messages, so callers can't tell which transport was used.
     */
    private fun setupChannel(socket: BluetoothSocket) {
        synchronized(l2capLock) {
            if (closed) {
                // Lost the race against disconnect() — discard rather than orphan the channel.
                try { socket.close() } catch (_: IOException) {}
                Log.w(TAG, "L2CAP socket arrived after disconnect for ${device.address}; discarded")
                return
            }
            l2capChannel?.close()
            lateinit var newChannel: L2capChannel
            newChannel = L2capChannel(
                socket = socket,
                label = "${role.name.lowercase()}-${device.address}",
                onMessageReceived = { data -> BleTaskScheduler.notifyMessageAssembled(device, data) },
                // Only clear if the field still points at THIS channel (a replacement may have
                // already taken its place).
                onClosed = { synchronized(l2capLock) { if (l2capChannel === newChannel) l2capChannel = null } },
                onBulkReceive = { active -> BleTaskScheduler.notifyBulkTransport(device, active) }
            )
            l2capChannel = newChannel
            Log.i(TAG, "L2CAP channel ready for ${device.address} ($role)")
        }
    }

    /**
     * Send a message. Transport is chosen automatically: if the L2CAP channel is up, use it
     * otherwise fall back to the GATT chunk-queue path. Callers don't need to know which is used.
     */
    fun sendMessage(payload: ByteArray, messageId: String = UUID.randomUUID().toString()) {
        val channel = synchronized(l2capLock) { l2capChannel }
        // large messages take the L2CAP bulk pipe, short messages take the GATT path so they keep the medium lane priority
        // and real FLC ACK. They ride separate channels, so a file can't block a routing
        // update. If there's no L2CAP channel, everything goes GATT and large lands in the BULK lane.
        if (channel != null && payload.size > BleConstants.MEDIUM_MESSAGE_MAX_BYTES) {
            // Escalation is signalled from here, not from the scheduler's BULK lane. This branch is
            // the only place that knows both "this is bulk" and "it took L2CAP" — the lane-based
            // signal is GATT-only, so before this every transfer big enough to use L2CAP ran its
            // full length at idle priority (45ms interval) on 1M, which is the worst configuration
            // available and exactly the one real files were getting.
            BleTaskScheduler.notifyBulkTransport(device, true)
            channel.send(payload) { BleTaskScheduler.notifyBulkTransport(device, false) }
            // TODO: L2CAP has no per-message ACK. is this ok?. the socket write is the delivery signal for now.
            onMessageResult?.invoke(messageId, true)
        } else {
            sendQueue.addMessage(payload, messageId)
            flushSendQueue()
        }
    }

    /**
     * Called by BleManager when a raw chunk arrives for this device (from either role).
     * Routes through ReceiveQueue for reassembly, then handles the result:
     * assembled messages are delivered to listeners, FLC responses are fed back to SendQueue.
     */
    fun onChunkReceived(chunk: ByteArray) {
        updateActivity()
        val result = receiveQueue.incomingMessage(chunk, device)

        // Remote's qaul ID resolved from the data stream (SEND_ID FLC). For PERIPHERAL
        // connections this is the only way to learn the remote qaul ID. Fire onQaulIdResolved once so ConnectionPool can dedup dual
        // connections via the qaul-ID tiebreaker.
        result.qaulIdReceived?.let { id ->
            if (remoteQaulId == null) {
                remoteQaulId = id
                Log.i(TAG, "Qaul ID resolved from data stream for ${device.address}: ${id.toHexString()}")
                onQaulIdResolved?.invoke(device, id)
            }
        }

        // Receive side half of the bulk transfer PHY/interval escalation signal, how a receiver knows we should upgrade the link due to an incoming bulk send
        result.bulkReceiveActive?.let { BleTaskScheduler.notifyBulkReceive(device, it) }

        // Fully assembled message — deliver upward
        result.receivedMessage?.let {
            Log.i(TAG, "Message assembled from ${device.address}: ${it.message.size} bytes")
            BleMetrics.onMessageAssembled(it.message.size, it.createdAt)
            BleTaskScheduler.notifyMessageAssembled(device, it.message)
        }

        // Receiver needs us to send an ACK back
        result.flcSendAck?.let { ack ->
            sendQueue.addFlcAck(ack.messageIndex, ack.success, ack.errorCode)
            flushSendQueue()
        }

        // remote shared its neighbour list, hand it up for 2 hop topology tracking.
        result.neighboursReceived?.let { onNeighboursReceived?.invoke(device, it) }

        // Remote is acknowledging receipt of a message we sent. Returns the completed
        // message's id (empty until a message, or all parts of a large one. is fully acked), which we
        // surface up as the real delivery result
        result.flcAckReceived?.let { ack ->
            val messageId = sendQueue.flcAckReceived(ack.messageIndex, ack.success, ack.errorCode)
            if (messageId.isNotEmpty()) onMessageResult?.invoke(messageId, ack.success)
            flushSendQueue() // Advance to the next queued message now that this one is ACK'd
        }

        // We are missing chunks — request them from the sender
        if (result.flcRequestChunks.isNotEmpty()) {
            result.flcRequestChunks.forEach { sendQueue.addMissingChunkIndexToRequest(it) }
            flushSendQueue()
        }

        // Remote requested chunks we need to resend
        if (result.flcChunksRequested.isNotEmpty()) {
            result.flcChunksRequested.forEach { sendQueue.addMissingChunkIndexToSend(it) }
            flushSendQueue()
        }
    }

    /**
     * Called when the central finishes discovering the peripheral's services (MSG_CHAR is now
     * available to write). Push our qaulId (SEND_ID) over GATT here, earlier and more reliable
     * than [onMtuNegotiated], which doesnt seem to always fire so we backup here. The qaulIdSent guard
     * in SendQueue makes the later MTU-time flush harmless if we already succeeded.
     */
    fun onServicesDiscovered() {
        if (role == BleRole.CENTRAL) {
            //a SEND_ID written before discovery finished fails with "characteristic not found" and is gone. Re-arm before flushing.
            markTransportReady()
            sendQueue.resendQaulId()
            flushSendQueue()
        }
    }

    /**
     * True once this link can actually carry a SEND_ID: services discovered (CENTRAL) or the remote
     * subscribed to MSG_CHAR (PERIPHERAL). Before that, sends are dropped by the transport, so
     * [resendIdentity] waits.
     */
    @Volatile private var transportReady = false

    /** When identity exchange became possible (services discovered, or the remote subscribed), or
     *  null while the connect is still in flight. The unresolved reaper measures from here.*/
    @Volatile var handshakeStartedAt: Long? = null
        private set

    private fun markTransportReady() {
        transportReady = true
        if (handshakeStartedAt == null) handshakeStartedAt = System.currentTimeMillis()
    }

    /**
     * Re-send our qaul ID if this link still hasn't resolved the remote's. Retried periodically
     * until resolved or the unresolved reaper drops the connection.
     */
    fun resendIdentity() {
        if (!transportReady || remoteQaulId != null) return
        sendQueue.resendQaulId()
        flushSendQueue()
    }

    /**
     * The remote enabled notifications on MSG_CHAR so our PERIPHERAL side notify path only
     * becomes usable now. Re-flush so the identity exchange actually completes.
     */
    fun onRemoteSubscribed() {
        markTransportReady()
        sendQueue.resendQaulId()
        flushSendQueue()
    }

    /**
     * Called when MTU is negotiated. Updates chunk size so SendQueue
     * splits messages correctly for this connection.
     */
    fun onMtuNegotiated(mtu: Int) {
        // TODO: Look into the cap at 495 bytes, the optimal ATT payload for exactly 2 DLE LL packets, does this matter?
        sendQueue.chunkSize = minOf(mtu - 3, 509)
        Log.i(TAG, "Chunk size updated to ${sendQueue.chunkSize} for ${device.address}")
        if (role == BleRole.CENTRAL){
            flushSendQueue()
        }
    }


   /**
    * Connection is being torn down, fail every message still in flight to this peer so libqaul gets
    * a real failure result.
    */
    fun failPendingMessages() {
        sendQueue.failAllPending().forEach { messageId ->
            onMessageResult?.invoke(messageId, false)
        }
    }

   /**
    * send our current neighbour list to this peer (SEND_NEIGHBOURS FLC) for 2 hop awareness.
    */
    fun sendNeighbourList(originId: ByteArray,
                          seq: Int,
                          ttl: Int, sealed: Boolean, prefixes: List<ByteArray>) {
        sendQueue.addFlcNeighbours(originId, seq, ttl, sealed, prefixes)
        flushSendQueue()
    }

   /**
    *  Queues and sends an FLC ping for this connection, used for maintaining liveness.
    */
    fun sendPing() {
        sendQueue.addFlcPing()
        flushSendQueue()
    }

    /**
     * Pulls all pending chunks from SendQueue and enqueues them to BleTaskScheduler on their lane:
     * FLC control first, then short-message payload (MEDIUM), then large-message payload (BULK), so a
     * file transfer can't stall routing/chat traffic or connection setup.
     */
    private fun flushSendQueue() {
        val batch = sendQueue.getChunks()
        when (role) {
            BleRole.CENTRAL -> {
                batch.flcChunks.forEach {
                    BleTaskScheduler.writeCharacteristic(device, BleConstants.MSG_CHAR, it, lane = OpLane.CONTROL)
                }
                batch.mediumChunks.forEach {
                    BleTaskScheduler.writeCharacteristic(device, BleConstants.MSG_CHAR, it, lane = OpLane.MEDIUM)
                }
                batch.bulkChunks.forEach {
                    BleTaskScheduler.writeCharacteristic(device, BleConstants.MSG_CHAR, it, lane = OpLane.BULK)
                }
            }
            BleRole.PERIPHERAL -> {
                batch.flcChunks.forEach {
                    BleTaskScheduler.notifyCharacteristicChanged(device, BleConstants.MSG_CHAR, false, it, lane = OpLane.CONTROL)
                }
                batch.mediumChunks.forEach {
                    BleTaskScheduler.notifyCharacteristicChanged(device, BleConstants.MSG_CHAR, false, it, lane = OpLane.MEDIUM)
                }
                batch.bulkChunks.forEach {
                    BleTaskScheduler.notifyCharacteristicChanged(device, BleConstants.MSG_CHAR, false, it, lane = OpLane.BULK)
                }
            }
        }
    }

}