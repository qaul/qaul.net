package net.qaul.ble.test.ble.server

import android.annotation.SuppressLint
import android.bluetooth.BluetoothAdapter
import android.bluetooth.BluetoothDevice
import android.bluetooth.BluetoothGatt
import android.bluetooth.BluetoothGattCharacteristic
import android.bluetooth.BluetoothGattDescriptor
import android.bluetooth.BluetoothGattServer
import android.bluetooth.BluetoothGattServerCallback
import android.bluetooth.BluetoothGattService
import android.bluetooth.BluetoothManager
import android.bluetooth.BluetoothProfile
import android.bluetooth.BluetoothServerSocket
import android.content.Context
import android.os.Build
import android.util.Log
import net.qaul.ble.BleConstants
import net.qaul.ble.test.ble.connection.BleRole
import net.qaul.ble.test.ble.connection.ConnectionPool
import net.qaul.ble.test.ble.manager.BleManager
import net.qaul.ble.test.ble.queue.BleTaskScheduler
import java.io.IOException
import java.nio.ByteBuffer
import java.nio.ByteOrder

@SuppressLint("MissingPermission")
object GattServer {

    private const val TAG = "GattServer"

    private var gattServer: BluetoothGattServer? = null
    private var bluetoothAdapter: BluetoothAdapter? = null

    // L2CAP CoC server state. The OS assigns the PSM dynamically; centrals read it via PSM_CHAR.
    // Accepted channels are handed to the matching BleConnection, which owns them for both roles
    // (and closes them on disconnect), so GattServer keeps no per-device channel map of its own.
    private var l2capServerSocket: BluetoothServerSocket? = null
    private var l2capPsm: Int = -1                                  // -1 = L2CAP unavailable

    // Devices that have enabled notifications on MSG_CHAR
    private val subscribedDevices = mutableSetOf<BluetoothDevice>()

    // Called when a remote device connects or disconnects from our server
    var onClientConnectionChanged: ((device: BluetoothDevice, connected: Boolean) -> Unit)? = null

    // Start / stop

    fun start(context: Context) {
        val bluetoothManager = context.getSystemService(Context.BLUETOOTH_SERVICE) as BluetoothManager
        bluetoothAdapter = bluetoothManager.adapter
        gattServer = bluetoothManager.openGattServer(context, gattServerCallback)
        gattServer?.let {
            it.addService(buildService())
            BleTaskScheduler.setGattServer(it)
        } ?: Log.e(TAG, "Failed to open GATT server")
        startL2capServer()
        Log.i(TAG, "GATT server started")
    }

    fun stop() {
        // Closing PERIPHERAL pool entries also closes their L2CAP channels (BleConnection owns them).
        ConnectionPool.allConnections()
            .filter { it.role == BleRole.PERIPHERAL }
            .forEach { ConnectionPool.disconnect(it.device) }

        // Close the L2CAP server socket to unblock and end the accept loop.
        try { l2capServerSocket?.close() } catch (_: IOException) {}
        l2capServerSocket = null
        l2capPsm = -1

        gattServer?.close()
        gattServer = null
        subscribedDevices.clear()
        BleTaskScheduler.clearGattServer()
        Log.i(TAG, "GATT server stopped")
    }

    /**
     * Open an insecure L2CAP CoC listening socket and start accepting connections.
     * Insecure (no bonding) is fine because qaul encrypts at its own layer. Requires API 29+.
     */
    private fun startL2capServer() {
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.Q) {
            Log.w(TAG, "L2CAP CoC requires API 29+, skipping — centrals will see PSM = -1")
            return
        }
        try {
            val ss = bluetoothAdapter?.listenUsingInsecureL2capChannel() ?: run {
                Log.e(TAG, "No BluetoothAdapter for L2CAP server")
                return
            }
            l2capServerSocket = ss
            l2capPsm = ss.psm
            Log.i(TAG, "L2CAP server listening, PSM=$l2capPsm")
            Thread({ acceptLoop(ss) }, "l2cap-accept").apply {
                isDaemon = true
                start()
            }
        } catch (e: IOException) {
            Log.e(TAG, "Failed to open L2CAP server socket: ${e.message}")
            l2capPsm = -1
        }
    }

    /** Blocking accept loop on its own thread. Exits when the server socket is closed in stop(). */
    private fun acceptLoop(ss: BluetoothServerSocket) {
        while (true) {
            val socket = try {
                ss.accept()                         // blocks; throws when ss.close() is called
            } catch (e: IOException) {
                Log.i(TAG, "L2CAP accept loop ended: ${e.message}")
                break
            }
            val addr = socket.remoteDevice.address
            Log.i(TAG, "L2CAP channel accepted from $addr")
            val conn = ConnectionPool.getByAddress(addr)
            if (conn != null) {
                conn.attachL2capSocket(socket)          // BleConnection takes ownership
            } else {
                // No BleConnection for this device (e.g. the dual-connection tiebreaker already
                // dropped it). Discard the orphan socket rather than leak it.
                Log.w(TAG, "No BleConnection for $addr; closing orphan L2CAP socket")
                try { socket.close() } catch (_: IOException) {}
            }
        }
    }

    fun getSubscribedDevices(): Set<BluetoothDevice> = subscribedDevices.toSet()

    /**
     * Drop a client we've discovered is gone, e.g. a notify hit a dead binder (DeadObjectException).
     * The normal path is onConnectionStateChange(DISCONNECTED), but Android sometimes drops that
     * server side callback, leaving a stale subscription that keeps failing. Removing it here makes
     * subsequent queued notifies to this device skip fast via the isSubscribed() check.
     */
    fun markClientGone(device: BluetoothDevice) {
        if (subscribedDevices.remove(device)) {
            Log.w(TAG, "Dropped stale subscription for ${device.address} (client gone)")
        }
    }

    // Service definition

    private fun buildService(): BluetoothGattService {
        val service = BluetoothGattService(
            BleConstants.SERVICE_UUID,
            BluetoothGattService.SERVICE_TYPE_PRIMARY
        )

        // READ_CHAR — remote reads this to get our identifier (test: just a fixed string)
        val readChar = BluetoothGattCharacteristic(
            BleConstants.READ_CHAR,
            BluetoothGattCharacteristic.PROPERTY_READ,
            BluetoothGattCharacteristic.PERMISSION_READ
        )

        // MSG_CHAR: remote writes to send us data, we notify to push data to them
        // WRITE_NO_RESPONSE is included so the central can use fire-and-forget writes,
        // which avoids a per-chunk round-trip acknowledgment and improves throughput
        val msgChar = BluetoothGattCharacteristic(
            BleConstants.MSG_CHAR,
            BluetoothGattCharacteristic.PROPERTY_WRITE or
                BluetoothGattCharacteristic.PROPERTY_WRITE_NO_RESPONSE or
                BluetoothGattCharacteristic.PROPERTY_NOTIFY,
            BluetoothGattCharacteristic.PERMISSION_WRITE
        )

        // CCCD descriptor is required for the remote to enable notifications on MSG_CHAR
        val cccd = BluetoothGattDescriptor(
            BleConstants.CCCD_UUID,
            BluetoothGattDescriptor.PERMISSION_READ or BluetoothGattDescriptor.PERMISSION_WRITE
        )
        msgChar.addDescriptor(cccd)

        // PSM_CHAR — central reads this to learn our L2CAP PSM, then opens the L2CAP channel.
        val psmChar = BluetoothGattCharacteristic(
            BleConstants.PSM_CHAR,
            BluetoothGattCharacteristic.PROPERTY_READ,
            BluetoothGattCharacteristic.PERMISSION_READ
        )

        service.addCharacteristic(readChar)
        service.addCharacteristic(msgChar)
        service.addCharacteristic(psmChar)
        return service
    }

    // GATT server callbacks

    /**
     * Wraps [BluetoothGattServer.sendResponse] and logs a failed local send. A false return means
     * the response never reached the stack, almost always the device already disconnected, or the
     * GATT server stack is wedged. Note: a true return means accepted locally, not that the central received it.
     */
    private fun respond(device: BluetoothDevice, requestId: Int, status: Int, offset: Int, value: ByteArray?) {
        val ok = gattServer?.sendResponse(device, requestId, status, offset, value) ?: false
        if (!ok) Log.e(TAG, "sendResponse REJECTED for ${device.address} (requestId=$requestId) — device gone or stack wedged")
    }

    private val gattServerCallback = object : BluetoothGattServerCallback() {

        override fun onConnectionStateChange(device: BluetoothDevice, status: Int, newState: Int) {
            if (newState == BluetoothProfile.STATE_CONNECTED) {
                if (status != BluetoothGatt.GATT_SUCCESS) {
                    Log.e(TAG, "Client connection error for ${device.address}, status=$status")
                    return
                }
                Log.i(TAG, "Client connected: ${device.address}")
                // Only create a PERIPHERAL pool entry if we don't already have a CENTRAL
                // connection to this device — if we do, the GattServer link is the second
                // leg of a dual-role pair and the CENTRAL entry should stay.
                val existing = ConnectionPool.getByAddress(device.address)
                when {
                    existing == null -> {
                        if (ConnectionPool.getSize() >= BleConstants.MAX_CONNECTIONS) {
                            Log.i(
                                TAG,
                                "At connection cap (${BleConstants.MAX_CONNECTIONS}), rejecting inbound ${device.address}"
                            )
                            gattServer?.cancelConnection(device)   // best-effort; old stacks honour it inconsistently
                            return
                        }
                        BleManager.connect(device, BleRole.PERIPHERAL)
                    }
                    existing.role == BleRole.PERIPHERAL -> {
                        // Stale entry from a previous connection — replace it
                        Log.i(TAG, "${device.address} reconnected, replacing stale PERIPHERAL entry")
                        BleManager.disconnect(device)
                        BleManager.connect(device, BleRole.PERIPHERAL)
                    }
                    else -> {
                        // Active CENTRAL connection to this device, don't create a second entry.
                        // TODO: same MAC dual connection. Pool is MAC keyed so it can't hold both
                        //  directions, and we always keep CENTRAL here regardless of the qaul-ID tiebreaker.
                        //  When we should be PERIPHERAL this keeps the wrong role. Unlikely (only when the
                        //  remote reuses one address for advertising + initiating), revisit if observed.
                        Log.i(TAG, "${device.address} already in pool as CENTRAL, skipping PERIPHERAL entry")
                    }
                }
                onClientConnectionChanged?.invoke(device, true)
            } else if (newState == BluetoothProfile.STATE_DISCONNECTED) {
                Log.i(TAG, "Client disconnected: ${device.address}")
                subscribedDevices.remove(device)
                // Clean up pool before notifying UI so the UI refresh reads correct state.
                // Only remove PERIPHERAL entries, don't touch a CENTRAL connection for the
                // same device that is still alive
                val existing = ConnectionPool.getByAddress(device.address)
                if (existing?.role == BleRole.PERIPHERAL) {
                    BleManager.disconnect(device)
                }
                onClientConnectionChanged?.invoke(device, false)
            }
        }

        override fun onCharacteristicReadRequest(
            device: BluetoothDevice,
            requestId: Int,
            offset: Int,
            characteristic: BluetoothGattCharacteristic
        ) {
            when (characteristic.uuid) {
                BleConstants.READ_CHAR -> {
                    val response = BleConstants.LOCAL_QAUL_ID
                    respond(device, requestId, BluetoothGatt.GATT_SUCCESS, offset, response)
                    Log.i(TAG, "READ_CHAR read by ${device.address}")
                }
                BleConstants.PSM_CHAR -> {
                    // 4-byte big-endian PSM (or -1 if L2CAP is unavailable)
                    val psmBytes = ByteBuffer.allocate(4).order(ByteOrder.BIG_ENDIAN).putInt(l2capPsm).array()
                    respond(device, requestId, BluetoothGatt.GATT_SUCCESS, offset, psmBytes)
                    Log.i(TAG, "PSM_CHAR ($l2capPsm) read by ${device.address}")
                }
                else -> respond(device, requestId, BluetoothGatt.GATT_FAILURE, 0, null)
            }
        }

        override fun onCharacteristicWriteRequest(
            device: BluetoothDevice,
            requestId: Int,
            characteristic: BluetoothGattCharacteristic,
            preparedWrite: Boolean,
            responseNeeded: Boolean,
            offset: Int,
            value: ByteArray
        ) {
            if (characteristic.uuid == BleConstants.MSG_CHAR) {
                // Hot path: per-chunk logcat write throttles the write-receive rate. Debug only.
                //Log.i(TAG, "Chunk received from ${device.address}: ${value.size} bytes")
                BleManager.routeIncomingChunk(device, value)
                if (responseNeeded) {
                    respond(device, requestId, BluetoothGatt.GATT_SUCCESS, 0, null)
                }
            } else {
                if (responseNeeded) {
                    respond(device, requestId, BluetoothGatt.GATT_FAILURE, 0, null)
                }
            }
        }

        override fun onDescriptorWriteRequest(
            device: BluetoothDevice,
            requestId: Int,
            descriptor: BluetoothGattDescriptor,
            preparedWrite: Boolean,
            responseNeeded: Boolean,
            offset: Int,
            value: ByteArray
        ) {
            if (descriptor.uuid == BleConstants.CCCD_UUID) {
                if (value.contentEquals(BluetoothGattDescriptor.ENABLE_NOTIFICATION_VALUE)) {
                    subscribedDevices.add(device)
                    Log.i(TAG, "${device.address} enabled notifications")
                } else if (value.contentEquals(BluetoothGattDescriptor.DISABLE_NOTIFICATION_VALUE)) {
                    subscribedDevices.remove(device)
                    Log.i(TAG, "${device.address} disabled notifications")
                }
                if (responseNeeded) {
                    respond(device, requestId, BluetoothGatt.GATT_SUCCESS, 0, null)
                }
            } else {
                if (responseNeeded) {
                    respond(device, requestId, BluetoothGatt.GATT_FAILURE, 0, null)
                }
            }
        }

        override fun onMtuChanged(device: BluetoothDevice, mtu: Int) {
            Log.i(TAG, "MTU changed to $mtu for ${device.address}")
            ConnectionPool.getByAddress(device.address)?.onMtuNegotiated(mtu)
        }

        override fun onNotificationSent(device: BluetoothDevice, status: Int) {
            BleTaskScheduler.notificationSent(device, status)
        }
    }

    // Allows task scheduler to check subscriptions
    fun isSubscribed(device: BluetoothDevice): Boolean = subscribedDevices.contains(device)
}
