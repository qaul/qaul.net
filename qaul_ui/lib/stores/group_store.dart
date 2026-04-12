part of 'stores.dart';

final groupStoreProvider = NotifierProvider<GroupStore, void>(GroupStore.new);

class GroupStore extends Notifier<void> {
  @override
  void build() {}

  Future<List<ChatRoom>> refreshChatRooms() async {
    final worker = ref.read(qaulWorkerProvider);
    final rooms = await worker.getAllChatRooms();
    if (!ref.mounted) return [];
    await _resolveMissingUsersForRooms(rooms);
    if (!ref.mounted) return [];
    return ref.read(chatRoomsProvider);
  }

  Future<List<GroupInvite>> refreshGroupInvites() async {
    final worker = ref.read(qaulWorkerProvider);
    final invites = await worker.getGroupInvitesReceived();
    if (!ref.mounted) return [];
    final rooms = invites.map((i) => i.groupDetails).toList();
    await _resolveMissingUsersForRooms(rooms);
    if (!ref.mounted) return [];

    final usersById = {
      for (final u in ref.read(usersStoreProvider)) u.idBase58: u,
    };
    final inviteState = ref.read(groupInvitesProvider.notifier);
    for (final invite in ref.read(groupInvitesProvider)) {
      final hydratedRoom = _hydrateRoomMembers(invite.groupDetails, usersById);
      if (hydratedRoom == invite.groupDetails) continue;
      inviteState.update(
        GroupInvite(
          senderId: invite.senderId,
          receivedAt: invite.receivedAt,
          groupDetails: hydratedRoom,
        ),
      );
    }
    return ref.read(groupInvitesProvider);
  }

  Future<void> _resolveMissingUsersForRooms(List<ChatRoom> rooms) async {
    final knownIds = ref.read(usersStoreProvider).map((u) => u.idBase58).toSet();
    final missingIds = <String>{};

    for (final room in rooms) {
      for (final member in room.members) {
        if (!knownIds.contains(member.idBase58)) {
          missingIds.add(member.idBase58);
        }
      }
    }

    if (missingIds.isNotEmpty) {
      final usersStore = ref.read(usersStoreProvider.notifier);
      await Future.wait(missingIds.map(usersStore.getByUserID));
    }
    if (!ref.mounted) return;

    final usersById = {
      for (final u in ref.read(usersStoreProvider)) u.idBase58: u,
    };
    final roomsState = ref.read(chatRoomsProvider.notifier);
    for (final room in ref.read(chatRoomsProvider)) {
      final hydrated = _hydrateRoomMembers(room, usersById);
      if (hydrated == room) continue;
      roomsState.update(hydrated);
    }
  }

  ChatRoom _hydrateRoomMembers(ChatRoom room, Map<String, User> usersById) {
    var changed = false;
    final members = room.members.map((member) {
      final user = usersById[member.idBase58];
      if (user == null) return member;
      if (member.name == user.name) return member;
      changed = true;
      return ChatRoomUser(
        user,
        joinedAt: member.joinedAt,
        roomId: member.roomId,
        role: member.role,
        invitationState: member.invitationState,
      );
    }).toList();

    if (!changed) return room;
    return room.copyWith(members: members);
  }
}
