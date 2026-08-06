import 'dart:typed_data';

import 'package:flutter/widgets.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:qaul_ui/providers/providers.dart';
import 'package:qaul_ui/providers/session_state_reset.dart';
import 'package:qaul_ui/stores/stores.dart';

void main() {
  User user(String name, int byte) => User(
        name: name,
        id: Uint8List.fromList(List<int>.filled(38, byte)),
      );

  ChatRoom room(String id) => ChatRoom(
        conversationId: Uint8List.fromList(id.codeUnits),
        name: id,
      );

  testWidgets('resetSessionScopedState clears stale account scoped providers',
      (tester) async {
    final container = ProviderContainer();
    addTearDown(container.dispose);
    late VoidCallback reset;

    final oldUser = user('Old', 1);
    final oldRoom = room('old-room');
    final invite = GroupInvite(
      senderId: oldUser.id,
      receivedAt: DateTime(2026),
      groupDetails: oldRoom,
    );

    container.read(defaultUserProvider.notifier).state = oldUser;
    container.read(userLookupProvider.notifier).state = [oldUser];
    container.read(chatRoomsProvider.notifier).add(oldRoom);
    container.read(currentOpenChatRoom.notifier).state = oldRoom;
    container.read(groupInvitesProvider.notifier).add(invite);
    container.read(homeScreenControllerProvider.notifier).goToTab(TabType.chat);

    await tester.pumpWidget(
      UncontrolledProviderScope(
        container: container,
        child: Consumer(
          builder: (_, ref, _) {
            reset = () => resetSessionScopedState(ref);
            return const SizedBox.shrink();
          },
        ),
      ),
    );

    reset();
    await tester.pump();

    expect(container.read(defaultUserProvider), isNull);
    expect(container.read(userLookupProvider), isEmpty);
    expect(container.read(chatRoomsProvider), isEmpty);
    expect(container.read(currentOpenChatRoom), isNull);
    expect(container.read(groupInvitesProvider), isEmpty);
    expect(container.read(homeScreenControllerProvider), TabType.public);
  });

  testWidgets('resetSessionScopedState preserves the newly active user',
      (tester) async {
    final container = ProviderContainer();
    addTearDown(container.dispose);
    late VoidCallback reset;

    final newUser = user('New', 2);

    await tester.pumpWidget(
      UncontrolledProviderScope(
        container: container,
        child: Consumer(
          builder: (_, ref, _) {
            reset = () => resetSessionScopedState(ref, activeUser: newUser);
            return const SizedBox.shrink();
          },
        ),
      ),
    );

    reset();
    await tester.pump();

    expect(container.read(defaultUserProvider), newUser);
    container.read(usersStoreProvider.notifier).stopOnlinePolling();
  });
}
