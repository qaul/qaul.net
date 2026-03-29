import 'dart:io';
import 'dart:typed_data';

import 'package:flutter_test/flutter_test.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:path_provider_platform_interface/path_provider_platform_interface.dart';
import 'package:plugin_platform_interface/plugin_platform_interface.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:qaul_ui/helpers/user_prefs_helper.dart';
import 'package:qaul_ui/providers/providers.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:shared_preferences_platform_interface/in_memory_shared_preferences_async.dart';
import 'package:shared_preferences_platform_interface/shared_preferences_async_platform_interface.dart';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();

  final roomId = Uint8List.fromList('room1'.codeUnits);
  final otherUserId = Uint8List.fromList('otherUser'.codeUnits);
  final localUserId = Uint8List.fromList('localUser'.codeUnits);

  ChatRoom makeRoom({required int unreadCount}) => ChatRoom(
        conversationId: roomId,
        name: 'Test Room',
        unreadCount: unreadCount,
        lastMessagePreview: const TextMessageContent('hello'),
        lastMessageSenderId: otherUserId,
      );

  late ProviderContainer container;
  late ChatNotificationController controller;

  setUpAll(() async {
    final tempDir = await Directory.systemTemp.createTemp('chat_ntfy_test_');
    PathProviderPlatform.instance = _FakePathProvider(tempDir);
    SharedPreferencesAsyncPlatform.instance =
        InMemorySharedPreferencesAsync.empty();
    await UserPrefsHelper.initialize(prefs: SharedPreferencesAsync());
  });

  setUp(() async {
    SharedPreferences.setMockInitialValues({});

    container = ProviderContainer(overrides: [
      defaultUserProvider.overrideWith(
        (_) => User(name: 'Local', id: localUserId),
      ),
      qaulWorkerProvider.overrideWithValue(_StubWorker()),
    ]);

    controller = container.read(chatNotificationControllerProvider);
    await controller.initialize();
  });

  tearDown(() => container.dispose());

  group('ChatNotificationController cache vs unreadCount', () {
    test(
        'after reading messages (unreadCount drops to 0), '
        'a new message (unreadCount back to 1) is detected', () {
      // 1. New room arrives with unreadCount: 1 — should be detected.
      var entries = controller.entriesToBeProcessed([makeRoom(unreadCount: 1)]);
      expect(entries, isNotEmpty, reason: 'new room should be detected');

      // process() updates the internal cache to unreadCount: 1.
      for (final e in entries) {
        controller.process(e);
      }

      // 2. User reads messages — backend reports unreadCount: 0.
      entries = controller.entriesToBeProcessed([makeRoom(unreadCount: 0)]);
      expect(entries, isEmpty, reason: 'no new messages after reading');

      // 3. New message arrives — unreadCount goes back to 1.
      entries = controller.entriesToBeProcessed([makeRoom(unreadCount: 1)]);
      expect(
        entries,
        isNotEmpty,
        reason:
            'unreadCount increased from 0→1, should be detected as new message',
      );
    });

    test('unreadCount going from 2→0→1 is detected', () {
      var entries = controller.entriesToBeProcessed([makeRoom(unreadCount: 2)]);
      for (final e in entries) {
        controller.process(e);
      }

      // User reads all.
      entries = controller.entriesToBeProcessed([makeRoom(unreadCount: 0)]);
      expect(entries, isEmpty);

      // One new message.
      entries = controller.entriesToBeProcessed([makeRoom(unreadCount: 1)]);
      expect(entries, isNotEmpty,
          reason: 'cache updated to 0, so 1 > 0 is a new message');
    });

    test('monotonically increasing unreadCount still works (regression guard)',
        () {
      var entries = controller.entriesToBeProcessed([makeRoom(unreadCount: 1)]);
      expect(entries, isNotEmpty);
      for (final e in entries) {
        controller.process(e);
      }

      entries = controller.entriesToBeProcessed([makeRoom(unreadCount: 2)]);
      expect(entries, isNotEmpty);
      for (final e in entries) {
        controller.process(e);
      }

      entries = controller.entriesToBeProcessed([makeRoom(unreadCount: 3)]);
      expect(entries, isNotEmpty);
    });

    test('same unreadCount on consecutive calls does not re-trigger', () {
      var entries = controller.entriesToBeProcessed([makeRoom(unreadCount: 1)]);
      for (final e in entries) {
        controller.process(e);
      }

      entries = controller.entriesToBeProcessed([makeRoom(unreadCount: 1)]);
      expect(entries, isEmpty,
          reason: 'same unreadCount should not re-trigger');
    });

    test(
        'unreadCount drop is persisted so a restart '
        'does not revert to stale high watermark', () async {
      // 1. First message arrives — detected and processed.
      final room = makeRoom(unreadCount: 1);
      var entries = controller.entriesToBeProcessed([room]);
      for (final e in entries) {
        controller.process(e);
      }
      // Persist (simulates what close() does at the end of execute()).
      controller.updatePersistentCachedData();

      // 2. User reads messages — unreadCount drops to 0.
      //    execute() would call entriesToBeProcessed, get an empty queue,
      //    and return early — so close()/updatePersistentCachedData() is
      //    NOT called by execute(). The fix must persist the drop itself.
      controller.entriesToBeProcessed([makeRoom(unreadCount: 0)]);

      // 3. Simulate app restart: new container, fresh controller loading
      //    from the same SharedPreferences.
      container.dispose();
      container = ProviderContainer(overrides: [
        defaultUserProvider.overrideWith(
          (_) => User(name: 'Local', id: localUserId),
        ),
        qaulWorkerProvider.overrideWithValue(_StubWorker()),
      ]);
      final freshController =
          container.read(chatNotificationControllerProvider);
      await freshController.initialize();

      // 4. New message arrives (unreadCount: 1). The fresh controller must
      //    detect it, not silently swallow it due to a stale cached count.
      entries = freshController.entriesToBeProcessed([makeRoom(unreadCount: 1)]);
      expect(
        entries,
        isNotEmpty,
        reason:
            'after restart, cache should reflect unreadCount 0 '
            'so the 0→1 transition is detected',
      );
    });
  });
}

/// Minimal stub that satisfies the [qaulWorkerProvider] dependency.
/// Only [getAllChatRooms] is called during [initialize].
class _StubWorker implements LibqaulWorker {
  @override
  Future<List<ChatRoom>> getAllChatRooms() async => [];

  @override
  dynamic noSuchMethod(Invocation invocation) =>
      throw UnimplementedError('${invocation.memberName} not stubbed');
}

class _FakePathProvider extends Fake
    with MockPlatformInterfaceMixin
    implements PathProviderPlatform {
  _FakePathProvider(this._tempDir);

  final Directory _tempDir;

  @override
  Future<String?> getApplicationDocumentsPath() async => _tempDir.path;
}
