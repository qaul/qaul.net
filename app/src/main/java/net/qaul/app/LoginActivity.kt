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
import android.os.Build
import android.os.Bundle
import android.os.Handler
import android.os.Looper
import android.util.Log
import android.widget.Button
import androidx.annotation.RequiresApi
import androidx.appcompat.app.AppCompatActivity
import net.qaul.app.net.WDService
import java.net.ServerSocket
import java.net.Socket

class LoginActivity : AppCompatActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.fragment_login)

        // Start our Wifi Direct service
        startService(Intent(this, WDService::class.java))

        // Check if we can do Wifi Aware
        if (applicationContext.packageManager.hasSystemFeature(PackageManager.FEATURE_WIFI_AWARE)) {
            val wMan = applicationContext.getSystemService(Context.WIFI_AWARE_SERVICE) as WifiAwareManager?
            val filter = IntentFilter(WifiAwareManager.ACTION_WIFI_AWARE_STATE_CHANGED)

            val recv = object : BroadcastReceiver() {
                override fun onReceive(context: Context?, intent: Intent?) {
                    if (wMan!!.isAvailable) {
                        Log.i("login", "WE CAN DO WIFI AWARE!")
                    } else {
                        Log.w("login", "WiFi Aware is not available (anymore)!")
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




        val butt = findViewById<Button>(R.id.button_login)
        butt.setOnClickListener {
            Log.i("login", "Successfully logged in!")
            val i = Intent(this, MainActivity::class.java)
            startActivity(i)
        }
    }
}
