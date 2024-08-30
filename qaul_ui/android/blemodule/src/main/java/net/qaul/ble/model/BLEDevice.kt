// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

package net.qaul.ble.model

import android.bluetooth.BluetoothDevice
import android.bluetooth.le.ScanResult

/**
 * This class contains Data for device found
 */
class BLEDevice : BLEScanDevice() {
    override var deviceRSSI: Int = 0
    override var scanResult: ScanResult? = null
    override var name: String? = null
    override var macAddress: String? = null
    var intervalNanos: Long = 0
        private set
    override var bluetoothDevice: BluetoothDevice? = null
    override var isConnectable = true
    override var lastFoundTime: Long? = null
    override var qaulId: ByteArray? = null
    override var isConnected = false

    companion object {
        private const val TAG = "qaul-blemodule BLEScanDevice"
    }

    override fun toString(): String {
        return "BLEScanDevice(deviceRSSI=$deviceRSSI, scanResult=$scanResult, name=$name, macAddress=$macAddress, intervalNanos=$intervalNanos, bluetoothDevice=$bluetoothDevice, isConnectable=$isConnectable, lastFoundTime=$lastFoundTime, qaulId=${qaulId?.contentToString()})"
    }

}