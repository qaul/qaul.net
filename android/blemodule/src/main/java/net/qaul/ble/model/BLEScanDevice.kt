package net.qaul.ble.model

import android.bluetooth.BluetoothDevice
import android.bluetooth.le.ScanResult
import androidx.databinding.BaseObservable
import androidx.databinding.ObservableInt

abstract class BLEScanDevice : BaseObservable() {
    companion object {
        fun getDevice(): BLEScanDevice {
            return BLEDevice()
        }
    }
    abstract var deviceRSSI: ObservableInt
    abstract var scanResult: ScanResult?
    abstract var bluetoothDevice: BluetoothDevice?
    abstract var name: String?
    abstract var macAddress: String?
    abstract var isConnectable: Boolean
}