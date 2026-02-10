// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

package net.qaul.ble.service

import android.annotation.SuppressLint
import android.bluetooth.*
import android.bluetooth.le.*
import android.content.Context
import android.content.Intent
import android.content.SharedPreferences
import android.os.Handler
import android.os.Looper
import android.os.ParcelUuid
import android.util.Log
import androidx.lifecycle.LifecycleService
import net.qaul.ble.AppLog
import net.qaul.ble.BLEUtils
//import net.qaul.ble.service.GattMessaging
import net.qaul.ble.service.SendQueue
import net.qaul.ble.service.ReceiveQueue
import net.qaul.ble.RemoteLog
import net.qaul.ble.core.BleActor
import net.qaul.ble.model.BLEScanDevice
//import net.qaul.ble.model.Message
import net.qaul.ble.model.FlowControlMessageType
import net.qaul.ble.model.FlowControlQueueMessage
import net.qaul.ble.model.MissingChunkQueueMessage
import java.util.*
import java.util.concurrent.CopyOnWriteArrayList
import java.util.concurrent.Executors

/**
 * Android Lifecycle Service class of the BLE Service
 */
@SuppressLint("MissingPermission")
class BleService : LifecycleService() {
    var bleCallback: BleScanCallBack? = null
    private val TAG: String = "qaul-blemodule BleService"
    private var bluetoothAdapter: BluetoothAdapter? = null
    private var bleAdvertiseCallback: BleAdvertiseCallback? = null
    private var qaulId: ByteArray? = null
    private var advertMode = ""
    private var bluetoothLeAdvertiser: BluetoothLeAdvertiser? = null
    private var gattServer: BluetoothGattServer? = null
    private var bluetoothManager: BluetoothManager? = null

    private lateinit var scanCallback: ScanCallback
    private lateinit var bleScanner: BluetoothLeScanner
    private val outOfRangeChecker = Handler(Looper.getMainLooper())
    private val handler = Handler(Looper.getMainLooper())
    private val sharedPrefFile = "sharedpreference_qaul_ble"
    private lateinit var sharedPreferences: SharedPreferences
    private var lastWriteTime = System.currentTimeMillis() + 60000
    private var executor = Executors.newSingleThreadExecutor()

    // qaul BLE UUID service list for device scanning
    private val uuidList = arrayListOf<ParcelUuid>()
    private var filters: ArrayList<ScanFilter> = arrayListOf()
    private var scanSettings: ScanSettings? = null

    // detected devices collections
    private val devicesList = CopyOnWriteArrayList(arrayListOf<BLEScanDevice>())
    private val ignoreList = CopyOnWriteArrayList(arrayListOf<BLEScanDevice>())
    private val receiveList = Collections.synchronizedList(arrayListOf<BLEScanDevice>())
    // ??? is `blackList` even used?
    private val blackList = Collections.synchronizedList(arrayListOf<BLEScanDevice>())

    // Active GATT client connection collection for outgoing GATT messages
    private val actorMap = Collections.synchronizedMap(hashMapOf<String, BleActor>())

    // OLD: incoming GATT message collection
    //private val msgMap = Collections.synchronizedMap(hashMapOf<String, String>())
    // OLD: GATT message sending queue (identified by BLE MAC address)
    //private val hashMap: HashMap<String, Queue<Triple<String, ByteArray, ByteArray>>> = hashMapOf()

    // NEW: SendQueue for each qaul device
    //private val hashMapSendQueue: HashMap<String, SendQueue> = hashMapOf()
    private val hashMapSendQueue = Collections.synchronizedMap(mutableMapOf<String, SendQueue>())
    // NEW: ReceiveQueue for each qaul device
    private val hashMapReceiveQueue = Collections.synchronizedMap(mutableMapOf<String, ReceiveQueue>())
    // NEW: ConnectionManager to relate receiving and sending queues
    val connectionManager: ConnectionManager = ConnectionManager()

    companion

    /**
    * SERVICE_UUID is the main service advertised by GATT Server
    * MSG_SERVICE_UUID is not used for normal advertisements(In current architecture)
    * It is used with extended advertisements (Future Goals).
    * READ_CHAR and MSG_CHAR are the characteristics inside SERVICE_UUID.
    * READ_CHAR stores qaul ID for other device to read and confirm qaul device.
    * MSG_CHAR is used to send and receive messages.
    * GD_CHAR is unused right now.
    */
    object {
        var bleService: BleService? = null
        var isAdvertisementRunning = false
        var isScanningRunning = false
        val SERVICE_UUID = "4db14399-0bd0-4445-9906-47d9c4791cff"
        val MSG_SERVICE_UUID = "4db14400-0bd0-4445-9906-47d9c4791cff"
        val READ_CHAR = "4db14401-0bd0-4445-9906-47d9c4791cff"
        val MSG_CHAR = "4db14402-0bd0-4445-9906-47d9c4791cff"
        val GD_CHAR = "4db14403-0bd0-4445-9906-47d9c4791cff"
    }

    override fun onCreate() {
        super.onCreate()
        bleService = this
        sharedPreferences = this.getSharedPreferences(sharedPrefFile, Context.MODE_PRIVATE)

        AppLog.e(TAG, "$TAG created")
    }

    override fun onStart(intent: Intent?, startId: Int) {
        super.onStart(intent, startId)
        AppLog.e(TAG, "$TAG started")
    }

    fun getRandomString(length: Int): String {
        val allowedChars = ('A'..'Z') + ('a'..'z') + ('0'..'9')
        return (1..length).map { allowedChars.random() }.joinToString("")
    }

    /**
     * This Method will set the necessary data and start the advertisement
     */
    fun startAdvertise(
        qaul_id: ByteArray, mode: String, bleCallback: BleAdvertiseCallback,
    ) {
        val name = getRandomString(5)
        bleService?.qaulId = qaul_id
        bleService?.advertMode = mode
        bleService?.bleAdvertiseCallback = bleCallback

        Executors.newSingleThreadExecutor().execute {
            bluetoothManager = bleService!!.getSystemService(BLUETOOTH_SERVICE) as BluetoothManager
            bluetoothAdapter = bluetoothManager!!.adapter
            bluetoothAdapter!!.name = name
            bluetoothLeAdvertiser = bluetoothAdapter!!.bluetoothLeAdvertiser
            if (bluetoothAdapter != null) {
                AppLog.e(
                    TAG, "Peripheral supported"
                )
                val mainService = BluetoothGattService(
                    UUID.fromString(SERVICE_UUID), BluetoothGattService.SERVICE_TYPE_PRIMARY
                )
                val mainChar = BluetoothGattCharacteristic(
                    UUID.fromString(READ_CHAR),
                    BluetoothGattCharacteristic.PROPERTY_READ,
                    BluetoothGattCharacteristic.PERMISSION_READ
                )
                AppLog.e(TAG, "qaulId : " + BLEUtils.byteToHex(qaul_id))
                mainChar.value = qaulId
                mainService.addCharacteristic(mainChar)

                val msgChar = BluetoothGattCharacteristic(
                    UUID.fromString(MSG_CHAR),
                    BluetoothGattCharacteristic.PROPERTY_WRITE,
                    BluetoothGattCharacteristic.PERMISSION_WRITE
                )
                mainService.addCharacteristic(msgChar)
                val serviceList = arrayListOf<BluetoothGattService>()
                serviceList.add(mainService)
                startGattServer(services = serviceList)

                val dataBuilder = AdvertiseData.Builder()
                val settingsBuilder = AdvertiseSettings.Builder()
                dataBuilder.setIncludeTxPowerLevel(true)

                val main_uuid = ParcelUuid(UUID.fromString(SERVICE_UUID))
                dataBuilder.addServiceUuid(main_uuid)
                dataBuilder.setIncludeDeviceName(true)
                when (advertMode) {
                    "low_power" -> {
                        settingsBuilder.setAdvertiseMode(AdvertiseSettings.ADVERTISE_MODE_LOW_POWER)
                    }

                    "balanced" -> {
                        settingsBuilder.setAdvertiseMode(AdvertiseSettings.ADVERTISE_MODE_BALANCED)
                    }

                    "low_latency" -> {
                        settingsBuilder.setAdvertiseMode(AdvertiseSettings.ADVERTISE_MODE_LOW_LATENCY)
                    }

                    "UNRECOGNIZED" -> {
                        settingsBuilder.setAdvertiseMode(AdvertiseSettings.ADVERTISE_MODE_LOW_LATENCY)
                    }
                }
                settingsBuilder.setTxPowerLevel(AdvertiseSettings.ADVERTISE_TX_POWER_HIGH)
                settingsBuilder.setConnectable(true)

                try {
                    bluetoothLeAdvertiser!!.startAdvertising(
                        settingsBuilder.build(), dataBuilder.build(), advertiseCallback
                    )
                } catch (e: Exception) {
                    e.printStackTrace()
                }
            } else {
                AppLog.e(
                    TAG, "Peripheral not supported"
                )
            }
        }
    }

    /**
     * This Method Will Start the Service
     */
    fun start(
        context: Context,
    ) {
        if (bleService == null) {
            val intent = Intent(context, BleService::class.java)
            context.startService(intent)
        } else {
            AppLog.e(TAG, "$TAG already started")
        }
    }

    /**
     * This Method Will Return True if Service is Running
     */
    fun isRunning(): Boolean {
        return bleService != null
    }

    /**
     * This Method Will Return True if Advertisement is ON
     */
    fun isAdvertiserRunning(): Boolean {
        return isAdvertisementRunning
    }

    /**
     * This Method Will Stop the Service if It Is Running
     */
    fun stop() {
        if (bleService != null) {
            var str = "$TAG stopped"
            bleService?.outOfRangeChecker?.removeCallbacks(outRangeRunnable)
            if (bleService!!.isAdvertiserRunning()) {
                bluetoothLeAdvertiser?.stopAdvertising(advertiseCallback)
                gattServer?.clearServices()
                gattServer?.close()
            }
            if (bleService!!.isScanRunning()) {
                stopScan()
            }

            bleService?.stopSelf()
        } else {
            bleAdvertiseCallback?.stopAdvertiseRes(
                status = false, errorText = "$TAG not started"
            )
            bleCallback?.stopScanRes(status = false, errorText = "")
            AppLog.e(TAG, "$TAG not started")
        }
    }

    /**
     * This Method Will Be Called When Scanning Is Failed
     */
    private fun onScanfailed(errorCode: Int) {
        var unknownError = false
        isScanningRunning = false
        var errorText = ""
        if (errorCode < 1 || errorCode > 4) {
            unknownError = true
        }
        when (errorCode) {
            1 -> {
                errorText = "SCAN_FAILED_ALREADY_STARTED"
            }

            2 -> {
                errorText = "SCAN_FAILED_APPLICATION_REGISTRATION_FAILED"
            }

            3 -> {
                errorText = "SCAN_FAILED_INTERNAL_ERROR"
            }

            4 -> {
                errorText = "SCAN_FAILED_FEATURE_UNSUPPORTED"
            }
        }
        val failMsg = "Scanning failed: $errorText"
        AppLog.e(TAG, failMsg)
        bleService?.bleCallback?.startScanRes(
            status = false, errorText = failMsg, unknownError = unknownError
        )
    }

    /**
     * This Method Will Set Filter of UUID for Scanning
     */
    private fun setFilter(uuidList: ArrayList<ParcelUuid>) {
        for (uuid in uuidList) {
            filters.add(
                ScanFilter.Builder().setServiceUuid(uuid).build()
            )
        }
    }

    /**
     * This Method Will Parse Result of ScanResult according to Device
     */
    private fun parseBLEFrame(device: BluetoothDevice, rssi: Int, result: ScanResult) {
        if (blackList.find { it.macAddress == device.address } == null) {
            val selectItem = devicesList.toMutableList().find { it.macAddress == device.address }
            if (selectItem == null) {
                RemoteLog[this]!!.addDebugLog("$TAG:device : " + device.address + " " + result.scanRecord!!.serviceUuids)
                val bleDevice: BLEScanDevice = BLEScanDevice.getDevice()
                bleDevice.bluetoothDevice = device
                bleDevice.scanResult = result
                bleDevice.name = device.name
                bleDevice.deviceRSSI = rssi
                bleDevice.macAddress = device.address
                bleDevice.isConnectable = result.isConnectable
                bleDevice.lastFoundTime = System.currentTimeMillis()
                devicesList.add(bleDevice)

                if (result.isConnectable) {
                    connectDevice(bleDevice, isFromMessage = false)
                }
            } else {
                ignoreList.find { it.macAddress == device.address }?.lastFoundTime = System.currentTimeMillis()
            }
        } else {
            AppLog.e(TAG, "zzz device blacklisted: " + device.address)
        }
    }

    /**
     * This Method Will Be called When any new Device will found
     */
    private fun deviceFound(bleDevice: BLEScanDevice, byteArray: ByteArray) {
        bleDevice.qaulId = byteArray
        bleCallback?.deviceFound(bleDevice = bleDevice)
    }

    /**
     * This method will stop the scanning
     */
    fun stopScan() {
        AppLog.e(TAG, "stopScan()")
        isScanningRunning = false
        try {
            if (::bleScanner.isInitialized) {
                bleScanner.stopScan(scanCallback)
            }
        } catch (ex: UninitializedPropertyAccessException) {
            ex.printStackTrace()
        }
        RemoteLog[this]!!.addDebugLog("$TAG:Scanning Stopped")
    }


    /**
     * This Method Will Start the GattServer.
     */
    private fun startGattServer(services: ArrayList<BluetoothGattService>) {
        gattServer = bluetoothManager!!.openGattServer(
            this, gattServerCallback
        )
        gattServer?.addService(services[0])
        // gattServer?.addService(services[1])
    }

    /**
     * This is a Object of a BluetoothGattServer with it's Callback.
     */
    private var gattServerCallback: BluetoothGattServerCallback =
        object : BluetoothGattServerCallback() {
            override fun onConnectionStateChange(
                device: BluetoothDevice, status: Int, newState: Int,
            ) {
                super.onConnectionStateChange(device, status, newState)
            }

            override fun onServiceAdded(status: Int, service: BluetoothGattService) {
                super.onServiceAdded(status, service)
            }

            override fun onCharacteristicReadRequest(
                device: BluetoothDevice,
                requestId: Int,
                offset: Int,
                characteristic: BluetoothGattCharacteristic,
            ) {
                super.onCharacteristicReadRequest(
                    device, requestId, offset, characteristic
                )
                AppLog.e(TAG, "Request Received : " + qaulId?.size)
                gattServer?.sendResponse(
                    device, requestId, 0, 0, getStoredValue(characteristic)
                )
            }

            private fun getStoredValue(characteristic: BluetoothGattCharacteristic): ByteArray {
                val `val` = ByteArray(characteristic.value.size)
                System.arraycopy(
                    characteristic.value, 0, `val`, 0, characteristic.value.size
                )   

                return `val`
            }

            /**
            * This method receives write request from the client and manages the bytes written.
            * The android gatt client has defaults message length per version:
            * 
            * - Android 13 and below: 20 bytes
            * - Android 14 and above: 512 bytes
            *
            * Every message is handled by the ReceiveQueue class which manages chunking, reassembly,
            * and flow control.
            */
            override fun onCharacteristicWriteRequest(
                device: BluetoothDevice,
                requestId: Int,
                characteristic: BluetoothGattCharacteristic,
                preparedWrite: Boolean,
                responseNeeded: Boolean,
                offset: Int,
                value: ByteArray,
            ) {
                super.onCharacteristicWriteRequest(
                    device, requestId, characteristic, preparedWrite, responseNeeded, offset, value
                )
                val b = BLEUtils.toBinaryString(value)

                // check if the device is in ignoreList or receiveList
                var bleDevice = ignoreList.find { it.macAddress == device.address }
                if (bleDevice == null) {
                    bleDevice = receiveList.find { it.macAddress == device.address }
                }
                // TODO: is a response really needed?
                // TODO: test without response and faster message send time
                gattServer!!.sendResponse(
                    device, requestId, BluetoothGatt.GATT_SUCCESS, offset, value
                )

                // add the 
                var receiveQueue: ReceiveQueue? = null
                if (hashMapReceiveQueue.containsKey(device.address)) {
                    AppLog.e(TAG, "hashMapReceiveQueue found ${device.address}")
                    receiveQueue = hashMapReceiveQueue[device.address]
                } else {
                    AppLog.e(TAG, "hashMapReceiveQueue new queue created for ${device.address}")
                    receiveQueue = ReceiveQueue()
                }

                // Process received data with the ReceiveQueue
                val receiveQueueResult = receiveQueue?.incomingMessage(value, device)
                hashMapReceiveQueue[device.address] = receiveQueue

                // check if we received an ACK
                if (receiveQueueResult!!.flcAckReceived != null) {
                    AppLog.e(TAG, "receiveQueueResult ACK received: Queue: ${receiveQueueResult!!.flcAckReceived!!.messageIndex}, Success: ${receiveQueueResult!!.flcAckReceived!!.success}, ErrorCode: ${receiveQueueResult!!.flcAckReceived!!.errorCode}")

                    // notify libqaul about message sending result
                    // TODO: I guess we can't do it here, because we haven't the message ID.
                    //bleCallback?.onMessageSent(
                    //    id = messageId,
                    //    success = receiveQueueResult.flcAckReceived!!.success,
                    //    data = ByteArray(0)
                    //)
                }

                // send receiveQueueResult to ConnectionManager to relate it with the SendQueue
                // check if qaulId received
                if (receiveQueueResult!!.qaulIdReceived != null) {
                    val qaulIdReceivedHex = BLEUtils.byteToHex(receiveQueueResult.qaulIdReceived!!)
                    val qaulIdSavedHex = BLEUtils.byteToHex(bleDevice?.qaulId)
                    AppLog.e(TAG, "receiveQueueResult QAUL ID received")
                    AppLog.e(TAG, "receiveQueueResult: $qaulIdReceivedHex , $qaulIdSavedHex , ${receiveQueueResult.qaulIdReceived!!} , ${bleDevice?.qaulId}")
                    bleDevice?.qaulId = receiveQueueResult.qaulIdReceived!!

                    // Add ReceiveQueueResult handling by the ConnectionManager below
                    connectionManager.addReceiveQueueResult(receiveQueueResult)
                } else {
                    AppLog.e(TAG, "receiveQueueResult QAUL ID not received")
                }

                // check if we received a message
                if (receiveQueueResult.receivedMessage != null) {
                    AppLog.e(TAG, "receiveQueueResult message successfully received")
                    // Send message to libqaul
                    bleAdvertiseCallback?.onMessageReceived(
                        qaulId = receiveQueueResult.receivedMessage!!.qaulId,
                        message = receiveQueueResult.receivedMessage!!.message
                    )
                }



                /*
                // check for result tasks
                // check if qaulId is missing
                // TODO: the following 3 checks need to be done here properly
                if (receiveQueueResult!!.qaulIdMissing) {
                    AppLog.e(TAG, "receiveQueueResult QAUL ID is missing")
                }

                // check if qaulId request received
                if (receiveQueueResult!!.qaulIdRequestReceived) {
                    AppLog.e(TAG, "receiveQueueResult FLC QAUL ID request received")

                    // update the sendQueue
                    val sendQueue = hashMapSendQueue[device.address]
                    if (sendQueue != null) {
                        AppLog.e(TAG, "schedule qaul ID FLC sending")
                        if (sendQueue.qaulIdSent) {
                            AppLog.e(TAG, "QAUL ID already sent to device ${device.address}")
                            sendQueue.addFlcSendQaulId()
                        }
                        else {
                            AppLog.e(TAG, "QAUL ID not yet sent to device ${device.address}")
                            sendQueue.addFlcSendQaulId()
                        }
                        hashMapSendQueue[device.address] = sendQueue
                    } else {
                        // what if there is no sendQueue yet?
                        AppLog.e(TAG, "received qaul ID request: No sendQueue for device ${device.address}")
                    }
                }
                */

                /*
                // ONLY HERE FOR REFERENCE, DELETE THE FOLLOWING IN THE FUTURE
                // check if we need to request chunks
                if (receiveQueueResult.flcRequestChunks.size > 0) {
                    AppLog.e(TAG, "receiveQueueResult chunks requested")

                    // add missing chunks to sendQueue
                    val sendQueue = hashMapSendQueue[device.address]
                    if (sendQueue != null) {
                        for (missingChunk in receiveQueueResult.flcRequestChunks) {
                            sendQueue.addMissingChunkIndexToRequest(missingChunk)
                        }
                        hashMapSendQueue[device.address] = sendQueue
                    }
                }
                // check if we need to send an ACK
                if (receiveQueueResult.flcSendAck != null) {
                    AppLog.e(TAG, "receiveQueueResult send FLC ACK")

                    // schedule ACK FLC in sendQueue
                    val sendQueue = hashMapSendQueue[device.address]
                    if (sendQueue != null) {
                        sendQueue.addFlcAck(
                            receiveQueueResult.flcSendAck!!.messageIndex,
                            receiveQueueResult.flcSendAck!!.success,
                            receiveQueueResult.flcSendAck!!.errorCode
                        )
                        hashMapSendQueue[device.address] = sendQueue
                        AppLog.e(TAG, "send FLC ACK scheduled: Queue: ${receiveQueueResult.flcSendAck!!.messageIndex}, Success: ${receiveQueueResult.flcSendAck!!.success}, ErrorCode: ${receiveQueueResult.flcSendAck!!.errorCode}")
                    } else {
                        AppLog.e(TAG, "send FLC ACK: No sendQueue for device ${device.address}")
                    }
                }
                // check if we need to request an ACK
                if (receiveQueueResult.flcRequestAck != null) {
                    AppLog.e(TAG, "receiveQueueResult request ACK")
                }
                // check if we received an ACK
                if (receiveQueueResult.flcAckReceived != null) {
                    AppLog.e(TAG, "receiveQueueResult ACK received: Queue: ${receiveQueueResult.flcAckReceived!!.messageIndex}, Success: ${receiveQueueResult.flcAckReceived!!.success}, ErrorCode: ${receiveQueueResult.flcAckReceived!!.errorCode}")

                    // update the sendQueue with the ACK received
                    val sendQueue = hashMapSendQueue[device.address]
                    if (sendQueue != null) {
                        val messageId = sendQueue.flcAckReceived(receiveQueueResult.flcAckReceived!!.messageIndex, receiveQueueResult.flcAckReceived!!.success, receiveQueueResult.flcAckReceived!!.errorCode)
                        hashMapSendQueue[device.address] = sendQueue

                        // notify libqaul about message sending result
                        bleCallback?.onMessageSent(
                            id = messageId,
                            success = receiveQueueResult.flcAckReceived!!.success,
                            data = ByteArray(0)
                        )
                    } else {
                        AppLog.e(TAG, "ACK received: No sendQueue for device ${device.address}")
                    }
                }
                */
            }

            // Descriptor related methods (Useless for current architecture)
            override fun onDescriptorReadRequest(
                device: BluetoothDevice,
                requestId: Int,
                offset: Int,
                descriptor: BluetoothGattDescriptor,
            ) {
                super.onDescriptorReadRequest(device, requestId, offset, descriptor)
                AppLog.e(TAG, "onDescriptorReadRequest()")
            }

            // Descriptor related methods (Useless for current architecture)
            override fun onDescriptorWriteRequest(
                device: BluetoothDevice,
                requestId: Int,
                descriptor: BluetoothGattDescriptor,
                preparedWrite: Boolean,
                responseNeeded: Boolean,
                offset: Int,
                value: ByteArray,
            ) {
                super.onDescriptorWriteRequest(
                    device, requestId, descriptor, preparedWrite, responseNeeded, offset, value
                )
                AppLog.e(TAG, "onDescriptorWriteRequest()")
                gattServer?.sendResponse(
                    device, requestId, BluetoothGatt.GATT_SUCCESS, offset, value
                )
            }

            override fun onExecuteWrite(device: BluetoothDevice, requestId: Int, execute: Boolean) {
                super.onExecuteWrite(device, requestId, execute)
            }

            // Not if related methods (Useless for current architecture)
            override fun onNotificationSent(device: BluetoothDevice, status: Int) {
                super.onNotificationSent(device, status)
            }
        }

    /**
     * This is a Object of a AdvertiseCallback.
     */
    private val advertiseCallback: AdvertiseCallback = object : AdvertiseCallback() {
        override fun onStartSuccess(advertiseSettings: AdvertiseSettings) {
            val successMsg = "Advertisement successful"
            isAdvertisementRunning = true
            AppLog.e(TAG, successMsg)
            bleService?.bleAdvertiseCallback?.startAdvertiseRes(
                status = true, errorText = successMsg, unknownError = false
            )
        }

        override fun onStartFailure(i: Int) {
            var unknownError = false
            isAdvertisementRunning = false
            var errorText = ""
            if (i < 1 || i > 5) {
                unknownError = true
            }
            when (i) {
                1 -> {
                    errorText = "ADVERTISE_FAILED_DATA_TOO_LARGE"
                }

                2 -> {
                    errorText = "ADVERTISE_FAILED_TOO_MANY_ADVERTISERS"
                }

                3 -> {
                    errorText = "ADVERTISE_FAILED_ALREADY_STARTED"
                }

                4 -> {
                    errorText = "ADVERTISE_FAILED_INTERNAL_ERROR"
                }

                5 -> {
                    errorText = "ADVERTISE_FAILED_FEATURE_UNSUPPORTED"
                }
            }

            val failMsg = "Advertisement failed: $errorText"
            AppLog.e(TAG, failMsg)
            bleService?.bleAdvertiseCallback?.startAdvertiseRes(
                status = false, errorText = failMsg, unknownError = unknownError
            )
        }
    }


    /**
     * This method Will be Called When Service Will Stopped/Destroyed
     */
    override fun onDestroy() {
        if (bleService != null) {
            if (bleService!!.isAdvertiserRunning()) {
                bluetoothLeAdvertiser?.stopAdvertising(advertiseCallback)
                gattServer?.clearServices()
                gattServer?.close()
            }
            bleService?.outOfRangeChecker?.removeCallbacks(outRangeRunnable)
            stopScan()
            bleAdvertiseCallback?.stopAdvertiseRes(
                status = true, errorText = "Advertisement Stopped"
            )
            bleCallback?.stopScanRes(status = true, errorText = "Scanning Stopped")
            bleService?.stopSelf()
        }
        bleService = null
        super.onDestroy()
    }

    /**
     * This Method Will Return True if Scan is Running
     */
    fun isScanRunning(): Boolean {
        return isScanningRunning
    }

    /**
     * This Method Will Set Filter, ScanMode, and Start Scanning
     */
    fun startScan(bleCallback: BleScanCallBack) {
        this.bleCallback = bleCallback
        if (bluetoothManager != null) {
            if (bluetoothAdapter != null) {
                bleScanner = bluetoothManager!!.adapter!!.bluetoothLeScanner
            } else {
                bluetoothAdapter = bluetoothManager!!.adapter
                bluetoothAdapter!!.name = "qaul"
                bleScanner = bluetoothAdapter!!.bluetoothLeScanner
            }
        } else {
            bluetoothManager = bleService!!.getSystemService(BLUETOOTH_SERVICE) as BluetoothManager
            bluetoothAdapter = bluetoothManager!!.adapter
            bluetoothAdapter!!.name = "qaul"
            bleScanner = bluetoothAdapter!!.bluetoothLeScanner
        }
        uuidList.clear()
        uuidList.add(ParcelUuid.fromString(SERVICE_UUID))
        setFilter(uuidList)
        scanCallback = object : ScanCallback() {
            override fun onScanResult(callbackType: Int, result: ScanResult?) {
                super.onScanResult(callbackType, result)

                RemoteLog[this@BleService]!!.addDebugLog("$TAG:device : " + result!!.device.address)
                parseBLEFrame(result!!.device, result.rssi, result)
            }

            override fun onScanFailed(errorCode: Int) {
                super.onScanFailed(errorCode)
                onScanfailed(errorCode)
                stopScan()
            }
        }

        // TODO: DK Change check
        scanSettings =
            ScanSettings.Builder().setScanMode(ScanSettings.SCAN_MODE_LOW_LATENCY).build()

        bleScanner.startScan(filters, scanSettings, scanCallback)
        if (!isScanRunning()) {
            bleService?.bleCallback?.startScanRes(
                status = true, errorText = "Scanning Started", unknownError = false
            )
            isScanningRunning = true
            startOutRangeChecker()
        }
    }

    /**
     * This Method Will Start Handler for Checking Device Out Of Range
     */
    private fun startOutRangeChecker() {
        outOfRangeChecker.postDelayed(outRangeRunnable, 2000)
    }

    /**
     * Object for Out range Checker
     */
    private var outRangeRunnable: Runnable = Runnable {
        if (ignoreList.isNotEmpty()) {
            for (bLEDevice in ignoreList) {
                if (bLEDevice.lastFoundTime != null && (bLEDevice.lastFoundTime!! < System.currentTimeMillis() - 5000)) {
                    bleCallback?.deviceOutOfRange(bleDevice = bLEDevice)
                    AppLog.e(TAG, " outRangeRunnable-  REMOVE HERE  -> ${bLEDevice.macAddress} ")
                    devicesList.remove(bLEDevice)
                    ignoreList.remove(bLEDevice)
//                    AppLog.d(TAG, "${bLEDevice.macAddress} out of range ${ignoreList.size}")
                } else {
//                    AppLog.e(TAG, "${bLEDevice.macAddress} Still in range")
                }
            }
        }
        startOutRangeChecker()
    }

    /**
     * This Method Will be Used to set Callback for Device Connection and Connect to Device
     */
    private fun connectDevice(device: BLEScanDevice, isFromMessage: Boolean): BleActor {
        class BleConnectionListener : BleActor.BleConnectionListener {
            override fun onConnected(macAddress: String?, device: BluetoothDevice?) {
                AppLog.e(TAG, " onConnected : $macAddress")
                // TODO: DELETE:
                //val editor: SharedPreferences.Editor = sharedPreferences.edit()
                //editor.putString("BLE_GATT", Gson().toJson(device))
                //editor.apply()
            }

            override fun onDisconnected(bleScanDevice: BLEScanDevice) {
                AppLog.e(TAG, " onDisconnected : ${bleScanDevice.macAddress}")
                if (!blackList.contains(bleScanDevice)) {
                    devicesList.remove(bleScanDevice)
//                    ignoreList.remove(bleScanDevice)
                }
            }

            override fun onServiceDiscovered(macAddress: String?) {
                AppLog.e(TAG, " onServiceDiscovered : $macAddress")
            }

            // Descriptor related methods (Useless for current architecture)
            override fun onDescriptorWrite(bleScanDevice: BLEScanDevice, bleActor: BleActor) {
                AppLog.e(TAG, " onDescriptorWrite : ${bleScanDevice.macAddress}")
                bleActor.readServiceData(SERVICE_UUID, READ_CHAR)
            }

            override fun onConnectionFailed(bleScanDevice: BLEScanDevice) {
                AppLog.e(TAG, "zzz onConnectionFailed : ${bleScanDevice.macAddress}")
                actorMap.remove(bleScanDevice.macAddress)
                Log.e(
                    TAG,
                    "REMOVE HERE onConnectionFailedDevice Mac Address:: ${bleScanDevice.macAddress}"
                )
                devicesList.remove(bleScanDevice)
            }

            override fun onCharacteristicRead(
                bleScanDevice: BLEScanDevice,
                gatt: BluetoothGatt?,
                characteristic: BluetoothGattCharacteristic?,
            ) {
                if (characteristic!!.uuid.toString().lowercase() == READ_CHAR.lowercase()) {
                    val existingDevice =
                        ignoreList.find { it.qaulId.contentEquals(characteristic.value) }
                    if (existingDevice != null) {
                        existingDevice.macAddress = bleScanDevice.macAddress
                        existingDevice.lastFoundTime = System.currentTimeMillis()
                    }
                    deviceFound(bleScanDevice, characteristic.value)
                }
            }

            override fun onCharacteristicWrite(
                gatt: BluetoothGatt?, characteristic: BluetoothGattCharacteristic?,
            ) {
                lastWriteTime = System.currentTimeMillis()
                AppLog.e("zzz lastWriteTime", "$lastWriteTime")
            }

            /**
             * This method is called when a message is sent successfully.
             * 
             * TODO: is this really needed? 
             * Maybe Better: focus on receive of ACK.
             */
            override fun onMessageSent(
                gatt: BluetoothGatt?, value: ByteArray, id: String,
            ) {
                AppLog.e(TAG, "onMessageSent:ID -> $id")
                val sendQueue: SendQueue? = hashMapSendQueue[gatt?.device?.address]
                if (sendQueue != null) {
                    sendQueue.messageSent(id)
                    hashMapSendQueue[gatt?.device?.address!!] = sendQueue
                }

                // TODO: notify Libqaul once we send an ACK
                // DON'T notify libqaul about message sent here
                // It will be notified when the ACK is received
                //bleCallback?.onMessageSent(id = id, success = true, data = value)
                sendMessageFromQueue(gatt?.device?.address!!)
            }

            override fun onCharacteristicChanged(
                macAddress: String?,
                gatt: BluetoothGatt?,
                characteristic: BluetoothGattCharacteristic?,
            ) {

            }

            override fun addToBlackList(bleScanDevice: BLEScanDevice) {
                blackList.add(bleScanDevice)
            }

            override fun addToIgnoreList(bleScanDevice: BLEScanDevice) {
                ignoreList.add(bleScanDevice)
            }

        }

        val baseBleActor: BleActor? = when {
            isFromMessage -> {
                if (actorMap[device.macAddress] == null) {
                    BleActor(this, BleConnectionListener())
                } else {
                    actorMap[device.macAddress]
                }
            }

            else -> {
                BleActor(this, BleConnectionListener())
            }
        }

        baseBleActor?.setDevice(device = device, isFromMessage = isFromMessage)
        return baseBleActor!!
    }

    // -------------------------------------------------------------------------
    // new sending functions
    // -------------------------------------------------------------------------
    /**
     * This Method Will Be Used to Send Data to Other Qaul-Device
     */
    fun sendMessage(message_id: String, to_id: ByteArray, message: ByteArray, from_id: ByteArray) {
        var bleDevice = ignoreList.find { it.qaulId.contentEquals(to_id) }
        if (bleDevice == null) {
            bleDevice = receiveList.find { it.qaulId.contentEquals(to_id) }
        }

        // DEBUG
        //AppLog.e(TAG, "sendMessage ${BLEUtils.byteToHex(message)}")
        //AppLog.e(TAG, "sendMessage to_id: ${BLEUtils.byteToHex(to_id)}")
        //AppLog.e(TAG, "sendMessage to_id: ${BLEUtils.toBinaryString(to_id)}")
        //AppLog.e(TAG, "sendMessage from_id: ${BLEUtils.byteToHex(from_id)}")
        //AppLog.e(TAG, "sendMessage from_id: ${BLEUtils.toBinaryString(from_id)}")

        bleDevice?.let {
            if (hashMapSendQueue.containsKey(it.macAddress)) {
                val sendQueue = hashMapSendQueue[it.macAddress!!]
                sendQueue!!.addMessage(message, message_id)
                hashMapSendQueue[it.macAddress!!] = sendQueue

                AppLog.e(TAG, "sendMessage: Used existing Queue for macAddress: ${it.macAddress!!}")
            } else {
                // create a new entry
                val sendQueue = SendQueue(from_id)
                sendQueue!!.addMessage(message, message_id)
                hashMapSendQueue[it.macAddress!!] = sendQueue

                AppLog.e(TAG, "sendMessage: New Queue updated for ${it.macAddress!!}")
            }
            
            sendMessageFromQueue(it.macAddress!!, true)
        }
    }

    /**
     * create and schedule chunks to send to other device
     */
    private fun sendMessageFromQueue(macAddress: String, isFromSendMessage: Boolean = false) {
        executor.execute {
            // check if queue is initialized and contains the macAddress for the connection
            if (hashMapSendQueue.isNotEmpty() && hashMapSendQueue.containsKey(macAddress)) {
                // get queue for connection
                val sendQueue = hashMapSendQueue[macAddress]
                if (sendQueue != null) {
                    AppLog.e(
                        TAG,
                        "sendMessageFromQueue ${isFromSendMessage}"
                    )
                    
                    if (!isFromSendMessage ||
                        sendQueue.messagesToSend.isNotEmpty() ||
                        sendQueue.flcToSend.isNotEmpty() ||
                        sendQueue.missingChunksToRequest.isNotEmpty()
                        ) {

                        var bleDevice = ignoreList.find { it.macAddress.contentEquals(macAddress) }
                        if (bleDevice == null) {
                            bleDevice = receiveList.find { it.macAddress.contentEquals(macAddress) }
                        }
                        // check if device is available
                        if (bleDevice == null) {
                            // TODO: check and how to proceed with this queue
                            //       - if device is connectable
                            //       - if device has changed MAC
                            AppLog.e(TAG, "bleDevice not found for macAddress: $macAddress")

                            // mark queue as connection lost
                            val messageId = sendQueue!!.setConnectionLost()
                            hashMapSendQueue[macAddress] = sendQueue

                            // notify libqaul that message sending failed
                            if (messageId != null) {
                                bleCallback?.onMessageSent(
                                    id = messageId,
                                    success = false,
                                    data = ByteArray(0)
                                )
                            }
                        } else {
                            // add info from connection manager to sendQueue
                            val connectionInfo = connectionManager.getAndRemoveSendQueue(sendQueue!!.qaulId)

                            // add ACKs to send
                            val acksToSendIterator = connectionInfo.acksToSend.iterator()
                            while (acksToSendIterator.hasNext()) {
                                AppLog.e(TAG, "sendMessageFromQueue schedule FLC ACK to send")
                                val (index, pairValue) = acksToSendIterator.next()
                                sendQueue.addFlcAck(index, pairValue.first, pairValue.second)
                            }

                            // add chunks request
                            for (missingChunk in connectionInfo.missingChunksToRequest) {
                                AppLog.e(TAG, "sendMessageFromQueue add chunk request")
                                sendQueue.addMissingChunkIndexToRequest(missingChunk)
                            }

                            // add received ACKs
                            val ackReceivedIterator = connectionInfo.acksReceived.iterator()
                            while (ackReceivedIterator.hasNext()) {
                                AppLog.e(TAG, "sendMessageFromQueue add received ACK")
                                
                                val (index, pairValue) = ackReceivedIterator.next()
                                val messageId = sendQueue.flcAckReceived(index, pairValue.first, pairValue.second)
                                // notify libqaul about message sending result
                                bleCallback?.onMessageSent(
                                    id = messageId,
                                    success = pairValue.first,
                                    data = ByteArray(0)
                                )
                            }
                            
                            // add chunks to send
                            for (missingChunk in connectionInfo.missingChunksToSend) {
                                AppLog.e(TAG, "sendMessageFromQueue add missing chunk to send")
                                sendQueue.addMissingChunkIndexToSend(missingChunk)
                            }

                            // prepare message for sending
                            val (chunks, messageIndex, messageId) = sendQueue!!.getChunks()
                            hashMapSendQueue[macAddress] = sendQueue

                            // schedule chunks for sending
                            val bleActor = connectDevice(device = bleDevice, isFromMessage = true)
                            bleActor.messageId = messageId
                            bleActor.sendChunkQueue.addAll(chunks)
                        }
                    }
                }
            }
        }
    }

    // -------------------------------------------------------------------------

    // TODO: is this of any use? Ever invoked?
    private val mGattCallback: BluetoothGattCallback = object : BluetoothGattCallback() {
        override fun onConnectionStateChange(gatt: BluetoothGatt, status: Int, newState: Int) {
            super.onConnectionStateChange(gatt, status, newState)

        }

        override fun onServicesDiscovered(gatt: BluetoothGatt, status: Int) {
            super.onServicesDiscovered(gatt, status)

        }

        override fun onCharacteristicRead(
            gatt: BluetoothGatt, characteristic: BluetoothGattCharacteristic, status: Int,
        ) {
            super.onCharacteristicRead(gatt, characteristic, status)

        }


        override fun onCharacteristicWrite(
            gatt: BluetoothGatt, characteristic: BluetoothGattCharacteristic, status: Int,
        ) {
            super.onCharacteristicWrite(gatt, characteristic, status)

        }

        override fun onCharacteristicChanged(
            gatt: BluetoothGatt, characteristic: BluetoothGattCharacteristic,
        ) {
            super.onCharacteristicChanged(gatt, characteristic)

        }

        override fun onDescriptorRead(
            gatt: BluetoothGatt, descriptor: BluetoothGattDescriptor, status: Int,
        ) {
            super.onDescriptorRead(gatt, descriptor, status)
        }

        override fun onDescriptorWrite(
            gatt: BluetoothGatt, descriptor: BluetoothGattDescriptor, status: Int,
        ) {
            super.onDescriptorWrite(gatt, descriptor, status)

        }

        override fun onMtuChanged(gatt: BluetoothGatt?, mtu: Int, status: Int) {
            super.onMtuChanged(gatt, mtu, status)
            AppLog.e("BleService MTU Size: ", "" + mtu)
        }
    }

    /**
     * This is a Interface for Sending Advertisement Start & Stop Response to BLEWrapperClass.
     */
    interface BleAdvertiseCallback {
        fun startAdvertiseRes(status: Boolean, errorText: String, unknownError: Boolean)
        fun stopAdvertiseRes(status: Boolean, errorText: String)
        fun onMessageReceived(qaulId: ByteArray, message: ByteArray)
    }

    /**
     * This is a Interface for Sending Scan Start & Stop Response to BLEWrapperClass.
     */
    interface BleScanCallBack {
        fun startScanRes(status: Boolean, errorText: String, unknownError: Boolean)
        fun stopScanRes(status: Boolean, errorText: String)
        fun deviceFound(bleDevice: BLEScanDevice)
        fun deviceOutOfRange(bleDevice: BLEScanDevice)
        fun onMessageSent(id: String, success: Boolean, data: ByteArray)
        fun restartService()
    }

}