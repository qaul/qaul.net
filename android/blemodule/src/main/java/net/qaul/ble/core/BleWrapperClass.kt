package net.qaul.ble.core

import android.Manifest
import android.app.Activity
import android.bluetooth.BluetoothAdapter
import android.bluetooth.BluetoothManager
import android.content.Context
import android.content.Intent
import android.content.IntentSender
import android.content.pm.PackageManager
import android.location.LocationManager
import android.os.Build
import android.util.Log
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity
import androidx.core.app.ActivityCompat
import com.google.android.gms.common.api.GoogleApiClient
import com.google.android.gms.common.api.PendingResult
import com.google.android.gms.location.*
import net.qaul.ble.AppLog
import net.qaul.ble.RemoteLog
import net.qaul.ble.service.BleService
import qaul.sys.ble.BleOuterClass
import java.lang.Exception
import android.R.attr.capitalize
import android.bluetooth.le.BluetoothLeAdvertiser
import android.os.Handler
import net.qaul.ble.callback.BleRequestCallback


class BleWrapperClass(context: AppCompatActivity) {
    private val TAG: String = BleWrapperClass.javaClass.simpleName
    private val context = context
    private var errorText = ""
    private var noRights = false
    private var bleCallback: BleRequestCallback? = null
    private var qaulId = "qaul_id"

    /**
     * Static Member Declaration
     */
    companion object {
        val serviceManager = this

        const val LOCATION_PERMISSION_REQ_CODE = 111
        const val LOCATION_ENABLE_REQ_CODE = 112
        const val REQUEST_ENABLE_BT = 113
        const val BLE_PERMISSION_REQ_CODE_12 = 114
    }

    /**
     * This Method get BLERequest from UI & Return BLEResponse by Callback Interface Method
     */
    fun receiveRequest(bleReq: BleOuterClass.Ble, param: BleRequestCallback) {
        if (bleReq.isInitialized) {
            bleCallback = param
            if (bleReq.messageCase == BleOuterClass.Ble.MessageCase.INFO_REQUEST) {
                Log.e(TAG, bleReq.messageCase.toString())
                getDeviceInfo()
            } else if (bleReq.messageCase == BleOuterClass.Ble.MessageCase.START_REQUEST) {
                qaulId = bleReq.startRequest.qaulId
                startService(context = context)
            }

        }
    }

    private fun startService(context: Context) {
        if (isBleScanConditionSatisfy()) {
            if (!BleService().isRunning()) {
                BleService().start(context)
                Handler().postDelayed(Runnable {
                    BleService().setData(qaulId, bleCallback) }, 500)
            } else {
                BleService().setupAdvertiser()
            }
        }
    }


    /**
     * This Method Return Device Information Regarding BLE Functionality & Permissions
     */
    private fun getDeviceInfo() {
        val adapter = BluetoothAdapter.getDefaultAdapter()
        val bleRes: BleOuterClass.Ble.Builder = BleOuterClass.Ble.newBuilder()
        val bleResInfoResponse = BleOuterClass.BleInfoResponse.newBuilder()
        if (bleRes.isInitialized) {
            if (bleResInfoResponse.isInitialized) {
                val deviceInfoBuilder: BleOuterClass.BleDeviceInfo.Builder =
                    BleOuterClass.BleDeviceInfo.newBuilder()
                deviceInfoBuilder.locationPermission = isLocationPermissionAllowed()
                deviceInfoBuilder.locationOn = isLocationEnable()
                deviceInfoBuilder.blePermission = isBluetoothPermissionAllowed()
                deviceInfoBuilder.bluetoothOn = isBluetoothEnable()
                deviceInfoBuilder.androidVersion = getOsVersion()
                deviceInfoBuilder.name = getDeviceName()
                deviceInfoBuilder.bleSupport = isBLeSupported()
                if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
                    deviceInfoBuilder.adv251 = adapter.leMaximumAdvertisingDataLength > 250
                    deviceInfoBuilder.adv1M = adapter.isLeExtendedAdvertisingSupported
                    deviceInfoBuilder.adv2M = adapter.isLe2MPhySupported
                    deviceInfoBuilder.advCoded = adapter.isLeCodedPhySupported
                    deviceInfoBuilder.advExtendedBytes = adapter.leMaximumAdvertisingDataLength

                    //Return true if LE Periodic Advertising feature is supported.
                    deviceInfoBuilder.lePeriodicAdvSupport =
                        adapter.isLePeriodicAdvertisingSupported

                    //Return true if the multi advertisement is supported by the chipset
                    deviceInfoBuilder.leMultipleAdvSupport =
                        adapter.isMultipleAdvertisementSupported

                    //Return true if offloaded filters are supported true if chipset supports on-chip filtering
                    deviceInfoBuilder.offloadFilterSupport = adapter.isOffloadedFilteringSupported

                    //Return true if offloaded scan batching is supported true if chipset supports on-chip scan batching
                    deviceInfoBuilder.offloadScanBatchingSupport =
                        adapter.isOffloadedScanBatchingSupported
                } else {
                    deviceInfoBuilder.adv251 = false
                    deviceInfoBuilder.adv1M = false
                    deviceInfoBuilder.adv2M = false
                    deviceInfoBuilder.advCoded = false
                    deviceInfoBuilder.advExtendedBytes = 20
                    deviceInfoBuilder.leAudio = false
                }
                if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
                    deviceInfoBuilder.leAudio = isClass("android.bluetooth.BluetoothLeAudio")
                }
                bleResInfoResponse.device = deviceInfoBuilder.build()
            }
            bleRes.infoResponse = bleResInfoResponse.build()
            bleCallback?.bleResponse(ble = bleRes.build())
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
        } catch (ex: java.lang.Exception) {
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
        return if (model.toLowerCase().startsWith(manufacturer.toLowerCase())) {
            capitalize(model)
        } else {
            capitalize(manufacturer).toString() + " " + model
        }
    }

    /**
     * Capitalize 1st Letter of String
     */
    private fun capitalize(s: String?): String? {
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
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.M && permissions != null) {
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
        locationRequest.setPriority(LocationRequest.PRIORITY_HIGH_ACCURACY)
        locationRequest.setInterval(10000)
        locationRequest.setFastestInterval((10000 / 2).toLong())

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
                        noRights = false
                    }
                    REQUEST_ENABLE_BT -> {
                        errorText = "Bluetooth is not enabled"
                        noRights = false
                    }
                }
                val bleRes = BleOuterClass.Ble.newBuilder()
                val startResult = BleOuterClass.BleStartResult.newBuilder()
                startResult.success = false
                startResult.noRights = noRights
                startResult.errorMessage = errorText
                startResult.unknonwError = false
                bleRes.startResult = startResult.build()
                bleCallback?.bleResponse(ble = bleRes.build())
            }
            else -> {
                startService(context = context)
            }
        }
    }

}