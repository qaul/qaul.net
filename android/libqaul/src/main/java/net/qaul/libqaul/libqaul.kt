package net.qaul.libqaul

// qaul ble module

import net.qaul.ble.AppLog
import net.qaul.ble.callback.BleRequestCallback
import net.qaul.ble.core.BleWrapperClass
import net.qaul.ble.core.BleWrapperClass.Companion.BLE_PERMISSION_REQ_CODE_12
import net.qaul.ble.core.BleWrapperClass.Companion.LOCATION_ENABLE_REQ_CODE
import net.qaul.ble.core.BleWrapperClass.Companion.LOCATION_PERMISSION_REQ_CODE
import net.qaul.ble.core.BleWrapperClass.Companion.REQUEST_ENABLE_BT
// ble protobuf communication
import com.google.gson.Gson
import com.google.protobuf.ByteString
import qaul.sys.ble.BleOuterClass

/// hello message from lib qaul
/// dummy function for testing
external fun hello(): String

/// start libqaul
/// this also intializes the logging
external fun start(path: String)

/// check if libqaul has finished initializing
external fun initialized(): Boolean

/// get number of RPC messages sent to libqaul
/// this function is only for testing
external fun sendcounter(): Int

/// send an RPC message to libqaul
external fun send(message: ByteArray)

/// how many RPC messages are queued by libqaul
/// to be received from this programme
external fun receivequeue(): Int

/// receive an RPC message from libqaul
external fun receive(): ByteArray

/// send an SYS message from BLE library to libqaul
external fun syssend(message: ByteArray)

/// how many SYS messages are queued by libqaul
/// to be received from BLE module
external fun sysreceivequeue(): Int

/// receive an SYS message from libqaul to BLE library
external fun sysreceive(): ByteArray

/// load rust libqaul shared library
fun loadLibqaul() {
    println("load libqaul")
    System.loadLibrary("libqaul")
    println("libqaul loaded")

    println("BLE test")
/*
    val bleReq: BleOuterClass.Ble.Builder = BleOuterClass.Ble.newBuilder()
    bleReq.infoRequest = BleOuterClass.BleInfoRequest.getDefaultInstance()
    bleWrapperClass.receiveRequest(data = bleReq.build().toByteString(), callback = this)
*/
    println("BLE test sent")
}
