import 'dart:typed_data';

import 'package:flutter_test/flutter_test.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:qaul_ui/stores/stores.dart';

import 'chat_tab/chat_tab_test.dart';

class _GroupStoreWorker extends StubLibqaulWorker {
  _GroupStoreWorker(super.ref, {required this.rooms, this.invites = const []});

  final List<ChatRoom> rooms;
  final List<GroupInvite> invites;

  @override
  Future<PaginatedChatRooms?> getAllChatRooms({int? offset, int? limit}) async {
    return PaginatedChatRooms(rooms: rooms);
  }

  @override
  Future<PaginatedGroupInvites?> getGroupInvitesReceived({
    int? offset,
    int? limit,
  }) async {
    return PaginatedGroupInvites(invites: invites);
  }
}

ChatRoom _room(String id, String name) {
  return ChatRoom(
    conversationId: Uint8List.fromList(id.codeUnits),
    name: name,
    isDirectChat: false,
  );
}

ProviderContainer _container(
  _GroupStoreWorker Function(Ref ref) workerBuilder,
) {
  return ProviderContainer(
    overrides: [qaulWorkerProvider.overrideWith(workerBuilder)],
  );
}

void main() {
  group('ChatRoomsStore', () {
    test('publishes fetched chat rooms to chatRoomsProvider', () async {
      final backendRoom = _room('group-1', 'New group');
      final container = _container(
        (ref) => _GroupStoreWorker(ref, rooms: [backendRoom]),
      );
      addTearDown(container.dispose);

      expect(container.read(chatRoomsProvider), isEmpty);

      final rooms = await container
          .read(chatRoomsStoreProvider.notifier)
          .refreshChatRooms();

      expect(rooms, hasLength(1));
      expect(rooms.single.idBase58, backendRoom.idBase58);
      expect(container.read(chatRoomsProvider), hasLength(1));
      expect(
        container.read(chatRoomsProvider).single.idBase58,
        backendRoom.idBase58,
      );
    });

    test('publishes fetched group invites to groupInvitesProvider', () async {
      final group = _room('group-2', 'Invited group');
      final invite = GroupInvite(
        senderId: Uint8List.fromList('sender'.codeUnits),
        receivedAt: DateTime(2026, 1, 1),
        groupDetails: group,
      );
      final container = _container(
        (ref) => _GroupStoreWorker(ref, rooms: const [], invites: [invite]),
      );
      addTearDown(container.dispose);

      expect(container.read(groupInvitesProvider), isEmpty);

      final invites = await container
          .read(chatRoomsStoreProvider.notifier)
          .refreshGroupInvites();

      expect(invites, hasLength(1));
      expect(invites.single, invite);
      expect(container.read(groupInvitesProvider), [invite]);
    });
  });
}
