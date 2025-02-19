import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:local_notifications/src/local_notifications.dart';
import 'package:logging/logging.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:qaul_ui/providers/providers.dart';
import 'package:qaul_ui/screens/home/tabs/chat/widgets/chat.dart';
import 'package:qaul_ui/screens/home/tabs/tab.dart';
import 'package:qaul_ui/widgets/widgets.dart';
import 'package:shared_preferences/shared_preferences.dart';

import '../test_utils/test_utils.dart';

part 'fixtures.dart';

part 'stubs.dart';

void main() {
  late Key chatKey;

  const shouldSkip = true;

  Logger.root.onRecord.listen((LogRecord r) {
    final msg = '${r.level.name}: [${r.loggerName}]@${r.time}: ${r.message}';
    r.level >= Level.SEVERE
        ? stderr.writeln('$msg\n${r.error}\n${r.stackTrace}')
        : stdout.writeln(msg);
  });

  setUp(() {
    chatKey = UniqueKey();
    SharedPreferences.setMockInitialValues({});
  });

  testResponsiveWidgets('empty state chat tab', (tester) async {
    final wut = ProviderScope(
      overrides: [
        defaultUserProvider.overrideWith(
          (_) => defaultUser,
        ),
        chatNotificationControllerProvider.overrideWithValue(
          NullChatNotificationController(),
        )
      ],
      child: materialAppWithLocalizations(BaseTab.chat(key: chatKey)),
    );

    await tester.pumpWidget(wut);
    expect(find.byKey(chatKey), findsOneWidget);
  }, goldenCallback: (sizeName, tester) async {
    await expectGoldenMatches(
      find.byKey(chatKey),
      '$sizeName.png',
      subPath: 'emptyState',
    );
  }, skip: shouldSkip);

  testResponsiveWidgets('chat tab with group chat', (tester) async {
    final wut = ProviderScope(
      overrides: [
        defaultUserProvider.overrideWith(
          (_) => defaultUser,
        ),
        chatNotificationControllerProvider.overrideWithValue(
          NullChatNotificationController(),
        ),
        chatRoomsProvider.overrideWith(
          (_) => ChatRoomListNotifier(rooms: [buildGroupChat()]),
        ),
        qaulWorkerProvider.overrideWith(
          (ref) => StubLibqaulWorker(ref),
        ),
      ],
      child: materialAppWithLocalizations(BaseTab.chat(key: chatKey)),
    );

    await tester.pumpWidget(wut);

    var chatRoomTileFinder = find.byType(QaulListTile);
    expect(
      chatRoomTileFinder,
      findsOneWidget,
      reason: 'one chat room available',
    );
  }, goldenCallback: (sizeName, tester) async {
    await expectGoldenMatches(
      find.byKey(chatKey),
      '$sizeName.png',
      subPath: 'tabWithGroupTile',
    );
  }, skip: shouldSkip);

  testResponsiveWidgets('opening a group chat', (tester) async {
    final wut = ProviderScope(
      overrides: [
        defaultUserProvider.overrideWith(
          (_) => defaultUser,
        ),
        chatNotificationControllerProvider.overrideWithValue(
          NullChatNotificationController(),
        ),
        chatRoomsProvider.overrideWith(
          (_) => ChatRoomListNotifier(rooms: [buildGroupChat()]),
        ),
        qaulWorkerProvider.overrideWith(
          (ref) => StubLibqaulWorker(ref),
        ),
      ],
      child: materialAppWithLocalizations(BaseTab.chat(key: chatKey)),
    );

    await tester.pumpWidget(wut);

    var chatRoomTileFinder = find.byType(QaulListTile);
    expect(
      chatRoomTileFinder,
      findsOneWidget,
      reason: 'one chat room available',
    );

    expect(find.byType(ChatScreen), findsNothing, reason: 'no open chats');
    await tester.tap(chatRoomTileFinder);
    await tester.pumpAndSettle();
    expect(find.byType(ChatScreen), findsOneWidget, reason: 'one open chat');
  }, goldenCallback: (sizeName, tester) async {
    await expectGoldenMatches(
      find.byType(ChatScreen),
      '$sizeName.png',
      subPath: 'openEmptyChat',
    );
  }, skip: shouldSkip);

  testResponsiveWidgets('sending a message to an open group chat',
      (tester) async {
    final wut = ProviderScope(
      overrides: [
        defaultUserProvider.overrideWith(
          (_) => defaultUser,
        ),
        chatNotificationControllerProvider.overrideWithValue(
          NullChatNotificationController(),
        ),
        chatRoomsProvider.overrideWith(
          (_) => ChatRoomListNotifier(rooms: [buildGroupChat()]),
        ),
        qaulWorkerProvider.overrideWith(
          (ref) => StubLibqaulWorker(ref),
        ),
      ],
      child: materialAppWithLocalizations(BaseTab.chat(key: chatKey)),
    );

    await tester.pumpWidget(wut);

    var chatRoomTileFinder = find.byType(QaulListTile);
    expect(
      chatRoomTileFinder,
      findsOneWidget,
      reason: 'one chat room available',
    );

    expect(find.byType(ChatScreen), findsNothing, reason: 'no open chats');
    await tester.tap(chatRoomTileFinder);
    await tester.pumpAndSettle();
    expect(find.byType(ChatScreen), findsOneWidget, reason: 'one open chat');

    final sendMessageButtonFinder = find.byType(SendMessageButton);

    await tester.enterText(find.byType(TextField), 'text');
    await tester.pump();

    expect(sendMessageButtonFinder, findsOneWidget);
    await tester.tap(sendMessageButtonFinder);
    await tester.pumpAndSettle();
  }, goldenCallback: (sizeName, tester) async {
    await expectGoldenMatches(
      find.byType(ChatScreen),
      '$sizeName.png',
      subPath: 'singleMessage',
    );
  }, skip: shouldSkip);

  testResponsiveWidgets('sending multiple messages to an open group chat',
      (tester) async {
    final wut = ProviderScope(
      overrides: [
        defaultUserProvider.overrideWith(
          (_) => defaultUser,
        ),
        chatNotificationControllerProvider.overrideWithValue(
          NullChatNotificationController(),
        ),
        chatRoomsProvider.overrideWith(
          (_) => ChatRoomListNotifier(rooms: [buildGroupChat()]),
        ),
        qaulWorkerProvider.overrideWith(
          (ref) => StubLibqaulWorker(ref),
        ),
      ],
      child: materialAppWithLocalizations(BaseTab.chat(key: chatKey)),
    );

    await tester.pumpWidget(wut);

    var chatRoomTileFinder = find.byType(QaulListTile);
    expect(
      chatRoomTileFinder,
      findsOneWidget,
      reason: 'one chat room available',
    );

    expect(find.byType(ChatScreen), findsNothing, reason: 'no open chats');
    await tester.tap(chatRoomTileFinder);
    await tester.pumpAndSettle();
    expect(find.byType(ChatScreen), findsOneWidget, reason: 'one open chat');

    final sendMessageButtonFinder = find.byType(SendMessageButton);

    for (var i = 0; i < 10; i++) {
      await tester.enterText(find.byType(TextField), 'text$i');
      await tester.pump();
      await tester.tap(sendMessageButtonFinder);
      await tester.pumpAndSettle();
    }
  }, goldenCallback: (sizeName, tester) async {
    await expectGoldenMatches(
      find.byType(ChatScreen),
      '$sizeName.png',
      subPath: 'multipleMessages',
    );
  }, skip: shouldSkip);

  testResponsiveWidgets(
    'sending 10 messages and then close the group chat',
    (tester) async {
      final wut = ProviderScope(
        overrides: [
          defaultUserProvider.overrideWith(
            (_) => defaultUser,
          ),
          chatNotificationControllerProvider.overrideWithValue(
            NullChatNotificationController(),
          ),
          chatRoomsProvider.overrideWith(
            (_) => ChatRoomListNotifier(rooms: [buildGroupChat()]),
          ),
          qaulWorkerProvider.overrideWith(
            (ref) => StubLibqaulWorker(ref),
          ),
        ],
        child: materialAppWithLocalizations(BaseTab.chat(key: chatKey)),
      );

      await tester.pumpWidget(wut);

      var chatRoomTileFinder = find.byType(QaulListTile);
      expect(
        chatRoomTileFinder,
        findsOneWidget,
        reason: 'one chat room available',
      );

      expect(find.byType(ChatScreen), findsNothing, reason: 'no open chats');
      await tester.tap(chatRoomTileFinder);
      await tester.pumpAndSettle();
      expect(find.byType(ChatScreen), findsOneWidget, reason: 'one open chat');

      final sendMessageButtonFinder = find.byType(SendMessageButton);

      for (var i = 0; i < 10; i++) {
        await tester.enterText(find.byType(TextField), 'text$i');
        await tester.pump();
        await tester.tap(sendMessageButtonFinder);
        await tester.pumpAndSettle();
      }

      await tester.tap(find.byType(IconButtonFactory));
      await tester.pumpAndSettle();
      expect(find.byType(ChatScreen), findsNothing, reason: 'chat was closed');
    },
    skip: shouldSkip,
  );
}
