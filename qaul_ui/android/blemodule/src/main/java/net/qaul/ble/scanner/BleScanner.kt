package net.qaul.ble.test.ble.scanner

import android.annotation.SuppressLint
import android.bluetooth.BluetoothManager
import android.bluetooth.le.BluetoothLeScanner
import android.bluetooth.le.ScanCallback
import android.bluetooth.le.ScanFilter
import android.bluetooth.le.ScanResult
import android.bluetooth.le.ScanSettings
import android.content.Context
import android.os.ParcelUuid
import android.util.Log
import net.qaul.ble.test.ble.BleConstants
import net.qaul.ble.test.ble.connection.BleRole
import net.qaul.ble.test.ble.connection.ConnectionPool
import net.qaul.ble.test.ble.manager.BleManager

/**
 *
 * Current auto-connect policy (role-aware dedup using the advertised truncated qaul ID, matched across
 * RPA address rotation):
 *   - Already connected to this peer as CENTRAL → skip (keep it; no churn).
 *   - Connected as PERIPHERAL and we SHOULD be peripheral → skip (correct role).
 *   - Connected as PERIPHERAL but we SHOULD be central → connect anyway. This forms a dual
 *     connection, the post-connection tiebreaker then drops the peripheral, leaving us central
 *     so roles converge to the deterministic "lower qaul ID = central" outcome regardless of who
 *     connected first. TODO: There could be ways to mitigate this, possibly waiting for the other to connect first to
 *     prevent all unnecesary connections.
 *   - Not connected → connect (as CENTRAL).
 *
 * This gives deterministic role assignment.
 * Asymmetric discovery (only one side can see the other) keeps a connected but suboptimal role
 * link, connectivity over role-optimality. (Same-MAC peripherals can't be fixed this way, since
 * the MAC-keyed pool can't hold both roles for one address; rare, and connectivity is preserved.)
 */
@SuppressLint("MissingPermission")
object BleScanner {
    private const val TAG = "BleScanner"
    private const val COOLDOWN_MS = 30_000L

    private var scanner: BluetoothLeScanner? = null
//    private var context: Context? = null

    var isScanning = false
        private set

    /** Manual mode = false (default). Toggle on to auto-connect to discovered valid peers. */
    var autoConnect = false

    /** Fired for every scan result so the UI (or anything) can list discovered peers. */
    var onPeerDiscovered: ((ScanResult) -> Unit)? = null

    private val lock = Any()
    // MAC → timestamp until which auto-connect is suppressed (set after a tiebreaker drop).
    private val cooldownUntil = mutableMapOf<String, Long>()

    fun start(context: Context) {
        if (isScanning) return
        val adapter = (context.getSystemService(Context.BLUETOOTH_SERVICE) as BluetoothManager).adapter
        scanner = adapter?.bluetoothLeScanner ?: run {
            Log.e(TAG, "No BluetoothLeScanner available")
            return
        }

        val settingsBuilder = ScanSettings.Builder()
            .setScanMode(ScanSettings.SCAN_MODE_LOW_LATENCY)
        if (adapter.isLeExtendedAdvertisingSupported) {
            // setLegacy(false) = report legacy AND extended (so we keep legacy visibility).
            settingsBuilder.setLegacy(false).setPhy(ScanSettings.PHY_LE_ALL_SUPPORTED)
            Log.i(TAG, "Extended scanning enabled")
        }
        val scanSettings = settingsBuilder.build()

        // Filter on our service UUID so we only see qaul peers (also required for background scans).
        val filter = ScanFilter.Builder()
            .setServiceUuid(ParcelUuid(BleConstants.SERVICE_UUID))
            .build()
        scanner?.startScan(listOf(filter), scanSettings, scanCallback)
        isScanning = true
        Log.i(TAG, "Scanning started (autoConnect=$autoConnect)")
    }

    fun stop() {
        if (!isScanning) return
        scanner?.stopScan(scanCallback)
        isScanning = false
        Log.i(TAG, "Scanning stopped")
    }

    /**
     * Suppress auto-connect to [macAddress] for a cooldown window. Call this when an auto-established
     * connection is dropped (e.g. by the dual-connection tiebreaker) so the next scan result for the
     * same MAC doesn't immediately re-trigger a connect. With qaul-ID dedup this is mostly a
     * safety net — after a dual-connection drop the kept connection already dedups the peer.
     */
    fun applyCooldown(macAddress: String) {
        synchronized(lock) {
            cooldownUntil[macAddress] = System.currentTimeMillis() + COOLDOWN_MS
        }
    }

    private val scanCallback = object : ScanCallback() {
        override fun onScanResult(callbackType: Int, result: ScanResult) {
            onPeerDiscovered?.invoke(result)            // always announce (manual mode)
            if (autoConnect) maybeAutoConnect(result)   // autonomous mode (opt-in)
        }

        override fun onScanFailed(errorCode: Int) {
            Log.e(TAG, "Scan failed: $errorCode")
        }
    }

    private fun maybeAutoConnect(result: ScanResult) {
        synchronized(lock) {
            val mac = result.device.address
            if (System.currentTimeMillis() < (cooldownUntil[mac] ?: 0L)) return   // recently dropped
            if (ConnectionPool.getSize() >= BleConstants.MAX_CONNECTIONS) return  // respect the cap

            val prefix = result.scanRecord?.getManufacturerSpecificData(BleConstants.QAUL_MANUFACTURER_ID)
            val existing = if (prefix != null) ConnectionPool.getByQaulIdPrefix(prefix) else null

            if (existing != null) {
                // We already know this peer by qaul ID (across RPA rotation). Decide by its role:
                when (existing.role) {
                    // Already CENTRAL → keep it. (If we "should" be peripheral, the peer fixes it by
                    // connecting to us; if asymmetric, we keep this suboptimal-but-connected link.)
                    BleRole.CENTRAL -> return
                    // PERIPHERAL in the role we SHOULD have → keep it, no churn.
                    // PERIPHERAL but we should be CENTRAL → fall through and connect to fix the role
                    // (creates a dual; the tiebreaker drops the peripheral, leaving us central).
                    BleRole.PERIPHERAL -> if (!ConnectionPool.localShouldBeCentral(prefix!!)) return
                }
            } else {
                // Unknown peer by qaul ID (ID not resolved yet, or no advertised ID) — fall back to
                // address-level dedup so we don't re-connect to a peer we're mid-handshake with.
                if (BleManager.isConnected(mac)) return
            }

            Log.i(TAG, "Auto-connecting to $mac")
            BleManager.connect(result.device, BleRole.CENTRAL)
        }
    }
}
