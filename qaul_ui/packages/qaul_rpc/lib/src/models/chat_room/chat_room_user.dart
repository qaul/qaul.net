part of 'chat_room.dart';

@immutable
class ChatRoomUser extends User {
  ChatRoomUser._(
    User u, {
    required this.joinedAt,
    this.roomId,
    this.role = ChatRoomUserRole.unknown,
    this.invitationState = InvitationState.unknown,
  }) : super(
          name: u.name,
          id: u.id,
          conversationId: u.conversationId,
          keyBase58: u.keyBase58,
          availableTypes: u.availableTypes,
          isBlocked: u.isBlocked,
          isVerified: u.isVerified,
          status: u.status,
        );

  final Uint8List? roomId;
  final ChatRoomUserRole role;
  final DateTime joinedAt;
  final InvitationState invitationState;

  factory ChatRoomUser.fromUser(User u, GroupMember m, {Uint8List? roomId}) {
    return ChatRoomUser._(
      u,
      roomId: Uint8List.fromList(m.userId),
      joinedAt: DateTime.fromMillisecondsSinceEpoch(m.joinedAt.toInt()),
      role: m.role == GroupMemberRole.User
          ? ChatRoomUserRole.normal
          : m.role == GroupMemberRole.Admin
              ? ChatRoomUserRole.admin
              : ChatRoomUserRole.unknown,
      invitationState: m.state == GroupMemberState.Invited
          ? InvitationState.sent
          : m.state == GroupMemberState.Activated
              ? InvitationState.accepted
              : InvitationState.unknown,
    );
  }

  @override
  List<Object?> get props => [name, idBase58, role, roomId];
}
