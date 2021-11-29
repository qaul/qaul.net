package net.qaul.qaul

import android.os.Bundle
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
import net.qaul.qaul.databinding.ActivityMainBinding
import qaul.sys.ble.BleOuterClass

class MainActivity : AppCompatActivity() {

    private lateinit var appBarConfiguration: AppBarConfiguration
    private lateinit var binding: ActivityMainBinding
    private lateinit var bleWrapperClass: BleWrapperClass
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        binding = ActivityMainBinding.inflate(layoutInflater)
        setContentView(binding.root)
        setSupportActionBar(binding.toolbar)
        binding.btnStart.setOnClickListener {
            bleWrapperClass = BleWrapperClass(context = this)
//            bleWrapperClass.isBleScanConditionSatisfy()
//            BleWrapperClass.startService(this)
            val bleReq : BleOuterClass.Ble.Builder = BleOuterClass.Ble.newBuilder()
            bleReq.infoRequest = BleOuterClass.BleInfoRequest.getDefaultInstance()
            bleWrapperClass.getRequest(bleReq = bleReq.build(), object : BleRequestCallback {
                override fun bleResponse(ble: BleOuterClass.Ble) {
                    if (ble.isInitialized) {
                        if (ble.messageCase == BleOuterClass.Ble.MessageCase.INFO_RESPONSE) {
                            val deviceInfo: BleOuterClass.BleDeviceInfo = ble.infoResponse.device
                            AppLog.e("bleResponse: ", Gson().toJson(deviceInfo))
                        }
                    }
                }
            })
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
}