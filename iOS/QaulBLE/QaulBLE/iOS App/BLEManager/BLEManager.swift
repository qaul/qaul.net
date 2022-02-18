//
//  BLEManager.swift
//  VSNEW
//
//  Created by Nikhil Jobanputra on 14/05/2021.
//  Copyright © 2021 Bluepixel. All rights reserved.
//

import UIKit
import CoreBluetooth
import UserNotifications

let bleManager = BLEManager()

public class BLEManager: NSObject {
    
    //MARK:-  Variables
    var cbManager: CBCentralManager!
    private let beaconOperationsQueue = DispatchQueue(label: "beacon_operations_queue")
    var isScanningStart = false
    
    static let shared = BLEManager()
    
    public var IsBluetoothOn = false

    private var arrOfBLEDevices = [String: BLEDevice]()
    
    private var timer: Timer!
    private var timerLostBeacon: Timer!
    private var timerConnectionTimeOut: Timer!
    
    private var services = [CBUUID]()

    public typealias complitionBLock = ((BLEDevice) -> Void)
    private var complition: complitionBLock!
    
    public typealias connectComplitionBLock = ((Bool, String, CBPeripheral?) -> Void)
    private var connectComplition: connectComplitionBLock?
    private var connectablePeripheral: CBPeripheral?
    
    public typealias OutRangecomplitionBLock = ((BLEDevice) -> Void)
    private var outRangeComplition: OutRangecomplitionBLock!

    //BLE-Updater Device
    typealias DisconnectDeviceBlock = (Bool) -> Void
    private var disConnectComplitionBlock: DisconnectDeviceBlock!
    
    typealias AutoDisconnect = (BLEDevice) -> Void
    private var autoDisconnect: AutoDisconnect?
    
    public typealias complitionBluetoothState = ((_ isOn: Bool) -> Void)
    private var complitionBLEState: complitionBluetoothState?
    
    func autoDisconnect(myComplition: @escaping AutoDisconnect) {
        
        autoDisconnect = myComplition
    }
    func disConnect(device:CBPeripheral, myComplition: @escaping DisconnectDeviceBlock) {
        
        self.cbManager.cancelPeripheralConnection(device)
        disConnectComplitionBlock = myComplition
    }
    override init() {
        super.init()

        self.cbManager = CBCentralManager(delegate: self, queue: nil, options: [CBCentralManagerOptionRestoreIdentifierKey: ""/*ALERT_TITLE*/])
        
        self.timer = Timer.scheduledTimer(timeInterval: 2.0, target: self, selector: #selector(getAccurateRssiBeacons), userInfo: nil, repeats: true)

        //To consider beacon out range call it every 3 seconds
        self.timerLostBeacon = Timer.scheduledTimer(timeInterval: 3.0, target: self, selector: #selector(ConsiderBeaconOutRange), userInfo: nil, repeats: true)
        
        IsBluetoothOn = cbManager.state == .poweredOn
    }
    
    //MARK:-  Usable Fuctions
    // TO start scanning Ble Devices
    public func StartScanning(WithServices: [String]?, mycomplition: @escaping complitionBLock) {
    
        self.complition = mycomplition
        
        self.services.removeAll()
        if let services = WithServices {
            for uuid in services {
                self.services.append(CBUUID(string: uuid))
            }
        }
        
        self.perform(#selector(self.StartScan), with: nil, afterDelay: 1.0)
    }
    
    @objc private func StartScan() {
    
        if isScanningStart == false {
            
            self.clearAllArays()
            if self.IsBluetoothOn {
    
                if self.cbManager == nil {
                    
                    self.cbManager = CBCentralManager(delegate: self, queue: beaconOperationsQueue, options: [CBCentralManagerOptionRestoreIdentifierKey: ""/*ALERT_TITLE*/])
                }
                self.cbManager.delegate = self
                self.cbManager.scanForPeripherals(withServices: self.services, options: [CBCentralManagerScanOptionAllowDuplicatesKey: true])
                self.isScanningStart = true
            }
        }
    }
    
    // To Clear all variables beacuse when we rescan Ble devices so its not add duplicate devices
    private func clearAllArays() {
        
        self.arrOfBLEDevices.removeAll()
    }
    public func connect(peripheral: CBPeripheral, myComplition: @escaping connectComplitionBLock) {
        
        self.timerConnectionTimeOut = Timer.scheduledTimer(timeInterval: 10, target: self, selector: #selector(connectionTimeOver(timer:)), userInfo: nil, repeats: false)
        self.connectComplition = myComplition
        self.connectablePeripheral = peripheral
        self.cbManager.connect(peripheral, options: nil)
    }
    @objc fileprivate func connectionTimeOver(timer: Timer) {
        
        self.connectComplition?(false, "Connection timeout", nil)
        self.timerConnectionTimeOut.invalidate()
        if let peri = connectablePeripheral {
            
            self.cbManager.cancelPeripheralConnection(peri)
        }
        self.connectComplition = nil
    }
    public func retrivePeripheral(device: BLEDevice,complition: ((BLEDevice) -> Void)) {
        
        let peripherals = cbManager.retrievePeripherals(withIdentifiers: [UUID(uuidString: device.uuid)!])
        
        if peripherals.count == 1 {
            
            device.peripheral = peripherals[0]
            complition(device)
        }
    }
    public func getBluetoothState(myComplition: @escaping complitionBluetoothState) {
        
        self.complitionBLEState = myComplition
    }
    public func StopScan() {
        
        self.clearAllArays()
        self.cbManager.stopScan()
        self.isScanningStart = false
    }
    
    //MARK:-  Custom Functions
   
    @objc private func getAccurateRssiBeacons() {
        
        for device in self.arrOfBLEDevices.values {
            
            device.arrOfRSSI = device.arrOfRSSI.sorted()
           
            if device.arrOfRSSI.count > 15 {
            
                let upperDeleteCount = (device.arrOfRSSI.count * 20) / 100
                let lowerDeleteCount = (device.arrOfRSSI.count * 20) / 100
                
                for _ in 0..<upperDeleteCount {
                    
                    device.arrOfRSSI.removeFirst()
                }
                
                for _ in 0..<lowerDeleteCount {
                    
                    device.arrOfRSSI.removeLast()
                }
            }
            
            let totalRssis = device.arrOfRSSI.reduce(0, +)
            device.avgRSSI = totalRssis / device.arrOfRSSI.count
            let newDevice = calculateDistanceInMeter(device: device)
    
            complition(newDevice)
        }
    }
    public func getOutRangeDevice(mycomplition: @escaping OutRangecomplitionBLock) {
        
        outRangeComplition = mycomplition
    }
    //Check device is out range or not
    @objc private func ConsiderBeaconOutRange() {
        
        //logfile.appendNewText(text: "Beacon outrange count method call")
        for device in arrOfBLEDevices.values {

            device.count += 1
            device.inRange = false
           // logfile.appendNewText(text: "Device count: \(device.count) isInRange: \(device.inRange) isGrayed: \(device.isGrayed)")
            
            //Outrange time and gray time set from setting screen
            if device.count >= 4 {
                
                if device.count >= 4 && device.inRange == false {
                    
                   // logfile.appendNewText(text: "Device is out of range")
                    self.arrOfBLEDevices.removeValue(forKey: device.uuid)
                    if self.outRangeComplition != nil {
                        
                        self.outRangeComplition(device)
                    }
                }
            } else {
//                device.isGrayed = false
            }
        }
    }
    

    
    //Calculate distance in meter on rssi and txpower
    private func calculateDistanceInMeter(device: BLEDevice) -> BLEDevice {
        
        let txPower = -59
        let strDistance = calculatorDistance(dividValue: -60, avarageRssi: device.avgRSSI)
        device.distanceInMeter = Float(strDistance)
        
        return device
    }
    private func calculatorDistance(dividValue: Int, avarageRssi: Int) -> String {
        
        var accuracy = 0.0
        
        if (avarageRssi == 0) {
            accuracy = -1.0 // if we cannot determine accuracy, return -1.
        }
        
        let ratio = Double(avarageRssi) * 1.0 / Double(dividValue)
        if (ratio < 1.0) {
            accuracy = pow(ratio, 10)
        } else {
            accuracy = (0.89976) * pow(ratio, 7.7095) + 0.111
        }
        
        var distanceFloatDisplay = "0.0"
        //print("actualyValue:->\(accuracy)")
        if (accuracy < 1) {
            distanceFloatDisplay =  "\(String(format: "%.1f", Double(accuracy)))"
        } else if (accuracy > 1 && accuracy < 4) {
            distanceFloatDisplay = "\(String(format: "%.1f", Double(accuracy)))"
        } else {
            distanceFloatDisplay = "\(String(format: "%.1f", Double(accuracy)))"
        }
        return distanceFloatDisplay
    }
}

//MARK:-  CBCentralManagerDelegate Methods
extension BLEManager: CBCentralManagerDelegate {
    
    // For checking bluetooth is on or off
    public func centralManagerDidUpdateState(_ central: CBCentralManager) {
        
        if central.state == .poweredOn {
            
            IsBluetoothOn = true
//            NotificationCenter.default.post(name: .bleOnOff, object: IsBluetoothOn, userInfo: nil)
            
        } else {
            
            isScanningStart = false
            IsBluetoothOn = false
//            NotificationCenter.default.post(name: .bleOnOff, object: IsBluetoothOn, userInfo: nil)
        }
    }
    
    // To get nearest bleDevices
    public func centralManager(_ central: CBCentralManager, didDiscover peripheral: CBPeripheral, advertisementData: [String : Any], rssi RSSI: NSNumber) {
        
        let peripheralName = peripheral.name ?? ""
       // logfile.appendNewText(text: "Peri:->>>>>> \(peripheral), \(advertisementData)")
       // print("Peri:->>>>>> \(peripheral), \(advertisementData)")
        let localName = advertisementData.peripheralname() ?? ""
//        print("Peri:->>>>>> \(peripheralName)")
        
        let localUUID = advertisementData.standardServiceid() ?? ""
//        print("localUUID:->>>>>> \(localUUID)")
      
        if localUUID == SERVICES.SERVICE_UUID {
            if let existingBLE = self.arrOfBLEDevices[peripheral.identifier.uuidString] {
                
                //logfile.appendNewText(text: "Existing Device Found : \(existingBLE.peripheral)")
                if existingBLE.arrOfRSSI.count > 15 {
                    
                    existingBLE.arrOfRSSI.remove(at: 0)
                }
                
                existingBLE.name      = peripheral.name
                existingBLE.localName = advertisementData.peripheralname()
                existingBLE.arrOfRSSI.append(RSSI.intValue)
                
                existingBLE.count = 0
                existingBLE.inRange = true
                self.arrOfBLEDevices[peripheral.identifier.uuidString] = existingBLE
                
            } else {
                let foundDevice = BLEDevice(peripheral: peripheral, advertisementdata: advertisementData, Rssi: RSSI.intValue)
               // logfile.appendNewText(text: "Device Found FirstTime \(foundDevice.peripheral)")
                self.arrOfBLEDevices[foundDevice.uuid] = foundDevice
                if self.complition != nil {
                    self.complition(foundDevice)
                }
            }
        }
    }
    public func centralManager(_ central: CBCentralManager, willRestoreState dict: [String : Any]) {
        
        //        print(dict)
    }
    public func centralManager(_ central: CBCentralManager, didConnect peripheral: CBPeripheral) {
        
        if peripheral.identifier == connectablePeripheral?.identifier && connectComplition != nil {
            
            connectComplition?(true, "Connect completed", peripheral)
            connectComplition = nil
            timerConnectionTimeOut.invalidate()
        }
    }
    public func centralManager(_ central: CBCentralManager, didFailToConnect peripheral: CBPeripheral, error: Error?) {
        
        if peripheral.identifier == connectablePeripheral?.identifier && connectComplition != nil {
            connectComplition?(false, error!.localizedDescription, peripheral)
            connectComplition = nil
            timerConnectionTimeOut.invalidate()
        }
    }
    public func centralManager(_ central: CBCentralManager, didDisconnectPeripheral peripheral: CBPeripheral, error: Error?) {
        
        print(" DisconnectPeripheral :->\(peripheral)")
        print("didDisconnectPeripheral", peripheral.state)
        
        let dict:NSDictionary = ["peripheralDevice":peripheral,"audtoDisconnect":true]
//        NotificationCenter.default.post(name: Notification.Name(kBLEDisconnect), object: dict)
        
//        NotificationCenter.default.post(name: .bluetoothDisconnect, object: false, userInfo: nil)
//        if disConnectComplitionBlock != nil{
//
//            disConnectComplitionBlock(true)
//        }
//        if autoDisconnect != nil {
//
//            let index = arrOfBLEDevices.firstIndex(where: { (device) -> Bool in
//
//                return device.value.uuid.uppercased() == peripheral.identifier.uuidString.uppercased()
//            })
//
//            if let i = index {
//                autoDisconnect?(arrOfBLEDevices[i].value)
//                arrOfBLEDevices.remove(at: i)
//            }
//        }
    }
}
