package net.qaul.app.net

import android.annotation.SuppressLint
import android.app.Service
import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.net.wifi.WpsInfo
import android.net.wifi.p2p.WifiP2pConfig
import android.net.wifi.p2p.WifiP2pInfo
import android.net.wifi.p2p.WifiP2pManager
import android.net.wifi.p2p.nsd.WifiP2pDnsSdServiceInfo
import android.net.wifi.p2p.nsd.WifiP2pDnsSdServiceRequest
import android.os.IBinder
import android.util.Log
import java.lang.Exception
import java.net.InetSocketAddress
import java.net.NoRouteToHostException
import java.net.ServerSocket
import java.net.Socket


class WifiP2PService : Service(), WifiP2pManager.ConnectionInfoListener {
    private val manager: WifiP2pManager by lazy(LazyThreadSafetyMode.NONE) {
        getSystemService(Context.WIFI_P2P_SERVICE) as WifiP2pManager
    }
    private lateinit var channel: WifiP2pManager.Channel
    private lateinit var receiver: BroadcastReceiver

    private val serverPort = 1602;
    private val peers: MutableMap<Int, PeerHandler> = mutableMapOf()

    override fun onBind(intent: Intent?): IBinder? {
        return null
    }

    override fun onCreate() {
        channel = manager.initialize(this, mainLooper, null)
        channel.also { c ->
            receiver = WifiDirectBroadcastReceiver(manager, c, this)
        }

        // Setup the peer discovery
        requestWifiP2pPeers()

        // Then, in a loop discover peers
        Thread {
            while (true) {
                discoverPeers()
                Thread.sleep(1000)
            }
        }.start()
    }

    // Ignore missing permissions because we check it when the app starts
    @SuppressLint("MissingPermission")
    fun requestWifiP2pPeers() {
        Log.d("WD", "Requesting Wi-Fi P2P peers")

        val serviceInfo = WifiP2pDnsSdServiceInfo.newInstance(
                "d${(Math.random() * 1000).toInt()}",
                "_ratman._tcp",
                mapOf())

        // Add a local "ratman" service
        manager.addLocalService(channel, serviceInfo, object : WifiP2pManager.ActionListener {
            override fun onSuccess() {
                Log.d("WD", "local service added successfully")
            }

            override fun onFailure(reason: Int) {
                Log.d("WD", "Failed to add local service: $reason")
            }
        })

        manager.setDnsSdResponseListeners(channel, { instanceName, registrationType, srcDevice ->
            Log.d("WD", "Found service $instanceName $registrationType $srcDevice")
            Log.d("WD", "can connect? " + (registrationType == "_ratman._tcp.local." && srcDevice != null).toString())
            if (registrationType == "_ratman._tcp.local." && srcDevice != null) {
                val config = WifiP2pConfig()
                config.deviceAddress = srcDevice.deviceAddress
                config.wps.setup = WpsInfo.PBC
                manager.connect(channel, config, object : WifiP2pManager.ActionListener {
                    override fun onSuccess() {
                        Log.d("WD", "Connecting to service")
                    }

                    override fun onFailure(reason: Int) {
                        Log.d("WD", "Failed to connect to service")
                    }
                })
            }
        }, { fullDomainName, txtRecordMap, srcDevice ->
            Log.d("WD", "DNS SD TXT record available: $fullDomainName $txtRecordMap $srcDevice")
        })

        manager.addServiceRequest(channel, WifiP2pDnsSdServiceRequest.newInstance(), object : WifiP2pManager.ActionListener {
            override fun onSuccess() {
                Log.d("WD", "Added service discovery request")
            }

            override fun onFailure(reason: Int) {
                Log.d("WD", "Failed adding service discovery request")
            }
        })
    }

    fun discoverPeers() = manager.discoverServices(channel, object : WifiP2pManager.ActionListener {
        override fun onSuccess() {
            Log.d("WD", "Started service discovery")
        }

        override fun onFailure(reason: Int) {
            Log.d("WD", "Service discovery failed")
        }
    })

    // This function will be called
    override fun onConnectionInfoAvailable(info: WifiP2pInfo?) {
        Log.d("WD", "onConnectionInfoAvailable: $info")

        Thread {
            if (info!!.isGroupOwner) {
                Log.d("WD", "I am the group leader")
                val socket = ServerSocket(serverPort)

                while (true) {
                    try {
                        socket.use {
                            val conn = socket.accept()
                            conn.getInputStream().use {
                                val line = it.bufferedReader().readLine()
                                Log.d("WD", "Received a message: '$line'")
                            }
                        }
                    } catch (e: NoRouteToHostException) {
                        Log.e("WD", "Client has gone away, so we can stop trying...")
                        break
                    } catch (e: Exception) {
                        Log.e("WD", "Error occured during session accept: $e")
                        continue
                    }
                }
            } else {
                Log.d("WD", "I am a group follower")
                Socket().use {
                    it.bind(null)

                    for (x in 0..10) {
                        try {
                            it.connect(InetSocketAddress(info.groupOwnerAddress.hostAddress, serverPort), 5000)

                            Log.d("WD", "connected to leader")
                            it.getOutputStream().use { out ->
                                val buf = out.bufferedWriter()
                                buf.write("Hello world!\n")
                                buf.flush()
                                Log.d("WD", "wrote a message")
                            }

                        } catch (e: NoRouteToHostException) {
                            Log.w("WD", "Device has gone away, so we can stop trying")
                            break
                        } catch (e: Exception) {
                            Log.i("WD", "Retrying connection in 100ms")
                            Thread.sleep(100)
                            continue
                        }
                    }
                }
                Log.d("WD", "Connection closed")
            }
        }.start()
    }
}