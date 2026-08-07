import 'dart:io';
import 'dart:typed_data';

import 'package:flutter_test/flutter_test.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:qaul_ui/providers/account_session_provider.dart';
import 'package:qaul_ui/providers/providers.dart';
import 'package:qaul_ui/session/session_scope.dart';
import 'package:qaul_ui/stores/stores.dart';

// `StubLibqaulWorker` is a part of this library, as in group_store_test.dart.
import 'chat_tab/chat_tab_test.dart';

const _pagination = PaginationState(
  hasMore: true,
  total: 10,
  offset: 0,
  limit: 5,
);

class _PaginatingWorker extends StubLibqaulWorker {
  _PaginatingWorker(super.ref);

  @override
  Future<PaginatedChatRooms?> getAllChatRooms({int? offset, int? limit}) async =>
      PaginatedChatRooms(rooms: const [], pagination: _pagination);

  @override
  Future<PaginatedGroupInvites?> getGroupInvitesReceived({
    int? offset,
    int? limit,
  }) async =>
      PaginatedGroupInvites(invites: const [], pagination: _pagination);
}

User _user(String name, int byte) => User(
      name: name,
      id: Uint8List.fromList(List<int>.filled(38, byte)),
    );

ChatRoom _room(String id) => ChatRoom(
      conversationId: Uint8List.fromList(id.codeUnits),
      name: id,
    );

/// Fills every session-scoped provider this test can populate without a worker.
void _populateSession(ProviderContainer container, User owner, ChatRoom room) {
  container.read(userLookupProvider.notifier).state = [owner];
  container.read(chatRoomsProvider.notifier).add(room);
  container.read(currentOpenChatRoom.notifier).state = room;
  container.read(groupInvitesProvider.notifier).add(
        GroupInvite(
          senderId: owner.id,
          receivedAt: DateTime(2026),
          groupDetails: room,
        ),
      );
  container.read(homeScreenControllerProvider.notifier).goToTab(TabType.chat);
}

void _expectSessionCleared(ProviderContainer container) {
  expect(container.read(userLookupProvider), isEmpty);
  expect(container.read(chatRoomsProvider), isEmpty);
  expect(container.read(currentOpenChatRoom), isNull);
  expect(container.read(groupInvitesProvider), isEmpty);
  expect(container.read(homeScreenControllerProvider), TabType.public);
}

void main() {
  group('listenForSessionChanges', () {
    test('discards the previous account\'s state when the account changes', () {
      final container = ProviderContainer();
      addTearDown(container.dispose);

      final accounts = <String?>[];
      addTearDown(
        listenForSessionChanges(container, onSessionChanged: accounts.add).close,
      );

      final previous = _user('Previous', 1);
      container.read(sessionKeyProvider.notifier).state = previous;
      _populateSession(container, previous, _room('previous-room'));

      final next = _user('Next', 2);
      container.read(sessionKeyProvider.notifier).state = next;

      _expectSessionCleared(container);
      // The key is what the reset is keyed on, so it survives its own boundary.
      expect(container.read(sessionKeyProvider), next);
      expect(accounts, [previous.idBase58, next.idBase58]);
    });

    test('clears the session when the account signs out', () {
      final container = ProviderContainer();
      addTearDown(container.dispose);

      final accounts = <String?>[];
      addTearDown(
        listenForSessionChanges(container, onSessionChanged: accounts.add).close,
      );

      final previous = _user('Previous', 1);
      container.read(sessionKeyProvider.notifier).state = previous;
      _populateSession(container, previous, _room('previous-room'));

      container.read(sessionKeyProvider.notifier).state = null;

      _expectSessionCleared(container);
      expect(accounts, [previous.idBase58, null]);
    });

    test('ignores a republish of the same account', () {
      // `getDefaultUserAccount` re-publishes the signed-in user on every splash
      // evaluation. Resetting on that would wipe a live session.
      final container = ProviderContainer();
      addTearDown(container.dispose);

      addTearDown(
        listenForSessionChanges(container, onSessionChanged: (_) {}).close,
      );

      final owner = _user('Owner', 1);
      final room = _room('room');
      container.read(sessionKeyProvider.notifier).state = owner;
      _populateSession(container, owner, room);

      container.read(sessionKeyProvider.notifier).state =
          _user('Owner renamed', 1);

      expect(container.read(chatRoomsProvider), [room]);
      expect(container.read(currentOpenChatRoom), room);
      expect(container.read(homeScreenControllerProvider), TabType.chat);
    });

    test('leaves sign-out-scoped state alone on the sign-in edge', () {
      // accountSessionProvider drives the splash screen's auto-navigate.
      // Refreshing it mid-login would race that against the login flow's own
      // push to /home, so it belongs to the sign-out edge only.
      expect(sessionScopedProviders, isNot(contains(accountSessionProvider)));
      expect(signOutScopedProviders, contains(accountSessionProvider));
      expect(sessionScopedProviders, isNot(contains(sessionKeyProvider)));
    });
  });

  group('resetSessionScopedState', () {
    test('clears notifier state that is not held in `state`', () async {
      // Riverpod reuses the Notifier *instance* when a NotifierProvider is
      // invalidated: it re-runs build() on the same object rather than
      // constructing a new one. Anything kept in a private field therefore
      // survives a session change unless build() resets it too — which is easy
      // to miss, because `state` itself looks correctly empty.
      final container = ProviderContainer(
        overrides: [
          qaulWorkerProvider.overrideWith(
            (ref) => _PaginatingWorker(ref),
          ),
        ],
      );
      addTearDown(container.dispose);

      final rooms = container.read(chatRoomsStoreProvider.notifier);
      await rooms.getChatRooms();
      await rooms.getGroupInvites();
      expect(rooms.chatRoomsPagination, isNotNull);
      expect(rooms.groupInvitesPagination, isNotNull);

      final before = rooms;
      resetSessionScopedState(container);
      container.read(chatRoomsStoreProvider);

      expect(
        container.read(chatRoomsStoreProvider.notifier),
        same(before),
        reason: 'Riverpod is expected to reuse the notifier instance; if this '
            'ever changes, the manual field resets in build() can go away.',
      );
      expect(rooms.chatRoomsPagination, isNull);
      expect(rooms.groupInvitesPagination, isNull);
    });

    test('hands the next session a usable PageController', () {
      final container = ProviderContainer();
      addTearDown(container.dispose);

      final before = container.read(homeScreenControllerProvider.notifier)
          .controller();

      resetSessionScopedState(container);

      final after = container.read(homeScreenControllerProvider.notifier)
          .controller();
      expect(after, isNot(same(before)));
      // Throws if `after` were the previous session's disposed controller.
      expect(after.hasClients, isFalse);
    });

    test('replaces notification controllers rather than reusing them', () {
      final container = ProviderContainer();
      addTearDown(container.dispose);

      final before = container.read(chatNotificationControllerProvider);
      final beforePublic = container.read(publicNotificationControllerProvider);

      resetSessionScopedState(container);

      // `NotificationController.initialize()` assigns `late final` fields, so
      // re-initializing the same instance for a second account throws. The
      // reset must hand back new instances.
      expect(
        container.read(chatNotificationControllerProvider),
        isNot(same(before)),
      );
      expect(
        container.read(publicNotificationControllerProvider),
        isNot(same(beforePublic)),
      );
    });
  });

  group('provider census', () {
    // Every provider must be classified into exactly one of the four buckets in
    // session_scope.dart. This is what stops a newly added provider from
    // silently surviving a session change — which is the class of bug the reset
    // exists to prevent in the first place.
    //
    // Each package classifies the providers it owns, so the census reads both
    // scope files.
    const scopeFiles = [
      'lib/session/session_scope.dart',
      'packages/qaul_rpc/lib/src/session_scope.dart',
    ];

    // `final <name> = <Something>Provider...`, with an optional leading
    // `static`/`late`, an optional explicit type, and the initializer allowed to
    // start on the next line.
    final declaration = RegExp(
      r'^\s*(?:static\s+)?(?:late\s+)?final\s+(?:[\w<>,?\s]+\s)??(\w+)\s*=\s*'
      r'(?:\r?\n\s*)?[A-Za-z]*Provider\b',
      multiLine: true,
    );

    // Anything that assigns something Provider-shaped but that `declaration`
    // could not attribute to a name. The census must fail loudly on shapes it
    // cannot parse rather than skipping them.
    final anyProviderAssignment = RegExp(
      r'^\s*(?:static\s+)?(?:late\s+)?final\b[^;=]*=\s*(?:\r?\n\s*)?'
      r'[A-Za-z]*Provider\b',
      multiLine: true,
    );

    bool isGenerated(File file) =>
        file.path.contains('/generated/') ||
        file.path.endsWith('.g.dart') ||
        file.path.endsWith('.freezed.dart') ||
        file.path.endsWith('.pb.dart');

    Iterable<File> dartFilesIn(String directory) => Directory(directory)
        .listSync(recursive: true)
        .whereType<File>()
        .where((f) => f.path.endsWith('.dart'))
        .where((f) => !isGenerated(f));

    /// Identifiers listed inside a `<ProviderOrFamily>[ ... ]` literal.
    Set<String> listedProviders(String source) {
      final lists = RegExp(
        r'<ProviderOrFamily>\[(.*?)\n\];',
        dotAll: true,
      ).allMatches(source);
      return {
        for (final list in lists)
          ...RegExp(r'^\s*(\w+),', multiLine: true)
              .allMatches(list.group(1)!)
              .map((m) => m.group(1)!),
      };
    }

    final scopeSources = {
      for (final path in scopeFiles) path: File(path).readAsStringSync(),
    };

    /// The provider each scope file names as its session key, plus the aliases
    /// that point at it.
    Set<String> sessionKeys(String source) => RegExp(
      r'^final (\w*[sS]essionKeyProvider) = (\w+);',
      multiLine: true,
    ).allMatches(source).expand((m) => [m.group(1)!, m.group(2)!]).toSet();

    /// The four buckets, by name. Membership — not "appears somewhere in the
    /// file", which a passing mention in a doc comment would satisfy.
    final classified = <String>{
      ...appScopedProviders.keys,
      for (final source in scopeSources.values) ...[
        ...sessionKeys(source),
        ...listedProviders(source),
      ],
    };

    test('every provider is classified in session_scope.dart', () {
      final unclassified = <String, String>{};
      for (final directory in const [
        'lib',
        'packages/qaul_rpc/lib',
        'packages/qaul_components/lib',
      ]) {
        for (final file in dartFilesIn(directory)) {
          if (scopeFiles.any(file.path.endsWith)) continue;
          final source = file.readAsStringSync();

          final named = declaration.allMatches(source).map((m) => m.group(1)!);
          final assignments = anyProviderAssignment.allMatches(source).length;
          expect(
            named.length,
            assignments,
            reason: '${file.path} declares a provider in a shape this census '
                'cannot attribute to a name. Widen `declaration` — a provider '
                'it silently skips is a provider nobody classifies.',
          );

          for (final name in named) {
            // File-private providers cannot be named from session_scope.dart.
            // They are out of scope by construction: they either hold local UI
            // state for one widget subtree, or derive from a session provider
            // via ref.watch and so reset with it.
            if (name.startsWith('_')) continue;
            if (classified.contains(name)) continue;
            unclassified[name] = file.path;
          }
        }
      }

      expect(
        unclassified,
        isEmpty,
        reason:
            'These providers are classified in no bucket:\n'
            '${unclassified.entries.map((e) => '  · ${e.key}  (${e.value})').join('\n')}\n\n'
            'Add each one to `sessionScopedProviders` if it holds state '
            'belonging to a single signed-in account, to `signOutScopedProviders` '
            'if it may only be dropped once a session has ended, or to '
            '`appScopedProviders` with the reason it must survive a session '
            'change.',
      );
    });

    test('the census can see the providers it is meant to guard', () {
      // Guards the extraction itself: if `listedProviders` ever stops matching,
      // every provider would count as classified and the census would pass
      // while checking nothing.
      expect(
        classified,
        containsAll(<String>[
          'defaultUserProvider', // qaul_rpc, session key
          'chatRoomsProvider', // qaul_rpc, session-scoped
          'usersStoreProvider', // app, session-scoped
          'accountSessionProvider', // app, sign-out-scoped
          'qaulWorkerProvider', // qaul_rpc, app-scoped
        ]),
      );
    });

    test('appScopedProviders documents a reason for every exclusion', () {
      for (final entry in appScopedProviders.entries) {
        expect(
          entry.value.trim(),
          isNotEmpty,
          reason: '${entry.key} is excluded from the session reset without a '
              'stated reason.',
        );
      }
    });
  });
}
