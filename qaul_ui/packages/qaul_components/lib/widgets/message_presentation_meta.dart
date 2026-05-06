import 'package:flutter/foundation.dart';

import 'qaul_chat_bubble.dart';

bool samePresentationLocalCalendarDay(DateTime a, DateTime b) =>
    a.year == b.year && a.month == b.month && a.day == b.day;

@immutable
class ChatTimelinePresentationRow {
  const ChatTimelinePresentationRow({
    required this.messageIdBase58,
    required this.senderIdBase58,
    required this.sentAt,
    required this.isText,
    required this.isOutgoing,
    this.qaulBubbleBaseWithoutLayout,
  });

  final String messageIdBase58;
  final String senderIdBase58;
  final DateTime sentAt;
  final bool isText;

  final bool isOutgoing;

  final QaulChatBubbleMessage? qaulBubbleBaseWithoutLayout;

  bool get isGroupIncomingEligible =>
      !isOutgoing &&
      senderIdBase58.isNotEmpty;
}

@immutable
class MessagePresentationMeta {
  const MessagePresentationMeta({
    required this.linksToPrevious,
    required this.linksToNext,
    required this.showAvatar,
    required this.showSenderName,
    required this.showTimestamp,
    required this.tailEdges,
    required this.topSpacing,
    required this.legacyBubbleClustersWithNext,
  });

  final bool linksToPrevious;

  final bool linksToNext;

  final bool showAvatar;

  final bool showSenderName;

  final bool showTimestamp;

  final List<TailEdge> tailEdges;

  final double topSpacing;

  final bool legacyBubbleClustersWithNext;
}

@immutable
class MessagePresentationComputation {
  const MessagePresentationComputation({
    required this.meta,
    required this.textDisplay,
    required this.isPrimary,
  });

  final MessagePresentationMeta meta;
  final QaulChatBubbleDisplayItem? textDisplay;
  final bool isPrimary;
}
