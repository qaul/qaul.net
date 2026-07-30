package net.qaul.ble.test.ble.connection

import android.bluetooth.BluetoothDevice
import android.bluetooth.BluetoothGattCharacteristic
import android.util.Log
import net.qaul.ble.BleConstants
import net.qaul.ble.test.ble.advertiser.BleAdvertiser
import net.qaul.ble.test.ble.scanner.BleScanner
import net.qaul.ble.test.ble.manager.ConnectionEventListener
import net.qaul.ble.test.ble.queue.BleTaskScheduler
import net.qaul.ble.test.ble.util.toHexString
import net.qaul.ble.test.ble.util.toHexKey
import net.qaul.ble.test.ble.metrics.SessionLogger
import net.qaul.ble.test.ble.metrics.GpsProvider
import android.content.Context
import net.qaul.ble.test.ble.queue.NeighbourUpdate
import java.util.concurrent.ConcurrentHashMap
import java.util.concurrent.Executors
import java.util.concurrent.ScheduledFuture
import java.util.concurrent.TimeUnit

object ConnectionPool {
    private const val TAG = "ConnectionPool"
    @Volatile private var appContext: Context? = null   // for SessionLogger / GpsProvider access
    private val connections = ConcurrentHashMap<String, BleConnection>() // MAC address → BleConnection. remoteQaulId set once READ_CHAR comes back.
    // We would likely want another map of qaul ids to connections once the qaul id is retrieved to improve lookups
    private val pendingDisconnects = mutableSetOf<String>() // addresses of intentional disconnects in flight

    /**
     * Fired after the connections map changes (add or remove), for both roles. Lets observers like the
     * the foreground service notification read an accurate count
     */
    var onConnectionsChanged: (() -> Unit)? = null

    /**
     * Neighbour up/down, keyed by qaul ID. A neighbour is "up" while at least
     * one connection to that qaul ID is resolved, and "down" once the last such connection goes
     * away. BleManager wires these out to its qaul facing callbacks.
     */
    var onNeighbourUp: ((qaulId: ByteArray) -> Unit)? = null

    /** Fired with a sent message's terminal delivery outcome (see each BleConnection.onMessageResult) forwarded up to libqaul as a real BleDirectSendResult. */
    var onMessageResult: ((messageId: String, success: Boolean) -> Unit)? = null
    var onNeighbourDown: ((qaulId: ByteArray) -> Unit)? = null

    // qaul IDs (hex) currently reachable via at least one connection — the dedup key for up/down.
    private val upNeighbours = mutableSetOf<String>()

    // Link-state table: every origin's reported neighbour list, learned via flooded SEND_NEIGHBOURS
    // gossip (TTL-hops). Keyed by the origin's QAUL_ID_ADVERT_BYTES prefix (hex). Soft-state: an
    // entry lives only while fresh gossip keeps arriving (dedup by seq) and is expired on a timer, no acks or resends used
    // no longer removed on our local disconnect, because an origin can still be reachable via a another route.
    private data class LsEntry(val seq: Int, val neighbours: Set<String>, val sealed: Boolean, val lastSeen: Long)
    private val linkState = ConcurrentHashMap<String, LsEntry>()
    // Our own origin key.
    private fun selfKey() = BleConstants.LOCAL_QAUL_ID.copyOf(BleConstants.QAUL_ID_ADVERT_BYTES).toHexKey()
    // Wraparound safe freshness: is `new` strictly ahead of `old` in the 16-bit seq space?
    private fun seqFresher(new: Int, old: Int) = ((new - old) and 0xFFFF) in 1 until 0x8000

    @Volatile private var sealedSince: Long = 0L   // 0 = not currently sealed

    private val reaper = Executors.newSingleThreadScheduledExecutor { r ->
        Thread(r, "connection-aliveness-watchdog").apply { isDaemon = true }
    }
    @Volatile private var unresolvedReaperTask: ScheduledFuture<*>? = null
    @Volatile private var livenessReaperTask: ScheduledFuture<*>? = null

    @Volatile private var snapshotTask: ScheduledFuture<*>? = null

    @Volatile private var pingTask: ScheduledFuture<*>? = null

    @Volatile private var identityRetryTask: ScheduledFuture<*>? = null

    @Volatile private var radioHealthTask: ScheduledFuture<*>? = null

    /**
     * Periodic one-line view of this node's connections, count
     */
    private fun logTopologySnapshot() {
        try {
            val conns = connections.values.toList()
            val summary = if (conns.isEmpty()) "none" else conns.joinToString("  ·  ") { c ->
                val id = c.remoteQaulId?.toHexString() ?: "unresolved"
                "${c.device.address}/${c.role}/$id"
            }
            Log.i(TAG, "my q8id: ${BleConstants.LOCAL_QAUL_ID.toHexString()}")
            Log.i(TAG, "TOPOLOGY neighbours=${conns.size} up=${upNeighbours.size}: $summary")
            // Radio health — if the mesh goes dark this line shows whether the scanner/advertiser
            // actually stopped (scanning=false / advertising=false) vs. just being lonely.
            val (scanResults, distinctPeers, msSinceResult) = BleScanner.drainScanStats()
            val lastResultStr = if (msSinceResult < 0) "never" else "${msSinceResult}ms ago"
            Log.i(
                TAG,
                "RADIO scanning=${BleScanner.isScanning} (pausedForConnect=${BleScanner.pausedForConnect}) " +
                        "advertising=${BleAdvertiser.isAdvertising} (pausedForCap=${BleAdvertiser.pausedForCap}) " +
                        "| scanResults=$scanResults distinctPeers=$distinctPeers lastResult=$lastResultStr"
            )
        } catch (e: Exception) {
            Log.e(TAG, "snapshot failed", e)
        }
    }

    /**
     * Field-test telemetry: one snapshot line per tick with our degree, neighbour qaul-ID prefixes,
     * their PHY, and current GPS fix. For reconstructing the global mesh graph + positions offline.
     */
    private fun logTelemetrySnapshot() {
        val ctx = appContext ?: return
        try {
            val conns = connections.values.toList()
            val nbrs = conns.map { c ->
                Triple(c.remoteQaulId?.toHexKey()?.take(6) ?: "unresolved", c.rssi, c.phyLabel)
            }
            SessionLogger[ctx].snapshot(conns.size, nbrs, GpsProvider.last())
        } catch (e: Exception) {
            Log.e(TAG, "telemetry snapshot failed", e)
        }
    }

    /**
     * Status for the on device debug overlay. Reads non draining scan values so it can refresh
     * faster than the 10s log without stealing its window.
     */
    fun debugStatusText(): String {
        val conns = connections.values.toList()
        val sb = StringBuilder()
        sb.append("q8id ${BleConstants.LOCAL_QAUL_ID.toHexString()}\n")
        sb.append("neighbours=${conns.size}  up=${upNeighbours.size}\n")
        if (conns.isEmpty()) {
            sb.append("  (no neighbours)\n")
        } else {
            conns.forEach { c ->
                val id = c.remoteQaulId?.toHexString() ?: "unresolved"
                sb.append("  ${c.role.name.take(1)} ${c.phyLabel} ${c.device.address}  $id\n")
            }
        }
        val since = BleScanner.millisSinceLastResult()
        val sinceStr = if (since < 0) "never" else "${since / 1000}s ago"
        sb.append("scan=${BleScanner.isScanning} adv=${BleAdvertiser.isAdvertising} capPaused=${BleAdvertiser.pausedForCap}\n")
        sb.append("scanResults(total)=${BleScanner.totalScanResults}  lastResult=$sinceStr")
        if (BleConstants.ANTI_ISLANDING) {
            // hop = distinct peers reachable via a neighbour; lists = how many neighbours have sent
            // us their list; open = exact free-slot count over the neighborhood
            val twoHop = linkState.values.flatMap { it.neighbours }.toSet().size
            sb.append("\nhop=$twoHop  open=${openSlots()}  lists=${linkState.size}")
        }
        return sb.toString()
    }

    /** Short one-liner for the overlay's collapsed pill. */
    fun debugSummary(): String {
        val s = if (BleScanner.isScanning) "S" else "s"
        val a = if (BleAdvertiser.isAdvertising) "A" else "a"
        return "BLE ${connections.size}n $s$a"
    }

    private fun reapLiveness() {
        try {
            val now = System.currentTimeMillis()
            connections.values.toList().forEach { conn ->
                if (now - conn.lastActivityAt > BleConstants.LIVENESS_TIMEOUT_MS)
                {
                    disconnect(conn.device)
                    Log.w(TAG, "Liveness: ${conn.device.address} last seen > ${BleConstants.LIVENESS_TIMEOUT_MS}ms ago — dropping")
                }
            }
        }
        catch (e: Exception){
            Log.e(TAG, "Reaper attempt failed", e)
        }
    }

   /**
     * Drops connections that never resolved a qaul ID within [BleConstants.UNRESOLVED_TIMEOUT_MS] such as stuck handshakes
     * and zombies ( an inbound peripheral leg whose central never sent SEND_ID, or whose remote
     * already abandoned it)
     */
    private fun reapUnresolved() {
        try {
            val now = System.currentTimeMillis()
            connections.values.toList().forEach { conn ->
                // Measured from when the handshake could actually start, not from createdAt
                // this reaper is only for established links whose handshake never finished
                val startedAt = conn.handshakeStartedAt
                if (startedAt != null && conn.remoteQaulId == null &&
                    now - startedAt > BleConstants.UNRESOLVED_TIMEOUT_MS) {
                    Log.w(TAG, "Unresolved reaper: ${conn.device.address}/${conn.role} never resolved in ${BleConstants.UNRESOLVED_TIMEOUT_MS}ms — dropping")
                    disconnect(conn.device)
                }
            }
        } catch (e: Exception) {
            Log.e(TAG, "Unresolved reaper failed", e)
        }
    }

    /** Re-send SEND_ID on every still unresolved link (no-op if the transport is ready and the
     *  ID has arrived). */
    private fun retryUnresolvedIdentity() {
        try {
            connections.values.toList().forEach { it.resendIdentity() }
        } catch (e: Exception) {
            Log.e(TAG, "Identity retry failed", e)
        }
    }

    private fun pingAll() {
        try {
            connections.values.toList().forEach {
                it.sendPing()
                // Refresh live RSSI for field-test telemetry currently (CENTRAL only)
                if (it.role == BleRole.CENTRAL) BleTaskScheduler.readRemoteRssi(it.device)
            }
        } catch (e: Exception) { Log.e(TAG, "pingAll failed", e) }
    }

    /**
     * Recovers from a silently killed scanner/advertiser (Android stops them with no callback, so the
     * flags lie). [BleScanner.maintainScan] restarts the scan only after backed-off silence, so a
     * device out of range doesn't churn, and we refresh the advertiser whenever the scan was refreshed.
     */
    private fun checkRadioHealth() {
        try {
            if (BleScanner.maintainScan(BleConstants.SCAN_SILENCE_RESTART_MS)) {
                BleAdvertiser.forceRestart()
            }
        } catch (e: Exception) { Log.e(TAG, "radio health check failed", e) }
    }

    fun start(context: Context) {
        appContext = context.applicationContext
        BleTaskScheduler.registerListener(connectionEventListener)
        // Diagnostic topology snapshot — no behavioural effect, safe to remove later.
        // And takes a telemetry snapshot to the field test SessionLogger
        snapshotTask = reaper.scheduleWithFixedDelay(
            { logTopologySnapshot(); logTelemetrySnapshot(); expireLinkState(); stage2Check() }, 3_000L, 3_000L, TimeUnit.MILLISECONDS
        )
        // Soft-state keepalive: periodically re broadcast our own neighbour list even if
        // unchanged, so peers' link-state timeouts don't starve on a static mesh and in case of lost packets. The change-driven
        // broadcast still fires immediately on connect/disconnect for fast updates.
        reaper.scheduleWithFixedDelay(
            { if (BleConstants.ANTI_ISLANDING) try { broadcastNeighbourList() } catch (e: Exception) { Log.e(TAG, "keepalive broadcast failed", e) } },
            BleConstants.NEIGHBOUR_KEEPALIVE_MS, BleConstants.NEIGHBOUR_KEEPALIVE_MS, TimeUnit.MILLISECONDS
        )
        // Unresolved-connection reaper: ENABLED. Drops stuck/zombie handshakes (remoteQaulId == null
        // after UNRESOLVED_TIMEOUT_MS). Safe to run always — it never targets resolved connections.
        unresolvedReaperTask = reaper.scheduleWithFixedDelay(
            { reapUnresolved() },
            BleConstants.LIVENESS_CHECK_INTERVAL_MS,
            BleConstants.LIVENESS_CHECK_INTERVAL_MS,
            TimeUnit.MILLISECONDS
        )

        livenessReaperTask = reaper.scheduleWithFixedDelay(
            { reapLiveness() },
            BleConstants.LIVENESS_CHECK_INTERVAL_MS,
            BleConstants.LIVENESS_CHECK_INTERVAL_MS,
            TimeUnit.MILLISECONDS
        )

        pingTask = reaper.scheduleWithFixedDelay(
            { pingAll() },
            BleConstants.PING_INTERVAL_MS,
            BleConstants.PING_INTERVAL_MS,
            TimeUnit.MILLISECONDS
        )
        // does it matter that this keeps going after all resolved?
        identityRetryTask = reaper.scheduleWithFixedDelay(
            { retryUnresolvedIdentity() },
            BleConstants.IDENTITY_RETRY_MS,
            BleConstants.IDENTITY_RETRY_MS,
            TimeUnit.MILLISECONDS
        )

        radioHealthTask = reaper.scheduleWithFixedDelay(
            { checkRadioHealth() },
            BleConstants.RADIO_HEALTH_INTERVAL_MS,
            BleConstants.RADIO_HEALTH_INTERVAL_MS,
            TimeUnit.MILLISECONDS
        )
    }

    fun stop() {
        unresolvedReaperTask?.cancel(false)
        unresolvedReaperTask = null
        livenessReaperTask?.cancel(false)
        livenessReaperTask = null
        pingTask?.cancel(false)
        pingTask = null
        identityRetryTask?.cancel(false)
        identityRetryTask = null
        radioHealthTask?.cancel(false)
        radioHealthTask = null
        snapshotTask?.cancel(false)
        snapshotTask = null
        BleTaskScheduler.unregisterListener(connectionEventListener)
        connections.values.forEach { it.disconnect() }
        connections.clear()
        upNeighbours.clear()
    }
    // How to deal with 2 devices both acting as central and peripheral at each other?

    // Connect / Disconnect

    fun createConnection(device: BluetoothDevice, role: BleRole, phy: Int = android.bluetooth.BluetoothDevice.PHY_LE_1M_MASK) {
        if (connections.containsKey(device.address)) {
            Log.w(TAG, "Already connected to ${device.address}, ignoring")
            return
        }
        // here we likely put device limit
        val newConnection = BleConnection(device, role, phy)
        newConnection.onQaulIdResolved = { dev, qaulId -> handleQaulIdResolved(dev, qaulId) }
        newConnection.onMessageResult = { messageId, success -> onMessageResult?.invoke(messageId, success) }
        newConnection.onNeighboursReceived = { dev, update -> handleNeighboursReceived(dev, update) }
        connections[device.address] = newConnection
        newConnection.connect()
        Log.i(TAG, "Connection added for ${device.address} (${connections.size} total)")
        notifyConnectionsChanged()
    }

    fun disconnect(device: BluetoothDevice) {
        val conn = connections.remove(device.address) ?: run {
            Log.w(TAG, "disconnect called but no connection found for ${device.address}")
            return
        }
        // Only CENTRAL connections get a BleTaskScheduler onDisconnectedFromDevice callback to
        // confirm the disconnect. PERIPHERAL disconnects are handled entirely by GattServer so
        // pendingDisconnects would never be cleared, so we ensure
        // here that only CENTRALS get disconnected
        if (conn.role == BleRole.CENTRAL) {
            pendingDisconnects.add(device.address)
        }
        conn.disconnect()
        conn.failPendingMessages()   // report any in-flight sends to this peer as failed
        // Telemtry disconnect logging
        appContext?.let { ctx ->
            SessionLogger[ctx].disconnect(
                conn.remoteQaulId?.toHexKey()?.take(6) ?: "unresolved",
                "intentional", System.currentTimeMillis() - conn.createdAt
            )
        }
        Log.i(TAG, "Connection removed for ${device.address} (${connections.size} remaining)")
        // Re-evaluate after removal: only reports DOWN if no other leg still holds this qaul ID.
        refreshNeighbourDown(conn.remoteQaulId)
        notifyConnectionsChanged()
    }



    /**
     * Call after any connection add/remove. Toggles advertising on the connection cap. stop
     * advertising once full so peers stop discovering us and stop trying to connect. the GattServer
     * rejects them at the cap anyway
     */
    private fun notifyConnectionsChanged() {
        if (getSize() >= BleConstants.MAX_CONNECTIONS) BleAdvertiser.pause() else BleAdvertiser.resume()
        onConnectionsChanged?.invoke()
    }


    // Lookups

    fun getByAddress(address: String) : BleConnection? = connections[address]

    fun getByQaulId(qaulId: ByteArray) : BleConnection? = connections.values.firstOrNull{it.remoteQaulId?.contentEquals(qaulId) == true}

    fun allConnections(): List<BleConnection> = connections.values.toList()

    fun getSize(): Int = connections.size

    /** Count of outbound (CENTRAL) connections still in-flight, connected but qaul id hasn't resolved.
     *  The scanner gates new auto-connects on this so it can't pile connects onto the serial GATT queue
     *  faster than they resolve. Inbound peripheral legs aren't counted as we don't initiate those. */
    fun connectingCount(): Int =
        connections.values.count { it.role == BleRole.CENTRAL && it.remoteQaulId == null }

    /**
     * The active connection whose remote qaul ID begins with [prefix] (the advertised truncated
     * ID), or null. Matches regardless of RPA address rotation — the basis for churn-free
     * auto-connect dedup. The scanner inspects the returned connection's role to decide whether to
     * skip (already in an acceptable role) or connect to fix a wrong-role peripheral. Connections
     * whose ID hasn't resolved yet (remoteQaulId == null) don't match; that brief window is covered
     * by the address-level dedup.
     */
    fun getByQaulIdPrefix(prefix: ByteArray): BleConnection? =
        connections.values.firstOrNull { conn ->
            val full = conn.remoteQaulId
            full != null && full.size >= prefix.size && full.copyOf(prefix.size).contentEquals(prefix)
        }

    /**
     * Pre-connection role hint: should WE be central given the peer's advertised qaul-ID [prefix]?
     * Lower qaul ID = central (compares our ID's matching-length prefix to theirs). Non-authoritative
     * — a prefix collision is resolved by the full-ID tiebreaker after connecting.
     */
    fun localShouldBeCentral(prefix: ByteArray): Boolean {
        val ours = BleConstants.LOCAL_QAUL_ID.copyOf(prefix.size)
        return compareUnsigned(ours, prefix) < 0
    }

    /**
     * Called by BleConnection when it first resolves the remote's qaul ID from the data stream
     * (SEND_ID FLC). For PERIPHERAL connections this is the only place we learn the remote ID,
     * as centrals can read it, so we use it to detect when two pool entries refer to the same physical device
     * so we use it to detect when two pool entries refer to the same physical device (both devices connected to each other simultaneously).
     * When a duplicate is found we resolve it with the qaul-ID tiebreaker: the device with the
     * lower qaul ID should be the CENTRAL. We drop whichever of the two connections contradicts
     * that, if we should be central we drop our PERIPHERAL entry, otherwise we drop our
     * CENTRAL entry. Both peers run the same comparison, so they agree on which connection survives.
     * TODO: Look into enhanced decision making for tie breaking, for example, the more powerful device should likely be CENTRAL as they can use a smaller connection interval, increasing throughput. there may be other factors as well
     */

    private fun handleQaulIdResolved(device: BluetoothDevice, qaulId: ByteArray) {
        markNeighbourUp(qaulId)

        val existing = connections.values.firstOrNull{
            it.remoteQaulId?.contentEquals(qaulId) == true && it.device.address != device.address
        }
        if (existing == null || existing.device.address == device.address) return

        val localShouldBeCentral = compareUnsigned(BleConstants.LOCAL_QAUL_ID, qaulId) < 0
        // Choose by ROLE,, lower qaul id should be central currently.
        val keepRole = if (localShouldBeCentral) BleRole.CENTRAL else BleRole.PERIPHERAL
        val justResolved = connections[device.address]
        val toDisconnect = if (justResolved?.role == keepRole) existing.device else device
        Log.w(TAG, "Tiebreaker (SEND_ID path): local ${if (localShouldBeCentral) "wins" else "loses"} CENTRAL, keeping $keepRole leg, dropping ${toDisconnect.address}")
        disconnect(toDisconnect)
    }

    private fun compareUnsigned(a: ByteArray, b: ByteArray): Int {
        val len = minOf(a.size, b.size)
        for (i in 0 until len) {
            val diff = (a[i].toInt() and 0xFF) - (b[i].toInt() and 0xFF)
            if (diff != 0) return diff
        }
        return a.size - b.size
    }


    // Neighbour up/down (qaul-ID keyed, deduplicated across connections)

    /**
     * Mark the neighbour with [qaulId] reachable. Fires [onNeighbourUp] only on the transition from
     * absent → present (the first connection to resolve this ID). A second connection to the same ID
     * (e.g. the other leg of a dual connection) is deduplicated — add() returns false — so qaul sees
     * exactly one UP per neighbour.
     */
    private fun markNeighbourUp(qaulId: ByteArray) {
        // Test topology:
        if (!BleConstants.isAllowedNeighbour(qaulId)) {
            Log.w(TAG, "Allowlist: ${qaulId.toHexString()} not a permitted neighbour — dropping")
            connections.values.toList()
                .filter { it.remoteQaulId?.contentEquals(qaulId) == true }
                .forEach { disconnect(it.device) }
            return
        }
        if (upNeighbours.add(qaulId.toHexString())) {
            Log.i(TAG, "Neighbour up: ${qaulId.toHexString()}")
            onNeighbourUp?.invoke(qaulId)
            broadcastNeighbourList()   // our neighbour set grew, tell everyone including the new peer
        }
    }

    /**
     * Re-evaluate reachability for [qaulId] after a connection was removed, firing [onNeighbourDown]
     * only if no remaining connection still holds this ID. This is what makes dropping one leg of a
     * dual connection (the tiebreaker) silent. has to be called after the connection has been removed
     * from [connections].
     */
     fun refreshNeighbourDown(qaulId: ByteArray?) {
        qaulId ?: return
        val stillReachable = connections.values.any { it.remoteQaulId?.contentEquals(qaulId) == true }
        if (!stillReachable && upNeighbours.remove(qaulId.toHexString())) {
            Log.i(TAG, "Neighbour down: ${qaulId.toHexString()}")
            onNeighbourDown?.invoke(qaulId)
            broadcastNeighbourList()   // our neighbour set shrank, tell the rest
        }
    }

    // --------------------------------------------------------------------------------------------
    // BLE Topology Management:
    // --------------------------------------------------------------------------------------------

    /** Store a peer's reported neighbour list (its entries are QAUL_ID_ADVERT_BYTES prefixes). */
    private fun handleNeighboursReceived(device: BluetoothDevice, u: NeighbourUpdate) {
        if (u.ttl < 1 || u.ttl > BleConstants.TTL) return           // out of range TTL (parser also guards)
        val originKey = u.origin.toHexKey()
        if (originKey == selfKey()) return                          // our own list echoed back, ignore
        val prev = linkState[originKey]
        if (prev != null && !seqFresher(u.seq, prev.seq)) return    // stale or duplicate, drop
        linkState[originKey] = LsEntry(
            u.seq, u.neighbours.map { it.toHexKey() }.toSet(), u.sealed, System.currentTimeMillis()
        )
        if (u.ttl > 1) relayNeighbourList(device, u)
    }

    /** Expire link state entries we haven't heard a fresh update for. Runs on the periodic tick. */ //TODO: Evaluate timings
    private fun expireLinkState() {
        val cutoff = System.currentTimeMillis() - BleConstants.LINK_STATE_TIMEOUT_MS
        linkState.entries.removeIf { it.value.lastSeen < cutoff }
    }

    /** Our current neighbour list: each resolved neighbour's QAUL_ID_ADVERT_BYTES. */
    private fun currentNeighbourPrefixes(): List<ByteArray> =
        connections.values.mapNotNull { it.remoteQaulId?.copyOf(BleConstants.QAUL_ID_ADVERT_BYTES) }

    /** Push our current neighbour list to every connection (small FLC message). Called whenever our resolved neighbour set changes. */
    private fun broadcastNeighbourList() {
        if (!BleConstants.ANTI_ISLANDING) return
        val prefixes = currentNeighbourPrefixes()
        val localId = BleConstants.LOCAL_QAUL_ID.copyOf(BleConstants.QAUL_ID_ADVERT_BYTES)
        val seq = BleConstants.nextNeighbourSeq()
        val sealed = openSlots() == 0   // our own "whole 3-hop view full" state, for Stage 2's election
        connections.values.forEach { it.sendNeighbourList(localId, seq, BleConstants.TTL, sealed, prefixes)
        }
    }


    /** Relay a received neighbour list onward to the rest of our connections*/
    private fun relayNeighbourList(receivedFrom: BluetoothDevice, neighboursUpdate: NeighbourUpdate) {
        if (!BleConstants.ANTI_ISLANDING) return
        connections.values.forEach {
            if (it.device.address != receivedFrom.address){   //  not back to sender
                it.sendNeighbourList(neighboursUpdate.origin, neighboursUpdate.seq, neighboursUpdate.ttl-1, neighboursUpdate.sealed, neighboursUpdate.neighbours)
            }
        }
    }

    /** Is [prefix] (an advertised qaul ID prefix) already reachable within the neighborhood?
     *  If true, connecting to it would just close a triangle, false, then it's a bridge to something new! */
    fun is3HopReachable(prefix: ByteArray): Boolean {
        val key = prefix.copyOf(BleConstants.QAUL_ID_ADVERT_BYTES).toHexKey()
        return linkState.values.any { it.neighbours.contains(key) }
    }

    /** deg(key) as known from the gossip view: for a node we've heard broadcast FROM, we check their broadcasted count
     *  an origin we have no entry for defaults to 0. Self uses our true real connection
     *  count. */
    private fun degreeOf(key: String): Int =
        if (key == selfKey()) connections.size else (linkState[key]?.neighbours?.size ?: 0)

    /** Sum of free slots (CAP − degree) across every node in the link-state ball +
     *  ourselves. optionally with a hypotheticalPeer edge added first. This is
     *  the exact recomputation both the Stage 1 fill-gate and Stage 2 share, so
     *  adding is the exact inverse of dropping which is needed so both checks agree*/
    private fun openSlots(hypotheticalPeer: String? = null): Int {
        val self = selfKey()
        val ball = linkState.keys + self + (hypotheticalPeer?.let { setOf(it) } ?: emptySet())
        return ball.sumOf { key ->
            val base = degreeOf(key)
            val bumped = if (hypotheticalPeer != null && (key == self || key == hypotheticalPeer)) base + 1 else base
            (BleConstants.MAX_CONNECTIONS - bumped.coerceAtMost(BleConstants.MAX_CONNECTIONS)).coerceAtLeast(0)
        }
    }

    /** Stage 1 fill-gate: should we form an edge to the peer advertising [prefix]?
     *  - Reject outright if my slots are full, or the PEER's are of course
     *  - MERGE (peer unreachable in our neighbourhood view) → always accept, it's a bridge to something new.
     *  - REDUNDANT (peer already reachable in that view) → accept only if the edge would NOT seal the
     *    whole neighborhood, via the open slot recomputation above. */


    fun shouldAcceptEdge(prefix: ByteArray): Boolean = fillGateDecision(prefix).first

    /** The fill gate decision plus a short reason, reason just for telemetry/replay markers.
     *  @return accept? to the reason it was/wasnt accepted. */
    fun fillGateDecision(prefix: ByteArray): Pair<Boolean, String> {
        if (connections.size >= BleConstants.MAX_CONNECTIONS) return false to "local slots full"
        val key = prefix.copyOf(BleConstants.QAUL_ID_ADVERT_BYTES).toHexKey()
        if (degreeOf(key) >= BleConstants.MAX_CONNECTIONS) return false to "peer slots full"
        if (!is3HopReachable(prefix)) return true to "merge"
        return if (openSlots(hypotheticalPeer = key) > 0) true to "redundant, slots remain"
               else false to "redundant, would seal"
    }

    /** openSlots() for callers outside the pool (telemetry only). */
    fun openSlotCount(): Int = openSlots()


    /** Stage 2 reactive drop: **/

    private fun stage2Check(){
        if (!BleConstants.ANTI_ISLANDING || !BleConstants.STAGE2_PROACTIVE_DROP) return
        if (openSlots() > 0){
            sealedSince = 0L
            return
        }
        if (sealedSince == 0L){
            sealedSince = System.currentTimeMillis() // just became sealed, start the stability clock
            return
        }

        if (System.currentTimeMillis() - sealedSince < BleConstants.T_STABLE_MS) return // hasnt been stable long enough

        val candidates = connections.values.filter { isTriangle(it) }
        if (candidates.isEmpty()) return // no triangle safe edge here, another node or stage 3's problem

        if (!isElectedNode()) return    // election: only the lowest-qaul-id node in the
        // 2-hop neighborhood actually acts this round (correctness floor)

        val victim = candidates.filter { it.rssi != null }.minByOrNull { it.rssi!! } ?: candidates.first()

        if (!isTriangle(victim)) return

        val peerKey = victim.remoteQaulId?.toHexKey()?.take(6) ?: "unresolved"
        Log.i(TAG, "Stage 2: dropping $peerKey (${victim.device.address}) — sealed + triangle-safe, rssi=${victim.rssi}")
        appContext?.let { ctx ->
            SessionLogger[ctx].topoEvent(
                2, "drop", peerKey,
                "sealed ${BleConstants.T_STABLE_MS / 1000}s + triangle-safe (rssi=${victim.rssi ?: "n/a"})",
                0
            )
        }
        disconnect(victim.device)                      // existing disconnect() path already does the
        sealedSince = 0L
    }

    private fun isTriangle(conn: BleConnection): Boolean {
        val peerKey = conn.remoteQaulId?.copyOf(BleConstants.QAUL_ID_ADVERT_BYTES)?.toHexKey() ?: return false
        val isTriangle = connections.values.any { other ->
            other.device.address != conn.device.address &&
                    linkState[other.remoteQaulId?.copyOf(BleConstants.QAUL_ID_ADVERT_BYTES)
                        ?.toHexKey()]
                        ?.neighbours?.contains(peerKey) == true
        }
        return isTriangle
    }

    private fun isElectedNode(): Boolean {
        val sealedRivals = linkState.filterValues { it.sealed }.keys
        return sealedRivals.all { it >= selfKey() }
    }






    // Send


    fun sendMessage(qaulId: ByteArray, payload: ByteArray, messageId: String) {
        val conn = getByQaulId(qaulId)
        if (conn != null) {
            conn.sendMessage(payload, messageId)
        }
        else {
            // Definite failure: no BLE link to this peer. Report it immediately rather than leaving
            // libqaul without a result.
            Log.i(TAG, "Send failed, not connected to any device with Qaul ID: $qaulId")
            onMessageResult?.invoke(messageId, false)
        }
    }

    // Sends to all connected devices
    fun broadcast(payload: ByteArray){
        connections.values.forEach { it.sendMessage(payload) }
    }

    private val connectionEventListener = object : ConnectionEventListener {

        // Callback only for a CENTRAL connection
        override fun onDisconnectedFromDevice(device: BluetoothDevice) {
            if (pendingDisconnects.remove(device.address)) {
                // Intentional disconnect — already removed from map in disconnect() (which also
                // already ran refreshNeighbourDown), don't touch it (a new connection for this
                // address may already exist)
                Log.i(TAG, "Intentional disconnect confirmed for ${device.address}")
            } else {
                // Unexpected drop — clean up, then re-evaluate neighbour reachability
                val conn = connections.remove(device.address)
                conn?.failPendingMessages()   // fail in-flight sends to this peer → libqaul re-routes
                // telemetry
                conn?.let { c ->
                    appContext?.let { ctx ->
                        SessionLogger[ctx].disconnect(
                            c.remoteQaulId?.toHexKey()?.take(6) ?: "unresolved",
                            "unexpected", System.currentTimeMillis() - c.createdAt
                        )
                    }
                }
                Log.i(TAG, "Unexpected disconnect cleaned up for ${device.address}")
                refreshNeighbourDown(conn?.remoteQaulId)
            }
            notifyConnectionsChanged()
        }

        override fun onNotificationReceived(
            device: BluetoothDevice,
            characteristic: BluetoothGattCharacteristic,
            value: ByteArray
        ) {
            if (characteristic.uuid == BleConstants.MSG_CHAR) {
                connections[device.address]?.onChunkReceived(value)
            }
        }

        override fun onMtuChanged(device: BluetoothDevice, newMtu: Int) {
            connections[device.address]?.onMtuNegotiated(newMtu)
        }

        override fun onPhyUpdated(device: BluetoothDevice, txPhy: Int, rxPhy: Int) {
            connections[device.address]?.phyLabel = when (txPhy) {
                BluetoothDevice.PHY_LE_1M -> "1M"
                BluetoothDevice.PHY_LE_2M -> "2M"
                BluetoothDevice.PHY_LE_CODED -> "Coded"
                else -> "phy$txPhy"
            }
        }

        override fun onRssiRead(device: BluetoothDevice, rssi: Int) {
            connections[device.address]?.rssi = rssi
        }

        override fun onServicesDiscovered(device: BluetoothDevice) {
            connections[device.address]?.onServicesDiscovered()
        }

        override fun onCharacteristicRead(
            device: BluetoothDevice,
            characteristic: BluetoothGattCharacteristic,
            value: ByteArray
        ) {
            if (characteristic.uuid == BleConstants.PSM_CHAR) {
                // Peripheral's L2CAP PSM (4-byte big-endian). Open the high-bandwidth channel.
                val psm = if (value.size >= 4) {
                    java.nio.ByteBuffer.wrap(value).order(java.nio.ByteOrder.BIG_ENDIAN).int
                } else -1
                Log.i(TAG, "PSM received from ${device.address}: $psm")
                connections[device.address]?.connectL2cap(psm)
                return
            }
            if (characteristic.uuid == BleConstants.READ_CHAR) {
                val existing = getByQaulId(value)

                // Record the ID and announce the neighbour UP first, before any tiebreaker
                // disconnect. With both legs holding the ID, dropping either one leaves the neighbour up.
                connections[device.address]?.let { conn ->
                    conn.remoteQaulId = value
                    Log.i(TAG, "Qaul ID received from ${device.address}: ${value.toHexString()}")
                    markNeighbourUp(value)
                    appContext?.let { ctx -> // telemetry
                        SessionLogger[ctx].connect(value.toHexKey().take(6), conn.phyLabel, null, conn.role.name)
                    }
                }

                if (existing != null && existing.device.address != device.address) {
                    when (existing.role) {
                        BleRole.CENTRAL -> {
                            // Two CENTRAL connections to same device — drop the newer one (the
                            // neighbour stays up via the existing CENTRAL leg).
                            Log.w(
                                TAG,
                                "Already connected as CENTRAL to this qaul ID via ${existing.device.address}, dropping duplicate CENTRAL ${device.address}"
                            )
                            disconnect(device)
                            return
                        }

                        BleRole.PERIPHERAL -> {
                            // Timing B: PERIPHERAL already resolved before we connected as CENTRAL.
                            // Apply tiebreaker. Both legs now hold the qaul ID, so dropping either
                            // leaves the neighbour up (no down will fire).
                            val localShouldBeCentral =
                                compareUnsigned(BleConstants.LOCAL_QAUL_ID, value) < 0
                            val toDisconnect = if (localShouldBeCentral) existing.device else device
                            Log.w(
                                TAG,
                                "Tiebreaker (READ_CHAR path): local ${if (localShouldBeCentral) "wins" else "loses"} CENTRAL, dropping ${toDisconnect.address}"
                            )
                            disconnect(toDisconnect)
                        }
                    }
                }
            }

        }
    }
}
