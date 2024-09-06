package net.qaul.qaul_app

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent
import android.app.Service
import android.content.Context
import android.content.Intent
import android.content.pm.ServiceInfo.FOREGROUND_SERVICE_TYPE_CONNECTED_DEVICE
import android.content.pm.ServiceInfo.FOREGROUND_SERVICE_TYPE_REMOTE_MESSAGING
import android.graphics.Color
import android.net.wifi.WifiManager
import android.os.Build
import android.os.IBinder
import android.os.PowerManager
import androidx.annotation.Nullable
import androidx.core.app.NotificationCompat
import androidx.core.app.ServiceCompat
import io.flutter.embedding.android.FlutterActivity
import io.flutter.embedding.engine.FlutterEngine
import io.flutter.embedding.engine.FlutterEngineCache
import io.flutter.plugins.GeneratedPluginRegistrant

class FlutterBackgroundService : Service() {
    companion object {
        @JvmStatic
        val WIFILOCK_TAG = "FlutterBackgroundService:WifiLock"

        @JvmStatic
        val WAKELOCK_TAG = "FlutterBackgroundService:WakeLock"

        @JvmStatic
        val CHANNEL_ID = "qaul_background"

        @JvmStatic
        val NOTIFICATION_ID = 5005

        @JvmStatic
        val CHANNEL_NAME = "qaul.net Channel"

        @JvmStatic
        val CHANNEL_DESCRIPTION = "This channel is used for the qaul.net background service to inform the user that the app is still running in the background"

        @JvmStatic
        val NOTIFICATION_TITLE = "qaul.net"

        @JvmStatic
        val NOTIFICATION_DESCRIPTION = "The app is running in the Background"
    }

    private var wifiLock: WifiManager.WifiLock? = null
    private var wakeLock: PowerManager.WakeLock? = null

    override fun onBind(intent: Intent?): IBinder? {
        return null
    }

    override fun onCreate() {
        super.onCreate()
        checkAndRequestPermissions()
    }

    override fun onDestroy() {
        stopService()
        super.onDestroy()
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        return START_STICKY
    }

    override fun onTaskRemoved(rootIntent: Intent) {
        super.onTaskRemoved(rootIntent)
        stopService()
        stopSelf()
    }

    private fun checkAndRequestPermissions() {
        val permissionHandler = MainActivity.permissionHandler
        permissionHandler.checkAndRequestPermissions { permissionsGranted ->
            if (permissionsGranted) {
                startService()
            }
        }
    }

    private fun startService() {
        acquireWifiLock()
        acquireWakeLock()
        val channelId = createNotificationChannel(CHANNEL_ID, CHANNEL_NAME)
        val notification = createNotification(channelId)
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) {
            // For API level 29 and higher, include FOREGROUND_SERVICE_TYPE_CONNECTED_DEVICE
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.UPSIDE_DOWN_CAKE) {
                // For API level 34 and higher, include both service types
                ServiceCompat.startForeground(
                    this,
                    NOTIFICATION_ID,
                    notification,
                    FOREGROUND_SERVICE_TYPE_CONNECTED_DEVICE or FOREGROUND_SERVICE_TYPE_REMOTE_MESSAGING
                )
            } else {
                // For API level 29 to 33, only include CONNECTED_DEVICE
                ServiceCompat.startForeground(
                    this,
                    NOTIFICATION_ID,
                    notification,
                    FOREGROUND_SERVICE_TYPE_CONNECTED_DEVICE
                )
            }
        } else {
            // For API levels lower than 29, call startForeground without specifying service types
            startForeground(NOTIFICATION_ID, notification)
        }
    }

    private fun stopService() {
        wifiLock?.release()
        wakeLock?.release()
        stopForeground(true)
        stopSelf()
    }

    private fun createNotificationChannel(channelId: String, channelName: String): String {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            val serviceChannel = NotificationChannel(
                channelId,
                channelName,
                NotificationManager.IMPORTANCE_DEFAULT
            ).apply {
                description = CHANNEL_DESCRIPTION
            }
            serviceChannel.lightColor = Color.GREEN
            val manager = getSystemService(NotificationManager::class.java)
            manager.createNotificationChannel(serviceChannel)
        }
        return channelId
    }

    private fun createNotification(channelId: String): Notification {
        val imageId = resources.getIdentifier("ic_notification", "drawable", packageName)
        val notificationIntent = Intent(this, MainActivity::class.java)
        val pendingIntent: PendingIntent
        pendingIntent = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.M) {
            // Introduced in Android 12; see: https://developer.android.com/about/versions/12/behavior-changes-12#pending-intent-mutability
            PendingIntent.getActivity(this,
                    0, notificationIntent, PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE)
        } else {
            PendingIntent.getActivity(this,
                    0, notificationIntent, PendingIntent.FLAG_UPDATE_CURRENT)
        }
        return NotificationCompat.Builder(this, channelId)
            .setContentTitle(NOTIFICATION_TITLE)
            .setContentText(NOTIFICATION_DESCRIPTION)
            .setSmallIcon(imageId)
            .setContentIntent(pendingIntent)
            .setOngoing(true)
            .setAutoCancel(true)
            .build()
    }

    private fun acquireWifiLock() {
        val wifiManager = applicationContext.getSystemService(Context.WIFI_SERVICE) as WifiManager
        wifiLock = wifiManager.createWifiLock(WifiManager.WIFI_MODE_FULL_HIGH_PERF, WIFILOCK_TAG)
        wifiLock?.acquire()
    }

    private fun acquireWakeLock() {
        val powerManager = getSystemService(Context.POWER_SERVICE) as PowerManager
        wakeLock = powerManager.newWakeLock(PowerManager.PARTIAL_WAKE_LOCK, WAKELOCK_TAG)
        wakeLock?.acquire()
    }
}
