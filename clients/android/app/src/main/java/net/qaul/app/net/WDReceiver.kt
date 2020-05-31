package net.qaul.app.net

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.net.wifi.p2p.WifiP2pDevice
import android.net.wifi.p2p.WifiP2pManager
import android.util.Log
import android.widget.Toast

// TODO: pause the receiver on activity pause
class WDReceiver(val context: Context,
                 val manager: WifiP2pManager,
                 val channel: WifiP2pManager.Channel)
    : BroadcastReceiver() {
    private val peers = mutableListOf<WifiP2pDevice>()

    private val peerListListener = WifiP2pManager.PeerListListener { peerList ->
        val refreshedPeers = peerList.deviceList

        // When we have a different set of peers
        if (refreshedPeers != peers) {
            peers.clear()
            peers.addAll(refreshedPeers)

            Log.d("WD", peers.toString())

            // TODO: no we notify anyone else? Like the Receiver?
            // TODO: handshake to figure out if this is qaul.net?
        }

        if (peers.isEmpty()) {
            Toast.makeText(context, "No more peers around to connect to...", Toast.LENGTH_LONG).show()
        }
    }

    override fun onReceive(context: Context, intent: Intent) {
        when (intent.action) {
            WifiP2pManager.WIFI_P2P_STATE_CHANGED_ACTION -> {
                val state = intent.getIntExtra(WifiP2pManager.EXTRA_WIFI_STATE, -1)
                manager.requestPeers(channel, peerListListener)
                Log.d("WD", "P2P state changed: " + state)
            }
            WifiP2pManager.WIFI_P2P_PEERS_CHANGED_ACTION -> {
                Log.i("WD", "Peers changed!: " + peers)
            }
            WifiP2pManager.WIFI_P2P_CONNECTION_CHANGED_ACTION -> {
                Log.i("WD", "P2p connection changed!")
            }
            WifiP2pManager.WIFI_P2P_THIS_DEVICE_CHANGED_ACTION -> {
                Log.i("WD", "This device changed!")
            }
        }
    }
}