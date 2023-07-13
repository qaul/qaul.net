// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

/// Main entry point of the qaul android app
///
/// It starts and configures the flutter GUI,
/// set's up the communication channels,
/// and starts libqaul via the android libqaul AAR.
/// Everything in this function is running in the main thread.

package net.qaul.qaul_app

import androidx.annotation.NonNull
import io.flutter.embedding.android.FlutterActivity
import io.flutter.embedding.engine.FlutterEngine
import io.flutter.plugin.common.MethodChannel

// import the libqaul AAR android library
import net.qaul.libqaul.*
import net.qaul.ble.core.BleWrapperClass
import net.qaul.ble.AppLog
import net.qaul.ble.RemoteLog

import android.content.pm.PackageManager
import android.content.Intent
import android.os.Build
import android.os.Bundle

class MainActivity: FlutterActivity() {
    private val CHANNEL = "libqaul"
    private var bleWrapperClass: BleWrapperClass? = null
    private var flutterEngine: FlutterEngine? = null

    companion object{
        const val LOCATION_PERMISSION_REQ_CODE = 111
        const val LOCATION_ENABLE_REQ_CODE = 112
        const val REQUEST_ENABLE_BT = 113
        const val BLE_PERMISSION_REQ_CODE_12 = 114

        lateinit var permissionHandler: PermissionHandler
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        permissionHandler = PermissionHandler(this)
    }

    override fun configureFlutterEngine(@NonNull FlutterEngine: FlutterEngine) {
        super.configureFlutterEngine(FlutterEngine)
        this.flutterEngine = FlutterEngine

        // load libqaul
        libqaulLoad()
		
		//initialize BleModule initialize -- must be before startLibqaul()
        bleWrapperClass = BleWrapperClass(context = this)

        // Start the FlutterBackgroundService
        val serviceIntent = Intent(this, FlutterBackgroundService::class.java)
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            startForegroundService(serviceIntent)
        } else {
            startService(serviceIntent)
        }
		
        // setup message channel between flutter and android
        MethodChannel(FlutterEngine.dartExecutor.binaryMessenger, CHANNEL).setMethodCallHandler {
            call, result ->
            when {
                call.method == "exit_app" -> {
                    stopServiceAndFinish()
                    result.success(true)
                }
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

    private fun stopServiceAndFinish() {
        val serviceIntent = Intent(this, FlutterBackgroundService::class.java)
        stopService(serviceIntent)
        flutterEngine?.let { engine ->
            engine.destroy()
            flutterEngine = null
        }
        finishAffinity()
        finish()
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
                "MainActivity",
                "REQ CODED -  " + requestCode + "  Size  " + grantResults.size
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
                "MainActivity",
                "REQ CODED -  " + requestCode + "  Size  " + grantResults.size
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
            "MainActivity",
            "onActivityResult requestCode=$requestCode | resultCode=$resultCode"
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
