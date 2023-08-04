package net.qaul.qaul_app

import android.Manifest
import android.app.Activity
import android.content.Context
import android.content.pm.PackageManager
import android.os.Build
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat

class PermissionHandler(private val context: Context) {
    companion object {
        private const val BLE_PERMISSION_REQ_CODE_12 = 114
        private const val LOCATION_PERMISSION_REQ_CODE = 111
        private const val WIFI_PERMISSION_REQUEST_CODE = 1001

        private const val LOCATION_ENABLE_REQ_CODE = 112
        private const val REQUEST_ENABLE_BT = 113

        private val REQUIRED_PERMISSIONS = arrayOf(Manifest.permission.ACCESS_WIFI_STATE, Manifest.permission.CHANGE_WIFI_STATE)
    }

    private var permissionCallback: ((Boolean) -> Unit)? = null

    fun checkAndRequestPermissions(callback: (Boolean) -> Unit) {
        permissionCallback = callback

        val permissionsToRequest = mutableListOf<String>()
        for (permission in REQUIRED_PERMISSIONS) {
            val permissionStatus = ContextCompat.checkSelfPermission(context, permission)
            if (permissionStatus != PackageManager.PERMISSION_GRANTED) {
                permissionsToRequest.add(permission)
            }
        }

        if (permissionsToRequest.isEmpty()) {
            // All permissions are already granted
            permissionCallback?.invoke(true)
        } else {
            ActivityCompat.requestPermissions(context as Activity, permissionsToRequest.toTypedArray(), WIFI_PERMISSION_REQUEST_CODE)
        }
    }

    fun onRequestPermissionsResult(requestCode: Int, permissions: Array<out String>, grantResults: IntArray) {
        if (requestCode == WIFI_PERMISSION_REQUEST_CODE) {
            var allPermissionsGranted = true
            for (result in grantResults) {
                if (result != PackageManager.PERMISSION_GRANTED) {
                    allPermissionsGranted = false
                    break
                }
            }
            permissionCallback?.invoke(allPermissionsGranted)
        }
    }

    fun hasLocationPermission() : Boolean {
        val permissionStatus = ContextCompat.checkSelfPermission(context, Manifest.permission.ACCESS_FINE_LOCATION)
        return permissionStatus != PackageManager.PERMISSION_GRANTED
    }

    fun hasBLEPermission() : Boolean {
        val permissionStatus = ContextCompat.checkSelfPermission(context, Manifest.permission.ACCESS_FINE_LOCATION)
        return permissionStatus != PackageManager.PERMISSION_GRANTED
    }

    fun requestLocationPermission() {
        ActivityCompat.requestPermissions(context as Activity, arrayOf(Manifest.permission.ACCESS_FINE_LOCATION), LOCATION_PERMISSION_REQ_CODE)
    }

    fun requestBLEPermission() {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            ActivityCompat.requestPermissions(context as Activity, arrayOf(Manifest.permission.BLUETOOTH_SCAN, Manifest.permission.BLUETOOTH_CONNECT, Manifest.permission.BLUETOOTH_ADVERTISE), BLE_PERMISSION_REQ_CODE_12)
        }
    }
}
