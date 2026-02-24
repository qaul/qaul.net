import 'dart:typed_data';

import 'package:flutter_test/flutter_test.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:qaul_ui/stores/stores.dart';

import 'chat_tab/chat_tab_test.dart';

List<User> _testUsersForStore = [];
PaginationState? _testPagination;

class _TestPaginatedUsersNotifier extends PaginatedDataNotifier<User> {
  @override
  PaginatedData<User> build() =>
      PaginatedData(data: _testUsersForStore, pagination: _testPagination);
}

class _MockWorkerForGetByUserID extends StubLibqaulWorker {
  _MockWorkerForGetByUserID(super.ref, {this.getUserByIdResult});
  final User? getUserByIdResult;
  @override
  Future<User?> getUserById(Uint8List userId) =>
      Future.value(getUserByIdResult);
}

class _MockWorkerForGetMoreUsers extends StubLibqaulWorker {
  _MockWorkerForGetMoreUsers(super.ref);
  int? lastOffset;
  int? lastLimit;
  @override
  Future<void> getUsers({int? offset, int? limit}) async {
    lastOffset = offset;
    lastLimit = limit;
    return super.getUsers(offset: offset, limit: limit);
  }
}

void main() {
  setUp(() {
    _testUsersForStore = [];
    _testPagination = null;
  });

  test('getByUserID returns author from first page so feed can display message', () async {
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

  test('getByUserID fetches author via worker when not in first page', () async {
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

  test('getByUserID returns null when author unknown so feed skips message', () async {
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

  test('store state excludes blocked users so feed and list do not show them', () async {
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

  test('store state is first page only so feed uses getByUserID for rest', () async {
    _testUsersForStore = List<User>.generate(
      60,
      (i) => User(
        name: 'User $i',
        id: Uint8List.fromList('user_$i'.codeUnits),
      ),
    );
    _testPagination = const PaginationState(
      hasMore: true,
      total: 60,
      offset: 0,
      limit: 50,
    );

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

    expect(state.length, 50);
  });

  test('getMoreUsers calls worker with offset and limit so users tab can load more', () async {
    _testUsersForStore = [];

    late _MockWorkerForGetMoreUsers mockWorker;
    final container = ProviderContainer(
      overrides: [
        defaultUserProvider.overrideWith((_) => defaultUser),
        usersProvider.overrideWith(() => _TestPaginatedUsersNotifier()),
        qaulWorkerProvider.overrideWith((ref) {
          mockWorker = _MockWorkerForGetMoreUsers(ref);
          return mockWorker;
        }),
      ],
    );
    addTearDown(container.dispose);

    await container.read(usersStoreProvider.notifier).getMoreUsers(23,
        limit: 10);

    expect(mockWorker.lastOffset, 23);
    expect(mockWorker.lastLimit, 10);
  });
}
