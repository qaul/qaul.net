import UIKit
import Flutter
import Photos

@main
@objc class AppDelegate: FlutterAppDelegate, UIDocumentPickerDelegate {
  // The document picker may retain the exported URL after this method returns.
  // Keep its temporary parent directory until the user finishes or cancels the
  // export, then remove only that directory.
  private var exportTemporaryDirectories: [ObjectIdentifier: URL] = [:]
  override func application(
    _ application: UIApplication,
    didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]?
  ) -> Bool {
    let controller : FlutterViewController = window?.rootViewController as! FlutterViewController
    let libqaulChannel = FlutterMethodChannel(name: "libqaul", binaryMessenger: controller.binaryMessenger)
    let attachmentClipboardChannel = FlutterMethodChannel(name: "qaul/attachment_clipboard", binaryMessenger: controller.binaryMessenger)
    let attachmentSaverChannel = FlutterMethodChannel(name: "qaul/attachment_saver", binaryMessenger: controller.binaryMessenger)
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

    attachmentClipboardChannel.setMethodCallHandler({
      (call: FlutterMethodCall, result: @escaping FlutterResult) -> Void in
      guard call.method == "copyFile" else {
        result(FlutterMethodNotImplemented)
        return
      }
      guard let args = call.arguments as? Dictionary<String, Any>,
            let path = args["path"] as? String,
            FileManager.default.fileExists(atPath: path) else {
        result(FlutterError(code: "file_not_found", message: "The attachment is no longer available", details: nil))
        return
      }

      if let image = UIImage(contentsOfFile: path) {
        UIPasteboard.general.image = image
      } else {
        UIPasteboard.general.setValue(URL(fileURLWithPath: path).absoluteString, forPasteboardType: "public.file-url")
      }
      result(nil)
    })

    attachmentSaverChannel.setMethodCallHandler({
      (call: FlutterMethodCall, result: @escaping FlutterResult) -> Void in
      guard call.method == "saveFile" else {
        result(FlutterMethodNotImplemented)
        return
      }
      guard let args = call.arguments as? Dictionary<String, Any>,
            let path = args["path"] as? String,
            FileManager.default.fileExists(atPath: path) else {
        result(FlutterError(code: "file_not_found", message: "The attachment is no longer available", details: nil))
        return
      }

      let name = args["name"] as? String ?? URL(fileURLWithPath: path).lastPathComponent
      let kind = args["kind"] as? String ?? "file"
      self.saveAttachment(
        URL(fileURLWithPath: path),
        name: name,
        kind: kind,
        from: controller,
        result: result
      )
    })

    GeneratedPluginRegistrant.register(with: self)
    return super.application(application, didFinishLaunchingWithOptions: launchOptions)
  }
    
  private func getApplicationDocumentsDirectory() -> String? {
      return NSSearchPathForDirectoriesInDomains(FileManager.SearchPathDirectory.documentDirectory, FileManager.SearchPathDomainMask.userDomainMask, true).first
  }

  /// Saves photos and videos to the photo library; other files go through the system export
  /// dialog. Replies "saved" or "handedToSystem", always on the main thread.
  private func saveAttachment(_ url: URL, name: String, kind: String, from controller: UIViewController, result: @escaping FlutterResult) {
    guard kind == "image" || kind == "video" else {
      let fileManager = FileManager.default
      let fallbackName = url.lastPathComponent
      let requestedName = URL(fileURLWithPath: name).lastPathComponent
      let exportName = requestedName.isEmpty || requestedName == "." ? fallbackName : requestedName
      let temporaryDirectory = fileManager.temporaryDirectory.appendingPathComponent(UUID().uuidString, isDirectory: true)
      let exportURL = temporaryDirectory.appendingPathComponent(exportName, isDirectory: false)

      do {
        try fileManager.createDirectory(at: temporaryDirectory, withIntermediateDirectories: true)
        try fileManager.copyItem(at: url, to: exportURL)
      } catch {
        try? fileManager.removeItem(at: temporaryDirectory)
        result(FlutterError(code: "export_prepare_failed", message: "Could not prepare the attachment for export", details: error.localizedDescription))
        return
      }

      let picker: UIDocumentPickerViewController
      if #available(iOS 14.0, *) {
        picker = UIDocumentPickerViewController(forExporting: [exportURL], asCopy: true)
      } else {
        picker = UIDocumentPickerViewController(url: exportURL, in: .exportToService)
      }
      picker.delegate = self
      exportTemporaryDirectories[ObjectIdentifier(picker)] = temporaryDirectory
      controller.present(picker, animated: true)
      result("handedToSystem")
      return
    }

    let reply: (Any?) -> Void = { value in DispatchQueue.main.async { result(value) } }
    let save = {
      PHPhotoLibrary.shared().performChanges({
        let request = PHAssetCreationRequest.forAsset()
        request.addResource(with: kind == "image" ? .photo : .video, fileURL: url, options: nil)
      }) { success, error in
        reply(success
          ? "saved"
          : FlutterError(code: "save_failed", message: "Could not save the attachment", details: error?.localizedDescription))
      }
    }
    let denied = {
      reply(FlutterError(code: "photo_permission_denied", message: "Photo Library access is required to save this attachment", details: nil))
    }

    if #available(iOS 14.0, *) {
      // Adding to the library only needs add-only access, which also covers "limited".
      PHPhotoLibrary.requestAuthorization(for: .addOnly) { status in
        status == .authorized || status == .limited ? save() : denied()
      }
    } else {
      PHPhotoLibrary.requestAuthorization { status in
        status == .authorized ? save() : denied()
      }
    }
  }

  func documentPicker(_ controller: UIDocumentPickerViewController, didPickDocumentsAt urls: [URL]) {
    removeExportTemporaryDirectory(for: controller)
  }

  func documentPickerWasCancelled(_ controller: UIDocumentPickerViewController) {
    removeExportTemporaryDirectory(for: controller)
  }

  private func removeExportTemporaryDirectory(for controller: UIDocumentPickerViewController) {
    guard let directory = exportTemporaryDirectories.removeValue(forKey: ObjectIdentifier(controller)) else {
      return
    }
    try? FileManager.default.removeItem(at: directory)
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
