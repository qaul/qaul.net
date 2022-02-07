//
//  BleWrapperClass.swift
//  QaulBLE
//
//  Created by BAPS on 13/01/22.
//

import UIKit
import Foundation
import CoreBluetooth

let bleWrapperClass = BleWrapperClass()

class BleWrapperClass: UIViewController {
    
    public typealias BleRequestCallback = ((Qaul_Sys_Ble_Ble) -> Void)
    public var bleCallback: BleRequestCallback!
    
    //private var TAG: String = BleWrapperClass::class.java.simpleName
   // private var context = context
    private var errorText = ""
    private var noRights = false
    private var qaulId: Data? = nil
    private var advertMode : Qaul_Sys_Ble_BleMode = .lowLatency
    private var isFromMessage = false
    
    /**
     * Static Member Declaration
     */
    //var serviceManager = this
    
    let REQUEST_ENABLE_BT = 113
    let BLE_PERMISSION_REQ_CODE_12 = 114
 
    
    /**
     * This Method get BLERequest from UI & Return BLEResponse by Callback Interface Method
     */
    func receiveRequest(bleReq: Qaul_Sys_Ble_Ble, SetdataforbleReq : Qaul_Sys_Ble_Ble ,callback: @escaping BleRequestCallback) {
        if (bleReq.isInitialized) {
        
            bleCallback = callback
            print("bleReq.message:- \(bleReq.message)")
            print("SetdataforbleReq.message:- \(SetdataforbleReq.message)")
//            Log.e(TAG, bleReq.messageCase.toString())
            switch SetdataforbleReq.message! {
            case .infoRequest(Qaul_Sys_Ble_BleInfoRequest()):
                getDeviceInfo()
                break
            case .startRequest(Qaul_Sys_Ble_BleStartRequest())://is Qaul_Sys_Ble_BleStartRequest:

                print(".startRequest(Qaul_Sys_Ble_BleStartRequest())")
                qaulId = bleReq.startRequest.qaulID
//                    //AppLog.e(TAG, "qaulid : " + qaulId?.size)
                advertMode = bleReq.startRequest.mode
                    if (qaulId != nil) {
                        startService()
                    } else {
                        var bleRes = Qaul_Sys_Ble_Ble.init()
                        
                        var startResult = Qaul_Sys_Ble_BleStartResult.init()
                        startResult.success = false
                        startResult.noRights = false
                        startResult.errorMessage = "qaul id required"
                        startResult.unknownError = false
                        
                        bleRes.startResult = startResult
                        bleCallback(bleRes)
                    }

                break
            case .stopRequest(Qaul_Sys_Ble_BleStopRequest())://is Qaul_Sys_Ble_BleStopRequest:
                print(".stopResult(Qaul_Sys_Ble_BleStopRequest())")
                stopService()
                break

            case .directSend(Qaul_Sys_Ble_BleDirectSend())://is Qaul_Sys_Ble_BleDirectSend:
                let bleDirectSend = bleReq.directSend
                if bleService.isRunning() {
                    bleService.sendMessage(id: bleDirectSend.id, to: bleDirectSend.to, message: bleDirectSend.data, from: bleDirectSend.qaulID) { id, success, data in
                        
                        var bleRes = Qaul_Sys_Ble_Ble.init()
                        
                        var directSendResult = Qaul_Sys_Ble_BleDirectSendResult.init()
                        directSendResult.success = false
                        directSendResult.id = id
                        directSendResult.errorMessage = success ? "Successfully sent" : "Connection not established. Please try again."
                        
                        bleRes.directSendResult = directSendResult
                        self.bleCallback(bleRes)
                        
                       
                    }
                }
                break
            default:
                print("Default")
                break
            }
        }
    }
    /**
     * This Method Return Device Information Regarding BLE Functionality & Permissions
     */
    private func getDeviceInfo() {
        //var bluetoothManager = context.getSystemService(LifecycleService.BLUETOOTH_SERVICE) as BluetoothManager
        //var adapter = bluetoothManager.adapter
        var bleRes: Qaul_Sys_Ble_Ble = Qaul_Sys_Ble_Ble()
        var bleResInfoResponse = Qaul_Sys_Ble_BleInfoResponse()
        if (bleRes.isInitialized) {
            if (bleResInfoResponse.isInitialized) {
                var deviceInfoBuilder : Qaul_Sys_Ble_BleDeviceInfo = Qaul_Sys_Ble_BleDeviceInfo()
               
//                deviceInfoBuilder.locationPermission = isLocationPermissionAllowed()
//                deviceInfoBuilder.locationOn = isLocationEnable()
                deviceInfoBuilder.blePermission = isBluetoothPermissionAllowed()
                deviceInfoBuilder.bluetoothOn = isBluetoothEnable()
                //deviceInfoBuilder.androidVersion = getOsVersion()
                deviceInfoBuilder.name = getDeviceName()
                deviceInfoBuilder.bleSupport = isBLeSupported()
                deviceInfoBuilder.adv251 = true//adapter.leMaximumAdvertisingDataLength > 250 //fix in ios 182
                deviceInfoBuilder.adv1M = false //adapter.isLeExtendedAdvertisingSupported // skip set default
                deviceInfoBuilder.adv2M = false //adapter.isLe2MPhySupported // skip set default
                deviceInfoBuilder.advCoded = false //adapter.isLeCodedPhySupported // skip set default
                deviceInfoBuilder.advExtendedBytes = 182//adapter.leMaximumAdvertisingDataLength // 182

                //Return true if LE Periodic Advertising feature is supported.
                deviceInfoBuilder.lePeriodicAdvSupport = false//adapter.isLePeriodicAdvertisingSupported // skip set default

                //Return true if the multi advertisement is supported by the chipset
                deviceInfoBuilder.leMultipleAdvSupport = false//adapter.isMultipleAdvertisementSupported //skip set default

                //Return true if offloaded filters are supported true if chipset supports on-chip filtering
                deviceInfoBuilder.offloadFilterSupport = false//adapter.isOffloadedFilteringSupported // skip set default

                //Return true if offloaded scan batching is supported true if chipset supports on-chip scan batching
//                deviceInfoBuilder.offloadScanBatchingSupport = adapter.isOffloadedScanBatchingSupported // skip
//                if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
                    deviceInfoBuilder.leAudio = false//isClass("android.bluetooth.BluetoothLeAudio") // skip set default
//                }
                
               // deviceInfoBuilder.iOS_version = getOsVersion()
                bleResInfoResponse.device = deviceInfoBuilder
            }
            bleRes.infoResponse = bleResInfoResponse
            
            bleCallback(bleRes)
            //?.bleResponse(ble = bleRes)
        }
    }
    
    /**
     * Checks if Bluetooth Permission is Allowed or Not for Android 12 & Above
     */
    private func isBluetoothPermissionAllowed() -> Bool {
        if #available(iOS 13.0, *) {
            return CBCentralManager().authorization == .allowedAlways
        }
        return CBPeripheralManager.authorizationStatus() == .authorized
    }
    
    /**
     * Checks if Bluetooth is Enabled or Not
     */
    private func isBluetoothEnable() -> Bool {
       
        let centralManager = CBCentralManager(delegate: self, queue: nil)
        return centralManager.state == .poweredOn
    }
    
    /**
     * Return the Current OS SDK Version
     */
    private func getOsVersion() -> String {
        return UIDevice.current.systemVersion
    }
    
    /**
     * Returns Device Manufacturer & Model Name/Number
     */
    private func getDeviceName() -> String {
       //return UIDevice.current.localizedModel
        return appendtextiOSdevice + UIDevice.modelName
    }
    
    /**
     * Checks if BLE Feature is Supported or Not
     */
    private func isBLeSupported() -> Bool {
        return true
        //context.packageManager.hasSystemFeature(PackageManager.FEATURE_BLUETOOTH_LE)
    }
    
    /**
     * This Method Will Stop the Service & Advertisement.
     */
    private func stopService() {
        if bleService.isAdvertiserRunning(){
            bleService.stop { status, errorText in
                var bleRes: Qaul_Sys_Ble_Ble = Qaul_Sys_Ble_Ble()
                var stopResult = Qaul_Sys_Ble_BleStopResult()
                stopResult.success = status
                stopResult.errorMessage = status ? "Advertisement Stopped" : errorText
                bleRes.stopResult = stopResult
                self.bleCallback(bleRes)
            }
        } else {
            var bleRes: Qaul_Sys_Ble_Ble = Qaul_Sys_Ble_Ble()
            var stopResult = Qaul_Sys_Ble_BleStopResult()
            stopResult.success = false
            stopResult.errorMessage = "Advertisement & Scanning is not Running"
            bleRes.stopResult = stopResult
            bleCallback(bleRes)
        }
        
        if bleService.isScanRunning(){
            bleService.stopScan { status, errorText in
                var bleRes: Qaul_Sys_Ble_Ble = Qaul_Sys_Ble_Ble()
                var stopResult = Qaul_Sys_Ble_BleStopResult()
                stopResult.success = false
                stopResult.errorMessage = "Scanning Stopped"
                bleRes.stopResult = stopResult
                self.bleCallback(bleRes)
            }
        } else{
            var bleRes: Qaul_Sys_Ble_Ble = Qaul_Sys_Ble_Ble()
            var stopResult = Qaul_Sys_Ble_BleStopResult()
            stopResult.success = false
            stopResult.errorMessage = "Advertisement & Scanning is not Running"
            bleRes.stopResult = stopResult
            bleCallback(bleRes)
        }
    }
    
    /**
     * This Method Will Start BLEService
     */
    private func startService() {
//        if (isBleScanConditionSatisfy()) {
            if !bleService.isRunning() {
                bleService.start()
//                Handler(Looper.myLooper()!!).postDelayed({
                    startAdvertiseAndCallback()
                    startScanAndCallback()
//                }, 500)
            } else {
                if (bleService.isAdvertiserRunning()) {
//                    AppLog.e(TAG, "Already Started")
                    var bleRes: Qaul_Sys_Ble_Ble = Qaul_Sys_Ble_Ble()
                    var startResult = Qaul_Sys_Ble_BleStartResult()
                    startResult.success = true
                    startResult.noRights = false
                    startResult.errorMessage = "Advertisement already Started"
                    startResult.unknownError = false
                    bleRes.startResult = startResult
                    bleCallback(bleRes)

                } else {
                    startAdvertiseAndCallback()
                }

                if (bleService.isScanRunning()) {
                    //AppLog.e(TAG, "Scan Already Started")
                    var bleRes: Qaul_Sys_Ble_Ble = Qaul_Sys_Ble_Ble()
                    var startResult = Qaul_Sys_Ble_BleStartResult()
                    startResult.success = true
                    startResult.noRights = false
                    startResult.errorMessage = "Scanning already Started"
                    startResult.unknownError = false
                    bleRes.startResult = startResult
                    bleCallback(bleRes)
                } else {
                    startScanAndCallback()
                }
            }
//        }
    }
    
    /**
     * This Method Will Assign Callback & Data to Start Scan and Receive Callback
     */
    private func startScanAndCallback() {
        
        bleService.startScan { status, errorText, unknownError in
            var bleRes: Qaul_Sys_Ble_Ble = Qaul_Sys_Ble_Ble()
            var startResult = Qaul_Sys_Ble_BleStartResult()
            startResult.success = status
            startResult.noRights = false
            startResult.errorMessage = status ? "Scanning Started" : errorText
            startResult.unknownError = unknownError
            bleRes.startResult = startResult
            self.bleCallback(bleRes)
        }
        
//        BleService.bleService?.startScan(
//            object : BleService.BleScanCallBack {
//                override func startScanRes(
//                    status: Bool,
//                    errorText: String,
//                    unknownError: Bool
//                ) {
//                    var bleRes: Qaul_Sys_Ble_Ble = Qaul_Sys_Ble_Ble()
//                    var startResult = Qaul_Sys_Ble_BleStartResult()
//                    startResult.success = status
//                    startResult.noRights = false
//                    startResult.errorMessage = errorText
//                    startResult.unknownError = unknownError
//                    bleRes.startResult = startResult.build()
//                    bleCallback?.bleResponse(ble = bleRes.build())
//                }
//
//                override func stopScanRes(status: Bool, errorText: String) {
//                    var bleRes: Qaul_Sys_Ble_Ble = Qaul_Sys_Ble_Ble()
//                    var stopResult = Qaul_Sys_Ble_BleStopResult()
//                    stopResult.success = status
//                    stopResult.errorMessage = errorText
//                    bleRes.stopResult = stopResult.build()
//                    bleCallback?.bleResponse(ble = bleRes.build())
//                }
//
//                override func deviceFound(bleDevice: BLEScanDevice) {
//                    var bleRes: Qaul_Sys_Ble_Ble = Qaul_Sys_Ble_Ble()
//                    var scanResult = Qaul_Sys_Ble_BleScanResult()
//                    scanResult.mac = bleDevice.macAddress
//                    scanResult.name = bleDevice.name
//                    scanResult.timestamp = bleDevice.lastFoundTime.toString()
//                    scanResult.rssi = bleDevice.deviceRSSI
//                    scanResult.isConnectable = bleDevice.isConnectable
//                    scanResult.isInTheRange = true
//                    scanResult.qaulId = ByteString.copyFrom(bleDevice.qaulId)
//                    bleRes.scanResult = scanResult.build()
//                    bleCallback?.bleResponse(ble = bleRes.build())
//                }
//
//                override func deviceOutOfRange(bleDevice: BLEScanDevice) {
//                    AppLog.e(TAG, "${bleDevice.macAddress} out of range")
//                    var bleRes: Qaul_Sys_Ble_Ble = Qaul_Sys_Ble_Ble()
//                    var scanResult = Qaul_Sys_Ble_BleScanResult()
//                    do {
//                        scanResult.mac = bleDevice.macAddress
//                        scanResult.name = bleDevice.name!!
//                        scanResult.timestamp = bleDevice.lastFoundTime.toString()
//                        scanResult.rssi = bleDevice.deviceRSSI
//                        scanResult.isConnectable = bleDevice.isConnectable
//                        scanResult.isInTheRange = false
//                        scanResult.qaulId = ByteString.copyFrom(bleDevice.qaulId)
//                        bleRes.scanResult = scanResult.build()
//                        bleCallback?.bleResponse(ble = bleRes.build())
//                    } catch (e: Exception) {
//                        e.printStackTrace()
//                    }
//                }
//
//                override func onMessageSent(id: String, success: Bool, data: Data) {
//                    var bleRes: Qaul_Sys_Ble_Ble = Qaul_Sys_Ble_Ble()
//                    var directSendResult = Qaul_Sys_Ble_BleDirectSendResult
//                    if (success) {
//                        directSendResult.errorMessage = "Successfully sent"
//                    } else {
//                        directSendResult.errorMessage = "Connection not established. Please try again."
//                    }
//                    directSendResult.success = success
//                    directSendResult.id = id
//                    bleRes.directSendResult = directSendResult.build()
//                    bleCallback?.bleResponse(ble = bleRes.build())
//                }
//            }
//        )
    }
    
    /**
     * This Method Will Assign Callback & Data to Start Advertiser and Receive Callback
     */
    private func startAdvertiseAndCallback() {
        bleService.qaulId = self.qaulId
            bleService.startAdvertise { status, errorText, unknownError in
                var bleRes: Qaul_Sys_Ble_Ble = Qaul_Sys_Ble_Ble()
                var startResult = Qaul_Sys_Ble_BleStartResult()
                startResult.success = status
                startResult.noRights = false
                startResult.errorMessage = status ? "Advertisement successful" : errorText
                startResult.unknownError = unknownError
                bleRes.startResult = startResult
                self.bleCallback(bleRes)
            }
        
//            BleService.bleService?.startAdvertise(
//                qaul_id = qaulId!!, mode = advertMode,
//                object : BleService.BleAdvertiseCallback {
//                    override func startAdvertiseRes(
//                        status: Bool,
//                        errorText: Bool,
//                        unknownError: Bool
//                    ) {
//                        var bleRes: Qaul_Sys_Ble_Ble = Qaul_Sys_Ble_Ble()
//                        var startResult = Qaul_Sys_Ble_BleStartResult()
//                        startResult.success = status
//                        startResult.noRights = false
//                        startResult.errorMessage = errorText
//                        startResult.unknownError = unknownError
//                        bleRes.startResult = startResult
//                        bleCallback(bleRes)
//                    }

//                    override func stopAdvertiseRes(status: Bool, errorText: String) {
//                        var bleRes: Qaul_Sys_Ble_Ble = Qaul_Sys_Ble_Ble()
//                        var startResult = Qaul_Sys_Ble_BleStopResult()
//                        stopResult.success = status
//                        stopResult.errorMessage = errorText
//                        bleRes.stopResult = stopResult
//                        bleCallback(bleRes)
//                    }

//                    override func onMessageReceived(bleDevice: BLEScanDevice, message: Data) {
//                        var bleRes: Qaul_Sys_Ble_Ble = Qaul_Sys_Ble_Ble()
//                        var directReceived = Qaul_Sys_Ble_BleDirectReceived
//                        var msgData = String(message).removeSuffix("$$")
//                            .removePrefix("$$")
//                        var msgObject = Gson().fromJson(msgData, Message::class.java)
//                        directReceived.from = bleDevice.macAddress
//                        directReceived.mode = BleOuterClass.BleMode.low_latency
//                        directReceived.qaulId = ByteString.copyFrom(bleDevice.qaulId)
//                        directReceived.data = ByteString.copyFrom(msgObject.message, Charset.defaultCharset())
//                        bleRes.directReceived = directReceived
//                        bleCallback(bleRes)
//                    }
//                }
//            )
//        }
    }
    
    /**
     * Checks if BLE Regarding All the Requirements Are Satisfies or Not
     */
    private func isBleScanConditionSatisfy() -> Bool {
        var isBleScanConditionSatisfy = true
        if (!isBLeSupported()) {
            //AppLog.e(TAG, "isBLeSupport : false")
            //RemoteLog[context]!!.addDebugLog("$TAG:isBLeSupport : false")
            isBleScanConditionSatisfy = false
        }
       
        if (!isBluetoothPermissionAllowed()) {
//            AppLog.e(
//                TAG,
//                "isBluetoothPermissionGranted() : false"
//            )
//            RemoteLog[context]!!.addDebugLog("$TAG:isBluetoothPermissionGranted() : false")
            isBleScanConditionSatisfy = false
            enableBlePermission()
            return false
        }
        
       
        if (!isBluetoothEnable()) {
//            AppLog.e(TAG, "isBluetoothEnable : false")
//            RemoteLog[context]!!.addDebugLog("$TAG:isBluetoothEnable : false")
            isBleScanConditionSatisfy = false
            enableBluetooth()
            return false
        }
        return isBleScanConditionSatisfy
    }
    
    /**
     * Request User to Allow Bluetooth Permissions for Android 12 & Above
     */
    private func enableBlePermission()
    {
        var message = ""
        var title = ""
        if #available(iOS 13.0, *) {
            if (CBCentralManager().authorization != .allowedAlways) {   //System will automatically ask user to turn on iOS system Bluetooth if this returns false
                title = "Bluetooth permission is currently disabled for the application. Enable Bluetooth from the application settings."
                message = ""
            }
        } else {
            let appName = (Bundle.main.infoDictionary?["CFBundleName"] as? String) ?? "QaulBLE"
            title = "\(appName) would like to use Bluetooth for new connections"
            message = "You can allow new connections in Settings"
        }
        
        let alertController = UIAlertController (title: title, message: message, preferredStyle: .alert)
              let settingsAction = UIAlertAction(title: "Settings", style: .default) { (_) -> Void in
                  guard let settingsUrl = URL(string: UIApplication.openSettingsURLString) else { return }
                  if UIApplication.shared.canOpenURL(settingsUrl) {
                      UIApplication.shared.open(settingsUrl, completionHandler: { (success) in
                          print("Settings opened: \(success)") // Prints true
                      })
                  }
              }
              alertController.addAction(settingsAction)
              let cancelAction = UIAlertAction(title: "Cancel", style: .default, handler: nil)
              alertController.addAction(cancelAction)
        navigationController?.topViewController?.present(alertController, animated: true, completion: nil)
    }
    
    /**
     * Request User to Enable Bluetooth
     */
    private func enableBluetooth() {
        var message = ""
        var title = ""
        if #available(iOS 13.0, *) {
            if (CBCentralManager().authorization != .allowedAlways) {   //System will automatically ask user to turn on iOS system Bluetooth if this returns false
                title = "Bluetooth permission is currently disabled for the application. Enable Bluetooth from the application settings."
                message = ""
            }
        } else {
            let appName = (Bundle.main.infoDictionary?["CFBundleName"] as? String) ?? "QaulBLE"
            title = "\(appName) would like to use Bluetooth for new connections"
            message = "You can allow new connections in Settings"
        }
        
        let alertController = UIAlertController (title: title, message: message, preferredStyle: .alert)
              let settingsAction = UIAlertAction(title: "Settings", style: .default) { (_) -> Void in
                  guard let settingsUrl = URL(string: UIApplication.openSettingsURLString) else { return }
                  if UIApplication.shared.canOpenURL(settingsUrl) {
                      UIApplication.shared.open(settingsUrl, completionHandler: { (success) in
                          print("Settings opened: \(success)") // Prints true
                      })
                  }
              }
              alertController.addAction(settingsAction)
//              let cancelAction = UIAlertAction(title: "Cancel", style: .default, handler: nil)
//              alertController.addAction(cancelAction)
        navigationcontroller.topViewController?.present(alertController, animated: true, completion: nil)
    }
    
    
    func onResult(requestCode: Int, status: Bool) {
//        if !status {
//                if  (requestCode = BLE_PERMISSION_REQ_CODE_12) {
//                        errorText = "BLE permissions are not granted"
//                        noRights = true
//                    }
//
//                 else if requestCode = REQUEST_ENABLE_BT  {
//                        errorText = "Bluetooth is not enabled"
//                        noRights = false
//                    }
//
//                var bleRes: Qaul_Sys_Ble_Ble = Qaul_Sys_Ble_Ble()
//                var startResult = Qaul_Sys_Ble_BleStartResult()
//                startResult.success = false
//                startResult.noRights = noRights
//                startResult.errorMessage = errorText
//                startResult.unknownError = false
//                bleRes.startResult = startResult.build()
//                bleCallback(bleRes)
//            }
//            else {
//                startService()
//            }
    }
}

extension BleWrapperClass : CBCentralManagerDelegate {
    func centralManagerDidUpdateState(_ central: CBCentralManager) {
        
    }
}

//extension Data {
//    var bytes: [UInt8] {
//        return [UInt8](self)
//    }
//}

public extension UIDevice {

    static let modelName: String = {
        var systemInfo = utsname()
        uname(&systemInfo)
        let machineMirror = Mirror(reflecting: systemInfo.machine)
        let identifier = machineMirror.children.reduce("") { identifier, element in
            guard let value = element.value as? Int8, value != 0 else { return identifier }
            return identifier + String(UnicodeScalar(UInt8(value)))
        }

        func mapToDevice(identifier: String) -> String { // swiftlint:disable:this cyclomatic_complexity
            #if os(iOS)
            switch identifier {
            case "iPod5,1":                                       return "iPod touch (5th generation)"
            case "iPod7,1":                                       return "iPod touch (6th generation)"
            case "iPod9,1":                                       return "iPod touch (7th generation)"
            case "iPhone3,1", "iPhone3,2", "iPhone3,3":           return "iPhone 4"
            case "iPhone4,1":                                     return "iPhone 4s"
            case "iPhone5,1", "iPhone5,2":                        return "iPhone 5"
            case "iPhone5,3", "iPhone5,4":                        return "iPhone 5c"
            case "iPhone6,1", "iPhone6,2":                        return "iPhone 5s"
            case "iPhone7,2":                                     return "iPhone 6"
            case "iPhone7,1":                                     return "iPhone 6 Plus"
            case "iPhone8,1":                                     return "iPhone 6s"
            case "iPhone8,2":                                     return "iPhone 6s Plus"
            case "iPhone8,4":                                     return "iPhone SE"
            case "iPhone9,1", "iPhone9,3":                        return "iPhone 7"
            case "iPhone9,2", "iPhone9,4":                        return "iPhone 7 Plus"
            case "iPhone10,1", "iPhone10,4":                      return "iPhone 8"
            case "iPhone10,2", "iPhone10,5":                      return "iPhone 8 Plus"
            case "iPhone10,3", "iPhone10,6":                      return "iPhone X"
            case "iPhone11,2":                                    return "iPhone XS"
            case "iPhone11,4", "iPhone11,6":                      return "iPhone XS Max"
            case "iPhone11,8":                                    return "iPhone XR"
            case "iPhone12,1":                                    return "iPhone 11"
            case "iPhone12,3":                                    return "iPhone 11 Pro"
            case "iPhone12,5":                                    return "iPhone 11 Pro Max"
            case "iPhone12,8":                                    return "iPhone SE (2nd generation)"
            case "iPhone13,1":                                    return "iPhone 12 mini"
            case "iPhone13,2":                                    return "iPhone 12"
            case "iPhone13,3":                                    return "iPhone 12 Pro"
            case "iPhone13,4":                                    return "iPhone 12 Pro Max"
            case "iPhone14,4":                                    return "iPhone 13 mini"
            case "iPhone14,5":                                    return "iPhone 13"
            case "iPhone14,2":                                    return "iPhone 13 Pro"
            case "iPhone14,3":                                    return "iPhone 13 Pro Max"
            case "iPad2,1", "iPad2,2", "iPad2,3", "iPad2,4":      return "iPad 2"
            case "iPad3,1", "iPad3,2", "iPad3,3":                 return "iPad (3rd generation)"
            case "iPad3,4", "iPad3,5", "iPad3,6":                 return "iPad (4th generation)"
            case "iPad6,11", "iPad6,12":                          return "iPad (5th generation)"
            case "iPad7,5", "iPad7,6":                            return "iPad (6th generation)"
            case "iPad7,11", "iPad7,12":                          return "iPad (7th generation)"
            case "iPad11,6", "iPad11,7":                          return "iPad (8th generation)"
            case "iPad12,1", "iPad12,2":                          return "iPad (9th generation)"
            case "iPad4,1", "iPad4,2", "iPad4,3":                 return "iPad Air"
            case "iPad5,3", "iPad5,4":                            return "iPad Air 2"
            case "iPad11,3", "iPad11,4":                          return "iPad Air (3rd generation)"
            case "iPad13,1", "iPad13,2":                          return "iPad Air (4th generation)"
            case "iPad2,5", "iPad2,6", "iPad2,7":                 return "iPad mini"
            case "iPad4,4", "iPad4,5", "iPad4,6":                 return "iPad mini 2"
            case "iPad4,7", "iPad4,8", "iPad4,9":                 return "iPad mini 3"
            case "iPad5,1", "iPad5,2":                            return "iPad mini 4"
            case "iPad11,1", "iPad11,2":                          return "iPad mini (5th generation)"
            case "iPad14,1", "iPad14,2":                          return "iPad mini (6th generation)"
            case "iPad6,3", "iPad6,4":                            return "iPad Pro (9.7-inch)"
            case "iPad7,3", "iPad7,4":                            return "iPad Pro (10.5-inch)"
            case "iPad8,1", "iPad8,2", "iPad8,3", "iPad8,4":      return "iPad Pro (11-inch) (1st generation)"
            case "iPad8,9", "iPad8,10":                           return "iPad Pro (11-inch) (2nd generation)"
            case "iPad13,4", "iPad13,5", "iPad13,6", "iPad13,7":  return "iPad Pro (11-inch) (3rd generation)"
            case "iPad6,7", "iPad6,8":                            return "iPad Pro (12.9-inch) (1st generation)"
            case "iPad7,1", "iPad7,2":                            return "iPad Pro (12.9-inch) (2nd generation)"
            case "iPad8,5", "iPad8,6", "iPad8,7", "iPad8,8":      return "iPad Pro (12.9-inch) (3rd generation)"
            case "iPad8,11", "iPad8,12":                          return "iPad Pro (12.9-inch) (4th generation)"
            case "iPad13,8", "iPad13,9", "iPad13,10", "iPad13,11":return "iPad Pro (12.9-inch) (5th generation)"
            case "AppleTV5,3":                                    return "Apple TV"
            case "AppleTV6,2":                                    return "Apple TV 4K"
            case "AudioAccessory1,1":                             return "HomePod"
            case "AudioAccessory5,1":                             return "HomePod mini"
            case "i386", "x86_64", "arm64":                                return "Simulator \(mapToDevice(identifier: ProcessInfo().environment["SIMULATOR_MODEL_IDENTIFIER"] ?? "iOS"))"
            default:                                              return identifier
            }
            #elseif os(tvOS)
            switch identifier {
            case "AppleTV5,3": return "Apple TV 4"
            case "AppleTV6,2": return "Apple TV 4K"
            case "i386", "x86_64": return "Simulator \(mapToDevice(identifier: ProcessInfo().environment["SIMULATOR_MODEL_IDENTIFIER"] ?? "tvOS"))"
            default: return identifier
            }
            #endif
        }

        return mapToDevice(identifier: identifier)
    }()

}
