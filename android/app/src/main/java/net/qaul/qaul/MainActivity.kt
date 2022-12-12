// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

package net.qaul.qaul

import android.content.Intent
import android.content.pm.PackageManager
import android.os.Build
import android.os.Bundle
import android.view.Menu
import android.view.MenuItem
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity
import androidx.navigation.findNavController
import androidx.navigation.ui.AppBarConfiguration
import androidx.navigation.ui.navigateUp
import androidx.navigation.ui.setupActionBarWithNavController
import com.google.gson.Gson
import com.google.protobuf.ByteString
import net.qaul.ble.AppLog
import net.qaul.ble.callback.BleRequestCallback
import net.qaul.ble.core.BleWrapperClass
import net.qaul.ble.core.BleWrapperClass.Companion.BLE_PERMISSION_REQ_CODE_12
import net.qaul.ble.core.BleWrapperClass.Companion.LOCATION_ENABLE_REQ_CODE
import net.qaul.ble.core.BleWrapperClass.Companion.LOCATION_PERMISSION_REQ_CODE
import net.qaul.ble.core.BleWrapperClass.Companion.REQUEST_ENABLE_BT
import net.qaul.qaul.databinding.ActivityMainBinding
import qaul.sys.ble.BleOuterClass
import java.nio.charset.Charset

class MainActivity : AppCompatActivity(), BleRequestCallback {

    private val TAG: String = "qaul-blemodule MainActivity"
    private lateinit var appBarConfiguration: AppBarConfiguration
    private lateinit var binding: ActivityMainBinding
    private lateinit var bleWrapperClass: BleWrapperClass
    private lateinit var qaulId: String

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        binding = ActivityMainBinding.inflate(layoutInflater)
        setContentView(binding.root)
        qaulId = getDeviceName()
        if (qaulId.length > 18) {
            qaulId = qaulId.substring(0,17)
        }
        setSupportActionBar(binding.toolbar)
        bleWrapperClass = BleWrapperClass(context = this)
        binding.btnInfoRequest.setOnClickListener {
            sendInfoRequest()
        }
        binding.btnStartRequest.setOnClickListener {
            sendStartRequest()
        }
        binding.btnStopRequest.setOnClickListener {
            sendStopRequest()
        }
        binding.btnSendMessage.setOnClickListener {
            validateData()
        }
        val navController = findNavController(R.id.nav_host_fragment_content_main)
        appBarConfiguration = AppBarConfiguration(navController.graph)
        setupActionBarWithNavController(navController, appBarConfiguration)
        oldCode()
    }

    /**
     * For Sending BleInfoRequest to BLEModule
     */
    private fun sendInfoRequest() {
        val bleReq: BleOuterClass.Ble.Builder = BleOuterClass.Ble.newBuilder()
        bleReq.infoRequest = BleOuterClass.BleInfoRequest.getDefaultInstance()
        bleWrapperClass.receiveRequest(data = bleReq.build().toByteString(), callback = this)
    }

    /**
     * For Sending BleStartRequest to BLEModule
     * Have to pass qaul_id and advertise_mode as parameter
     */
    private fun sendStartRequest() {
        val bleReq: BleOuterClass.Ble.Builder = BleOuterClass.Ble.newBuilder()
        val startRequest = BleOuterClass.BleStartRequest.newBuilder()
        val qaulid = qaulId.toByteArray(Charset.forName("UTF-8"))
//        val qaulid = byteArrayOf(
//            0x48,0x65,0x6c,0x6c,0x6f,0x41,0x6a,0x61,0x79,0x48,0x6f,0x77,0x41,0x72,0x65,0x59,0x6f,0x75,0x48,0x65
//        )
        AppLog.e(TAG, "qaulid : " + qaulid.size)
        startRequest.qaulId = ByteString.copyFrom(qaulid)
        startRequest.powerSetting = BleOuterClass.BlePowerSetting.low_latency
        bleReq.startRequest = startRequest.build()
        bleWrapperClass.receiveRequest(data = bleReq.build().toByteString(), callback = this)
    }

    /**
     * For Sending BleStopRequest to BLEModule. It Is Used To Stop Service.
     */
    private fun sendStopRequest() {
        val bleReq: BleOuterClass.Ble.Builder = BleOuterClass.Ble.newBuilder()
        bleReq.stopRequest = BleOuterClass.BleStopRequest.getDefaultInstance()
        bleWrapperClass.receiveRequest(data = bleReq.build().toByteString(), callback = this)
    }

    private fun validateData() {
        val qaulId = binding.etQaulId.text.toString()
        val message = binding.etMessage.text.toString()
        when {
            qaulId.length < 10 -> {
                Toast.makeText(
                    this,
                    "Please enter correct qaul_id of receiver",
                    Toast.LENGTH_SHORT
                ).show()
                return
            }
            message.isEmpty() -> {
                Toast.makeText(
                    this,
                    "Please enter at least 1 character of message",
                    Toast.LENGTH_SHORT
                ).show()
                return
            }
            else -> {
                sendData(qaulId = qaulId, message = "$message")
            }
        }
    }


    private fun sendData(qaulId: String, message: String) {
        val bleReq: BleOuterClass.Ble.Builder = BleOuterClass.Ble.newBuilder()
        val directSend = BleOuterClass.BleDirectSend.newBuilder()
        directSend.data = ByteString.copyFrom(message.toByteArray(Charset.forName("UTF-8")))
        directSend.receiverId = ByteString.copyFrom(qaulId.toByteArray(Charset.forName("UTF-8")))
        directSend.senderId = ByteString.copyFrom(this.qaulId, Charset.defaultCharset())
        directSend.messageId = ByteString.copyFrom(
            System.currentTimeMillis().toString().toByteArray(Charset.forName("UTF-8"))
        )
        bleReq.directSend = directSend.build()
        bleWrapperClass.receiveRequest(data = bleReq.build().toByteString(), callback = this)
        runOnUiThread {
            Toast.makeText(this, "Connecting...", Toast.LENGTH_SHORT).show()
        }
    }


    /**
     * This Method Will Be Called When User Accept/Decline Asked Permission From BLEModule
     * After Response It Will Send User's Response to BLEModule
     */
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

    /**
     * This Method Will Be Called When User Accept/Decline Asked to Turn On
     * Bluetooth and/or Location(GPS) From BLEModule
     * After Response It Will Send User's Response to BLEModule
     */
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

    /**
     * This Method Will Be Called When BLEModule Send Response To BLERequests Sent
     */
    override fun bleResponse(data: ByteString) {
        val ble: BleOuterClass.Ble = BleOuterClass.Ble.parseFrom(data)
        if (ble.isInitialized) {
            when (ble.messageCase) {
                BleOuterClass.Ble.MessageCase.INFO_RESPONSE -> {
                    val deviceInfo: BleOuterClass.BleDeviceInfo = ble.infoResponse.device
                    AppLog.e("bleResponse: ", Gson().toJson(deviceInfo))
                    Toast.makeText(
                        this,
                        "Device info received from : ${deviceInfo.name}",
                        Toast.LENGTH_SHORT
                    ).show()
                }
                BleOuterClass.Ble.MessageCase.START_RESULT -> {
                    val startResult: BleOuterClass.BleStartResult = ble.startResult
                    AppLog.e("startResult: ", Gson().toJson(startResult))
                    Toast.makeText(
                        this,
                        startResult.errorMessage,
                        Toast.LENGTH_SHORT
                    ).show()
                }
                BleOuterClass.Ble.MessageCase.STOP_RESULT -> {
                    val stopResult: BleOuterClass.BleStopResult = ble.stopResult
                    AppLog.e("stopResult: ", Gson().toJson(stopResult))
                    Toast.makeText(
                        this,
                        stopResult.errorMessage,
                        Toast.LENGTH_SHORT
                    ).show()
                }
                BleOuterClass.Ble.MessageCase.DEVICE_DISCOVERED -> {
                    val scanResult: BleOuterClass.BleDeviceDiscovered = ble.deviceDiscovered
                    AppLog.e(
                        "deviceDiscovered: ",
                        Gson().toJson(scanResult) + " " + String(scanResult.qaulId.toByteArray())
                    )

                    runOnUiThread {
                        if (!scanResult.qaulId.isEmpty) {
                            binding.etQaulId.setText(String(scanResult.qaulId.toByteArray()))
                        }
                    }

                }
                BleOuterClass.Ble.MessageCase.DEVICE_UNAVAILABLE -> {
                    val scanResult: BleOuterClass.BleDeviceUnavailable = ble.deviceUnavailable
                    AppLog.e(
                        "deviceUnavailable: ",
                        Gson().toJson(scanResult) + " " + String(scanResult.qaulId.toByteArray())
                    )
                }
                BleOuterClass.Ble.MessageCase.DIRECT_RECEIVED -> {
                    val directReceived: BleOuterClass.BleDirectReceived = ble.directReceived
                    AppLog.e("directReceived: ", Gson().toJson(directReceived))
                    val message: String = directReceived.data.toString(Charset.defaultCharset())
                    val qaulId: String = directReceived.from.toString(Charset.defaultCharset())
                    runOnUiThread {
                        binding.tvMessage.text = message
                        binding.etQaulId.setText(qaulId)
                    }
                }
                BleOuterClass.Ble.MessageCase.DIRECT_SEND_RESULT -> {
                    val directSendResult: BleOuterClass.BleDirectSendResult = ble.directSendResult
                    AppLog.e("directSendResult: ", Gson().toJson(directSendResult))
                    runOnUiThread {
                        Toast.makeText(this, directSendResult.errorMessage, Toast.LENGTH_SHORT)
                            .show()
                    }
                }
                else -> {

                }
            }
        }
    }

    /**
     * Returns Device Manufacturer & Model Name/Number
     */
    private fun getDeviceName(): String {
        val manufacturer = Build.MANUFACTURER
        val model = Build.MODEL
        return if (model.lowercase().startsWith(manufacturer.lowercase())) {
            capitalize(model)
        } else {
            capitalize(manufacturer).toString() + " " + model
        }
    }

    /**
     * Capitalize 1st Letter of String
     */
    private fun capitalize(s: String?): String {
        if (s == null || s.isEmpty()) {
            return ""
        }
        val first = s[0]
        return if (Character.isUpperCase(first)) {
            s
        } else {
            Character.toUpperCase(first).toString() + s.substring(1)
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

    private fun oldCode() {
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
}