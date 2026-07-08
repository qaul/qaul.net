part of 'stores.dart';

final chatRoomsStoreProvider = NotifierProvider<ChatRoomsStore, void>(
  ChatRoomsStore.new,
);

final chatRoomsSearchProvider =
    NotifierProvider<ChatRoomsSearchStore, ChatRoomsSearchState>(
  ChatRoomsSearchStore.new,
);

class ChatRoomsSearchState {
  const ChatRoomsSearchState({
    this.query = '',
    this.results = const [],
    this.pagination,
    this.isLoading = false,
  });

  final String query;
  final List<ChatRoom> results;
  final PaginationState? pagination;
  final bool isLoading;

  bool get isActive => query.trim().isNotEmpty;

  bool get hasMore => pagination?.hasMore ?? false;

  ChatRoomsSearchState copyWith({
    String? query,
    List<ChatRoom>? results,
    PaginationState? pagination,
    bool? isLoading,
  }) {
    return ChatRoomsSearchState(
      query: query ?? this.query,
      results: results ?? this.results,
      pagination: pagination ?? this.pagination,
      isLoading: isLoading ?? this.isLoading,
    );
  }
}

class ChatRoomsSearchStore extends Notifier<ChatRoomsSearchState> {
  static const _debounceDuration = Duration(milliseconds: 250);
  static const _pageSize = ChatRoomsStore.defaultPageSize;

  Timer? _debounceTimer;
  int _requestGeneration = 0;
  bool _loadMoreInFlight = false;

  @override
  ChatRoomsSearchState build() {
    ref.onDispose(() => _debounceTimer?.cancel());
    return const ChatRoomsSearchState();
  }

  void setQuery(String query) {
    _debounceTimer?.cancel();
    if (query.trim().isEmpty) {
      clear();
      return;
    }

    _requestGeneration++;
    state = state.copyWith(query: query, isLoading: true);
    _debounceTimer = Timer(
      _debounceDuration,
      () => _search(offset: 0, replace: true),
    );
  }

  void clear() {
    _debounceTimer?.cancel();
    _requestGeneration++;
    state = const ChatRoomsSearchState();
  }

  Future<void> loadMore() async {
    if (_loadMoreInFlight ||
        !state.isActive ||
        state.isLoading ||
        !state.hasMore) {
      return;
    }
    final pagination = state.pagination;
    if (pagination == null) return;

    _loadMoreInFlight = true;
    try {
      await _search(
        offset: pagination.offset + pagination.limit,
        replace: false,
      );
    } finally {
      _loadMoreInFlight = false;
    }
  }

  Future<void> refresh() async {
    if (!state.isActive) return;
    _debounceTimer?.cancel();
    await _search(offset: 0, replace: true);
  }

  bool _isCurrentSearch(int generation, String requestedQuery) {
    return ref.mounted &&
        generation == _requestGeneration &&
        requestedQuery == state.query.trim();
  }

  Future<void> _search({required int offset, required bool replace}) async {
    final requestedQuery = state.query.trim();
    if (requestedQuery.isEmpty) return;

    final generation = ++_requestGeneration;
    state = state.copyWith(isLoading: true);

    final result = await ref.read(qaulWorkerProvider).searchChatRooms(
          query: requestedQuery,
          offset: offset,
          limit: _pageSize,
        );

    if (!_isCurrentSearch(generation, requestedQuery)) return;

    if (result == null) {
      if (_isCurrentSearch(generation, requestedQuery)) {
        state = state.copyWith(isLoading: false);
      }
      return;
    }

    final roomsStore = ref.read(chatRoomsStoreProvider.notifier);
    await roomsStore.resolveUsersForRooms(result.rooms);
    if (!_isCurrentSearch(generation, requestedQuery)) return;

    final hydrated = _hydrateFromGlobalState(result.rooms);
    final filtered = _withoutBlockedRooms(hydrated);

    if (!_isCurrentSearch(generation, requestedQuery)) return;

    state = ChatRoomsSearchState(
      query: state.query,
      results: replace ? filtered : [...state.results, ...filtered],
      pagination: result.pagination,
      isLoading: false,
    );
  }

  List<ChatRoom> _hydrateFromGlobalState(List<ChatRoom> rooms) {
    final byId = {
      for (final r in ref.read(chatRoomsProvider)) r.idBase58: r,
    };
    return rooms
        .map((r) => byId[r.idBase58] ?? r)
        .toList(growable: false);
  }

  List<ChatRoom> _withoutBlockedRooms(List<ChatRoom> rooms) {
    final blockedIds = ref
        .read(usersStoreProvider)
        .where((u) => u.isBlocked ?? false)
        .map((u) => u.conversationId);
    return rooms
        .where((m) => !blockedIds.contains(m.conversationId))
        .toList(growable: false);
  }
}

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
    final state = ref.read(chatRoomsProvider.notifier);
    final pagination = result.pagination;
    if (pagination != null && pagination.offset > 0) {
      state.append(result.rooms);
    } else {
      state.mergeOrderedFromBackend(result.rooms);
    }
    await resolveUsersForRooms(result.rooms);
    if (!ref.mounted) return null;
    return PaginatedChatRooms(
      rooms: ref.read(chatRoomsProvider),
      pagination: result.pagination,
    );
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
    final inviteState = ref.read(groupInvitesProvider.notifier);
    final pagination = result.pagination;
    if (pagination != null && pagination.offset > 0) {
      inviteState.append(result.invites);
    } else {
      for (final invite in result.invites) {
        if (!inviteState.contains(invite)) {
          inviteState.add(invite);
        } else {
          inviteState.update(invite);
        }
      }
      final isCompleteList =
          pagination == null || (pagination.offset == 0 && !pagination.hasMore);
      if (isCompleteList) {
        inviteState.retainAll(result.invites);
      }
    }
    final rooms = result.invites.map((i) => i.groupDetails).toList();
    await resolveUsersForRooms(rooms);
    if (!ref.mounted) return null;
    await _hydrateGroupInvitesFromKnownUsers();
    return PaginatedGroupInvites(
      invites: ref.read(groupInvitesProvider),
      pagination: result.pagination,
    );
  }

  Future<void> pollChatRoomsAndInvites() async {
    await Future.wait([
      getChatRooms(offset: 0, limit: defaultPageSize),
      getGroupInvites(offset: 0, limit: defaultPageSize),
    ]);
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

  Future<void> resolveUsersForRooms(List<ChatRoom> rooms) async {
    await _resolveMissingUsersForRooms(rooms);
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
