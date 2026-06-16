package net.qaul.ble.test.ble.advertiser

import android.annotation.SuppressLint
import android.bluetooth.BluetoothDevice
import android.bluetooth.BluetoothManager
import android.bluetooth.le.AdvertiseCallback
import android.bluetooth.le.AdvertiseData
import android.bluetooth.le.AdvertiseSettings
import android.bluetooth.le.AdvertisingSetParameters
import android.bluetooth.le.BluetoothLeAdvertiser
import android.content.Context
import android.os.ParcelUuid
import android.util.Log
import net.qaul.ble.test.ble.BleConstants

@SuppressLint("MissingPermission")
object BleAdvertiser {

    private const val TAG = "BleAdvertiser"

    private var advertiser: BluetoothLeAdvertiser? = null
    var isAdvertising = false
        private set

    private val advertiseSettings = AdvertiseSettings.Builder()
        .setAdvertiseMode(AdvertiseSettings.ADVERTISE_MODE_LOW_LATENCY) // TODO: Change advertising and scanning mode to be adabtable to preserve battery, currently its all on the strongest / highest battery usage modes
        .setConnectable(true)
        .setTimeout(0) // advertise indefinitely
        .setTxPowerLevel(AdvertiseSettings.ADVERTISE_TX_POWER_HIGH)
        .build()

    private val advertiseData = AdvertiseData.Builder()
        .setIncludeDeviceName(false) // name bloats the packet; UUID is enough for filtering
        .addServiceUuid(ParcelUuid(BleConstants.SERVICE_UUID))
        // Include a truncated part of the qaul id, as advertisements can only fit 31 bytes.
        // This is only a hint,the full id is verified post-connection.
        // TODO: Review how best to truncate the qaul id, best to use the end of it, think theres a qaul function which reutrns the short version of the id which we can truncate
        .addManufacturerData(BleConstants.QAUL_MANUFACTURER_ID, BleConstants.LOCAL_QAUL_ID.copyOf(BleConstants.QAUL_ID_ADVERT_BYTES))
        .build()

    private val advertiseSetParameters = AdvertisingSetParameters.Builder()
        .setLegacyMode(false)
        .setInterval(AdvertisingSetParameters.INTERVAL_LOW)
        .setTxPowerLevel(AdvertisingSetParameters.TX_POWER_MAX)
        .setPrimaryPhy(BluetoothDevice.PHY_LE_CODED)
        .setSecondaryPhy(BluetoothDevice.PHY_LE_CODED)
        .setScannable(false)
        .setConnectable(true)
        .build()


    fun start(context: Context) {
        if (isAdvertising) {
            Log.w(TAG, "Already advertising")
            return
        }
        val bluetoothManager = context.getSystemService(Context.BLUETOOTH_SERVICE) as BluetoothManager
        advertiser = bluetoothManager.adapter.bluetoothLeAdvertiser
        if (advertiser == null) {
            Log.e(TAG, "Device does not support BLE advertising")
            return
        }
        advertiser?.startAdvertising(advertiseSettings, advertiseData, advertiseCallback)
        //advertiser?.startAdvertisingSet(advertiseSetParameters, advertiseData, null, null, ) TODO: Figure out what ways we can use this
    }

    fun stop() {
        advertiser?.stopAdvertising(advertiseCallback)
        isAdvertising = false
        Log.i(TAG, "Advertising stopped")
    }

    private val advertiseCallback = object : AdvertiseCallback() {
        override fun onStartSuccess(settingsInEffect: AdvertiseSettings) {
            isAdvertising = true
            Log.i(TAG, "Advertising started with SERVICE_UUID ${BleConstants.SERVICE_UUID}")
        }

        override fun onStartFailure(errorCode: Int) {
            isAdvertising = false
            val reason = when (errorCode) {
                ADVERTISE_FAILED_DATA_TOO_LARGE -> "data too large"
                ADVERTISE_FAILED_TOO_MANY_ADVERTISERS -> "too many advertisers"
                ADVERTISE_FAILED_ALREADY_STARTED -> "already started"
                ADVERTISE_FAILED_INTERNAL_ERROR -> "internal error"
                ADVERTISE_FAILED_FEATURE_UNSUPPORTED -> "feature unsupported"
                else -> "unknown ($errorCode)"
            }
            Log.e(TAG, "Advertising failed: $reason")
        }
    }
}
