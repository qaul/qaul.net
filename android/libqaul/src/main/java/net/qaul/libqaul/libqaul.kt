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
import net.qaul.libqaul.lib.AndroidBindings
import qaul.sys.ble.BleOuterClass

fun loadLibqaul() {
    println("load libqaul")
    System.loadLibrary("libqaul")
    AndroidBindings.initialiseLogging()
    println("loaded libqaul")

/*
    val bleReq: BleOuterClass.Ble.Builder = BleOuterClass.Ble.newBuilder()
    bleReq.infoRequest = BleOuterClass.BleInfoRequest.getDefaultInstance()
    bleWrapperClass.receiveRequest(data = bleReq.build().toByteString(), callback = this)
*/
}