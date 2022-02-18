//
//  BLEDevice.swift
//  VSNEW
//
//  Created by Nikhil Jobanputra on 14/05/2021.
//  Copyright © 2021 Bluepixel. All rights reserved.
//

import UIKit
import CoreBluetooth

enum ConnectionStateProgress{
    
    case isConnected
    case none
}

public class BLEDevice: NSObject {
    
    //MARK:- Variables Declarations
    public var peripheral: CBPeripheral!
    private var AdvData: [String: Any]!
    public var inRange: Bool!
    public var count: Int!
    public var arrOfRSSI = [Int]()
    
    public var uuid: String!
    public var name: String?
    public var aliasName: String?
    public var localName: String?
    public var beaconTypeName: String?
    public var standardServiceName: String?
    public var manufacturerData: String?
    public var isConnectable: Bool?
    
    public var avgRSSI: Int!
    public var distanceInMeter: Float!
    
    public var serivces = [Service]()
    private var operationalCharacteristic: CBCharacteristic!
    
    public typealias ConnectionComlition = ((Bool, BLEDevice, Error?) -> Void)
    private var connectionComlition: ConnectionComlition!
    
    public typealias DeviceServices = ((Bool) -> Void)
    private var deviceServices: DeviceServices!
    
    
    public typealias operationComplitionBlock = ((Bool, String, Characterstic?, BLEDevice) -> Void)
    private var charComplition: operationComplitionBlock?
    
    public typealias updateCharComplitionBlock = ((Characterstic, BLEDevice) -> Void)
    private var updateCharComplition: updateCharComplitionBlock?
    
    public typealias updateRSSI = (Int) -> Void
    var getUpdateRSSI:updateRSSI!
    

    typealias GetAnyUpdateComplitionBlock = (String) -> Void
    private var getAnyUpdateComplition: GetAnyUpdateComplitionBlock!
    
    //isDeviceConencted, isCharFound, qaulId , qaulIdbyte
    public typealias GetQaulIDComplition = ((Bool, Bool, String , Data) -> Void)
    private var getQaulIDComplition: GetQaulIDComplition!
    
    //isDeviceConencted, isCharFound, qaulId , qaulIdbyte
    public typealias GetSendMessageComplition = ((Bool, Bool, String , Data) -> Void)
    private var getSendMessageComplition: GetQaulIDComplition!
    
    private var connectionTime  = Timer()
    private var readLogTime     = Timer()
    private var commandTimer     = Timer()
    
    var connectionStatus:Int = Int()
    
    public var requestType: RequestType!
    
    //var dataView = ODBDataView()
    
    public var commandQueue: [String] = [String]()
    
    public override init() {
        
    }
    //MARK:-  Functions
    init(peripheral: CBPeripheral, advertisementdata: [String: Any], Rssi: Int) {
        super.init()
        
        let exactRSSI = Rssi > -1 ? -100: Rssi < -100 ? -100: Rssi
        self.peripheral = peripheral
        self.AdvData = advertisementdata
        self.avgRSSI = exactRSSI
        self.arrOfRSSI = [exactRSSI]
        
        self.inRange = true
        self.count = 0
        self.uuid = peripheral.identifier.uuidString
        
        self.name           = peripheral.name
        self.localName       = advertisementdata.peripheralname()
        self.standardServiceName = advertisementdata.standardServiceName()
        self.isConnectable = advertisementdata.IsConnectable()
        self.manufacturerData = advertisementdata.getManufacturerData()
    }
    
    public func isConnected() -> Bool {
        
        return peripheral.state == .connected
    }
    
    private func startTimer(timeInterval: Double){
        
        self.connectionTime = Timer.scheduledTimer(timeInterval: timeInterval, target: self, selector: #selector(self.checkTimeFinish), userInfo: nil, repeats: true)
    }
    private func stopTimer(){
        
        self.connectionTime.invalidate()
    }
    @objc func checkTimeFinish(){
        
        self.connectionTime.invalidate()
        
        switch self.connectionStatus{
            
        case ConnectionStateProgress.isConnected.hashValue:
            self.connectionComlition!(false, self, nil)
            break;
        default:
            self.connectionComlition!(false, self, nil)
            print("Default run")
            break;
        }
    }
    public func retriveDevice(complition: @escaping ((BLEDevice) -> Void)) {
        
        bleManager.retrivePeripheral(device: self) { (d) in
            
            complition(d)
        }
    }
    func disConnect(myComplition: @escaping ((_ isDisconnected: Bool) -> Void)) {
        
        bleManager.disConnect(device: self.peripheral) { (isDisconnected) in
            myComplition(isDisconnected)
        }
    }
    public func connectedDevice(serviceSuccess: @escaping DeviceServices, complition: @escaping ConnectionComlition) {
        
        self.deviceServices      = serviceSuccess
        self.connectionComlition = complition
        self.startTimer(timeInterval: 20.0)
        self.connectionStatus = ConnectionStateProgress.isConnected.hashValue
        bleManager.connect(peripheral: peripheral) { (isConnected, msg, peri) in
        
            if isConnected {
                
                self.peripheral = peri
                self.peripheral.delegate = self
                self.peripheral.discoverServices(nil)
                self.stopTimer()
                self.deviceServices(true)
                DispatchQueue.main.asyncAfter(deadline: .now() + 1.5, execute: {
                    print("Complete Connected")
                    self.connectionComlition(true, self, nil)
                })
            }
        }
    }
    private func getCharsticFrom(UUID: String) -> CBCharacteristic? {
        
        var charStic: CBCharacteristic?
        for service in serivces {
            for char in service.characteristics {
                if char.uuid == UUID {
                    charStic = char.characteristic
                    break
                }
            }
        }
        return charStic
    }
    
    public func getUpdatedCharstic(myComplition: @escaping updateCharComplitionBlock) {
        updateCharComplition = myComplition
    }
    public func readRSSI(completionBlockSuccess: @escaping updateRSSI){
        
        self.getUpdateRSSI = completionBlockSuccess
        self.peripheral.readRSSI()
    }
    
    func getAnyUpdateOnDevice(myComplition: @escaping GetAnyUpdateComplitionBlock) {
        
        self.getAnyUpdateComplition = myComplition
    }
    
    func readQaulID(myComplition: @escaping GetQaulIDComplition) {
       
        self.getQaulIDComplition = myComplition
        if let char = self.getCharsticFrom(UUID: CHAR.READ_CHAR) {
            
            self.readDataWithCharacteristic(characteristic: char)
        } else {
            //Char is not found in this conected device
            //Disconnect Device here
            self.getQaulIDComplition(true, false, "Char is not found" , Data())
        }
    }
    
    func writeQaulID(message : Data ,myComplition: @escaping GetSendMessageComplition) {
        print("data send size :- \(message)")
        self.getSendMessageComplition = myComplition
        
        if let char = self.getCharsticFrom(UUID: CHAR.MSG_CHAR) {
            //self.getSendMessageComplition(true, true, "" , Data())
            
            let bytes = message.bytes
            
            if let convertString = String(bytes: bytes, encoding: .utf8) {
                print("Original Message:: \(convertString)")
            }
            let numberofinteraction = message.bytes.count / 20
            
            let remainBytes = numberofinteraction * 20
            let remainingCount = message.bytes.count - remainBytes
            
            var startIndex = 0
            for index in 1...numberofinteraction{
                var lastIndex = startIndex + 20
                if index == numberofinteraction && remainingCount == 0{
                    lastIndex = lastIndex - 1
                }
                let newBytes = Array(bytes[startIndex...lastIndex])
                let data = Data(newBytes)
                if let convertString = String(bytes: newBytes, encoding: .utf8) {
                    print("Index: \(index) :: \(convertString)")
                }
                startIndex = lastIndex
                self.writeDataWithResponse(writeData: data, characteristic: char)
            }
            if remainingCount > 0 {
                let lastIndex = startIndex + remainingCount
                let data = Data(Array(bytes[(startIndex + 1)...lastIndex - 1]))
                if let convertString = String(bytes: Array(bytes[(startIndex + 1)...lastIndex - 1]), encoding: .utf8) {
                    print("\(convertString)")
                }
                self.writeDataWithResponse(writeData: data, characteristic: char)
            }
        } else {
            //Char is not found in this conected device
            //Disconnect Device here
            self.getSendMessageComplition(true, false, "Char is not found" , Data())
        }
    }
    
}

//MARK:-  CBPeripheralDelegate Methods
extension BLEDevice: CBPeripheralDelegate {
    
    public func peripheral(_ peripheral: CBPeripheral, didReadRSSI RSSI: NSNumber, error: Error?) {
        
        if self.getUpdateRSSI != nil {
            
            self.getUpdateRSSI(RSSI.intValue)
        }
    }
    public func peripheral(_ peripheral: CBPeripheral, didDiscoverServices error: Error?) {
        
        //appDelegate.appendNewText(fileName: KTXTFName, text: "-------- didDiscoverServices call -------")
        guard let serviceArray = peripheral.services else {
            return
        }
        
        serivces.removeAll()
        for oneService in serviceArray {
            
            //appDelegate.appendNewText(fileName: KTXTFName, text: "-------- Services for -------\(oneService)")
            print("Services for \(oneService)")
            //logfile.appendNewText(text: "didDiscoverServices Found : \(oneService)")
            serivces.append(Service(service: oneService))
            peripheral.discoverCharacteristics(nil, for: oneService)
        }
    }
    
   
    public func peripheral(_ peripheral: CBPeripheral, didDiscoverCharacteristicsFor service: CBService, error: Error?) {
        
        let ser = self.serivces.filter({$0.uuid == service.uuid.uuidString})
        if ser.count == 1 {
            ser[0].characteristics.removeAll()
        }
        for chr in service.characteristics ?? [CBCharacteristic]() {
            
            if chr.properties.contains(.notify) {
                
                peripheral.setNotifyValue(true, for: chr)
            }
            if chr.properties.contains(.indicate) {
                
                peripheral.setNotifyValue(true, for: chr)
            }
            if chr.properties.contains(.broadcast) {
                
                peripheral.setNotifyValue(true, for: chr)
            }
            if chr.properties.contains(.read) {
                
            }
            if chr.properties.contains(.write) {
            }
            print("Characteristics for \(chr)")
            //logfile.appendNewText(text: "didDiscoverCharacteristicsFor Found : \(chr)")
            peripheral.discoverDescriptors(for: chr)
            ser[0].characteristics.append(Characterstic(characterstic: chr))
        }
    }
    public func peripheral(_ peripheral: CBPeripheral, didDiscoverDescriptorsFor characteristic: CBCharacteristic, error: Error?) {
        
        let ser = self.serivces.filter({$0.uuid == characteristic.service?.uuid.uuidString})
        let char = ser.first?.characteristics.filter({$0.uuid == characteristic.uuid.uuidString})
        
        if char!.count == 1 {
            char![0].descriptors.removeAll()
        }
        for des in characteristic.descriptors ?? [CBDescriptor]() {
            
            char![0].descriptors.append(Descriptor(descriptor: des))
            peripheral.readValue(for: des)
        }
    }
    
    public func peripheral(_ peripheral: CBPeripheral, didUpdateValueFor descriptor: CBDescriptor, error: Error?) {
        
        if error == nil {
            updateDescriptorInBLEDevice(descriptor: descriptor)
        }
    }
    public func peripheral(_ peripheral: CBPeripheral, didUpdateValueFor characteristic: CBCharacteristic, error: Error?) {
        
        let data = peripheral.maximumWriteValueLength(for: .withResponse)
        //peripheral.maximumWriteValueLength(for: .withResponse)
        //peripheral.maximumWriteValueLength(for: .withoutResponse)
//        print("connectTarget : \(peripheral)")
//        print("data : \(data)")
        
        //print("characteristic.uuid.uuidString:- \(characteristic.uuid.uuidString)")
        if characteristic.uuid.uuidString == CHAR.READ_CHAR {
            let updatedCharacterstic = Characterstic(characterstic: characteristic)
            if let resultDate = updatedCharacterstic.dataValue {
                
                if let str = String(bytes: resultDate.bytes, encoding: .utf8) {
                    self.getQaulIDComplition(true, true, str , resultDate)
                } else {
                    print("not a valid UTF-8 sequence")
                }
            } else {
        
            }
        } else if characteristic.uuid.uuidString == CHAR.MSG_CHAR {
            let updatedCharacterstic = Characterstic(characterstic: characteristic)
            if let resultDate = updatedCharacterstic.dataValue {
                
                if let str = String(bytes: resultDate.bytes, encoding: .utf8) {
                    self.getSendMessageComplition(true, false, str ,resultDate)
                } else {
                    print("not a valid UTF-8 sequence")
                }
            } else {
        
            }
        }
    }
    
    
    public func peripheral(_ peripheral: CBPeripheral, didWriteValueFor characteristic: CBCharacteristic, error: Error?) {
        
        print("characteristic.uuid.uuidString:- \(characteristic.uuid.uuidString)")
        if error == nil {
            
            if ((charComplition != nil) && operationalCharacteristic.uuid == characteristic.uuid) {
                //DataIn
                updateCharactersticInBLEDevice(charstic: characteristic)
                charComplition!(true, "Command Fire Successfully", Characterstic(characterstic: characteristic), self)
                charComplition = nil
            }
        }
        
        if characteristic.uuid.uuidString == CHAR.MSG_CHAR {
            let updatedCharacterstic = Characterstic(characterstic: characteristic)
            if let resultDate = updatedCharacterstic.dataValue {
                
                if let str = String(bytes: resultDate.bytes, encoding: .utf8) {
                    self.getSendMessageComplition(true, false, str ,resultDate)
                } else {
                    print("not a valid UTF-8 sequence")
                }
            } else {
        
            }
        }
    }
    
    public func peripheral(_ peripheral: CBPeripheral, didWriteValueFor descriptor: CBDescriptor, error: Error?) {
        
    }
    
    func updateCharactersticInBLEDevice(charstic: CBCharacteristic) {
        
        let serviceIndex = self.serivces.firstIndex { (ser) -> Bool in
            
            return ser.uuid == charstic.uuid.uuidString
        }
        if let index = serviceIndex {
            let charindex = self.serivces[index].characteristics.firstIndex { (char) -> Bool in
                return char.uuid == charstic.uuid.uuidString
            }
            if let i = charindex {
                self.serivces[index].characteristics[i] = Characterstic(characterstic: charstic)
            }
        }
    }
    
    func updateDescriptorInBLEDevice(descriptor: CBDescriptor) {
        
        let serviceIndex = self.serivces.firstIndex { (ser) -> Bool in
            
            return ser.uuid == descriptor.characteristic?.service?.uuid.uuidString
        }
        if let index = serviceIndex {
            
            let charindex = self.serivces[index].characteristics.firstIndex { (char) -> Bool in
                return char.uuid == descriptor.characteristic?.uuid.uuidString
            }
            if let i = charindex {
                
                let desindex = self.serivces[index].characteristics[i].descriptors.firstIndex { (des) -> Bool in
                    
                    return des.uuid == descriptor.uuid.uuidString
                }
                let descriptorValue = Descriptor(descriptor: descriptor)
                if let d = desindex {
                    
                    self.serivces[index].characteristics[i].descriptors[d] = descriptorValue
                } else {
                    
                    self.serivces[index].characteristics[i].descriptors.append(descriptorValue)
                }
                self.serivces[index].characteristics[i].characteristic = descriptor.characteristic
                if descriptorValue.type == .some(.UserDescription){
                    
                    self.serivces[index].characteristics[i].name =  DictOfCharacteristic[(descriptor.characteristic?.uuid.uuidString)!] ?? descriptor.value as? String ?? "Custom Characteristic" //.uppercased()
                }
            }
        }
    }
    
    private func writeDataWithResponse(writeData: Data, characteristic: CBCharacteristic) {
        
        self.peripheral!.writeValue(writeData, for: characteristic, type: .withResponse)
    }
    
    private func writeDataWithoutResponse(writeData: Data, characteristic: CBCharacteristic) {
        
        self.peripheral!.writeValue(writeData, for: characteristic, type: .withoutResponse)
    }
    private func readDataWithCharacteristic(characteristic: CBCharacteristic) {
        
        self.peripheral!.readValue(for: characteristic)
    }
    private func setNotifyWithCharacteristic(characteristic: CBCharacteristic) {
        
        self.peripheral!.setNotifyValue(true, for: characteristic)
    }
}

public class Service: NSObject {
    
    public var name: String?
    public var uuid: String?
    public var characteristics = [Characterstic]()
    
    public init(service: CBService) {
        super.init()
        
        name = DictOfservices[service.uuid.uuidString] ?? "Custom Service".uppercased()
        uuid = service.uuid.uuidString
    }
}

public class Characterstic: NSObject {
    
    public var characteristic: CBCharacteristic!
    public var name: String?
    public var uuid: String?
    public var stringValue: String?
    public var base64EncodedStringValue: String?
    public var hexValue: String?
    public var byteArray: [UInt8]?
    public var dataValue: Data?
    public var descriptors = [Descriptor]()
    
    init(characterstic: CBCharacteristic) {
        super.init()
        
        self.characteristic = characterstic
        self.uuid           = characterstic.uuid.uuidString
        self.name           = DictOfCharacteristic[characterstic.uuid.uuidString] ?? "Custom Characteristic" //.uppercased()
        self.stringValue    = getStringValue()
        self.hexValue       = getHexValue().uppercased()
        self.base64EncodedStringValue = characteristic.value?.base64EncodedString()
        self.dataValue      = characteristic.value
        
        if let newData = dataValue {
            
            byteArray = Array<UInt8>(newData)
        }
    }
    func isNotifiable() -> Bool {
        
        return characteristic.properties.contains(.notify)
    }
    func isReadable() -> Bool {
        
        return characteristic.properties.contains(.read)
    }
    func isWriteable() -> Bool {
        return characteristic.properties.contains(.write)
    }
    private func getStringValue() -> String {
        
        var strvalue = ""
        if let value = characteristic.value {
            for character in value {
                strvalue = strvalue.appending("\(UnicodeScalar(character))")
            }
        }
        return strvalue
    }
    private func getHexValue() -> String {
        
        if let value = characteristic.value {
            return (value.hexEncodedString())
        }
        return ""
    }
}

enum DescriptorType {
    
    case ExtendedProperties
    case UserDescription
    case ClientConfiguration
    case Serverconfiguration
    case Format
    case AggregateFormat
    case None
}

public class Descriptor: NSObject {
    
    public var descriptor: CBDescriptor!
    public var uuid: String?
    public var title: String?
    public var value: String?
    var type: DescriptorType!
    
    init(descriptor: CBDescriptor) {
        super.init()
        
        self.descriptor = descriptor
        self.uuid       = descriptor.uuid.uuidString
        self.value      = "\(descriptor.value ?? "Custom")"
        
        switch descriptor.uuid.uuidString {
            
        case CBUUIDCharacteristicExtendedPropertiesString:
            guard let properties = descriptor.value as? NSNumber else {
                break
            }
            self.title = "Extended properties"
            self.type  = .ExtendedProperties
            print("Extended properties: \(properties)")
        case CBUUIDCharacteristicUserDescriptionString:
            guard let description = descriptor.value as? NSString else {
                break
            }
            self.title = "User description"
            self.type  = .UserDescription
            print("User description: \(description)")
        case CBUUIDClientCharacteristicConfigurationString:
            guard let clientConfig = descriptor.value as? NSNumber else {
                break
            }
            self.title = "Client configuration"
            self.type  = .ClientConfiguration
            self.value = "\(clientConfig)"
            print("Client configuration: \(clientConfig)")
        case CBUUIDServerCharacteristicConfigurationString:
            guard let serverConfig = descriptor.value as? NSNumber else {
                break
            }
            self.title = "Server configuration"
            self.type  = .Serverconfiguration
            print("Server configuration: \(serverConfig)")
        case CBUUIDCharacteristicFormatString:
            guard let format = descriptor.value as? NSData else {
                break
            }
            self.title = "Format"
            self.type  = .Format
            print("  Format: \(format)")
        case CBUUIDCharacteristicAggregateFormatString:
            self.title = "Aggregate Format"
            self.type  = .AggregateFormat
            print("Aggregate Format: (is not documented)")
        default:
            self.title = ""
            self.type  = .None
            break
        }
    }
}

extension Collection where Element == UInt8 {
    var data: Data {
        return Data(self)
    }
    var hexa: String {
        return map{ String(format: "%02X", $0) }.joined()
    }
}
