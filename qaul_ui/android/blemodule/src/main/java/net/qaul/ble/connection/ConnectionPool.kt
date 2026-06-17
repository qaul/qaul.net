package net.qaul.ble.test.ble.connection

import android.bluetooth.BluetoothDevice
import android.bluetooth.BluetoothGattCharacteristic
import android.util.Log
import net.qaul.ble.BleConstants
import net.qaul.ble.test.ble.manager.ConnectionEventListener
import net.qaul.ble.test.ble.queue.BleTaskScheduler
import net.qaul.ble.test.ble.util.toHexString
import java.util.concurrent.ConcurrentHashMap
import java.util.concurrent.Executors
import java.util.concurrent.ScheduledFuture
import java.util.concurrent.TimeUnit

object ConnectionPool {
    private const val TAG = "ConnectionPool"
    private val connections = ConcurrentHashMap<String, BleConnection>() // MAC address → BleConnection. remoteQaulId set once READ_CHAR comes back.
    // We would likely want another map of qaul ids to connections once the qaul id is retrieved to improve lookups
    private val pendingDisconnects = mutableSetOf<String>() // addresses of intentional disconnects in flight

    /**
     * Fired after the connections map changes (add or remove), for both roles. Lets observers like the
     * the foreground service notification read an accurate count
     */
    var onConnectionsChanged: (() -> Unit)? = null

    /**
     * Neighbour up/down, keyed by qaul ID. A neighbour is "up" while at least
     * one connection to that qaul ID is resolved, and "down" once the last such connection goes
     * away. BleManager wires these out to its qaul facing callbacks.
     */
    var onNeighbourUp: ((qaulId: ByteArray) -> Unit)? = null
    var onNeighbourDown: ((qaulId: ByteArray) -> Unit)? = null

    // qaul IDs (hex) currently reachable via at least one connection — the dedup key for up/down.
    private val upNeighbours = mutableSetOf<String>()

    private val reaper = Executors.newSingleThreadScheduledExecutor { r ->
        Thread(r, "connection-aliveness-watchdog").apply { isDaemon = true }
    }
    @Volatile private var reaperTask: ScheduledFuture<*>? = null
    @Volatile private var snapshotTask: ScheduledFuture<*>? = null

    /**
     * Periodic one-line view of this node's connections, count
     */
    private fun logTopologySnapshot() {
        try {
            val conns = connections.values.toList()
            val summary = if (conns.isEmpty()) "none" else conns.joinToString("  ·  ") { c ->
                val id = c.remoteQaulId?.toHexString()?.take(6) ?: "unresolved"
                "${c.device.address}/${c.role}/$id"
            }
            Log.i(TAG, "TOPOLOGY neighbours=${conns.size} up=${upNeighbours.size}: $summary")
        } catch (e: Exception) {
            Log.e(TAG, "snapshot failed", e)
        }
    }

    private fun reap() {
        try {
            val now = System.currentTimeMillis()
            connections.values.toList().forEach { conn ->
                if (now - conn.lastActivityAt > BleConstants.LIVENESS_TIMEOUT_MS)
                {
                    disconnect(conn.device)
                    Log.w(TAG, "Liveness: ${conn.device.address} last seen > ${BleConstants.LIVENESS_TIMEOUT_MS}ms ago — dropping")
                }
            }
        }
        catch (e: Exception){
            Log.e(TAG, "Reaper attempt failed", e)
        }
    }

    fun start() {
        BleTaskScheduler.registerListener(connectionEventListener)
        // Diagnostic topology snapshot every 10s — no behavioural effect, safe to remove later.
        snapshotTask = reaper.scheduleWithFixedDelay(
            { logTopologySnapshot() }, 10_000L, 10_000L, TimeUnit.MILLISECONDS
        )
//        reaperTask = reaper.scheduleWithFixedDelay(
//            {reap()},
//            BleConstants.LIVENESS_CHECK_INTERVAL_MS,
//            BleConstants.LIVENESS_CHECK_INTERVAL_MS,
//            TimeUnit.MILLISECONDS
//        ) TODO: Enable this once ble module is plugged into qaul and routing updates are actually circulating as they act as the live connection signal
    }

    fun stop() {
        reaperTask?.cancel(false)
        reaperTask = null
        snapshotTask?.cancel(false)
        snapshotTask = null
        BleTaskScheduler.unregisterListener(connectionEventListener)
        connections.values.forEach { it.disconnect() }
        connections.clear()
        upNeighbours.clear()
    }
    // How to deal with 2 devices both acting as central and peripheral at each other?

    // Connect / Disconnect

    fun createConnection(device: BluetoothDevice, role: BleRole) {
        if (connections.containsKey(device.address)) {
            Log.w(TAG, "Already connected to ${device.address}, ignoring")
            return
        }
        // here we likely put device limit
        val newConnection = BleConnection(device, role)
        newConnection.onQaulIdResolved = { dev, qaulId -> handleQaulIdResolved(dev, qaulId) }
        connections[device.address] = newConnection
        newConnection.connect()
        Log.i(TAG, "Connection added for ${device.address} (${connections.size} total)")
        onConnectionsChanged?.invoke()
    }

    fun disconnect(device: BluetoothDevice) {
        val conn = connections.remove(device.address) ?: run {
            Log.w(TAG, "disconnect called but no connection found for ${device.address}")
            return
        }
        // Only CENTRAL connections get a BleTaskScheduler onDisconnectedFromDevice callback to
        // confirm the disconnect. PERIPHERAL disconnects are handled entirely by GattServer so
        // pendingDisconnects would never be cleared, so we ensure
        // here that only CENTRALS get disconnected
        if (conn.role == BleRole.CENTRAL) {
            pendingDisconnects.add(device.address)
        }
        conn.disconnect()
        Log.i(TAG, "Connection removed for ${device.address} (${connections.size} remaining)")
        // Re-evaluate after removal: only reports DOWN if no other leg still holds this qaul ID.
        refreshNeighbourDown(conn.remoteQaulId)
        onConnectionsChanged?.invoke()
    }



    // Lookups

    fun getByAddress(address: String) : BleConnection? = connections[address]

    fun getByQaulId(qaulId: ByteArray) : BleConnection? = connections.values.firstOrNull{it.remoteQaulId?.contentEquals(qaulId) == true}

    fun allConnections(): List<BleConnection> = connections.values.toList()

    fun getSize(): Int = connections.size

    /**
     * The active connection whose remote qaul ID begins with [prefix] (the advertised truncated
     * ID), or null. Matches regardless of RPA address rotation — the basis for churn-free
     * auto-connect dedup. The scanner inspects the returned connection's role to decide whether to
     * skip (already in an acceptable role) or connect to fix a wrong-role peripheral. Connections
     * whose ID hasn't resolved yet (remoteQaulId == null) don't match; that brief window is covered
     * by the address-level dedup.
     */
    fun getByQaulIdPrefix(prefix: ByteArray): BleConnection? =
        connections.values.firstOrNull { conn ->
            val full = conn.remoteQaulId
            full != null && full.size >= prefix.size && full.copyOf(prefix.size).contentEquals(prefix)
        }

    /**
     * Pre-connection role hint: should WE be central given the peer's advertised qaul-ID [prefix]?
     * Lower qaul ID = central (compares our ID's matching-length prefix to theirs). Non-authoritative
     * — a prefix collision is resolved by the full-ID tiebreaker after connecting.
     */
    fun localShouldBeCentral(prefix: ByteArray): Boolean {
        val ours = BleConstants.LOCAL_QAUL_ID.copyOf(prefix.size)
        return compareUnsigned(ours, prefix) < 0
    }

    /**
     * Called by BleConnection when it first resolves the remote's qaul ID from the data stream
     * (SEND_ID FLC). For PERIPHERAL connections this is the only place we learn the remote ID,
     * as centrals can read it, so we use it to detect when two pool entries refer to the same physical device
     * so we use it to detect when two pool entries refer to the same physical device (both devices connected to each other simultaneously).
     * When a duplicate is found we resolve it with the qaul-ID tiebreaker: the device with the
     * lower qaul ID should be the CENTRAL. We drop whichever of the two connections contradicts
     * that, if we should be central we drop our PERIPHERAL entry, otherwise we drop our
     * CENTRAL entry. Both peers run the same comparison, so they agree on which connection survives.
     * TODO: Look into enhanced decision making for tie breaking, for example, the more powerful device should likely be CENTRAL as they can use a smaller connection interval, increasing throughput. there may be other factors as well
     */
    // TODO: Investigate bug where dual connection resolution only occurs after a message is sent
    private fun handleQaulIdResolved(device: BluetoothDevice, qaulId: ByteArray) {
        markNeighbourUp(qaulId)

        val existing = getByQaulId(qaulId)
        if (existing == null || existing.device.address == device.address) return

        // Timing A: PERIPHERAL resolves after CENTRAL already exists
        val localShouldBeCentral = compareUnsigned(BleConstants.LOCAL_QAUL_ID, qaulId) < 0
        val toDisconnect = if (localShouldBeCentral) device else existing.device
        Log.w(TAG, "Tiebreaker (SEND_ID path): local ${if (localShouldBeCentral) "wins" else "loses"} CENTRAL, dropping ${toDisconnect.address}")
        disconnect(toDisconnect)
    }

    private fun compareUnsigned(a: ByteArray, b: ByteArray): Int {
        val len = minOf(a.size, b.size)
        for (i in 0 until len) {
            val diff = (a[i].toInt() and 0xFF) - (b[i].toInt() and 0xFF)
            if (diff != 0) return diff
        }
        return a.size - b.size
    }


    // Neighbour up/down (qaul-ID keyed, deduplicated across connections)

    /**
     * Mark the neighbour with [qaulId] reachable. Fires [onNeighbourUp] only on the transition from
     * absent → present (the first connection to resolve this ID). A second connection to the same ID
     * (e.g. the other leg of a dual connection) is deduplicated — add() returns false — so qaul sees
     * exactly one UP per neighbour.
     */
    private fun markNeighbourUp(qaulId: ByteArray) {
        if (upNeighbours.add(qaulId.toHexString())) {
            Log.i(TAG, "Neighbour up: ${qaulId.toHexString()}")
            onNeighbourUp?.invoke(qaulId)
        }
    }

    /**
     * Re-evaluate reachability for [qaulId] after a connection was removed, firing [onNeighbourDown]
     * only if no remaining connection still holds this ID. This is what makes dropping one leg of a
     * dual connection (the tiebreaker) silent. has to be called after the connection has been removed
     * from [connections].
     */
     fun refreshNeighbourDown(qaulId: ByteArray?) {
        qaulId ?: return
        val stillReachable = connections.values.any { it.remoteQaulId?.contentEquals(qaulId) == true }
        if (!stillReachable && upNeighbours.remove(qaulId.toHexString())) {
            Log.i(TAG, "Neighbour down: ${qaulId.toHexString()}")
            onNeighbourDown?.invoke(qaulId)
        }
    }

    // Send


    fun sendMessage(qaulId: ByteArray, payload: ByteArray) {
        val conn = getByQaulId(qaulId)
        if (conn != null) {
            conn.sendMessage(payload)
        }
        else {
            Log.i(TAG, "Send failed, not connected to any device with Qaul ID: $qaulId")
        }
    }

    // Sends to all connected devices
    fun broadcast(payload: ByteArray){
        connections.values.forEach { it.sendMessage(payload) }
    }

    private val connectionEventListener = object : ConnectionEventListener {

        // Callback only for a CENTRAL connection
        override fun onDisconnectedFromDevice(device: BluetoothDevice) {
            if (pendingDisconnects.remove(device.address)) {
                // Intentional disconnect — already removed from map in disconnect() (which also
                // already ran refreshNeighbourDown), don't touch it (a new connection for this
                // address may already exist)
                Log.i(TAG, "Intentional disconnect confirmed for ${device.address}")
            } else {
                // Unexpected drop — clean up, then re-evaluate neighbour reachability
                val conn = connections.remove(device.address)
                Log.i(TAG, "Unexpected disconnect cleaned up for ${device.address}")
                refreshNeighbourDown(conn?.remoteQaulId)
            }
            onConnectionsChanged?.invoke()
        }

        override fun onNotificationReceived(
            device: BluetoothDevice,
            characteristic: BluetoothGattCharacteristic,
            value: ByteArray
        ) {
            if (characteristic.uuid == BleConstants.MSG_CHAR) {
                connections[device.address]?.onChunkReceived(value)
            }
        }

        override fun onMtuChanged(device: BluetoothDevice, newMtu: Int) {
            connections[device.address]?.onMtuNegotiated(newMtu)
        }

        override fun onCharacteristicRead(
            device: BluetoothDevice,
            characteristic: BluetoothGattCharacteristic,
            value: ByteArray
        ) {
            if (characteristic.uuid == BleConstants.PSM_CHAR) {
                // Peripheral's L2CAP PSM (4-byte big-endian). Open the high-bandwidth channel.
                val psm = if (value.size >= 4) {
                    java.nio.ByteBuffer.wrap(value).order(java.nio.ByteOrder.BIG_ENDIAN).int
                } else -1
                Log.i(TAG, "PSM received from ${device.address}: $psm")
                connections[device.address]?.connectL2cap(psm)
                return
            }
            if (characteristic.uuid == BleConstants.READ_CHAR) {
                val existing = getByQaulId(value)

                // Record the ID and announce the neighbour UP first, before any tiebreaker
                // disconnect. With both legs holding the ID, dropping either one leaves the neighbour up.
                connections[device.address]?.let { conn ->
                    conn.remoteQaulId = value
                    Log.i(TAG, "Qaul ID received from ${device.address}: ${value.toHexString()}")
                    markNeighbourUp(value)
                }

                if (existing != null && existing.device.address != device.address) {
                    when (existing.role) {
                        BleRole.CENTRAL -> {
                            // Two CENTRAL connections to same device — drop the newer one (the
                            // neighbour stays up via the existing CENTRAL leg).
                            Log.w(
                                TAG,
                                "Already connected as CENTRAL to this qaul ID via ${existing.device.address}, dropping duplicate CENTRAL ${device.address}"
                            )
                            disconnect(device)
                            return
                        }

                        BleRole.PERIPHERAL -> {
                            // Timing B: PERIPHERAL already resolved before we connected as CENTRAL.
                            // Apply tiebreaker. Both legs now hold the qaul ID, so dropping either
                            // leaves the neighbour up (no down will fire).
                            val localShouldBeCentral =
                                compareUnsigned(BleConstants.LOCAL_QAUL_ID, value) < 0
                            val toDisconnect = if (localShouldBeCentral) existing.device else device
                            Log.w(
                                TAG,
                                "Tiebreaker (READ_CHAR path): local ${if (localShouldBeCentral) "wins" else "loses"} CENTRAL, dropping ${toDisconnect.address}"
                            )
                            disconnect(toDisconnect)
                        }
                    }
                }
            }

        }
    }
}
