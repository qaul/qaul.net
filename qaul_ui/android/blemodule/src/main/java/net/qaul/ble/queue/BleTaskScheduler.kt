package net.qaul.ble.test.ble.queue

import android.annotation.SuppressLint
import android.bluetooth.BluetoothDevice
import android.bluetooth.BluetoothGatt
import android.bluetooth.BluetoothGattCallback
import android.bluetooth.BluetoothGattCharacteristic
import android.bluetooth.BluetoothGattDescriptor
import android.bluetooth.BluetoothGattServer
import android.bluetooth.BluetoothProfile
import android.bluetooth.BluetoothStatusCodes
import android.content.Context
import android.os.Build
import android.util.Log
import net.qaul.ble.BleConstants
import net.qaul.ble.test.ble.manager.BleManager
import net.qaul.ble.test.ble.manager.ConnectionEventListener
import net.qaul.ble.test.ble.scanner.BleScanner
import net.qaul.ble.test.ble.server.GattServer
import net.qaul.ble.test.ble.server.GattServer.isSubscribed
import net.qaul.ble.test.ble.util.isIndicatable
import net.qaul.ble.test.ble.util.isNotifiable
import net.qaul.ble.test.ble.util.isReadable
import net.qaul.ble.test.ble.util.isWritable
import net.qaul.ble.test.ble.util.isWritableWithoutResponse
import net.qaul.ble.test.ble.util.printGattTable
import net.qaul.ble.test.ble.util.toHexString
import java.lang.ref.WeakReference
import java.util.UUID
import java.util.concurrent.ConcurrentLinkedQueue
import java.util.concurrent.Executors
import java.util.concurrent.ScheduledFuture
import java.util.concurrent.TimeUnit

object BleTaskScheduler {

    private const val TAG = "BleTaskScheduler"
    private const val GATT_MIN_MTU = 23
    private const val GATT_MAX_MTU = 517

    // Three queues (see OpLane). CONTROL is served fully before MEDIUM, MEDIUM fully
    // before BULK, so a lower lane can never delay a higher one.
    // CONTROL: connection/setup ops (connect, discover, MTU, PHY, notifications, disconnect) + flow control.
    private val bleOperationQueue = ConcurrentLinkedQueue<BleOperationType>()
    // MEDIUM: short message payload (routing updates, chat). Ahead of large transfers so a file can't
    // starve routing convergence, but behind connection setup / flow control.
    private val mediumOperationQueue = ConcurrentLinkedQueue<BleOperationType>()
    // BULK: large message payloads (images/files). Drained only when CONTROL and MEDIUM are both empty.
    private val bulkOperationQueue = ConcurrentLinkedQueue<BleOperationType>()
    @Volatile private var pendingOperation: BleOperationType? = null

    // Watchdog: if a pending operation never completes — a dropped GATT callback (connectGatt
    // hanging, a missing write/read callback, which the Android BLE stack genuinely does) —
    // pendingOperation would stay non-null forever and BOTH queues wedge permanently until the
    // app restarts. The watchdog arms a per-operation timeout and force-advances if it fires.
    private val watchdog = Executors.newSingleThreadScheduledExecutor { r ->
        Thread(r, "ble-scheduler-watchdog").apply { isDaemon = true }
    }
    @Volatile private var watchdogTask: ScheduledFuture<*>? = null

    /** Which lane an operation belongs in. Only MSG_CHAR payload ops have a lane everything else (connect,
     *  discover, MTU, etc) is CONTROL. */
    private fun queueForOperation(op: BleOperationType): ConcurrentLinkedQueue<BleOperationType> {
        val lane = when (op) {
            is CharacteristicWrite -> if (op.characteristicUuid == BleConstants.MSG_CHAR) op.lane else OpLane.CONTROL
            is NotifyCharacteristicChange -> if (op.characteristicUuid == BleConstants.MSG_CHAR) op.lane else OpLane.CONTROL
            else -> OpLane.CONTROL
        }
        return when (lane) {
            OpLane.CONTROL -> bleOperationQueue
            OpLane.MEDIUM -> mediumOperationQueue
            OpLane.BULK -> bulkOperationQueue
        }
    }

    private fun hasPendingOps(): Boolean =
        bleOperationQueue.isNotEmpty() || mediumOperationQueue.isNotEmpty() || bulkOperationQueue.isNotEmpty()

    // Tracks GATT connections by device. Touched from the scheduler, GATT callback threads, and
    // the watchdog thread, so it must be concurrent.
    private val deviceGattMap = java.util.concurrent.ConcurrentHashMap<BluetoothDevice, BluetoothGatt>()

    // Application context, captured once at startup (BleManager.start). connectGatt needs a
    // Context, but the app context is process-lifetime constant, so we hold it here instead of
    // threading it through every connect() call.
    @Volatile private var appContext: Context? = null

    fun setAppContext(context: Context) {
        appContext = context.applicationContext
    }

    private var gattServer: BluetoothGattServer? = null

    private val listeners = mutableSetOf<WeakReference<ConnectionEventListener>>()

    // Listener registration

    fun registerListener(listener: ConnectionEventListener) {
        if (listeners.map { it.get() }.contains(listener)) return
        listeners.add(WeakReference(listener))
        trimListeners()
    }

    fun unregisterListener(listener: ConnectionEventListener) {
        listeners.removeIf { it.get() == listener }
    }

    private fun trimListeners() {
        listeners.removeIf { it.get() == null }
    }

    private fun notifyListeners(block: ConnectionEventListener.() -> Unit) {
        trimListeners()
        listeners.forEach { it.get()?.block() }
    }

    // Public API — each call just enqueues an operation

    fun connect(device: BluetoothDevice, phy: Int = BluetoothDevice.PHY_LE_1M_MASK) =
        scheduleOperation(Connect(device, phy))

    fun disconnect(device: BluetoothDevice) {
        // Drop any writes/reads still queued for this device first, we're tearing it down, so running
        // them against a dead link could potentially burn a 5s watchdog timeout each and block the single
        // execution slot.
        purgeOperationsForDevice(device)
        scheduleOperation(Disconnect(device))
    }

    /**
     * Remove every queued operation for [device] from both queues. Called when a device is torn down
     * or disconnects, so stale ops to a dead link don't each stall the queue. The watchdog covers the currently pending op.
     */
    @Synchronized
    private fun purgeOperationsForDevice(device: BluetoothDevice) {
        val before = bleOperationQueue.size + mediumOperationQueue.size + bulkOperationQueue.size
        bleOperationQueue.removeIf { it.device == device }
        mediumOperationQueue.removeIf { it.device == device }
        bulkOperationQueue.removeIf { it.device == device }
        val removed = before - (bleOperationQueue.size + mediumOperationQueue.size + bulkOperationQueue.size)
        if (removed > 0) Log.i(TAG, "Purged $removed queued op(s) for ${device.address}")
    }

    /**
     * Force-close every GATT client handle and reset the scheduler. The safety net for teardown/BT-off:
     * callbacks should close handles on disconnect, but if the engine stops (or BT toggles) while
     * connections are live or Disconnect ops are still queued, those client interfaces would leak
     * Call on engine stop.
     */
    @Synchronized
    fun closeAllGatts() {
        @SuppressLint("MissingPermission")
        deviceGattMap.values.forEach { gatt ->
            try { gatt.disconnect() } catch (_: Exception) {}
            try { gatt.close() } catch (_: Exception) {}
        }
        val n = deviceGattMap.size
        deviceGattMap.clear()
        bleOperationQueue.clear()
        mediumOperationQueue.clear()
        bulkOperationQueue.clear()
        pendingOperation = null
        disarmWatchdog()
        Log.i(TAG, "closeAllGatts: closed $n client handle(s) and reset scheduler")
    }

    fun setGattServer(server: BluetoothGattServer) {
        gattServer = server
    }

    fun clearGattServer() {
        gattServer = null
    }

    fun notifyMessageAssembled(device: BluetoothDevice, payload: ByteArray) {
        notifyListeners { onMessageAssembled(device, payload) }
    }

    fun readCharacteristic(device: BluetoothDevice, uuid: UUID) =
        scheduleOperation(CharacteristicRead(device, uuid))

    fun writeCharacteristic(
        device: BluetoothDevice,
        uuid: UUID,
        payload: ByteArray,
        lane: OpLane = OpLane.BULK
    ) {
        val gatt = deviceGattMap[device] ?: run {
            Log.e(TAG, "Cannot write to $uuid: not connected")
            return
        }
        val characteristic = gatt.findCharacteristic(uuid) ?: run {
            Log.e(TAG, "Cannot write to $uuid: characteristic not found")
            return
        }
        val writeType = when {
            characteristic.isWritable() -> BluetoothGattCharacteristic.WRITE_TYPE_NO_RESPONSE
            characteristic.isWritableWithoutResponse() -> BluetoothGattCharacteristic.WRITE_TYPE_NO_RESPONSE
            else -> {
                Log.e(TAG, "${characteristic.uuid} cannot be written to")
                return
            }
        }
        scheduleOperation(CharacteristicWrite(device, characteristic.uuid, writeType, payload, lane))
    }

    fun readDescriptor(device: BluetoothDevice, descriptor: BluetoothGattDescriptor) {
        if (deviceGattMap.containsKey(device)) {
            scheduleOperation(DescriptorRead(device, descriptor.uuid))
        } else {
            Log.e(TAG, "Cannot read descriptor: not connected to ${device.address}")
        }
    }

    fun writeDescriptor(
        device: BluetoothDevice,
        descriptor: BluetoothGattDescriptor,
        payload: ByteArray
    ) {
        if (deviceGattMap.containsKey(device)) {
            scheduleOperation(DescriptorWrite(device, descriptor.uuid, payload))
        } else {
            Log.e(TAG, "Cannot write descriptor: not connected to ${device.address}")
        }
    }

    fun discoverServices(device: BluetoothDevice) =
        scheduleOperation(ServiceDiscovery(device))

    fun requestMtu(device: BluetoothDevice, mtu: Int) {
        scheduleOperation(MtuRequest(device, mtu.coerceIn(GATT_MIN_MTU, GATT_MAX_MTU)))
    }
    // TODO: Better failure logging, check if these implementations are too similar to the tutorial
    fun enableNotifications(device: BluetoothDevice, uuid: UUID) {
        scheduleOperation(EnableNotifications(device, uuid))
    }

    fun disableNotifications(device: BluetoothDevice, uuid: UUID) {
        if (deviceGattMap.containsKey(device)) {
            scheduleOperation(DisableNotifications(device, uuid))
        } else {
            Log.e(TAG, "Cannot disable notifications for ${uuid}: not connected")
        }
    }

    fun notifyCharacteristicChanged(
        device: BluetoothDevice,
        uuid: UUID,
        confirmation: Boolean,
        payload: ByteArray,
        lane: OpLane = OpLane.BULK
    ) {
        scheduleOperation(NotifyCharacteristicChange(device, uuid, confirmation, payload, lane))
    }

    /**
     * Request a connection parameter update. Android has no
     * paired callback, so the scheduler skips immediately after the call.
     *
     * Use [BluetoothGatt.CONNECTION_PRIORITY_HIGH] during active transfers for the tightest
     * connection interval . TODO: Look into how this affects battery life, maybe only switch on when speed is needed
     */
    fun requestConnectionPriority(device: BluetoothDevice, priority: Int) =
        scheduleOperation(ConnectionPriorityRequest(device, priority))

    /**
     * Request a PHY update. Completion is signalled via [onPhyUpdate] in the GATT callback.
     * Requires API 26+, skipped on older devices.
     *
     * Pass [BluetoothDevice.PHY_LE_2M_MASK] for both tx and rx for maximum throughput.
     * [BluetoothDevice.PHY_OPTION_NO_PREFERRED] lets the controller choose the coding scheme.
     */
    fun setPreferredPhy(device: BluetoothDevice, txPhy: Int, rxPhy: Int, phyOptions: Int) =
        scheduleOperation(PhyRequest(device, txPhy, rxPhy, phyOptions))


    // Queue management
    //
    // KNOWN EDGE CASES / FAILURE MODES (not yet handled):
    //
    // 1. No watchdog on pendingOperation.
    //    The scheduler only advances when a callback fires signalOperationComplete/skipOperation.
    //    If the Android BLE stack drops a callback, pendingOperation stays non-null forever, executeNext returns early
    //    on every future call, and the scheduler gets stuck.
    //    Fix: a timeout that calls skipOperation if no callback within N seconds.
    //
    // 2. With the new priority queue, there could be a scenario where something like continuous
    //    connection / disconnect attempts keep the priority queue full which would block all
    //    message sends.
    //

    @Synchronized
    private fun scheduleOperation(operation: BleOperationType) {
        queueForOperation(operation).add(operation)
        if (pendingOperation == null) {
            executeNext()
        }
    }

    @Synchronized
    private fun executeNext() {
        if (pendingOperation != null) {
            Log.e(TAG, "doNextOperation called while operation already pending, aborting")
            return
        }

        // Strict lane priority: CONTROL first, then MEDIUM (short messages), then BULK (large transfers)
        val operation = bleOperationQueue.poll() ?: mediumOperationQueue.poll() ?: bulkOperationQueue.poll() ?: run {
            Log.d(TAG, "Queue empty")
            disarmWatchdog()
            return
        }
        pendingOperation = operation
        armWatchdog(operation)

        when (operation) {
            is Connect -> with(operation) {
                val ctx = appContext
                if (ctx == null) {
                    Log.e(TAG, "Cannot connect to ${device.address}: appContext not set (call BleManager.start(context))")
                    skipOperation()
                } else {
                    Log.i(TAG, "Connecting to ${device.address}")
                    // Pause scanning so the radio isn't busy scanning while we establish the link. Having both at once is known to cause trouble
                    // for some devices as active scan can starve connectGatt (hangs / status 133). Resumed
                    // once the connect settles (connected / error etc). TODO: Double check / Think about how much this reduces our ability to / speed of getting other connections, can it block scanning permanently
                    BleScanner.pauseForConnect()
                    @SuppressLint("MissingPermission")
                    // On API 26+ open the connection on the chosen PHY (Coded for long-range peers we
                    // only saw on the Coded advert, 1M otherwise). Older devices lack the phy overload
                    // and connect on 1M only
                    val gatt = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
                        device.connectGatt(ctx, false, gattCallback, BluetoothDevice.TRANSPORT_LE, phy)
                    } else {
                        device.connectGatt(ctx, false, gattCallback, BluetoothDevice.TRANSPORT_LE)
                    }
                    if (gatt != null) {
                        // Hold the handle immediately, connectGatt allocates a client interface that
                        // MUST be closed even if the connection never completes, otherwise it
                        // leaks, and after a few leaks all new connects fail with status 133.
                        // Storing it now means a stuck Connect or a Disconnect can always
                        // find and close it.If a stale handle is already mapped for this
                        // device, close it before overwriting so we can never orphan a client interface.
                        @SuppressLint("MissingPermission")
                        deviceGattMap.put(device, gatt)?.let { old ->
                            if (old !== gatt) { try { old.close() } catch (_: Exception) {} }
                        }
                    } else {
                        Log.e(TAG, "connectGatt returned null for ${device.address}")
                        BleScanner.resumeAfterConnect()   // connect never started, let the scan back
                        skipOperation()
                    }
                }
            }
            is Disconnect -> with(operation) {
                val gatt = deviceGattMap[device]
                if (gatt != null) {
                    Log.i(TAG, "Disconnecting from ${device.address}")
                    @SuppressLint("MissingPermission")
                    gatt.disconnect()
                } else {
                    Log.e(TAG, "Cannot disconnect from ${device.address}: no GATT found")
                    skipOperation()
                }
            }
            is ServiceDiscovery -> with(operation) {
                val gatt = deviceGattMap[device]
                if (gatt != null) {
                    Log.i(TAG, "Discovering services for ${device.address}")
                    @SuppressLint("MissingPermission")
                    gatt.discoverServices()
                } else {
                    Log.e(TAG, "Cannot discover services: no GATT for ${device.address}")
                    skipOperation()
                }
            }
            is CharacteristicRead -> with(operation) {
                val gatt = deviceGattMap[device]
                val characteristic = gatt?.findCharacteristic(characteristicUuid)
                if (gatt != null && characteristic != null) {
                    if (characteristic.isReadable()) {
                        @SuppressLint("MissingPermission")
                        gatt.readCharacteristic(characteristic)
                    }
                    else {
                        Log.e(TAG, "Cannot read ${characteristic.uuid}: not a readable characteristic")
                    }
                } else {
                    Log.e(TAG, "Cannot read $characteristicUuid: ${if (gatt == null) "no GATT" else "characteristic not found"}")
                    skipOperation()
                }
            }
            is CharacteristicWrite -> with(operation) {
                val gatt = deviceGattMap[device]
                val characteristic = gatt?.findCharacteristic(characteristicUuid)
                if (gatt != null && characteristic != null) {
                    try {
                        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
                            @SuppressLint("MissingPermission")
                            gatt.writeCharacteristic(characteristic, payload, writeType)
                        } else {
                            @Suppress("DEPRECATION")
                            characteristic.writeType = writeType
                            @Suppress("DEPRECATION")
                            characteristic.value = payload
                            @SuppressLint("MissingPermission")
                            @Suppress("DEPRECATION")
                            gatt.writeCharacteristic(characteristic)
                        }
                    } catch (e: IllegalArgumentException) {
                        Log.e(TAG, "Payload too large for $characteristicUuid (${payload.size} bytes): ${e.message}")
                        skipOperation()
                    }
                } else {
                    Log.e(TAG, "Cannot write $characteristicUuid: ${if (gatt == null) "no GATT" else "characteristic not found"}")
                    skipOperation()
                }
            }
            is DescriptorRead -> with(operation) {
                val gatt = deviceGattMap[device]
                val descriptor = gatt?.findDescriptor(descriptorUuid)
                if (gatt != null && descriptor != null) {
                    @SuppressLint("MissingPermission")
                    gatt.readDescriptor(descriptor)
                } else {
                    Log.e(TAG, "Cannot read descriptor $descriptorUuid: ${if (gatt == null) "no GATT" else "descriptor not found"}")
                    skipOperation()
                }
            }
            is DescriptorWrite -> with(operation) {
                val gatt = deviceGattMap[device]
                val descriptor = gatt?.findDescriptor(descriptorUuid)
                if (gatt != null && descriptor != null) {
                    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
                        @SuppressLint("MissingPermission")
                        gatt.writeDescriptor(descriptor, payload)
                    } else {
                        @Suppress("DEPRECATION")
                        descriptor.value = payload
                        @SuppressLint("MissingPermission")
                        @Suppress("DEPRECATION")
                        gatt.writeDescriptor(descriptor)
                    }
                } else {
                    Log.e(TAG, "Cannot write descriptor $descriptorUuid: ${if (gatt == null) "no GATT" else "descriptor not found"}")
                    skipOperation()
                }
            }
            is MtuRequest -> with(operation) {
                val gatt = deviceGattMap[device]
                if (gatt != null) {
                    @SuppressLint("MissingPermission")
                    gatt.requestMtu(mtu)
                } else {
                    Log.e(TAG, "Cannot request MTU: no GATT for ${device.address}")
                    skipOperation()
                }
            }
            is EnableNotifications -> with(operation) {
                val gatt = deviceGattMap[device]
                val characteristic = gatt?.findCharacteristic(characteristicUuid)
                if (gatt == null || characteristic == null) {
                    Log.e(TAG, "Cannot enable notifications for $characteristicUuid: ${if (gatt == null) "no GATT" else "characteristic not found"}")
                    skipOperation()
                    return
                }
                val cccd = characteristic.getDescriptor(BleConstants.CCCD_UUID)
                if (cccd == null) {
                    Log.e(TAG, "$characteristicUuid has no CCCD descriptor")
                    skipOperation()
                    return
                }
                val payload = when {
                    characteristic.isIndicatable() -> BluetoothGattDescriptor.ENABLE_INDICATION_VALUE
                    characteristic.isNotifiable() -> BluetoothGattDescriptor.ENABLE_NOTIFICATION_VALUE
                    else -> {
                        Log.e(TAG, "$characteristicUuid doesn't support notifications or indications")
                        skipOperation()
                        return
                    }
                }
                @SuppressLint("MissingPermission")
                if (!gatt.setCharacteristicNotification(characteristic, true)) {
                    Log.e(TAG, "setCharacteristicNotification(true) failed for $characteristicUuid")
                    skipOperation()
                    return
                }
                if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
                    @SuppressLint("MissingPermission")
                    gatt.writeDescriptor(cccd, payload)
                } else {
                    @Suppress("DEPRECATION")
                    cccd.value = payload
                    @SuppressLint("MissingPermission")
                    @Suppress("DEPRECATION")
                    gatt.writeDescriptor(cccd)
                }
            }
            is DisableNotifications -> with(operation) {
                val gatt = deviceGattMap[device]
                val characteristic = gatt?.findCharacteristic(characteristicUuid)
                if (gatt == null || characteristic == null) {
                    Log.e(TAG, "Cannot disable notifications for $characteristicUuid")
                    skipOperation()
                    return
                }
                val cccd = characteristic.getDescriptor(BleConstants.CCCD_UUID)
                if (cccd == null) {
                    Log.e(TAG, "$characteristicUuid has no CCCD descriptor")
                    skipOperation()
                    return
                }
                @SuppressLint("MissingPermission")
                if (!gatt.setCharacteristicNotification(characteristic, false)) {
                    Log.e(TAG, "setCharacteristicNotification(false) failed for $characteristicUuid")
                    skipOperation()
                    return
                }
                if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
                    @SuppressLint("MissingPermission")
                    gatt.writeDescriptor(cccd, BluetoothGattDescriptor.DISABLE_NOTIFICATION_VALUE)
                } else {
                    @Suppress("DEPRECATION")
                    cccd.value = BluetoothGattDescriptor.DISABLE_NOTIFICATION_VALUE
                    @SuppressLint("MissingPermission")
                    @Suppress("DEPRECATION")
                    gatt.writeDescriptor(cccd)
                }
            }
            is ConnectionPriorityRequest -> with(operation) {
                val gatt = deviceGattMap[device]
                if (gatt != null) {
                    @SuppressLint("MissingPermission")
                    val accepted = gatt.requestConnectionPriority(priority)
                    Log.i(TAG, "requestConnectionPriority($priority) for ${device.address}: accepted=$accepted")
                } else {
                    Log.e(TAG, "requestConnectionPriority: no GATT for ${device.address}")
                }
                // No callback exists for this operation, release the queue immediately
                skipOperation()
            }
            is PhyRequest -> with(operation) {
                val gatt = deviceGattMap[device]
                if (gatt != null && Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
                    @SuppressLint("MissingPermission")
                    gatt.setPreferredPhy(txPhy, rxPhy, phyOptions)
                    // onPhyUpdate signals completion
                } else {
                    Log.w(TAG, "setPreferredPhy: ${if (gatt == null) "no GATT" else "requires API 26+"} for ${device.address}")
                    skipOperation()
                }
            }
            is NotifyCharacteristicChange -> with(operation) {
                if (!isSubscribed(device)){
                    Log.e(TAG, "$device is no longer subscribed to the Gatt server, skipping")
                    skipOperation()
                    return
                }
                val msgChar = gattServer?.getService(BleConstants.SERVICE_UUID)
                    ?.getCharacteristic(characteristicUuid)
                if (msgChar == null) {
                    skipOperation()
                    return
                }
                // Check the result. If the notify isn't accepted (dead binder / client vanished —
                // Android may drop the server-side disconnect callback, leaving a stale subscription),
                // advance NOW instead of waiting for an onNotificationSent that will never fire — which
                // would block the single pendingOperation slot for the full watchdog timeout (5s) per
                // chunk, stalling priority ops too. Also drop the stale subscription so the rest of this
                // message's chunks skip fast.
                var clientGone = false   // true only on a definitely dead signal, not transient congestion
                val sent = try {
                    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
                        @SuppressLint("MissingPermission")
                        val status = gattServer?.notifyCharacteristicChanged(device, msgChar, confirmation, payload)
                        // Keep the subscription only on transient congestion, any thing else
                        // means the client is gone. (There's no
                        // public ERROR_DEVICE_NOT_CONNECTED constant, so we invert: busy = keep, else drop.)
                        if (status != BluetoothStatusCodes.SUCCESS && status != BluetoothStatusCodes.ERROR_GATT_WRITE_REQUEST_BUSY) {
                            clientGone = true
                        }
                        status == BluetoothStatusCodes.SUCCESS
                    } else {
                        @Suppress("DEPRECATION")
                        msgChar.value = payload
                        @Suppress("DEPRECATION")
                        @SuppressLint("MissingPermission")
                        val ok = gattServer?.notifyCharacteristicChanged(device, msgChar, confirmation)
                        ok == true
                    }
                } catch (e: Exception) {
                    // An exception here (typically DeadObjectException) is a binder failure to the
                    // Bluetooth system service, the whole BT stack process died/restarted, not
                    // specifically this client. So skip this op but dont drop the subscription: that
                    // could cut off a still-fine device on a one-off. stack-wide death is the radio watchdog /
                    // BT state receiver's job to recover.
                    Log.e(TAG, "notify to ${device.address} threw ${e.javaClass.simpleName} (likely BT stack issue) — skipping, keeping subscription")
                    false
                }
                if (!sent) {
                    // Skip immediately either way (don't stall the slot for the 5s watchdog). Only drop
                    // the subscription when the client is actually gone, a transient busy/congestion
                    // error must not unsubscribe a still connected device.
                    Log.e(TAG, "notify to ${device.address} not delivered (clientGone=$clientGone) — skipping")
                    if (clientGone) GattServer.markClientGone(device)
                    skipOperation()
                    return
                }
            }
        }
    }

    private inline fun <reified T : BleOperationType> signalOperationComplete(device: BluetoothDevice? = null) {
        synchronized(this) {
            if (pendingOperation !is T) {
                Log.e(TAG, "Rogue callback signalled completion, ignoring: expected ${T::class.simpleName}, got $pendingOperation")
                return
            }
            if (device != null && pendingOperation?.device != device) {
                Log.e(TAG, "Rogue callback: device mismatch, expected $device, got ${pendingOperation?.device}")
                return
            }
            pendingOperation = null
            if (hasPendingOps()) executeNext() else disarmWatchdog()
        }
    }

    @Synchronized
    private fun skipOperation() {
        pendingOperation = null
        if (hasPendingOps()) executeNext() else disarmWatchdog()
    }

    /**
     * Per operation type watchdog timeout, sized to each op's real completion time so a hung op (which
     * holds the single scheduler slot and blocks all queued ops) is caught as fast as is safe.
     */
    private fun timeoutFor(operation: BleOperationType): Long = when (operation) {
        is Connect, is Disconnect -> BleConstants.CONNECTION_TIMEOUT_MS        // connects can legitimately be slow
        is ServiceDiscovery       -> BleConstants.SERVICE_DISCOVERY_TIMEOUT_MS // the one slow non connect op 1+ seconds
        else                      -> BleConstants.FAST_OP_TIMEOUT_MS           // reads/writes/notify/MTU/PHY/desc - fast
    }

    /**
     * Arm a one-shot timeout for [operation]. If it's still the pending operation when the timer
     * fires (i.e. its callback never arrived), force-advance the queue. A late callback that
     * arrives afterwards is harmlessly ignored by the rogue-callback guard in signalOperationComplete.
     */
    private fun armWatchdog(operation: BleOperationType) {
        watchdogTask?.cancel(false)
        val timeoutMs = timeoutFor(operation)
        watchdogTask = watchdog.schedule({
            synchronized(this) {
                if (pendingOperation === operation) {
                    Log.e(TAG, "Watchdog: $operation stuck >${timeoutMs}ms (dropped callback?) — force-advancing")
                    cleanupStuckOperation(operation)
                    skipOperation()
                }
            }
        }, timeoutMs, TimeUnit.MILLISECONDS)
    }

    private fun disarmWatchdog() {
        watchdogTask?.cancel(false)
        watchdogTask = null
    }

    /**
     * Release any resource a timed-out operation left dangling. The important case is a stuck
     * Connect: its BluetoothGatt holds a client interface that leaks (→ status 133 on future
     * connects) unless we close it here. We also notify listeners so ConnectionPool drops the
     * now-dead BleConnection.
     */
    @SuppressLint("MissingPermission")
    private fun cleanupStuckOperation(operation: BleOperationType) {
        if (operation is Connect) {
            deviceGattMap.remove(operation.device)?.let { gatt ->
                try { gatt.disconnect() } catch (_: Exception) {}
                try { gatt.close() } catch (_: Exception) {}
                Log.w(TAG, "Watchdog: closed leaked GATT for ${operation.device.address}")
            }
            purgeOperationsForDevice(operation.device)   // drop the now orphaned setup ops for this device
            BleScanner.noteConnectFailure(operation.device.address)   // backoff before retrying this MAC
            BleScanner.resumeAfterConnect()   // stuck connect timed out, let the scan back
            notifyListeners { onDisconnectedFromDevice(operation.device) }
        }
    }

    // --------------------------------------------------------------------------------------------
    // GATT Callback
    // --------------------------------------------------------------------------------------------

    private val gattCallback = object : BluetoothGattCallback() {
        // Callback only for a CENTRAL connection
        @SuppressLint("MissingPermission")
        override fun onConnectionStateChange(gatt: BluetoothGatt, status: Int, newState: Int) {
            val address = gatt.device.address
            if (status == BluetoothGatt.GATT_SUCCESS) {
                when (newState) {
                    BluetoothProfile.STATE_CONNECTED -> {
                        Log.i(TAG, "Connected to $address")
                        deviceGattMap[gatt.device] = gatt
                        BleScanner.noteConnectSuccess(address)   // clear any reconnect backoff
                        BleScanner.resumeAfterConnect()   // link established, scan can resume
                        signalOperationComplete<Connect>(gatt.device)
                        // Service discovery is the first step after connecting
                    }
                    BluetoothProfile.STATE_DISCONNECTED -> {
                        Log.i(TAG, "Disconnected from $address")
                        deviceGattMap.remove(gatt.device)
                        gatt.close()
                        purgeOperationsForDevice(gatt.device)   // drop stale ops for a peer that just dropped
                        signalOperationComplete<Disconnect>(gatt.device)
                        notifyListeners { onDisconnectedFromDevice(gatt.device) }
                    }
                }
            } else {
                Log.e(TAG, "Connection error $status for $address")
                deviceGattMap.remove(gatt.device)
                gatt.close()
                purgeOperationsForDevice(gatt.device)   // drop stale ops for the now dead link
                BleScanner.noteConnectFailure(address)   // exponential backoff before retrying this MAC
                BleScanner.resumeAfterConnect()   // connect failed, let the scan back (debounced)
                if (pendingOperation is Connect || pendingOperation is Disconnect) {
                    skipOperation()
                }
                notifyListeners { onDisconnectedFromDevice(gatt.device) }
            }
        }

        @SuppressLint("MissingPermission")
        override fun onServicesDiscovered(gatt: BluetoothGatt, status: Int) {
            if (status == BluetoothGatt.GATT_SUCCESS) {
                Log.i(TAG, "Discovered ${gatt.services.size} services for ${gatt.device.address}")
                gatt.printGattTable()
                notifyListeners { onServicesDiscovered(gatt.device) }
            } else {
                Log.e(TAG, "Service discovery failed for ${gatt.device.address}, status: $status")
                scheduleOperation(Disconnect(gatt.device))
            }
            signalOperationComplete<ServiceDiscovery>(gatt.device)
        }
        // Only central gets this currently, can peripheral call phy update?
        override fun onPhyUpdate(gatt: BluetoothGatt, txPhy: Int, rxPhy: Int, status: Int) {
            fun phyName(phy: Int) = when (phy) {
                BluetoothDevice.PHY_LE_1M -> "1M"
                BluetoothDevice.PHY_LE_2M -> "2M"
                BluetoothDevice.PHY_LE_CODED -> "Coded"
                else -> "unknown($phy)"
            }
            if (status == BluetoothGatt.GATT_SUCCESS) {
                Log.i(TAG, "PHY updated for ${gatt.device.address}: TX=${phyName(txPhy)}, RX=${phyName(rxPhy)}")
                notifyListeners { onPhyUpdated(gatt.device, txPhy, rxPhy) }
            } else {
                Log.e(TAG, "PHY update failed for ${gatt.device.address}, status=$status — device may not support 2M PHY")
            }
            signalOperationComplete<PhyRequest>(gatt.device)
        }

        override fun onMtuChanged(gatt: BluetoothGatt, mtu: Int, status: Int) {
            if (status == BluetoothGatt.GATT_SUCCESS) {
                Log.i(TAG, "MTU changed to $mtu for ${gatt.device.address}")
                notifyListeners { onMtuChanged(gatt.device, mtu) }
                // Connection is fully set up — notify listeners - later l2capp could be checked next
                notifyListeners { onConnectionSetupComplete(gatt) }
            } else {
                Log.e(TAG, "MTU request failed for ${gatt.device.address}, status: $status")
            }
            signalOperationComplete<MtuRequest>(gatt.device)
        }

        @Deprecated("Deprecated for Android 13+")
        @Suppress("DEPRECATION")
        override fun onCharacteristicRead(
            gatt: BluetoothGatt,
            characteristic: BluetoothGattCharacteristic,
            status: Int
        ) {
            if (Build.VERSION.SDK_INT < Build.VERSION_CODES.TIRAMISU) {
                processCharacteristicRead(gatt, characteristic, characteristic.value ?: byteArrayOf(), status)
            }
        }

        override fun onCharacteristicRead(
            gatt: BluetoothGatt,
            characteristic: BluetoothGattCharacteristic,
            value: ByteArray,
            status: Int
        ) {
            processCharacteristicRead(gatt, characteristic, value, status)
        }

        private fun processCharacteristicRead(
            gatt: BluetoothGatt,
            characteristic: BluetoothGattCharacteristic,
            value: ByteArray,
            status: Int
        ) {
            if (status == BluetoothGatt.GATT_SUCCESS) {
                Log.i(TAG, "Read ${characteristic.uuid}: ${value.toHexString()}")
                notifyListeners { onCharacteristicRead(gatt.device, characteristic, value) }
            } else {
                Log.e(TAG, "Read failed for ${characteristic.uuid}, status: $status")
            }
            signalOperationComplete<CharacteristicRead>(gatt.device)

        }

        override fun onCharacteristicWrite(
            gatt: BluetoothGatt,
            characteristic: BluetoothGattCharacteristic,
            status: Int
        ) {
            if (status != BluetoothGatt.GATT_SUCCESS) {
                Log.e(TAG, "Write failed for ${characteristic.uuid}, status: $status")
            }
            signalOperationComplete<CharacteristicWrite>(gatt.device)
        }

        @Deprecated("Deprecated for Android 13+")
        @Suppress("DEPRECATION")
        override fun onCharacteristicChanged(
            gatt: BluetoothGatt,
            characteristic: BluetoothGattCharacteristic
        ) {
            if (Build.VERSION.SDK_INT < Build.VERSION_CODES.TIRAMISU) {
                processCharacteristicChanged(gatt, characteristic, characteristic.value ?: byteArrayOf())
            }
        }

        override fun onCharacteristicChanged(
            gatt: BluetoothGatt,
            characteristic: BluetoothGattCharacteristic,
            value: ByteArray
        ) {
            processCharacteristicChanged(gatt, characteristic, value)
        }

        private fun processCharacteristicChanged(
            gatt: BluetoothGatt,
            characteristic: BluetoothGattCharacteristic,
            value: ByteArray
        ) {
            // Notifications are unsolicited — they do NOT signal end of a pending operation.
            // Hot path: fires on every notification received. The per-chunk toHexString + logcat
            // write throttles the notification-receive rate, which backpressures the remote
            // peripheral's send rate (this was the cause of the ~100 kbps peripheral-send case vs
            // ~690 kbps central-send). Re-enable for debugging only.
            //Log.i(TAG, "Notification on ${characteristic.uuid}: ${value.toHexString()}")
            notifyListeners { onNotificationReceived(gatt.device, characteristic, value) }
        }

        @Deprecated("Deprecated for Android 13+")
        @Suppress("DEPRECATION")
        override fun onDescriptorRead(
            gatt: BluetoothGatt,
            descriptor: BluetoothGattDescriptor,
            status: Int
        ) {
            if (Build.VERSION.SDK_INT < Build.VERSION_CODES.TIRAMISU) {
                processDescriptorRead(gatt, descriptor, descriptor.value ?: byteArrayOf(), status)
            }
        }

        override fun onDescriptorRead(
            gatt: BluetoothGatt,
            descriptor: BluetoothGattDescriptor,
            status: Int,
            value: ByteArray
        ) {
            processDescriptorRead(gatt, descriptor, value, status)
        }

        private fun processDescriptorRead(
            gatt: BluetoothGatt,
            descriptor: BluetoothGattDescriptor,
            value: ByteArray,
            status: Int
        ) {
            if (status == BluetoothGatt.GATT_SUCCESS) {
                Log.i(TAG, "Read descriptor ${descriptor.uuid}: ${value.toHexString()}")
            } else {
                Log.e(TAG, "Descriptor read failed for ${descriptor.uuid}, status: $status")
            }
            signalOperationComplete<DescriptorRead>(gatt.device)
        }

        override fun onDescriptorWrite(
            gatt: BluetoothGatt,
            descriptor: BluetoothGattDescriptor,
            status: Int
        ) {
            if (status == BluetoothGatt.GATT_SUCCESS) {
                Log.i(TAG, "Wrote descriptor ${descriptor.uuid}")
                if (descriptor.uuid == BleConstants.CCCD_UUID) {
                    val characteristic = descriptor.characteristic
                    when (pendingOperation) {
                        is EnableNotifications -> {
                            notifyListeners { onNotificationsEnabled(gatt.device, characteristic) }
                            signalOperationComplete<EnableNotifications>(gatt.device)
                        }
                        is DisableNotifications -> {
                            notifyListeners { onNotificationsDisabled(gatt.device, characteristic) }
                            signalOperationComplete<DisableNotifications>(gatt.device)
                        }
                        else -> {}
                    }
                } else{
                    signalOperationComplete<DescriptorWrite>(gatt.device)
                }
            } else {
                Log.e(TAG, "Descriptor write failed for ${descriptor.uuid}, status: $status")
            }
        }
    }

    // Gatt server callback helpers

    fun notificationSent(device: BluetoothDevice, status: Int) {
        if (status == BluetoothGatt.GATT_SUCCESS) {
            signalOperationComplete<NotifyCharacteristicChange>(device)
        } else {
            Log.e(TAG, "Notification send failed for ${device.address}, status: $status")
            signalOperationComplete<NotifyCharacteristicChange>(device)
        }
    }


    // --------------------------------------------------------------------------------------------
    // Private helpers
    // --------------------------------------------------------------------------------------------

    private fun BluetoothGatt.findCharacteristic(uuid: UUID): BluetoothGattCharacteristic? {
        return getService(BleConstants.SERVICE_UUID)?.getCharacteristic(uuid)
    }

    private fun BluetoothGatt.findDescriptor(uuid: UUID): BluetoothGattDescriptor? {
        services.forEach { service ->
            service.characteristics.forEach { char ->
                char.descriptors.firstOrNull { it.uuid == uuid }?.let { return it }
            }
        }
        return null
    }
}