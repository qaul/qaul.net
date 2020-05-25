package net.qaul.app.ui.main

import android.os.Bundle
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity
import net.qaul.app.R
import java.io.*

class MainActivity : AppCompatActivity() {
    protected var libqaulState: Long? = null

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
        setContentView(R.layout.activity_main)
        // Example of a call to a native method
        val tv = findViewById<TextView>(R.id.sample_text)
        tv.text = hello("qaul.net")

        var assetsPath: String? = null
        try {
            assetsPath = unpackAssets("")
        } catch (e: Exception) {
            e.printStackTrace()
        }

        // Setup the libqaul state
        this.setup(5000, assetsPath);
        assert(this.libqaulState != null);




//        // Start the libqaul machinery under the hood
//        libqaulState = startServer(5000, "$assetsPath/webgui")
//        println(libqaulState)
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
                    unpackAssets( if(path == "") child else "$path/$child" )
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

    external fun setup(port: Int, root_path: String?);

    external fun hello(to: String?): String?
    external fun startServer(port: Int, path: String?): Long
}