package net.qaul.ble.test.ble.metrics

import android.content.ContentUris
import android.content.ContentValues
import android.content.Context
import android.media.MediaScannerConnection
import android.net.Uri
import android.os.Build
import android.os.Environment
import android.provider.MediaStore
import android.util.Log
import java.io.File

/**
 * Copies a log file into the phone's public Downloads folder, so a field-test participant can find
 * it in the Files app and send it on without adb, a debug overlay, or any action inside the app.
 *
 * Why mirror rather than write there directly — the API allows it (MediaStore supports an append
 * stream on API 29+), but [SessionLogger]'s primary file must stay on internal storage:
 *  - it's on a hot path (a flush per event), and every MediaStore write is a binder round trip;
 *  - a long-lived MediaStore stream isn't flushed if the process is killed, which is exactly when
 *    the log matters most;
 *  - the resume-on-restart lookup keys off File.lastModified() in the app's own directory.
 * So the internal file stays authoritative and fast, and the public copy is a replaceable export.
 * That also leaves the existing adb pull path untouched for the devices we own.
 */
object DownloadsMirror {

    private const val TAG = "qaul-blemodule DownloadsMirror"

    /** Sub-folder inside Downloads, so a participant's Downloads root isn't littered. */
    private const val SUBDIR = "qaul"

    /** text/plain, not application/jsonl: share targets routinely refuse MIME types they don't
     *  know, and discovering that with 20 people waiting is not the moment. */
    private const val MIME = "text/plain"

    /** Cached MediaStore entry per file name, so repeated mirrors overwrite one entry instead of
     *  accumulating "session-x (1).jsonl", "(2)", … Cleared whenever a write against it fails. */
    private val uris = mutableMapOf<String, Uri>()

    /** Copy [source] into Downloads/[SUBDIR], replacing any previous copy. Returns false on failure
     *  (caller should retry later — a failed mirror is not fatal, the internal file still has it). */
    @Synchronized
    fun mirror(context: Context, source: File): Boolean {
        if (!source.isFile) return false
        return try {
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) mediaStoreCopy(context, source)
            else legacyCopy(context, source)
        } catch (e: Exception) {
            Log.e(TAG, "mirror of ${source.name} failed: ${e.message}")
            uris.remove(source.name)          // force a fresh insert next attempt
            false
        }
    }

    // API 29+: scoped storage. No permission needed to write our own entry in Downloads.
    private fun mediaStoreCopy(context: Context, source: File): Boolean {
        val resolver = context.contentResolver
        val name = source.name

        // Reuse the entry from a previous mirror. After process death the cache is empty, so also
        // look it up by name — otherwise a restarted app inserts a duplicate every time, and a
        // resumed session (same file name) would litter Downloads with numbered copies.
        val existing = uris[name] ?: findExisting(context, name)?.also { uris[name] = it }

        if (existing != null) {
            try {
                // "rwt" truncates first, so a shrinking file can't leave a stale tail behind.
                resolver.openOutputStream(existing, "rwt")?.use { out ->
                    source.inputStream().use { it.copyTo(out) }
                    return true
                }
            } catch (e: Exception) {
                Log.w(TAG, "existing entry for $name unusable (${e.message}) — reinserting")
            }
            uris.remove(name)
        }

        // IS_PENDING hides the entry from other apps until the copy is complete, so a participant
        // browsing Files mid-write never sees a half-written log.
        val pending = ContentValues().apply {
            put(MediaStore.MediaColumns.DISPLAY_NAME, name)
            put(MediaStore.MediaColumns.MIME_TYPE, MIME)
            put(MediaStore.MediaColumns.RELATIVE_PATH, "${Environment.DIRECTORY_DOWNLOADS}/$SUBDIR")
            put(MediaStore.MediaColumns.IS_PENDING, 1)
        }
        val uri = resolver.insert(MediaStore.Downloads.EXTERNAL_CONTENT_URI, pending)
            ?: run { Log.e(TAG, "MediaStore insert returned null for $name"); return false }

        resolver.openOutputStream(uri)?.use { out -> source.inputStream().use { it.copyTo(out) } }
        resolver.update(uri, ContentValues().apply {
            put(MediaStore.MediaColumns.IS_PENDING, 0)
        }, null, null)

        uris[name] = uri
        return true
    }

    /**
     * Our own previously-inserted entry for [name], if it's still there (an app only sees entries it
     * owns). Any extras beyond the first are deleted, so repeated runs converge on one file.
     *
     * Matches on a PREFIX rather than the exact name, because MediaStore does not necessarily store
     * the display name we asked for:
     *  - it enforces that the extension matches MIME_TYPE, so ".jsonl" + "text/plain" can land as
     *    "….jsonl.txt";
     *  - on a name collision it appends " (1)", " (2)", …
     * An exact-match lookup misses both, which is how a relaunch ended up inserting a duplicate
     * every time instead of overwriting. Session/routing names embed the device id and a timestamp,
     * so a prefix is still unambiguous.
     */
    private fun findExisting(context: Context, name: String): Uri? {
        val projection = arrayOf(MediaStore.MediaColumns._ID)
        val selection = "${MediaStore.MediaColumns.DISPLAY_NAME} LIKE ?"
        val ids = mutableListOf<Long>()
        context.contentResolver.query(
            MediaStore.Downloads.EXTERNAL_CONTENT_URI, projection, selection, arrayOf("$name%"), null
        )?.use { c ->
            val col = c.getColumnIndexOrThrow(MediaStore.MediaColumns._ID)
            while (c.moveToNext()) ids += c.getLong(col)
        }
        if (ids.isEmpty()) return null
        val keep = ContentUris.withAppendedId(MediaStore.Downloads.EXTERNAL_CONTENT_URI, ids.first())
        // Tidy up duplicates a previous build left behind, so a participant sends one file per log
        // rather than picking between "(1)" and "(2)".
        ids.drop(1).forEach { extra ->
            try {
                context.contentResolver.delete(
                    ContentUris.withAppendedId(MediaStore.Downloads.EXTERNAL_CONTENT_URI, extra),
                    null, null
                )
            } catch (e: Exception) {
                Log.w(TAG, "could not remove duplicate export of $name: ${e.message}")
            }
        }
        return keep
    }

    // API 26-28: pre-scoped-storage, so a plain file copy works — but it needs
    // WRITE_EXTERNAL_STORAGE to have been granted at runtime. If it wasn't, mkdirs/copy throws or
    // returns false and we just log it; the internal file is unaffected.
    @Suppress("DEPRECATION")
    private fun legacyCopy(context: Context, source: File): Boolean {
        val dir = File(Environment.getExternalStoragePublicDirectory(Environment.DIRECTORY_DOWNLOADS), SUBDIR)
        if (!dir.exists() && !dir.mkdirs()) {
            Log.e(TAG, "could not create ${dir.absolutePath} — is WRITE_EXTERNAL_STORAGE granted?")
            return false
        }
        val dest = File(dir, source.name)
        source.copyTo(dest, overwrite = true)

        // Writing the bytes isn't enough on this path. Files/Downloads browsers read the MediaStore
        // index, not the raw filesystem, and a plain File write doesn't touch it — so without this
        // the copy exists on disk and is invisible in the Files app, which for a participant is the
        // same as it not being there. API 29+ doesn't need it: MediaStore *is* the write path.
        MediaScannerConnection.scanFile(context, arrayOf(dest.absolutePath), arrayOf(MIME), null)

        Log.i(TAG, "mirrored ${source.name} → ${dest.absolutePath}")
        return true
    }
}
