package net.qaul.app.net

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.net.NetworkInfo
import android.net.wifi.WpsInfo
import android.net.wifi.p2p.WifiP2pConfig
import android.net.wifi.p2p.WifiP2pDevice
import android.net.wifi.p2p.WifiP2pManager
import android.util.Log
import android.widget.Toast

// TODO: pause the receiver on activity pause
class WDReceiver(val serv: WDService,
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

            // FIXME: get a change-set of devices

            peers.map { WifiP2pConfig().apply {
                deviceAddress = it.deviceAddress
                wps.setup = WpsInfo.PBC
            } }.forEach {
                serv.connect(it) // Then connect!
            }

            Log.d("WD", peers.toString())

            // TODO: do we notify anyone else? Like the Receiver?
            // TODO: handshake to figure out if this is qaul.net?
        }

        if (peers.isEmpty()) {
            Toast.makeText(serv.applicationContext,
                    "No more peers around to connect to...",
                    Toast.LENGTH_LONG).show()
        }
    }

    private val connectionListener = WifiP2pManager.ConnectionInfoListener { info ->
        val groupAddr: String = info.groupOwnerAddress.hostAddress

        if (info.groupFormed && info.isGroupOwner) {
            Log.i("WD", "Peer is group owner and a new group is formed!: " + groupAddr)
        } else if (info.groupFormed) {
            Log.i("WD", "A new group  is formed:" + groupAddr)
        } else {
            Log.i("WD", "Not part of a group? " + groupAddr)
        }
    }

    override fun onReceive(context: Context, intent: Intent) {
        when (intent.action) {
            WifiP2pManager.WIFI_P2P_STATE_CHANGED_ACTION -> {
                val state = intent.getIntExtra(WifiP2pManager.EXTRA_WIFI_STATE, -1)
                manager.requestPeers(channel, peerListListener)
                Log.d("WD", "P2P state changed: " + state)
                if (state == WifiP2pManager.WIFI_P2P_STATE_ENABLED)
                    serv.setState(true)
                else if (state == WifiP2pManager.WIFI_P2P_STATE_DISABLED)
                    serv.setState(false)
            }
            WifiP2pManager.WIFI_P2P_PEERS_CHANGED_ACTION -> {
                Log.i("WD", "Peers changed!: " + peers)
                manager.requestPeers(channel, peerListListener)
            }
            WifiP2pManager.WIFI_P2P_CONNECTION_CHANGED_ACTION -> {
                Log.i("WD", "P2p connection changed!")

                manager.let { manager ->
                    val info = intent.getParcelableExtra<NetworkInfo>(WifiP2pManager.EXTRA_NETWORK_INFO)

                    if (info?.isConnected!!) {
                        manager.requestConnectionInfo(channel, connectionListener)
                    }

                }

                // FIXME: replace with a NetworkCallback
                val net: NetworkInfo = intent.getParcelableExtra()!!

                if (net.isConnected()) {
                    Log.i("WD", "Connected to a peer!")
                } else {
                    Log.i("WD", "Disconnected from a peer")
                }
            }
            WifiP2pManager.WIFI_P2P_THIS_DEVICE_CHANGED_ACTION -> {
                Log.i("WD", "This device changed?!")
            }
        }
    }
}