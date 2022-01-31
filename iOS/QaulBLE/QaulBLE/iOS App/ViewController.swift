//
//  ViewController.swift
//  QaulBLE
//
//  Created by BAPS on 12/01/22.
//

import UIKit

var navigationcontroller = UINavigationController()

class ViewController: UIViewController {

    //-----------------------------------------------------------------
    //                        MARK: - Outlet -
    //-----------------------------------------------------------------
    
    // -----------------------------------------------------------------
    //                        MARK: - Property -
    // -----------------------------------------------------------------
    private var value = "iOSQaulBLE"
    private var qaulId: String = ""

    //-----------------------------------------------------------------
    //                       MARK: - View Life Cycle -
    //-----------------------------------------------------------------
    
    override func viewDidLoad() {
        super.viewDidLoad()
        
        navigationcontroller = self.navigationController ?? UINavigationController()
        // Do any additional setup after loading the view.
        
    }
    
    //-----------------------------------------------------------------
    //                    MARK: - Button Action -
    //-----------------------------------------------------------------
    @IBAction func btnInfoRequest(sender: UIButton) {
        
        var info = Qaul_Sys_Ble_BleInfoRequest.init()

        var initobj = Qaul_Sys_Ble_Ble.init()
        initobj.infoRequest = info
        initobj.message = .infoRequest(info)
        print(initobj.message)

        bleWrapperClass.receiveRequest(bleReq: initobj) { qaul_Sys_Ble_Ble in
            print("qaul_Sys_Ble_Ble:- \(qaul_Sys_Ble_Ble)")
            if qaul_Sys_Ble_Ble.infoResponse != nil {
                let strmessage = "Device info recived from : \(qaul_Sys_Ble_Ble.infoResponse.device.name)"
                DispatchQueue.main.async {
                    self.view.makeToast(strmessage)
                }
            }
        }
    }
    
    @IBAction func btnAdvertizing(sender: UIButton) {
        
//        blePeripheral.startAdvertising(serviceID: kTRANSFER_SERVICE_UUID, name: self.value)
        sendStartRequest()
    }
    
    @IBAction func btnStopAdvertizing(sender: UIButton) {
        
//        blePeripheral.startAdvertising(serviceID: kTRANSFER_SERVICE_UUID, name: self.value)
        sendStopRequest()
    }
    
    //-----------------------------------------------------------------
    //                    MARK: - Functions -
    //-----------------------------------------------------------------
    /**
     * For Sending BleStartRequest to BLEModule
     * Have to pass qaul_id and advertise_mode as parameter
     */
    private func sendStartRequest() {
    
        var startRequest = Qaul_Sys_Ble_BleStartRequest.init()
    
        startRequest.qaulID = UIDevice.modelName.data(using: .utf8)!
        startRequest.mode = Qaul_Sys_Ble_BleMode.lowLatency //BleOuterClass.BleMode.low_latency
    
        var bleReq = Qaul_Sys_Ble_Ble.init()
        bleReq.startRequest = startRequest
        bleReq.message = .startRequest(Qaul_Sys_Ble_BleStartRequest.init())
    
        bleWrapperClass.receiveRequest(bleReq: bleReq) { qaul_Sys_Ble_Ble in
            print("qaul_Sys_Ble_Ble:- \(qaul_Sys_Ble_Ble)")
            if qaul_Sys_Ble_Ble.startResult != nil {
                let strmessage = qaul_Sys_Ble_Ble.startResult.errorMessage
                DispatchQueue.main.async {
                    self.view.makeToast(strmessage)
                }
            }
        }
    }
    
    /**
     * For Sending BleStopRequest to BLEModule. It Is Used To Stop Service.
     */
    private func sendStopRequest() {
      
        var stopRequest = Qaul_Sys_Ble_BleStopRequest.init()
     
        var bleReq = Qaul_Sys_Ble_Ble.init()
        bleReq.stopRequest = stopRequest
        bleReq.message = .stopRequest(Qaul_Sys_Ble_BleStopRequest.init())
        
        bleWrapperClass.receiveRequest(bleReq: bleReq) { qaul_Sys_Ble_Ble in
            print("qaul_Sys_Ble_Ble:- \(qaul_Sys_Ble_Ble)")
            if qaul_Sys_Ble_Ble.stopResult != nil {
                let strmessage = qaul_Sys_Ble_Ble.stopResult.errorMessage
                DispatchQueue.main.async {
                    self.view.makeToast(strmessage)
                }
            }
        }
    }
}



//public extension String {
//    
//    var bytes: Array<UInt8> {
//        return data(using: String.Encoding.utf8, allowLossyConversion: true)?.bytes ?? Array(utf8)
//    }
//    
//}
