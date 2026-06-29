package net.qaul.ble.test.ble.advertiser

import android.annotation.SuppressLint
import android.bluetooth.BluetoothDevice
import android.bluetooth.BluetoothManager
import android.bluetooth.le.AdvertiseCallback
import android.bluetooth.le.AdvertiseData
import android.bluetooth.le.AdvertiseSettings
import android.bluetooth.le.AdvertisingSet
import android.bluetooth.le.AdvertisingSetCallback
import android.bluetooth.le.AdvertisingSetParameters
import android.bluetooth.le.BluetoothLeAdvertiser
import android.content.Context
import android.os.Build
import android.os.ParcelUuid
import android.util.Log
import net.qaul.ble.BleConstants

@SuppressLint("MissingPermission")
object BleAdvertiser {

    private const val TAG = "BleAdvertiser"

    private var advertiser: BluetoothLeAdvertiser? = null
    private var appContext: Context? = null   // cached so the watchdog can restart without a Context
    var isAdvertising = false
        private set

    // True while advertising is suppressed because were at the connection cap,distinct from a full stop()
    @Volatile var pausedForCap = false
        private set

    // Whether the current advertising uses extended advertising.
    @Volatile private var extendedMode = false

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
        .addManufacturerData(BleConstants.QAUL_MANUFACTURER_ID, BleConstants.LOCAL_QAUL_ID.copyOf(BleConstants.QAUL_ID_ADVERT_BYTES))
        .build()

    // Extended advertising parameters
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
        appContext = context.applicationContext
        if (isAdvertising) {
            Log.w(TAG, "Already advertising")
            return
        }
        val adapter = (context.getSystemService(Context.BLUETOOTH_SERVICE) as BluetoothManager).adapter
        advertiser = adapter?.bluetoothLeAdvertiser
        if (advertiser == null) {
            Log.e(TAG, "Device does not support BLE advertising")
            return
        }
        // Use extended advertising only if enabled and the hardware supports
        // both Coded PHY and extended advertising, otherwise fall back to 1M advertising
        extendedMode = BleConstants.USE_CODED_PHY &&
                Build.VERSION.SDK_INT >= Build.VERSION_CODES.O &&
                adapter.isLeCodedPhySupported &&
                adapter.isLeExtendedAdvertisingSupported
        startAdvertiser()
    }

    private fun startAdvertiser() {
        if (extendedMode) {
            Log.i(TAG, "Starting extended (Coded PHY / long range) advertising")
            advertiser?.startAdvertisingSet(advertiseSetParameters, advertiseData, null, null, null, advertisingSetCallback)
        } else {
            advertiser?.startAdvertising(advertiseSettings, advertiseData, advertiseCallback)
        }
    }

    private fun stopAdvertiser() {
        try {
            if (extendedMode) advertiser?.stopAdvertisingSet(advertisingSetCallback)
            else advertiser?.stopAdvertising(advertiseCallback)
        } catch (_: Exception) {}
    }

    fun stop() {
        stopAdvertiser()
        isAdvertising = false
        pausedForCap = false
        Log.i(TAG, "Advertising stopped")
    }

    /**
     * Suppress advertising because we've hit the connection cap, becomes undiscoverable so other
     * nodes stop trying to connect to us. Reversible via [resume], [stop] clears it permanently.
     */
    fun pause() {
        if (!isAdvertising) return
        stopAdvertiser()
        isAdvertising = false
        pausedForCap = true
        Log.i(TAG, "Advertising paused (at connection cap)")
    }

    /**
     * Force restart advertising (stop + start) to recover from a silent advert death. Called by the
     * radio watchdog alongside a scan restart. Doesn't run if lack of scanned adverts is because we intentionally paused at the connection cap
     */
    fun forceRestart() {
        if (pausedForCap) return
        val ctx = appContext ?: return
        stopAdvertiser()
        isAdvertising = false
        start(ctx)
        Log.w(TAG, "Advertising force-restarted (radio watchdog)")
    }

    // Resume advertising after dropping below the cap. it never restarts advertising after a deliberate [stop] or before the first [start].
    fun resume() {
        if (isAdvertising || !pausedForCap) return
        pausedForCap = false
        startAdvertiser()
        Log.i(TAG, "Advertising resumed (below cap)")
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

    private val advertisingSetCallback = object : AdvertisingSetCallback() {
        override fun onAdvertisingSetStarted(advertisingSet: AdvertisingSet?, txPower: Int, status: Int) {
            if (status == AdvertisingSetCallback.ADVERTISE_SUCCESS) {
                isAdvertising = true
                Log.i(TAG, "Extended (Coded) advertising started, txPower=$txPower")
            } else {
                isAdvertising = false
                Log.e(TAG, "Extended advertising failed: status=$status")
            }
        }

        override fun onAdvertisingSetStopped(advertisingSet: AdvertisingSet?) {
            isAdvertising = false
            Log.i(TAG, "Extended advertising stopped")
        }
    }
}
