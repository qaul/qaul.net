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
import net.qaul.ble.test.ble.queue.ReceiveQueue
import net.qaul.ble.test.ble.queue.SendQueue
import net.qaul.ble.test.ble.util.toHexString
import java.io.IOException
import java.util.UUID

class BleConnection(
    val device: BluetoothDevice,
    val role: BleRole
) {
    /** Populated after the central reads READ_CHAR during connection setup. */
    var remoteQaulId: ByteArray? = null

    /**
     * Fired the first time we learn the remote's qaul ID from the data stream (SEND_ID FLC).
     * ConnectionPool sets this to detect duplicates across MAC addresses.
     */
    var onQaulIdResolved: ((device: BluetoothDevice, qaulId: ByteArray) -> Unit)? = null

    private val TAG = "BleConnection"

    private val sendQueue = SendQueue(BleConstants.LOCAL_QAUL_ID)
    private val receiveQueue = ReceiveQueue()

    // High-bandwidth L2CAP data channel. Owned for BOTH roles: the CENTRAL opens it after
    // reading the peripheral's PSM; the PERIPHERAL has its end handed in by GattServer.
    // All access is guarded by l2capLock. `closed` prevents a late L2CAP connect — one that
    // completes on its background thread AFTER disconnect() ran (e.g. the dual-connection
    // tiebreaker dropped us mid-connect) — from orphaning a channel and leaking its threads.
    private val l2capLock = Any()
    @Volatile private var closed = false

    @Volatile var lastActivityAt = System.currentTimeMillis()
        private set
    fun updateActivity() { lastActivityAt = System.currentTimeMillis() }
    private var l2capChannel: L2capChannel? = null

    fun connect() {
        when (role) {
            BleRole.CENTRAL -> {
                BleTaskScheduler.connect(device)
                BleTaskScheduler.discoverServices(device)
                BleTaskScheduler.requestMtu(device, BleConstants.TARGET_MTU)
                BleTaskScheduler.readCharacteristic(device, BleConstants.READ_CHAR) // Gets qaul id
                BleTaskScheduler.readCharacteristic(device, BleConstants.PSM_CHAR)  // Gets L2CAP PSM
                BleTaskScheduler.enableNotifications(device, BleConstants.MSG_CHAR)
                BleTaskScheduler.setPreferredPhy(
                    device,
                    BluetoothDevice.PHY_LE_2M_MASK,
                    BluetoothDevice.PHY_LE_2M_MASK,
                    BluetoothDevice.PHY_OPTION_NO_PREFERRED
                )
//                BleTaskScheduler.setPreferredPhy(
//                    device,
//                    BluetoothDevice.PHY_LE_CODED_MASK,
//                    BluetoothDevice.PHY_LE_CODED_MASK,
//                    BluetoothDevice.PHY_OPTION_S8
//                )
                // Request a tighter connection interval for faster throughput
                // no callback, runs after all setup steps are queued
                BleTaskScheduler.requestConnectionPriority(device, BluetoothGatt.CONNECTION_PRIORITY_HIGH)
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
                onClosed = { synchronized(l2capLock) { if (l2capChannel === newChannel) l2capChannel = null } }
            )
            l2capChannel = newChannel
            Log.i(TAG, "L2CAP channel ready for ${device.address} ($role)")
        }
    }

    /**
     * Send a message. Transport is chosen automatically: if the L2CAP channel is up, use it
     * otherwise fall back to the GATT chunk-queue path. Callers don't need to know which is used.
     */
    fun sendMessage(payload: ByteArray) {
        val channel = synchronized(l2capLock) { l2capChannel }
        if (channel != null) {
            channel.send(payload)
        } else {
            val messageId = UUID.randomUUID().toString()
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

        // Remote is acknowledging receipt of a message we sent
        result.flcAckReceived?.let { ack ->
            sendQueue.flcAckReceived(ack.messageIndex, ack.success, ack.errorCode)
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
     * Pulls all pending chunks from SendQueue (FLC priority first, then data)
     * and enqueues them to BleTaskScheduler.
     */
    private fun flushSendQueue() {
        val (chunks, _, _) = sendQueue.getChunks()
        when (role) {
            BleRole.CENTRAL -> chunks.forEach {
                BleTaskScheduler.writeCharacteristic(device, BleConstants.MSG_CHAR, it)
            }
            BleRole.PERIPHERAL -> chunks.forEach {
                BleTaskScheduler.notifyCharacteristicChanged(device, BleConstants.MSG_CHAR, false, it)
            }
        }
    }

}