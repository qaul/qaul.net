package net.qaul.app.net

import android.net.wifi.p2p.WifiP2pInfo
import android.util.Log
import java.net.NoRouteToHostException
import java.net.ServerSocket
import java.net.Socket

class PeerHandler(val id: Int, val server: ServerSocket, val client: Socket) {

    fun start(info: WifiP2pInfo) {
        while (true) {
            try {


            } catch (e: NoRouteToHostException) {
                Log.e("WD", "Device has gone away...!");
                break
            }
        }
    }
}