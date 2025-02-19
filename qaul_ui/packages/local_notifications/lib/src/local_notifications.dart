// ignore_for_file: depend_on_referenced_packages
import 'dart:async';
import 'dart:io' show Platform;

import 'package:app_badge_plus/app_badge_plus.dart';
import 'package:equatable/equatable.dart';
import 'package:flutter_local_notifications/flutter_local_notifications.dart';
import 'package:logging/logging.dart';

abstract class LocalNotifications {
  static LocalNotifications instance = Platform.isWindows ||
          const bool.fromEnvironment('testing_mode', defaultValue: false)
      ? _NullLocalNotifications()
      : _LocalNotifications();

  Future<bool> initialize();

  Stream<LocalNotification> get onNotificationOpened;

  Future<bool> requestPermissions();

  Future<void> displayNotification(LocalNotification message);

  Future<void> removeNotifications();
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
      iOS: DarwinInitializationSettings(
        requestBadgePermission: true,
      ),
      macOS: const DarwinInitializationSettings(requestBadgePermission: true),
      linux: LinuxInitializationSettings(
        defaultActionName: 'qaul-app',
        defaultIcon: AssetsLinuxIcon('assets/logo/icon_android.png'),
      ),
    );

    final r = await _localNotificationsPlugin.initialize(
      initializationSettings,
      onDidReceiveNotificationResponse: (payload) async {
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
    return result ?? false;
  }

  @override
  Future<void> displayNotification(LocalNotification message) async {
    if (!Platform.isLinux && (await AppBadgePlus.isSupported())) {
      AppBadgePlus.updateBadge(1);
    }
    await _localNotificationsPlugin.show(
        message.id, message.title, message.body, _notificationDetails(),
        payload: message.payload);
  }

  @override
  Future<void> removeNotifications() async =>
      _localNotificationsPlugin.cancelAll();

  // ***************************************************************************
  Future<void> _handleNewLocalNotificationOpened(
      NotificationResponse payload) async {
    if (!Platform.isLinux && (await AppBadgePlus.isSupported())) {
      AppBadgePlus.updateBadge(0);
    }
  }

  NotificationDetails _notificationDetails() {
    if (!Platform.isWindows) {
      return NotificationDetails(
        android: _androidDetails,
        iOS: _darwinDetails,
        macOS: _darwinDetails,
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

  DarwinNotificationDetails? get _darwinDetails =>
      !(Platform.isIOS || Platform.isMacOS)
          ? null
          : const DarwinNotificationDetails();

  LinuxNotificationDetails? get _linuxDetails =>
      !Platform.isLinux ? null : const LinuxNotificationDetails();
}

class _NullLocalNotifications implements LocalNotifications {
  @override
  Future<void> displayNotification(LocalNotification message) async {}

  @override
  Future<bool> initialize() {
    Logger('LocalNotifications').config(
        '(NullLocalNotifications): The package is initializing a Null implementation, either because it has been told to (with --dart-define=testing_mode=true), or because the host platform is not supported.');
    return Future.value(false);
  }

  @override
  Stream<LocalNotification> get onNotificationOpened =>
      throw UnimplementedError();

  @override
  Future<void> removeNotifications() async {}

  @override
  Future<bool> requestPermissions() => Future.value(false);
}
