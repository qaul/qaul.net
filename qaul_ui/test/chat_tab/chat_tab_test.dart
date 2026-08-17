import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:local_notifications/src/local_notifications.dart';
import 'package:logging/logging.dart';
import 'package:qaul_components/qaul_components.dart'
    show ChatFooter, ChatHeader, QaulComponentsLocalizations;
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:qaul_rpc/src/generated/services/chat/chat.pb.dart';
import 'package:qaul_ui/l10n/app_localizations.dart';
import 'package:qaul_ui/providers/providers.dart';
import 'package:qaul_ui/screens/home/tabs/chat/widgets/chat.dart';
import 'package:qaul_ui/screens/home/tabs/tab.dart';
import 'package:qaul_ui/screens/home/user_details_screen.dart';
import 'package:qaul_ui/widgets/widgets.dart';
import 'package:shared_preferences/shared_preferences.dart';

import '../test_utils/test_utils.dart';

part 'fixtures.dart';

part 'stubs.dart';

class TestChatRoomListNotifier extends ChatRoomListNotifier {
  @override
  List<ChatRoom> build() => [buildGroupChat()];
}

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
    StubLibqaulWorker.sentTexts.clear();
    SharedPreferences.setMockInitialValues({});
  });

  Future<void> pumpChatScreen(
    WidgetTester tester,
    ChatRoom room, {
    User? otherUser,
  }) async {
    final wut = ProviderScope(
      overrides: [
        defaultUserProvider.overrideWith((_) => defaultUser),
        chatNotificationControllerProvider.overrideWithValue(
          NullChatNotificationController(),
        ),
        chatRoomsProvider.overrideWith(TestChatRoomListNotifier.new),
        qaulWorkerProvider.overrideWith((ref) => StubLibqaulWorker(ref)),
      ],
      child: materialAppWithLocalizations(
        ChatScreen(room, defaultUser, otherUser: otherUser),
      ),
    );

    await tester.pumpWidget(wut);
    await tester.pump();
  }

  testWidgets('direct chat renders ChatHeader and opens peer details', (
    tester,
  ) async {
    await pumpChatScreen(tester, buildDirectChat(), otherUser: otherUser);

    expect(find.byType(ChatHeader), findsOneWidget);
    expect(find.byType(ChatFooter), findsOneWidget);
    expect(find.text(otherUser.name), findsOneWidget);
    expect(find.text('Secure private message'), findsOneWidget);
    expect(
      find.byTooltip('Record audio message'),
      Platform.isLinux ? findsNothing : findsOneWidget,
    );
    expect(find.byTooltip('Send File'), findsOneWidget);
    expect(find.byTooltip('Back'), findsNothing);

    final avatarTapTarget = find.descendant(
      of: find.byType(ChatHeader),
      matching: find.byType(InkResponse),
    );
    expect(avatarTapTarget, findsOneWidget);

    await tester.tap(avatarTapTarget);
    await tester.pumpAndSettle();

    expect(find.byType(UserDetailsScreen), findsOneWidget);
  });

  testWidgets('group chat renders ChatHeader with menu', (tester) async {
    await pumpChatScreen(tester, buildGroupChat());

    expect(find.byType(ChatHeader), findsOneWidget);
    expect(find.text('Group Chat'), findsOneWidget);
    expect(find.text('2 members'), findsOneWidget);
    expect(find.text('Group chat message'), findsOneWidget);
    expect(find.byIcon(Icons.more_vert), findsOneWidget);
    expect(find.byTooltip('Back'), findsNothing);
  });

  testWidgets('chat footer sends typed text and clears draft', (tester) async {
    await pumpChatScreen(tester, buildGroupChat());

    await tester.enterText(find.byType(TextField), 'hello footer');
    await tester.pump();
    await tester.tap(find.byTooltip('Send'));
    await tester.pumpAndSettle();

    expect(StubLibqaulWorker.sentTexts, ['hello footer']);
    expect(tester.widget<TextField>(find.byType(TextField)).controller!.text, '');
  });

  testWidgets('chat footer prevents empty sends', (tester) async {
    await pumpChatScreen(tester, buildGroupChat());

    expect(find.byTooltip('Send'), findsNothing);

    await tester.enterText(find.byType(TextField), '   ');
    await tester.pump();

    expect(find.byTooltip('Send'), findsNothing);
    expect(StubLibqaulWorker.sentTexts, isEmpty);
  });

  testWidgets('disabled room blocks chat footer sending', (tester) async {
    await pumpChatScreen(
      tester,
      buildGroupChat(status: ChatRoomStatus.inviteAccepted),
    );

    expect(
      find.text(
        'Please wait for the admin to confirm your acceptance to send messages',
      ),
      findsOneWidget,
    );

    await tester.enterText(find.byType(TextField), 'blocked');
    await tester.pump();
    await tester.tap(find.byTooltip('Send'), warnIfMissed: false);
    await tester.pumpAndSettle();

    expect(StubLibqaulWorker.sentTexts, isEmpty);
  });

  testWidgets('group header menu opens group settings', (tester) async {
    await pumpChatScreen(tester, buildGroupChat());

    await tester.tap(find.byIcon(Icons.more_vert));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Group Settings'));
    await tester.pumpAndSettle();

    expect(find.text('Members'), findsOneWidget);
  });

  testWidgets('chat header close pops the mobile chat route', (tester) async {
    tester.view.physicalSize = const Size(414, 736);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);

    final room = buildDirectChat();
    final wut = ProviderScope(
      overrides: [
        defaultUserProvider.overrideWith((_) => defaultUser),
        chatNotificationControllerProvider.overrideWithValue(
          NullChatNotificationController(),
        ),
        qaulWorkerProvider.overrideWith((ref) => StubLibqaulWorker(ref)),
      ],
      child: MaterialApp(
        localizationsDelegates: [
          ...AppLocalizations.localizationsDelegates,
          QaulComponentsLocalizations.delegate,
        ],
        supportedLocales: AppLocalizations.supportedLocales,
        home: Builder(
          builder: (context) => TextButton(
            onPressed: () {
              Navigator.push(
                context,
                MaterialPageRoute(
                  builder: (_) =>
                      ChatScreen(room, defaultUser, otherUser: otherUser),
                  settings: const RouteSettings(name: '/chat'),
                ),
              );
            },
            child: const Text('Open chat'),
          ),
        ),
      ),
    );

    await tester.pumpWidget(wut);
    await tester.tap(find.text('Open chat'));
    await tester.pump();
    await tester.pump();
    expect(find.byType(ChatHeader), findsOneWidget);

    await tester.tap(find.byTooltip('Back'));
    await tester.pumpAndSettle();

    expect(find.byType(ChatHeader), findsNothing);
    expect(find.text('Open chat'), findsOneWidget);
  });

  testResponsiveWidgets(
    'empty state chat tab',
    (tester) async {
      final wut = ProviderScope(
        overrides: [
          defaultUserProvider.overrideWith((_) => defaultUser),
          chatNotificationControllerProvider.overrideWithValue(
            NullChatNotificationController(),
          ),
        ],
        child: materialAppWithLocalizations(BaseTab.chat(key: chatKey)),
      );

      await tester.pumpWidget(wut);
      expect(find.byKey(chatKey), findsOneWidget);
    },
    goldenCallback: (sizeName, tester) async {
      await expectGoldenMatches(
        find.byKey(chatKey),
        '$sizeName.png',
        subPath: 'emptyState',
      );
    },
    skip: shouldSkip,
  );

  testResponsiveWidgets(
    'chat tab with group chat',
    (tester) async {
      final wut = ProviderScope(
        overrides: [
          defaultUserProvider.overrideWith((_) => defaultUser),
          chatNotificationControllerProvider.overrideWithValue(
            NullChatNotificationController(),
          ),
          chatRoomsProvider.overrideWith(TestChatRoomListNotifier.new),
          qaulWorkerProvider.overrideWith((ref) => StubLibqaulWorker(ref)),
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
    },
    goldenCallback: (sizeName, tester) async {
      await expectGoldenMatches(
        find.byKey(chatKey),
        '$sizeName.png',
        subPath: 'tabWithGroupTile',
      );
    },
    skip: shouldSkip,
  );

  testResponsiveWidgets(
    'opening a group chat',
    (tester) async {
      final wut = ProviderScope(
        overrides: [
          defaultUserProvider.overrideWith((_) => defaultUser),
          chatNotificationControllerProvider.overrideWithValue(
            NullChatNotificationController(),
          ),
          chatRoomsProvider.overrideWith(TestChatRoomListNotifier.new),
          qaulWorkerProvider.overrideWith((ref) => StubLibqaulWorker(ref)),
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
    },
    goldenCallback: (sizeName, tester) async {
      await expectGoldenMatches(
        find.byType(ChatScreen),
        '$sizeName.png',
        subPath: 'openEmptyChat',
      );
    },
    skip: shouldSkip,
  );

  testResponsiveWidgets(
    'sending a message to an open group chat',
    (tester) async {
      final wut = ProviderScope(
        overrides: [
          defaultUserProvider.overrideWith((_) => defaultUser),
          chatNotificationControllerProvider.overrideWithValue(
            NullChatNotificationController(),
          ),
          chatRoomsProvider.overrideWith(TestChatRoomListNotifier.new),
          qaulWorkerProvider.overrideWith((ref) => StubLibqaulWorker(ref)),
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
    },
    goldenCallback: (sizeName, tester) async {
      await expectGoldenMatches(
        find.byType(ChatScreen),
        '$sizeName.png',
        subPath: 'singleMessage',
      );
    },
    skip: shouldSkip,
  );

  testResponsiveWidgets(
    'sending multiple messages to an open group chat',
    (tester) async {
      final wut = ProviderScope(
        overrides: [
          defaultUserProvider.overrideWith((_) => defaultUser),
          chatNotificationControllerProvider.overrideWithValue(
            NullChatNotificationController(),
          ),
          chatRoomsProvider.overrideWith(TestChatRoomListNotifier.new),
          qaulWorkerProvider.overrideWith((ref) => StubLibqaulWorker(ref)),
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
    },
    goldenCallback: (sizeName, tester) async {
      await expectGoldenMatches(
        find.byType(ChatScreen),
        '$sizeName.png',
        subPath: 'multipleMessages',
      );
    },
    skip: shouldSkip,
  );

  testResponsiveWidgets('sending 10 messages and then close the group chat', (
    tester,
  ) async {
    final wut = ProviderScope(
      overrides: [
        defaultUserProvider.overrideWith((_) => defaultUser),
        chatNotificationControllerProvider.overrideWithValue(
          NullChatNotificationController(),
        ),
        chatRoomsProvider.overrideWith(TestChatRoomListNotifier.new),
        qaulWorkerProvider.overrideWith((ref) => StubLibqaulWorker(ref)),
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

    await tester.tap(find.byTooltip('Back'));
    await tester.pumpAndSettle();
    expect(find.byType(ChatScreen), findsNothing, reason: 'chat was closed');
  }, skip: shouldSkip);
}
