package net.qaul.ble.core

import android.content.Context
import android.content.Intent
import net.qaul.ble.service.BleService

class ServiceManager() {
    companion object {
        lateinit var bleService: BleService
        fun startService(context: Context) {
            BleService().start(context)
        }
        val serviceManager = this
    }
}