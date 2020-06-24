package net.qaul.app

import android.content.Intent
import android.os.Bundle
import android.util.Log
import android.widget.Button
import android.widget.EditText
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity

class LoginActivity : AppCompatActivity() {
    var connected = true

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
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
                Log.i("login", "Successfully logged in!")
                val i = Intent(this, MainActivity::class.java)
                startActivity(i)
            }
        }
    }
}
