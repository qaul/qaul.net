//
//  CommandConstVC.swift
//  VSNEW
//
//  Created by Nikhil Jobanputra on 10/05/2021.
//  Copyright © 2021 Bluepixel. All rights reserved.
//

import UIKit
import Foundation

struct SERVICES {
    
    static let PRIMARY_SERVICE_UUID1 = "0000E0FF-3C17-D293-8E48-14FE2E4DA212"
}

struct CHAR{
    
    static let VANEW_WRITE_NOTIFY = "FFE2"
    static let VANEW_WRITE_WRITE  = "FFE1"
    
    
//    static let VANEW_WRITE_NOTIFY = "0000FFE2-3C17-D293-8E48-14FE2E4DA212"
  //  static let VANEW_WRITE_WRITE  = "0000FFE1-3C17-D293-8E48-14FE2E4DA212"
}

struct COMMAND {
    
    static let Disconnect = 254
    static let Restart    = 255
}

struct REQUEST_TYPE {

    public static let READ = 1
    public static let WRITE = 2
}

struct timeout {
    
    static let defaultTimeoutInS: TimeInterval = 10
}

public enum RequestType {
    
    case TIMESYNC
    case VIN
    case SPEED
    case RPM
    case MAF
    case DIST_DTC
    case ACCEL
    case GYRO
    case TEMPERATURE
    case VBAT
    case STFTB1
    case STFTB2
    case LTFTB1
    case LTFTB2
    case CLT

    case None
}


public enum RequestTypeIndex: Int{
    
    case TIMESYNC = 0
    case VIN
    case SPEED
    case RPM
    case MAF
    case DIST_DTC
    case ACCEL
    case GYRO
    case TEMPERATURE
    case VBAT
    case STFTB1
    case STFTB2
    case LTFTB1
    case LTFTB2
    case CLT
}
