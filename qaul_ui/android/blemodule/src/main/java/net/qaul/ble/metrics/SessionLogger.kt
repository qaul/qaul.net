package net.qaul.ble.test.ble.metrics

import android.content.Context
import android.os.Build
import android.util.Log
import net.qaul.ble.BleConstants
import net.qaul.ble.test.ble.advertiser.BleAdvertiser
import net.qaul.ble.test.ble.queue.BleTaskScheduler
import org.json.JSONArray
import org.json.JSONObject
import java.io.BufferedWriter
import java.io.File
import java.io.FileWriter
import java.text.SimpleDateFormat
import java.util.Locale
import java.util.concurrent.Executors
import java.util.concurrent.ScheduledFuture
import java.util.concurrent.TimeUnit

/**
 * Field-test telemetry logger. Writes one JSON object per line (JSONL) to a per-session file
 * under the app's external files dir, so many devices can be pulled/shared and the global mesh
 * topology reconstructed offline (no single device sees it).
 *
 * Every line carries `t` (epoch ms) and `dev` (device label = qaul-id prefix once known).
 * Flushes on every write so a crash / BLE-controller wedge mid-test doesn't lose the tail.
 *
 * Event types (skinny MVP):
 *   session    — once at start: model, os, qaul-id, clock offset
 *   connect    — a peer connection came up (peer id, phy, rssi, role)
 *   disconnect — a peer connection went down (peer id, reason, held ms)
 *   snapshot   — periodic: degree, neighbour list [{id,rssi,phy}], gps {lat,lon,acc}
 *
 * Message throughput lives in [BleMetrics]; this logger covers topology + position + timing.
 */
class SessionLogger private constructor(context: Context) {
    private val appContext = context.applicationContext
    private val TAG = "qaul-blemodule SessionLogger"

    @Volatile private var dev: String = "unknown"
    @Volatile private var clockOffsetMs: Long = 0     // trueTime = deviceTime + offset (from SNTP)
    private var writer: BufferedWriter? = null
    private var file: File? = null
    private val lock = Any()

    // Public-Downloads mirroring. The internal file is authoritative; this copy exists so a field
    // test participant can retrieve their own log with no adb and no in-app action — which matters
    // because the debug overlay needs draw-over-other-apps, and at least one handset refuses it.
    //
    // Keyed by file name → lastModified() at the last successful mirror, so a sweep only copies
    // what actually changed. Deliberately mtime-based rather than a "we wrote something" flag: the
    // Dart side writes routing-*.jsonl into this same directory, and mtime notices that too.
    private val mirroredAt = mutableMapOf<String, Long>()
    private val mirrorExec = Executors.newSingleThreadScheduledExecutor { r ->
        Thread(r, "session-mirror").apply { isDaemon = true }
    }
    private var mirrorTask: ScheduledFuture<*>? = null

    /**
     * Open a session file and write the header line. Called automatically once the qaul ID is known.
     *
     * Resumes the most recent file for this device if it was last written within
     * [SESSION_RESUME_GAP_MS], otherwise starts a fresh one. This keeps an app restart *during* a
     * test run appending to the same file (so a mid-test crash/reopen doesn't fragment the run),
     * while a genuinely new run on a different day still gets its own file rather than accumulating
     * into one unfindable week-long log. `resumed` on the header line marks which happened.
     */
    fun startSession(qaulId: String, clockOffsetMs: Long = 0L) {
        // Simple single gate for all telemetry. Returning here leaves writer null,, so every call
        // site across the module becomes a no op, no session file is created, and startMirroring()
        //below is never reached so nothing is written into the users Downloads folder either.
        if (!BleConstants.FIELD_TEST) {
            Log.i(TAG, "Field test logging disabled")
            return
        }
        var resumed = false
        synchronized(lock) {
            close()
            this.dev = if (qaulId.length >= 6) qaulId.substring(0, 6) else qaulId
            this.clockOffsetMs = clockOffsetMs
            try {
                val dir = sessionsDir()
                val recent = dir.listFiles { f ->
                    f.isFile && f.name.startsWith("session-$dev-") && f.name.endsWith(".jsonl")
                }?.maxByOrNull { it.lastModified() }
                    ?.takeIf { now() - it.lastModified() < SESSION_RESUME_GAP_MS }
                file = if (recent != null) {
                    resumed = true
                    recent
                } else {
                    val stamp = SimpleDateFormat("yyyyMMdd-HHmmss", Locale.US).format(now())
                    File(dir, "session-$dev-$stamp.jsonl")
                }
                writer = BufferedWriter(FileWriter(file!!, true))
            } catch (e: Exception) {
                Log.e(TAG, "startSession failed: ${e.message}")
            }
        }
        write(JSONObject().apply {
            put("type", "session")
            put("model", Build.MANUFACTURER + " " + Build.MODEL)
            put("os", Build.VERSION.RELEASE)
            put("qid", qaulId)
            put("clock_off_ms", clockOffsetMs)
            put("resumed", resumed)

            put("cap", BleConstants.MAX_CONNECTIONS)
            put("anti_islanding", BleConstants.ANTI_ISLANDING)
            put("stage1", BleConstants.STAGE1_FILL_GATE)
            put("stage2", BleConstants.STAGE2_PROACTIVE_DROP)
            put("prefer_2m", BleConstants.PREFER_2M)
            put("allow_phy_upgrade", BleConstants.ALLOW_PHY_UPGRADE)
            // The tuned timeout budgets. . Values also let
            // the analysis tools check a measured duration against the budget that was actually
            // flown, rather than against whatever the constants say today.
            put("t_fast_op", BleConstants.FAST_OP_TIMEOUT_MS)
            put("t_negotiation", BleConstants.NEGOTIATION_OP_TIMEOUT_MS)
            put("t_service_discovery", BleConstants.SERVICE_DISCOVERY_TIMEOUT_MS)
            put("t_connect", BleConstants.CONNECTION_TIMEOUT_MS)
            put("t_connect_coded", BleConstants.CODED_CONNECT_TIMEOUT_MS)
            put("t_unresolved", BleConstants.UNRESOLVED_TIMEOUT_MS)
            put("t_identity_retry", BleConstants.IDENTITY_RETRY_MS)
            put("t_liveness", BleConstants.LIVENESS_TIMEOUT_MS)
            put("t_liveness_coded", BleConstants.CODED_LIVENESS_TIMEOUT_MS)
            put("t_rssi_refresh", BleConstants.RSSI_REFRESH_MS)
            // Can this controller do long range at all? Extended advertising and Coded PHY are
            // seperate optional Bluetooth 5 features. A device may have neither and
            // the same flag gates both Coded TX and Coded RX , and some devices may only support RX or only TX
            val (capable, codedPhy, extAdv) = BleAdvertiser.codedCapabilityNow(appContext)
            put("coded_capable", capable)
            put("le_coded_phy", codedPhy)
            put("le_extended_adv", extAdv)
        })
        Log.i(TAG, "Session ${if (resumed) "resumed" else "started"} → ${file?.absolutePath}")
        startMirroring()
    }

    /**
     * Keep a copy of this run in Downloads/qaul, refreshed on a timer.
     *
     * The timer is not about the API — it's about process death. Mirroring only on a clean close
     * would lose the whole log on any phone the OS kills, and across a two-hour multi-device test
     * several will be. A periodic copy bounds the worst case to one interval instead of everything.
     * Only mirrors when something was actually written since last time, so an idle session costs
     * nothing.
     *
     * Also sweeps any other recent session file once at startup: a run split by a >30min gap (see
     * [SESSION_RESUME_GAP_MS]) leaves an earlier file that the participant still needs to send.
     */
    private fun startMirroring() {
        mirrorTask?.cancel(false)
        mirrorTask = mirrorExec.scheduleWithFixedDelay(
            { mirrorSweep() }, MIRROR_INTERVAL_MS, MIRROR_INTERVAL_MS, TimeUnit.MILLISECONDS
        )
        mirrorExec.execute { mirrorSweep() }     // don't wait a full interval for the first copy
    }

    /**
     * Mirror every recently-touched log in the sessions dir whose content has changed since we last
     * copied it.
     *
     * Sweeps the whole directory rather than just [file] on purpose: the Dart side writes
     * routing-*.jsonl alongside our session-*.jsonl (same dir, same naming, same 30-min resume
     * rule), and a participant needs both. Going by mtime means we pick those up without either
     * side having to tell the other anything.
     *
     * The [MIRROR_LOOKBACK_MS] cutoff is what keeps this bounded — it covers a whole test day,
     * including a run split by a long break, without re-copying last week's logs on every tick.
     */
    private fun mirrorSweep() {

        if (!BleConstants.FIELD_TEST) return
        try {
            val cutoff = now() - MIRROR_LOOKBACK_MS
            val files = sessionsDir().listFiles { f ->
                f.isFile && f.name.endsWith(".jsonl") && f.lastModified() >= cutoff
            } ?: return
            for (f in files) {
                val stamp = f.lastModified()
                if (mirroredAt[f.name] == stamp) continue          // unchanged since last mirror
                // Only record success, so a failed copy is simply retried on the next tick.
                if (DownloadsMirror.mirror(appContext, f)) mirroredAt[f.name] = stamp
            }
        } catch (e: Exception) {
            Log.e(TAG, "mirror sweep failed: ${e.message}")
        }
    }

    // ── Connection attempt tracking
    //

    /** Furthest point an attempt got */
    private class Attempt(
        val role: String, val phy: String, val startedAt: Long,
        /** Advertised qaul-id prefix of the peer we are dialling, when the scanner knew one.. */
        val advertPeer: String? = null
    ) {
        var stage: String = "connecting"
        /** stage name -> ms from attempt start to entering it. Lets each timeout in
         *  BleTaskScheduler.timeoutFor be set from the measured distribution of that stage rather
         *  than argued from constants */
        val stageAt = linkedMapOf<String, Long>()
    }

    private val attempts = mutableMapOf<String, Attempt>()

    fun attemptStarted(mac: String, role: String, phy: String, advertPeer: String? = null) {
        synchronized(lock) { attempts[mac] = Attempt(role, phy, now(), advertPeer) }
    }

    fun attemptStage(mac: String, stage: String) {
        synchronized(lock) {
            attempts[mac]?.let { it.stage = stage; it.stageAt.putIfAbsent(stage, now() - it.startedAt) }
        }
    }

    fun attemptEnded(mac: String, peerId: String?, success: Boolean, error: String? = null) {
        val a = synchronized(lock) { attempts.remove(mac) } ?: return
        write(JSONObject().apply {
            put("type", "connect_attempt")
            put("mac", mac)
            (peerId ?: a.advertPeer)?.let { put("peer", it) }
            put("role", a.role)
            put("phy", a.phy)
            put("ok", success)
            put("reached", a.stage)
            put("ms", now() - a.startedAt)   // time it took to establish or failed
            if (a.stageAt.isNotEmpty()) put("at", JSONObject(a.stageAt as Map<*, *>))
            error?.let { put("err", it) }
        })
    }

    fun gossipTx(seq: Int, sealed: Boolean, nbrs: Int, fanout: Int, trigger: String) =
        write(JSONObject().apply {
            put("type", "gossip_tx"); put("seq", seq); put("sealed", sealed)
            put("nbrs", nbrs); put("fanout", fanout); put("trigger", trigger)
        })

    /** first receipt of a new neighbour gossip. Duplicates are only counted in the snapshot instead to avoid bloat  */
    fun gossipRx(origin: String, seq: Int, hops: Int, relayed: Boolean, lsSize: Int) =
        write(JSONObject().apply {
            put("type", "gossip_rx"); put("origin", origin); put("seq", seq)
            put("hops", hops); put("relayed", relayed); put("ls", lsSize)
        })

    /**
     * A completed bulk transfer
     */
    fun transfer(peer: String, direction: String, transport: String, sizeBytes: Int, ms: Long,
                 phy: String, rssi: Int?) = write(JSONObject().apply {
        put("type", "transfer"); put("peer", peer); put("dir", direction); put("transport", transport)
        put("bytes", sizeBytes); put("ms", ms)
        put("kbps", if (ms > 0) (sizeBytes * 8.0 / ms) else 0.0)
        put("phy", phy); rssi?.let { put("rssi", it) }
    })

    /**
     * A link's negotiated PHY changed. Covers every path that can move it: the initial connect PHY,
     * the bulk transfer 1M to 2M escalation and its downgrade back, and the Coded to 1M upgrade
     */
    fun phyChange(peer: String, from: String, to: String, reason: String, rssi: Int?) =
        write(JSONObject().apply {
            put("type", "phy"); put("peer", peer); put("from", from); put("to", to)
            put("reason", reason); rssi?.let { put("rssi", it) }
        })

    fun connect(peerId: String, phy: String, rssi: Int?, role: String) = write(JSONObject().apply {
        put("type", "connect"); put("peer", peerId); put("phy", phy); put("role", role)
        rssi?.let { put("rssi", it) }   // omitted until RSSI is plumbed through
    })

    fun disconnect(peerId: String, reason: String, heldMs: Long) = write(JSONObject().apply {
        put("type", "disconnect"); put("peer", peerId); put("reason", reason); put("held_ms", heldMs)
    })

    /**
     * Topology-management decision (anti-islanding). Surfaced as a marker in the replay tool so a
     * fill-gate rejection or a proactive drop is visible at the moment it happened.
     *
     * @param stage  1 = fill gate (edge formation), 2 = proactive drop
     * @param action e.g. "reject" (stage 1), "drop" (stage 2)
     * @param peer   the peer id prefix the decision was about
     * @param reason short human-readable why, shown in the replay popup
     * @param open   openSlots() at decision time — the number the decision keyed off
     */
    fun topoEvent(stage: Int, action: String, peer: String, reason: String, open: Int) =
        write(JSONObject().apply {
            put("type", "topo"); put("stage", stage); put("action", action)
            put("peer", peer); put("reason", reason); put("open", open)
        })

    fun messageSent(messageId: String, messageSize: Int, messageReceiver: String, nextHopReceiver: String) = write(JSONObject().apply {
       put("type", "message_sent"); put("id", messageId); put("size", messageSize); put("to", messageReceiver); put("next_hop", nextHopReceiver)
    })

    fun messageAcked() {

    }

    fun messageReceived() {

    }

    /**
     * @param neighbours list of (peerId, rssi?, phyLabel) for currently-held connections (rssi may be null)
     * @param gps        (lat, lon, accuracyMeters) or null if unavailable
     */
    fun snapshot(degree: Int, neighbours: List<Triple<String, Int?, String>>,
                 gps: Triple<Double, Double, Float>?,
                 gossipRx: Int = 0, gossipDup: Int = 0, gossipRelayed: Int = 0,
                 linkStateSize: Int = 0,
                 queue: BleTaskScheduler.QueueDepths? = null,
                 adv1M: Boolean? = null, advCoded: Boolean? = null, // advertiser state, the what and why
                 pausedForCap: Boolean? = null) = write(JSONObject().apply {
        put("type", "snapshot")
        put("deg", degree)
        adv1M?.let { put("adv_1m", it) }
        advCoded?.let { put("adv_coded", it) }
        pausedForCap?.let { put("adv_paused_cap", it) }
        put("g_rx", gossipRx); put("g_dup", gossipDup)
        put("g_relay", gossipRelayed); put("g_ls", linkStateSize)
        queue?.let { q ->
            put("q", JSONObject().apply {
                put("ctrl", q.control); put("med", q.medium); put("bulk", q.bulk)
                put("pending", q.pending); put("pending_ms", q.pendingMs)
                q.pendingPeer?.let { put("pending_peer", it) }
            })
        }
        put("nbrs", JSONArray().apply {
            neighbours.forEach { (id, rssi, phy) ->
                put(JSONObject().apply { put("id", id); rssi?.let { put("rssi", it) }; put("phy", phy) })
            }
        })
        gps?.let { (lat, lon, acc) ->
            put("gps", JSONObject().apply { put("lat", lat); put("lon", lon); put("acc", acc.toDouble()) })
        }
    })

    /**
     * Where session files live: INTERNAL storage (`filesDir`), not the external files dir.
     *
     * Android 11 (API 30) blocks adb from reading /storage/emulated/0/Android/data/<pkg>/ — plain
     * `adb shell`/`adb pull` get EPERM, and `run-as` fails too because it doesn't inherit the app's
     * storage mount namespace. Files written there are readable by the app but unretrievable over
     * adb, which is useless for a field test. Internal storage has no such restriction: `run-as`
     * reads it on every API level (debug builds only, which is what we field-test with).
     *
     * Pull with tools/field-test/pull-sessions.sh, or by hand:
     *   adb shell run-as net.qaul.qaul_app ls files/sessions
     *   adb exec-out run-as net.qaul.qaul_app cat files/sessions/<name>.jsonl > <name>.jsonl
     */
    private fun sessionsDir(): File {
        val dir = File(appContext.filesDir, "sessions")
        if (!dir.exists() && !dir.mkdirs()) Log.e(TAG, "could not create ${dir.absolutePath}")
        return dir
    }

    private fun now() = System.currentTimeMillis()

    private fun write(obj: JSONObject) {
        synchronized(lock) {
            val w = writer ?: return
            try {
                obj.put("t", now())
                obj.put("dev", dev)
                w.append(obj.toString()); w.append('\n'); w.flush()
            } catch (e: Exception) {
                Log.e(TAG, "write failed: ${e.message}")
            }
        }
    }

    /**
     * What the scanner could see this interval, and every reason it declined to connect.
     */
    fun scanVisibility(rows: List<Triple<String, Triple<Int, Int, String>, Map<String, Int>>>) {
        if (rows.isEmpty()) return
        write(JSONObject().apply {
            put("type", "scan")
            put("peers", JSONArray().apply {
                rows.forEach { (peer, seen, skips) ->
                    put(JSONObject().apply {
                        put("id", peer.take(6))
                        put("n", seen.first)
                        if (seen.first > 0) { put("rssi", seen.second); put("phy", seen.third) }
                        if (skips.isNotEmpty()) put("skip", JSONObject(skips as Map<*, *>))
                    })
                }
            })
        })
    }

    /**
     * We turned an inbound connection away. Logged from the peripheral side, which is the half of
     * this event that has never been visible: the central only ever saw a CCCD write fail, with no
     * way to tell whether the peer was genuinely full, still staging, or something else entirely.
     *
     * [poolSize] vs [connecting] is the important pair. A refusal at poolSize=4 with connecting=0 is
     * a genuinely saturated device; the same refusal with connecting=2 means half the cap was held
     * by handshakes that had not resolved, the "falsely full" case, where real capacity existed.
     */
    fun inboundRefused(mac: String, reason: String, poolSize: Int, connecting: Int) =
        write(JSONObject().apply {
            put("type", "refused")
            put("mac", mac)
            put("reason", reason)
            put("pool", poolSize)
            put("connecting", connecting)
        })

    /** A scheduler operation finished or was killed by its watchdog.*/
    fun op(mac: String, op: String, ms: Long, ok: Boolean, budget: Long? = null) =
        write(JSONObject().apply {
            put("type", "op")
            put("mac", mac)
            put("op", op)
            put("ms", ms)
            put("ok", ok)
            budget?.let { put("budget", it) }
        })

    fun currentFilePath(): String? = file?.absolutePath

    fun close() {
        synchronized(lock) {
            mirrorTask?.cancel(false)
            mirrorTask = null
            try { writer?.flush(); writer?.close() } catch (_: Exception) {}
            writer = null
        }
        // Sweep after the writer is closed, so the exported copy is the complete file. Off the
        // caller's thread: close() runs on shutdown paths and a MediaStore write is not instant.
        mirrorExec.execute { mirrorSweep() }
    }

    companion object {
        /** Reopening within this window appends to the previous file (same test run, app was
         *  restarted); a longer gap starts a new one. */
        const val SESSION_RESUME_GAP_MS = 30 * 60 * 1000L   // 30 min

        /** How often the Downloads copy is refreshed. Also the worst-case data loss if the OS kills
         *  the process, since nothing else flushes the export. */
        const val MIRROR_INTERVAL_MS = 90_000L

        /** At startup, mirror any session file touched this recently — wide enough to cover a whole
         *  test day (including a run split by a long break) without dragging in last week's. */
        const val MIRROR_LOOKBACK_MS = 8 * 60 * 60 * 1000L  // 8 h

        @Volatile private var instance: SessionLogger? = null
        operator fun get(context: Context): SessionLogger =
            instance ?: synchronized(this) {
                instance ?: SessionLogger(context).also { instance = it }
            }
    }
}
