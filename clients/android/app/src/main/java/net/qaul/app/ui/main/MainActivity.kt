package net.qaul.app.ui.main

import android.os.Bundle
import android.text.Layout
import android.view.ViewGroup
import android.widget.LinearLayout
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity
import net.qaul.app.R

class MainActivity : AppCompatActivity() {
    protected var libqaulState: Long = 0

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

        // Start the libqaul machinery under the hood
        libqaulState = startServer(5000, "/")
        println(libqaulState)

        // Set the login fragment so people can make an account
        // setContentView(R.layout.fragment_login)
    }

    external fun hello(to: String?): String?
    external fun startServer(port: Int, path: String?): Long
}