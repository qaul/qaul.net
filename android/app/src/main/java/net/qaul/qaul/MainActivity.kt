package net.qaul.qaul

import android.content.Intent
import android.content.pm.PackageManager
import android.os.Bundle
import android.os.Handler
import androidx.appcompat.app.AppCompatActivity
import androidx.navigation.findNavController
import androidx.navigation.ui.AppBarConfiguration
import androidx.navigation.ui.navigateUp
import androidx.navigation.ui.setupActionBarWithNavController
import android.view.Menu
import android.view.MenuItem
import com.google.gson.Gson
import net.qaul.ble.AppLog
import net.qaul.ble.callback.BleRequestCallback
import net.qaul.ble.core.BleWrapperClass
import net.qaul.ble.core.BleWrapperClass.Companion.BLE_PERMISSION_REQ_CODE_12
import net.qaul.ble.core.BleWrapperClass.Companion.LOCATION_ENABLE_REQ_CODE
import net.qaul.ble.core.BleWrapperClass.Companion.LOCATION_PERMISSION_REQ_CODE
import net.qaul.ble.core.BleWrapperClass.Companion.REQUEST_ENABLE_BT
import net.qaul.qaul.databinding.ActivityMainBinding
import qaul.sys.ble.BleOuterClass

class MainActivity : AppCompatActivity(), BleRequestCallback {

    private lateinit var appBarConfiguration: AppBarConfiguration
    private lateinit var binding: ActivityMainBinding
    private lateinit var bleWrapperClass: BleWrapperClass

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        binding = ActivityMainBinding.inflate(layoutInflater)
        setContentView(binding.root)
        setSupportActionBar(binding.toolbar)
        bleWrapperClass = BleWrapperClass(context = this)
        binding.btnInfoRequest.setOnClickListener {
            val bleReq: BleOuterClass.Ble.Builder = BleOuterClass.Ble.newBuilder()
            bleReq.infoRequest = BleOuterClass.BleInfoRequest.getDefaultInstance()
            bleWrapperClass.receiveRequest(bleReq = bleReq.build(), this)
        }
        binding.btnStartRequest.setOnClickListener {
            val bleReq: BleOuterClass.Ble.Builder = BleOuterClass.Ble.newBuilder()
            val startRequest = BleOuterClass.BleStartRequest.newBuilder()
            startRequest.qaulId = "123abc456ABC"
            bleReq.startRequest = startRequest.build()
            bleWrapperClass.receiveRequest(bleReq = bleReq.build(), this)
        }
        val navController = findNavController(R.id.nav_host_fragment_content_main)
        appBarConfiguration = AppBarConfiguration(navController.graph)
        setupActionBarWithNavController(navController, appBarConfiguration)

        // load libqaul
        println("load libqaul")
//        loadLibqaul()
        println("libqaul loaded")

        // get app storage path
        val storagePath = this.filesDir.absolutePath

        // start libqaul
        println("start libqaul")
        println("libqaul storage path: $storagePath")
        println("from now on, the logging should work")
//        start(storagePath)
        println("libqaul started 6")

        // print log "Hello qaul!"
        println("before calling hello")
//        println(hello())
        println("after calling hello")

        // wait until library finished starting up
//        while (initialized() == false) {
//            Thread.sleep(1)
//        }
        println("libqaul finished initializing")

        // TODO: create and send rpc message

        // get messages received
//        var counter = sendcounter()
//        println("libqaul RPC messages sent = $counter")

        // get messages queued
//        var queued = receivequeue()
//        println("libqaul RPC messages queued = $queued")

        // set text from libqaul to click on bottom icon
        // it prints the message "Hello qaul!"
//        var hellotxt = hello()

        binding.fab.setOnClickListener { view ->
//            Snackbar.make(view, hellotxt, Snackbar.LENGTH_LONG)
//                .setAction("Action", null).show()
        }
    }

    override fun onRequestPermissionsResult(
        requestCode: Int,
        permissions: Array<String?>,
        grantResults: IntArray
    ) {
        super.onRequestPermissionsResult(requestCode, permissions, grantResults)
        if (requestCode == LOCATION_PERMISSION_REQ_CODE) {
            AppLog.e(
                "MainActivity",
                "REQ CODED -  " + requestCode + "  Size  " + grantResults.size
            )
            if (grantResults.isNotEmpty()) {
                for (grantResult in grantResults) {
                    if (grantResult == PackageManager.PERMISSION_DENIED) {
                        AppLog.e("MainActivity", "grantResults- IF $grantResult")
                        bleWrapperClass.onResult(requestCode = requestCode, status = false)
                        break
                    }
                }
                bleWrapperClass.onResult(requestCode = requestCode, status = true)
            }
        } else if (requestCode == BLE_PERMISSION_REQ_CODE_12) {
            AppLog.e(
                "MainActivity",
                "REQ CODED -  " + requestCode + "  Size  " + grantResults.size
            )
            if (grantResults.isNotEmpty()) {
                for (grantResult in grantResults) {
                    if (grantResult == PackageManager.PERMISSION_DENIED) {
                        AppLog.e("MainActivity", "grantResults- IF $grantResult")
                        bleWrapperClass.onResult(requestCode = requestCode, status = false)
                        break
                    }
                }
                bleWrapperClass.onResult(requestCode = requestCode, status = true)
            }
        }
    }

    override fun onActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {
        super.onActivityResult(requestCode, resultCode, data)
        AppLog.e(
            "MainActivity",
            "onActivityResult requestCode=$requestCode | resultCode=$resultCode"
        )
        if (requestCode == LOCATION_ENABLE_REQ_CODE) {
            if (resultCode == RESULT_OK) {
                AppLog.e("MainActivity", "Location Yes")
                bleWrapperClass.onResult(requestCode = requestCode, status = true)
            } else {
                AppLog.e("MainActivity", "Location No")
                bleWrapperClass.onResult(requestCode = requestCode, status = false)
            }
        } else if (requestCode == REQUEST_ENABLE_BT) {
            if (resultCode == RESULT_OK) {
                AppLog.e("MainActivity", "BT Yes")
                bleWrapperClass.onResult(requestCode = requestCode, status = true)
            } else {
                AppLog.e("MainActivity", "BT No")
                bleWrapperClass.onResult(requestCode = requestCode, status = false)
            }
        }
    }

    override fun onCreateOptionsMenu(menu: Menu): Boolean {
        // Inflate the menu; this adds items to the action bar if it is present.
        menuInflater.inflate(R.menu.menu_main, menu)
        return true
    }

    override fun onOptionsItemSelected(item: MenuItem): Boolean {
        // Handle action bar item clicks here. The action bar will
        // automatically handle clicks on the Home/Up button, so long
        // as you specify a parent activity in AndroidManifest.xml.
        return when (item.itemId) {
            R.id.action_settings -> true
            else -> super.onOptionsItemSelected(item)
        }
    }

    override fun onSupportNavigateUp(): Boolean {
        val navController = findNavController(R.id.nav_host_fragment_content_main)
        return navController.navigateUp(appBarConfiguration)
                || super.onSupportNavigateUp()
    }

    override fun bleResponse(ble: BleOuterClass.Ble) {
        if (ble.isInitialized) {
            if (ble.messageCase == BleOuterClass.Ble.MessageCase.INFO_RESPONSE) {
                val deviceInfo: BleOuterClass.BleDeviceInfo = ble.infoResponse.device
                AppLog.e("bleResponse: ", Gson().toJson(deviceInfo))
            } else if (ble.messageCase == BleOuterClass.Ble.MessageCase.START_RESULT) {
                val startResult: BleOuterClass.BleStartResult = ble.startResult
                AppLog.e("startResult: ", Gson().toJson(startResult))
            }
        }
    }
}