import 'message_presentation_meta.dart';
import 'qaul_chat_bubble.dart';

// Returns false for incoming neighbors when either senderIdBase58 is empty.
// Callers without senderIds (e.g. the legacy `computeChatBubbleDisplayItems`
// path used by widget tests) therefore never cluster incoming messages in
// group mode — only the timeline-aware path supplies senderIds.
bool _sameParticipantStreakNeighbor(
  ChatTimelinePresentationRow a,
  ChatTimelinePresentationRow b,
) {
  if (a.isOutgoing != b.isOutgoing) return false;
  if (a.isOutgoing) return true;
  if (a.senderIdBase58.isEmpty || b.senderIdBase58.isEmpty) return false;
  return a.senderIdBase58 == b.senderIdBase58;
}

Map<String, MessagePresentationComputation> computeChatMessagePresentation({
  required List<ChatTimelinePresentationRow> ascendingTimeline,
  required ChatRenderMode layoutMode,
}) {
  final textRows = ascendingTimeline
      .where(
        (r) => r.isText && r.qaulBubbleBaseWithoutLayout != null,
      )
      .toList();

  MessagePresentationComputation buildTextComputation(
    ChatTimelinePresentationRow row,
    int textIndexInSequence,
    int timelineIndex,
  ) {
    final bubble = row.qaulBubbleBaseWithoutLayout!;
    final prevTimeline =
        timelineIndex > 0 ? ascendingTimeline[timelineIndex - 1] : null;
    final nextTimeline = timelineIndex < ascendingTimeline.length - 1
        ? ascendingTimeline[timelineIndex + 1]
        : null;

    final prevMinute = textIndexInSequence > 0
        ? textRows[textIndexInSequence - 1].qaulBubbleBaseWithoutLayout
        : null;
    final nextMinute = textIndexInSequence < textRows.length - 1
        ? textRows[textIndexInSequence + 1].qaulBubbleBaseWithoutLayout
        : null;

    final linksToPrevious =
        prevMinute != null && directChatBubblesShareMinute(prevMinute, bubble);
    final linksToNext =
        nextMinute != null && directChatBubblesShareMinute(bubble, nextMinute);

    final isPrimary = row.isOutgoing;

    final tailEdges = isPrimary
        ? tailEdgesForPrimary(linksToPrevious, linksToNext)
        : tailEdgesForSecondary(linksToPrevious, linksToNext);

    final double topSpacing;
    if (prevTimeline == null) {
      topSpacing = 0;
    } else if (layoutMode == ChatRenderMode.group) {
      final isSameStreakSameDay =
          _sameParticipantStreakNeighbor(prevTimeline, row) &&
          samePresentationLocalCalendarDay(prevTimeline.sentAt, row.sentAt);
      if (isSameStreakSameDay) {
        topSpacing = kChatBubbleLinkedGap;
      } else {
        topSpacing = kChatBubbleSeparatedGap;
      }
    } else if (linksToPrevious) {
      topSpacing = kChatBubbleLinkedGap;
    } else {
      topSpacing = kChatBubbleSeparatedGap;
    }

    final showTimestamp = !linksToNext;

    var showSenderName = false;
    var showAvatar = false;

    if (layoutMode == ChatRenderMode.group && row.isGroupIncomingEligible) {
      showSenderName = prevTimeline == null ||
          !_sameParticipantStreakNeighbor(prevTimeline, row) ||
          !samePresentationLocalCalendarDay(
              prevTimeline.sentAt, row.sentAt);

      final continuesAfter = nextTimeline != null &&
          _sameParticipantStreakNeighbor(row, nextTimeline) &&
          samePresentationLocalCalendarDay(row.sentAt, nextTimeline.sentAt);
      showAvatar = !continuesAfter;
    }

    final nonTextClustersWithNext = nextTimeline != null &&
        _sameParticipantStreakNeighbor(row, nextTimeline);

    final meta = MessagePresentationMeta(
      linksToPrevious: linksToPrevious,
      linksToNext: linksToNext,
      showAvatar: showAvatar,
      showSenderName: showSenderName,
      showTimestamp: showTimestamp,
      tailEdges: tailEdges,
      topSpacing: topSpacing,
      nonTextClustersWithNext: nonTextClustersWithNext,
    );

    final messageWithTail = bubble.copyWith(edges: tailEdges);

    final item = QaulChatBubbleDisplayItem(
      message: messageWithTail,
      showTimestamp: showTimestamp,
      marginTop: topSpacing,
    );

    return MessagePresentationComputation(
      meta: meta,
      textDisplay: item,
      isPrimary: row.isOutgoing,
    );
  }

  MessagePresentationComputation emptyNonText(
    ChatTimelinePresentationRow row,
    int timelineIndex,
  ) {
    final prevTimeline =
        timelineIndex > 0 ? ascendingTimeline[timelineIndex - 1] : null;
    final nextTimeline = timelineIndex < ascendingTimeline.length - 1
        ? ascendingTimeline[timelineIndex + 1]
        : null;

    double topSpacing = 0;
    var showAvatar = false;
    var showSenderName = false;

    if (layoutMode == ChatRenderMode.group && row.isGroupIncomingEligible) {
      showSenderName = prevTimeline == null ||
          !_sameParticipantStreakNeighbor(prevTimeline, row) ||
          !samePresentationLocalCalendarDay(prevTimeline.sentAt, row.sentAt);

      final continuesAfter = nextTimeline != null &&
          _sameParticipantStreakNeighbor(row, nextTimeline) &&
          samePresentationLocalCalendarDay(row.sentAt, nextTimeline.sentAt);
      showAvatar = !continuesAfter;

      if (prevTimeline != null) {
        final isSameStreakSameDay =
            _sameParticipantStreakNeighbor(prevTimeline, row) &&
                samePresentationLocalCalendarDay(prevTimeline.sentAt, row.sentAt);
        if (isSameStreakSameDay) {
          topSpacing = kChatBubbleLinkedGap;
        } else {
          topSpacing = kChatBubbleSeparatedGap;
        }
      }
    }

    final nonTextClustersWithNext = nextTimeline != null &&
        _sameParticipantStreakNeighbor(row, nextTimeline);

    final meta = MessagePresentationMeta(
      linksToPrevious: false,
      linksToNext: false,
      showAvatar: showAvatar,
      showSenderName: showSenderName,
      showTimestamp: false,
      tailEdges: const [],
      topSpacing: topSpacing,
      nonTextClustersWithNext: nonTextClustersWithNext,
    );

    return MessagePresentationComputation(
      meta: meta,
      textDisplay: null,
      isPrimary: row.isOutgoing,
    );
  }

  final map = <String, MessagePresentationComputation>{};
  var textSeen = 0;

  for (var i = 0; i < ascendingTimeline.length; i++) {
    final row = ascendingTimeline[i];
    if (row.isText && row.qaulBubbleBaseWithoutLayout != null) {
      map[row.messageIdBase58] =
          buildTextComputation(row, textSeen, i);
      textSeen++;
    } else {
      map[row.messageIdBase58] = emptyNonText(row, i);
    }
  }

  return map;
}

List<QaulChatBubbleDisplayItem> computeChatBubbleDisplayItems(
  List<QaulChatBubbleMessage> messages, {
  ChatRenderMode layoutMode = ChatRenderMode.direct,
}) {
  if (messages.isEmpty) return [];

  final rows = <ChatTimelinePresentationRow>[
    for (var i = 0; i < messages.length; i++)
      ChatTimelinePresentationRow(
        messageIdBase58: 'text-only-$i',
        senderIdBase58: messages[i].senderIdBase58 ?? '',
        sentAt: messages[i].sentAt,
        isText: true,
        isOutgoing: messages[i].messageType == MessageType.primary,
        qaulBubbleBaseWithoutLayout: messages[i].copyWith(edges: const []),
      ),
  ];

  final computed = computeChatMessagePresentation(
    ascendingTimeline: rows,
    layoutMode: layoutMode,
  );

  return [
    for (var i = 0; i < messages.length; i++)
      computed['text-only-$i']!.textDisplay!,
  ];
}
