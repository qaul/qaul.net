import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:qaul_components/qaul_components.dart';

Finder _findTimestampText(WidgetTester tester) {
  return find.byWidgetPredicate(
    (w) =>
        w is Text &&
        (w.data == null ||
            w.data!.contains('min') ||
            w.data!.contains(':') ||
            w.data!.contains('Now') ||
            w.data!.contains('day')),
  );
}

Finder _findMessageRichText(String content) {
  return find.byWidgetPredicate(
    (widget) =>
        widget is RichText && widget.text.toPlainText().trim() == content,
  );
}

void main() {
  group('computeChatBubbleDisplayItems', () {
    test('links messages from same sender and same minute', () {
      final base = DateTime(2026, 4, 19, 19, 23);

      final messages = [
        QaulChatBubbleMessage(
          content: 'first',
          sentAt: base,
          receivedAt: base,
          status: MessageStatus.read,
          messageType: MessageType.primary,
          edges: const [],
        ),
        QaulChatBubbleMessage(
          content: 'middle',
          sentAt: base,
          receivedAt: base,
          status: MessageStatus.read,
          messageType: MessageType.primary,
          edges: const [],
        ),
        QaulChatBubbleMessage(
          content: 'last',
          sentAt: base,
          receivedAt: base,
          status: MessageStatus.read,
          messageType: MessageType.primary,
          edges: const [],
        ),
      ];

      final items = computeChatBubbleDisplayItems(
        messages,
        layoutMode: ChatRenderMode.direct,
      );
      expect(items, hasLength(3));

      expect(items[0].message.edges, const [TailEdge.bottomEnd]);
      expect(items[1].message.edges, const [
        TailEdge.topEnd,
        TailEdge.bottomEnd,
      ]);
      expect(items[2].message.edges, const [TailEdge.topEnd]);

      expect(items[0].showTimestamp, isFalse);
      expect(items[1].showTimestamp, isFalse);
      expect(items[2].showTimestamp, isTrue);

      expect(items[0].marginTop, 0.0);
      expect(items[1].marginTop, kChatBubbleLinkedGap);
      expect(items[2].marginTop, kChatBubbleLinkedGap);
    });

    test('does not link messages from different sender or minute', () {
      final base = DateTime(2026, 4, 19, 19, 23);

      final messages = [
        QaulChatBubbleMessage(
          content: 'primary',
          sentAt: base,
          receivedAt: base,
          status: MessageStatus.sent,
          messageType: MessageType.primary,
          edges: const [],
        ),
        QaulChatBubbleMessage(
          content: 'primary later',
          sentAt: base.add(const Duration(minutes: 1)),
          receivedAt: base.add(const Duration(minutes: 1)),
          status: MessageStatus.sent,
          messageType: MessageType.primary,
          edges: const [],
        ),
        QaulChatBubbleMessage(
          content: 'secondary same minute as second',
          sentAt: base.add(const Duration(minutes: 1)),
          receivedAt: base.add(const Duration(minutes: 1)),
          status: MessageStatus.sent,
          messageType: MessageType.secondary,
          edges: const [],
        ),
      ];

      final items = computeChatBubbleDisplayItems(
        messages,
        layoutMode: ChatRenderMode.direct,
      );
      expect(items, hasLength(3));

      expect(items[0].message.edges, const [TailEdge.bottomEnd]);
      expect(items[1].message.edges, const [TailEdge.bottomEnd]);
      expect(items[2].message.edges, const [TailEdge.bottomStart]);

      expect(items[0].showTimestamp, isTrue);
      expect(items[1].showTimestamp, isTrue);
      expect(items[2].showTimestamp, isTrue);

      expect(items[0].marginTop, 0.0);
      expect(items[1].marginTop, kChatBubbleSeparatedGap);
      expect(items[2].marginTop, kChatBubbleSeparatedGap);
    });

    test(
      'group layout: edges/timestamps match direct; 4px vertical only for same '
      'participant streak, 12px when switching me vs others',
      () {
        final base = DateTime(2026, 4, 19, 19, 23);

        final messages = [
          QaulChatBubbleMessage(
            content: 'primary',
            sentAt: base,
            receivedAt: base,
            status: MessageStatus.sent,
            messageType: MessageType.primary,
            edges: const [],
          ),
          QaulChatBubbleMessage(
            content: 'primary later',
            sentAt: base.add(const Duration(minutes: 1)),
            receivedAt: base.add(const Duration(minutes: 1)),
            status: MessageStatus.sent,
            messageType: MessageType.primary,
            edges: const [],
          ),
          QaulChatBubbleMessage(
            content: 'secondary same minute as second',
            sentAt: base.add(const Duration(minutes: 1)),
            receivedAt: base.add(const Duration(minutes: 1)),
            status: MessageStatus.sent,
            messageType: MessageType.secondary,
            edges: const [],
          ),
        ];

        final directItems = computeChatBubbleDisplayItems(
          messages,
          layoutMode: ChatRenderMode.direct,
        );
        final groupItems = computeChatBubbleDisplayItems(
          messages,
          layoutMode: ChatRenderMode.group,
        );

        for (var i = 0; i < 3; i++) {
          expect(groupItems[i].message.edges, directItems[i].message.edges);
          expect(groupItems[i].showTimestamp, directItems[i].showTimestamp);
        }

        expect(directItems[1].marginTop, kChatBubbleSeparatedGap);
        expect(directItems[2].marginTop, kChatBubbleSeparatedGap);
        expect(groupItems[1].marginTop, kChatBubbleLinkedGap);
        expect(groupItems[2].marginTop, kChatBubbleSeparatedGap);
      },
    );
  });

  group('formatQaulChatBubbleTime', () {
    test('relative minutes uses clock, not wall time', () {
      final clock = DateTime(2026, 6, 1, 12, 0);
      final sent = clock.subtract(const Duration(minutes: 5));
      final m = QaulChatBubbleMessage(
        content: 'x',
        sentAt: sent,
        receivedAt: sent,
        status: MessageStatus.sent,
        messageType: MessageType.primary,
        edges: const [],
      );
      expect(formatQaulChatBubbleTime(m, clock), '5 min');
    });
  });

  group('QaulChatBubble timestamp formatting', () {
    testWidgets('shows minutes when sent less than an hour before clock', (
      tester,
    ) async {
      final clock = DateTime(2026, 6, 1, 12, 0);
      final fiveMinutesAgo = clock.subtract(const Duration(minutes: 5));

      final message = QaulChatBubbleMessage(
        content: 'recent message',
        sentAt: fiveMinutesAgo,
        receivedAt: fiveMinutesAgo,
        status: MessageStatus.sent,
        messageType: MessageType.primary,
        edges: const [],
      );

      await tester.pumpWidget(
        MaterialApp(
          home: Material(
            child: QaulChatBubble(
              message: message,
              clock: clock,
              showTimestamp: true,
            ),
          ),
        ),
      );

      expect(find.text('5 min'), findsOneWidget);
    });

    testWidgets(
      'shows absolute time when sent more than an hour before clock',
      (tester) async {
        final clock = DateTime(2026, 6, 1, 12, 0);
        final ninetyMinutesAgo = clock.subtract(const Duration(minutes: 90));

        final message = QaulChatBubbleMessage(
          content: 'older message',
          sentAt: ninetyMinutesAgo,
          receivedAt: ninetyMinutesAgo,
          status: MessageStatus.sent,
          messageType: MessageType.primary,
          edges: const [],
        );

        await tester.pumpWidget(
          MaterialApp(
            home: Material(
              child: QaulChatBubble(
                message: message,
                clock: clock,
                showTimestamp: true,
              ),
            ),
          ),
        );

        final timestampText = tester.widget<Text>(
          _findTimestampText(tester).first,
        );
        final label = timestampText.data ?? '';
        expect(label.contains('min'), isFalse);
        expect(label.contains(':'), isTrue);
      },
    );

    testWidgets(
      'sender (primary) read message shows sent time + days when received later',
      (tester) async {
        final clock = DateTime(2026, 4, 21, 16, 0);
        final sentAt = DateTime(2026, 4, 19, 14, 50);
        final receivedAt = DateTime(2026, 4, 20, 15, 50);

        final message = QaulChatBubbleMessage(
          content: 'hello',
          sentAt: sentAt,
          receivedAt: receivedAt,
          status: MessageStatus.read,
          messageType: MessageType.primary,
          edges: const [],
        );

        await tester.pumpWidget(
          MaterialApp(
            home: Material(
              child: QaulChatBubble(
                message: message,
                clock: clock,
                showTimestamp: true,
              ),
            ),
          ),
        );

        final timestampText = tester.widget<Text>(
          _findTimestampText(tester).first,
        );
        final label = timestampText.data ?? '';
        expect(label.contains('+ 1 day'), isTrue);
      },
    );

    testWidgets(
      'receiver (secondary) read message shows received time + days ago',
      (tester) async {
        final clock = DateTime(2026, 4, 21, 16, 0);
        final sentAt = DateTime(2026, 4, 19, 14, 50);
        final receivedAt = DateTime(2026, 4, 20, 15, 50);

        final message = QaulChatBubbleMessage(
          content: 'hello',
          sentAt: sentAt,
          receivedAt: receivedAt,
          status: MessageStatus.read,
          messageType: MessageType.secondary,
          edges: const [],
        );

        await tester.pumpWidget(
          MaterialApp(
            home: Material(
              child: QaulChatBubble(
                message: message,
                clock: clock,
                showTimestamp: true,
              ),
            ),
          ),
        );

        final timestampText = tester.widget<Text>(
          _findTimestampText(tester).first,
        );
        final label = timestampText.data ?? '';
        expect(label.contains('1 day ago'), isTrue);
      },
    );

    testWidgets('read message same day has no days suffix', (tester) async {
      final clock = DateTime(2026, 4, 19, 18, 0);
      final sameDay = DateTime(2026, 4, 19, 14, 50);
      final receivedSameDay = DateTime(2026, 4, 19, 15, 50);

      final message = QaulChatBubbleMessage(
        content: 'hello',
        sentAt: sameDay,
        receivedAt: receivedSameDay,
        status: MessageStatus.read,
        messageType: MessageType.primary,
        edges: const [],
      );

      await tester.pumpWidget(
        MaterialApp(
          home: Material(
            child: QaulChatBubble(
              message: message,
              clock: clock,
              showTimestamp: true,
            ),
          ),
        ),
      );

      final timestampText = tester.widget<Text>(
        _findTimestampText(tester).first,
      );
      final label = timestampText.data ?? '';
      expect(label.contains('+ '), isFalse);
      expect(label.contains(' ago'), isFalse);
    });

    testWidgets('sent (not read) message has no days suffix', (tester) async {
      final clock = DateTime(2026, 4, 21, 16, 0);
      final sentAt = DateTime(2026, 4, 19, 14, 50);
      final receivedAt = DateTime(2026, 4, 20, 15, 50);

      final message = QaulChatBubbleMessage(
        content: 'hello',
        sentAt: sentAt,
        receivedAt: receivedAt,
        status: MessageStatus.sent,
        messageType: MessageType.primary,
        edges: const [],
      );

      await tester.pumpWidget(
        MaterialApp(
          home: Material(
            child: QaulChatBubble(
              message: message,
              clock: clock,
              showTimestamp: true,
            ),
          ),
        ),
      );

      final timestampText = tester.widget<Text>(
        _findTimestampText(tester).first,
      );
      final label = timestampText.data ?? '';
      expect(label.contains('+ 1 day'), isFalse);
      expect(label.contains(' ago'), isFalse);
    });
  });

  group('QaulChatBubble content', () {
    testWidgets('preserves internal newlines (trim only)', (tester) async {
      final clock = DateTime(2026, 1, 1, 12, 0);
      final message = QaulChatBubbleMessage(
        content: '  line one\nline two  ',
        sentAt: clock,
        receivedAt: clock,
        status: MessageStatus.sent,
        messageType: MessageType.secondary,
        edges: const [],
      );

      await tester.pumpWidget(
        MaterialApp(
          home: Material(
            child: QaulChatBubble(
              message: message,
              clock: clock,
              showTimestamp: false,
            ),
          ),
        ),
      );

      final rich = tester.widget<RichText>(find.byType(RichText).first);
      final text = rich.text.toPlainText();
      expect(text, 'line one\nline two');
    });

    testWidgets('scales message and timestamp text with chat bubble cap', (
      tester,
    ) async {
      final clock = DateTime(2026, 1, 1, 12, 0);
      final message = QaulChatBubbleMessage(
        content: 'scaled message',
        sentAt: clock,
        receivedAt: clock,
        status: MessageStatus.sent,
        messageType: MessageType.primary,
        edges: const [],
      );

      await tester.pumpWidget(
        MaterialApp(
          home: MediaQuery(
            data: const MediaQueryData(textScaler: TextScaler.linear(2)),
            child: Material(
              child: QaulChatBubble(
                message: message,
                clock: clock,
                showTimestamp: true,
              ),
            ),
          ),
        ),
      );

      final rich = tester.widget<RichText>(find.byType(RichText).first);
      final timestampText = tester.widget<Text>(find.text('Now'));

      expect(
        rich.textScaler.scale(ChatBubbleStyle.textStyle.fontSize!),
        moreOrLessEquals(
          ChatBubbleStyle.textStyle.fontSize! * kChatBubbleMaxTextScaleFactor,
        ),
      );
      expect(
        timestampText.textScaler!.scale(ChatBubbleStyle.timeStyle.fontSize!),
        moreOrLessEquals(
          ChatBubbleStyle.timeStyle.fontSize! * kChatBubbleMaxTextScaleFactor,
        ),
      );
    });
  });

  group('QaulChatBubble reply preview', () {
    Widget app(QaulChatBubbleMessage message, DateTime clock) {
      return MaterialApp(
        home: Material(
          child: QaulChatBubble(
            message: message,
            clock: clock,
            showTimestamp: true,
          ),
        ),
      );
    }

    testWidgets('renders an outgoing reply to own message above content', (
      tester,
    ) async {
      final clock = DateTime(2026, 4, 19, 10, 53);
      final message = QaulChatBubbleMessage(
        content: 'As I said earlier...',
        sentAt: clock,
        receivedAt: clock,
        status: MessageStatus.read,
        messageType: MessageType.primary,
        edges: const [TailEdge.bottomEnd],
        replyPreview: const ChatReplyPreviewData(
          author: 'You',
          content: 'This is about quoting myself.',
        ),
      );

      await tester.pumpWidget(app(message, clock));

      expect(find.byKey(const ValueKey('chat-reply-preview')), findsOneWidget);
      expect(find.text('You'), findsOneWidget);
      expect(find.text('This is about quoting myself.'), findsOneWidget);
      expect(_findMessageRichText('As I said earlier...'), findsOneWidget);
      expect(find.text('Now'), findsOneWidget);
      expect(find.byIcon(Icons.done_all), findsOneWidget);

      final previewPadding = tester.widget<Padding>(
        find.descendant(
          of: find.byKey(const ValueKey('chat-reply-preview')),
          matching: find.byType(Padding),
        ),
      );
      expect(
        previewPadding.padding,
        const EdgeInsets.symmetric(horizontal: 10, vertical: 4),
      );
      final bubblePadding = tester.widget<Padding>(
        find.byKey(const ValueKey('chat-bubble-padding')),
      );
      expect(
        bubblePadding.padding,
        const EdgeInsets.symmetric(horizontal: 10, vertical: 10),
      );

      final bubbleRect = tester.getRect(
        find.byKey(const ValueKey('chat-bubble-surface')),
      );
      final messageRect = tester.getRect(
        _findMessageRichText('As I said earlier...'),
      );
      final statusRect = tester.getRect(find.byIcon(Icons.done_all));
      expect(messageRect.left - bubbleRect.left, 10);
      expect(bubbleRect.right - statusRect.right, 10);

      final previewTop = tester.getTopLeft(find.text('You')).dy;
      final messageTop = tester
          .getTopLeft(_findMessageRichText('As I said earlier...'))
          .dy;
      expect(previewTop, lessThan(messageTop));
    });

    testWidgets('renders an incoming reply to another user', (tester) async {
      final clock = DateTime(2026, 4, 19, 10, 53);
      final message = QaulChatBubbleMessage(
        content: 'Written in the morning',
        sentAt: clock.subtract(const Duration(minutes: 44)),
        receivedAt: clock.subtract(const Duration(minutes: 44)),
        status: MessageStatus.sent,
        messageType: MessageType.secondary,
        edges: const [TailEdge.bottomStart],
        replyPreview: const ChatReplyPreviewData(
          author: 'MaxX',
          content: 'This is a quote sent by another user.',
        ),
      );

      await tester.pumpWidget(app(message, clock));

      expect(find.text('MaxX'), findsOneWidget);
      expect(
        find.text('This is a quote sent by another user.'),
        findsOneWidget,
      );
      expect(_findMessageRichText('Written in the morning'), findsOneWidget);

      final author = tester.widget<Text>(find.text('MaxX'));
      expect(author.style!.color, Colors.white);
    });

    testWidgets('constrains and truncates a long reply excerpt', (
      tester,
    ) async {
      final clock = DateTime(2026, 4, 19, 10, 53);
      const excerpt =
          'This is a very long replied message that should remain inside the '
          'bubble and be truncated after a few lines instead of expanding the '
          'timeline row indefinitely or overflowing its horizontal bounds.';
      final message = QaulChatBubbleMessage(
        content: 'Short response',
        sentAt: clock,
        receivedAt: clock,
        status: MessageStatus.sent,
        messageType: MessageType.primary,
        edges: const [TailEdge.bottomEnd],
        replyPreview: const ChatReplyPreviewData(
          author: 'Another participant with a long name',
          content: excerpt,
        ),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: MediaQuery(
            data: const MediaQueryData(size: Size(320, 640)),
            child: Material(
              child: QaulChatBubble(
                message: message,
                clock: clock,
                showTimestamp: true,
              ),
            ),
          ),
        ),
      );

      expect(
        tester.getSize(find.byKey(const ValueKey('chat-reply-preview'))).width,
        lessThanOrEqualTo(
          ChatBubbleStyle.maxBubbleWidthMobile -
              ChatBubbleStyle.horizontalPadding * 2,
        ),
      );
      final excerptText = tester.widget<Text>(find.text(excerpt));
      expect(excerptText.maxLines, 3);
      expect(excerptText.overflow, TextOverflow.ellipsis);
    });

    testWidgets('does not render reply UI without reply data', (tester) async {
      final clock = DateTime(2026, 4, 19, 10, 53);
      final message = QaulChatBubbleMessage(
        content: 'Regular message',
        sentAt: clock,
        receivedAt: clock,
        status: MessageStatus.sent,
        messageType: MessageType.secondary,
        edges: const [TailEdge.bottomStart],
      );

      await tester.pumpWidget(app(message, clock));

      expect(find.byKey(const ValueKey('chat-reply-preview')), findsNothing);
      expect(_findMessageRichText('Regular message'), findsOneWidget);
    });

    testWidgets('keeps timestamp beside short text without reply data', (
      tester,
    ) async {
      final clock = DateTime(2026, 4, 19, 10, 53);
      final message = QaulChatBubbleMessage(
        content: 'Hi',
        sentAt: clock,
        receivedAt: clock,
        status: MessageStatus.sent,
        messageType: MessageType.secondary,
        edges: const [TailEdge.bottomStart],
      );

      await tester.pumpWidget(app(message, clock));

      final messageRect = tester.getRect(_findMessageRichText('Hi'));
      final timestampRect = tester.getRect(find.text('Now'));
      expect(
        timestampRect.left - messageRect.right,
        ChatBubbleStyle.gapBetweenTextAndDate,
      );
    });
  });
}
