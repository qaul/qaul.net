package net.qaul.ble.test.ble.service

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent
import android.app.Service
import android.bluetooth.BluetoothDevice
import android.content.Context
import android.content.Intent
import android.os.Build
import android.os.IBinder
import androidx.core.app.NotificationCompat
import androidx.core.content.ContextCompat
import net.qaul.ble.test.MainActivity
import net.qaul.ble.test.ble.advertiser.BleAdvertiser
import net.qaul.ble.test.ble.connection.ConnectionPool
import net.qaul.ble.test.ble.manager.BleManager
import net.qaul.ble.test.ble.manager.ConnectionEventListener
import net.qaul.ble.test.ble.queue.BleTaskScheduler
import net.qaul.ble.test.ble.scanner.BleScanner
import net.qaul.ble.test.ble.server.GattServer

/**
 * Foreground service that keeps the BLE module alive when the app is in the background.
 *
 * Android kills backgrounded app processes within minutes, which would stop scanning, advertising
 * and connecting. A foreground service with the ongoing notification below exempts the process
 * from that, so the module keeps running while the user is in another app or the screen is off.
 *
 * It owns the module's lifetime (starts BleManager, stops everything in onDestroy) and listens for
 * connection/message events to keep the notification's live counts (peers connected, messages
 * received) up to date
 */
class BleMeshService : Service(), ConnectionEventListener {

    private var messagesReceived = 0

    override fun onBind(intent: Intent?): IBinder? = null   // started service, not bound

    override fun onCreate() {
        super.onCreate()
        startForeground(NOTIF_ID, buildNotification(statusText()))
        BleManager.start(applicationContext)            // idempotent; ensures the module is up
        BleTaskScheduler.registerListener(this)         // for message-received count (onMessageAssembled)
        ConnectionPool.onConnectionsChanged = { updateNotification() }  // accurate peer count, both roles
    }

    // START_STICKY: if the OS kills us under memory pressure, ask it to recreate us.
    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int = START_STICKY

    override fun onDestroy() {
        super.onDestroy()
        BleTaskScheduler.unregisterListener(this)
        ConnectionPool.onConnectionsChanged = null
        BleScanner.stop()
        BleAdvertiser.stop()
        GattServer.stop()
        BleManager.stop()
    }

    // Peer count is driven by ConnectionPool.onConnectionsChanged (set in onCreate); this listener
    // only needs the message count. onMessageAssembled fires for both GATT and L2CAP messages.
    override fun onMessageAssembled(device: BluetoothDevice, payload: ByteArray) {
        messagesReceived++
        updateNotification()
    }

    // --- Notification ---

    private fun statusText(): String {
        val peers = BleManager.connectedDevices().size
        return "$peers peer${if (peers == 1) "" else "s"} connected · $messagesReceived received"
    }

    private fun updateNotification() {
        getSystemService(NotificationManager::class.java)
            .notify(NOTIF_ID, buildNotification(statusText()))
    }

    private fun buildNotification(content: String): Notification {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            val channel = NotificationChannel(
                CHANNEL_ID, "qaul mesh", NotificationManager.IMPORTANCE_LOW
            )
            getSystemService(NotificationManager::class.java).createNotificationChannel(channel)
        }
        val tapToOpen = PendingIntent.getActivity(
            this, 0, Intent(this, MainActivity::class.java),
            PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE
        )
        return NotificationCompat.Builder(this, CHANNEL_ID)
            .setContentTitle("qaul mesh running")
            .setContentText(content)
            .setSmallIcon(android.R.drawable.stat_sys_data_bluetooth)
            .setContentIntent(tapToOpen)
            .setOngoing(true)
            .build()
    }

    companion object {
        private const val NOTIF_ID = 1
        private const val CHANNEL_ID = "ble_mesh"

        fun start(context: Context) =
            ContextCompat.startForegroundService(context, Intent(context, BleMeshService::class.java))

        fun stop(context: Context) =
            context.stopService(Intent(context, BleMeshService::class.java))
    }
}
