import 'package:device_info_plus/device_info_plus.dart';
import 'package:package_info_plus/package_info_plus.dart';

class InfoProvider {
  static Future<Map<String, dynamic>> getPackageInfo() async {
    PackageInfo packageInfo = await PackageInfo.fromPlatform();
    return {
      'appName': packageInfo.appName,
      'packageName': packageInfo.packageName,
      'appVersion': packageInfo.version,
      'buildNumber': packageInfo.buildNumber,
    };
  }

  static Future<Map<String, dynamic>> getDeviceInfo() async {
    final deviceInfo = await DeviceInfoPlugin().deviceInfo;
    return deviceInfo.toMap();
  }
}
