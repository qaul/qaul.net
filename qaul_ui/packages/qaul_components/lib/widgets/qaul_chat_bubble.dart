import 'package:flutter/material.dart';

enum TailEdge { topStart, topEnd, bottomStart, bottomEnd }

enum MessageStatus { notSent, sent, read }

enum MessageType { primary, secondary }

class QaulChatBubbleMessage {
  const QaulChatBubbleMessage({
    required this.content,
    required this.sentAt,
    required this.receivedAt,
    required this.status,
    required this.messageType,
    required this.edges,
  });

  final String content;
  final DateTime sentAt;
  final DateTime receivedAt;
  final MessageStatus status;
  final MessageType messageType;
  final List<TailEdge> edges;

  QaulChatBubbleMessage copyWith({
    String? content,
    DateTime? sentAt,
    DateTime? receivedAt,
    MessageStatus? status,
    MessageType? messageType,
    List<TailEdge>? edges,
  }) {
    return QaulChatBubbleMessage(
      content: content ?? this.content,
      sentAt: sentAt ?? this.sentAt,
      receivedAt: receivedAt ?? this.receivedAt,
      status: status ?? this.status,
      messageType: messageType ?? this.messageType,
      edges: edges ?? this.edges,
    );
  }
}

class QaulChatBubble extends StatelessWidget {
  const QaulChatBubble({
    super.key,
    required this.message,
    this.showTimestamp = true,
  });

  final QaulChatBubbleMessage message;

  /// When false, the timestamp and status row is hidden (e.g. for non-last
  /// messages in a linked group).
  final bool showTimestamp;

  @override
  Widget build(BuildContext context) {
    final isPrimary = message.messageType == MessageType.primary;

    final backgroundColor = isPrimary
        ? const Color(0xFF1976D2)
        : const Color(0xFF424242);

    final textColor = Colors.white;

    final defaultRadius = const Radius.circular(18);

    final hasTopStart = message.edges.contains(TailEdge.topStart);
    final hasTopEnd = message.edges.contains(TailEdge.topEnd);
    final hasBottomStart = message.edges.contains(TailEdge.bottomStart);
    final hasBottomEnd = message.edges.contains(TailEdge.bottomEnd);

    final borderRadius = BorderRadiusDirectional.only(
      topStart: hasTopStart ? Radius.zero : defaultRadius,
      topEnd: hasTopEnd ? Radius.zero : defaultRadius,
      bottomStart: hasBottomStart ? Radius.zero : defaultRadius,
      bottomEnd: hasBottomEnd ? Radius.zero : defaultRadius,
    );

    final timeOfDay = TimeOfDay.fromDateTime(message.sentAt);
    final timeLabel = timeOfDay.format(context);

    Widget? statusIcon;
    switch (message.status) {
      case MessageStatus.notSent:
        statusIcon = null;
        break;
      case MessageStatus.sent:
        statusIcon = Icon(
          Icons.check,
          size: 14,
          color: textColor.withValues(alpha: 0.8),
        );
        break;
      case MessageStatus.read:
        statusIcon = Icon(
          Icons.done_all,
          size: 14,
          color: textColor.withValues(alpha: 0.9),
        );
        break;
    }

    final maxBubbleWidth =
        MediaQuery.sizeOf(context).width.clamp(280.0, 420.0) * 0.75;

    return Align(
      alignment: isPrimary ? Alignment.centerRight : Alignment.centerLeft,
      child: ConstrainedBox(
        constraints: BoxConstraints(maxWidth: maxBubbleWidth),
        child: DecoratedBox(
          decoration: BoxDecoration(
            color: backgroundColor,
            borderRadius: borderRadius,
          ),
          child: Padding(
            padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              mainAxisSize: MainAxisSize.min,
              children: [
                Text(
                  message.content,
                  style: TextStyle(
                    fontSize: 16,
                    fontWeight: FontWeight.w300,
                    color: textColor,
                  ),
                ),
                if (showTimestamp) ...[
                  const SizedBox(height: 4),
                  Row(
                    mainAxisSize: MainAxisSize.min,
                    mainAxisAlignment: MainAxisAlignment.end,
                    children: [
                      Text(
                        timeLabel,
                        style: TextStyle(
                          fontSize: 11,
                          color: textColor.withValues(alpha: 0.8),
                        ),
                      ),
                      if (isPrimary && statusIcon != null) ...[
                        const SizedBox(width: 4),
                        statusIcon,
                      ],
                    ],
                  ),
                ],
              ],
            ),
          ),
        ),
      ),
    );
  }
}

/// Spacing between bubbles: 4px when linked, 12px when separated.
const double kChatBubbleLinkedGap = 4.0;
const double kChatBubbleSeparatedGap = 12.0;

/// One bubble ready to display: message with computed [edges], whether to show
/// timestamp, and margin above.
class QaulChatBubbleDisplayItem {
  const QaulChatBubbleDisplayItem({
    required this.message,
    required this.showTimestamp,
    required this.marginTop,
  });

  final QaulChatBubbleMessage message;
  final bool showTimestamp;
  final double marginTop;
}

/// Two messages are "linked" when from same sender (messageType) and sent in
/// the same minute (same year, month, day, hour, minute of [sentAt]).
bool isChatBubbleLinked(QaulChatBubbleMessage a, QaulChatBubbleMessage b) {
  if (a.messageType != b.messageType) return false;
  final ta = a.sentAt;
  final tb = b.sentAt;
  return ta.year == tb.year &&
      ta.month == tb.month &&
      ta.day == tb.day &&
      ta.hour == tb.hour &&
      ta.minute == tb.minute;
}

List<TailEdge> _edgesForPrimary(bool hasPreviousLinked, bool hasNextLinked) {
  if (!hasPreviousLinked && !hasNextLinked) return const [TailEdge.bottomEnd];
  if (hasPreviousLinked && hasNextLinked) {
    return const [TailEdge.topEnd, TailEdge.bottomEnd];
  }
  if (!hasPreviousLinked && hasNextLinked) return const [TailEdge.bottomEnd];
  return const [TailEdge.topEnd];
}

List<TailEdge> _edgesForSecondary(
    bool hasPreviousLinked, bool hasNextLinked) {
  if (!hasPreviousLinked && !hasNextLinked) {
    return const [TailEdge.bottomStart];
  }
  if (hasPreviousLinked && hasNextLinked) {
    return const [TailEdge.topStart, TailEdge.bottomStart];
  }
  if (!hasPreviousLinked && hasNextLinked) {
    return const [TailEdge.bottomStart];
  }
  return const [TailEdge.topStart];
}

/// Computes [QaulChatBubbleDisplayItem] for each message: edges (start/middle/end),
/// [showTimestamp] only on last of a linked group, and [marginTop] (4px linked,
/// 12px separated).
List<QaulChatBubbleDisplayItem> computeChatBubbleDisplayItems(
  List<QaulChatBubbleMessage> messages,
) {
  if (messages.isEmpty) return [];
  final result = <QaulChatBubbleDisplayItem>[];
  for (var i = 0; i < messages.length; i++) {
    final prev = i > 0 ? messages[i - 1] : null;
    final curr = messages[i];
    final next = i < messages.length - 1 ? messages[i + 1] : null;
    final prevLinked = prev != null && isChatBubbleLinked(prev, curr);
    final nextLinked = next != null && isChatBubbleLinked(curr, next);
    final isPrimary = curr.messageType == MessageType.primary;
    final edges = isPrimary
        ? _edgesForPrimary(prevLinked, nextLinked)
        : _edgesForSecondary(prevLinked, nextLinked);
    final showTimestamp = !nextLinked;
    final marginTop = i == 0
        ? 0.0
        : (prevLinked ? kChatBubbleLinkedGap : kChatBubbleSeparatedGap);
    result.add(QaulChatBubbleDisplayItem(
      message: curr.copyWith(edges: edges),
      showTimestamp: showTimestamp,
      marginTop: marginTop,
    ));
  }
  return result;
}