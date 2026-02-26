import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:local_notifications/local_notifications.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:qaul_ui/providers/providers.dart';
import 'package:shared_preferences/shared_preferences.dart';

final _stubPublicMessagesProvider = Provider<List<PublicPost>>((ref) => []);

final _stubChatRoomsProvider = Provider<List<ChatRoom>>((ref) => []);

class WidgetbookStubPublicNotificationController
    extends PublicNotificationController {
  WidgetbookStubPublicNotificationController(super.ref);

  @override
  String get cacheKey => 'widgetbook_public';

  @override
  MapEntry<dynamic, void Function(List<PublicPost>?, List<PublicPost>)>
      get strategy => MapEntry(_stubPublicMessagesProvider, (_, _) {});

  @override
  Future<void> initialize() async {
    await super.initialize();
  }

  @override
  void updatePersistentCachedData() {}

  @override
  Iterable<PublicPost> entriesToBeProcessed(List<PublicPost> values) => [];

  @override
  LocalNotification? process(PublicPost value) => null;

  @override
  void close() {}
}

class WidgetbookStubChatNotificationController
    extends ChatNotificationController {
  WidgetbookStubChatNotificationController(super.ref);

  @override
  String get cacheKey => 'widgetbook_chat';

  @override
  MapEntry<dynamic, void Function(List<ChatRoom>?, List<ChatRoom>)>
      get strategy => MapEntry(_stubChatRoomsProvider, (_, _) {});

  @override
  Future<void> initialize() async {
    await super.initialize();
  }

  @override
  Iterable<ChatRoom> entriesToBeProcessed(List<ChatRoom> values) => [];

  @override
  void execute(List<ChatRoom>? previous, List<ChatRoom> current) {}

  @override
  User get localUser => throw UnimplementedError();

  @override
  SharedPreferences get preferences => throw UnimplementedError();

  @override
  LocalNotification? process(ChatRoom value) => null;

  @override
  void removeNotifications() {}

  @override
  void updatePersistentCachedData() {}
}
