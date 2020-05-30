package net.qaul.app

import android.os.Bundle
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity
import net.qaul.app.R
import net.qaul.app.ffi.NativeQaul
import java.io.*
import java.lang.StringBuilder

class MainActivity : AppCompatActivity() {
    protected var libqaul: NativeQaul? = null;

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
        val tv = findViewById<TextView>(R.id.sample_text);
        var assetsPath: String? = null;
        try {
            assetsPath = unpackAssets("");
        } catch (e: Exception) {
            e.printStackTrace();
        }

        this.libqaul = NativeQaul(9999, assetsPath);

//        val sb = StringBuilder();
//        for (user in this.libqaul!!.usersList()) {
//            sb.append("User: " + user.displayName + " | ");
//        }
//
//        tv.text = sb;
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
}