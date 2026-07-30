package net.qaul.ble.test.ble.metrics

import android.content.Context
import android.location.Location
import android.location.LocationListener
import android.location.LocationManager
import android.os.Bundle
import android.os.Looper
import android.os.SystemClock
import android.util.Log

/**
 * Minimal last-known-position source for field-test telemetry. Uses [LocationManager] directly
 * (no play-services dependency). ACCESS_FINE_LOCATION is already required/granted for BLE scanning
 * on the versions we target, so no new permission is introduced.
 *
 * Started when a test session starts, stopped when it ends. [last] returns the freshest fix.
 */
object GpsProvider {
    private const val TAG = "qaul-blemodule GpsProvider"
    private const val MAX_AGE_MS = 90_000L   // a fix older than this is treated as "no fix" (stale/lost signal)
    private var lm: LocationManager? = null
    @Volatile private var fix: Location? = null

    /** Fix age in ms via the monotonic elapsed-realtime clock (immune to wall-clock changes). */
    private fun ageMs(l: Location): Long = (SystemClock.elapsedRealtimeNanos() - l.elapsedRealtimeNanos) / 1_000_000L

    // Full LocationListener impl (onStatusChanged/onProvider* were not default on older APIs).
    private val listener = object : LocationListener {
        override fun onLocationChanged(location: Location) { fix = location }
        override fun onStatusChanged(provider: String?, status: Int, extras: Bundle?) {}
        override fun onProviderEnabled(provider: String) {}
        override fun onProviderDisabled(provider: String) {}
    }

    fun start(context: Context) {
        try {
            val ctx = context.applicationContext
            val manager = ctx.getSystemService(Context.LOCATION_SERVICE) as LocationManager
            lm = manager
            // Seed from last-known ONLY if it's fresh — otherwise we'd report a stale cached fix
            // (e.g. a previous town) until/unless a real fix arrives.
            val lk = manager.getLastKnownLocation(LocationManager.GPS_PROVIDER)
                ?: manager.getLastKnownLocation(LocationManager.NETWORK_PROVIDER)
            fix = if (lk != null && ageMs(lk) <= MAX_AGE_MS) lk else null
            manager.requestLocationUpdates(LocationManager.GPS_PROVIDER, 3000L, 0f, listener, Looper.getMainLooper())
            try {
                manager.requestLocationUpdates(LocationManager.NETWORK_PROVIDER, 3000L, 0f, listener, Looper.getMainLooper())
            } catch (_: Exception) { /* network provider may be absent outdoors */ }
            Log.i(TAG, "GPS updates started")
        } catch (e: SecurityException) {
            Log.e(TAG, "location permission missing: ${e.message}")
        } catch (e: Exception) {
            Log.e(TAG, "start failed: ${e.message}")
        }
    }

    fun stop() {
        try { lm?.removeUpdates(listener) } catch (_: Exception) {}
        lm = null
    }

    /** (lat, lon, accuracyMeters), or null if there's no fix or the latest fix has gone stale. */
    fun last(): Triple<Double, Double, Float>? {
        val f = fix ?: return null
        if (ageMs(f) > MAX_AGE_MS) return null   // signal lost — report "no fix" rather than a stale one
        return Triple(f.latitude, f.longitude, f.accuracy)
    }
}
