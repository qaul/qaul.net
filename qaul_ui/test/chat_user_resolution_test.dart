import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:local_notifications/src/local_notifications.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:qaul_ui/providers/providers.dart';
import 'package:qaul_ui/screens/home/tabs/tab.dart';
import 'package:shared_preferences/shared_preferences.dart';

import 'test_utils/test_utils.dart';

class _NoopWorker implements LibqaulWorker {
  _NoopWorker(this.ref);
  final Ref ref;

  @override
  Future<List<ChatRoom>> getAllChatRooms() async => [];

  @override
  Future<List<GroupInvite>> getGroupInvitesReceived() async => [];

  @override
  Future<bool> get initialized => Future.value(true);

  @override
  Future<PaginatedUsers?> getUsers({int? offset, int? limit}) async {
    return PaginatedUsers(users: []);
  }

  @override
  noSuchMethod(Invocation invocation) => super.noSuchMethod(invocation);
}

class _NoopChatNotificationController implements ChatNotificationController {
  @override
  String get cacheKey => '';

  @override
  TabType get currentVisibleHomeTab => TabType.chat;

  @override
  User get localUser =>
      User(name: 'noop', id: Uint8List.fromList('noop'.codeUnits));

  @override
  SharedPreferences get preferences =>
      throw UnimplementedError('Not used in tests');

  @override
  Ref get ref => throw UnimplementedError('Not used in tests');

  @override
  MapEntry<dynamic, void Function(List<ChatRoom>?, List<ChatRoom>)>
      get strategy => const MapEntry(null, _noopStrategy);

  static void _noopStrategy(List<ChatRoom>? _, List<ChatRoom> _) {}

  @override
  ValueNotifier<int?> newNotificationCount = ValueNotifier<int?>(null);

  @override
  void close() {}

  @override
  Iterable<ChatRoom> entriesToBeProcessed(List<ChatRoom> values) => const [];

  @override
  void execute(List<ChatRoom>? previous, List<ChatRoom> current) {}

  @override
  Future<void> initialize() async {}

  @override
  LocalNotification? process(ChatRoom value) => null;

  @override
  void removeNotifications() {}

  @override
  void updatePersistentCachedData() {}
}

class _DirectRoomNotifier extends ChatRoomListNotifier {
  @override
  List<ChatRoom> build() {
    final defaultUser =
        User(name: 'Default', id: Uint8List.fromList('default-user'.codeUnits));
    final otherUser =
        User(name: 'Peer', id: Uint8List.fromList('peer-user'.codeUnits));
    return [
      ChatRoom(
        conversationId: Uint8List.fromList('dm-room'.codeUnits),
        isDirectChat: true,
        members: [
          ChatRoomUser(defaultUser, joinedAt: DateTime(2024)),
          ChatRoomUser(otherUser, joinedAt: DateTime(2024)),
        ],
        name: 'DM',
      ),
    ];
  }
}

class _GroupRoomWithUnknownEventNotifier extends ChatRoomListNotifier {
  @override
  List<ChatRoom> build() {
    final defaultUser =
        User(name: 'Default', id: Uint8List.fromList('default-user'.codeUnits));
    return [
      ChatRoom(
        conversationId: Uint8List.fromList('group-room'.codeUnits),
        isDirectChat: false,
        members: [
          ChatRoomUser(defaultUser, joinedAt: DateTime(2024)),
        ],
        name: 'Group',
        lastMessagePreview: GroupEventContent(
          userId: Uint8List.fromList('unknown-user'.codeUnits),
          type: GroupEventContentType.joined,
        ),
      ),
    ];
  }
}

void main() {
  setUp(() {
    SharedPreferences.setMockInitialValues({});
  });

  testWidgets('direct room renders using member fallback when users store is empty',
      (tester) async {
    final defaultUser =
        User(name: 'Default', id: Uint8List.fromList('default-user'.codeUnits));

    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          defaultUserProvider.overrideWith((_) => defaultUser),
          chatNotificationControllerProvider
              .overrideWithValue(_NoopChatNotificationController()),
          chatRoomsProvider.overrideWith(_DirectRoomNotifier.new),
          qaulWorkerProvider.overrideWith((ref) => _NoopWorker(ref)),
        ],
        child: materialAppWithLocalizations(BaseTab.chat()),
      ),
    );
    await tester.pumpAndSettle();

    expect(find.text('Peer'), findsOneWidget);
  });

  testWidgets('group event with missing user shows unknown fallback',
      (tester) async {
    final defaultUser =
        User(name: 'Default', id: Uint8List.fromList('default-user'.codeUnits));

    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          defaultUserProvider.overrideWith((_) => defaultUser),
          chatNotificationControllerProvider
              .overrideWithValue(_NoopChatNotificationController()),
          chatRoomsProvider.overrideWith(_GroupRoomWithUnknownEventNotifier.new),
          qaulWorkerProvider.overrideWith((ref) => _NoopWorker(ref)),
        ],
        child: materialAppWithLocalizations(BaseTab.chat()),
      ),
    );
    await tester.pumpAndSettle();

    expect(find.text('Unknown'), findsOneWidget);
  });
}
