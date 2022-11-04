part of 'chat_tab_test.dart';

final defaultUser = User(
  name: 'defaultUser',
  id: Uint8List.fromList('id'.codeUnits),
);

class NullChatNotificationController implements ChatNotificationController {
  @override
  late ValueNotifier<int?> newNotificationCount;

  @override
  String get cacheKey => throw UnimplementedError();

  @override
  void close() {}

  @override
  TabType get currentVisibleHomeTab => throw UnimplementedError();

  @override
  Iterable<ChatRoom> entriesToBeProcessed(List<ChatRoom> values) {
    throw UnimplementedError();
  }

  @override
  void execute(List<ChatRoom>? previous, List<ChatRoom> current) {}

  @override
  Future<void> initialize() async {}

  @override
  User get localUser => throw UnimplementedError();

  @override
  SharedPreferences get preferences => throw UnimplementedError();

  @override
  LocalNotification? process(ChatRoom value) {
    throw UnimplementedError();
  }

  @override
  Ref get ref => throw UnimplementedError();

  @override
  void removeNotifications() {}

  @override
  MapEntry<AlwaysAliveProviderListenable<List<ChatRoom>>,
      void Function(List<ChatRoom>? p1, List<ChatRoom> p2)>
  get strategy => throw UnimplementedError();

  @override
  void updatePersistentCachedData() {}
}
