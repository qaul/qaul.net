import 'package:flutter_test/flutter_test.dart';
import 'package:qaul_components/qaul_components.dart';

void main() {
  group('computeChatMessagePresentation', () {
    test('group texts: same sender same day stays 4px even when minutes differ',
        () {
      final t1 = DateTime(2026, 4, 19, 8, 9);
      final t2 = DateTime(2026, 4, 19, 23, 39);
      final m0 = QaulChatBubbleMessage(
        content: 'a',
        sentAt: t1,
        receivedAt: t1,
        status: MessageStatus.sent,
        messageType: MessageType.secondary,
        edges: const [],
        senderIdBase58: 'tm',
      );
      final m1 = QaulChatBubbleMessage(
        content: 'b',
        sentAt: t2,
        receivedAt: t2,
        status: MessageStatus.sent,
        messageType: MessageType.secondary,
        edges: const [],
        senderIdBase58: 'tm',
      );
      final timeline = <ChatTimelinePresentationRow>[
        ChatTimelinePresentationRow(
          messageIdBase58: 'id-0',
          senderIdBase58: 'tm',
          sentAt: t1,
          isText: true,
          isOutgoing: false,
          qaulBubbleBaseWithoutLayout: m0,
        ),
        ChatTimelinePresentationRow(
          messageIdBase58: 'id-1',
          senderIdBase58: 'tm',
          sentAt: t2,
          isText: true,
          isOutgoing: false,
          qaulBubbleBaseWithoutLayout: m1,
        ),
      ];

      final out = computeChatMessagePresentation(
        ascendingTimeline: timeline,
        layoutMode: ChatRenderMode.group,
      );

      expect(out['id-1']!.meta.linksToPrevious, isFalse,
          reason: 'different calendar minutes → unlinked tails');
      expect(out['id-1']!.meta.topSpacing, kChatBubbleLinkedGap,
          reason: 'same streak (sender + calendar day) → compact gap');
    });

    test('group incoming: name at streak start, avatar at streak end same day', () {
      final base = DateTime(2026, 4, 19, 21, 19);
      final m0 = QaulChatBubbleMessage(
        content: 'a',
        sentAt: base,
        receivedAt: base,
        status: MessageStatus.sent,
        messageType: MessageType.secondary,
        edges: const [],
        senderIdBase58: 'tm',
      );
      final m1 = QaulChatBubbleMessage(
        content: 'b',
        sentAt: base,
        receivedAt: base,
        status: MessageStatus.sent,
        messageType: MessageType.secondary,
        edges: const [],
        senderIdBase58: 'tm',
      );
      final timeline = <ChatTimelinePresentationRow>[
        ChatTimelinePresentationRow(
          messageIdBase58: 'id-0',
          senderIdBase58: 'tm',
          sentAt: base,
          isText: true,
          isOutgoing: false,
          qaulBubbleBaseWithoutLayout: m0,
        ),
        ChatTimelinePresentationRow(
          messageIdBase58: 'id-1',
          senderIdBase58: 'tm',
          sentAt: base,
          isText: true,
          isOutgoing: false,
          qaulBubbleBaseWithoutLayout: m1,
        ),
      ];

      final out = computeChatMessagePresentation(
        ascendingTimeline: timeline,
        layoutMode: ChatRenderMode.group,
      );

      expect(out['id-0']!.meta.showSenderName, isTrue);
      expect(out['id-0']!.meta.showAvatar, isFalse);
      expect(out['id-1']!.meta.showSenderName, isFalse);
      expect(out['id-1']!.meta.showAvatar, isTrue);
    });

    test('group incoming: new local calendar day restarts streak', () {
      final d0 = DateTime(2026, 4, 19, 23, 0);
      final d1 = DateTime(2026, 4, 20, 1, 0);
      final m0 = QaulChatBubbleMessage(
        content: 'a',
        sentAt: d0,
        receivedAt: d0,
        status: MessageStatus.sent,
        messageType: MessageType.secondary,
        edges: const [],
        senderIdBase58: 'tm',
      );
      final m1 = QaulChatBubbleMessage(
        content: 'b',
        sentAt: d1,
        receivedAt: d1,
        status: MessageStatus.sent,
        messageType: MessageType.secondary,
        edges: const [],
        senderIdBase58: 'tm',
      );
      final timeline = <ChatTimelinePresentationRow>[
        ChatTimelinePresentationRow(
          messageIdBase58: 'id-0',
          senderIdBase58: 'tm',
          sentAt: d0,
          isText: true,
          isOutgoing: false,
          qaulBubbleBaseWithoutLayout: m0,
        ),
        ChatTimelinePresentationRow(
          messageIdBase58: 'id-1',
          senderIdBase58: 'tm',
          sentAt: d1,
          isText: true,
          isOutgoing: false,
          qaulBubbleBaseWithoutLayout: m1,
        ),
      ];

      final out = computeChatMessagePresentation(
        ascendingTimeline: timeline,
        layoutMode: ChatRenderMode.group,
      );

      expect(out['id-0']!.meta.showAvatar, isTrue);
      expect(out['id-1']!.meta.showSenderName, isTrue);
      expect(out['id-1']!.meta.showAvatar, isTrue);
    });

    test('non-text sandwiched between same sender does not break streak', () {
      final t0 = DateTime(2026, 4, 19, 10, 0);
      final tMid = DateTime(2026, 4, 19, 10, 1);
      final t2 = DateTime(2026, 4, 19, 10, 2);
      final text0 = QaulChatBubbleMessage(
        content: 'a',
        sentAt: t0,
        receivedAt: t0,
        status: MessageStatus.sent,
        messageType: MessageType.secondary,
        edges: const [],
        senderIdBase58: 'alice',
      );
      final text1 = QaulChatBubbleMessage(
        content: 'b',
        sentAt: t2,
        receivedAt: t2,
        status: MessageStatus.sent,
        messageType: MessageType.secondary,
        edges: const [],
        senderIdBase58: 'alice',
      );
      final timeline = <ChatTimelinePresentationRow>[
        ChatTimelinePresentationRow(
          messageIdBase58: 'id-0',
          senderIdBase58: 'alice',
          sentAt: t0,
          isText: true,
          isOutgoing: false,
          qaulBubbleBaseWithoutLayout: text0,
        ),
        ChatTimelinePresentationRow(
          messageIdBase58: 'id-media',
          senderIdBase58: 'alice',
          sentAt: tMid,
          isText: false,
          isOutgoing: false,
        ),
        ChatTimelinePresentationRow(
          messageIdBase58: 'id-1',
          senderIdBase58: 'alice',
          sentAt: t2,
          isText: true,
          isOutgoing: false,
          qaulBubbleBaseWithoutLayout: text1,
        ),
      ];

      final out = computeChatMessagePresentation(
        ascendingTimeline: timeline,
        layoutMode: ChatRenderMode.group,
      );

      expect(out['id-0']!.meta.showSenderName, isTrue);
      expect(out['id-0']!.meta.showAvatar, isFalse);
      expect(out['id-1']!.meta.showSenderName, isFalse);
      expect(out['id-1']!.meta.showAvatar, isTrue);
    });

    test('direct: same clock minute on different calendar days does not link',
        () {
      // Same wall-clock minute, but a day apart → must not cluster/link.
      final d1 = DateTime(2026, 5, 8, 11, 38);
      final d2 = DateTime(2026, 5, 9, 11, 38);
      final m0 = QaulChatBubbleMessage(
        content: 'a',
        sentAt: d1,
        receivedAt: d1,
        status: MessageStatus.sent,
        messageType: MessageType.primary,
        edges: const [],
        senderIdBase58: 'me',
      );
      final m1 = QaulChatBubbleMessage(
        content: 'b',
        sentAt: d2,
        receivedAt: d2,
        status: MessageStatus.sent,
        messageType: MessageType.primary,
        edges: const [],
        senderIdBase58: 'me',
      );
      final timeline = <ChatTimelinePresentationRow>[
        ChatTimelinePresentationRow(
          messageIdBase58: 'id-0',
          senderIdBase58: 'me',
          sentAt: d1,
          isText: true,
          isOutgoing: true,
          qaulBubbleBaseWithoutLayout: m0,
        ),
        ChatTimelinePresentationRow(
          messageIdBase58: 'id-1',
          senderIdBase58: 'me',
          sentAt: d2,
          isText: true,
          isOutgoing: true,
          qaulBubbleBaseWithoutLayout: m1,
        ),
      ];

      final out = computeChatMessagePresentation(
        ascendingTimeline: timeline,
        layoutMode: ChatRenderMode.direct,
      );

      expect(out['id-0']!.meta.linksToNext, isFalse,
          reason: 'a day boundary breaks the minute cluster');
      expect(out['id-1']!.meta.linksToPrevious, isFalse,
          reason: 'a day boundary breaks the minute cluster');
      expect(out['id-1']!.meta.topSpacing, kChatBubbleSeparatedGap,
          reason: 'cross-day neighbors get the separated gap');
    });
  });
}
