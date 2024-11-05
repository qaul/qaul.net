import UIKit
import Flutter

@main
@objc class AppDelegate: FlutterAppDelegate {
  override func application(
    _ application: UIApplication,
    didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]?
  ) -> Bool {
    let controller : FlutterViewController = window?.rootViewController as! FlutterViewController
    let libqaulChannel = FlutterMethodChannel(name: "libqaul", binaryMessenger: controller.binaryMessenger)
    if #available(iOS 10.0, *) {
      UNUserNotificationCenter.current().delegate = self as? UNUserNotificationCenterDelegate
    }
    
    libqaulChannel.setMethodCallHandler({
      (call: FlutterMethodCall, result: @escaping FlutterResult) -> Void in
      if ("start" == call.method) {
          if let directory = self.getApplicationDocumentsDirectory() {
              let nsstring = NSString(string: directory)
              start(nsstring.utf8String)
              result(nil)
          }
      } else if ("initialized" == call.method) {
          result(initialized())
      } else if ("sendRpcMessage" == call.method) {
          if let args = call.arguments as? Dictionary<String, Any>, let message = args["message"] as? FlutterStandardTypedData {
              let byte = [UInt8](message.data)
              result(send_rpc_to_libqaul(byte, UInt32(byte.count)))
          } else {
              result(FlutterError.init(code: "errorSetMessage", message: "data or format error", details: nil))
          }
      } else if ("receiveRpcMessage" == call.method) {
          let bufferSize = 259072;
          let pointer = UnsafeMutablePointer<UInt8>.allocate(capacity: bufferSize)
          defer {
              pointer.deinitialize(count: bufferSize)
              pointer.deallocate()
          }
          
          let message_length = receive_rpc_from_libqaul(pointer, UInt32(bufferSize))
          
          if (message_length == -2) {
              result(FlutterError.init(code: "errorReceiveRpc", message: "an unknown error occurred when receiving a message from libqaul", details: nil))
          } else if (message_length == -3) {
              result(FlutterError.init(code: "errorReceiveRpc", message: "buffer sent to libqaul is too small", details: nil))
          } else if (message_length < 0) {
                  result(FlutterError.init(code: "errorReceiveRpc", message: "buffer sent to libqaul is nil", details: nil))
          } else {
              let buffer = UnsafeMutableBufferPointer<UInt8>(start: pointer, count: Int(message_length))
              result(FlutterStandardTypedData(bytes: Data(buffer)))
          }
      } else if ("receivequeue" == call.method) {
          result(receivequeue())
      } else {
          result(FlutterMethodNotImplemented);
      }
    })

    GeneratedPluginRegistrant.register(with: self)
    return super.application(application, didFinishLaunchingWithOptions: launchOptions)
  }
    
  private func getApplicationDocumentsDirectory() -> String? {
      return NSSearchPathForDirectoriesInDomains(FileManager.SearchPathDirectory.documentDirectory, FileManager.SearchPathDomainMask.userDomainMask, true).first
  }

  private func dummyMethodToEnforceBundling() {
    // This code will force the bundler to use these functions, but will never be called
    hello();
    start("");
    initialized();
    send_rpc_to_libqaul(UnsafeMutablePointer<UInt8>.allocate(capacity: 0), 0);
    receive_rpc_from_libqaul(UnsafeMutablePointer<UInt8>.allocate(capacity: 0), 0);
  }
}
