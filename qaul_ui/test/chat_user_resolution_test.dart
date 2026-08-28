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
  Future<PaginatedChatRooms?> getAllChatRooms({
    int? offset,
    int? limit,
  }) async => PaginatedChatRooms(rooms: []);

  @override
  Future<PaginatedGroupInvites?> getGroupInvitesReceived({
    int? offset,
    int? limit,
  }) async => PaginatedGroupInvites(invites: []);

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
  String get legacyCacheKey => '';

  @override
  Future<void> adoptLegacyValue(Object value) async {}

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
  int notificationCountIncrement(ChatRoom value) => 1;

  @override
  void removeNotifications() {}

  @override
  void updatePersistentCachedData() {}
}

class _DirectRoomNotifier extends ChatRoomListNotifier {
  @override
  List<ChatRoom> build() {
    final defaultUser = User(
      name: 'Default',
      id: Uint8List.fromList('default-user'.codeUnits),
    );
    final otherUser = User(
      name: 'Peer',
      id: Uint8List.fromList('peer-user'.codeUnits),
    );
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
    final defaultUser = User(
      name: 'Default',
      id: Uint8List.fromList('default-user'.codeUnits),
    );
    return [
      ChatRoom(
        conversationId: Uint8List.fromList('group-room'.codeUnits),
        isDirectChat: false,
        members: [ChatRoomUser(defaultUser, joinedAt: DateTime(2024))],
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

  test('matches full user ids with q8 chat sender ids', () {
    final fullUserId = Uint8List.fromList([
      0,
      1,
      2,
      3,
      4,
      5,
      10,
      11,
      12,
      13,
      14,
      15,
      16,
      17,
      18,
    ]);
    final q8SenderId = Uint8List.fromList(fullUserId.sublist(6, 14));
    final otherSenderId = Uint8List.fromList([10, 11, 12, 13, 14, 15, 16, 99]);

    expect(qaulUserIdsEqual(fullUserId, q8SenderId), isTrue);
    expect(qaulUserIdsEqual(fullUserId, otherSenderId), isFalse);
  });

  test('new direct rooms are calculated from the active local user', () {
    final activeUser = User(
      name: 'Active',
      id: Uint8List.fromList([
        0,
        1,
        2,
        3,
        4,
        5,
        20,
        21,
        22,
        23,
        24,
        25,
        26,
        27,
      ]),
    );
    final otherUser = User(
      name: 'Other',
      id: Uint8List.fromList([
        0,
        1,
        2,
        3,
        4,
        5,
        10,
        11,
        12,
        13,
        14,
        15,
        16,
        17,
      ]),
      conversationId: Uint8List.fromList(List.filled(16, 99)),
    );

    final room = ChatRoom.blank(localUser: activeUser, otherUser: otherUser);

    expect(room.conversationId, qaulDirectChatId(activeUser.id, otherUser.id));
    expect(room.conversationId, isNot(otherUser.conversationId));
  });

  testWidgets(
    'direct room renders using member fallback when users store is empty',
    (tester) async {
      final defaultUser = User(
        name: 'Default',
        id: Uint8List.fromList('default-user'.codeUnits),
      );

      await tester.pumpWidget(
        ProviderScope(
          overrides: [
            defaultUserProvider.overrideWith((_) => defaultUser),
            chatNotificationControllerProvider.overrideWithValue(
              _NoopChatNotificationController(),
            ),
            chatRoomsProvider.overrideWith(_DirectRoomNotifier.new),
            qaulWorkerProvider.overrideWith((ref) => _NoopWorker(ref)),
          ],
          child: materialAppWithLocalizations(BaseTab.chat()),
        ),
      );
      await tester.pumpAndSettle();

      expect(find.text('Peer'), findsOneWidget);
    },
  );

  testWidgets('group event with missing user shows unknown fallback', (
    tester,
  ) async {
    final defaultUser = User(
      name: 'Default',
      id: Uint8List.fromList('default-user'.codeUnits),
    );

    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          defaultUserProvider.overrideWith((_) => defaultUser),
          chatNotificationControllerProvider.overrideWithValue(
            _NoopChatNotificationController(),
          ),
          chatRoomsProvider.overrideWith(
            _GroupRoomWithUnknownEventNotifier.new,
          ),
          qaulWorkerProvider.overrideWith((ref) => _NoopWorker(ref)),
        ],
        child: materialAppWithLocalizations(BaseTab.chat()),
      ),
    );
    await tester.pumpAndSettle();

    expect(find.text('Unknown'), findsOneWidget);
  });
}
