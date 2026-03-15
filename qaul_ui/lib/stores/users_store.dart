part of 'stores.dart';

final usersStoreProvider = NotifierProvider<UsersStore, List<User>>(
  UsersStore.new,
);

class UsersStore extends Notifier<List<User>> {
  PaginationState? _pagination;
  PaginationState? get pagination => _pagination;

  @override
  List<User> build() => [];

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

  void _appendMany(List<User> items) {
    final existingIds = state.map((u) => u.idBase58).toSet();
    final newUsers =
        items.where((u) => !existingIds.contains(u.idBase58)).toList();
    if (newUsers.isEmpty) return;
    state = [...state, ...newUsers];
  }

  void _syncLookup() {
    ref.read(userLookupProvider.notifier).state = state;
  }
}
