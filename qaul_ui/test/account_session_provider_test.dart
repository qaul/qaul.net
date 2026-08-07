import 'dart:async';
import 'dart:typed_data';

import 'package:flutter_test/flutter_test.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:qaul_ui/providers/account_session_provider.dart';

import 'chat_tab/chat_tab_test.dart';

class _SessionWorker extends StubLibqaulWorker {
  _SessionWorker(
    super.ref, {
    this.user,
    this.authenticated = false,
    Future<bool>? initialized,
  }) : _initialized = initialized ?? Future.value(true);

  final Future<bool> _initialized;
  final User? user;
  final bool authenticated;
  int defaultUserAccountReads = 0;

  @override
  Future<bool> get initialized => _initialized;

  @override
  Future<User?> getDefaultUserAccount() async {
    defaultUserAccountReads++;
    return user;
  }

  @override
  Future<bool> getSessionStatus({Uint8List? userId}) async => authenticated;
}

User _testUser() => User(
      name: 'tester',
      id: Uint8List.fromList([1, 2, 3]),
    );

ProviderContainer _container({
  required _SessionWorker Function(Ref ref) workerBuilder,
}) {
  return ProviderContainer(
    overrides: [
      qaulWorkerProvider.overrideWith(workerBuilder),
    ],
  );
}

void main() {
  group('accountSessionProvider', () {
    test('reports signedIn when daemon session is active', () async {
      final container = _container(
        workerBuilder: (ref) => _SessionWorker(ref, user: _testUser(), authenticated: true),
      );
      addTearDown(container.dispose);

      expect(
        await container.read(accountSessionProvider.future),
        QaulAccountSessionState.signedIn,
      );
    });

    test('reports signedOut when account exists but session is inactive', () async {
      final container = _container(
        workerBuilder: (ref) => _SessionWorker(ref, user: _testUser()),
      );
      addTearDown(container.dispose);

      expect(
        await container.read(accountSessionProvider.future),
        QaulAccountSessionState.signedOut,
      );
    });

    test('reports noLocalAccount when there is no default user', () async {
      final container = _container(
        workerBuilder: (ref) => _SessionWorker(ref),
      );
      addTearDown(container.dispose);

      expect(
        await container.read(accountSessionProvider.future),
        QaulAccountSessionState.noLocalAccount,
      );
    });

    test('waits for the worker before reading the default account', () async {
      final initialized = Completer<bool>();
      late _SessionWorker worker;
      final container = _container(
        workerBuilder: (ref) {
          worker = _SessionWorker(ref, initialized: initialized.future);
          return worker;
        },
      );
      addTearDown(container.dispose);

      final session = container.read(accountSessionProvider.future);
      await Future<void>.delayed(Duration.zero);

      expect(worker.defaultUserAccountReads, 0);

      initialized.complete(true);

      expect(await session, QaulAccountSessionState.noLocalAccount);
      expect(worker.defaultUserAccountReads, 1);
    });

    test('forceSignedOut skips daemon check after explicit logout', () async {
      final container = _container(
        workerBuilder: (ref) => _SessionWorker(
          ref,
          user: _testUser(),
          authenticated: true,
        ),
      );
      container.read(forceSignedOutProvider.notifier).state = true;
      addTearDown(container.dispose);

      expect(
        await container.read(accountSessionProvider.future),
        QaulAccountSessionState.signedOut,
      );
      await Future<void>.delayed(Duration.zero);
      expect(container.read(forceSignedOutProvider), isFalse);
    });
  });
}
