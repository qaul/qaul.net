part of 'stores.dart';

final usersStoreProvider = NotifierProvider<UsersStore, List<User>>(
  UsersStore.new,
);

class UsersStore extends Notifier<List<User>> {
  @override
  List<User> build() {
    // TODO should only get first users page
    // TODO (refactor out of users tab, i.e. anything that calls qaulWorker directly on qaul_ui/lib/screens/home/tabs/users_tab.dart)
    final users = ref
        .watch(usersProvider)
        .data
        .where((u) => !(u.isBlocked ?? false))
        .toList();
    return users;
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

  Future<List<User>> getMoreUsers(int offset) async {
    // TODO: needs to be refactored from qaul_ui/lib/screens/home/tabs/users_tab.dart
    throw UnimplementedError("TODO must call libqaul worker with pagination controls");
  }
}
