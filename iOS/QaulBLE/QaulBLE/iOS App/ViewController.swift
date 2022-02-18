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
    
    @IBOutlet weak var txtQaulBLE: UITextField!
    @IBOutlet weak var txtMessage: UITextView!
    @IBOutlet weak var lblMessagePlaceholader: UILabel!
    @IBOutlet weak var heightOfTxtMessage: NSLayoutConstraint!
    @IBOutlet weak var lblMessage: UILabel!
    // -----------------------------------------------------------------
    //                        MARK: - Property -
    // -----------------------------------------------------------------
    private var value = "iOSQaulBLE"
    private var qaulId: String = ""
    private let maxHeightOfTxtMessage: CGFloat = 1000

    //-----------------------------------------------------------------
    //                       MARK: - View Life Cycle -
    //-----------------------------------------------------------------
    
    override func viewDidLoad() {
        super.viewDidLoad()
        
        navigationcontroller = self.navigationController ?? UINavigationController()
        // Do any additional setup after loading the view.
        
        NotificationCenter.default.removeObserver(self, name: .GetscanDevice, object: nil)
        NotificationCenter.default.addObserver(self, selector: #selector(SetScanDevice(_:)), name: .GetscanDevice, object: nil)
        
        NotificationCenter.default.removeObserver(self, name: .GetscanMessage, object: nil)
        NotificationCenter.default.addObserver(self, selector: #selector(SetScanMessage(_:)), name: .GetscanMessage, object: nil)
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

        var setbleReq = Qaul_Sys_Ble_Ble.init()
        setbleReq.message = .infoRequest(Qaul_Sys_Ble_BleInfoRequest.init())
        
        bleWrapperClass.receiveRequest(bleReq: initobj, SetdataforbleReq: setbleReq) { qaul_Sys_Ble_Ble in
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
    
    @IBAction func btnSendMessage(sender: UIButton) {
        
        if (txtQaulBLE.text?.count ?? 0) < 10 &&  (txtQaulBLE.text?.count ?? 0) > 0 {
            self.view.makeToast("Please enter correct qaul_id of receiver")
        }
        if txtMessage.text.count <= 0 {
            self.view.makeToast("Please enter at least 1 character of message")
            return
        }
        self.sendData(strqaulId: (txtQaulBLE.text ?? ""), message: (txtMessage.text ?? ""))
//        blePeripheral.startAdvertising(serviceID: kTRANSFER_SERVICE_UUID, name: self.value)
//        sendStopRequest()
    }
    
    //-----------------------------------------------------------------
    //                    MARK: - Functions -
    //-----------------------------------------------------------------
    
    @objc func SetScanDevice(_ notification: NSNotification) {
        guard let strQaulID = notification.object as? BLEScanDevice else { return }
       
        self.txtQaulBLE.text = strQaulID.strqaulId
        
    }
    
    @objc func SetScanMessage(_ notification: NSNotification) {
        guard let strMessage = notification.object as? Message else { return }
       
        DispatchQueue.main.async {
        self.lblMessage.text = strMessage.message ?? ""
        }
    }
    
    /**
     * For Sending BleStartRequest to BLEModule
     * Have to pass qaul_id and advertise_mode as parameter
     */
    private func sendStartRequest() {
    
        var startRequest = Qaul_Sys_Ble_BleStartRequest.init()
    
        startRequest.qaulID = (appendtextiOSdevice + UIDevice.modelName).data(using: .utf8)!
        startRequest.mode = Qaul_Sys_Ble_BleMode.lowLatency //BleOuterClass.BleMode.low_latency
    
        var bleReq = Qaul_Sys_Ble_Ble.init()
        bleReq.startRequest = startRequest
//        bleReq.message = .startRequest(Qaul_Sys_Ble_BleStartRequest.init())
    
        var setbleReq = Qaul_Sys_Ble_Ble.init()
        setbleReq.message = .startRequest(Qaul_Sys_Ble_BleStartRequest.init())
        
        bleWrapperClass.receiveRequest(bleReq: bleReq, SetdataforbleReq: setbleReq) { qaul_Sys_Ble_Ble in
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
        bleReq.message = .stopRequest(Qaul_Sys_Ble_BleStopRequest.init())
        bleReq.stopRequest = stopRequest
        
        var setbleReq = Qaul_Sys_Ble_Ble.init()
        setbleReq.message = .stopRequest( Qaul_Sys_Ble_BleStopRequest.init())
        
        bleWrapperClass.receiveRequest(bleReq: bleReq, SetdataforbleReq: setbleReq) { qaul_Sys_Ble_Ble in
            print("qaul_Sys_Ble_Ble:- \(qaul_Sys_Ble_Ble)")
            if qaul_Sys_Ble_Ble.stopResult != nil {
                let strmessage = qaul_Sys_Ble_Ble.stopResult.errorMessage
                DispatchQueue.main.async {
                    self.view.makeToast(strmessage)
                }
            }
        }
    }
    
    private func sendData(strqaulId: String, message: String) {
        var directSend = Qaul_Sys_Ble_BleDirectSend.init()
       
        directSend.data = Data(message.utf8)
        directSend.to = strqaulId.data(using: .utf8)!
        directSend.qaulID = (appendtextiOSdevice + UIDevice.modelName).data(using: .utf8)!
        directSend.id = String(Int64(Date().timeIntervalSince1970 * 1000))
//        directSend.unknownFields = SwiftProtobuf.UnknownStorage()
        
        var bleReq = Qaul_Sys_Ble_Ble.init()
        //bleReq.message = .directSend(Qaul_Sys_Ble_BleDirectSend.init())
        bleReq.directSend = directSend
    
        
        var setbleReq = Qaul_Sys_Ble_Ble.init()
        setbleReq.message = .directSend(Qaul_Sys_Ble_BleDirectSend.init())
//        bleReq.directSend = directSend
        
        bleWrapperClass.receiveRequest(bleReq: bleReq, SetdataforbleReq: setbleReq) { qaul_Sys_Ble_Ble in
            print("qaul_Sys_Ble_Ble:- \(qaul_Sys_Ble_Ble)")
            if qaul_Sys_Ble_Ble.directSendResult != nil {
                let strmessage = qaul_Sys_Ble_Ble.directSendResult.errorMessage
                DispatchQueue.main.async {
                    self.view.makeToast(strmessage)
                }
            }
        }
        DispatchQueue.main.async {
            self.view.makeToast("Connecting...")
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
// ---------------------------------------------------------------------------
//                          MARK: - UITextViewDelegate -
// ---------------------------------------------------------------------------
extension ViewController: UITextViewDelegate {

func textView(_ textView: UITextView, shouldChangeTextIn range: NSRange, replacementText text: String) -> Bool {
    
    let updatedText = (textView.text ?? "").count + text.count - range.length
    
    switch textView {
        case self.txtMessage:
            
            if updatedText > 0 {
                self.lblMessagePlaceholader.isHidden = true
                
            } else {
                self.lblMessagePlaceholader.isHidden = false
            }
            
            if self.txtMessage.contentSize.height >= self.maxHeightOfTxtMessage {
                self.txtMessage.isScrollEnabled = true
                    //                    self.heightOfTxtMessage.constant = 37

            } else {
                self.txtMessage.isScrollEnabled = false
                self.heightOfTxtMessage.constant = self.maxHeightOfTxtMessage
            }
            return updatedText <= 1500
            
        default:
            break
    }
    return true
}
}
