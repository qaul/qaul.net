// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

package net.qaul.ble.core

import android.Manifest
import android.annotation.SuppressLint
import android.app.Activity
import android.bluetooth.BluetoothAdapter
import android.bluetooth.BluetoothManager
import android.content.Context
import android.content.Intent
import android.content.IntentSender
import android.content.pm.PackageManager
import android.location.LocationManager
import android.os.Build
import android.os.Handler
import android.os.Looper
import android.util.Log
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity
import androidx.core.app.ActivityCompat
import androidx.lifecycle.LifecycleService
import com.google.android.gms.common.api.GoogleApiClient
import com.google.android.gms.common.api.PendingResult
import com.google.android.gms.location.*
import com.google.gson.Gson
import com.google.protobuf.ByteString
import net.qaul.ble.AppLog
import net.qaul.ble.RemoteLog
import net.qaul.ble.callback.BleRequestCallback
import net.qaul.ble.model.BLEScanDevice
import net.qaul.ble.model.Message
import net.qaul.ble.service.BleService
import qaul.sys.ble.BleOuterClass
import java.nio.charset.Charset

@SuppressLint("MissingPermission")
open class BleWrapperClass(context: AppCompatActivity) {
    private val TAG: String = BleWrapperClass::class.java.simpleName
    private val context = context
    private var errorText = ""
    private var noRights = false
    private var bleCallback: BleRequestCallback? = null
    private var qaulId: ByteArray? = null
    private var advertMode = "low_latency"
    private var isFromMessage = false

    public var mHandler: Handler? = null
	
	/**
	* Constructor of BleWrapperClass
	*/
    init{
        this.also { sInstance = it }
        mHandler = Handler()
		
		loadLibqaul()
        nativeSetCallback(object: ILibqaulCallback {
            override fun OnLibqaulMessage(data: ByteArray){
                AppLog.i("===libqaul===","This was called from libqaul.  :-)")

                mHandler?.post {
                    BleWrapperClass.sInstance.receiveRequest(ByteString.copyFrom(data), null)
                }
            }
        })
    }

    /**
     * Static Member Declaration
     */
    companion object {
        val serviceManager = this
        lateinit var sInstance:BleWrapperClass

        const val LOCATION_PERMISSION_REQ_CODE = 111
        const val LOCATION_ENABLE_REQ_CODE = 112
        const val REQUEST_ENABLE_BT = 113
        const val BLE_PERMISSION_REQ_CODE_12 = 114
	}

    interface ILibqaulCallback {
        fun OnLibqaulMessage(data: ByteArray)
    }

    external fun nativeSetCallback(callback:ILibqaulCallback)

    /**
     * This Method set BleRequestCallback
     */
    open fun setBleRequestCallback(callback: BleRequestCallback?) {
        bleCallback = callback
    }

    /**
     * This Method get BLERequest from UI & Return BLEResponse by Callback Interface Method
     */
    open fun receiveRequest(data: ByteString, callback: BleRequestCallback?) {
        val bleReq: BleOuterClass.Ble = BleOuterClass.Ble.parseFrom(data)
        if (bleReq.isInitialized) {
            if(callback != null)
                bleCallback = callback

            Log.e(TAG, bleReq.messageCase.toString())
            when (bleReq.messageCase) {
                BleOuterClass.Ble.MessageCase.INFO_REQUEST -> {
                    getDeviceInfo()
                }
                BleOuterClass.Ble.MessageCase.START_REQUEST -> {
                    qaulId = bleReq.startRequest.qaulId.toByteArray()
                    AppLog.e(TAG, "qaulid : " + qaulId?.size)
                    advertMode = bleReq.startRequest.powerSetting.toString()
                    if (qaulId != null) {
                        startService(context = context)
                    } else {
                        val bleRes = BleOuterClass.Ble.newBuilder()
                        val startResult = BleOuterClass.BleStartResult.newBuilder()
                        startResult.success = false
                        startResult.errorReason = BleOuterClass.BleError.UNKNOWN_ERROR
                        startResult.errorMessage = "qaul id is required"
                        bleRes.startResult = startResult.build()
                        sendResponse(bleRes)
                    }
                }
                BleOuterClass.Ble.MessageCase.STOP_REQUEST -> {
                    stopService()
                }
                BleOuterClass.Ble.MessageCase.DIRECT_SEND -> {
                    val bleDirectSend = bleReq.directSend
                    if (BleService().isRunning()) {
                        BleService.bleService?.sendMessage(
                            id = bleDirectSend.messageId.toString(Charset.defaultCharset()),
                            to = bleDirectSend.receiverId.toByteArray(),
                            message = bleDirectSend.data.toByteArray(),
                            from = bleDirectSend.senderId.toByteArray()
                        )
                    }
                }
                else -> {}
            }
        }
    }

    /**
     * This Method Will send response message to App & libqaul library
     */
    private fun sendResponse(bleRes:BleOuterClass.Ble.Builder) {
        mHandler?.post{
            //callback response for App

            bleCallback?.bleResponse(bleRes.build().toByteString())

            //callback response for libqaul
            net.qaul.libqaul.syssend(bleRes.build().toByteArray())
        }

    }

    /**
     * This Method Will Stop the Service & Advertisement.
     */
    private fun stopService() {
        if (BleService().isRunning()) {
            BleService.bleService?.stop()
        } else {
            val bleRes = BleOuterClass.Ble.newBuilder()
            val stopResult = BleOuterClass.BleStopResult.newBuilder()
            stopResult.success = false
            stopResult.errorMessage = "Advertisement & Scanning is not Running"
            bleRes.stopResult = stopResult.build()
            sendResponse(bleRes)
        }
    }

    /**
     * This Method Will Start BLEService
     */
    private fun startService(context: Context) {
        if (isBleScanConditionSatisfy()) {
            if (!BleService().isRunning()) {
                BleService().start(context = context)
                Handler(Looper.myLooper()!!).postDelayed({
                    startAdvertiseAndCallback()
                    startScanAndCallback()
                }, 500)
            } else {
                if (BleService.bleService!!.isAdvertiserRunning()) {
                    AppLog.e(TAG, "Already Started")
                    val bleRes = BleOuterClass.Ble.newBuilder()
                    val startResult = BleOuterClass.BleStartResult.newBuilder()
                    startResult.success = true
                    startResult.errorReason = BleOuterClass.BleError.UNKNOWN_ERROR
                    startResult.errorMessage = "Advertisement already Started"
                    bleRes.startResult = startResult.build()
                    sendResponse(bleRes)
                } else {
                    startAdvertiseAndCallback()
                }

                if (BleService.bleService!!.isScanRunning()) {
                    AppLog.e(TAG, "Scan Already Started")
                    val bleRes = BleOuterClass.Ble.newBuilder()
                    val startResult = BleOuterClass.BleStartResult.newBuilder()
                    startResult.success = true
                    startResult.errorReason = BleOuterClass.BleError.UNKNOWN_ERROR
                    startResult.errorMessage = "Scanning already Started"
                    bleRes.startResult = startResult.build()
                    sendResponse(bleRes)
                } else {
                    startScanAndCallback()
                }
            }
        }
    }


    /**
     * This Method Will Assign Callback & Data to Start Advertiser and Receive Callback
     */
    private fun startAdvertiseAndCallback() {
        if (qaulId != null) {
            BleService.bleService?.startAdvertise(
                qaul_id = qaulId!!, mode = advertMode,
                object : BleService.BleAdvertiseCallback {
                    override fun startAdvertiseRes(
                        status: Boolean,
                        errorText: String,
                        unknownError: Boolean
                    ) {
                        val bleRes = BleOuterClass.Ble.newBuilder()
                        val startResult = BleOuterClass.BleStartResult.newBuilder()
                        startResult.success = status
                        if (unknownError) {
                            startResult.errorReason = BleOuterClass.BleError.UNKNOWN_ERROR
                        } else {
                            startResult.errorReason = BleOuterClass.BleError.UNRECOGNIZED
                        }
                        startResult.errorMessage = errorText
                        bleRes.startResult = startResult.build()
                        sendResponse(bleRes)
                    }

                    override fun stopAdvertiseRes(status: Boolean, errorText: String) {
                        val bleRes = BleOuterClass.Ble.newBuilder()
                        val stopResult = BleOuterClass.BleStopResult.newBuilder()
                        stopResult.success = status
                        stopResult.errorMessage = errorText
                        bleRes.stopResult = stopResult.build()
                        sendResponse(bleRes)
                    }

                    override fun onMessageReceived(bleDevice: BLEScanDevice, message: ByteArray) {
                        val bleRes = BleOuterClass.Ble.newBuilder()
                        val directReceived = BleOuterClass.BleDirectReceived.newBuilder()
                        val msgData = String(message).removeSuffix("$$")
                            .removePrefix("$$")
                        val msgObject = Gson().fromJson(msgData, Message::class.java)
                        directReceived.from = ByteString.copyFrom(bleDevice.qaulId)
                        directReceived.data =
                            ByteString.copyFrom(msgObject.message, Charset.defaultCharset())
                        bleRes.directReceived = directReceived.build()
                        sendResponse(bleRes)
                    }
                }
            )
        }
    }

    /**
     * This Method Will Assign Callback & Data to Start Scan and Receive Callback
     */
    private fun startScanAndCallback() {
        BleService.bleService?.startScan(
            object : BleService.BleScanCallBack {
                override fun startScanRes(
                    status: Boolean,
                    errorText: String,
                    unknownError: Boolean
                ) {
                    val bleRes = BleOuterClass.Ble.newBuilder()
                    val startResult = BleOuterClass.BleStartResult.newBuilder()
                    startResult.success = status
                    if (unknownError) {
                        startResult.errorReason = BleOuterClass.BleError.UNKNOWN_ERROR
                    } else {
                        startResult.errorReason = BleOuterClass.BleError.UNRECOGNIZED
                    }
                    startResult.errorMessage = errorText
                    bleRes.startResult = startResult.build()
                    sendResponse(bleRes)
                }

                override fun stopScanRes(status: Boolean, errorText: String) {
                    val bleRes = BleOuterClass.Ble.newBuilder()
                    val stopResult = BleOuterClass.BleStopResult.newBuilder()
                    stopResult.success = status
                    stopResult.errorMessage = errorText
                    bleRes.stopResult = stopResult.build()
                    sendResponse(bleRes)
                }

                override fun deviceFound(bleDevice: BLEScanDevice) {
                    val bleRes = BleOuterClass.Ble.newBuilder()
                    val deviceDiscovered = BleOuterClass.BleDeviceDiscovered.newBuilder()
                    deviceDiscovered.rssi = bleDevice.deviceRSSI
                    deviceDiscovered.qaulId = ByteString.copyFrom(bleDevice.qaulId)
                    bleRes.deviceDiscovered = deviceDiscovered.build()
                    sendResponse(bleRes)
                }

                override fun deviceOutOfRange(bleDevice: BLEScanDevice) {
                    AppLog.e(TAG, "${bleDevice.macAddress} out of range")
                    val bleRes = BleOuterClass.Ble.newBuilder()
                    val deviceUnavailable = BleOuterClass.BleDeviceUnavailable.newBuilder()
                    try {
                        deviceUnavailable.qaulId = ByteString.copyFrom(bleDevice.qaulId)
                        bleRes.deviceUnavailable = deviceUnavailable.build()
                        sendResponse(bleRes)
                    } catch (e: Exception) {
                        e.printStackTrace()
                    }
                }

                override fun onMessageSent(id: String, success: Boolean, data: ByteArray) {
                    val bleRes = BleOuterClass.Ble.newBuilder()
                    val directSendResult = BleOuterClass.BleDirectSendResult.newBuilder()
                    if (success) {
                        directSendResult.errorMessage = "Successfully sent"
                    } else {
                        directSendResult.errorMessage =
                            "Connection not established. Please try again."
                    }
                    directSendResult.success = success
                    directSendResult.id = ByteString.copyFrom(id.toByteArray(Charset.forName("UTF-8")))
                    bleRes.directSendResult = directSendResult.build()
                    sendResponse(bleRes)
                }

            }
        )
    }

    /**
     * This Method Return Device Information Regarding BLE Functionality & Permissions
     */
    private fun getDeviceInfo() {
        val bluetoothManager =
            context.getSystemService(LifecycleService.BLUETOOTH_SERVICE) as BluetoothManager
        val adapter = bluetoothManager.adapter
        val bleRes: BleOuterClass.Ble.Builder = BleOuterClass.Ble.newBuilder()
        val bleResInfoResponse = BleOuterClass.BleInfoResponse.newBuilder()
        if (bleRes.isInitialized) {
            if (bleResInfoResponse.isInitialized) {
                val deviceInfoBuilder: BleOuterClass.BleDeviceInfo.Builder =
                    BleOuterClass.BleDeviceInfo.newBuilder()
                deviceInfoBuilder.bluetoothOn = isBluetoothEnable()
                deviceInfoBuilder.id = ""
                if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
                    if (ActivityCompat.checkSelfPermission(
                            context,
                            Manifest.permission.BLUETOOTH_CONNECT
                        ) == PackageManager.PERMISSION_GRANTED
                    ) {
                        deviceInfoBuilder.name = adapter.name
                    } else {
                        deviceInfoBuilder.name = getDeviceName()
                    }
                } else {
                    deviceInfoBuilder.name = adapter.name
                }
                deviceInfoBuilder.bleSupport = isBLeSupported()
                if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
                    deviceInfoBuilder.advExtended = adapter.isLeExtendedAdvertisingSupported
                    deviceInfoBuilder.le2M = adapter.isLe2MPhySupported
                    deviceInfoBuilder.leCoded = adapter.isLeCodedPhySupported
                    deviceInfoBuilder.advExtendedBytes = adapter.leMaximumAdvertisingDataLength

                    //Return true if LE Periodic Advertising feature is supported.
                    deviceInfoBuilder.lePeriodicAdvSupport =
                        adapter.isLePeriodicAdvertisingSupported
                    //Return true if the multi advertisement is supported by the chipset
                    deviceInfoBuilder.leMultipleAdvSupport =
                        adapter.isMultipleAdvertisementSupported
                }
                //Return true if offloaded filters are supported true if chipset supports on-chip filtering
                deviceInfoBuilder.offloadFilterSupport = adapter.isOffloadedFilteringSupported

                //Return true if offloaded scan batching is supported true if chipset supports on-chip scan batching
                deviceInfoBuilder.offloadScanBatchingSupport =
                    adapter.isOffloadedScanBatchingSupported
                if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
                    deviceInfoBuilder.leAudio = isClass("android.bluetooth.BluetoothLeAudio")
                }
                bleResInfoResponse.device = deviceInfoBuilder.build()
            }
            bleRes.infoResponse = bleResInfoResponse.build()
            sendResponse(bleRes)
        }
    }

    /**
     * This Method Checks if inputted Class Exist or Not
     */
    private fun isClass(className: String): Boolean {
        return try {
            Class.forName(className)
            true
        } catch (e: ClassNotFoundException) {
            e.printStackTrace()
            false
        }
    }

    /**
     * Checks if BLE Feature is Supported or Not
     */
    private fun isBLeSupported(): Boolean {
        return context.packageManager.hasSystemFeature(PackageManager.FEATURE_BLUETOOTH_LE)
    }

    /**
     * Checks if Bluetooth is Enabled or Not
     */
    private fun isBluetoothEnable(): Boolean {
        val bluetoothManager =
            context.getSystemService(Context.BLUETOOTH_SERVICE) as BluetoothManager
        val bluetoothAdapter = bluetoothManager.adapter
        return if (bluetoothAdapter != null) {
            bluetoothAdapter.isEnabled
        } else {
            AppLog.e(TAG, "Bluetooth Not Supported")
            RemoteLog[context]!!.addDebugLog("$TAG:Bluetooth Not Supported::")
            false
        }
    }

    /**
     * Checks if Location is Enabled or Not
     */
    private fun isLocationEnable(): Boolean {
        val lm = context.getSystemService(Context.LOCATION_SERVICE) as LocationManager
        return try {
            lm.isProviderEnabled(LocationManager.GPS_PROVIDER)
        } catch (ex: Exception) {
            AppLog.e(TAG, "isLocationEnable() Exception :$ex")
            RemoteLog[context]!!.addDebugLog("$TAG:isLocationEnable() Exception :$ex")
            false
        }
    }

    /**
     * Return the Current OS SDK Version
     */
    private fun getOsVersion(): Int {
        return Build.VERSION.SDK_INT
    }

    /**
     * Returns Device Manufacturer & Model Name/Number
     */
    private fun getDeviceName(): String? {
        val manufacturer = Build.MANUFACTURER
        val model = Build.MODEL
        return if (model.lowercase().startsWith(manufacturer.lowercase())) {
            capitalize(model)
        } else {
            capitalize(manufacturer).toString() + " " + model
        }
    }

    /**
     * Capitalize 1st Letter of String
     */
    private fun capitalize(s: String?): String {
        if (s == null || s.isEmpty()) {
            return ""
        }
        val first = s[0]
        return if (Character.isUpperCase(first)) {
            s
        } else {
            Character.toUpperCase(first).toString() + s.substring(1)
        }
    }

    /**
     * Request User to Enable Bluetooth
     */
    private fun enableBluetooth(context: Activity, requestCode: Int): Boolean {
        val bluetoothManager =
            context.getSystemService(Context.BLUETOOTH_SERVICE) as BluetoothManager
        val bluetoothAdapter = bluetoothManager.adapter
        return if (bluetoothAdapter != null) {
            if (!bluetoothAdapter.isEnabled) {
                val enableBtIntent = Intent(BluetoothAdapter.ACTION_REQUEST_ENABLE)
                context.startActivityForResult(enableBtIntent, requestCode)
                false
            } else {
                true
            }
        } else {
            Toast.makeText(context, "Bluetooth Not Supported", Toast.LENGTH_SHORT).show()
            false
        }
    }

    /**
     * Disable Bluetooth
     */
    private fun disableBluetooth(): Boolean {
        val bluetoothManager =
            context.getSystemService(Context.BLUETOOTH_SERVICE) as BluetoothManager
        val bluetoothAdapter = bluetoothManager.adapter
        bluetoothAdapter?.disable()
        return false
    }

    /**
     * Checks if Bluetooth Permission is Allowed or Not for Android 12 & Above
     */
    private fun isBluetoothPermissionAllowed(): Boolean {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            return hasPermission(
                arrayOf(
                    Manifest.permission.BLUETOOTH_SCAN,
                    Manifest.permission.BLUETOOTH_CONNECT,
                    Manifest.permission.BLUETOOTH_ADVERTISE
                )
            )
        }
        return false
    }

    /**
     * Checks if Location Permission is Allowed or Not
     */
    private fun isLocationPermissionAllowed(): Boolean {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.M && Build.VERSION.SDK_INT < Build.VERSION_CODES.S) {
            return hasPermission(
                arrayOf(
                    Manifest.permission.ACCESS_FINE_LOCATION
                )
            )
        }
        return hasPermission(
            arrayOf()
        )
    }

    /**
     * Checks if Given Permissions (input as array) are Allowed or Not
     */
    private fun hasPermission(permissions: Array<String>?): Boolean {
        if (permissions != null) {
            for (permission in permissions) {
                if (ActivityCompat.checkSelfPermission(
                        context,
                        permission
                    ) != PackageManager.PERMISSION_GRANTED
                ) return false
            }
        }
        return true
    }

    /**
     * Request User to Allow Location Permission
     */
    private fun enableLocationPermission(
        activity: Activity?,
        requestCode: Int
    ) {
        ActivityCompat.requestPermissions(
            activity!!,
            arrayOf(Manifest.permission.ACCESS_FINE_LOCATION),
            requestCode
        )
    }

    /**
     * Request User to Allow Bluetooth Permissions for Android 12 & Above
     */
    private fun enableBlePermission(
        activity: Activity?,
        requestCode: Int
    ) {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            ActivityCompat.requestPermissions(
                activity!!,
                arrayOf(
                    Manifest.permission.BLUETOOTH_SCAN,
                    Manifest.permission.BLUETOOTH_CONNECT,
                    Manifest.permission.BLUETOOTH_ADVERTISE
                ),
                requestCode
            )
        }
    }

    /**
     * Request User to Turn On Location
     */
    private fun enableLocation(context: Activity, locationReqCode: Int) {
        val googleApiClient: GoogleApiClient = GoogleApiClient.Builder(context)
            .addApi(LocationServices.API).build()
        googleApiClient.connect()

        val locationRequest: LocationRequest = LocationRequest.create()
        locationRequest.priority = LocationRequest.PRIORITY_HIGH_ACCURACY
        locationRequest.interval = 10000
        locationRequest.fastestInterval = (10000 / 2).toLong()

        val builder: LocationSettingsRequest.Builder =
            LocationSettingsRequest.Builder().addLocationRequest(locationRequest)
        builder.setAlwaysShow(true)
        val activity1 = context
        val result: PendingResult<LocationSettingsResult> =
            LocationServices.SettingsApi.checkLocationSettings(googleApiClient, builder.build())
        result.setResultCallback { result ->
            val status = result.status
            when (status.statusCode) {
                LocationSettingsStatusCodes.SUCCESS -> AppLog.i(
                    TAG,
                    "All location settings are satisfied."
                )
                LocationSettingsStatusCodes.RESOLUTION_REQUIRED -> {
                    AppLog.i(
                        TAG,
                        "Location settings are not satisfied. Show the user a dialog to upgrade location settings "
                    )
                    try {
                        // Show the dialog by calling startResolutionForResult(), and check the result
                        // in onActivityResult().
                        status.startResolutionForResult(activity1, locationReqCode)
                    } catch (e: IntentSender.SendIntentException) {
                        AppLog.i(
                            TAG,
                            "PendingIntent unable to execute request."
                        )
                    }
                }
                LocationSettingsStatusCodes.SETTINGS_CHANGE_UNAVAILABLE -> AppLog.i(
                    TAG,
                    "Location settings are inadequate, and cannot be fixed here. Dialog not created."
                )
            }
        }
    }

    /**
     * Checks if BLE Regarding All the Requirements Are Satisfies or Not
     */
    private fun isBleScanConditionSatisfy(): Boolean {
        var isBleScanConditionSatisfy = true
        if (!isBLeSupported()) {
            AppLog.e(TAG, "isBLeSupport : false")
            RemoteLog[context]!!.addDebugLog("$TAG:isBLeSupport : false")
//            onScanfailed(BLEErrorType.BLE_NO_SUPPORTED)
            isBleScanConditionSatisfy = false
        }
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            if (!isBluetoothPermissionAllowed()) {
                AppLog.e(
                    TAG,
                    "isBluetoothPermissionGranted() : false"
                )
                RemoteLog[context]!!.addDebugLog("$TAG:isBluetoothPermissionGranted() : false")
                isBleScanConditionSatisfy = false
                enableBlePermission(context, BLE_PERMISSION_REQ_CODE_12)
                return false
            }
        } else {
            if (!isLocationPermissionAllowed()) {
                AppLog.e(
                    TAG,
                    "isLocationPermissionGranted() : false"
                )
                RemoteLog[context]!!.addDebugLog("$TAG:isLocationPermissionGranted() : false")
                isBleScanConditionSatisfy = false
                enableLocationPermission(context, LOCATION_PERMISSION_REQ_CODE)
                return false
            }
        }
        if (!isLocationEnable()) {
            AppLog.e(TAG, "isLocationEnable : false")
            RemoteLog[context]!!.addDebugLog("$TAG:isLocationEnable : false")
            isBleScanConditionSatisfy = false
            enableLocation(context, LOCATION_ENABLE_REQ_CODE)
            return false
        }
        if (!isBluetoothEnable()) {
            AppLog.e(TAG, "isBluetoothEnable : false")
            RemoteLog[context]!!.addDebugLog("$TAG:isBluetoothEnable : false")
            isBleScanConditionSatisfy = false
            enableBluetooth(context, REQUEST_ENABLE_BT)
            return false
        }
        return isBleScanConditionSatisfy
    }

    fun onResult(requestCode: Int, status: Boolean) {
        when {
            !status -> {
                when (requestCode) {
                    LOCATION_PERMISSION_REQ_CODE -> {
                        errorText = "Location permission is not granted"
                        noRights = true
                    }
                    BLE_PERMISSION_REQ_CODE_12 -> {
                        errorText = "BLE permissions are not granted"
                        noRights = true
                    }
                    LOCATION_ENABLE_REQ_CODE -> {
                        errorText = "Location is not enabled"
                        noRights = true
                    }
                    REQUEST_ENABLE_BT -> {
                        errorText = "Bluetooth is not enabled"
                        noRights = true
                    }
                }
                val bleRes = BleOuterClass.Ble.newBuilder()
                val startResult = BleOuterClass.BleStartResult.newBuilder()
                startResult.success = false
                if (noRights) {
                    startResult.errorReason = BleOuterClass.BleError.RIGHTS_MISSING
                } else {
                    startResult.errorReason = BleOuterClass.BleError.UNKNOWN_ERROR
                }
                startResult.errorMessage = errorText
                bleRes.startResult = startResult.build()
                sendResponse(bleRes)
            }
            else -> {
                startService(context = context)
            }
        }
    }
}

