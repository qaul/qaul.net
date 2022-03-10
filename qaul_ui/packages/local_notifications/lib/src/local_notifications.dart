import 'dart:async';
import 'dart:io' show Platform;

import 'package:equatable/equatable.dart';
import 'package:flutter_local_notifications/flutter_local_notifications.dart';

abstract class LocalNotifications {
  static LocalNotifications instance = _LocalNotifications();

  Future<bool> initialize();

  Stream<LocalNotification> get onNotificationOpened;

  Future<bool> requestPermissions();

  Future<void> displayNotification(LocalNotification message);
}

class LocalNotification extends Equatable {
  const LocalNotification({
    required this.id,
    required this.title,
    required this.body,
    required this.payload,
  });

  final int id;
  final String title;
  final String body;
  final String payload;

  @override
  List<Object?> get props => [title, body, payload];
}

class _LocalNotifications implements LocalNotifications {
  final _localNotificationsPlugin = FlutterLocalNotificationsPlugin();

  final _messageStreamController =
      StreamController<LocalNotification>.broadcast();

  @override
  Stream<LocalNotification> get onNotificationOpened =>
      _messageStreamController.stream;

  @override
  Future<bool> initialize() async {
    final initializationSettings = InitializationSettings(
      android: const AndroidInitializationSettings('@drawable/ic_notification'),
      iOS: IOSInitializationSettings(
        onDidReceiveLocalNotification: _onDidReceiveLocalNotification,
      ),
      macOS: const MacOSInitializationSettings(),
      linux: LinuxInitializationSettings(
        defaultActionName: 'qaul-app',
        defaultIcon: AssetsLinuxIcon('assets/logo/icon_android.png'),
      ),
    );

    final r = await _localNotificationsPlugin.initialize(
      initializationSettings,
      onSelectNotification: (payload) async {
        if (payload == null) return;
        await _handleNewLocalNotificationOpened(payload);
      },
    );

    if (r == null) return false;
    return r;
  }

  @override
  Future<bool> requestPermissions() async {
    if (!(Platform.isIOS || Platform.isMacOS)) return true;

    bool? result;
    if (Platform.isIOS) {
      result = await _localNotificationsPlugin
          .resolvePlatformSpecificImplementation<
              IOSFlutterLocalNotificationsPlugin>()
          ?.requestPermissions(
            alert: true,
            badge: true,
            sound: true,
          );
    } else {
      result = await _localNotificationsPlugin
          .resolvePlatformSpecificImplementation<
              MacOSFlutterLocalNotificationsPlugin>()
          ?.requestPermissions(
            alert: true,
            badge: true,
            sound: true,
          );
    }

    if (result == null) return false;
    return result;
  }

  @override
  Future<void> displayNotification(LocalNotification message) async {
    await _localNotificationsPlugin.show(
        message.id, message.title, message.body, _notificationDetails(),
        payload: message.payload);
  }

  // ***************************************************************************
  void _onDidReceiveLocalNotification(
      int id, String? title, String? body, String? payload) {
    // TODO
  }

  Future<void> _handleNewLocalNotificationOpened(String payload) async {
    // TODO
  }

  NotificationDetails _notificationDetails() {
    if (!Platform.isWindows) {
      return NotificationDetails(
        android: _androidDetails,
        iOS: _iosDetails,
        macOS: _macosDetails,
        linux: _linuxDetails,
      );
    }
    throw UnimplementedError(
        '_notificationDetails() Failed on platform ${Platform.operatingSystem}');
  }

  AndroidNotificationDetails? get _androidDetails => !Platform.isAndroid
      ? null
      : const AndroidNotificationDetails(
          'qaulAppChannelNotificationsID',
          'qaulAppNotifications',
          channelDescription: 'Used to display new message notifications',
          importance: Importance.max,
          priority: Priority.high,
        );

  IOSNotificationDetails? get _iosDetails =>
      !Platform.isIOS ? null : const IOSNotificationDetails();

  MacOSNotificationDetails? get _macosDetails =>
      !Platform.isIOS ? null : const MacOSNotificationDetails();

  LinuxNotificationDetails? get _linuxDetails =>
      !Platform.isLinux ? null : const LinuxNotificationDetails();
}
