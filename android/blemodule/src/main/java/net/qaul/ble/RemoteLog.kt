package net.qaul.ble

import android.content.Context
import android.util.Log
import java.io.BufferedWriter
import java.io.File
import java.io.FileWriter
import java.lang.Exception
import java.text.SimpleDateFormat
import java.util.*

/**
 * class for print log on app (device)
 */
class RemoteLog(private val context: Context) {
    private var bw: BufferedWriter? = null
    private var fw: FileWriter? = null
    private val TAG = javaClass.simpleName
    private var logDir: File? = null
    private val DIR_NAME: String
    private val FILE_NAME: String
    private var logFile: File? = null
    private fun createDirAndFile() {
        try {
            logDir = File(context.externalCacheDir, DIR_NAME)
            if (!logDir!!.exists()) {
                logDir!!.mkdir()
            }
            logFile = File(logDir!!.absolutePath, FILE_NAME)
            if (!logFile!!.exists()) {
                logFile!!.createNewFile()
            }
        } catch (e: Exception) {
            Log.e(TAG, "ex  -: " + e.message)
        }
    }

    val filePath: String
        get() {
            createDirAndFile()
            val logFile = File(logDir!!.absolutePath, FILE_NAME)
            return if (logFile != null) {
                logFile.absolutePath
            } else ""
        }
    private val fullDate: String
        private get() {
            val sdf = SimpleDateFormat("yyyy-MM-dd HH:mm:ss.SSS", Locale.getDefault())
            val date = Calendar.getInstance().time
            return sdf.format(date)
        }

    fun addDebugLog(log: String) {
        try {
            createDirAndFile()
            if (fw == null) fw = FileWriter(logFile!!.absoluteFile, true)
            if (bw == null) bw = BufferedWriter(fw)
            bw!!.append("$fullDate $log\n")
            bw!!.flush()
        } catch (e: Exception) {
            Log.e(TAG, "exe --: " + e.message)
        }
    }

    fun clearLog() {
        try {
            createDirAndFile()
            if (fw == null) fw = FileWriter(logFile!!.absoluteFile, false)
            if (bw == null) bw = BufferedWriter(fw)
            bw!!.write("")
            bw!!.flush()
        } catch (e: Exception) {
            Log.e(TAG, "exe --: " + e.message)
        }
    }

    companion object {
        private var log: RemoteLog? = null

        @Synchronized
        operator fun get(context: Context): RemoteLog? {
            if (log == null) {
                log = RemoteLog(context)
            }
            return log
        }
    }

    init {
        DIR_NAME = "LogFile"
        FILE_NAME = "log.txt"
        createDirAndFile()
    }
}