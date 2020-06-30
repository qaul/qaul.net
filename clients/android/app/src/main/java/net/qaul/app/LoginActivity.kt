package net.qaul.app

import android.R.layout.simple_spinner_dropdown_item
import android.R.layout.simple_spinner_item

import android.content.Intent
import android.os.Bundle
import android.util.Log
import android.widget.*
import androidx.appcompat.app.AppCompatActivity
import androidx.fragment.app.FragmentTransaction
import net.qaul.app.ffi.NativeQaul
import net.qaul.app.ffi.models.Id
import net.qaul.app.ffi.models.UserProfile
import net.qaul.app.net.WifiP2PService
import net.qaul.app.ui.UserCreateFragment
import net.qaul.app.util.AppState
import android.widget.ArrayAdapter as ArrayAdapter


/** The main login activity */
class LoginActivity : AppCompatActivity() {
    companion object {
        init {
            // The "android-support" crate creates a dynamic library called "libqauldroid"
            // which we can include here simply via "qauldroid" because it's being put
            // into the library search path via ~ m a g i c ~
            System.loadLibrary("qauldroid")
        }
    }

    val LOG_TAG = "login"

    var tcpConnected = false
    val localUsers: MutableList<UserProfile> = mutableListOf()
    lateinit var spinner: Spinner

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.fragment_login)

        // TODO: Request permissions

        // Start the wifi service
        startService(Intent(this, WifiP2PService::class.java))

        // Handle the register screen
        val register = findViewById<Button>(R.id.button_register)
        register.setOnClickListener {
            Log.d(LOG_TAG, "Pressing the register button!")
            val man = supportFragmentManager
            val trans = man.beginTransaction()
            trans.replace(R.id.login_container_layout, UserCreateFragment(this)).addToBackStack(null)
            trans.setTransition(FragmentTransaction.TRANSIT_FRAGMENT_OPEN)
            trans.commit()
        }

        // Update users once
        spinner = findViewById(R.id.user_list_picker)
        updateUsers()

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

//        val login = findViewById<Button>(R.id.button_login)
//        login.setOnClickListener {
//            if (!tcpConnected) {
//                Toast.makeText(baseContext, "Can't login, not peered!", Toast.LENGTH_LONG).show()
//            } else {
//                AppState.self = UserProfile(Id("0"), "@tester", "Tony Tester", false)
//                Log.i(LOG_TAG, "Successfully logged in!")
//                val i = Intent(this, MainActivity::class.java)
//                startActivity(i)
//            }
//        }
    }

    fun makeAdapter(): ArrayAdapter<UserProfile> {
        val aa = ArrayAdapter(this, simple_spinner_item, localUsers)
        aa.setDropDownViewResource(simple_spinner_dropdown_item)
        return aa
    }

    fun updateUsers() {
        localUsers.clear()
        val users = AppState.get().usersList(true)
        for (u in users) { localUsers.add(u) }
        spinner.adapter = makeAdapter()
    }
}
