//
//  BleService.swift
//  QaulBLE
//
//  Created by BAPS on 28/01/22.
//

import Foundation

let deviceName = "qauliOS"
let appendtextiOSdevice = "Apple"
var bleService = BleService()

public class BleService {
    
    private var devicesList = [String: BLEScanDevice]() // All devices in list
    private var ignoreList = [String: BLEScanDevice]() // Qualid match
    private var receiveList = [String:BLEScanDevice]() // if not in ignorelist and message get then its in ignorlist
    
    
//    private var blackList = [BLEScanDevice]() not need in iOS
    
//    public typealias BleRequestCallback = ((Qaul_Sys_Ble_Ble) -> Void)
//    public var bleCallback: BleRequestCallback!
    
    static let shared = BleService()
    
//    var bleCallback: BleScanCallBack? = null

    //((status: Bool, errorText: String, unknownError: Bool) -> Void)
    public typealias startAdvertiseRes = ((Bool, String, Bool) -> Void)
    private var StartbleAdvertiseCallback: startAdvertiseRes!
    
    //(status: Bool, errorText: String)
    public typealias stopAdvertiseRes = ((Bool, String) -> Void)
    private var StopbleAdvertiseCallback: stopAdvertiseRes!
    
    //(status: Boolean, errorText: String, unknownError: Boolean)
    public typealias startScanRes = ((Bool, String, Bool) -> Void)
    private var startbleScanCallback: startScanRes!
    
    //(status: Bool, errorText: String)
    public typealias stopScanRes = ((Bool, String) -> Void)
    private var StopbleScanCallback: stopAdvertiseRes!
    
    //onMessageSent(id: String, success: Boolean, data: ByteArray)
    public typealias onMessageSent = ((String, Bool, Date) -> Void)
    private var onMessageSentCallback: onMessageSent!
    
    var qaulId: Data? = nil
    private var advertMode = ""
    
    var isAdvertisementRunning = false
    var isScanningRunning = false
    
    /**
     * This Method Will Set the necessary data and start the advertisement
     */
    func startAdvertise( bleCallback: @escaping startAdvertiseRes) {
//        bleService.qaulId = qaul_id
//        bleService.advertMode = mode
        bleService.StartbleAdvertiseCallback = bleCallback
        bLEPeripheral.startAdvertising(serviceID: SERVICES.SERVICE_UUID, name: deviceName){ (status, errorText, unknownError) in
            
            self.isAdvertisementRunning = status
            self.StartbleAdvertiseCallback(status, errorText , unknownError)
        }
        
    }
    
    /**
     * This Method Will Stop the Service if It Is Running
     */
    func stop(bleCallback: @escaping stopAdvertiseRes) {
        
        bleService.StopbleAdvertiseCallback = bleCallback
        
        if self.isAdvertisementRunning {
            bLEPeripheral.stopAdvertising { status, errorText in
                self.isAdvertisementRunning = !status
                self.StopbleAdvertiseCallback(status, errorText)
            }
        } else {
            StopbleAdvertiseCallback(false , "")
        }
        
        
//        if (bleService != nil) {
//            var str = "$TAG stopped"
//            bleService?.outOfRangeChecker?.removeCallbacks(outRangeRunnable)
//            if bleService.isAdvertiserRunning() {
//
//            }
////            if (bleService.isScanRunning()) {
////                stopScan()
////            }
//
//            bleService.stopSelf()
//        } else {
//            bleAdvertiseCallback?.stopAdvertiseRes(
//                status = false,
//                errorText = "$TAG not started"
//            )
//            bleCallback?.stopScanRes(status = false, errorText = "")
//            AppLog.e(TAG, "$TAG not started")
//        }
    }
    func stopScan(bleCallback: @escaping stopScanRes) {
        
        bleService.StopbleScanCallback = bleCallback
    
        if self.isScanningRunning{
            bleManager.StopScan()
            self.isScanningRunning = false
            self.StopbleScanCallback(true, "")
        } else {
            self.StopbleScanCallback(false, "")
        }
    }
    /**
     * This Method Will Set Filter, ScanMode, and Start Scanning
     */
    func startScan(bleCallback: @escaping startScanRes) {
        startbleScanCallback = bleCallback
        self.isScanningRunning = true//bleManager.isScanningStart
        self.startbleScanCallback(true, "" , false)
//        self.startbleScanCallback(bleManager.isScanningStart, "" , false)

        bleManager.StartScanning(WithServices: [SERVICES.SERVICE_UUID]) { bLEDevice in
        
            if self.ignoreList[bLEDevice.uuid] == nil {
                
                if self.devicesList[bLEDevice.uuid] == nil {
                    
                    var scanDeviceObj             = BLEScanDevice()
                    scanDeviceObj.macAddress      = bLEDevice.uuid
                    scanDeviceObj.bluetoothDevice = bLEDevice
                    self.devicesList[bLEDevice.uuid] = scanDeviceObj
                    
//                    self.devicesList[bLEDevice.uuid]?.bluetoothDevice = bLEDevice
                    
                }
            } else {
                // Just ingore device here anyhow
            }
        }
       
    }
    
    /**
     * This Method Will Be Used to Send Data to Other Qaul-Device
     */
    
    /**
     * This Method Will Return True if Service is Running
     */
    func isRunning() -> Bool {
        return bleService != nil
    }

    /**
     * This Method Will Return True if Advertisement is ON
     */
    func isAdvertiserRunning() -> Bool {
        return isAdvertisementRunning
    }
    
    /**
     * This Method Will Return True if Scan is Running
     */
    func isScanRunning() -> Bool {
        return isScanningRunning
    }
    
    /**
     * This Method Will Start the Service
     */
    func start() {
        if (bleService == nil) {
            bleService = BleService()
        } else {
//            AppLog.e(TAG, "$TAG already started")
        }
    }
    
    /**
     * This is a Interface for Sending Advertisement Start & Stop Response to BLEWrapperClass.
     */
//    interface BleAdvertiseCallback {
//        fun startAdvertiseRes(status: Boolean, errorText: String, unknownError: Boolean)
//        fun stopAdvertiseRes(status: Boolean, errorText: String)
//        fun onMessageReceived(bleDevice: BLEScanDevice, message: ByteArray)
//    }
//
//    /**
//     * This is a Interface for Sending Scan Start & Stop Response to BLEWrapperClass.
//     */
//    interface BleScanCallBack {
//        fun startScanRes(status: Boolean, errorText: String, unknownError: Boolean)
//        fun stopScanRes(status: Boolean, errorText: String)
//        fun deviceFound(bleDevice: BLEScanDevice)
//        fun deviceOutOfRange(bleDevice: BLEScanDevice)
//        fun onMessageSent(id: String, success: Boolean, data: ByteArray)
//    }
}



extension Notification.Name {
    
    static let GetscanDevice = Notification.Name(rawValue: "GetscanDevice")
   
}
