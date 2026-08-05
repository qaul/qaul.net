package net.qaul.ble.test.ble.metrics

import android.content.Context
import android.os.Build
import android.util.Log
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
                 gps: Triple<Double, Double, Float>?) = write(JSONObject().apply {
        put("type", "snapshot")
        put("deg", degree)
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
