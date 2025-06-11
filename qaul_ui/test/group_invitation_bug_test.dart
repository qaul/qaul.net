import 'dart:typed_data';

import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:qaul_rpc/qaul_rpc.dart';

import 'package:qaul_ui/screens/home/tabs/chat/widgets/chat.dart';

void main() {
  group('Group Invitation Tests', () {
    late User alice;
    late User bob;
    late User charlie;
    late _MockAppLocalizations l10n;

    setUpAll(() {
      alice = User(
        name: 'Alice',
        id: Uint8List.fromList('alice'.codeUnits),
      );
      bob = User(
        name: 'Bob',
        id: Uint8List.fromList('bob'.codeUnits),
      );
      charlie = User(
        name: 'Charlie',
        id: Uint8List.fromList('charlie'.codeUnits),
      );
      l10n = _MockAppLocalizations();
    });

    test(
        'when user accepts invitation, pending invites do not show "left the group"',
        () {
      // Scenario: Group chat where Alice is admin, Bob has pending invite, Charlie accepts invitation
      // Expected: Bob (pending invite) should NOT show "left the group" message

      final groupMembers = [
        ChatRoomUser(
          alice,
          // Admin who created group
          invitationState: InvitationState.accepted,
          role: ChatRoomUserRole.admin,
          joinedAt: DateTime.now(),
        ),
        ChatRoomUser(
          bob,
          // Has pending invitation only
          invitationState: InvitationState.sent,
          role: ChatRoomUserRole.normal,
          joinedAt: DateTime.now(),
        ),
        ChatRoomUser(
          charlie,
          // Just accepted invitation
          invitationState: InvitationState.accepted,
          role: ChatRoomUserRole.normal,
          joinedAt: DateTime.now(),
        ),
      ];

      final chatRoom = ChatRoom(
        conversationId: Uint8List.fromList('testgroup'.codeUnits),
        name: 'Test Group',
        isDirectChat: false,
        members: groupMembers,
        status: ChatRoomStatus.active,
        revisionNumber: 1,
        unreadCount: 0,
      );

      // Simulate the problematic scenario: system generates "left" event for pending user
      final bobLeftEvent = GroupEventContent(
        userId: bob.id,
        type: GroupEventContentType.left,
      );

      final message = ChatScreen.translateGroupEventMessage(
        bobLeftEvent,
        bob,
        l10n: l10n,
        room: chatRoom,
      );
      // Bob (who only had pending invite) does not show "left the group"
      expect(message, isEmpty);
    });
  });
}

class _MockAppLocalizations implements AppLocalizations {
  @override
  String groupEventLeft(String userName) => '"$userName" left the group';

  @override
  String groupEventJoined(String userName) => '"$userName" joined the group';

  @override
  String groupEventInvited(String userName) => '"$userName" was invited';

  @override
  String groupEventInviteAccepted(String userName) =>
      '"$userName" accepted the invite';

  @override
  String groupEventRemoved(String userName) => '"$userName" was removed';

  @override
  String get groupStateEventCreated => 'Group was created';

  @override
  String get groupStateEventClosed => 'Group was closed';

  @override
  dynamic noSuchMethod(Invocation invocation) => '';
}
