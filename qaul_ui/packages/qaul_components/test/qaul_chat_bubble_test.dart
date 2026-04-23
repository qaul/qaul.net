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

      final items = computeChatBubbleDisplayItems(messages);
      expect(items, hasLength(3));

      expect(items[0].message.edges, const [TailEdge.bottomEnd]);
      expect(items[1].message.edges,
          const [TailEdge.topEnd, TailEdge.bottomEnd]);
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

      final items = computeChatBubbleDisplayItems(messages);
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
    testWidgets('shows minutes when sent less than an hour before clock',
        (tester) async {
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

    testWidgets('shows absolute time when sent more than an hour before clock',
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

      final timestampText =
          tester.widget<Text>(_findTimestampText(tester).first);
      final label = timestampText.data ?? '';
      expect(label.contains('min'), isFalse);
      expect(label.contains(':'), isTrue);
    });

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

      final timestampText =
          tester.widget<Text>(_findTimestampText(tester).first);
      final label = timestampText.data ?? '';
      expect(label.contains('+ 1 day'), isTrue);
    });

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

      final timestampText =
          tester.widget<Text>(_findTimestampText(tester).first);
      final label = timestampText.data ?? '';
      expect(label.contains('1 day ago'), isTrue);
    });

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

      final timestampText =
          tester.widget<Text>(_findTimestampText(tester).first);
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

      final timestampText =
          tester.widget<Text>(_findTimestampText(tester).first);
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
  });
}
