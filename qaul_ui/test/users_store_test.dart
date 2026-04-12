import 'dart:async';
import 'dart:typed_data';

import 'package:fast_base58/fast_base58.dart';
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

/// Mock worker whose `getUserById` future is held open by an external
/// `Completer`, so tests can fire concurrent callers before the RPC resolves.
class _DeferredMockWorkerForGetByUserID extends StubLibqaulWorker {
  _DeferredMockWorkerForGetByUserID(super.ref);
  final completer = Completer<User?>();
  int callCount = 0;

  @override
  Future<User?> getUserById(Uint8List userId) {
    callCount++;
    return completer.future;
  }
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

class _MockWorkerForOnlineUsers extends StubLibqaulWorker {
  _MockWorkerForOnlineUsers(super.ref, {required this.onlineUsers});
  final List<User> onlineUsers;

  @override
  Future<PaginatedUsers?> getOnlineUsers({int? offset, int? limit}) async {
    return PaginatedUsers(
      users: onlineUsers,
      pagination: PaginationState(
        hasMore: false,
        total: onlineUsers.length,
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

  group('getByUserID dedup', () {
    test('concurrent callers share a single RPC and one state merge',
        () async {
      final user = User(
        name: 'Shared',
        id: Uint8List.fromList('shared_user_id'.codeUnits),
      );

      late _DeferredMockWorkerForGetByUserID mockWorker;
      final container = ProviderContainer(
        overrides: [
          defaultUserProvider.overrideWith((_) => defaultUser),
          qaulWorkerProvider.overrideWith((ref) {
            mockWorker = _DeferredMockWorkerForGetByUserID(ref);
            return mockWorker;
          }),
        ],
      );
      addTearDown(container.dispose);

      final store = container.read(usersStoreProvider.notifier);

      // Fire three concurrent callers BEFORE the RPC resolves.
      final f1 = store.getByUserID(user.idBase58);
      final f2 = store.getByUserID(user.idBase58);
      final f3 = store.getByUserID(user.idBase58);

      // Only one underlying RPC should have been dispatched.
      expect(mockWorker.callCount, 1);

      mockWorker.completer.complete(user);
      final results = await Future.wait([f1, f2, f3]);

      expect(results.every((r) => r?.idBase58 == user.idBase58), isTrue);
      expect(mockWorker.callCount, 1);

      // The user is merged into state exactly once.
      final state = container.read(usersStoreProvider);
      expect(state.where((u) => u.idBase58 == user.idBase58).length, 1);
    });

    test('in-flight cache is released after the RPC completes', () async {
      final user = User(
        name: 'Released',
        id: Uint8List.fromList('released_user_id'.codeUnits),
      );

      late _DeferredMockWorkerForGetByUserID mockWorker;
      final container = ProviderContainer(
        overrides: [
          defaultUserProvider.overrideWith((_) => defaultUser),
          qaulWorkerProvider.overrideWith((ref) {
            mockWorker = _DeferredMockWorkerForGetByUserID(ref);
            return mockWorker;
          }),
        ],
      );
      addTearDown(container.dispose);

      final store = container.read(usersStoreProvider.notifier);

      final first = store.getByUserID(user.idBase58);
      mockWorker.completer.complete(user);
      await first;

      // Second call hits the local-first check and does not dispatch a new RPC.
      final cached = await store.getByUserID(user.idBase58);
      expect(cached?.idBase58, user.idBase58);
      expect(mockWorker.callCount, 1);
    });

    test('error path dedups, returns null, and clears the in-flight entry',
        () async {
      final unknownIdBase58 = User(
        name: 'X',
        id: Uint8List.fromList('error_user_id'.codeUnits),
      ).idBase58;

      late _DeferredMockWorkerForGetByUserID mockWorker;
      final container = ProviderContainer(
        overrides: [
          defaultUserProvider.overrideWith((_) => defaultUser),
          qaulWorkerProvider.overrideWith((ref) {
            mockWorker = _DeferredMockWorkerForGetByUserID(ref);
            return mockWorker;
          }),
        ],
      );
      addTearDown(container.dispose);

      final store = container.read(usersStoreProvider.notifier);

      final f1 = store.getByUserID(unknownIdBase58);
      final f2 = store.getByUserID(unknownIdBase58);
      expect(mockWorker.callCount, 1);

      mockWorker.completer.completeError(StateError('boom'));
      final results = await Future.wait([f1, f2]);

      expect(results, [null, null]);
      expect(mockWorker.callCount, 1);
      expect(
        container
            .read(usersStoreProvider)
            .any((u) => u.idBase58 == unknownIdBase58),
        isFalse,
      );

      // A follow-up call must dispatch a fresh RPC, proving the `finally`
      // in `_fetchAndMergeUser` cleared the in-flight entry on the error path.
      final follow = store.getByUserID(unknownIdBase58);
      expect(mockWorker.callCount, 2);
      // The mock's completer has already errored; the new fetch awaits the
      // same already-errored future and is caught by the inner `try/catch`.
      expect(await follow, isNull);
    });
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

  group('getOnlineUsers', () {
    test('merges online users into state without replacing existing', () async {
      final existingUser = User(
        name: 'Existing',
        id: Uint8List.fromList('existing_id'.codeUnits),
      );
      final onlineUser = User(
        name: 'Online',
        id: Uint8List.fromList('online_id'.codeUnits),
        status: ConnectionStatus.online,
      );

      final container = ProviderContainer(
        overrides: [
          defaultUserProvider.overrideWith((_) => defaultUser),
          qaulWorkerProvider.overrideWith((ref) => _CombinedMockWorker(
                ref,
                seedUsers: [existingUser],
                onlineUsers: [onlineUser],
              )),
        ],
      );
      addTearDown(container.dispose);

      final store = container.read(usersStoreProvider.notifier);
      await store.getUsers();
      expect(container.read(usersStoreProvider).length, 1);

      await store.getOnlineUsers();

      final state = container.read(usersStoreProvider);
      expect(state.length, 2);
      expect(state.any((u) => u.idBase58 == existingUser.idBase58), isTrue);
      expect(state.any((u) => u.idBase58 == onlineUser.idBase58), isTrue);
    });

    test('updates existing user via merge when online response contains it',
        () async {
      final userId = Uint8List.fromList('user_a'.codeUnits);
      final existingUser = User(
        name: 'UserA',
        id: userId,
        status: ConnectionStatus.offline,
      );
      final onlineVersion = User(
        name: 'UserA',
        id: userId,
        status: ConnectionStatus.online,
        availableTypes: {ConnectionType.lan: const ConnectionInfo(ping: 10)},
      );

      late _CombinedMockWorker mockWorker;
      final container = ProviderContainer(
        overrides: [
          defaultUserProvider.overrideWith((_) => defaultUser),
          qaulWorkerProvider.overrideWith((ref) {
            mockWorker = _CombinedMockWorker(
              ref,
              seedUsers: [existingUser],
              onlineUsers: [onlineVersion],
            );
            return mockWorker;
          }),
        ],
      );
      addTearDown(container.dispose);

      final store = container.read(usersStoreProvider.notifier);
      await store.getUsers();

      expect(container.read(usersStoreProvider).first.status,
          ConnectionStatus.offline);

      await store.getOnlineUsers();

      final updated = container.read(usersStoreProvider).first;
      expect(updated.status, ConnectionStatus.online);
      expect(updated.availableTypes?[ConnectionType.lan]?.ping, 10);
    });
  });

  group('refreshUser', () {
    test('always calls worker, bypassing local cache', () async {
      final userId = Uint8List.fromList('refresh_user'.codeUnits);
      final user = User(
        name: 'Refresh',
        id: userId,
        status: ConnectionStatus.online,
      );
      final refreshedUser = User(
        name: 'Refresh',
        id: userId,
        status: ConnectionStatus.offline,
      );

      late _CombinedMockWorker mockWorker;
      final container = ProviderContainer(
        overrides: [
          defaultUserProvider.overrideWith((_) => defaultUser),
          qaulWorkerProvider.overrideWith((ref) {
            mockWorker = _CombinedMockWorker(
              ref,
              seedUsers: [user],
              onlineUsers: [],
              getUserByIdResults: {user.idBase58: refreshedUser},
            );
            return mockWorker;
          }),
        ],
      );
      addTearDown(container.dispose);

      final store = container.read(usersStoreProvider.notifier);
      await store.getUsers();

      // User is already in state, but refreshUser should still call worker
      final result = await store.refreshUser(user.idBase58);
      expect(result, isNotNull);
      expect(mockWorker.getUserByIdCallCount, 1);
    });

    test('updates existing user in state via merge', () async {
      final userId = Uint8List.fromList('merge_user'.codeUnits);
      final original = User(
        name: 'Original',
        id: userId,
        status: ConnectionStatus.online,
        keyBase58: 'key123',
      );
      final refreshed = User(
        name: 'Original',
        id: userId,
        status: ConnectionStatus.offline,
      );

      late _CombinedMockWorker mockWorker;
      final container = ProviderContainer(
        overrides: [
          defaultUserProvider.overrideWith((_) => defaultUser),
          qaulWorkerProvider.overrideWith((ref) {
            mockWorker = _CombinedMockWorker(
              ref,
              seedUsers: [original],
              onlineUsers: [],
              getUserByIdResults: {original.idBase58: refreshed},
            );
            return mockWorker;
          }),
        ],
      );
      addTearDown(container.dispose);

      final store = container.read(usersStoreProvider.notifier);
      await store.getUsers();
      await store.refreshUser(original.idBase58);

      final updated = container.read(usersStoreProvider).first;
      // _mergeUser keeps current status when incoming is offline
      expect(updated.status, ConnectionStatus.online);
      // _mergeUser preserves keyBase58 from incoming (null) ?? current
      expect(updated.keyBase58, 'key123');
    });
  });

  group('_mergeUser', () {
    test('preserves current name when incoming has undefined name', () async {
      final userId = Uint8List.fromList('name_test'.codeUnits);
      final current = User(name: 'RealName', id: userId);
      final incoming = User(name: 'Name Undefined', id: userId);

      late _CombinedMockWorker mockWorker;
      final container = ProviderContainer(
        overrides: [
          defaultUserProvider.overrideWith((_) => defaultUser),
          qaulWorkerProvider.overrideWith((ref) {
            mockWorker = _CombinedMockWorker(
              ref,
              seedUsers: [current],
              onlineUsers: [incoming],
            );
            return mockWorker;
          }),
        ],
      );
      addTearDown(container.dispose);

      await container.read(usersStoreProvider.notifier).getUsers();
      await container.read(usersStoreProvider.notifier).getOnlineUsers();

      final merged = container.read(usersStoreProvider).first;
      expect(merged.name, 'RealName');
    });

    test('keeps current status when incoming is offline', () async {
      final userId = Uint8List.fromList('status_test'.codeUnits);
      final current = User(
        name: 'User',
        id: userId,
        status: ConnectionStatus.online,
      );
      final incoming = User(
        name: 'User',
        id: userId,
        status: ConnectionStatus.offline,
      );

      final container = ProviderContainer(
        overrides: [
          defaultUserProvider.overrideWith((_) => defaultUser),
          qaulWorkerProvider.overrideWith((ref) => _CombinedMockWorker(
                ref,
                seedUsers: [current],
                onlineUsers: [incoming],
              )),
        ],
      );
      addTearDown(container.dispose);

      await container.read(usersStoreProvider.notifier).getUsers();
      await container.read(usersStoreProvider.notifier).getOnlineUsers();

      expect(
          container.read(usersStoreProvider).first.status, ConnectionStatus.online);
    });

    test('takes incoming availableTypes when present', () async {
      final userId = Uint8List.fromList('types_test'.codeUnits);
      final current = User(name: 'User', id: userId);
      final incoming = User(
        name: 'User',
        id: userId,
        availableTypes: {ConnectionType.ble: const ConnectionInfo(ping: 5)},
      );

      final container = ProviderContainer(
        overrides: [
          defaultUserProvider.overrideWith((_) => defaultUser),
          qaulWorkerProvider.overrideWith((ref) => _CombinedMockWorker(
                ref,
                seedUsers: [current],
                onlineUsers: [incoming],
              )),
        ],
      );
      addTearDown(container.dispose);

      await container.read(usersStoreProvider.notifier).getUsers();
      await container.read(usersStoreProvider.notifier).getOnlineUsers();

      final merged = container.read(usersStoreProvider).first;
      expect(merged.availableTypes?[ConnectionType.ble]?.ping, 5);
    });
  });

  group('polling', () {
    test('startOnlinePolling and stopOnlinePolling control timer', () async {
      final container = ProviderContainer(
        overrides: [
          defaultUserProvider.overrideWith((_) => defaultUser),
          qaulWorkerProvider.overrideWith(
              (ref) => _MockWorkerForOnlineUsers(ref, onlineUsers: [])),
        ],
      );
      addTearDown(container.dispose);

      final store = container.read(usersStoreProvider.notifier);

      store.startOnlinePolling();
      // Starting again should not throw (cancels previous timer)
      store.startOnlinePolling();
      store.stopOnlinePolling();
      // Stopping again should not throw
      store.stopOnlinePolling();
    });
  });
}

/// Combined mock worker that supports both getUsers (for seeding) and
/// getOnlineUsers (for polling tests).
class _CombinedMockWorker extends StubLibqaulWorker {
  _CombinedMockWorker(
    super.ref, {
    required this.seedUsers,
    required this.onlineUsers,
    this.getUserByIdResults = const {},
  });

  final List<User> seedUsers;
  final List<User> onlineUsers;
  final Map<String, User?> getUserByIdResults;
  int getUserByIdCallCount = 0;

  @override
  Future<PaginatedUsers?> getUsers({int? offset, int? limit}) async {
    return PaginatedUsers(
      users: seedUsers,
      pagination: PaginationState(
        hasMore: false,
        total: seedUsers.length,
        offset: offset ?? 0,
        limit: limit ?? 50,
      ),
    );
  }

  @override
  Future<PaginatedUsers?> getOnlineUsers({int? offset, int? limit}) async {
    return PaginatedUsers(
      users: onlineUsers,
      pagination: PaginationState(
        hasMore: false,
        total: onlineUsers.length,
        offset: offset ?? 0,
        limit: limit ?? 50,
      ),
    );
  }

  @override
  Future<User?> getUserById(Uint8List userId) {
    getUserByIdCallCount++;
    return Future.value(getUserByIdResults[Base58Encode(userId)]);
  }
}
