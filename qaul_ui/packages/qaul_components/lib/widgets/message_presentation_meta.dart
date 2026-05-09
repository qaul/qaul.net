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
    required this.nonTextClustersWithNext,
  });

  final bool linksToPrevious;

  final bool linksToNext;

  final bool showAvatar;

  final bool showSenderName;

  final bool showTimestamp;

  final List<TailEdge> tailEdges;

  final double topSpacing;

  /// True when the next timeline neighbor belongs to the same participant
  /// streak. The non-text bubble path uses this to suppress its tail nip when
  /// clustered. Distinct from flutter-chat-ui's old `nextMessageInGroup`
  /// (which keyed on author + minute, not streak).
  final bool nonTextClustersWithNext;
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
