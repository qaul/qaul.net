package net.qaul.ble.test.ble.manager

import android.bluetooth.BluetoothDevice
import android.content.Context
import android.util.Log
import net.qaul.ble.test.ble.connection.BleConnection
import net.qaul.ble.test.ble.connection.BleRole
import net.qaul.ble.test.ble.connection.ConnectionPool
import net.qaul.ble.test.ble.queue.BleTaskScheduler

object BleManager : ConnectionEventListener {

    var onMessageReceived: ((neighbourQaulId: ByteArray, payload: ByteArray) -> Unit)? = null

    /**
     * qaul-facing neighbour events, keyed by the peer's 8-byte qaul ID. Fired once when a neighbour
     * becomes reachable and once when it becomes unreachable — deduplicated across the dual-connection
     * window inside ConnectionPool. qaul sets these; the module invokes them.
     * TODO: extend onNeighbourUp with transport (BLE 1M vs Coded) and RSSI for the routing metric.
     */
    var onNeighbourUp: ((qaulId: ByteArray) -> Unit)? = null
    var onNeighbourDown: ((qaulId: ByteArray) -> Unit)? = null

    /** Must be called once at startup. Captures the application context for connectGatt so it
     *  doesn't have to be threaded through every connect() call. */
    fun start(context: Context) {
        BleTaskScheduler.setAppContext(context)
        // Forward ConnectionPool's qaul-ID-keyed neighbour events out to qaul.
        ConnectionPool.onNeighbourUp = { qaulId -> onNeighbourUp?.invoke(qaulId) }
        ConnectionPool.onNeighbourDown = { qaulId -> onNeighbourDown?.invoke(qaulId) }
        ConnectionPool.start()
        BleTaskScheduler.registerListener(this)
    }
    fun stop(){
        BleTaskScheduler.unregisterListener(this)
        ConnectionPool.onNeighbourUp = null
        ConnectionPool.onNeighbourDown = null
        ConnectionPool.stop()
        // Safety net: force-close any GATT client handles that ConnectionPool's disconnects didn't
        // (queued Disconnect ops may not have run). Prevents leaked client interfaces across stop/start.
        BleTaskScheduler.closeAllGatts()
    }

    fun connect(device: BluetoothDevice, role: BleRole) =
        ConnectionPool.createConnection(device, role)

    fun disconnect(device: BluetoothDevice) =
        ConnectionPool.disconnect(device)

    fun sendMessage(qaulId: ByteArray, payload: ByteArray) =
        ConnectionPool.sendMessage(qaulId, payload)

    fun isConnected(device: BluetoothDevice): Boolean =
        ConnectionPool.getByAddress(device.address) != null

    fun isConnected(address: String): Boolean =
        ConnectionPool.getByAddress(address) != null

    fun connectedDevices(): List<BleConnection> =
        ConnectionPool.allConnections()

    fun routeIncomingChunk(device: BluetoothDevice, chunk: ByteArray) {
        ConnectionPool.getByAddress(device.address)?.onChunkReceived(chunk)
    }

    override fun onMessageAssembled(device: BluetoothDevice, payload: ByteArray) {
        val qaulId = ConnectionPool.getByAddress(device.address)?.remoteQaulId
        if (qaulId != null) {
            onMessageReceived?.invoke(qaulId, payload)
        }
        else{
            // (ID resolves during setup / early in the stream), but guard so we never
            // hand qaul a null-keyed message.
            Log.w("BleManager", "Assembled message from ${device.address} with no resolved qaulId; dropping")
        }

    }

    /**
     * Returns a display label for the scan list, e.g. "★C", "★P", or null if not connected.
     * Each scanned MAC is looked up independently, so if two MACs for the same physical device
     * are in the pool (brief dual-role window) they will each show their own role separately.
     */
    fun connectionLabel(address: String): String? {
        val conn = ConnectionPool.getByAddress(address) ?: return null
        return if (conn.role == BleRole.CENTRAL) "★C" else "★P"
    }
}