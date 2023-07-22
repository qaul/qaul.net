package net.qaul.qaul_app

import android.content.Context

object PreferenceManager {
    private const val PREFS_NAME = "qaulPrefs"
    private const val KEY_BACKGROUND_SERVICE_ENABLED = "backgroundServiceEnabled"

    fun isBackgroundServiceEnabled(context: Context): Boolean {
        val sharedPreferences = context.getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE)
        // Set the default value to true for the first time
        return sharedPreferences.getBoolean(KEY_BACKGROUND_SERVICE_ENABLED, true)
    }

    fun setBackgroundServiceEnabled(context: Context, enabled: Boolean) {
        val sharedPreferences = context.getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE)
        val editor = sharedPreferences.edit()
        editor.putBoolean(KEY_BACKGROUND_SERVICE_ENABLED, enabled)
        editor.apply()
    }
}
