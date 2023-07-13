package net.qaul.qaul_app

import android.app.Activity
import android.content.Context
import android.content.pm.PackageManager
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat

class PermissionHandler(private val context: Context) {
    companion object {
        private const val WIFI_PERMISSION_REQUEST_CODE = 1001
        private val REQUIRED_PERMISSIONS = arrayOf(
            android.Manifest.permission.ACCESS_WIFI_STATE,
            android.Manifest.permission.CHANGE_WIFI_STATE
        )
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
            ActivityCompat.requestPermissions(
                context as Activity,
                permissionsToRequest.toTypedArray(),
                WIFI_PERMISSION_REQUEST_CODE
            )
        }
    }

    fun onRequestPermissionsResult(
        requestCode: Int,
        permissions: Array<out String>,
        grantResults: IntArray
    ) {
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
}
