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
  MapEntry<dynamic, void Function(T?, T)> get strategy =>
      throw UnimplementedError('Must be implemented by child class');

  /// The account-agnostic key this controller's cache used to live under, and
  /// the prefix its per-account keys are derived from.
  @protected
  @visibleForOverriding
  String get legacyCacheKey =>
      throw UnimplementedError('Must be implemented by child class');

  /// Unread state is per-account: sharing one key let the next account to sign
  /// in inherit the previous one's cache, and then treat all of its own
  /// content as new.
  @protected
  String get cacheKey => '$legacyCacheKey.${localUser.idBase58}';

  @mustCallSuper
  Future<void> initialize() async {
    _preferences = await SharedPreferences.getInstance();
    _user = ref.read(defaultUserProvider)!;
    await _migrateLegacyCache();
    ref.listen(strategy.key, strategy.value);
  }

  /// Hands the pre-namespacing entry to the first account that reads it (the
  /// one signed in when the app was updated) and drops the shared key, so no
  /// later session can pick it up.
  Future<void> _migrateLegacyCache() async {
    final legacy = preferences.get(legacyCacheKey);
    if (legacy == null) return;
    if (!preferences.containsKey(cacheKey)) await adoptLegacyValue(legacy);
    await preferences.remove(legacyCacheKey);
  }

  /// Writes [value], read from [legacyCacheKey], under [cacheKey].
  @protected
  @visibleForOverriding
  Future<void> adoptLegacyValue(Object value) =>
      throw UnimplementedError('Must be implemented by child class');

  @protected
  @visibleForOverriding
  void updatePersistentCachedData() =>
      throw UnimplementedError('Must be implemented by child class');

  void removeNotifications() =>
      LocalNotifications.instance.removeNotifications();
}

mixin DataProcessingStrategy<T> on NotificationController<List<T>> {
  ValueNotifier<int?> newNotificationCount = ValueNotifier(null);

  @override
  void removeNotifications() {
    newNotificationCount.value = null;
    super.removeNotifications();
  }

  void execute(List<T>? previous, List<T> current) async {
    final queue = Queue<T>()..addAll(entriesToBeProcessed(current));
    if (queue.isEmpty) return;

    while (queue.isNotEmpty) {
      final entry = queue.removeFirst();
      final countIncrement = notificationCountIncrement(entry);
      final message = process(entry);
      if (message == null) continue;
      newNotificationCount.value =
          (newNotificationCount.value ?? 0) + countIncrement;
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
  int notificationCountIncrement(T value) => 1;

  @visibleForOverriding
  void close();
}
