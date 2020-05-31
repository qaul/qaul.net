package net.qaul.app.net

import android.app.Service
import android.content.Context
import android.content.Intent
import android.content.IntentFilter
import android.net.wifi.p2p.WifiP2pDevice
import android.net.wifi.p2p.WifiP2pManager
import android.os.IBinder
import android.util.Log
import android.widget.Toast

/** A handler for wifi direct messages and state */
class WDService : Service() {
    private val intentFilter = IntentFilter()
    private lateinit var channel: WifiP2pManager.Channel
    private lateinit var manager: WifiP2pManager

    override fun onBind(intent: Intent?): IBinder? {
        return null
    }

    override fun onCreate() {
        super.onCreate()

        intentFilter.addAction(WifiP2pManager.WIFI_P2P_STATE_CHANGED_ACTION)
        intentFilter.addAction(WifiP2pManager.WIFI_P2P_PEERS_CHANGED_ACTION)
        intentFilter.addAction(WifiP2pManager.WIFI_P2P_CONNECTION_CHANGED_ACTION)
        intentFilter.addAction(WifiP2pManager.WIFI_P2P_THIS_DEVICE_CHANGED_ACTION)

        // Create the manager and channel
        manager = getSystemService(Context.WIFI_P2P_SERVICE) as WifiP2pManager
        channel = manager.initialize(this, mainLooper, null)

        // Register the broadcast receiver
        val receiver = WDReceiver(applicationContext, manager, channel)
        applicationContext.registerReceiver(receiver, intentFilter)

        // Start looking for peers
        manager.discoverPeers(channel, object : WifiP2pManager.ActionListener {
            override fun onSuccess() {
                Log.i("WD", "onSuccess() of discoverPeers")

            }

            override fun onFailure(reason: Int) {
                // if 2 == turn on wifi here

                Log.i("WD", "onFailure(`" + reason + "`) of discoverPeers")
            }
        })
    }

    val channelListener = WifiP2pManager.ChannelListener {

    }
}
