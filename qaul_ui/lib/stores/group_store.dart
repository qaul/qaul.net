part of 'stores.dart';

final chatRoomsStoreProvider = NotifierProvider<ChatRoomsStore, void>(
  ChatRoomsStore.new,
);

@Deprecated('Use chatRoomsStoreProvider instead.')
final groupStoreProvider = chatRoomsStoreProvider;

class ChatRoomsStore extends Notifier<void> {
  static const defaultPageSize = 50;

  PaginationState? _chatRoomsPagination;
  PaginationState? _groupInvitesPagination;

  PaginationState? get chatRoomsPagination => _chatRoomsPagination;
  PaginationState? get groupInvitesPagination => _groupInvitesPagination;

  @override
  void build() {}

  Future<PaginatedChatRooms?> getChatRooms({
    int offset = 0,
    int limit = defaultPageSize,
  }) async {
    final worker = ref.read(qaulWorkerProvider);
    final result = await worker.getAllChatRooms(offset: offset, limit: limit);
    if (!ref.mounted || result == null) return null;
    _chatRoomsPagination = result.pagination;
    await _resolveMissingUsersForRooms(result.rooms);
    if (!ref.mounted) return null;
    return PaginatedChatRooms(
      rooms: ref.read(chatRoomsProvider),
      pagination: result.pagination,
    );
  }

  Future<PaginatedChatRooms?> getMoreChatRooms(
    int offset, {
    int limit = defaultPageSize,
  }) async {
    return getChatRooms(offset: offset, limit: limit);
  }

  Future<PaginatedGroupInvites?> getGroupInvites({
    int offset = 0,
    int limit = defaultPageSize,
  }) async {
    final worker = ref.read(qaulWorkerProvider);
    final result = await worker.getGroupInvitesReceived(
      offset: offset,
      limit: limit,
    );
    if (!ref.mounted || result == null) return null;
    _groupInvitesPagination = result.pagination;
    final rooms = result.invites.map((i) => i.groupDetails).toList();
    await _resolveMissingUsersForRooms(rooms);
    if (!ref.mounted) return null;
    await _hydrateGroupInvitesFromKnownUsers();
    return PaginatedGroupInvites(
      invites: ref.read(groupInvitesProvider),
      pagination: result.pagination,
    );
  }

  Future<PaginatedGroupInvites?> getMoreGroupInvites(
    int offset, {
    int limit = defaultPageSize,
  }) async {
    return getGroupInvites(offset: offset, limit: limit);
  }

  Future<List<ChatRoom>> refreshChatRooms({int limit = defaultPageSize}) async {
    final result = await getChatRooms(offset: 0, limit: limit);
    return result?.rooms ?? [];
  }

  Future<List<GroupInvite>> refreshGroupInvites({
    int limit = defaultPageSize,
  }) async {
    final result = await getGroupInvites(offset: 0, limit: limit);
    return result?.invites ?? [];
  }

  Future<void> _hydrateGroupInvitesFromKnownUsers() async {
    if (!ref.mounted) return;
    final invites = ref.read(groupInvitesProvider);
    if (invites.isEmpty) return;

    final usersById = {
      for (final u in ref.read(usersStoreProvider)) u.idBase58: u,
    };
    final inviteState = ref.read(groupInvitesProvider.notifier);
    for (final invite in invites) {
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
  }

  Future<void> _resolveMissingUsersForRooms(List<ChatRoom> rooms) async {
    final knownIds = ref
        .read(usersStoreProvider)
        .map((u) => u.idBase58)
        .toSet();
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
