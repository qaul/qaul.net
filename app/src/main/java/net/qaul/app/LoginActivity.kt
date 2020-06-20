package net.qaul.app

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.content.IntentFilter
import android.content.pm.PackageManager
import android.net.ConnectivityManager
import android.net.Network
import android.net.NetworkCapabilities
import android.net.NetworkRequest
import android.net.wifi.aware.*
import android.net.wifi.p2p.WifiP2pDevice
import android.net.wifi.p2p.WifiP2pDeviceList
import android.os.Build
import android.os.Bundle
import android.os.Handler
import android.os.Looper
import android.util.Log
import android.widget.Button
import android.widget.TextView
import androidx.annotation.RequiresApi
import androidx.appcompat.app.AppCompatActivity
import com.wifiscanner.listener.WifiP2PConnectionCallback
import com.wifiscanner.service.WifiP2PService
import com.wifiscanner.service.WifiP2PServiceImpl
import net.qaul.app.net.WDService
import java.net.ServerSocket
import java.net.Socket

class LoginActivity : AppCompatActivity() {

    lateinit var service: WifiP2PServiceImpl

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_debug)

        // Start our Wifi Direct service
        startService(Intent(this, WDService::class.java))

        // Check if we can do Wifi Aware
        val canDoWA = applicationContext.packageManager.hasSystemFeature(PackageManager.FEATURE_WIFI_AWARE)
        Log.i("wifiaware", "Can we do wifi aware? ... " + if (canDoWA) "YES" else "No...")

        if (canDoWA) {
            val wMan = applicationContext.getSystemService(Context.WIFI_AWARE_SERVICE) as WifiAwareManager?
            val filter = IntentFilter(WifiAwareManager.ACTION_WIFI_AWARE_STATE_CHANGED)

            val recv = object : BroadcastReceiver() {
                override fun onReceive(context: Context?, intent: Intent?) {
                    if (wMan!!.isAvailable) {
                        Log.i("wifiaware", "WE CAN DO WIFI AWARE!")
                    } else {
                        Log.w("wifiaware", "WiFi Aware is not available (anymore)!")
                    }
                }
            }

            applicationContext.registerReceiver(recv, filter)

            // Create a WiFi Aware Session
            val callback = object : AttachCallback() {
                override fun onAttached(session: WifiAwareSession?) {
                    super.onAttached(session)
                    Log.v("wifiaware", "Attach successful!")

                    // TODO: Publish a service here
                    val cfg: PublishConfig = PublishConfig.Builder()
                            .setServiceName("net.qaul.qauldroid")
                            .build()

                    // Publish a service to be discovered
                    session?.publish(cfg, object : DiscoverySessionCallback() {
                        lateinit var session: PublishDiscoverySession

                        override fun onPublishStarted(session: PublishDiscoverySession) {
                            super.onPublishStarted(session)
                            this.session = session
                        }

                        @RequiresApi(Build.VERSION_CODES.Q)
                        override fun onMessageReceived(peerHandle: PeerHandle?, message: ByteArray?) {
                            super.onMessageReceived(peerHandle, message)
                            Log.i("wifiaware", "Received a connection by a subscriber: " + message)

                            // TODO: maybe check the message payload?! Naaaah, who needs that
                            val sock = ServerSocket(0)
                            val port = sock.localPort

                            val netSpec = WifiAwareNetworkSpecifier.Builder(this.session, peerHandle!!)
                                    .setPskPassphrase("qaul-net-is-cool")
                                    .setPort(port)
                                    .build()

                            val netReq = NetworkRequest.Builder()
                                    .addTransportType(NetworkCapabilities.TRANSPORT_WIFI_AWARE)
                                    .setNetworkSpecifier(netSpec)
                                    .build()

                            val callback = object : ConnectivityManager.NetworkCallback() {
                                override fun onAvailable(network: Network) {
                                    super.onAvailable(network)
                                    Log.i("wifiaware", "Network has become available!")
                                }

                                override fun onLost(network: Network) {
                                    super.onLost(network)
                                    Log.i("wifiaware", "Connection was lost :(")
                                }
                            }

                            val conMan = applicationContext.getSystemService(Context.CONNECTIVITY_SERVICE) as ConnectivityManager?
                            conMan?.requestNetwork(netReq, callback)

                            // pub.sendMessage(peerHandle, 1312, "hello qaul".toByteArray())
                        }
                    }, null)

                    // Each publisher is also a subscriber, so we create the subscriber next
                    val subCfg = SubscribeConfig.Builder()
                            .setServiceName("qaul.net.qauldroid")
                            .build()

                    session?.subscribe(subCfg, object : DiscoverySessionCallback() {
                        lateinit var session: SubscribeDiscoverySession;

                        override fun onSubscribeStarted(session: SubscribeDiscoverySession) {
                            super.onSubscribeStarted(session)
                            Log.i("wifiaware", "Started looking for publishers")
                            this.session = session
                        }

                        @RequiresApi(Build.VERSION_CODES.Q)
                        override fun onServiceDiscovered(peerHandle: PeerHandle?,
                                                         serviceSpecificInfo: ByteArray?,
                                                         matchFilter: MutableList<ByteArray>?) {
                            super.onServiceDiscovered(peerHandle, serviceSpecificInfo, matchFilter)
                            Log.i("wifiaware", "Found other qaul.net service")
                            this.session.sendMessage(peerHandle!!, 1312, "hello!".toByteArray())

                            // Then we probably want to wait for a network creation request?
                            val netSpec = WifiAwareNetworkSpecifier.Builder(this.session, peerHandle)
                                    .setPskPassphrase("qaul-net-is-cool")
                                    .build()

                            val netReq = NetworkRequest.Builder()
                                    .addTransportType(NetworkCapabilities.TRANSPORT_WIFI_AWARE)
                                    .setNetworkSpecifier(netSpec)
                                    .build()

                            val callback = object : ConnectivityManager.NetworkCallback() {
                                lateinit var network: Network
                                lateinit var socket: Socket

                                override fun onAvailable(network: Network) {
                                    super.onAvailable(network)
                                    Log.i("wifiaware", "Network became available ?!")
                                    this.network = network
                                }

                                override fun onCapabilitiesChanged(network: Network,
                                                                   netCaps: NetworkCapabilities) {
                                    super.onCapabilitiesChanged(network, netCaps)
                                    Log.i("wifiaware", "Capabilities changed")

                                    val peerInfo = netCaps.transportInfo as WifiAwareNetworkInfo
                                    val peerIpv6 = peerInfo.peerIpv6Addr
                                    val peerPort = peerInfo.port

                                    this.socket = network.socketFactory.createSocket(peerIpv6, peerPort)
                                }
                            }

                            val conMan = applicationContext.getSystemService(Context.CONNECTIVITY_SERVICE) as ConnectivityManager?
                            conMan?.requestNetwork(netReq, callback)
                        }
                    }, null)
                }

                override fun onAttachFailed() {
                    Log.v("wifiaware", "Failed to attach!")
                    super.onAttachFailed()
                }
            }

            val handler = Handler(Looper.getMainLooper())
            wMan?.attach(callback, handler)
        }


        // Debug printer
        val text = findViewById<TextView>(R.id.debugText)
        text.text = ""

        val appendLog = { s: String -> text.append(s + "\n") }
        appendLog("--- Welcome to the qaul.net automated test ---\n")

        val callback = object : WifiP2PConnectionCallback {
            override fun onDataReceiving() {
                appendLog("Receivig data")
            }

            override fun onInitiateDiscovery() {
                appendLog("Initiated discovery")
            }

            override fun onDataReceivedSuccess(p0: String?) {
                appendLog("Data received: " + p0)
            }

            override fun onPeerConnectionFailure() {
                appendLog("Connection failed")
            }

            override fun onPeerDisconnectionFailure() {
                appendLog("Peer failed to connect")
            }

            override fun onPeerAvailable(p0: WifiP2pDeviceList?) {
                appendLog("Peer now available: " + p0)
            }

            override fun onDataTransferredSuccess() {
                appendLog("Data transfer success")
            }

            override fun onDiscoverySuccess() {
                appendLog("Discovery success")
            }

            override fun onDiscoveryFailure() {
                appendLog("Discovery failed")
            }

            override fun onPeerConnectionSuccess() {
                appendLog("Peer connection success")
            }

            override fun onDataTransferring() {
                appendLog("Data transfering...")
            }

            override fun onDataReceivedFailure() {
                appendLog("Data receive failed")
            }

            override fun onPeerDisconnectionSuccess() {
                appendLog("Peer disconnected!")
            }

            override fun onDataTransferredFailure() {
                appendLog("Data transfer failed")
            }

            override fun onPeerStatusChanged(p0: WifiP2pDevice?) {
                appendLog("Peer status changed: " + p0)
            }
        }

        this.service = WifiP2PServiceImpl.Builder()
                .setSender(this)
                .setWifiP2PConnectionCallback(callback)
                .build()
        service.onCreate()
    }
}
