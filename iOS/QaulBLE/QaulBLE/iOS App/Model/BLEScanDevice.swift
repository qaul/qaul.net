//
//  BLEScanDevice.swift
//  QaulBLE
//
//  Created by BAPS on 31/01/22.
//

import UIKit

public class BLEScanDevice: NSObject {
    
    var deviceRSSI: Int = 0
    
//    var scanResult: ScanResult? skip for iOS
    var name: String?
    var macAddress: String? //UUID
    var intervalNanos: Double?
    var bluetoothDevice: BLEDevice?
    var isConnectable = true
    var lastFoundTime: Double?
    var qaulId: Data?
    var strqaulId: String?
    
    public override init() {
        
    }
}


