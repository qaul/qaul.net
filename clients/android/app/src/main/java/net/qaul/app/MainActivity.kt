package net.qaul.app

import android.os.Bundle
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity

class MainActivity : AppCompatActivity() {
    protected var libqaulState: Long = 0

    companion object {
        init { // The "android-support" crate creates a dynamic library called "libqauldroid"
// which we can include here simply via "qauldroid" because it's being put
// into the library search path via ~ m a g i c ~
            System.loadLibrary("qauldroid")
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
        // Example of a call to a native method
        val tv = findViewById<TextView>(R.id.sample_text)
        tv.text = hello("qaul.net")
        // Start the libqaul machinery under the hood
        libqaulState = startServer(5000, "/")
        println(libqaulState)
    }

    external fun hello(to: String?): String?
    external fun startServer(port: Int, path: String?): Long
}