import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_chat_types/flutter_chat_types.dart' as types;
import 'package:flutter_chat_ui/flutter_chat_ui.dart' as chat_ui;
import 'package:flutter_test/flutter_test.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:local_notifications/src/local_notifications.dart';
import 'package:logging/logging.dart';
import 'package:qaul_components/qaul_components.dart'
    show
        ChatFooter,
        ChatHeader,
        ChatMessageContextMenu,
        QaulComponentsLocalizations;
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

class TestUnreadChatRoomListNotifier extends ChatRoomListNotifier {
  @override
  List<ChatRoom> build() => [buildGroupChat().copyWith(unreadCount: 3)];
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
    expect(find.byType(ChatFooter), findsOneWidget);
    expect(find.text(otherUser.name), findsOneWidget);
    expect(find.text('Secure private message'), findsOneWidget);
    expect(
      find.byTooltip('Record audio message'),
      Platform.isLinux ? findsNothing : findsOneWidget,
    );
    expect(find.byTooltip('Send File'), findsWidgets);
    expect(find.byTooltip('Back'), findsNothing);

    await tester.tap(find.byKey(const ValueKey('chat-footer-more')));
    await tester.pumpAndSettle();

    expect(find.byKey(const ValueKey('attachment')), findsOneWidget);
    expect(find.byKey(const ValueKey('emoji')), findsNothing);
    expect(find.byKey(const ValueKey('location')), findsNothing);

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
    expect(
      tester.widget<TextField>(find.byType(TextField)).controller!.text,
      '',
    );
  });

  testWidgets('chat footer sends on Enter and breaks line on Shift+Enter', (
    tester,
  ) async {
    await pumpChatScreen(tester, buildGroupChat());

    final field = find.byType(TextField);
    await tester.tap(field);
    await tester.enterText(field, 'first line');
    await tester.pump();

    await tester.sendKeyDownEvent(LogicalKeyboardKey.shift);
    await tester.sendKeyEvent(LogicalKeyboardKey.enter);
    await tester.sendKeyUpEvent(LogicalKeyboardKey.shift);
    await tester.pump();

    expect(StubLibqaulWorker.sentTexts, isEmpty);
    expect(tester.widget<TextField>(field).controller!.text, 'first line\n');

    await tester.enterText(field, 'first line\nsecond line');
    await tester.pump();
    await tester.sendKeyEvent(LogicalKeyboardKey.enter);
    await tester.pumpAndSettle();

    expect(StubLibqaulWorker.sentTexts, ['first line\nsecond line']);
    expect(tester.widget<TextField>(field).controller!.text, '');
  });

  testWidgets('chat footer prevents empty sends', (tester) async {
    await pumpChatScreen(tester, buildGroupChat());

    expect(find.byTooltip('Send'), findsNothing);

    await tester.enterText(find.byType(TextField), '   ');
    await tester.pump();

    expect(find.byTooltip('Send'), findsNothing);
    expect(StubLibqaulWorker.sentTexts, isEmpty);
  });

  testWidgets('copies a text message from the long-press menu', (tester) async {
    String? copiedText;
    TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
        .setMockMethodCallHandler(SystemChannels.platform, (call) async {
          if (call.method == 'Clipboard.setData') {
            copiedText =
                (call.arguments as Map<Object?, Object?>)['text'] as String?;
          }
          return null;
        });
    addTearDown(
      () => TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
          .setMockMethodCallHandler(SystemChannels.platform, null),
    );

    await pumpChatScreen(tester, buildDirectChat(), otherUser: otherUser);

    final chat = tester.widget<chat_ui.Chat>(find.byType(chat_ui.Chat));
    chat.onMessageLongPress!(
      tester.element(find.byType(chat_ui.Chat)),
      types.TextMessage(
        id: 'copy-target',
        author: types.User(id: otherUser.idBase58),
        text: 'Text to copy',
      ),
    );
    await tester.pumpAndSettle();

    expect(find.byType(ChatMessageContextMenu), findsOneWidget);
    await tester.tap(find.byKey(const ValueKey('next-page')));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Copy'));
    await tester.pumpAndSettle();

    expect(copiedText, 'Text to copy');
    expect(find.text('Message Copied'), findsOneWidget);
    expect(
      tester.getSize(find.byKey(const ValueKey('copy-feedback-toast'))),
      const Size(140, 52),
    );
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

  testWidgets('disabled room ignores pointers on the chat footer', (
    tester,
  ) async {
    await pumpChatScreen(
      tester,
      buildGroupChat(status: ChatRoomStatus.deactivated),
    );

    final field = find.byType(TextField);
    await tester.tap(field, warnIfMissed: false);
    await tester.pumpAndSettle();

    expect(tester.widget<TextField>(field).focusNode?.hasFocus, isNot(isTrue));
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

  ChatRoom directRoomWith(User user, {DateTime? lastMessageTime}) {
    return ChatRoom(
      name: user.name,
      conversationId: user.conversationId ??
          Uint8List.fromList('${user.name}-conversation'.codeUnits),
      lastMessageTime: lastMessageTime,
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
    expect(
      find.byKey(const ValueKey('forward-user-search-toggle')),
      findsOneWidget,
    );
    expect(
      find.byKey(const ValueKey('forward-group-search-toggle')),
      findsOneWidget,
    );
    expect(find.text('Users / Contacts'), findsOneWidget);
    expect(find.text(otherUser.name), findsOneWidget);
    expect(find.text('Groups'), findsOneWidget);
    expect(find.text('Group Chat'), findsOneWidget);
    expect(
      tester.getTopLeft(find.text('Users / Contacts')).dy,
      lessThan(tester.getTopLeft(find.text('Groups')).dy),
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

  testWidgets('recipient searches filter their own sections', (tester) async {
    await pumpChatScreen(tester, buildDirectChat(), otherUser: otherUser);

    final chat = tester.widget<chat_ui.Chat>(find.byType(chat_ui.Chat));
    chat.onMessageLongPress!(
      tester.element(find.byType(chat_ui.Chat)),
      forwardTargetMessage(),
    );
    await tester.pumpAndSettle();
    await tester.tap(find.text('Forward'));
    await tester.pumpAndSettle();

    await tester.tap(find.byKey(const ValueKey('forward-group-search-toggle')));
    await tester.pumpAndSettle();
    await tester.enterText(
      find.byKey(const ValueKey('forward-group-search')),
      'group',
    );
    await tester.pumpAndSettle();

    expect(find.text('Group Chat'), findsOneWidget);
    expect(find.text(otherUser.name), findsOneWidget);

    await tester.tap(find.byKey(const ValueKey('forward-user-search-toggle')));
    await tester.pumpAndSettle();
    await tester.enterText(
      find.byKey(const ValueKey('forward-user-search')),
      'other',
    );
    await tester.pumpAndSettle();

    expect(find.text(otherUser.name), findsOneWidget);
    expect(find.text('Group Chat'), findsOneWidget);
  });

  testWidgets('forward selector shows five most recent users and groups', (
    tester,
  ) async {
    tester.view.physicalSize = const Size(414, 2000);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);

    final users = List.generate(
      6,
      (index) => testUser(
        'Recent User $index',
        conversationId: Uint8List.fromList('user-$index'.codeUnits),
      ),
    );
    final groups = List.generate(
      6,
      (index) => ChatRoom(
        name: 'Recent Group $index',
        conversationId: Uint8List.fromList('group-$index'.codeUnits),
        isDirectChat: false,
        lastMessageTime: DateTime(2026, 1, index + 1),
      ),
    );
    TestUsersStore.users = users;
    TestChatRoomListNotifier.rooms = [
      ...groups,
      for (var index = 0; index < users.length; index++)
        directRoomWith(
          users[index],
          lastMessageTime: DateTime(2026, 1, index + 1),
        ),
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

    expect(find.text('Recent User 5'), findsOneWidget);
    expect(find.text('Recent User 1'), findsOneWidget);
    expect(find.text('Recent User 0'), findsNothing);
    expect(
      tester.getTopLeft(find.text('Recent User 5')).dy,
      lessThan(tester.getTopLeft(find.text('Recent User 1')).dy),
    );
    expect(find.text('Recent Group 5'), findsOneWidget);
    expect(find.text('Recent Group 1'), findsOneWidget);
    expect(find.text('Recent Group 0'), findsNothing);
    expect(
      tester.getTopLeft(find.text('Recent Group 5')).dy,
      lessThan(tester.getTopLeft(find.text('Recent Group 1')).dy),
    );
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

  testWidgets('opening a chat clears that room unread count locally', (
    tester,
  ) async {
    tester.view.physicalSize = const Size(1024, 768);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);

    late WidgetRef capturedRef;
    final wut = ProviderScope(
      overrides: [
        defaultUserProvider.overrideWith((_) => defaultUser),
        chatNotificationControllerProvider.overrideWithValue(
          NullChatNotificationController(),
        ),
        chatRoomsProvider.overrideWith(TestUnreadChatRoomListNotifier.new),
        qaulWorkerProvider.overrideWith((ref) => StubLibqaulWorker(ref)),
      ],
      child: materialAppWithLocalizations(
        Consumer(
          builder: (context, ref, _) {
            capturedRef = ref;
            return Builder(
              builder: (context) => TextButton(
                onPressed: () => openChat(
                  ref.read(chatRoomsProvider).single,
                  ref: ref,
                  context: context,
                  user: defaultUser,
                ),
                child: const Text('Open chat'),
              ),
            );
          },
        ),
      ),
    );

    await tester.pumpWidget(wut);
    expect(capturedRef.read(chatRoomsProvider).single.unreadCount, 3);

    await tester.tap(find.text('Open chat'));
    await tester.pump();

    expect(capturedRef.read(chatRoomsProvider).single.unreadCount, 0);
    expect(capturedRef.read(currentOpenChatRoom)!.unreadCount, 0);
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
