part of 'stores.dart';

final usersStoreProvider = NotifierProvider<UsersStore, List<User>>(
  UsersStore.new,
);

class UsersStore extends Notifier<List<User>> {
  PaginationState? _pagination;
  PaginationState? get pagination => _pagination;

  Timer? _pollingTimer;
  static const _pollingInterval = Duration(seconds: 3);

  @override
  List<User> build() => [];

  // ---------------------------------------------------------------------------
  // Paginated user fetching
  // ---------------------------------------------------------------------------

  Future<PaginatedUsers?> getUsers({int offset = 0, int limit = 50}) async {
    final worker = ref.read(qaulWorkerProvider);
    final result = await worker.getUsers(offset: offset, limit: limit);
    if (result == null) return null;

    if (offset == 0) {
      state = result.users;
    } else {
      _appendMany(result.users);
    }
    _pagination = result.pagination;
    _syncLookup();
    return result;
  }

  Future<PaginatedUsers?> getMoreUsers(int offset, {int limit = 50}) async {
    return getUsers(offset: offset, limit: limit);
  }

  // ---------------------------------------------------------------------------
  // Single-user lookups
  // ---------------------------------------------------------------------------

  Future<User?> getByUserID(String idBase58) async {
    final match = state.where((u) => u.idBase58 == idBase58);
    if (match.isNotEmpty) return match.first;
    try {
      final worker = ref.read(qaulWorkerProvider);
      final userId = Uint8List.fromList(Base58Decode(idBase58));
      return worker.getUserById(userId);
    } catch (_) {
      return null;
    }
  }

  /// Always hits the RPC, bypassing the local-first check in [getByUserID].
  /// Updates the user in state via merge if found.
  Future<User?> refreshUser(String idBase58) async {
    try {
      final worker = ref.read(qaulWorkerProvider);
      final userId = Uint8List.fromList(Base58Decode(idBase58));
      final user = await worker.getUserById(userId);
      if (user != null) {
        _updateUser(user);
      }
      return user;
    } catch (_) {
      return null;
    }
  }

  // ---------------------------------------------------------------------------
  // Online user polling
  // ---------------------------------------------------------------------------

  /// Fetches only currently-online users and merges them into state.
  Future<PaginatedUsers?> getOnlineUsers() async {
    final worker = ref.read(qaulWorkerProvider);
    final result = await worker.getOnlineUsers();
    if (result == null) return null;
    _updateMany(result.users);
    _syncLookup();
    return result;
  }

  void startOnlinePolling() {
    stopOnlinePolling();
    _pollingTimer = Timer.periodic(
      _pollingInterval,
      (_) => _pollOnlineStatus(),
    );
  }

  void stopOnlinePolling() {
    _pollingTimer?.cancel();
    _pollingTimer = null;
  }

  Future<void> _pollOnlineStatus() async {
    final previouslyOnline = state
        .where((u) => u.isConnected)
        .map((u) => u.idBase58)
        .toSet();

    final result = await getOnlineUsers();
    if (result == null) return;

    final nowOnline = result.users.map((u) => u.idBase58).toSet();
    final wentOffline = previouslyOnline.difference(nowOnline);
    for (final id in wentOffline) {
      await refreshUser(id);
    }
  }

  // ---------------------------------------------------------------------------
  // State mutation helpers
  // ---------------------------------------------------------------------------

  void _appendMany(List<User> items) {
    final existingIds = state.map((u) => u.idBase58).toSet();
    final newUsers = items
        .where((u) => !existingIds.contains(u.idBase58))
        .toList();
    if (newUsers.isEmpty) return;
    state = [...state, ...newUsers];
  }

  void _updateUser(User incoming) {
    final idx = state.indexWhere((u) => u.idBase58 == incoming.idBase58);
    if (idx == -1) return;
    final merged = _mergeUser(state[idx], incoming);
    state = [...state]..[idx] = merged;
    _syncLookup();
  }

  void _updateMany(List<User> items) {
    final users = [...state];
    final indexById = <String, int>{};
    for (var i = 0; i < users.length; i++) {
      indexById[users[i].idBase58] = i;
    }
    for (final u in items) {
      final idx = indexById[u.idBase58];
      if (idx == null) {
        users.add(u);
        indexById[u.idBase58] = users.length - 1;
      } else {
        users[idx] = _mergeUser(users[idx], u);
      }
    }
    state = users;
  }

  static User _mergeUser(User current, User incoming) {
    return User(
      name: current.name == 'Name Undefined' ? incoming.name : current.name,
      id: incoming.id,
      conversationId: incoming.conversationId ?? current.conversationId,
      status: incoming.status == ConnectionStatus.offline
          ? current.status
          : incoming.status,
      keyBase58: incoming.keyBase58 ?? current.keyBase58,
      isBlocked: incoming.isBlocked ?? current.isBlocked,
      isVerified: incoming.isVerified ?? current.isVerified,
      availableTypes: incoming.availableTypes ?? current.availableTypes,
    );
  }

  void _syncLookup() {
    ref.read(userLookupProvider.notifier).state = state;
  }
}
