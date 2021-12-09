package net.qaul.ble.model

import android.bluetooth.BluetoothDevice
import android.bluetooth.le.ScanResult
import androidx.databinding.ObservableInt

/**
 * This class contains Data for device found
 */
class BLEDevice : BLEScanDevice() {
    override var deviceRSSI: ObservableInt = ObservableInt()
    override var scanResult: ScanResult? = null
    override var name: String? = null
    override var macAddress: String? = null
    var intervalNanos: Long = 0
        private set
    override var bluetoothDevice: BluetoothDevice? = null
    override var isConnectable = true

    override fun toString(): String {
        return "BlueDevice{" +
                "scanResult=" + scanResult +
                ", intervalNanos=" + intervalNanos +
                '}'
    }



    /**
     * get advertise interval of device in millisecond
     * @return
     */

    companion object {
        private const val TAG = "BlueDevice"
    }
}