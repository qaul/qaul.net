package net.qaul.ble.core

import android.content.Context
import android.content.Intent
import android.util.Log
import net.qaul.ble.service.BleService
import qaul.sys.ble.BleOuterClass

class BleWrapperClass() {
    companion object {
        lateinit var bleService: BleService
        fun startService(context: Context) {
            BleService().start(context)
        }

        val serviceManager = this
    }

    fun getRequest(bleReq: BleOuterClass.Ble) {
        if (bleReq.isInitialized) {
            if (bleReq.messageCase == BleOuterClass.Ble.MessageCase.INFO_REQUEST) {
                Log.e("BleWrapperClass", bleReq.messageCase.toString())
                getDeviceInfo()
            }
        }
    }

    private fun getDeviceInfo() {
        val bleRes: BleOuterClass.Ble.Builder = BleOuterClass.Ble.newBuilder()
        bleRes.infoResponse = BleOuterClass.BleInfoResponse.getDefaultInstance()
        if (bleRes.infoResponse.isInitialized) {
//            bleRes.infoResponse.
        } else {
            false
        }
    }
}