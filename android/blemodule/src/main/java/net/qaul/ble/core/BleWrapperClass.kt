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

class BleWrapperClass(context: AppCompatActivity) {
    private val TAG: String = BleWrapperClass.javaClass.simpleName
    private val context = context
    private var isBleSupported: Boolean = false
    private var isLocationPermission: Boolean = false
    private var isLocationOn: Boolean = false
    private var isAdvertPermission : Boolean = false
    private var isBluetoothOn : Boolean = false
    companion object {
        lateinit var bleService: BleService
        fun startService(context: Context) {
            BleService().start(context)
        }

        val serviceManager = this

        private const val BLE_PERMISSION_REQ_CODE = 111
        private const val LOCATION_ENABLE_REQ_CODE = 112
        private const val REQUEST_ENABLE_BT = 113
        private const val BLE_PERMISSION_REQ_CODE_12 = 114
    }

    fun getRequest(bleReq: BleOuterClass.Ble) {
        if (bleReq.isInitialized) {
            if (bleReq.messageCase == BleOuterClass.Ble.MessageCase.INFO_REQUEST) {
                Log.e(TAG, bleReq.messageCase.toString())
                getDeviceInfo()
            }
        }
    }

    private fun getDeviceInfo() {
        isBleScanConditionSatisfy()
        val bleRes: BleOuterClass.Ble.Builder = BleOuterClass.Ble.newBuilder()
        bleRes.infoResponse = BleOuterClass.BleInfoResponse.getDefaultInstance()
        if (bleRes.infoResponse.isInitialized) {
//            bleRes.infoResponse.
        } else {
            false
        }
    }

    private fun isBLeSupported(): Boolean {
        return context.packageManager.hasSystemFeature(PackageManager.FEATURE_BLUETOOTH_LE)
    }

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

//    private fun isScanRunning(): Boolean {
//        return mScanning
//    }

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

    private fun disableBluetooth(): Boolean {
        val bluetoothManager =
            context.getSystemService(Context.BLUETOOTH_SERVICE) as BluetoothManager
        val bluetoothAdapter = bluetoothManager.adapter
        bluetoothAdapter?.disable()
        return false
    }

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
        return hasPermission(
            arrayOf()
        )
    }

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

    fun isBleScanConditionSatisfy(): Boolean {
        var isBleScanConditionSatisfy = true
        if (!isBLeSupported()) {
            AppLog.e(TAG, "isBLeSupport : false")
            RemoteLog[context]!!.addDebugLog("$TAG:isBLeSupport : false")
//            onScanfailed(BLEErrorType.BLE_NO_SUPPORTED)
            isBleScanConditionSatisfy = false
            return false
        }
        if (!isBluetoothEnable()) {
            AppLog.e(TAG, "isBluetoothEnable : false")
            RemoteLog[context]!!.addDebugLog("$TAG:isBluetoothEnable : false")
            isBleScanConditionSatisfy = false
            enableBluetooth(context, REQUEST_ENABLE_BT)
            return false
        }
        if (!isLocationPermissionAllowed()) {
            AppLog.e(
                TAG,
                "isLocationPermissionGranted() : false"
            )
            RemoteLog[context]!!.addDebugLog("$TAG:isLocationPermissionGranted() : false")
            isBleScanConditionSatisfy = false
            enableLocationPermission(context, BLE_PERMISSION_REQ_CODE)
            return false
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
        }
        if (!isLocationEnable()) {
            AppLog.e(TAG, "isLocationEnable : false")
            RemoteLog[context]!!.addDebugLog("$TAG:isLocationEnable : false")
            isBleScanConditionSatisfy = false
            enableLocation(context, LOCATION_ENABLE_REQ_CODE)
            return false
        }

        return isBleScanConditionSatisfy
    }
}