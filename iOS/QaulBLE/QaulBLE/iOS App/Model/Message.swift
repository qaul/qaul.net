//
//  Message.swift
//  QaulBLE
//
//  Created by BAPS on 31/01/22.
//

import UIKit
import ObjectMapper

class Message: Mappable {
    
    var qaulId: String?
    var message: String?
    
    required init?(map: Map) {
    }
    
    init() {
    }
    
    func mapping(map: Map) {
        
        qaulId <- map["qaul_id"]
        message <- map["message"]
    }
}


