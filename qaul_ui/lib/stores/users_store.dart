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
        .where((u) => !(u.isBlocked ?? false))
        .toList();
    return users;
  }

  Future<User?> getByUserID(String idBase58) async {
    // TODO: first verify if user is in store state, otherwise fetch that individual using the new libqaul message
    throw UnimplementedError("TODO must call libqaul worker with new message (to be created)");
  }

  Future<List<User>> getMoreUsers(int offset) async {
    // TODO: needs to be refactored from qaul_ui/lib/screens/home/tabs/users_tab.dart
    throw UnimplementedError("TODO must call libqaul worker with pagination controls");
  }
}
