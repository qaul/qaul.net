package net.qaul.ble.core

import android.bluetooth.*
import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.content.IntentFilter
import android.os.Build
import net.qaul.ble.core.BaseBleActor.BleConnectionListener
import net.qaul.ble.core.BaseBleActor
import androidx.localbroadcastmanager.content.LocalBroadcastManager
import net.qaul.ble.AppLog
import net.qaul.ble.BLEUtils
import net.qaul.ble.model.BLEScanDevice
import net.qaul.ble.service.BleService
import java.lang.Exception
import java.util.*

class BaseBleActor(private val mContext: Context, var listener: BleConnectionListener?) {
    private var mBluetoothGatt: BluetoothGatt? = null
    private val descriptorWriteQueue: Queue<BluetoothGattDescriptor>? = LinkedList()
    private var failTimer: Timer? = null
    private var failedTask: ConnectionFailedTask? = null
    var disconnectedFromDevice = false
    var bluetoothDevice: BluetoothDevice? = null
    var bleDevice: BLEScanDevice? = null
    fun disConnectedDevice() {
        if (mBluetoothGatt != null) {
//            disconnectedFromDevice = true;
            refreshDeviceCache(mBluetoothGatt!!)
            mBluetoothGatt!!.disconnect()
        }
    }


    // Use to make connection to device
    fun setDevice(device: BLEScanDevice?) {
        bleDevice = device
        bluetoothDevice = device!!.bluetoothDevice
        connectDevice()
    }

    fun connectDevice(): Boolean {
        AppLog.e(TAG, "connectDevice : $bluetoothDevice")
        if (bluetoothDevice == null) {
            listener!!.onConnectionFailed("")
        }
        failTimer = Timer()
        failedTask = ConnectionFailedTask()
        failTimer!!.schedule(failedTask, 10000)
        try {
            mBluetoothGatt =
                bluetoothDevice!!.connectGatt(
                    mContext,
                    false,
                    mGattCallback,
                    BluetoothDevice.TRANSPORT_LE
                )
        } catch (e: Exception) {
            e.printStackTrace()
        }
        return true
    }


    val mGattCallback: BluetoothGattCallback = object : BluetoothGattCallback() {
        override fun onConnectionStateChange(gatt: BluetoothGatt, status: Int, newState: Int) {
            super.onConnectionStateChange(gatt, status, newState)
            if (newState == BluetoothProfile.STATE_CONNECTED) {
            }
            if (newState == BluetoothProfile.STATE_CONNECTED) {
                AppLog.e(TAG, "onConnectionStateChange: STATE_CONNECTED")
                listener!!.onConnected(bluetoothDevice!!.address)
                try {
                    if (failedTask != null && failTimer != null) {
                        failTimer!!.cancel()
                        failedTask!!.cancel()
                    }
                    if (mBluetoothGatt != null) {
                        mBluetoothGatt!!.discoverServices()
                    }
                } catch (e: Exception) {
                    e.printStackTrace()
                }
            } else if (newState == BluetoothProfile.STATE_DISCONNECTED) {
                AppLog.e(TAG, "onConnectionStateChange: STATE_DISCONNECTED")
                if (mBluetoothGatt != null) {
                    refreshDeviceCache(mBluetoothGatt!!)
                    mBluetoothGatt!!.close()
                    mBluetoothGatt = null
                }
                if (failedTask != null && failTimer != null) {
                    failTimer!!.cancel()
                    failedTask!!.cancel()
                }
                if (descriptorWriteQueue != null && descriptorWriteQueue.size > 0) descriptorWriteQueue.clear()
                if (!disconnectedFromDevice) listener!!.onDisconnected(bluetoothDevice!!.address) else disconnectedFromDevice =
                    false
            }
        }

        override fun onServicesDiscovered(gatt: BluetoothGatt, status: Int) {
            super.onServicesDiscovered(gatt, status)
            discoverServices(gatt.services)
            if (listener != null) {
                listener!!.onServiceDiscovered(bluetoothDevice!!.address)
            }
        }

        override fun onCharacteristicRead(
            gatt: BluetoothGatt,
            characteristic: BluetoothGattCharacteristic,
            status: Int
        ) {
            super.onCharacteristicRead(gatt, characteristic, status)
            AppLog.d(
                TAG,
                "onCharacteristicRead : " + characteristic.uuid.toString() + " , data : " + BLEUtils.byteToHex(
                    characteristic.value
                )
            )
            if (listener != null) {
                listener!!.onCharacteristicRead(bluetoothDevice!!.address, gatt, characteristic)
            }
        }

        override fun onCharacteristicWrite(
            gatt: BluetoothGatt,
            characteristic: BluetoothGattCharacteristic,
            status: Int
        ) {
            super.onCharacteristicWrite(gatt, characteristic, status)
            if (listener != null) {
                listener!!.onCharacteristicWrite(gatt, characteristic)
            }
            AppLog.d(
                TAG,
                "onCharacteristicWrite : " + characteristic.uuid.toString() + " , data : " + BLEUtils.byteToHex(
                    characteristic.value
                )
            )
        }

        override fun onCharacteristicChanged(
            gatt: BluetoothGatt,
            characteristic: BluetoothGattCharacteristic
        ) {
            super.onCharacteristicChanged(gatt, characteristic)
            AppLog.d(
                TAG,
                "onCharacteristicChanged : " + characteristic.uuid.toString() + " , data : " + BLEUtils.byteToHex(
                    characteristic.value
                )
            )
            if (listener != null) {
                listener!!.onCharacteristicChanged(bluetoothDevice!!.address, gatt, characteristic)
            }
        }

        override fun onDescriptorRead(
            gatt: BluetoothGatt,
            descriptor: BluetoothGattDescriptor,
            status: Int
        ) {
            super.onDescriptorRead(gatt, descriptor, status)
        }

        override fun onDescriptorWrite(
            gatt: BluetoothGatt,
            descriptor: BluetoothGattDescriptor,
            status: Int
        ) {
            super.onDescriptorWrite(gatt, descriptor, status)
            if (descriptorWriteQueue != null && descriptorWriteQueue.size > 0) {
                descriptorWriteQueue.remove()
                if (descriptorWriteQueue.size > 0) writeGattDescriptor(descriptorWriteQueue.element()) else {
                    if (listener != null) {
                        listener!!.onDescriptorWrite(bleDevice!!)
                    }
                }
            }
        }
    }

    // Discover the services of Connected BLE device.
    private fun discoverServices(services: List<BluetoothGattService>?) {
        val serviceList = services as ArrayList<BluetoothGattService>?
        if (services != null && serviceList!!.size > 0) {
            var isQaulDevice = false
            for (gattService in serviceList) {
                AppLog.e("SERVICE_UUID", gattService.uuid.toString())
                if (gattService.uuid.toString().lowercase() == BleService.SERVICE_UUID.lowercase()) {
                    isQaulDevice = true
                    listener?.addToIgnoreList(this.bleDevice!!)
                    AppLog.d(TAG, "service : " + gattService.uuid.toString())
                    val characteristics =
                        gattService.characteristics as ArrayList<BluetoothGattCharacteristic>
                    if (characteristics != null && characteristics.size > 0) {
                        for (i in characteristics.indices) {
                            val characteristic = characteristics[i]
                            if (characteristic != null && (isCharacteristicNotifiable(characteristic) || isCharacteristicIndicate(
                                    characteristic
                                ))
                            ) {
                                AppLog.d(TAG, "characteristic : " + characteristic.uuid.toString())
                                mBluetoothGatt!!.setCharacteristicNotification(characteristic, true)
                                val gattDescriptor =
                                    characteristic.descriptors as ArrayList<BluetoothGattDescriptor>
                                descriptorWriteQueue!!.addAll(gattDescriptor)
                            }
                        }
                    }
                }
            }
            if (!isQaulDevice) {
                listener?.addToBlackList(this.bleDevice!!)
                disConnectedDevice()
                return
            }
        }
        if (descriptorWriteQueue!!.size > 0) {
            writeGattDescriptor(descriptorWriteQueue.element())
        } else {
            if (listener != null) {
                listener!!.onDescriptorWrite(this.bleDevice!!)
            }
        }
    }

    private fun writeGattDescriptor(d: BluetoothGattDescriptor) {
        if (isCharacteristicNotifiable(d.characteristic)) {
            d.value = BluetoothGattDescriptor.ENABLE_NOTIFICATION_VALUE
        } else {
            d.value = BluetoothGattDescriptor.ENABLE_INDICATION_VALUE
        }
        mBluetoothGatt!!.writeDescriptor(d)
    }

    // Check characteristic notifiable or not
    private fun isCharacteristicNotifiable(pChar: BluetoothGattCharacteristic): Boolean {
        return pChar.properties and BluetoothGattCharacteristic.PROPERTY_NOTIFY != 0
    }

    private fun isCharacteristicIndicate(pChar: BluetoothGattCharacteristic): Boolean {
        return pChar.properties and BluetoothGattCharacteristic.PROPERTY_INDICATE != 0
    }

    private fun broadcastUpdate(intentAction: String) {
        val i = Intent()
        i.action = intentAction
        LocalBroadcastManager.getInstance(mContext).sendBroadcast(i)
    }

    private fun broadcastUpdate(intentAction: String, key: String, data: String) {
        val i = Intent()
        i.action = intentAction
        LocalBroadcastManager.getInstance(mContext).sendBroadcast(i)
    }

    //Device connection timeout call back
    internal inner class ConnectionFailedTask : TimerTask() {
        override fun run() {
            failTimer!!.cancel()
            failedTask!!.cancel()
            if (listener != null) {
                listener!!.onConnectionFailed(bluetoothDevice!!.address)
            }
        }
    }

    //Refresh device bluetooth gatt cache
    private fun refreshDeviceCache(gatt: BluetoothGatt) {
        try {
            val localMethod =
                gatt.javaClass.getMethod("refresh", *arrayOfNulls(0))
            localMethod.invoke(gatt, *arrayOfNulls(0))
        } catch (localException: Exception) {
        }
    }

    // User read data from sensor
    fun readServiceData(serUUID: String, charUUID: String) {
        AppLog.d(TAG, "readServiceData : serUUID : $serUUID, charUUID:$charUUID")
        if (mBluetoothGatt != null) {
            val service = mBluetoothGatt!!.getService(UUID.fromString(serUUID))
            if (service != null) {
                val characteristic = service.getCharacteristic(UUID.fromString(charUUID))
                if (characteristic != null) {
                    mBluetoothGatt!!.readCharacteristic(characteristic)
                }
            }
        }
    }

    // User read data from sensor
    fun writeServiceData(serUUID: String, charUUID: String, data: ByteArray?): Boolean {
        if (data != null) {
            AppLog.d(
                TAG,
                "writeServiceData : serUUID : $serUUID, charUUID:$charUUID, data :" + BLEUtils.byteToHex(
                    data
                )
            )
            if (mBluetoothGatt != null) {
                val service = mBluetoothGatt!!.getService(UUID.fromString(serUUID))
                if (service != null) {
                    val characteristic = service.getCharacteristic(UUID.fromString(charUUID))
                    if (characteristic != null) {
                        characteristic.value = data
                        return mBluetoothGatt!!.writeCharacteristic(characteristic)
                    }
                }
            } else {
                try {
                    mBluetoothGatt = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.M) {
                        bluetoothDevice!!.connectGatt(
                            mContext,
                            false,
                            mGattCallback,
                            BluetoothDevice.TRANSPORT_LE
                        )
                    } else {
                        bluetoothDevice!!.connectGatt(mContext, false, mGattCallback)
                    }
                    writeServiceData(serUUID, charUUID, data)
                } catch (e: Exception) {
                    e.printStackTrace()
                }
            }
        }
        return false
    }

    interface BleConnectionListener {
        fun onConnected(macAddress: String?)
        fun onDisconnected(macAddress: String?)
        fun onServiceDiscovered(macAddress: String?)
        fun onDescriptorWrite(bleScanDevice: BLEScanDevice)
        fun onConnectionFailed(macAddress: String?)
        fun onCharacteristicRead(
            macAddress: String?,
            gatt: BluetoothGatt?,
            characteristic: BluetoothGattCharacteristic?
        )
        fun onCharacteristicWrite(
            gatt: BluetoothGatt?,
            characteristic: BluetoothGattCharacteristic?
        )
        fun onCharacteristicChanged(
            macAddress: String?,
            gatt: BluetoothGatt?,
            characteristic: BluetoothGattCharacteristic?
        )
        fun addToBlackList(bleScanDevice: BLEScanDevice)
        fun addToIgnoreList(bleScanDevice: BLEScanDevice)
    }

    companion object {
        private val TAG = BaseBleActor::class.java.simpleName
    }
}