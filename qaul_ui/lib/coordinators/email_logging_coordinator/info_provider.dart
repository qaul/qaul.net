part of 'email_logging_coordinator.dart';

class _InfoProvider {
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
