//
//  BLEPeripheral.swift
//  QaulBLE
//
//  Created by BAPS on 27/01/22.
//


import Foundation
import CoreBluetooth
import ObjectMapper

//public let kTRANSFER_SERVICE_UUID        = "33c0ac57-d316-43ec-a883-691fc200e37b".uppercased()
//public let kTRANSFER_CHARACTERISTIC_UUID = "aec5e807-83e9-4fce-a5a9-3790cd63a977".uppercased()

var recivemessage = ""

let bLEPeripheral = BLEPeripheral()

public class BLEPeripheral: NSObject, CBPeripheralManagerDelegate, CBPeripheralDelegate {
    
    static let shared = BLEPeripheral()
        
    //((status: Bool, errorText: String, unknownError: Bool) -> Void)
    public typealias startAdvertiseRes = ((Bool,  String, Bool) -> Void)
    private var StartbleAdvertiseCallback: startAdvertiseRes!
    
    //((status: Bool, errorText: String) -> Void)
    public typealias stopAdvertiseRes = ((Bool,  String) -> Void)
    private var StopbleAdvertiseCallback: stopAdvertiseRes!
    
    var peripheralManager: CBPeripheralManager
    private let beaconOperationsQueue = DispatchQueue(label: "beacon_operations_queue")
    private var peripheralName: String?
    private var connectTarget: CBPeripheral?
    
    private var servicesIDs = CBUUID()
    
    var isStartAdvertising = false
    
    override init() {
        
        self.peripheralManager = CBPeripheralManager(delegate: nil, queue: beaconOperationsQueue, options: nil)
        super.init()

        self.peripheralManager.delegate = self
    }
    
    public func startAdvertising(serviceID: String, name: String, bleCallback: @escaping startAdvertiseRes) {
        
        StartbleAdvertiseCallback = bleCallback
        
        let valueData = bleService.qaulId //.data(using: .utf8)
        
        self.peripheralName = name
        
        servicesIDs = CBUUID(string: serviceID)
        
        let CustomChar = CBMutableCharacteristic(type: CBUUID(string: CHAR.READ_CHAR), properties: [.read], value: valueData, permissions: [.readable])
        let CustomChar2 = CBMutableCharacteristic(type: CBUUID(string: CHAR.MSG_CHAR), properties: [ .write], value: nil, permissions: [.writeable])
//        let CustomChar2 = CBMutableCharacteristic(type:  CBUUID(string: CHAR.MSG_CHAR), properties: [CBCharacteristicProperties.read,CBCharacteristicProperties.writeWithoutResponse,CBCharacteristicProperties.notify], value: nil, permissions: [CBAttributePermissions.readable, CBAttributePermissions.writeable])
        
//        CBMutableCharacteristic(type: CBUUID(string: CHAR.MSG_CHAR), properties: [.notify, .write, .read], value: nil, permissions: [.readable, .writeable])
        
        let myService = CBMutableService(type: servicesIDs, primary: true)
        myService.characteristics = [CustomChar , CustomChar2]
        
        peripheralManager.removeAllServices()
        peripheralManager.add(myService)
//        self.perform(#selector(self.startAdvertise), with: nil, afterDelay: 1.0)
        startAdvertise()
    }
    @objc fileprivate func startAdvertise() {
        peripheralManager.startAdvertising([
            CBAdvertisementDataServiceUUIDsKey: [servicesIDs],
            CBAdvertisementDataLocalNameKey: self.peripheralName!])
    }
    
    public func stopAdvertising(bleCallback: @escaping stopAdvertiseRes) {
        StopbleAdvertiseCallback = bleCallback
        self.peripheralManager.stopAdvertising()
        StopbleAdvertiseCallback(true , "")
    }
    
    public func peripheralManagerDidUpdateState(_ peripheral: CBPeripheralManager) {
        
        if peripheral.state == .poweredOn {
            
            let dict = ["isBluetoothOn": true, "comeFrom": true] as [String : Any]
            NotificationCenter.default.post(name: Notification.Name("bleState"), object: dict)
            print("Powered on, start advertising")
            
        } else {
            
            self.peripheralManager.stopAdvertising()
            let dict = ["isBluetoothOn": true, "comeFrom": true] as [String : Any]
            NotificationCenter.default.post(name: Notification.Name("bleState"), object: dict)
        }
    }
    public func peripheralManagerDidStartAdvertising(_ peripheral: CBPeripheralManager, error: Error?) {
        
        if error == nil{
            self.isStartAdvertising =  true
            print("Start Advertising.....")
            
            StartbleAdvertiseCallback(true , "" , false)
        }else{
            self.isStartAdvertising =  false
            let errstring = error?.localizedDescription ?? "Un knownError"
            StartbleAdvertiseCallback(false , errstring , errstring == "Un knownError" ? true : false)
        }
        
        //locManager.appendNewText(text: "Start Advertising.....")
    }
    
    public func peripheralManager(_ peripheral: CBPeripheralManager, didReceiveWrite requests: [CBATTRequest]) {
//        print("data message \(requests)")
        print("data message \(String(bytes: requests[0].value!, encoding: .utf8))")
//        print(request)
        var str = String(bytes: requests[0].value!, encoding: .utf8)
        str = str?.replacingOccurrences(of: "$$", with: "")
        
        recivemessage = recivemessage + (str ?? "")
        print("data message recivemessage : \(recivemessage)")
        let messagerecive = Message(JSONString: recivemessage ?? "", context: .none)
        if messagerecive != nil {
            recivemessage = ""
            NotificationCenter.default.post(name: .GetscanMessage, object: messagerecive, userInfo: nil)
        }
    }
    
    public func peripheralManager(_ peripheral: CBPeripheralManager, didReceiveRead request: CBATTRequest) {
       
    }
}
