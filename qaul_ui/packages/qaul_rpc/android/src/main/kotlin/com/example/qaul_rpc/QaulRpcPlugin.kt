// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

/// Main entry point of the qaul android app
///
/// It starts and configures the flutter GUI,
/// set's up the communication channels,
/// and starts libqaul via the android libqaul AAR.
/// Everything in this function is running in the main thread.

package com.example.qaul_rpc

import androidx.annotation.NonNull

import io.flutter.embedding.engine.plugins.FlutterPlugin
import io.flutter.plugin.common.MethodCall
import io.flutter.plugin.common.MethodChannel
import io.flutter.plugin.common.MethodChannel.MethodCallHandler
import io.flutter.plugin.common.MethodChannel.Result

// import the libqaul AAR android library
import net.qaul.libqaul.*

/** QaulRpcPlugin */
class QaulRpcPlugin : FlutterPlugin, MethodCallHandler {
    /// The MethodChannel that will the communication between Flutter and native Android
    ///
    /// This local reference serves to register the plugin with the Flutter Engine and unregister it
    /// when the Flutter Engine is detached from the Activity
    private lateinit var channel: MethodChannel

    override fun onAttachedToEngine(@NonNull flutterPluginBinding: FlutterPlugin.FlutterPluginBinding) {
        // load libqaul
        libqaulLoad()

        channel = MethodChannel(flutterPluginBinding.binaryMessenger, "qaul_rpc")
        channel.setMethodCallHandler(this)
    }

    override fun onMethodCall(@NonNull call: MethodCall, @NonNull result: Result) {
        if (call.method == "getPlatformVersion") {
            result.success("Android ${android.os.Build.VERSION.RELEASE}")
        } else if (call.method == "loadlibrary") {
            libqaulLoad()
            result.success(true)
        } else if (call.method == "hello") {
            val res = getHello()
            result.success(res)
        } else if (call.method == "start") {
            startLibqaul()
            result.success(true)
        } else if (call.method == "initialized") {
            val res = initializedLibqaul()
            result.success(res)
        } else if (call.method == "sendcounter") {
            val res = getSendCounter()
            result.success(res)
        } else if (call.method == "receivequeue") {
            val res = getReceiveCounter()
            result.success(res)
        } else if (call.method == "sendRpcMessage") {
            // get argument
            val message = call.argument<ByteArray>("message")
            val bytes = message ?: byteArrayOf()
            // send it to libqaul
            sendRpcMessage(bytes)
            result.success(true)
        } else if (call.method == "receiveRpcMessage") {
            val res = receiveRpcMessage()
            result.success(res)
        } else {
            result.notImplemented()
        }
    }

    override fun onDetachedFromEngine(@NonNull binding: FlutterPlugin.FlutterPluginBinding) {
        channel.setMethodCallHandler(null)
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
        start()
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
