//
//  BaseVC.swift
//  VSNEW
//
//  Created by Nikhil Jobanputra on 14/05/2021.
//  Copyright © 2021 Bluepixel. All rights reserved.
//

import Foundation
import CoreBluetooth
import UIKit

//MARK:-  For Peripherals
public extension CBPeripheral {
    func getperipheralid() -> String {
        return identifier.uuidString.uppercased()
        
    }
    func Isconnected() -> Bool {
         return state == .connected
    }
}

//MARK:-  For AdvertisementData
public extension Dictionary where Key == String, Value == Any {
    
    func peripheralname() -> String? {

        return self[CBAdvertisementDataLocalNameKey] as? String
    }
    func IsConnectable() -> Bool {
        return (self[CBAdvertisementDataIsConnectable] as! Int == 1) ? true:false
    }
    func IsEddystone() -> Bool {
        if (self[CBAdvertisementDataServiceDataKey] != nil) {
            if (self[CBAdvertisementDataServiceDataKey] as! [CBUUID: Any])[CBUUID(string: "FEAA")] != nil {
                return true
            }
        }
        return false
    }
    func standardServiceName() -> String? {
        
        if let serviceIDS = self[CBAdvertisementDataServiceUUIDsKey] as? NSArray {
            for seviceid in serviceIDS {
                
                let key = DictOfservices.keys.filter({$0 == "\(seviceid)"})
                if key.count > 0 {
                    return DictOfservices[key[0]]
                }
            }
        }
        return nil
    }
    
    func standardServiceid() -> String? {
        
        if let serviceIDS = self[CBAdvertisementDataServiceUUIDsKey] as? NSArray {
            for seviceid in serviceIDS {
                
                return "\(seviceid)"
            }
        }
        return nil
    }
    
    func getManufacturerData() -> String? {
        
        if let manufacturerData = self[CBAdvertisementDataManufacturerDataKey] as? Data {
            return manufacturerData.hexEncodedString()
        }
        return nil
    }
}

//MARK:-  For String
public extension String {
    func isValidHexValue() -> Bool {
        
        let chars = CharacterSet(charactersIn: "0123456789ABCDEF").inverted
        guard uppercased().rangeOfCharacter(from: chars) == nil else {
            return false
        }
        return true
    }
    func trimString() -> String {
        return trimmingCharacters(in: .whitespacesAndNewlines)
    }
    
    var bytes: Array<UInt8> {
        return data(using: String.Encoding.utf8, allowLossyConversion: true)?.bytes ?? Array(utf8)
    }
}

//MARK:-  For Characteristic
public extension CBService {
    
    func getUUIDString() -> String {
        
        return uuid.uuidString.uppercased()
    }
    func servicename() -> String {
        return "\(DictOfservices[uuid.uuidString.uppercased()] ?? "CUSTOM SERVICE")"
    }
}

//MARK:-  For Characteristic
public extension CBCharacteristic {
    
    func getUUIDString() -> String {
        
        return uuid.uuidString.uppercased()
    }
    func isContainReadProperty() -> Bool {
        return properties.contains(.read)
    }
    func isContainWriteProperty() -> Bool {
        return properties.contains(.write)
    }
    func isContainNotifyProperty() -> Bool {
        return properties.contains(.notify)
    }
    func getProperties() -> String {
    
        var strPermission = ""
        if properties.contains(.read) {
            strPermission = "Read "
        }
        if properties.contains(.write) {
            strPermission = strPermission + "Write "
        }
        if properties.contains(.notify) {
            strPermission = strPermission + "Notify"
        }
        return strPermission
    }
}

//MARK:-  For Data
extension Data {
    struct HexEncodingOptions: OptionSet {
        let rawValue: Int
        static let upperCase = HexEncodingOptions(rawValue: 1 << 0)
    }
    
    func hexEncodedString(options: HexEncodingOptions = []) -> String {
        let format = options.contains(.upperCase) ? "%02hhX" : "%02hhx"
        return map { String(format: format, $0) }.joined()
    }
    func intValue() -> Int {
        
        let stringInt = String(data: self, encoding: String.Encoding.utf8)
        return Int(stringInt ?? "") ?? 0
    }
    var bytes: Array<UInt8> {
        return Array(self)
    }
    
    var uint16: UInt16 {
        get {
            let i16array = self.withUnsafeBytes {
                UnsafeBufferPointer<UInt16>(start: $0, count: self.count / 2).map(UInt16.init(littleEndian:))
            }
            return i16array[0]
        }
    }
    
    var int16: Int16 {
           get {
               let i16array = self.withUnsafeBytes {
                   UnsafeBufferPointer<Int16>(start: $0, count: self.count / 2).map(Int16.init(littleEndian:))
               }
               return i16array[0]
           }
       }
    
}

//MARK:-  For UIViewController
public extension UIViewController {
    
    func ShowAlertWithOKButton(message: String,complition: @escaping () -> Void) {
        
        let alert = UIAlertController(title: ""/*ALERT_TITLE*/, message: message, preferredStyle: .alert)
        let okaction = UIAlertAction(title: "OK", style: .cancel) { (okaction) in
            complition()
        }
        alert.addAction(okaction)
        present(alert, animated: true, completion: nil)
        
    }
    
    func ShowAlertWithTwoButtons(title: String, message: String, btn1name: String, btn2name: String,complition: @escaping (Int) -> Void) {
        
        let alert = UIAlertController(title: title, message: message, preferredStyle: .alert)
        let btn1ction = UIAlertAction(title: btn1name, style: .default) { (btn1) in
            complition(1)
        }
        alert.addAction(btn1ction)
        
        let btn2ction = UIAlertAction(title: btn2name, style: .default) { (btn2) in
            complition(2)
        }
        alert.addAction(btn2ction)
        
        present(alert, animated: true, completion: nil)
    }
}
