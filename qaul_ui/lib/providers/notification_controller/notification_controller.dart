import 'dart:collection';

import 'package:local_notifications/local_notifications.dart';
import 'package:meta/meta.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:riverpod/riverpod.dart';
import 'package:shared_preferences/shared_preferences.dart';

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
  @visibleForOverriding
  Map<AlwaysAliveProviderListenable<T>, void Function(T?, T)> get strategies =>
      throw UnimplementedError('Must be implemented by child class');

  @protected
  @visibleForOverriding
  String get cacheKey => throw UnimplementedError('Must be implemented by child class');

  @mustCallSuper
  Future<void> initialize() async {
    _preferences = await SharedPreferences.getInstance();
    _user = ref.read(defaultUserProvider)!;
    for (final entry in strategies.entries) {
      ref.listen(entry.key, entry.value);
    }
  }

  @protected
  @visibleForOverriding
  void updatePersistentCachedData() =>
      throw UnimplementedError('Must be implemented by child class');
}

mixin DataProcessingStrategy<T> {
  void execute(List<T>? previous, List<T> current) async {
    final queue = Queue<T>()..addAll(entriesToBeProcessed(current));
    if (queue.isEmpty) return;

    while (queue.isNotEmpty) {
      final entry = queue.removeFirst();
      final message = process(entry);
      if (message == null) continue;
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
