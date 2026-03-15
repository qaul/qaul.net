import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:qaul_components/qaul_components.dart';

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

  group('QaulChatBubble timestamp formatting', () {
    testWidgets('shows minutes when sent less than an hour ago',
        (tester) async {
      final now = DateTime.now();
      final fiveMinutesAgo = now.subtract(const Duration(minutes: 5));

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
              showTimestamp: true,
            ),
          ),
        ),
      );

      expect(find.text('5 min'), findsOneWidget);
    });

    testWidgets('shows absolute time when sent more than an hour ago',
        (tester) async {
      final now = DateTime.now();
      final ninetyMinutesAgo = now.subtract(const Duration(minutes: 90));

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
              showTimestamp: true,
            ),
          ),
        ),
      );

      final timestampFinder = find.byType(Text).at(1);
      final timestampText = tester.widget<Text>(timestampFinder);
      final label = timestampText.data ?? '';
      expect(label.contains('min'), isFalse);
      expect(label.contains(':'), isTrue);
    });
  });
}
