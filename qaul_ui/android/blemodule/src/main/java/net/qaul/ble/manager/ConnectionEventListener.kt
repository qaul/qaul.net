package net.qaul.ble.test.ble.manager

import android.bluetooth.BluetoothDevice
import android.bluetooth.BluetoothGatt
import android.bluetooth.BluetoothGattCharacteristic

interface ConnectionEventListener {
    fun onConnectionSetupComplete(gatt: BluetoothGatt) {}
    fun onServicesDiscovered(device: BluetoothDevice) {}
    fun onDisconnectedFromDevice(device: BluetoothDevice) {}
    fun onCharacteristicRead(device: BluetoothDevice, characteristic: BluetoothGattCharacteristic, value: ByteArray) {}
    fun onCharacteristicWrite(device: BluetoothDevice, characteristic: BluetoothGattCharacteristic) {}
    fun onMessageAssembled(device: BluetoothDevice, payload: ByteArray) {}
    fun onNotificationsEnabled(device: BluetoothDevice, characteristic: BluetoothGattCharacteristic) {}
    fun onNotificationsDisabled(device: BluetoothDevice, characteristic: BluetoothGattCharacteristic) {}
    fun onNotificationReceived(device: BluetoothDevice, characteristic: BluetoothGattCharacteristic, value: ByteArray) {}
    fun onMtuChanged(device: BluetoothDevice, newMtu: Int) {}
}
