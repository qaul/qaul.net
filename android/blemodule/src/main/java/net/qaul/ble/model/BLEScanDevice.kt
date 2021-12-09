package net.qaul.ble.model

import android.bluetooth.BluetoothDevice
import android.bluetooth.le.ScanResult

abstract class BLEScanDevice  {
    companion object {
        fun getDevice(): BLEScanDevice {
            return BLEDevice()
        }
    }
    abstract var deviceRSSI: Int
    abstract var scanResult: ScanResult?
    abstract var bluetoothDevice: BluetoothDevice?
    abstract var name: String?
    abstract var macAddress: String?
    abstract var isConnectable: Boolean
}