import UIKit
import Flutter

@UIApplicationMain
@objc class AppDelegate: FlutterAppDelegate {
  override func application(
    _ application: UIApplication,
    didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]?
  ) -> Bool {
    GeneratedPluginRegistrant.register(with: self)
    return super.application(application, didFinishLaunchingWithOptions: launchOptions)
  }

  public func dummyMethodToEnforceBundling() {
      // ...
      // This code will force the bundler to use these functions, but will never be called
      hello();
      start();
      initialized();
      send_rpc_to_libqaul(UnsafeMutablePointer<UInt8>.allocate(capacity: 0), 0);
      receive_rpc_from_libqaul(UnsafeMutablePointer<UInt8>.allocate(capacity: 0), 0);
    }
}
