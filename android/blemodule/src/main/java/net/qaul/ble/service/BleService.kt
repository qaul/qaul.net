// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

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
import androidx.lifecycle.LifecycleService
import net.qaul.ble.AppLog
import java.util.*

class BleService : LifecycleService() {
    private val TAG: String = BleService::class.java.simpleName
    private var bluetoothAdapter: BluetoothAdapter? = null
    private var bleResponseCallback: BleResponseCallback? = null
    private val SERVICE_UUID = "99E91399-80ED-4943-9BCB-39C532A76023"
    private val READ_CHAR = "99E91401-80ED-4943-9BCB-39C532A76023"
    private var qaulId = ""
    private var advertMode = ""
    private var bluetoothLeAdvertiser: BluetoothLeAdvertiser? = null
    private var gattServer: BluetoothGattServer? = null
    private var bluetoothManager: BluetoothManager? = null

    companion object {
        var bleService: BleService? = null
    }

    override fun onCreate() {
        super.onCreate()
        bleService = this
        AppLog.e(TAG, "$TAG created")
    }

    override fun onStart(intent: Intent?, startId: Int) {
        super.onStart(intent, startId)
        AppLog.e(TAG, "$TAG started")
    }

    /**
     * This Method Will Set Necessary Data for Staring Advertisement
     */
    fun setData(
        qaul_id: String,
        mode: String, bleCallback: BleResponseCallback
    ) {
        bleService?.qaulId = qaul_id
        bleService?.advertMode = mode
        bleService?.bleResponseCallback = bleCallback
        setupAdvertiser()
    }

    /**
     * This Method Will Start the Service
     */
    fun start(
        context: Context
    ) {
        if (bleService == null) {
            val intent = Intent(context, BleService::class.java)
            context.startService(intent)
        } else {
            AppLog.e(TAG, "$TAG already started")
        }
    }

    /**
     * This Method Will Return True if Service is Running
     */
    fun isRunning(): Boolean {
        return bleService != null
    }

    /**
     * This Method Will Stop the Service if It Is Running
     */
    fun stop() {
        if (bleService != null) {
            bleService?.stopSelf()
        } else {
            AppLog.e(TAG, "$TAG not started")
        }
    }

    /**
     * This Method Will Set Service, Characteristic & Other Data to Run Advertiser
     */
    private fun setupAdvertiser() {
        val t = Thread {
            bluetoothManager = bleService!!.getSystemService(BLUETOOTH_SERVICE) as BluetoothManager
            bluetoothAdapter = bluetoothManager!!.adapter
            bluetoothAdapter!!.name = "Qaul"
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

    /**
     * This Method Will Start Advertisement According to Configuration
     */
    private fun startAdvertisement(service: BluetoothGattService) {
        startGattServer(service = service)
        val dataBuilder = AdvertiseData.Builder()
        val settingsBuilder = AdvertiseSettings.Builder()
        dataBuilder.setIncludeTxPowerLevel(true)
        val uuid = ParcelUuid(UUID.fromString(SERVICE_UUID))
        dataBuilder.addServiceUuid(uuid)
        dataBuilder.setIncludeDeviceName(true)
        when (advertMode) {
            "low_power" -> {
                settingsBuilder
                    .setAdvertiseMode(AdvertiseSettings.ADVERTISE_MODE_LOW_POWER)
            }
            "balanced" -> {
                settingsBuilder
                    .setAdvertiseMode(AdvertiseSettings.ADVERTISE_MODE_BALANCED)
            }
            "low_latency" -> {
                settingsBuilder
                    .setAdvertiseMode(AdvertiseSettings.ADVERTISE_MODE_LOW_LATENCY)
            }
            "UNRECOGNIZED" -> {
                settingsBuilder
                    .setAdvertiseMode(AdvertiseSettings.ADVERTISE_MODE_LOW_LATENCY)
            }
        }
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

            var counter = 20;
            var remaincounter = 0;
            var qaulId = "rakesh Jiyani welcome to india okay"
            var sendSubstring = "";
            override fun onCharacteristicReadRequest(
                device: BluetoothDevice,
                requestId: Int, offset: Int,
                characteristic: BluetoothGattCharacteristic
            ) {
                super.onCharacteristicReadRequest(
                    device, requestId, offset,
                    characteristic
                )
                //TODO multiple request received
                AppLog.e(TAG, "Request Received")
                if (remaincounter > 0) {
                    return
                }
                sendSubstring = sendSubstring.substring(0, counter);
                remaincounter = sendSubstring.length;
                gattServer?.sendResponse(
                    device,
                    requestId,
                    0,
                    remaincounter,
                    sendSubstring.toByteArray(Charsets.UTF_8)
                );
                counter += 15;


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
            bleService?.bleResponseCallback?.bleAdvertResponse(
                status = true,
                errorText = successMsg,
                unknownError = false
            )
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
            bleService?.bleResponseCallback?.bleAdvertResponse(
                status = false,
                errorText = failMsg,
                unknownError = unknownError
            )
        }
    }

    interface BleResponseCallback {
        fun bleAdvertResponse(status: Boolean, errorText: String, unknownError: Boolean)
    }
}