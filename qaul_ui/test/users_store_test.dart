import 'dart:typed_data';

import 'package:flutter_test/flutter_test.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:qaul_ui/stores/stores.dart';

import 'chat_tab/chat_tab_test.dart';

List<User> _testUsersForStore = [];

class _TestPaginatedUsersNotifier extends PaginatedDataNotifier<User> {
  @override
  PaginatedData<User> build() =>
      PaginatedData(data: _testUsersForStore, pagination: null);
}

class _MockWorkerForGetByUserID extends StubLibqaulWorker {
  _MockWorkerForGetByUserID(super.ref, {this.getUserByIdResult});
  final User? getUserByIdResult;
  @override
  Future<User?> getUserById(Uint8List userId) =>
      Future.value(getUserByIdResult);
}

void main() {
  setUp(() {
    _testUsersForStore = [];
  });

  test('getByUserID returns user when found in store state', () async {
    final userInState = User(
      name: 'In State',
      id: Uint8List.fromList('user_in_state_id'.codeUnits),
    );
    _testUsersForStore = [userInState];

    final container = ProviderContainer(
      overrides: [
        defaultUserProvider.overrideWith((_) => defaultUser),
        usersProvider.overrideWith(() => _TestPaginatedUsersNotifier()),
        qaulWorkerProvider.overrideWith(
            (ref) => _MockWorkerForGetByUserID(ref, getUserByIdResult: null)),
      ],
    );
    addTearDown(container.dispose);

    final store = container.read(usersStoreProvider.notifier);
    final result = await store.getByUserID(userInState.idBase58);

    expect(result, isNotNull);
    expect(result!.idBase58, userInState.idBase58);
    expect(result.name, userInState.name);
  });

  test('getByUserID calls worker and returns user when not in state', () async {
    final userFromWorker = User(
      name: 'From Worker',
      id: Uint8List.fromList('user_from_worker_id'.codeUnits),
    );
    _testUsersForStore = [];

    final container = ProviderContainer(
      overrides: [
        defaultUserProvider.overrideWith((_) => defaultUser),
        usersProvider.overrideWith(() => _TestPaginatedUsersNotifier()),
        qaulWorkerProvider.overrideWith(
          (ref) =>
              _MockWorkerForGetByUserID(ref, getUserByIdResult: userFromWorker),
        ),
      ],
    );
    addTearDown(container.dispose);

    final store = container.read(usersStoreProvider.notifier);
    final result = await store.getByUserID(userFromWorker.idBase58);

    expect(result, isNotNull);
    expect(result!.idBase58, userFromWorker.idBase58);
    expect(result.name, userFromWorker.name);
  });

  test('getByUserID returns null when not in state and worker returns null',
      () async {
    _testUsersForStore = [];
    final unknownIdBase58 =
        User(name: 'X', id: Uint8List.fromList('unknown'.codeUnits)).idBase58;

    final container = ProviderContainer(
      overrides: [
        defaultUserProvider.overrideWith((_) => defaultUser),
        usersProvider.overrideWith(() => _TestPaginatedUsersNotifier()),
        qaulWorkerProvider.overrideWith(
            (ref) => _MockWorkerForGetByUserID(ref, getUserByIdResult: null)),
      ],
    );
    addTearDown(container.dispose);

    final store = container.read(usersStoreProvider.notifier);
    final result = await store.getByUserID(unknownIdBase58);

    expect(result, isNull);
  });

  test('getByUserID returns null on invalid base58', () async {
    _testUsersForStore = [];

    final container = ProviderContainer(
      overrides: [
        defaultUserProvider.overrideWith((_) => defaultUser),
        usersProvider.overrideWith(() => _TestPaginatedUsersNotifier()),
        qaulWorkerProvider.overrideWith(
            (ref) => _MockWorkerForGetByUserID(ref, getUserByIdResult: null)),
      ],
    );
    addTearDown(container.dispose);

    final store = container.read(usersStoreProvider.notifier);
    final result = await store.getByUserID('!!!invalid-base58!!!');

    expect(result, isNull);
  });

  test('store state excludes blocked users', () async {
    final normalUser = User(
      name: 'Normal',
      id: Uint8List.fromList('normal_id'.codeUnits),
    );
    final blockedUser = User(
      name: 'Blocked',
      id: Uint8List.fromList('blocked_id'.codeUnits),
      isBlocked: true,
    );
    _testUsersForStore = [normalUser, blockedUser];

    final container = ProviderContainer(
      overrides: [
        defaultUserProvider.overrideWith((_) => defaultUser),
        usersProvider.overrideWith(() => _TestPaginatedUsersNotifier()),
        qaulWorkerProvider.overrideWith(
            (ref) => _MockWorkerForGetByUserID(ref, getUserByIdResult: null)),
      ],
    );
    addTearDown(container.dispose);

    final state = container.read(usersStoreProvider);

    expect(state.length, 1);
    expect(state.single.idBase58, normalUser.idBase58);
    expect(state.any((u) => u.idBase58 == blockedUser.idBase58), isFalse);
  });
}
