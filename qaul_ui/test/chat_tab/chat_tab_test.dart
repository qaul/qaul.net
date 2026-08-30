import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter_chat_types/flutter_chat_types.dart' as types;
import 'package:flutter_chat_ui/flutter_chat_ui.dart' as chat_ui;
import 'package:flutter_test/flutter_test.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:local_notifications/src/local_notifications.dart';
import 'package:logging/logging.dart';
import 'package:qaul_components/qaul_components.dart'
    show ChatHeader, ChatMessageContextMenu, QaulComponentsLocalizations;
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:qaul_rpc/src/generated/services/chat/chat.pb.dart';
import 'package:qaul_ui/l10n/app_localizations.dart';
import 'package:qaul_ui/providers/providers.dart';
import 'package:qaul_ui/screens/home/tabs/chat/widgets/chat.dart';
import 'package:qaul_ui/screens/home/tabs/tab.dart';
import 'package:qaul_ui/screens/home/user_details_screen.dart';
import 'package:qaul_ui/stores/stores.dart';
import 'package:qaul_ui/widgets/widgets.dart';
import 'package:shared_preferences/shared_preferences.dart';

import '../test_utils/test_utils.dart';

part 'fixtures.dart';

part 'stubs.dart';

class TestChatRoomListNotifier extends ChatRoomListNotifier {
  static List<ChatRoom> rooms = [buildGroupChat()];

  @override
  List<ChatRoom> build() => rooms;
}

class TestUsersStore extends UsersStore {
  static List<User> users = [otherUser];

  @override
  List<User> build() => users;
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
    TestChatRoomListNotifier.rooms = [buildGroupChat()];
    TestUsersStore.users = [otherUser];
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
        usersStoreProvider.overrideWith(() => TestUsersStore()),
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
    expect(find.text(otherUser.name), findsOneWidget);
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
    expect(find.byIcon(Icons.more_vert), findsOneWidget);
    expect(find.byTooltip('Back'), findsNothing);
  });

  testWidgets('group header menu opens group settings', (tester) async {
    await pumpChatScreen(tester, buildGroupChat());

    await tester.tap(find.byIcon(Icons.more_vert));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Group Settings'));
    await tester.pumpAndSettle();

    expect(find.text('Members'), findsOneWidget);
  });

  types.TextMessage forwardTargetMessage() => types.TextMessage(
    id: 'forward-target',
    author: types.User(id: otherUser.idBase58, firstName: otherUser.name),
    text: 'forward this text',
  );

  Message textMessage({
    required String id,
    required User sender,
    required String text,
  }) {
    return Message(
      senderId: sender.id,
      messageId: Uint8List.fromList(id.codeUnits),
      content: TextMessageContent(text),
      index: 1,
      sentAt: DateTime(2000),
      receivedAt: DateTime(2000),
    );
  }

  BoxBorder? bubbleBorder(WidgetTester tester) {
    final decoration = tester
        .widget<DecoratedBox>(find.byKey(const ValueKey('chat-bubble-surface')))
        .decoration as BoxDecoration;
    return decoration.border;
  }

  User testUser(
    String name, {
    String? id,
    ConnectionStatus status = ConnectionStatus.offline,
    Uint8List? conversationId,
  }) {
    return User(
      name: name,
      id: Uint8List.fromList((id ?? name).codeUnits),
      status: status,
      conversationId: conversationId,
    );
  }

  ChatRoom directRoomWith(User user) {
    return ChatRoom(
      name: user.name,
      conversationId: user.conversationId ??
          Uint8List.fromList('${user.name}-conversation'.codeUnits),
      members: [
        ChatRoomUser(defaultUser, joinedAt: DateTime(2000)),
        ChatRoomUser(user, joinedAt: DateTime(2000)),
      ],
    );
  }

  testWidgets('text message long press opens context menu with forward enabled', (
    tester,
  ) async {
    await pumpChatScreen(tester, buildDirectChat(), otherUser: otherUser);

    final chat = tester.widget<chat_ui.Chat>(find.byType(chat_ui.Chat));
    chat.onMessageLongPress!(
      tester.element(find.byType(chat_ui.Chat)),
      forwardTargetMessage(),
    );
    await tester.pumpAndSettle();

    expect(find.byType(ChatMessageContextMenu), findsOneWidget);
    expect(find.text('Forward'), findsOneWidget);
    expect(find.text('Reply'), findsOneWidget);
    expect(find.text('Edit'), findsOneWidget);
    await tester.tap(find.text('Reply'));
    await tester.pumpAndSettle();
    expect(find.byType(ChatMessageContextMenu), findsOneWidget);
    expect(find.text('Forward to'), findsNothing);

    await tester.tap(find.text('Edit'));
    await tester.pumpAndSettle();
    expect(find.byType(ChatMessageContextMenu), findsOneWidget);
    expect(find.text('Forward to'), findsNothing);
  });

  testWidgets('text message long press highlights the selected bubble', (
    tester,
  ) async {
    final message = textMessage(
      id: 'selected-message',
      sender: otherUser,
      text: 'select me',
    );
    await pumpChatScreen(
      tester,
      buildDirectChat(messages: [message]),
      otherUser: otherUser,
    );

    expect(bubbleBorder(tester), isNull);

    final chat = tester.widget<chat_ui.Chat>(find.byType(chat_ui.Chat));
    chat.onMessageLongPress!(
      tester.element(find.byType(chat_ui.Chat)),
      chat.messages.first,
    );
    await tester.pumpAndSettle();

    expect(find.byType(ChatMessageContextMenu), findsOneWidget);
    expect(bubbleBorder(tester), isNotNull);
  });

  testWidgets('forward action opens recipient selector with users and groups', (
    tester,
  ) async {
    await pumpChatScreen(tester, buildDirectChat(), otherUser: otherUser);

    final chat = tester.widget<chat_ui.Chat>(find.byType(chat_ui.Chat));
    chat.onMessageLongPress!(
      tester.element(find.byType(chat_ui.Chat)),
      forwardTargetMessage(),
    );
    await tester.pumpAndSettle();
    await tester.tap(find.text('Forward'));
    await tester.pumpAndSettle();

    expect(find.text('Forward to'), findsOneWidget);
    expect(find.byKey(const ValueKey('forward-recipient-search')), findsOneWidget);
    expect(find.text('Groups'), findsOneWidget);
    expect(find.text('Group Chat'), findsOneWidget);
    expect(find.text('Users / Contacts'), findsOneWidget);
    expect(find.text(otherUser.name), findsOneWidget);
    expect(
      tester.getTopLeft(find.text('Groups')).dy,
      lessThan(tester.getTopLeft(find.text('Users / Contacts')).dy),
    );
  });

  testWidgets('forward selector prioritizes messaged and online users', (
    tester,
  ) async {
    final messagedUser = testUser('Messaged User');
    final onlineUser = testUser(
      'Online User',
      status: ConnectionStatus.online,
      conversationId: Uint8List.fromList('onlineConversation'.codeUnits),
    );
    final offlineUser = testUser(
      'Offline User',
      conversationId: Uint8List.fromList('offlineConversation'.codeUnits),
    );
    TestUsersStore.users = [offlineUser, onlineUser, messagedUser];
    TestChatRoomListNotifier.rooms = [
      buildGroupChat(),
      directRoomWith(messagedUser),
    ];

    await pumpChatScreen(tester, buildDirectChat(), otherUser: otherUser);

    final chat = tester.widget<chat_ui.Chat>(find.byType(chat_ui.Chat));
    chat.onMessageLongPress!(
      tester.element(find.byType(chat_ui.Chat)),
      forwardTargetMessage(),
    );
    await tester.pumpAndSettle();
    await tester.tap(find.text('Forward'));
    await tester.pumpAndSettle();

    expect(
      tester.getTopLeft(find.text('Messaged User')).dy,
      lessThan(tester.getTopLeft(find.text('Online User')).dy),
    );
    expect(
      tester.getTopLeft(find.text('Online User')).dy,
      lessThan(tester.getTopLeft(find.text('Offline User')).dy),
    );
  });

  testWidgets('recipient search filters users and groups', (tester) async {
    await pumpChatScreen(tester, buildDirectChat(), otherUser: otherUser);

    final chat = tester.widget<chat_ui.Chat>(find.byType(chat_ui.Chat));
    chat.onMessageLongPress!(
      tester.element(find.byType(chat_ui.Chat)),
      forwardTargetMessage(),
    );
    await tester.pumpAndSettle();
    await tester.tap(find.text('Forward'));
    await tester.pumpAndSettle();

    await tester.enterText(
      find.byKey(const ValueKey('forward-recipient-search')),
      'group',
    );
    await tester.pumpAndSettle();

    expect(find.text('Group Chat'), findsOneWidget);
    expect(find.text(otherUser.name), findsNothing);
  });

  testWidgets('selecting a recipient opens chat with forwarded text draft', (
    tester,
  ) async {
    await pumpChatScreen(tester, buildDirectChat(), otherUser: otherUser);

    final chat = tester.widget<chat_ui.Chat>(find.byType(chat_ui.Chat));
    chat.onMessageLongPress!(
      tester.element(find.byType(chat_ui.Chat)),
      forwardTargetMessage(),
    );
    await tester.pumpAndSettle();
    await tester.tap(find.text('Forward'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Group Chat'));
    await tester.pumpAndSettle();

    expect(find.text('Forward to'), findsNothing);
    expect(find.text('Group Chat'), findsOneWidget);
    expect(tester.widget<TextField>(find.byType(TextField)).controller!.text,
        'forward this text');

    ProviderScope.containerOf(tester.element(find.byType(chat_ui.Chat)))
        .read(currentOpenChatRoom.notifier)
        .state = buildGroupChat(messages: []);
    await tester.pumpAndSettle();

    expect(tester.widget<TextField>(find.byType(TextField)).controller!.text,
        'forward this text');
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
