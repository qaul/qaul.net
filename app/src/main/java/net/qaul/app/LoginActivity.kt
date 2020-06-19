package net.qaul.app

import android.content.Intent
import android.os.Bundle
import android.util.Log
import android.widget.Button
import androidx.appcompat.app.AppCompatActivity
import net.qaul.app.net.WDService

class LoginActivity : AppCompatActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.fragment_login)

        // Start our Wifi Direct service
        startService(Intent(this, WDService::class.java))

        val butt = findViewById<Button>(R.id.button_login)
        butt.setOnClickListener {
            Log.i("login", "Successfully logged in!")
            val i = Intent(this, MainActivity::class.java)
            startActivity(i)
        }
    }
}
