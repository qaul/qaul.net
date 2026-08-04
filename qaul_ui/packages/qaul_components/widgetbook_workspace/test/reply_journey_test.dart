import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:qaul_components_widgetbook/use_cases/design/chat/reply_journey/reply_journey.dart';

const _me = ChatUser(id: 'me', name: 'Me');
const _other = ChatUser(id: 'other', name: 'Other user');
final _clock = DateTime(2026, 4, 18, 18, 30);

Finder _findBubbleText(String text) => find.byWidgetPredicate(
  (widget) =>
      widget is RichText &&
      widget.text is TextSpan &&
      (widget.text as TextSpan).text == text,
);

TextChatMessage _message({
  required String id,
  required ChatUser sender,
  required String content,
  int minutesAgo = 1,
}) {
  return TextChatMessage(
    id: id,
    sender: sender,
    content: content,
    sentAt: _clock.subtract(Duration(minutes: minutesAgo)),
    receivedAt: _clock.subtract(Duration(minutes: minutesAgo)),
    status: MessageStatus.read,
  );
}

Widget _app({
  bool group = false,
  List<ChatMessage>? initialMessages,
  ValueChanged<TextChatMessage>? onMessageSent,
  ThemeData? theme,
}) {
  final messages =
      initialMessages ??
      <ChatMessage>[
        _message(id: 'target', sender: _other, content: 'Original message'),
      ];
  final flow = group
      ? ChatReplyJourney.group(
          currentUser: _me,
          initialMessages: messages,
          placeholder: 'Group message',
          clock: _clock,
          currentUserReplyLabel: 'You',
          onMessageSent: onMessageSent,
        )
      : ChatReplyJourney.direct(
          currentUser: _me,
          initialMessages: messages,
          placeholder: 'Secure private message',
          clock: _clock,
          currentUserReplyLabel: 'You',
          onMessageSent: onMessageSent,
        );

  return MaterialApp(
    theme: theme ?? ThemeData.dark(),
    home: Scaffold(body: flow),
  );
}

Future<void> _startReply(WidgetTester tester) async {
  await tester.longPress(find.byKey(const ValueKey('chat-message-target')));
  await tester.pump();
  expect(find.byKey(const ValueKey('chat-reply-context-menu')), findsOneWidget);

  await tester.tap(find.text('Reply'));
  await tester.pump();
}

void main() {
  testWidgets('context menu follows the newly selected message', (
    tester,
  ) async {
    await tester.pumpWidget(
      _app(
        initialMessages: [
          _message(
            id: 'first',
            sender: _other,
            content: 'First message',
            minutesAgo: 10,
          ),
          _message(id: 'second', sender: _other, content: 'Second message'),
        ],
      ),
    );

    await tester.longPress(find.byKey(const ValueKey('chat-message-first')));
    await tester.pump();
    final firstMenuPosition = tester.getTopLeft(
      find.byKey(const ValueKey('chat-reply-context-menu')),
    );

    await tester.tap(find.text('Reply'));
    await tester.pump();
    await tester.tap(find.byKey(const ValueKey('cancel-chat-reply')));
    await tester.pump();

    await tester.longPress(find.byKey(const ValueKey('chat-message-second')));
    await tester.pump();
    final secondMenuPosition = tester.getTopLeft(
      find.byKey(const ValueKey('chat-reply-context-menu')),
    );

    expect(secondMenuPosition, isNot(firstMenuPosition));
  });

  testWidgets('selected outline follows dark and light themes', (tester) async {
    Future<Color> selectedOutline(ThemeData theme) async {
      await tester.pumpWidget(const SizedBox.shrink());
      await tester.pump();
      await tester.pumpWidget(_app(theme: theme));
      await tester.pumpAndSettle();

      await tester.longPress(find.byKey(const ValueKey('chat-message-target')));
      await tester.pump();

      final selectedSurface = tester.widget<DecoratedBox>(
        find.byKey(const ValueKey('chat-bubble-surface')),
      );
      final selectedBorder =
          (selectedSurface.decoration as BoxDecoration).border! as Border;
      expect(selectedBorder.top.width, 2);
      return selectedBorder.top.color;
    }

    expect(await selectedOutline(ThemeData.dark()), Colors.white);
    expect(await selectedOutline(ThemeData.light()), Colors.black);

    await tester.tap(find.text('Reply'));
    await tester.pump();

    final clearedSurface = tester.widget<DecoratedBox>(
      find.byKey(const ValueKey('chat-bubble-surface')),
    );
    expect((clearedSurface.decoration as BoxDecoration).border, isNull);
  });

  testWidgets(
    'Reply closes the menu and shows the selected message in footer',
    (tester) async {
      await tester.pumpWidget(_app());

      await _startReply(tester);

      expect(
        find.byKey(const ValueKey('chat-reply-context-menu')),
        findsNothing,
      );
      final footerPreview = find.byKey(
        const ValueKey('chat-footer-reply-preview'),
      );
      expect(footerPreview, findsOneWidget);
      expect(
        find.descendant(of: footerPreview, matching: find.text('Other user')),
        findsOneWidget,
      );
      expect(
        find.descendant(
          of: footerPreview,
          matching: find.text('Original message'),
        ),
        findsOneWidget,
      );
    },
  );

  testWidgets('cancel clears the active footer reply', (tester) async {
    await tester.pumpWidget(_app());
    await _startReply(tester);

    await tester.tap(find.byKey(const ValueKey('cancel-chat-reply')));
    await tester.pump();

    expect(
      find.byKey(const ValueKey('chat-footer-reply-preview')),
      findsNothing,
    );
  });

  testWidgets('send adds a reply bubble and clears active reply state', (
    tester,
  ) async {
    TextChatMessage? sentMessage;
    await tester.pumpWidget(
      _app(onMessageSent: (message) => sentMessage = message),
    );
    await _startReply(tester);

    await tester.enterText(find.byType(TextField), 'My reply');
    await tester.pump();
    await tester.tap(find.byKey(const ValueKey('chat-footer-send')));
    await tester.pump();

    expect(
      find.byKey(const ValueKey('chat-footer-reply-preview')),
      findsNothing,
    );
    expect(find.byKey(const ValueKey('chat-reply-preview')), findsOneWidget);
    expect(find.text('Original message'), findsWidgets);
    expect(find.text('Other user'), findsOneWidget);
    expect(_findBubbleText('My reply'), findsOneWidget);
    expect(sentMessage?.replyPreview?.author, 'Other user');
    expect(sentMessage?.replyPreview?.content, 'Original message');
  });

  testWidgets('group chat uses the same reply flow', (tester) async {
    TextChatMessage? sentMessage;
    await tester.pumpWidget(
      _app(group: true, onMessageSent: (message) => sentMessage = message),
    );
    await _startReply(tester);

    await tester.enterText(find.byType(TextField), 'Group reply');
    await tester.pump();
    await tester.tap(find.byKey(const ValueKey('chat-footer-send')));
    await tester.pump();

    expect(find.byKey(const ValueKey('chat-reply-preview')), findsOneWidget);
    expect(_findBubbleText('Group reply'), findsOneWidget);
    expect(sentMessage?.replyPreview?.content, 'Original message');
    expect(
      find.byKey(const ValueKey('chat-footer-reply-preview')),
      findsNothing,
    );
  });
}
