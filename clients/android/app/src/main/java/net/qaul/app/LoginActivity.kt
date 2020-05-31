package net.qaul.app

import android.R.layout.simple_spinner_dropdown_item
import android.R.layout.simple_spinner_item
import android.content.Intent
import android.os.Bundle
import android.util.Log
import android.widget.*
import androidx.appcompat.app.AppCompatActivity
import net.qaul.app.ffi.NativeQaul
import net.qaul.app.ffi.NativeState
import java.io.*
import java.lang.annotation.Native

/**
 * The first activity to run it handles setting up the native bridge
 * and injects it into Koin
 */
class LoginActivity : AppCompatActivity() {

    companion object {
        init {
            // The "android-support" crate creates a dynamic library called "libqauldroid"
            // which we can include here simply via "qauldroid" because it's being put
            // into the library search path via ~ m a g i c ~
            System.loadLibrary("qauldroid")
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.fragment_login)

        // Setup the native state
        NativeState.setup(setupQaul())

        // Populate the users list
        val users: ArrayList<String> = NativeState.get().usersList().map { it.displayName } as ArrayList<String>
        val spinner = findViewById<Spinner>(R.id.user_list_picker)

        Log.i("login", "Done fetching users")

        val aa = ArrayAdapter(this, simple_spinner_item, users)
        aa.setDropDownViewResource(simple_spinner_dropdown_item)
        spinner!!.adapter = aa

        val butt = findViewById<Button>(R.id.button_login)
        butt.setOnClickListener {
            Log.i("login", "Trying to log-in...")

            // Get current selected values
            val id = NativeState.get().usersList().find { it.displayName == spinner.selectedItem.toString() }!!.id
            val pwField = findViewById<EditText>(R.id.user_password_entry)
            val pw = pwField!!.text.toString()
            Log.i("login", "Trying to log-in 2...")

            if (NativeState.get().usersLogin(id, "1234")) {
                Log.i("login", "Successfully logged in!")
                val i = Intent(this, MainActivity::class.java)
                startActivity(i)
            } else {
                Toast.makeText(
                        applicationContext,
                        "Failed to login: wrong password!",
                        Toast.LENGTH_LONG).show();
            }
        }
    }

    private fun setupQaul(): NativeQaul {
        var assetsPath: String? = null
        try {
            assetsPath = unpackAssets("")
        } catch (e: Exception) {
            e.printStackTrace()
        }

        return NativeQaul(9900, assetsPath)
    }

    @Throws(Exception::class)
    private fun unpackAssets(path: String): String {
        val rootPath = applicationContext.filesDir.path
        val assetManager = this.assets
        val assets: Array<String>?
        try {
            assets = assetManager.list(path)
            assert(assets != null)
            if (assets!!.size == 0) {
                copyFile(path)
            } else {
                val fullPath = "$rootPath/webgui/$path"
                // Create the directory and recurse...
                val dir = File(fullPath)
                if (!dir.exists()) dir.mkdir()
                for (child in assets) {
                    unpackAssets(if (path == "") child else "$path/$child")
                }
            }
        } catch (ex: IOException) {
            ex.printStackTrace()
        }
        return applicationContext.filesDir.path
    }

    private fun copyFile(filename: String) {
        val rootPath = applicationContext.filesDir.path
        val assetManager = this.assets
        val in_: InputStream
        val out: OutputStream
        try {
            in_ = assetManager.open(filename)
            val newFileName = "$rootPath/webgui/$filename"
            out = FileOutputStream(newFileName)
            val buffer = ByteArray(1024)
            var read: Int
            while (in_.read(buffer).also { read = it } != -1) {
                out.write(buffer, 0, read)
            }
            in_.close()
            out.flush()
            out.close()
        } catch (e: Exception) {
            e.printStackTrace()
        }
    }
}
