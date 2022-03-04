import 'dart:async';
import 'dart:io' show Platform;

import 'package:equatable/equatable.dart';
import 'package:flutter_local_notifications/flutter_local_notifications.dart';

abstract class LocalNotifications {
  static LocalNotifications instance = _LocalNotifications();

  Future<bool> initialize();

  Stream<Message> get onNotificationOpened;

  Future<bool> requestPermissions();

  Future<void> displayNotification(Message message);
}

class Message extends Equatable {
  const Message({
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

  final _messageStreamController = StreamController<Message>.broadcast();

  @override
  Stream<Message> get onNotificationOpened => _messageStreamController.stream;

  @override
  Future<bool> initialize() async {
    final initializationSettings = InitializationSettings(
      android: const AndroidInitializationSettings('ic_stat_name'),
      iOS: IOSInitializationSettings(
        onDidReceiveLocalNotification: _onDidReceiveLocalNotification,
      ),
      macOS: const MacOSInitializationSettings(),
      linux: const LinuxInitializationSettings(defaultActionName: 'action-name'),
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
          .resolvePlatformSpecificImplementation<IOSFlutterLocalNotificationsPlugin>()
          ?.requestPermissions(
            alert: true,
            badge: true,
            sound: true,
          );
    } else {
      result = await _localNotificationsPlugin
          .resolvePlatformSpecificImplementation<MacOSFlutterLocalNotificationsPlugin>()
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
  Future<void> displayNotification(Message message) async {
    await _localNotificationsPlugin.show(
        message.id, message.title, message.body, _notificationDetails(),
        payload: message.payload);
  }

  // ***************************************************************************
  void _onDidReceiveLocalNotification(int id, String? title, String? body, String? payload) {
    // TODO
  }

  Future<void> _handleNewLocalNotificationOpened(String payload) async {
    // TODO
  }

  NotificationDetails _notificationDetails() {
    if (!Platform.isWindows) {
      return NotificationDetails(
        android: androidNotificationDetails,
        iOS: iosNotificationDetails,
        macOS: macosNotificationDetails,
        linux: linuxNotificationDetails,
      );
    }
    throw UnimplementedError(
        '_notificationDetails() Failed on platform ${Platform.operatingSystem}');
  }

  AndroidNotificationDetails? get androidNotificationDetails => !Platform.isAndroid
      ? null
      : const AndroidNotificationDetails(
          'your channel id',
          'your channel name',
          channelDescription: 'your channel description',
          importance: Importance.max,
          priority: Priority.high,
        );

  IOSNotificationDetails? get iosNotificationDetails =>
      !Platform.isIOS ? null : const IOSNotificationDetails();

  MacOSNotificationDetails? get macosNotificationDetails =>
      !Platform.isIOS ? null : const MacOSNotificationDetails();

  LinuxNotificationDetails? get linuxNotificationDetails =>
      !Platform.isLinux ? null : const LinuxNotificationDetails();
}
