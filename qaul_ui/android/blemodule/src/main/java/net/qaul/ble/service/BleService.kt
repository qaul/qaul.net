// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
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
import com.google.gson.Gson
import net.qaul.ble.AppLog
import net.qaul.ble.BLEUtils
import net.qaul.ble.RemoteLog
import net.qaul.ble.core.BleActor
import net.qaul.ble.model.BLEScanDevice
import net.qaul.ble.model.Message
import java.util.*
import java.util.concurrent.CopyOnWriteArrayList
import java.util.concurrent.Executors

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
    private val devicesList = CopyOnWriteArrayList(arrayListOf<BLEScanDevice>())
    private val ignoreList = CopyOnWriteArrayList(arrayListOf<BLEScanDevice>())
    private val receiveList = Collections.synchronizedList(arrayListOf<BLEScanDevice>())
    private val blackList = Collections.synchronizedList(arrayListOf<BLEScanDevice>())
    private val uuidList = arrayListOf<ParcelUuid>()
    private var filters: ArrayList<ScanFilter> = arrayListOf()
    private var scanSettings: ScanSettings? = null
    private val msgMap = Collections.synchronizedMap(hashMapOf<String, String>())
    private val actorMap = Collections.synchronizedMap(hashMapOf<String, BleActor>())
    private val handler = Handler(Looper.getMainLooper())
    private var lastWriteTime = System.currentTimeMillis() + 60000
    private var executor = Executors.newSingleThreadExecutor()

    //
    private val hashMap: HashMap<String, Queue<Triple<String, ByteArray, ByteArray>>> = hashMapOf()
    private val sharedPrefFile = "sharedpreference_qaul_ble"
    private lateinit var sharedPreferences: SharedPreferences

    companion

    object {
        var bleService: BleService? = null
        var isAdvertisementRunning = false
        var isScanningRunning = false
        val SERVICE_UUID = "99E91399-80ED-4943-9BCB-39C532A76023"
        val MSG_SERVICE_UUID = "99E91400-80ED-4943-9BCB-39C532A76023"
        val READ_CHAR = "99E91401-80ED-4943-9BCB-39C532A76023"
        val MSG_CHAR = "99E91402-80ED-4943-9BCB-39C532A76023"
        val GD_CHAR = "99E91403-80ED-4943-9BCB-39C532A76023"
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
     * This Method Will Set the necessary data and start the advertisement
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

                mainChar.value = qaulId
                mainService.addCharacteristic(mainChar)

                val msgService = BluetoothGattService(
                    UUID.fromString(MSG_SERVICE_UUID), BluetoothGattService.SERVICE_TYPE_PRIMARY
                )

                val msgChar = BluetoothGattCharacteristic(
                    UUID.fromString(MSG_CHAR),
                    BluetoothGattCharacteristic.PROPERTY_WRITE,
                    BluetoothGattCharacteristic.PERMISSION_WRITE
                )
//                msgChar.writeType = BluetoothGattCharacteristic.WRITE_TYPE_DEFAULT

                msgService.addCharacteristic(msgChar)
                mainService.addCharacteristic(msgChar)
                val serviceList = arrayListOf<BluetoothGattService>()
                serviceList.add(mainService)
                serviceList.add(msgService)
                startGattServer(services = serviceList)

                val dataBuilder = AdvertiseData.Builder()
                val settingsBuilder = AdvertiseSettings.Builder()
                dataBuilder.setIncludeTxPowerLevel(true)

                val uuid = ParcelUuid(UUID.fromString(SERVICE_UUID))
                dataBuilder.addServiceUuid(uuid)
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
//        t.start()

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
//        AppLog.e(TAG, "device : " + device.address)
        if (blackList.find { it.macAddress == device.address } == null) {
            val selectItem = devicesList.toMutableList().find { it.macAddress == device.address }
//            handler.postDelayed({
            if (selectItem == null) {
//                AppLog.e(TAG, "device : " + device.address)
//                AppLog.e(TAG, "UUID : " + result.scanRecord!!.serviceUuids)
                RemoteLog[this]!!.addDebugLog("$TAG:device : " + device.address + " " + result.scanRecord!!.serviceUuids)
                val bleDevice: BLEScanDevice = BLEScanDevice.getDevice()
                bleDevice.bluetoothDevice = device
                bleDevice.scanResult = result
                bleDevice.name = device.name
                bleDevice.deviceRSSI = rssi
                bleDevice.macAddress = device.address
                bleDevice.isConnectable = result.isConnectable
                //bleDevice.lastFoundTime = System.currentTimeMillis()
                devicesList.add(bleDevice)


//                Handler(Looper.getMainLooper()).postDelayed({
                if (result.isConnectable) {
                    connectDevice(bleDevice, isFromMessage = false)
                }
//                }, 1200)

            } else {
//                val selectItemIgnore = ignoreList.find { it.macAddress == device.address }
//                if (selectItemIgnore != null) {
//                    selectItemIgnore.deviceRSSI = rssi
//                    selectItemIgnore.scanResult = result
//                    selectItemIgnore.name = device.name
//                    selectItemIgnore.isConnectable = result.isConnectable
//                    selectItemIgnore.lastFoundTime = System.currentTimeMillis()
////                    if (!selectItemIgnore.isConnected) {
////                        connectDevice(selectItemIgnore, isFromMessage = false)
////                    }
//                } else {
////                    selectItem.isConnected = false
////                    devicesList.remove(selectItem)
//                    AppLog.e(TAG, "zzz device ignored: " + device.address)
//                }
                //AppLog.e(TAG, "-------------------> HERE FOR CONNECTION   parseBLEFrame ")
                ignoreList.find { it.macAddress == device.address }?.lastFoundTime = System.currentTimeMillis()
            }
//            }, 5000)
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
            //    AppLog.e(TAG, "Write Request Received: " + String(value) + " :: " + requestId)
                val s = BLEUtils.byteToHex(value)
               AppLog.e(TAG, "Data in hex:: $s")
                var bleDevice = ignoreList.find { it.macAddress == device.address }
                if (bleDevice == null) {
                    bleDevice = receiveList.find { it.macAddress == device.address }
                }
                gattServer!!.sendResponse(
                    device, requestId, BluetoothGatt.GATT_SUCCESS, offset, value
                )
//                Log.e(TAG, "Device Address:: ${device.address}")
                if (msgMap.containsKey(device.address)) {
                    var oldValue = msgMap[device.address]
                    if (s.endsWith("2424") || (oldValue!!.endsWith("24") && s == "24")) {
                        //SendResponse of oldValue

                        AppLog.e(TAG, "onCharacteristicWriteRequest:  contain 2424")
                        oldValue += s
                        val msgData = String(BLEUtils.hexToByteArray(oldValue)!!).removeSuffix("$$")
                            .removePrefix("$$")
                        Log.e(TAG, "Msg Data:: $msgData")
                        if (!msgData.contains("$$")) {
                            val msgObject = Gson().fromJson(msgData, Message::class.java)
                            if (bleDevice == null) {
                                bleDevice = BLEScanDevice.getDevice()
                                bleDevice.macAddress = device.address
                                bleDevice.qaulId = msgObject.qaulId
                                bleDevice.bluetoothDevice = device
                                receiveList.add(bleDevice)
                            }
                            bleAdvertiseCallback!!.onMessageReceived(
                                bleDevice = bleDevice, BLEUtils.hexToByteArray(oldValue)!!
                            )
                            msgMap.remove(device.address)
                        } else {
                            Log.e(TAG, "onCharacteristicWriteRequest:  contain $$")
                        }
                    } else {
                        AppLog.e(TAG, "onCharacteristicWriteRequest:  not contain 2424")
                        oldValue += s
                        msgMap[device.address] = oldValue
                    }
                } else {
                    if (s.startsWith("2424") && s.endsWith("2424")) {
                        //Send Response of s
                        val msgData = String(BLEUtils.hexToByteArray(s)!!).removeSuffix("$$")
                            .removePrefix("$$")
                        AppLog.e(TAG, "Got whole message at once $msgData")    
                        val msgObject = Gson().fromJson(msgData, Message::class.java)
                        if (bleDevice == null) {
                            bleDevice = BLEScanDevice.getDevice()
                            bleDevice.macAddress = device.address
                            bleDevice.qaulId = msgObject.qaulId
                            bleDevice.bluetoothDevice = device
                            receiveList.add(bleDevice)
                        }
                        bleAdvertiseCallback!!.onMessageReceived(
                            bleDevice = bleDevice, BLEUtils.hexToByteArray(s)!!
                        )
                    } else if (s.startsWith("2424")) {
                        msgMap[device.address] = s
                    } else {
                        AppLog.e("onCharacteristicWriteRequest()", "Invalid data received.")
                    }
                }
            }

            override fun onDescriptorReadRequest(
                device: BluetoothDevice,
                requestId: Int,
                offset: Int,
                descriptor: BluetoothGattDescriptor,
            ) {
                super.onDescriptorReadRequest(device, requestId, offset, descriptor)
                AppLog.e(TAG, "onDescriptorReadRequest()")
            }

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
        // TODO: DK
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
//                    AppLog.d(TAG, "${bLEDevice.macAddress} out of range ${ignoreList.size}")
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
                val editor: SharedPreferences.Editor = sharedPreferences.edit()
                editor.putString("BLE_GATT", Gson().toJson(device))
                editor.apply()
            }

            override fun onDisconnected(bleScanDevice: BLEScanDevice) {
                AppLog.e(TAG, " onDisconnected : ${bleScanDevice.macAddress}")
//                bleScanDevice.isConnected = false
//                device.isConnected = false
                if (!blackList.contains(bleScanDevice)) {
                    devicesList.remove(bleScanDevice)
//                    ignoreList.remove(bleScanDevice)
                }
                if (System.currentTimeMillis() - 60000 > lastWriteTime) {
//                    bleCallback?.restartService()
                    //removeGatt()
                }
            }

            private fun removeGatt() {
                AppLog.e(TAG, "  REMOVE SAVED GATT")
                val gatt = sharedPreferences.getString("BLE_GATT", "")
                gatt?.let {
                    if (it.isNotEmpty()) {
                        var bleDevice = Gson().fromJson(it, BluetoothDevice::class.java)
                        var connectGatt: BluetoothGatt? =
                            bleDevice.connectGatt(this@BleService, false, mGattCallback)
                        AppLog.e(TAG, "  REMOVE SAVED GATT")
                        connectGatt?.close()
                        connectGatt = null
                    }
                }
            }

            override fun onServiceDiscovered(macAddress: String?) {
                AppLog.e(TAG, " onServiceDiscovered : $macAddress")
            }

            override fun onDescriptorWrite(bleScanDevice: BLEScanDevice, bleActor: BleActor) {
                AppLog.e(TAG, " onDescriptorWrite : ${bleScanDevice.macAddress}")
//                if (!bleActor.isFromMessage) {
                bleActor.readServiceData(SERVICE_UUID, READ_CHAR)
//            }
            }

            override fun onConnectionFailed(bleScanDevice: BLEScanDevice) {
                AppLog.e(TAG, "zzz onConnectionFailed : ${bleScanDevice.macAddress}")
//                bleScanDevice.isConnected = false
//                device.isConnected = false
//                ignoreList.removeConcurrent(bleScanDevice)
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

            override fun onMessageSent(
                gatt: BluetoothGatt?, value: ByteArray, id: String,
            ) {
                val queue = hashMap[gatt?.device?.address]
                if (queue?.isNotEmpty() == true) {
                    AppLog.e(TAG, "onMessageSent:SIZE ->  queue.isNotEmpty()  ")
                    queue.poll()
                    hashMap[gatt?.device?.address!!] = queue
                }

                AppLog.e(TAG, "onMessageSent:SIZE ->  ${queue?.size} ")
                bleCallback?.onMessageSent(id = id, success = true, data = value)
                sendMessageFromQueu(gatt?.device?.address!!)

            }

            override fun onCharacteristicChanged(
                macAddress: String?,
                gatt: BluetoothGatt?,
                characteristic: BluetoothGattCharacteristic?,
            ) {

            }

            override fun addToBlackList(bleScanDevice: BLEScanDevice) {
                blackList.add(bleScanDevice)
                // AppLog.e(TAG, " addToBlackList : $blackList")
            }

            override fun addToIgnoreList(bleScanDevice: BLEScanDevice) {
                ignoreList.add(bleScanDevice)
                // AppLog.e(TAG, " addToIgnoreList : $ignoreList")
            }

        }

//        device.isConnected = true

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

    /**
     * This Method Will Be Used to Send Data to Other Qaul-Device
     */
    fun sendMessage(id: String, to: ByteArray, message: ByteArray, from: ByteArray) {
        var bleDevice = ignoreList.find { it.qaulId.contentEquals(to) }
        if (bleDevice == null) {
            bleDevice = receiveList.find { it.qaulId.contentEquals(to) }
        }

        AppLog.e(
            TAG, "sendMessage   ${BLEUtils.byteToHex(message)}"
        )
        // var mainQueue: Queue<Triple<String, ByteArray, ByteArray>>? = null
        bleDevice?.let {
            if (hashMap.containsKey(it.macAddress)) {
                var queue = hashMap[it.macAddress!!]
                if(queue!!.size < 2) {
                    queue?.add(Triple(id, from, message))
                } else{
                    queue = LinkedList()
                }
                hashMap[it.macAddress!!] = queue!!
                // mainQueue = queue
                // AppLog.d(TAG, " Manual send =======  Queue size was already 1 ")
            } else {
                // AppLog.d(TAG, " Manual send =====  Queue size was empty ")
                val queue: Queue<Triple<String, ByteArray, ByteArray>> = LinkedList()
                queue.add(Triple(id, from, message))
                hashMap[it.macAddress!!] = queue
                // mainQueue = queue
            }
            // AppLog.e(TAG, "device--> ${it.macAddress} ${mainQueue?.size}")
            sendMessageFromQueu(it.macAddress!!, true)

        }
    }


    private fun sendMessageFromQueu(macAddress: String, isFromSendMessage: Boolean = false) {
        //Thread.sleep(10)
        executor.execute {
            if (hashMap.isNotEmpty()) {
                val queue = hashMap[macAddress]
                if (!queue.isNullOrEmpty()) {
                    AppLog.e(
                        TAG,
                        "sendMessageFromQueu ${queue.size} ${isFromSendMessage}"
                    )
                    if (!isFromSendMessage || queue.size == 1) {
                        var bleDevice = ignoreList.find { it.macAddress.contentEquals(macAddress) }
                        if (bleDevice == null) {
                            bleDevice = receiveList.find { it.macAddress.contentEquals(macAddress) }
                        }
                        val messageFormQueue = queue.peek()

                        messageFormQueue?.let { mesTrip ->
                            val msg = Message()
                            msg.message = mesTrip.third
                            msg.qaulId = mesTrip.second
                            if (bleDevice != null) {
                                AppLog.e(
                                    TAG,
                                    "-------------------> HERE FOR CONNECTION   sendMessageFromQueu "
                                )
                                val bleActor =
                                    connectDevice(device = bleDevice, isFromMessage = true)
//                                Handler(Looper.getMainLooper()).postDelayed({
                                bleActor.messageId = mesTrip.first
                                val btArray = Gson().toJson(msg).toByteArray()
                                val delimiter = ByteArray(2)
                                delimiter[0] = 36
                                delimiter[1] = 36
                                bleActor.tempData = delimiter + btArray + delimiter
                                AppLog.e(
                                    TAG,
                                    "data------------>sendMessage   ${BLEUtils.byteToHex(bleActor.tempData)}"
                                )
//                                },500)

                            } else {
                                AppLog.e(
                                    TAG, "data------------>onMessageSent Failed"
                                )
                                bleCallback?.onMessageSent(
                                    id = mesTrip.first, success = false, data = ByteArray(0)
                                )
                                queue.poll()
                                hashMap[macAddress] = queue
                            }
                        }
                    }
                }
            }
        }
    }

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
            AppLog.e("MTU Size: ", "" + mtu)
        }
    }

    /**
     * This is a Interface for Sending Advertisement Start & Stop Response to BLEWrapperClass.
     */
    interface BleAdvertiseCallback {
        fun startAdvertiseRes(status: Boolean, errorText: String, unknownError: Boolean)
        fun stopAdvertiseRes(status: Boolean, errorText: String)
        fun onMessageReceived(bleDevice: BLEScanDevice, message: ByteArray)
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