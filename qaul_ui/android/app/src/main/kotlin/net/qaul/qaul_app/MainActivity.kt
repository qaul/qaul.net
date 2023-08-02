package net.qaul.qaul_app

import android.Manifest
import android.content.DialogInterface
import android.content.Intent
import android.content.pm.PackageManager

import io.flutter.embedding.android.FlutterActivity
import io.flutter.embedding.engine.FlutterEngine
import io.flutter.plugin.common.MethodChannel

import net.qaul.ble.AppLog
import net.qaul.ble.core.BleWrapperClass
import net.qaul.libqaul.*

import android.os.Build
import android.os.Bundle

import com.google.android.material.dialog.MaterialAlertDialogBuilder

import androidx.annotation.NonNull
import androidx.annotation.RequiresApi
import androidx.appcompat.app.AppCompatActivity
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat

class MainActivity : FlutterActivity() {
    private val CHANNEL = "libqaul"
    private var bleWrapperClass: BleWrapperClass? = null
    private var flutterEngine: FlutterEngine? = null

    companion object {
        const val LOCATION_PERMISSION_REQ_CODE = 111
        const val LOCATION_ENABLE_REQ_CODE = 112
        const val REQUEST_ENABLE_BT = 113
        const val BLE_PERMISSION_REQ_CODE_12 = 114

        lateinit var permissionHandler: PermissionHandler
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        permissionHandler = PermissionHandler(this)
        // Check if the background service is enabled in preferences, if so, start it
        if (PreferenceManager.isBackgroundServiceEnabled(this)) {
            startBackgroundService()
        }
    }

    @RequiresApi(Build.VERSION_CODES.M)
    private fun showLocationPermissionDialog() {
        val builder: MaterialAlertDialogBuilder = MaterialAlertDialogBuilder(this)
        builder.setTitle("About the background execution and location permission")
        builder.setMessage("We need access to your location to provide better service.")
        builder.setPositiveButton(
                "OK"
        ) { dialog: DialogInterface, _: Int ->
            dialog.dismiss()
            //initialize BleModule initialize -- must be before startLibqaul()
            bleWrapperClass = BleWrapperClass(context = this)
            // load libqaul
            libqaulLoad()
        }
        builder.setCancelable(false)
        builder.show()
    }

    override fun configureFlutterEngine(@NonNull FlutterEngine: FlutterEngine) {
        super.configureFlutterEngine(FlutterEngine)
        this.flutterEngine = FlutterEngine

        // load libqaul
        libqaulLoad()

        //initialize BleModule initialize -- must be before startLibqaul()
        bleWrapperClass = BleWrapperClass(context = this)

        // setup message channel between flutter and android
        MethodChannel(
            FlutterEngine.dartExecutor.binaryMessenger, CHANNEL
        ).setMethodCallHandler { call, result ->
            when {
                // utility methods
                call.method == "isBackgroundExecutionEnabled" -> {
                    result.success(PreferenceManager.isBackgroundServiceEnabled(this))
                }
                call.method == "enableBackgroundExecution" -> {
                    if (!PreferenceManager.isBackgroundServiceEnabled(this)) {
                        startBackgroundService()
                        PreferenceManager.setBackgroundServiceEnabled(this, true)
                    }
                    result.success(true)
                }
                call.method == "disableBackgroundExecution" -> {
                    if (PreferenceManager.isBackgroundServiceEnabled(this)) {
                        stopBackgroundService()
                        PreferenceManager.setBackgroundServiceEnabled(this, false)
                    }
                    result.success(true)
                }

                // libqaul adapter methods
                call.method == "getPlatformVersion" -> {
                    val res = getSystemVersion()
                    result.success(res)
                }
                call.method == "loadlibrary" -> {
                    libqaulLoad()
                    result.success(true)
                }
                call.method == "hello" -> {
                    val res = getHello()
                    result.success(res)
                }
                call.method == "start" -> {
                    startLibqaul()
                    result.success(true)
                }
                call.method == "initialized" -> {
                    val res = initializedLibqaul()
                    result.success(res)
                }
                call.method == "sendcounter" -> {
                    val res = getSendCounter()
                    result.success(res)
                }
                call.method == "receivequeue" -> {
                    val res = getReceiveCounter()
                    result.success(res)
                }
                call.method == "sendRpcMessage" -> {
                    // get argument
                    val message = call.argument<ByteArray>("message")
                    val bytes = message ?: byteArrayOf()
                    // send it to libqaul
                    sendRpcMessage(bytes)
                    result.success(true)
                }
                call.method == "receiveRpcMessage" -> {
                    val res = receiveRpcMessage()
                    result.success(res)
                }
                else -> result.notImplemented()
            }
        }
    }

    private fun startBackgroundService() {
        val serviceIntent = Intent(this, FlutterBackgroundService::class.java)
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            startForegroundService(serviceIntent)
        } else {
            startService(serviceIntent)
        }
    }

    private fun stopBackgroundService() {
        val serviceIntent = Intent(this, FlutterBackgroundService::class.java)
        stopService(serviceIntent)
    }

    /// get android system version
    private fun getSystemVersion(): String {
        return "Android ${android.os.Build.VERSION.RELEASE}"
    }

    /// Load shared libqaul library
    /// this needs to be invoked before any other function
    private fun libqaulLoad() {
        loadLibqaul()
    }

    /// get dummy hello function from libqaul
    private fun getHello(): String {
        return hello()
    }

    /// start libqaul
    private fun startLibqaul() {
        // get path to storage directory
        val storagePath = context.filesDir.absolutePath

        println("Initialize libqaul with storage path: $storagePath")

        // start libqaul
        start(storagePath)
    }

    /// check if libqaul has finished initializing
    private fun initializedLibqaul(): Boolean {
        return initialized()
    }

    /// get message send counter from libqaul
    private fun getSendCounter(): Int {
        return sendcounter()
    }

    /// get message receive counter from libqaul
    private fun getReceiveCounter(): Int {
        return receivequeue()
    }

    /// send an RPC message to libqaul
    private fun sendRpcMessage(message: ByteArray) {
        send(message)
    }

    /// receive an RPC message from libqaul
    private fun receiveRpcMessage(): ByteArray {
        return receive()
    }

    override fun onRequestPermissionsResult(
        requestCode: Int,
        permissions: Array<out String>,
        grantResults: IntArray
    ) {
        super.onRequestPermissionsResult(requestCode, permissions, grantResults)
        permissionHandler.onRequestPermissionsResult(requestCode, permissions, grantResults)

        if (requestCode == LOCATION_PERMISSION_REQ_CODE) {
            AppLog.e(
                "MainActivity", "REQ CODED -  " + requestCode + "  Size  " + grantResults.size
            )
            if (grantResults.isNotEmpty()) {
                for (grantResult in grantResults) {
                    if (grantResult == PackageManager.PERMISSION_DENIED) {
                        AppLog.e("MainActivity", "grantResults- IF $grantResult")
                        bleWrapperClass?.onResult(requestCode = requestCode, status = false)
                        break
                    }
                }
                bleWrapperClass?.onResult(requestCode = requestCode, status = true)
            }
        } else if (requestCode == BLE_PERMISSION_REQ_CODE_12) {
            AppLog.e(
                "MainActivity", "REQ CODED -  " + requestCode + "  Size  " + grantResults.size
            )
            if (grantResults.isNotEmpty()) {
                for (grantResult in grantResults) {
                    if (grantResult == PackageManager.PERMISSION_DENIED) {
                        AppLog.e("MainActivity", "grantResults- IF $grantResult")
                        bleWrapperClass?.onResult(requestCode = requestCode, status = false)
                        break
                    }
                }
                bleWrapperClass?.onResult(requestCode = requestCode, status = true)
            }
        }
    }

    /**
     * This Method Will Be Called When User Accept/Decline Asked to Turn On
     * Bluetooth and/or Location(GPS) From BLEModule
     * After Response It Will Send User's Response to BLEModule
     */
    override fun onActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {
        super.onActivityResult(requestCode, resultCode, data)
        AppLog.e(
            "MainActivity", "onActivityResult requestCode=$requestCode | resultCode=$resultCode"
        )
        if (requestCode == LOCATION_ENABLE_REQ_CODE) {
            if (resultCode == RESULT_OK) {
                AppLog.e("MainActivity", "Location Yes")
                bleWrapperClass?.onResult(requestCode = requestCode, status = true)
            } else {
                AppLog.e("MainActivity", "Location No")
                bleWrapperClass?.onResult(requestCode = requestCode, status = false)
            }
        } else if (requestCode == REQUEST_ENABLE_BT) {
            if (resultCode == RESULT_OK) {
                AppLog.e("MainActivity", "BT Yes")
                bleWrapperClass?.onResult(requestCode = requestCode, status = true)
            } else {
                AppLog.e("MainActivity", "BT No")
                bleWrapperClass?.onResult(requestCode = requestCode, status = false)
            }
        }
    }
}
