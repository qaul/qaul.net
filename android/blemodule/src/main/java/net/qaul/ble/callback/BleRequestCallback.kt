package net.qaul.ble.callback

import qaul.sys.ble.BleOuterClass

interface BleRequestCallback {
    fun bleResponse(ble: BleOuterClass.Ble)
}