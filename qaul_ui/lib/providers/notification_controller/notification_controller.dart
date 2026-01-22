import 'dart:collection';

import 'package:flutter/foundation.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:local_notifications/local_notifications.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:shared_preferences/shared_preferences.dart';

import '../providers.dart';

class NotificationController<T> {
  NotificationController(this.ref);

  @protected
  final Ref ref;

  @protected
  SharedPreferences get preferences => _preferences;
  late final SharedPreferences _preferences;

  @protected
  User get localUser => _user;
  late final User _user;

  @protected
  TabType get currentVisibleHomeTab => ref.read(homeScreenControllerProvider);

  @protected
  @visibleForOverriding
  MapEntry<dynamic, void Function(T?, T)>
      get strategy =>
          throw UnimplementedError('Must be implemented by child class');

  @protected
  @visibleForOverriding
  String get cacheKey =>
      throw UnimplementedError('Must be implemented by child class');

  @mustCallSuper
  Future<void> initialize() async {
    _preferences = await SharedPreferences.getInstance();
    _user = ref.read(defaultUserProvider)!;
    ref.listen(strategy.key, strategy.value);
  }

  @protected
  @visibleForOverriding
  void updatePersistentCachedData() =>
      throw UnimplementedError('Must be implemented by child class');

  void removeNotifications() =>
      LocalNotifications.instance.removeNotifications();
}

mixin DataProcessingStrategy<T> {
  ValueNotifier<int?> newNotificationCount = ValueNotifier(null);

  void execute(List<T>? previous, List<T> current) async {
    final queue = Queue<T>()..addAll(entriesToBeProcessed(current));
    if (queue.isEmpty) return;

    while (queue.isNotEmpty) {
      final entry = queue.removeFirst();
      final message = process(entry);
      if (message == null) continue;
      newNotificationCount.value = (newNotificationCount.value ?? 0) + 1;
      LocalNotifications.instance.displayNotification(message);
      await Future.delayed(const Duration(milliseconds: 500));
    }

    close();
  }

  @visibleForOverriding
  Iterable<T> entriesToBeProcessed(List<T> values);

  @visibleForOverriding
  LocalNotification? process(T value);

  @visibleForOverriding
  void close();
}
