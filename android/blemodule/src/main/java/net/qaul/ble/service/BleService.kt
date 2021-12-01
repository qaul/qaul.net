package net.qaul.ble.service

import android.bluetooth.*
import android.bluetooth.le.AdvertiseCallback
import android.bluetooth.le.AdvertiseData
import android.bluetooth.le.AdvertiseSettings
import android.bluetooth.le.BluetoothLeAdvertiser
import android.content.Context
import android.content.Intent
import android.os.Build
import android.os.ParcelUuid
import android.util.Log
import androidx.lifecycle.LifecycleService
import net.qaul.ble.AppLog
import net.qaul.ble.callback.BleRequestCallback
import qaul.sys.ble.BleOuterClass
import java.util.*

class BleService() : LifecycleService() {
    private val TAG: String = BleService::class.java.simpleName
    private var bleService: BleService? = null
    private var bluetoothAdapter: BluetoothAdapter? = null
    private val SERVICE_UUID = "6E400001-B5A3-F393-E0A9-E50E24DCCA9E"
    private val READ_CHAR = "6E400003-B5A3-F393-E0A9-E50E24DCCA9E"
    private var qaulId = ""
    private var bluetoothLeAdvertiser: BluetoothLeAdvertiser? = null
    private var gattServer: BluetoothGattServer? = null
    private var bluetoothManager: BluetoothManager? = null
    private var bleCallback: BleRequestCallback? = null

    override fun onCreate() {
        super.onCreate()
        AppLog.e(TAG, "$TAG created")
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        bleService = this
        setupAdvertiser()
        return super.onStartCommand(intent, flags, startId)
    }


    override fun onStart(intent: Intent?, startId: Int) {
        super.onStart(intent, startId)
        AppLog.e(TAG, "$TAG started")
    }

    fun setData(qaul_id: String, bleRequestCallback: BleRequestCallback?) {
        qaulId = qaul_id
        bleCallback = bleRequestCallback
    }

    fun start(context: Context) {
        if (bleService == null) {
            val intent = Intent(context, BleService::class.java)
            context.startService(intent)
        } else {
            AppLog.e(TAG, "$TAG already started")
        }
    }

    fun isRunning(): Boolean {
        return bleService != null
    }

    fun stop() {
        if (bleService != null) {
            bleService?.stopSelf()
        } else {
            AppLog.e(TAG, "$TAG not started")
        }
    }

    fun setupAdvertiser() {
        val t = Thread {
            bluetoothAdapter = BluetoothAdapter.getDefaultAdapter()
            bluetoothAdapter!!.name = "NewH"
            bluetoothManager = this.getSystemService(BLUETOOTH_SERVICE) as BluetoothManager
            bluetoothLeAdvertiser = bluetoothAdapter!!.bluetoothLeAdvertiser
            if (Build.VERSION.SDK_INT > 21) {
                if (bluetoothAdapter != null) {
                    AppLog.e(
                        TAG,
                        "Peripheral supported"
                    )
                    val firstService = BluetoothGattService(
                        UUID.fromString(SERVICE_UUID),
                        BluetoothGattService.SERVICE_TYPE_PRIMARY
                    )
                    val firstServiceChar = BluetoothGattCharacteristic(
                        UUID.fromString(READ_CHAR),
                        BluetoothGattCharacteristic.PROPERTY_READ,
                        BluetoothGattCharacteristic.PERMISSION_READ
                    )

                    firstServiceChar.setValue(qaulId)
                    firstService.addCharacteristic(firstServiceChar)
                    startAdvertisement(service = firstService)
                } else {
                    AppLog.e(
                        TAG,
                        "Peripheral not supported"
                    )
                }
            } else {
                AppLog.e(
                    TAG,
                    "Peripheral not supported"
                )
            }
        }
        t.start()
    }

    private fun startAdvertisement(service: BluetoothGattService) {
        startGattServer(service = service)
        val dataBuilder = AdvertiseData.Builder()
        val settingsBuilder = AdvertiseSettings.Builder()

        dataBuilder.setIncludeTxPowerLevel(true)

        val uuid = ParcelUuid(UUID.fromString(SERVICE_UUID))
        dataBuilder.addServiceUuid(uuid)
        settingsBuilder
            .setAdvertiseMode(AdvertiseSettings.ADVERTISE_MODE_BALANCED)
        settingsBuilder
            .setTxPowerLevel(AdvertiseSettings.ADVERTISE_TX_POWER_HIGH)
        settingsBuilder.setConnectable(true)

        bluetoothLeAdvertiser!!.startAdvertising(
            settingsBuilder.build(),
            dataBuilder.build(), advertiseCallback
        )
    }

    private fun startGattServer(service: BluetoothGattService) {
        gattServer = bluetoothManager!!.openGattServer(
            this,
            gattServerCallback
        )
        gattServer?.addService(service)
    }

    private var gattServerCallback: BluetoothGattServerCallback =
        object : BluetoothGattServerCallback() {
            override fun onConnectionStateChange(
                device: BluetoothDevice, status: Int,
                newState: Int
            ) {
                super.onConnectionStateChange(device, status, newState)
            }

            override fun onServiceAdded(status: Int, service: BluetoothGattService) {
                super.onServiceAdded(status, service)
            }

            override fun onCharacteristicReadRequest(
                device: BluetoothDevice,
                requestId: Int, offset: Int,
                characteristic: BluetoothGattCharacteristic
            ) {
                super.onCharacteristicReadRequest(
                    device, requestId, offset,
                    characteristic
                )
                AppLog.e(TAG,  "Request Received")
            }

            private fun getStoredValue(characteristic: BluetoothGattCharacteristic): ByteArray {
                val `val` = ByteArray(characteristic.value.size)
                System.arraycopy(
                    characteristic.value, 0, `val`, 0,
                    characteristic.value.size
                )
                return `val`
            }

            override fun onCharacteristicWriteRequest(
                device: BluetoothDevice,
                requestId: Int, characteristic: BluetoothGattCharacteristic,
                preparedWrite: Boolean, responseNeeded: Boolean, offset: Int,
                value: ByteArray
            ) {
                super.onCharacteristicWriteRequest(
                    device, requestId,
                    characteristic, preparedWrite, responseNeeded, offset,
                    value
                )
            }

            override fun onDescriptorReadRequest(
                device: BluetoothDevice,
                requestId: Int,
                offset: Int,
                descriptor: BluetoothGattDescriptor
            ) {
                super.onDescriptorReadRequest(device, requestId, offset, descriptor)
            }

            override fun onDescriptorWriteRequest(
                device: BluetoothDevice,
                requestId: Int,
                descriptor: BluetoothGattDescriptor,
                preparedWrite: Boolean,
                responseNeeded: Boolean,
                offset: Int,
                value: ByteArray
            ) {
                super.onDescriptorWriteRequest(
                    device,
                    requestId,
                    descriptor,
                    preparedWrite,
                    responseNeeded,
                    offset,
                    value
                )
            }

            override fun onExecuteWrite(device: BluetoothDevice, requestId: Int, execute: Boolean) {
                super.onExecuteWrite(device, requestId, execute)
            }

            override fun onNotificationSent(device: BluetoothDevice, status: Int) {
                super.onNotificationSent(device, status)
            }
        }

    private val advertiseCallback: AdvertiseCallback = object : AdvertiseCallback() {
        override fun onStartSuccess(advertiseSettings: AdvertiseSettings) {
            val successMsg = "Advertisement successful"
            AppLog.e(TAG, successMsg)
            val bleRes = BleOuterClass.Ble.newBuilder()
            val startResult = BleOuterClass.BleStartResult.newBuilder()
            startResult.success = true
            startResult.noRights = false
            startResult.errorMessage = ""
            startResult.unknonwError = false
            bleRes.startResult = startResult.build()
            bleCallback?.bleResponse(ble = bleRes.build())
        }

        override fun onStartFailure(i: Int) {
            var unknownError = false
            var errorText = ""
            if (i < 1 || i > 5) {
                unknownError = true
            }
            when (i) {
                1 -> {
                    errorText = "ADVERTISE_FAILED_DATA_TOO_LARGE"
                }
                2 -> {
                    errorText = "ADVERTISE_FAILED_TOO_MANY_ADVERTISERS"
                }
                3 -> {
                    errorText = "ADVERTISE_FAILED_ALREADY_STARTED"
                }
                4 -> {
                    errorText = "ADVERTISE_FAILED_INTERNAL_ERROR"
                }
                5 -> {
                    errorText = "ADVERTISE_FAILED_FEATURE_UNSUPPORTED"
                }
            }

            val failMsg = "Advertisement failed: $errorText"
            AppLog.e(TAG, failMsg)
            val bleRes = BleOuterClass.Ble.newBuilder()
            val startResult = BleOuterClass.BleStartResult.newBuilder()
            startResult.success = false
            startResult.noRights = false
            startResult.errorMessage = failMsg
            startResult.unknonwError = unknownError
            bleRes.startResult = startResult.build()
            bleCallback?.bleResponse(ble = bleRes.build())
        }
    }
}