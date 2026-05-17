// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

/**
* BleActor is an instance of GATT Client which is used to connect to other GATT Servers.
*/
package net.qaul.ble.core

import android.annotation.SuppressLint
import android.bluetooth.*
import android.content.Context
import android.os.Build
import android.os.Handler
import android.os.Looper
import net.qaul.ble.AppLog
import net.qaul.ble.BLEUtils
import net.qaul.ble.model.BLEScanDevice
import net.qaul.ble.service.BleService
import java.util.*
import java.util.concurrent.ConcurrentLinkedDeque

@SuppressLint("MissingPermission")
class BleActor(private val mContext: Context, var listener: BleConnectionListener?) {
    private var mBluetoothGatt: BluetoothGatt? = null
    private val descriptorWriteQueue: Queue<BluetoothGattDescriptor> = LinkedList()
    private var failTimer: Timer? = null
    private var failedTask: ConnectionFailedTask? = null
    var disconnectedFromDevice = false
    var bluetoothDevice: BluetoothDevice? = null
    var bleDevice: BLEScanDevice? = null
    var messageId: String = ""
    var isFromMessage = false
    var isReconnect = false
    var tempData = ByteArray(0)
    var attempt = 0

    private var isWriting = false
    var sendChunkQueue: Queue<ByteArray> = ConcurrentLinkedDeque<ByteArray>()


    /**
     * Disconnect current device.
     */
    fun disConnectedDevice() {
        if (mBluetoothGatt != null && BleService.bleService != null) {
            disconnectedFromDevice = true
            refreshDeviceCache(mBluetoothGatt!!)
            if (mBluetoothGatt != null) {
                mBluetoothGatt?.disconnect()
                Handler(Looper.getMainLooper()).postDelayed({
                    if (mBluetoothGatt != null) {
                        mBluetoothGatt?.close()
                        mBluetoothGatt = null
                    }
                }, 200)
            }
        }
    }

    /**
     * Set device in Actor
     */
    fun setDevice(device: BLEScanDevice?, isFromMessage: Boolean) {
        this.isFromMessage = isFromMessage
        bleDevice = device
        bluetoothDevice = device!!.bluetoothDevice

        Handler(Looper.getMainLooper()).postDelayed({
            connectDevice()
        }, 500)
    }

    /**
     * Use to make connection to device
     */
    private fun connectDevice(): Boolean {
        if (mBluetoothGatt != null) {
            AppLog.e(TAG, "Already connected to $bluetoothDevice")
            return true
        }

        AppLog.i(TAG, "connectDevice : $bluetoothDevice")
        if (bluetoothDevice == null) {
            AppLog.e(TAG, "connectDevice : $bluetoothDevice")
            listener!!.onConnectionFailed(bleScanDevice = bleDevice!!)
            return false
        }
        
        cancelTimer()
        failTimer = Timer()
        failedTask = ConnectionFailedTask()
        failTimer!!.schedule(failedTask, 20000)
        try {
            mBluetoothGatt = bluetoothDevice!!.connectGatt(
                mContext, false, mGattCallback, BluetoothDevice.TRANSPORT_LE
            )
        } catch (e: Exception) {
            e.printStackTrace()
            return false
        }
        return true
    }

    /**
     * Object of a bluetoothGattCallback
     */
    private val mGattCallback: BluetoothGattCallback = object : BluetoothGattCallback() {
        override fun onConnectionStateChange(gatt: BluetoothGatt, status: Int, newState: Int) {
            super.onConnectionStateChange(gatt, status, newState)
            if (newState == BluetoothProfile.STATE_CONNECTING) {
                AppLog.e(TAG, "onConnectionStateChange: STATE_CONNECTING")
            }
            if (newState == BluetoothProfile.STATE_CONNECTED) {
                AppLog.e(TAG, "onConnectionStateChange: STATE_CONNECTED")
                listener!!.onConnected(bluetoothDevice!!.address, bluetoothDevice)
                try {
                    cancelTimer()
                    if (mBluetoothGatt != null) {
                        mBluetoothGatt!!.discoverServices()
                    }

                } catch (e: Exception) {
                    e.printStackTrace()
                }
            } else if (newState == BluetoothProfile.STATE_DISCONNECTED) {
                AppLog.e(TAG, "onConnectionStateChange: STATE_DISCONNECTED")
                closeGatt()
                cancelTimer()
                if (descriptorWriteQueue != null && descriptorWriteQueue.size > 0) descriptorWriteQueue.clear()
                if (!disconnectedFromDevice) listener!!.onDisconnected(bleDevice!!) else disconnectedFromDevice =
                    false
                if (isFromMessage) {
                    if (mBluetoothGatt != null) {
                        BleService.bleService!!.bleCallback?.onMessageSent(
                            id = messageId, success = false, data = tempData
                        )
                    }
                }
            }
        }

        override fun onServicesDiscovered(gatt: BluetoothGatt, status: Int) {
            super.onServicesDiscovered(gatt, status)
            discoverServices(gatt.services)
        }

        override fun onCharacteristicRead(
            gatt: BluetoothGatt, characteristic: BluetoothGattCharacteristic, status: Int
        ) {
            super.onCharacteristicRead(gatt, characteristic, status)
            isOperationInProgress = false
            processNextOperation()

            var data = characteristic.value
            AppLog.e(
                TAG,
                "onCharacteristicRead : " + characteristic.uuid.toString() + " , value ->  $data"
            )

            // don't do anything if $data is null
            // TODO: does this device need to be removed?
            if(data != null) {
                if (isFromMessage) {
                    send(BLEUtils.byteToHex(tempData))
                    return
                }

                if (listener != null) {
                    listener!!.onCharacteristicRead(bleDevice!!, gatt, characteristic)
                }


                if (characteristic.uuid.toString()
                        .lowercase() == BleService.READ_CHAR.lowercase() && !isFromMessage
                ) {
    //                disConnectedDevice()
                }
            }
        }

        /**
        * This method keeps on sending the data to the device until the queue is empty
        */
        override fun onCharacteristicWrite(
            gatt: BluetoothGatt, characteristic: BluetoothGattCharacteristic, status: Int
        ) {
            super.onCharacteristicWrite(gatt, characteristic, status)
            isOperationInProgress = false
            processNextOperation()

            if (listener != null) {
                if (messageId.isEmpty() || messageId.isBlank()) {
                    listener!!.onCharacteristicWrite(gatt = gatt, characteristic = characteristic)
                } else {
                    isWriting = false
                    if (!_send()) {
                        listener!!.onMessageSent(
                            gatt = gatt, value = tempData, id = messageId
                        )
                        messageId = ""
                        tempData = ByteArray(0)
                    }
                }
            }
        }

        override fun onCharacteristicChanged(
            gatt: BluetoothGatt, characteristic: BluetoothGattCharacteristic
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

        // Descriptor method (Unused for this architecture) 
        override fun onDescriptorRead(
            gatt: BluetoothGatt, descriptor: BluetoothGattDescriptor, status: Int
        ) {
            super.onDescriptorRead(gatt, descriptor, status)
        }

        // Descriptor method (Unused for this architecture)
        override fun onDescriptorWrite(
            gatt: BluetoothGatt, descriptor: BluetoothGattDescriptor, status: Int
        ) {
            super.onDescriptorWrite(gatt, descriptor, status)
            if (descriptorWriteQueue != null && descriptorWriteQueue.size > 0) {
                descriptorWriteQueue.remove()
                if (descriptorWriteQueue.size > 0) writeGattDescriptor(descriptorWriteQueue.element()) 
                else {
                    if (listener != null) {
                        listener!!.onDescriptorWrite(bleDevice!!, this@BleActor)
                    }
                }
            }
        }

        override fun onMtuChanged(gatt: BluetoothGatt?, mtu: Int, status: Int) {
            super.onMtuChanged(gatt, mtu, status)
            AppLog.e("MTU Size: ", "" + mtu)
        }
    }

    private fun closeGatt() {
        if (mBluetoothGatt != null) {
            refreshDeviceCache(mBluetoothGatt!!)
            mBluetoothGatt!!.close()
            mBluetoothGatt = null
        }
    }

    private fun cancelTimer() {
        if (failedTask != null && failTimer != null) {
            failTimer!!.cancel()
            failedTask!!.cancel()
        }
    }

    /**
    * This method sends data to the GATT Server and manages the bytes written.
    * The android gatt client can write only 20 or less bytes at a time. 
    * On the server side, it will concatenate the received bytes and check for the delimiters to form the complete message.
    */
    fun send(data: String): Int {
        //AppLog.e(TAG, "send data-----------------> data $data")
        if(sendChunkQueue.size > 0) {
            if (!isWriting) _send()
        }
        return 0
    }

    private fun _send(): Boolean {
        if (sendChunkQueue.isEmpty()) {
            AppLog.e(TAG, "_send(): EMPTY QUEUE")
            return false
        }
        val tx = sendChunkQueue.poll()
        //AppLog.e(TAG, "_send(): tx: ${BLEUtils.toBinaryString(tx)}")
        isWriting = true // Set the write in progress flag
        if (!writeServiceData(BleService.SERVICE_UUID, BleService.MSG_CHAR, tx)) {
            return false;
        }
        return true
    }

    /**
     * Discover the services of Connected BLE device.
     */
    private fun discoverServices(services: List<BluetoothGattService>?) {
        val serviceList = services as ArrayList<BluetoothGattService>?
        if (services != null && serviceList!!.size > 0) {
            var isQaulDevice = false
            for (gattService in serviceList) {
                AppLog.e("SERVICE_UUID", gattService.uuid.toString())
                if (gattService.uuid.toString().lowercase()
                        .trim() == BleService.SERVICE_UUID.lowercase().trim()
                ) {
                    AppLog.e(
                        TAG,
                        "service : " + gattService.uuid.toString() + " " + bleDevice?.macAddress
                    )
                    isQaulDevice = true
                    listener?.addToIgnoreList(this.bleDevice!!)
                    val characteristics =
                        gattService.characteristics as ArrayList<BluetoothGattCharacteristic>
                    if (characteristics != null && characteristics.size > 0) {
                        for (i in characteristics.indices) {
                            val characteristic = characteristics[i]
                            // Below lines of code are useless and are always false for current architecture.
                            // Present to maintain code consistency.
                            if (characteristic != null && (isCharacteristicNotifiable(characteristic) || isCharacteristicIndicate(
                                    characteristic
                                ))
                            ) {
                                AppLog.d(TAG, "Notify or Indicate characteristic : " + characteristic.uuid.toString())
                                mBluetoothGatt!!.setCharacteristicNotification(characteristic, true)
                                val gattDescriptor =
                                    characteristic.descriptors as ArrayList<BluetoothGattDescriptor>
                                descriptorWriteQueue.addAll(gattDescriptor)
                            }
                        }
                    }
                }
            }
            if (!isQaulDevice) {
                disConnectedDevice() //discoverServices
                return
            }

            if (listener != null) {
                listener!!.onServiceDiscovered(bluetoothDevice!!.address)
            }
        }
        // Descriptor write queue always empty for current architecture.
        // It finally calls "readServiceData()" function to read read_char value from other qaul device.
        if (descriptorWriteQueue.size > 0) {
        } 
        else {
            if (listener != null) {
                mBluetoothGatt
                listener!!.onDescriptorWrite(this.bleDevice!!, this)
            }
        }
    }

    /**
     * This method is used to write descriptor of gatt
     */
    private fun writeGattDescriptor(d: BluetoothGattDescriptor) {
        if (isCharacteristicNotifiable(d.characteristic)) {
            d.value = BluetoothGattDescriptor.ENABLE_NOTIFICATION_VALUE
        } else {
            d.value = BluetoothGattDescriptor.ENABLE_INDICATION_VALUE
        }
        mBluetoothGatt!!.writeDescriptor(d)
    }

    /**
     * Check characteristic notifiable or not
     */
    private fun isCharacteristicNotifiable(pChar: BluetoothGattCharacteristic): Boolean {
        return pChar.properties and BluetoothGattCharacteristic.PROPERTY_NOTIFY != 0
    }

    /**
     * Check characteristic can indicate or not
     */
    private fun isCharacteristicIndicate(pChar: BluetoothGattCharacteristic): Boolean {
        return pChar.properties and BluetoothGattCharacteristic.PROPERTY_INDICATE != 0
    }


    /**
     * Device connection timeout call back
     */
    internal inner class ConnectionFailedTask : TimerTask() {
        override fun run() {
            if (listener != null) {
                listener!!.onConnectionFailed(bleDevice!!)
                AppLog.e(TAG, "ConnectionFailedTask : $bluetoothDevice")
                disConnectedDevice()
                listener!!.onDisconnected(bleDevice!!)
                Handler(Looper.getMainLooper()).postDelayed({
                    if (isFromMessage) {
                        if (mBluetoothGatt != null) {
                            BleService.bleService!!.bleCallback?.onMessageSent(
                                id = messageId, success = false, data = tempData
                            )
                        }
                    }
                }, 1000)
            }
            failTimer!!.cancel()
            failedTask!!.cancel()
        }
    }

    /**
     * Refresh device bluetooth gatt cache
     */
    private fun refreshDeviceCache(gatt: BluetoothGatt) {
        try {
            val localMethod = gatt.javaClass.getMethod("refresh", *arrayOfNulls(0))
            localMethod.invoke(gatt, *arrayOfNulls(0))
        } catch (localException: Exception) {
        }
    }


    /**
     * GATT Operation types for serialization
     */
    private enum class GattOpType { READ, WRITE }
    private data class GattOperation(val type: GattOpType, val characteristic: BluetoothGattCharacteristic, val data: ByteArray? = null)

    private val operationQueue: Queue<GattOperation> = LinkedList()
    private var isOperationInProgress = false

    private fun processNextOperation() {
        if (isOperationInProgress || operationQueue.isEmpty()) return

        val op = operationQueue.poll() ?: return
        isOperationInProgress = true

        val success = when (op.type) {
            GattOpType.READ -> mBluetoothGatt!!.readCharacteristic(op.characteristic)
            GattOpType.WRITE -> {
                if (Build.VERSION.SDK_INT >= 33) {
                    mBluetoothGatt!!.writeCharacteristic(op.characteristic, op.data!!, BluetoothGattCharacteristic.WRITE_TYPE_DEFAULT)
                    true
                } else {
                    op.characteristic.value = op.data
                    mBluetoothGatt!!.writeCharacteristic(op.characteristic)
                }
            }
        }

        if (!success) {
            AppLog.e(TAG, "GATT operation failed to start: ${op.type}")
            isOperationInProgress = false
            processNextOperation()
        }
    }

    /**
     * User read data from device
     */
    fun readServiceData(serUUID: String, charUUID: String) {
        AppLog.d(TAG, "readServiceData : serUUID : $serUUID, charUUID:$charUUID")
        if (mBluetoothGatt != null) {
            val service = mBluetoothGatt!!.getService(UUID.fromString(serUUID))
            if (service != null) {
                val characteristic = service.getCharacteristic(UUID.fromString(charUUID))
                if (characteristic != null) {
                    operationQueue.add(GattOperation(GattOpType.READ, characteristic))
                    processNextOperation()
                }
            }
        }
    }

    /**
     * User write data to device
     */
    fun writeServiceData(
        serUUID: String, charUUID: String, data: ByteArray?
    ): Boolean {
        if (data != null) {
            if (mBluetoothGatt != null) {
                val service = mBluetoothGatt!!.getService(UUID.fromString(serUUID))
                if (service != null) {
                    val characteristic = service.getCharacteristic(UUID.fromString(charUUID))
                    if (characteristic != null) {
                        operationQueue.add(GattOperation(GattOpType.WRITE, characteristic, data))
                        processNextOperation()
                        return true
                    } else {
                        AppLog.e(TAG, "writeServiceData failed: Characteristic $charUUID not found")
                    }
                } else {
                    AppLog.e(TAG, "writeServiceData failed: Service $serUUID not found")
                }
            } else {
                AppLog.e(TAG, "writeServiceData failed: mBluetoothGatt is null")
            }
        }

        BleService.bleService!!.bleCallback?.onMessageSent(
            id = messageId, success = false, data = data ?: ByteArray(0)
        )
        return false
    }

    /**
     * Interface To Send Callback of Connection Status & Read Data Result to service
     */
    interface BleConnectionListener {
        fun onConnected(macAddress: String?, device: BluetoothDevice?)
        fun onDisconnected(bleScanDevice: BLEScanDevice)
        fun onServiceDiscovered(macAddress: String?)
        fun onDescriptorWrite(bleScanDevice: BLEScanDevice, bleActor: BleActor)
        fun onConnectionFailed(bleScanDevice: BLEScanDevice)
        fun onCharacteristicRead(
            bleScanDevice: BLEScanDevice,
            gatt: BluetoothGatt?,
            characteristic: BluetoothGattCharacteristic?
        )

        fun onCharacteristicWrite(
            gatt: BluetoothGatt?, characteristic: BluetoothGattCharacteristic?
        )

        fun onMessageSent(
            gatt: BluetoothGatt?, value: ByteArray, id: String
        )

        fun onCharacteristicChanged(
            macAddress: String?, gatt: BluetoothGatt?, characteristic: BluetoothGattCharacteristic?
        )

        fun addToBlackList(bleScanDevice: BLEScanDevice)
        fun addToIgnoreList(bleScanDevice: BLEScanDevice)
    }

    companion object {
        private val TAG: String = "qaul-blemodule BleActor"
    }
}