package net.qaul.app.net

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.net.NetworkInfo
import android.net.wifi.p2p.*
import android.net.wifi.p2p.WifiP2pManager.*
import android.util.Log
import net.qaul.app.LoginActivity

/** A broadcast receiver that reacts to WifiP2P events */
class WifiDirectBroadcastReceiver(
        private val manager: WifiP2pManager,
        private val channel: Channel,
        private val service: WifiP2PService
) : BroadcastReceiver() {
    override fun onReceive(context: Context, intent: Intent) {
        when (intent.action) {
            WIFI_P2P_STATE_CHANGED_ACTION -> {
                Log.d("WD", "WIFI_P2P_STATE_CHANGED_ACTION")
                val state = intent.getIntExtra(WifiP2pManager.EXTRA_WIFI_STATE, -1)
                when (state) {
                    WIFI_P2P_STATE_ENABLED -> {
                        Log.d("WD", "  WIFI_P2P_STATE_ENABLED")
                        // Wi-Fi P2P is enabled
                    }
                    else -> {
                        Log.d("WD", "other state: " + state)
                        // Wi-FI P2P is not enabled
                    }
                }
                // Check to see if Wi-Fi is enabled and notify appropriate activity.
            }
            WIFI_P2P_PEERS_CHANGED_ACTION -> {
                Log.d("WD", "WIFI_P2P_PEERS_CHANGED_ACTION")
            }
            WIFI_P2P_CONNECTION_CHANGED_ACTION -> {
                Log.d("WD", "WIFI_P2P_CONNECTION_CHANGED_ACTION")
                val wifiP2pInfo = intent.getParcelableExtra<WifiP2pInfo>(EXTRA_WIFI_P2P_INFO)
                val networkInfo = intent.getParcelableExtra<NetworkInfo>(EXTRA_NETWORK_INFO)
                val wifiP2pGroup = intent.getParcelableExtra<WifiP2pGroup>(EXTRA_WIFI_P2P_GROUP)
                Log.d("WD", wifiP2pInfo.toString())
                Log.d("WD", networkInfo.toString())
                Log.d("WD", wifiP2pGroup.toString())

                if (networkInfo.isConnected) {
                    Log.d("WD", "Connection is up!")
                    manager.requestConnectionInfo(channel, service)
                } else {
                    Log.d("WD", "Connection is down")
                }
            }
            WIFI_P2P_THIS_DEVICE_CHANGED_ACTION -> {
                Log.d("WD", "WIFI_P2P_THIS_DEVICE_CHANGED_ACTION")
                val device = intent.getParcelableExtra<WifiP2pDevice>(EXTRA_WIFI_P2P_DEVICE)
                Log.d("WD", device.toString())
            }
            else -> {
                Log.d("WD", "Unknown action")
            }
        }
    }
}
