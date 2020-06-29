package net.qaul.app

import android.content.Intent
import android.os.Bundle
import android.util.Log
import android.widget.Button
import android.widget.EditText
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity
import net.qaul.app.ffi.models.Id
import net.qaul.app.ffi.models.UserProfile
import net.qaul.app.net.WifiP2PService
import net.qaul.app.util.AppState


/** The main login activity */
class LoginActivity : AppCompatActivity() {
    var tcpConnected = false

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.fragment_login)

        // TODO: Request permissions

        // Start the wifi service
        startService(Intent(this, WifiP2PService::class.java))

        // Connect the TCP stack to the selected peering server
        val peerEntry = findViewById<EditText>(R.id.app_peering_server)
        val peerConnect = findViewById<Button>(R.id.peering_connect)
        peerConnect.setOnClickListener {
            val server = peerEntry.text;
            // TODO: add tcp-connect handshake here
            tcpConnected = true;
            peerConnect.text = getString(R.string.peering_button_disconnect)
            Toast.makeText(baseContext, "Connected to server...", Toast.LENGTH_LONG).show()
        }

        val login = findViewById<Button>(R.id.button_login)
        login.setOnClickListener {
            if (!tcpConnected) {
                Toast.makeText(baseContext, "Can't login, not peered!", Toast.LENGTH_LONG).show()
            } else {
                AppState.self = UserProfile(Id("0"), "@tester", "Tony Tester", false)
                Log.i("login", "Successfully logged in!")
                val i = Intent(this, MainActivity::class.java)
                startActivity(i)
            }
        }

    }
}
