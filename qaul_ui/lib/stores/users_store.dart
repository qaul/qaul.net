part of 'stores.dart';

final usersStoreProvider = NotifierProvider<UsersStore, List<User>>(
  UsersStore.new,
);

class UsersStore extends Notifier<List<User>> {
  static const int _firstPageLimit = 50;

  @override
  List<User> build() {
    final paginated = ref.watch(usersProvider);
    final limit = paginated.pagination?.limit ?? _firstPageLimit;
    final firstPage = paginated.data.take(limit).toList();
    return firstPage.where((u) => !(u.isBlocked ?? false)).toList();
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

  Future<void> getMoreUsers(int offset, {int limit = 50}) async {
    final worker = ref.read(qaulWorkerProvider);
    await worker.getUsers(offset: offset, limit: limit);
  }
}
