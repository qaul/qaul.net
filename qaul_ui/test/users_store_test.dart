import 'dart:typed_data';

import 'package:flutter_test/flutter_test.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:qaul_ui/stores/stores.dart';

import 'chat_tab/chat_tab_test.dart';

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
  Future<PaginatedUsers?> getUsers({int? offset, int? limit}) async {
    lastOffset = offset;
    lastLimit = limit;
    return super.getUsers(offset: offset, limit: limit);
  }
}

/// A mock worker that returns a pre-configured list of users.
class _MockWorkerWithUsers extends StubLibqaulWorker {
  _MockWorkerWithUsers(super.ref, {required this.mockUsers});
  final List<User> mockUsers;

  @override
  Future<PaginatedUsers?> getUsers({int? offset, int? limit}) async {
    return PaginatedUsers(
      users: mockUsers,
      pagination: PaginationState(
        hasMore: false,
        total: mockUsers.length,
        offset: offset ?? 0,
        limit: limit ?? 50,
      ),
    );
  }
}

void main() {
  test('getByUserID returns author from store so feed can display message',
      () async {
    final userInState = User(
      name: 'In State',
      id: Uint8List.fromList('user_in_state_id'.codeUnits),
    );

    final container = ProviderContainer(
      overrides: [
        defaultUserProvider.overrideWith((_) => defaultUser),
        qaulWorkerProvider.overrideWith(
            (ref) => _MockWorkerWithUsers(ref, mockUsers: [userInState])),
      ],
    );
    addTearDown(container.dispose);

    final store = container.read(usersStoreProvider.notifier);
    // Seed state by fetching users
    await store.getUsers();

    final result = await store.getByUserID(userInState.idBase58);

    expect(result, isNotNull);
    expect(result!.idBase58, userInState.idBase58);
    expect(result.name, userInState.name);
  });

  test('getByUserID fetches author via worker when not in store', () async {
    final userFromWorker = User(
      name: 'From Worker',
      id: Uint8List.fromList('user_from_worker_id'.codeUnits),
    );

    final container = ProviderContainer(
      overrides: [
        defaultUserProvider.overrideWith((_) => defaultUser),
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

  test('getByUserID returns null when author unknown so feed skips message',
      () async {
    final unknownIdBase58 =
        User(name: 'X', id: Uint8List.fromList('unknown'.codeUnits)).idBase58;

    final container = ProviderContainer(
      overrides: [
        defaultUserProvider.overrideWith((_) => defaultUser),
        qaulWorkerProvider.overrideWith(
            (ref) => _MockWorkerForGetByUserID(ref, getUserByIdResult: null)),
      ],
    );
    addTearDown(container.dispose);

    final store = container.read(usersStoreProvider.notifier);
    final result = await store.getByUserID(unknownIdBase58);

    expect(result, isNull);
  });

  test('store state includes all users including blocked', () async {
    final normalUser = User(
      name: 'Normal',
      id: Uint8List.fromList('normal_id'.codeUnits),
    );
    final blockedUser = User(
      name: 'Blocked',
      id: Uint8List.fromList('blocked_id'.codeUnits),
      isBlocked: true,
    );

    final container = ProviderContainer(
      overrides: [
        defaultUserProvider.overrideWith((_) => defaultUser),
        qaulWorkerProvider.overrideWith((ref) =>
            _MockWorkerWithUsers(ref, mockUsers: [normalUser, blockedUser])),
      ],
    );
    addTearDown(container.dispose);

    await container.read(usersStoreProvider.notifier).getUsers();
    final state = container.read(usersStoreProvider);

    expect(state.length, 2);
    expect(state.any((u) => u.idBase58 == normalUser.idBase58), isTrue);
    expect(state.any((u) => u.idBase58 == blockedUser.idBase58), isTrue);
  });

  test('store state includes all loaded users so list and feed can use them',
      () async {
    final users = List<User>.generate(
      60,
      (i) => User(
        name: 'User $i',
        id: Uint8List.fromList('user_$i'.codeUnits),
      ),
    );

    final container = ProviderContainer(
      overrides: [
        defaultUserProvider.overrideWith((_) => defaultUser),
        qaulWorkerProvider
            .overrideWith((ref) => _MockWorkerWithUsers(ref, mockUsers: users)),
      ],
    );
    addTearDown(container.dispose);

    await container.read(usersStoreProvider.notifier).getUsers();
    final state = container.read(usersStoreProvider);

    expect(state.length, 60);
  });

  test(
      'getMoreUsers calls worker with offset and limit so users tab can load more',
      () async {
    late _MockWorkerForGetMoreUsers mockWorker;
    final container = ProviderContainer(
      overrides: [
        defaultUserProvider.overrideWith((_) => defaultUser),
        qaulWorkerProvider.overrideWith((ref) {
          mockWorker = _MockWorkerForGetMoreUsers(ref);
          return mockWorker;
        }),
      ],
    );
    addTearDown(container.dispose);

    await container
        .read(usersStoreProvider.notifier)
        .getMoreUsers(23, limit: 10);

    expect(mockWorker.lastOffset, 23);
    expect(mockWorker.lastLimit, 10);
  });
}
