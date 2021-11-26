package net.qaul.ble.service

import android.content.Context
import android.content.Intent
import androidx.lifecycle.LifecycleService
import net.qaul.ble.AppLog

class BleService(): LifecycleService() {
    private val TAG: String = BleService::class.java.getSimpleName()
    var bleService: BleService? = null

    override fun onCreate() {
        super.onCreate()
        AppLog.e(TAG, "$TAG created")
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        bleService = this
        return super.onStartCommand(intent, flags, startId)

    }

    override fun onDestroy() {
        super.onDestroy()
    }

    override fun onStart(intent: Intent?, startId: Int) {
        super.onStart(intent, startId)
        AppLog.e(TAG, "$TAG started")
    }

    fun start(context: Context) {
        if (bleService == null) {
            val intent = Intent(context, BleService::class.java)
            context.startService(intent)
        } else {
            AppLog.e(TAG, "$TAG already started")
        }
    }


    fun stop() {
        if (bleService != null) {
            bleService?.stopSelf()
        } else {
            AppLog.e(TAG, "$TAG not started")
        }
    }

}