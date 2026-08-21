package net.qaul.ble.test.ble.scanner

import android.annotation.SuppressLint
import android.bluetooth.BluetoothDevice
import android.bluetooth.BluetoothManager
import android.bluetooth.le.BluetoothLeScanner
import android.bluetooth.le.ScanCallback
import android.bluetooth.le.ScanFilter
import android.bluetooth.le.ScanResult
import android.bluetooth.le.ScanSettings
import android.content.Context
import android.os.Build
import android.os.Handler
import android.os.Looper
import android.os.ParcelUuid
import android.util.Log
import net.qaul.ble.BleConstants
import net.qaul.ble.test.ble.connection.BleRole
import net.qaul.ble.test.ble.connection.ConnectionPool
import net.qaul.ble.test.ble.manager.BleManager
import net.qaul.ble.test.ble.queue.BleTaskScheduler
import net.qaul.ble.test.ble.util.toHexString
import net.qaul.ble.test.ble.util.toHexKey
import net.qaul.ble.test.ble.metrics.SessionLogger
import kotlin.math.pow
import kotlin.random.Random

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

    // After a connect attempt finishes, wait this long with no further connects before resuming the
    // scan. Debounces a burst of connects into one pause window, as Android has a limit of ~5 startScan / 30s.
    private const val SCAN_RESUME_DEBOUNCE_MS = 2_000L

    private var scanner: BluetoothLeScanner? = null
//    private var context: Context? = null

    var isScanning = false
        private set

    /** Toggle on to auto-connect to discovered valid peers. */
    var autoConnect = true

    /** Fired for every scan result so the UI (or anything) can list discovered peers. */
    var onPeerDiscovered: ((ScanResult) -> Unit)? = null

    private val lock = Any()
    // MAC → timestamp until which auto-connect is suppressed (set after a tiebreaker drop).
    private val cooldownUntil = mutableMapOf<String, Long>()

    // Advertised qaul ID prefix (hex). First time we saw a should be peripheral peer, that's been defered to let it connect. Cleared once a connection to that peer forms.
    private val deferredSince = mutableMapOf<String, Long>()

    // last time we saw them on the 1M PHY (short range). Drives the connect PHY choice: seen on 1M recently → connect 1M (fast)
    private val seen1MAt = mutableMapOf<String, Long>()

    /** When we first saw a peer on Coded with no 1M sighting to contradict it. Cleared by recordPhySighting when a 1M advert arrives.
    * If this has stood for [BleConstants.CODED_ONLY_CONFIRM_MS] then we accept Coded as the peers only route. */
    private val codedOnlySince = mutableMapOf<String, Long>()

    /** Start of the current UNBROKEN run of 1M sightings for a peer (by advertised prefix). Reset
     *  whenever the run lapses — see recordPhySighting. Drives the Coded→1M upgrade. */
    private val first1MOfStreakAt = mutableMapOf<String, Long>()

    private val lastPhyUpgradeAt = mutableMapOf<String, Long>()   // prefix hex -> last upgrade attempt

    // MACs currently rejected because of fill-gate. Logged once on the transition into rejection, not on
    // every scan cycle, it would otherwise spam one log line per SCAN_INTERVAL_MS forever.
    private val fillGateRejected = mutableSetOf<String>()

    // Pause scanning during connecting state. The BLE radio is shared between scanning and connecting, so we pause the scan around
    // each connect attempt and resume it once the connect activity quiets.
    private var appContext: Context? = null
    @Volatile var pausedForConnect = false
        private set
    private val mainHandler = Handler(Looper.getMainLooper())
    private val resumeRunnable = Runnable { doResume() }

    fun start(context: Context) {
        appContext = context.applicationContext   // cached so the debounced resume needs no Context
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
        scanStartedAt = System.currentTimeMillis()
        Log.i(TAG, "Scanning started (autoConnect=$autoConnect)")
    }

    fun stop() {
        mainHandler.removeCallbacks(resumeRunnable)
        mainHandler.removeCallbacks(scanRestartRunnable)
        pausedForConnect = false
        if (!isScanning) return
        scanner?.stopScan(scanCallback)
        isScanning = false
        Log.i(TAG, "Scanning stopped")
    }

    /**
     * Pause scanning for the duration of a connection attempt. Reversed by
     * [resumeAfterConnect] (debounced).
     */
    fun pauseForConnect() {
        if (!BleConstants.SCAN_PAUSE_DURING_CONNECT) return   // disabled: see constant; keeps scan continuous
        mainHandler.removeCallbacks(resumeRunnable)   // cancel any pending resume
        if (isScanning) {
            scanner?.stopScan(scanCallback)
            isScanning = false
            pausedForConnect = true
            Log.i(TAG, "Scan paused for connect")
        }
    }

    /**
     * Resume scanning after a connect attempt finishes (success, failure, or timeout). Debounced:
     * a run of back to back connects keeps the scan paused and restarts it only once activity quiets,
     * so we never toggle the scan fast enough to hit the limit.
     */
    fun resumeAfterConnect() {
        if (!pausedForConnect) return
        mainHandler.removeCallbacks(resumeRunnable)
        mainHandler.postDelayed(resumeRunnable, SCAN_RESUME_DEBOUNCE_MS)
    }

    private fun doResume() {
        if (!pausedForConnect) return
        pausedForConnect = false
        appContext?.let { start(it) }
    }

    /**
     * Suppress auto-connect to [macAddress] for a cooldown window. Call this when an auto-established
     * connection is dropped (e.g. by the dual-connection tiebreaker) so the next scan result for the
     * same MAC doesn't immediately re-trigger a connect. With qaul-ID dedup this is mostly a
     * safety net — after a dual-connection drop the kept connection already dedups the peer.
     */
    fun applyCooldown(macAddress: String) {
        val key = backoffKey(macAddress)
        synchronized(lock) {
            cooldownUntil[key] = System.currentTimeMillis() + COOLDOWN_MS
        }
    }

    /**
     * A link to this peer went away, restart its wrong role defer window
     *
     */
    fun noteDisconnected(macAddress: String) {
        synchronized(lock) {
            macToPrefix[macAddress]?.let { deferredSince.remove(it) }
        }
    }

    // Consecutive connect failure count, used to grow the reconnect backoff. Reset on a
    // successful GATT connect. This backoff exists to dampen the rapid same address retry churn (133s) within a few seconds.
    private val failureCount = mutableMapOf<String, Int>()

    // ---- scan visibility telemetry ----------

    private class Sighting(var count: Int = 0, var rssi: Int = 0, var phy: String = "?", var lastAt: Long = 0)
    private val sightings = mutableMapOf<String, Sighting>()        // peer prefix (or mac) -> seen
    private val skipReasons = mutableMapOf<String, MutableMap<String, Int>>()  // peer -> reason -> n

    /** Record that we saw this peer's advert. Called for every qaul scan result. */
    private fun noteSighting(key: String, rssi: Int, phy: Int) {
        val s = sightings.getOrPut(key) { Sighting() }
        s.count++; s.rssi = rssi; s.lastAt = System.currentTimeMillis()
        s.phy = when (phy) {
            BluetoothDevice.PHY_LE_1M -> "1M"
            BluetoothDevice.PHY_LE_CODED -> "Coded"
            BluetoothDevice.PHY_LE_2M -> "2M"
            else -> "?"
        }
    }

    /** Record why we saw a peer and chose not to connect to it. */
    private fun noteSkip(key: String, reason: String) {
        skipReasons.getOrPut(key) { mutableMapOf() }.merge(reason, 1, Int::plus)
    }

    /** One peers visibility over the last snapshot interval: how often we saw it, how strong, on
     *  which PHY, and every reason we declined to connect  */
    data class ScanVisibility(
        val peer: String, val seen: Int, val rssi: Int, val phy: String, val skips: Map<String, Int>
    )

    /** Hand the intervals visibility to the telemetry snapshot and reset */
    fun drainScanVisibility(): List<ScanVisibility> = synchronized(lock) {
        val keys = sightings.keys + skipReasons.keys
        val out = keys.map { k ->
            val s = sightings[k]
            ScanVisibility(k, s?.count ?: 0, s?.rssi ?: 0, s?.phy ?: "?", skipReasons[k]?.toMap() ?: emptyMap())
        }
        sightings.clear(); skipReasons.clear()
        out
    }

    // MAC -> advertised qaul ID prefix, learned from scan results. Peers rotate their RPA (and draw a
    // fresh one every time their advertising set restarts), so a mac keyed backoff will reset
    // on every rotation and never engage as the peer looks like a brand new device each time
    private val macToPrefix = mutableMapOf<String, String>()

    /** Addresses confirmed (by GATT inspection, not the advert) to not be qaul peers
    * Not a permanent blocklist: as RPA rotation eventually gives the same device
     *  a new address */
    private val nonQaulPeers = mutableSetOf<String>()

    /**
     * Record that [macAddress] passed our scan filter, connected, and discovered cleanly, but
     * exposed none of qaul's characteristics, a confirmed non qaul device.
     */
    fun noteNonQaulPeer(macAddress: String) {
        synchronized(lock) { nonQaulPeers.add(macAddress) }
    }

    /** Stable backoff identity for [mac]: its advertised qaul id prefix if we've seen one, else the
     *  mac itself */
    private fun backoffKey(mac: String): String =
        synchronized(lock) { macToPrefix[mac] ?: mac }

    private fun rememberPrefix(mac: String, prefix: ByteArray?) {
        if (prefix == null) return
        synchronized(lock) { macToPrefix[mac] = prefix.toHexString() }
    }

    /**
     * Record a failed/abandoned connect to [macAddress] (status 133, watchdog timeout, etc.) and
     * apply an exponential backoff before auto connect is allowed to that MAC again:
     * delay = min(MAX, MIN · MULTIPLIER^(attempts-1)) ± JITTER. Without this, a 133 just drops the
     * connection and the very next scan result re fires the connect, hammering a peer that keeps failing.
     * The jitter keeps several peers that fail together from all retrying on the same tick.
     */
    fun noteConnectFailure(macAddress: String) {
        val key = backoffKey(macAddress)
        synchronized(lock) {
            val attempts = (failureCount[key] ?: 0) + 1
            failureCount[key] = attempts

            val freeRetries = if (ConnectionPool.getByAddress(macAddress)?.isCoded == true)
                BleConstants.CODED_RECONNECT_FREE_RETRIES else BleConstants.RECONNECT_FREE_RETRIES

            val reconnectDelayMax = if (ConnectionPool.getByAddress(macAddress)?.isCoded == true)
                BleConstants.CODED_RECONNECT_DELAY_MAX_MS else BleConstants.RECONNECT_DELAY_MAX_MS
            // First few failures retry immediately a single transient 133 shouldn't silence the
            // node. Only escalate once a peer keeps failing past the free retry grace.
            if (attempts <= freeRetries) {
                Log.i(TAG, "Connect to $macAddress failed (attempt $attempts) — retrying without backoff")
                return
            }
            val n = attempts - freeRetries
            val base = (BleConstants.RECONNECT_DELAY_MIN_MS *
                    BleConstants.RECONNECT_BACKOFF_MULTIPLIER.pow(n - 1))
                .toLong()
                .coerceAtMost(reconnectDelayMax)
            val jitter = (base * BleConstants.RECONNECT_JITTER_FACTOR).toLong()
            val delay = base + Random.nextLong(-jitter, jitter + 1)   // full ± jitter
            cooldownUntil[key] = System.currentTimeMillis() + delay
            Log.i(TAG, "Connect to $macAddress failed (attempt $attempts) — backing off ${delay}ms")
        }
    }

    @Volatile private var lastScanRestartAt = 0L
    @Volatile private var scanStartedAt = 0L
    // Consecutive watchdog restarts with no scan result since , grows the restart threshold so a
    // device legitimately out of range doesn't restart every interval. Reset to 0 on any scan result.
    @Volatile private var silentRestartCount = 0

    /**
     * Called periodically by the radio watchdog. Restarts the scan if it's been silent
     * past a threshold, but that threshold backs off with each consecutive silent restart
     * (base, 2x, 4x, ... cap), so:
     *   - a scanner that died while peers are present recovers fast
     *   - a device that's simply out of range doesn't churn every interval, restarts cap to minutes
     *   - the first scan result resets the backoff, restoring fast recovery.
     * Returns true if it restarted so the caller can also refresh the advertiser.
     */
    fun maintainScan(baseThresholdMs: Long): Boolean {
        if (pausedForConnect) return false
        if (appContext == null) return false
        // Most recent positive event: a real result, our last restart, or the scan first starting
        val lastEvent = maxOf(lastScanResultAt, lastScanRestartAt, scanStartedAt)
        if (lastEvent == 0L) return false                       // scanner hasn't started yet
        val threshold = baseThresholdMs shl minOf(silentRestartCount, 4)
        val silence = System.currentTimeMillis() - lastEvent
        if (silence < threshold) return false
        silentRestartCount++
        Log.w(TAG, "Watchdog: scan silent ${silence}ms — restart #$silentRestartCount (threshold ${threshold}ms)")
        forceRestart()
        return true
    }

    /**
     * Force a fresh scan (stop + start) to recover from a silent scan death.
     */
    fun forceRestart() {
        val ctx = appContext ?: return
        lastScanRestartAt = System.currentTimeMillis()
        if (isScanning) {
            try { scanner?.stopScan(scanCallback) } catch (_: Exception) {}
            isScanning = false
        }
        start(ctx)
        Log.w(TAG, "Scan force-restarted (radio watchdog)")
    }

    /** Clear the backoff for [macAddress] after a successful GATT connect. */
    fun noteConnectSuccess(macAddress: String) {
        val key = backoffKey(macAddress)
        synchronized(lock) {
            if (failureCount.remove(key) != null) {
                cooldownUntil.remove(key)
                Log.i(TAG, "Connect to $macAddress succeeded — backoff cleared")
            }
        }
    }

    // Scan-receive instrumentation — ground truth for "is the scanner actually delivering results"
    // (the isScanning flag is a start-time latch and can't see a silently-killed scan). onScanResult
    // fires on a binder thread, drainScanStats() runs on the snapshot thread, so both are guarded.
    private val scanStatsLock = Any()
    private var windowResultCount = 0
    private val windowDistinctPeers = mutableSetOf<String>()
    @Volatile private var lastScanResultAt = 0L   // 0 = no scan result has ever arrived

    /** Monotonic count of every scan result this session — for the debug overlay (non-draining, so it
     *  doesn't fight the 10s log's drainScanStats). Watching it climb = the scanner is alive. */
    @Volatile var totalScanResults = 0L
        private set

    /** Milliseconds since the last scan result, or -1 if none ever. for the overlay. */
    fun millisSinceLastResult(): Long =
        if (lastScanResultAt == 0L) -1L else System.currentTimeMillis() - lastScanResultAt

    /**
     * Returns (results, distinctPeers, msSinceLastResult) accumulated since the previous call, then
     * resets the window. msSinceLastResult is -1 if no scan result has ever arrived. Drives the
     * RADIO snapshot line so a dead scanner (results stuck at 0 while peers are advertising) is visible.
     */
    fun drainScanStats(): Triple<Int, Int, Long> {
        synchronized(scanStatsLock) {
            val count = windowResultCount
            val distinct = windowDistinctPeers.size
            val since = if (lastScanResultAt == 0L) -1L else System.currentTimeMillis() - lastScanResultAt
            windowResultCount = 0
            windowDistinctPeers.clear()
            return Triple(count, distinct, since)
        }
    }

    private val scanCallback = object : ScanCallback() {
        override fun onScanResult(callbackType: Int, result: ScanResult) {
            synchronized(scanStatsLock) {
                windowResultCount++
                totalScanResults++
                windowDistinctPeers.add(result.device.address)
                lastScanResultAt = System.currentTimeMillis()
            }
            silentRestartCount = 0   // results are flowing, restore fast watchdog recovery
            recordPhySighting(result)                   // track which PHY we saw this peer on
            onPeerDiscovered?.invoke(result)            // always announce (manual mode)
            if (autoConnect) maybeAutoConnect(result)   // autonomous mode (opt-in)
        }

        override fun onScanFailed(errorCode: Int) {
            Log.e(TAG, "Scan failed: $errorCode")
            if (errorCode == ScanCallback.SCAN_FAILED_ALREADY_STARTED) return
            // Schedule a restart so a silently-killed scan recovers instead of staying dead forever. SCANNING_TOO_FREQUENTLY needs a long wait to clear
            // Android's ~5-startScan/30s window, other errors can retry sooner.
            isScanning = false
            val retryDelay =
                if (errorCode == ScanCallback.SCAN_FAILED_SCANNING_TOO_FREQUENTLY) 35_000L else 5_000L
            mainHandler.removeCallbacks(scanRestartRunnable)
            mainHandler.postDelayed(scanRestartRunnable, retryDelay)
            Log.w(TAG, "Scan restart scheduled in ${retryDelay}ms (error $errorCode)")
        }
    }

    // Restarts the scan after a failure, using the cached app context.
    private val scanRestartRunnable = Runnable {
        if (!isScanning && !pausedForConnect) appContext?.let { start(it) }
    }

    private fun maybeAutoConnect(result: ScanResult) {
        // LOCK ORDER.
        var deferred: (() -> Unit)? = null
        synchronized(lock) {
            val mac = result.device.address
            val prefix = result.scanRecord?.getManufacturerSpecificData(BleConstants.QAUL_MANUFACTURER_ID)
            rememberPrefix(mac, prefix)   // so backoff survives this peer rotating its RPA
            if (mac in nonQaulPeers) { noteSkip(prefix?.toHexKey() ?: mac, "non_qaul"); return }
            val vkey = prefix?.toHexKey() ?: mac
            noteSighting(vkey, result.rssi, if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) result.primaryPhy else BluetoothDevice.PHY_LE_1M)
            val existing = if (prefix != null) ConnectionPool.getByQaulIdPrefix(prefix) else null

            val currentPhy = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) result.primaryPhy
            else BluetoothDevice.PHY_LE_1M
            // Upgrading from Coded to 1M/2M PHY when back in range. Work in progress. This is can only be done from a central connection, asymmetry, peripherals will ignore and wait for central to upgrade

            // Coded to 1M upgrade. Based on an unbroken run of 1M sightings,
            val streakStart = if (prefix != null) first1MOfStreakAt[prefix.toHexString()] else null
            val sustained1M = streakStart != null &&
                    System.currentTimeMillis() - streakStart >= BleConstants.PHY_UPGRADE_CONFIRM_MS
            if (BleConstants.ALLOW_PHY_UPGRADE && sustained1M &&
                prefix != null && existing != null && currentPhy == BluetoothDevice.PHY_LE_1M && existing.role == BleRole.CENTRAL && existing.phyLabel == "Coded") {
                val key = prefix.toHexString()
                val lastTry = lastPhyUpgradeAt[key] ?: 0L
                if (System.currentTimeMillis() - lastTry > 10_000L) {
                    lastPhyUpgradeAt[key] = System.currentTimeMillis()
                    ConnectionPool.notePhyRequestReason(existing.device.address, "coded_upgrade")
                    val target = existing.device
                    deferred = {
                        BleTaskScheduler.setPreferredPhy(
                            target,
                            BluetoothDevice.PHY_LE_1M_MASK,
                            BluetoothDevice.PHY_LE_1M_MASK,
                            BluetoothDevice.PHY_OPTION_NO_PREFERRED
                        )
                    }
                    Log.i(TAG, "Upgrading $mac from Coded to 1M.")
                    return@synchronized
                }
            }


            if (System.currentTimeMillis() < (cooldownUntil[backoffKey(mac)] ?: 0L)) { noteSkip(vkey, "backoff"); return } // recently dropped
            // Stage 1 fill-gate: reject if my slots or the peers's are full, merge (peer unreachable
            // in our TTL=3 gossip view) = accept, redundant (visible in neighbourhood) = accept only if it wouldn't seal the
            // 3-hop ball
            if (BleConstants.ANTI_ISLANDING && BleConstants.STAGE1_FILL_GATE && prefix != null) {
                val (accept, reason) = ConnectionPool.fillGateDecision(prefix)
                if (!accept && reason == "redundant, would seal") {
                    if (fillGateRejected.add(mac)) {
                        Log.i(TAG, "Fill-gate: rejecting $mac — $reason")
                        appContext?.let { ctx ->
                            SessionLogger[ctx].topoEvent(
                                // toHexKey (compact lowercase) must match the snapshot/dev id format the replay tool expects
                                1, "reject", prefix.toHexKey().take(6),
                                reason, ConnectionPool.openSlotCount()
                            )
                        }
                    }
                    noteSkip(vkey, "fill_gate")
                    return
                }
            }
            fillGateRejected.remove(mac)   // no longer rejected, re-arm the log
            if (ConnectionPool.getSize() >= BleConstants.MAX_CONNECTIONS) { noteSkip(vkey, "local_cap"); return }  // respect the cap
            // Admission control: don't start another connect while one is still resolving. Keeps the
            // serial GATT queue from jamming with concurrent connectGatts during a discovery burst.
            if (ConnectionPool.connectingCount() >= BleConstants.MAX_CONCURRENT_CONNECTING) { noteSkip(vkey, "concurrent_limit"); return }

            // Test topology: with the allowlist on, only auto-connect to designated neighbours
            if (BleConstants.TEST_NEIGHBOUR_ALLOWLIST.isNotEmpty() &&
                (prefix == null || !BleConstants.isAllowedNeighbour(prefix))) {
                noteSkip(vkey, "allowlist"); return
            }

            if (existing != null) {
                // Connected now: reset this peer's wrong role defer timer.
                prefix?.toHexString()?.let { deferredSince.remove(it) }
                // We already know this peer by qaul ID (across RPA rotation). Decide by its role:
                when (existing.role) {
                    // Already CENTRAL → keep it. (If we "should" be peripheral, the peer fixes it by
                    // connecting to us; if asymmetric, we keep this suboptimal-but-connected link.)
                    BleRole.CENTRAL -> { noteSkip(vkey, "already_central"); return }
                    // PERIPHERAL in the role we SHOULD have → keep it, no churn.
                    // PERIPHERAL but we should be CENTRAL → fall through and connect to fix the role
                    // (creates a dual; the tiebreaker drops the peripheral, leaving us central).
                    BleRole.PERIPHERAL -> if (!ConnectionPool.localShouldBeCentral(prefix!!)) { noteSkip(vkey, "role_ok_peripheral"); return }
                }
            } else {
                // Unknown peer by qaul ID (ID not resolved yet, or no advertised ID) — fall back to
                // address-level dedup so we don't re-connect to a peer we're mid-handshake with.
                if (BleManager.isConnected(mac)) { noteSkip(vkey, "addr_dedup"); return }

                // If their advertised ID is lower than ours we should be
                // PERIPHERAL, so don't race their inbound connect with our own outbound one. Wait WRONG_ROLE_DEFER_MS for them to connect to
                // us, connect outbound only as a fallback if they haven't (such as if they can't see us).
                if (prefix != null && !ConnectionPool.localShouldBeCentral(prefix)) {
                    val key = prefix.toHexString()
                    val firstSeen = deferredSince.getOrPut(key) { System.currentTimeMillis() }
                    if (System.currentTimeMillis() - firstSeen < BleConstants.WRONG_ROLE_DEFER_MS) {
                        noteSkip(vkey, "wrong_role_defer")
                        return   // give them the chance to connect to us first (correct role)
                    }
                    Log.i(TAG, "Defer window lapsed for $mac — connecting outbound (asymmetric fallback)")
                }
            }

            // Connect over the PHY of the advert we're responding to, so the peer's address and PHY
            // match. If this is the long-range (Coded) advert but we can also see this peer up close on
            // 1M, skip it, their 1M advert will drive a faster short range connection instead.
            //
            // Coded_sippressed_by_1m helps reveal whether 1m ads ar eevr slipping into farther rnage bands and blocking a coded connection, so far this seems to not be the case
            if (currentPhy == BluetoothDevice.PHY_LE_CODED && seenOn1MRecently(prefix)) {
                noteSkip(vkey, "coded_suppressed_by_1m")
                return   // prefer the peer's short-range advert
            }
            // Coded is a last resort. A dual advertising peer is visible on 1M and Coded at the same time, so
            // if we just choose based on first seen advertisement, we may create a coded link even if both devices are next to each other.

            if (currentPhy == BluetoothDevice.PHY_LE_CODED) {
                val key = prefix?.toHexString() ?: mac
                val firstCodedAt = codedOnlySince.getOrPut(key) { System.currentTimeMillis() }
                if (System.currentTimeMillis() - firstCodedAt < BleConstants.CODED_ONLY_CONFIRM_MS) {
                    noteSkip(vkey, "coded_confirming")
                    return   // still waiting to see whether they're also on 1M
                }
                Log.i(TAG, "Only seen $mac on Coded for ${BleConstants.CODED_ONLY_CONFIRM_MS}ms — connecting long-range")
            }
            val phy = if (currentPhy == BluetoothDevice.PHY_LE_CODED)
                          BluetoothDevice.PHY_LE_CODED_MASK
                      else BluetoothDevice.PHY_LE_1M_MASK
            Log.i(TAG, "Auto-connecting to $mac (${if (phy == BluetoothDevice.PHY_LE_CODED_MASK) "Coded/long-range" else "1M/short-range"})")
            appContext?.let { ctx ->
                SessionLogger[ctx].attemptStarted(
                    mac, BleRole.CENTRAL.name,
                    if (phy == BluetoothDevice.PHY_LE_CODED_MASK) "Coded" else "1M"
                )
            }
            deferred = { BleManager.connect(result.device, BleRole.CENTRAL, phy) }
        }

        deferred?.invoke()
    }

    /** Record that we saw [result]'s peer on the 1M PHY (short range), keyed by advertised prefix.
     *  API 26+ only. */
    private fun recordPhySighting(result: ScanResult) {
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.O) return
        if (result.primaryPhy != BluetoothDevice.PHY_LE_1M) return
        val prefix = result.scanRecord?.getManufacturerSpecificData(BleConstants.QAUL_MANUFACTURER_ID) ?: return
        synchronized(lock) {
            val key = prefix.toHexString()
            val now = System.currentTimeMillis()
            // A 1M streak: if the last sighting is stale the peer went out of 1M range and came
            // back, so the run restarts. Prevents inconsistent advertisements triggering upgrade
            val last = seen1MAt[key]
            if (last == null || now - last > BleConstants.OUT_OF_RANGE_TIMEOUT_MS) {
                first1MOfStreakAt[key] = now
            }
            seen1MAt[key] = now
            // Proof this peer is reachable on 1M, so it's no longer a coded only candidate
            codedOnlySince.remove(key)
        }
    }

    /** True if we've seen this peers 1M advert within the out of range window (i.e. it's close enough
     *  to connect short-range). Callers must hold [lock]. */
    private fun seenOn1MRecently(prefix: ByteArray?): Boolean {
        if (prefix == null) return false
        val last = seen1MAt[prefix.toHexString()] ?: return false
        return System.currentTimeMillis() - last < BleConstants.OUT_OF_RANGE_TIMEOUT_MS
    }
}
