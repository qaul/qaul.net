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

class MainActivity: FlutterActivity() {
    private val CHANNEL = "libqaul"

    override fun configureFlutterEngine(@NonNull FlutterEngine: FlutterEngine) {
        super.configureFlutterEngine(FlutterEngine)

        // load libqaul
        libqaulLoad()

        // setup message channel between flutter and android
        MethodChannel(FlutterEngine.dartExecutor.binaryMessenger, CHANNEL).setMethodCallHandler {
            call, result ->
            when {
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
        libqaulLoad()
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
}
