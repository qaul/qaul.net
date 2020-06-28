package net.qaul.app

import android.Manifest.permission.ACCESS_FINE_LOCATION
import android.annotation.SuppressLint
import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.content.IntentFilter
import android.content.pm.PackageManager.PERMISSION_DENIED
import android.content.pm.PackageManager.PERMISSION_GRANTED
import android.net.wifi.WpsInfo
import android.net.wifi.p2p.*
import android.net.wifi.p2p.WifiP2pManager.*
import android.net.wifi.p2p.nsd.WifiP2pDnsSdServiceInfo
import android.net.wifi.p2p.nsd.WifiP2pDnsSdServiceRequest
import android.os.Build.VERSION.SDK_INT
import android.os.Build.VERSION_CODES.M
import android.os.Bundle
import android.util.Log
import android.widget.Button
import android.widget.EditText
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity
import net.qaul.app.ffi.models.UserProfile
import net.qaul.app.net.WifiDirectBroadcastReceiver
import net.qaul.app.util.AppState
import java.net.InetSocketAddress
import java.net.ServerSocket
import java.net.Socket

class LoginActivity : AppCompatActivity(), ConnectionInfoListener {
    private var connected = true

    private fun kookieThings() {
        setContentView(R.layout.fragment_login)

        // Connect the TCP stack to the selected peering server
        val peerEntry = findViewById<EditText>(R.id.app_peering_server)
        val peerConnect = findViewById<Button>(R.id.peering_connect)
        peerConnect.setOnClickListener {
            val server = peerEntry.text;
            // TODO: add tcp-connect handshake here
            connected = true;
            peerConnect.text = getString(R.string.peering_button_disconnect)
            Toast.makeText(baseContext, "Connected to server...", Toast.LENGTH_LONG).show()
        }

        val login = findViewById<Button>(R.id.button_login)
        login.setOnClickListener {
            if (!connected) {
                Toast.makeText(baseContext, "Can't login, not peered!", Toast.LENGTH_LONG).show()
            } else {
                AppState.self = UserProfile("0", "@tester", "Tony Tester", false)
                Log.i("login", "Successfully logged in!")
                val i = Intent(this, MainActivity::class.java)
                startActivity(i)
            }
        }
    }

    private val manager: WifiP2pManager by lazy(LazyThreadSafetyMode.NONE) {
        // FIXME: can this fail?
        getSystemService(WIFI_P2P_SERVICE) as WifiP2pManager
    }

    private var channel: Channel? = null
    private var receiver: BroadcastReceiver? = null

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        kookieThings()


        channel = manager.initialize(this, mainLooper, null)
        channel?.also { channel ->
            receiver = WifiDirectBroadcastReceiver(manager, channel, this)
        }

        tryRequestWifiP2pPeers()
    }

    private val wifiP2pRequestCode = 1
    private val serverPort = 1602

    @SuppressLint("MissingPermission")
    fun requestWifiP2pPeers() {
        Log.d("WD", "Requesting Wi-Fi P2P peers")

        val serviceInfo = WifiP2pDnsSdServiceInfo.newInstance(
                "d${(Math.random() * 1000).toInt()}",
                "_ratman._tcp",
                mapOf())

        manager.addLocalService(channel, serviceInfo, object : ActionListener {
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
                manager.connect(channel, config, object : ActionListener {
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

        manager.addServiceRequest(channel, WifiP2pDnsSdServiceRequest.newInstance(), object : ActionListener {
            override fun onSuccess() {
                Log.d("WD", "Added service discovery request")
            }

            override fun onFailure(reason: Int) {
                Log.d("WD", "Failed adding service discovery request")
            }
        })

        manager.discoverServices(channel, object : ActionListener {
            override fun onSuccess() {
                Log.d("WD", "Started service discovery")
            }

            override fun onFailure(reason: Int) {
                Log.d("WD", "Service discovery failed")
            }
        })
    }

    private val wifiP2pPermissions = arrayOf(ACCESS_FINE_LOCATION)

    private fun wifiP2pPermitted(): Boolean {
        return SDK_INT < M ||
                wifiP2pPermissions.all { p -> checkSelfPermission(p) == PERMISSION_GRANTED }
    }

    // Lint suppressed because Wi-Fi P2P is always permitted on Android versions without
    // requestPermissions.
    @SuppressLint("NewApi")
    private fun tryRequestWifiP2pPeers() {
        when {
            wifiP2pPermitted() -> {
                requestWifiP2pPeers()
            }
            // FIXME: check shouldShowRequestPermissionRationale
            else -> {
                Log.d("WD", "Requesting permissions for Wi-Fi direct")
                requestPermissions(wifiP2pPermissions, wifiP2pRequestCode)
            }
        }
    }

    override fun onRequestPermissionsResult(requestCode: Int, permissions: Array<out String>, grantResults: IntArray) {
        super.onRequestPermissionsResult(requestCode, permissions, grantResults)

        when (requestCode) {
            wifiP2pRequestCode -> {
                if (grantResults.isEmpty() || grantResults.contains(PERMISSION_DENIED)) {
                    Log.d("WD", "Permission needed for Wi-Fi P2P rejected")
                } else {
                    requestWifiP2pPeers()
                }
            }
        }
    }

    override fun onConnectionInfoAvailable(info: WifiP2pInfo?) {
        Log.d("WD", "onConnectionInfoAvailable: $info")

        Thread {
            if (info!!.isGroupOwner) {
                Log.d("WD", "I am the group owner")
                val socket = ServerSocket(serverPort)
                socket.use {
                    val conn = socket.accept()
                    conn.getInputStream().use {
                        val line = it.bufferedReader().readLine()
                        Log.d("WD", "Received a message: '$line'")
                    }
                }
            } else {
                Log.d("WD", "Fuck ownership")
                Thread.sleep(1000)
                Socket().use {
                    it.bind(null)
                    it.connect(InetSocketAddress(info.groupOwnerAddress.hostAddress, serverPort), 5000)
                    Log.d("WD", "connected to leader")
                    it.getOutputStream().use { out ->
                        val buf = out.bufferedWriter()
                        buf.write("Hello world!\n")
                        buf.flush()
                        Log.d("WD", "wrote a message")
                    }
                }
                Log.d("WD", "Connection closed")
            }
        }.start()
    }

    override fun onResume() {
        super.onResume()

        val intentFilter = IntentFilter().apply {
            addAction(WIFI_P2P_STATE_CHANGED_ACTION)
            addAction(WIFI_P2P_PEERS_CHANGED_ACTION)
            addAction(WIFI_P2P_CONNECTION_CHANGED_ACTION)
            addAction(WIFI_P2P_THIS_DEVICE_CHANGED_ACTION)
        }

        receiver?.also { receiver ->
            registerReceiver(receiver, intentFilter)
        }

        manager.discoverServices(channel, object : ActionListener {
            override fun onSuccess() {
                Log.d("WD", "Service discovery initiated")
            }

            override fun onFailure(reason: Int) {
                Log.d("WD", "Failed to discover services: $reason")
            }
        })
    }

    override fun onPause() {
        super.onPause()
        receiver?.also { receiver ->
            unregisterReceiver(receiver)
        }
    }
}
