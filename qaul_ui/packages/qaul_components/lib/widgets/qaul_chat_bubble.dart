import 'package:flutter/material.dart';

enum TailEdge { topStart, topEnd, bottomStart, bottomEnd }

enum MessageStatus { notSent, sent, read }

enum MessageType { primary, secondary }

class ChatBubbleStyle {
  static const maxBubbleWidth = 272.0;
  static const minBubbleWidth = 32.0;
  static const maxTextWidth = 252.0;

  static const horizontalPadding = 10.0;
  static const verticalPadding = 6.0;

  static const gapBetweenTextAndDate = 4.0;

  static const radius = Radius.circular(10);

  static const primaryColor = Color(0xFF0288D1);
  static const secondaryColor = Color(0xFF424242);

  static const textStyle = TextStyle(
    fontSize: 16,
    fontWeight: FontWeight.w300,
    height: 1.2,
    letterSpacing: 0.1,
    color: Colors.white,
  );

  static const timeStyle = TextStyle(
    fontSize: 11,
    fontWeight: FontWeight.w400,
    height: 1.2,
    letterSpacing: 0.1,
    color: Colors.white70,
  );
}

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
  final bool showTimestamp;

  String _formatTime(BuildContext context) {
    final diff = DateTime.now().difference(message.sentAt);

    if (diff.inMinutes < 1) return 'Now';
    if (diff.inMinutes < 60) return '${diff.inMinutes} min';

    return TimeOfDay.fromDateTime(message.sentAt).format(context);
  }

  Widget? _buildStatusIcon(Color textColor) {
    switch (message.status) {
      case MessageStatus.notSent:
        return null;

      case MessageStatus.sent:
        return Icon(
          Icons.check,
          size: 14,
          color: textColor.withValues(alpha: 0.8),
        );

      case MessageStatus.read:
        return Icon(
          Icons.done_all,
          size: 14,
          color: textColor.withValues(alpha: 0.9),
        );
    }
  }

  @override
  Widget build(BuildContext context) {
    final isPrimary = message.messageType == MessageType.primary;

    final backgroundColor = isPrimary
        ? ChatBubbleStyle.primaryColor
        : ChatBubbleStyle.secondaryColor;

    final textColor = Colors.white;

    final edges = message.edges.toSet();

    final borderRadius = BorderRadiusDirectional.only(
      topStart: edges.contains(TailEdge.topStart)
          ? Radius.zero
          : ChatBubbleStyle.radius,
      topEnd: edges.contains(TailEdge.topEnd)
          ? Radius.zero
          : ChatBubbleStyle.radius,
      bottomStart: edges.contains(TailEdge.bottomStart)
          ? Radius.zero
          : ChatBubbleStyle.radius,
      bottomEnd: edges.contains(TailEdge.bottomEnd)
          ? Radius.zero
          : ChatBubbleStyle.radius,
    );

    final timeLabel = _formatTime(context);
    final statusIcon = _buildStatusIcon(textColor);

    return Align(
      alignment: isPrimary ? Alignment.centerRight : Alignment.centerLeft,
      child: ConstrainedBox(
        constraints: const BoxConstraints(
          minWidth: ChatBubbleStyle.minBubbleWidth,
          maxWidth: ChatBubbleStyle.maxBubbleWidth,
        ),
        child: DecoratedBox(
          decoration: BoxDecoration(
            color: backgroundColor,
            borderRadius: borderRadius,
          ),
          child: Padding(
            padding: const EdgeInsets.symmetric(
              horizontal: ChatBubbleStyle.horizontalPadding,
              vertical: ChatBubbleStyle.verticalPadding,
            ),
            child: Row(
              crossAxisAlignment: CrossAxisAlignment.end,
              mainAxisSize: MainAxisSize.min,
              children: [
                Flexible(
                  fit: FlexFit.loose,
                  child: ConstrainedBox(
                    constraints: const BoxConstraints(
                      maxWidth: ChatBubbleStyle.maxTextWidth,
                    ),
                    child: Text(
                      message.content.trim().replaceAll(RegExp(r'\s+'), ' '),
                      textAlign: TextAlign.left,
                      textWidthBasis: TextWidthBasis.longestLine,
                      style: ChatBubbleStyle.textStyle,
                    ),
                  ),
                ),
                if (showTimestamp)
                  Padding(
                    padding: const EdgeInsets.only(bottom: 1.5),
                    child: Row(
                      mainAxisSize: MainAxisSize.min,
                      crossAxisAlignment: CrossAxisAlignment.end,
                      children: [
                        const SizedBox(
                          width: ChatBubbleStyle.gapBetweenTextAndDate,
                        ),
                        Text(timeLabel, style: ChatBubbleStyle.timeStyle),
                        if (isPrimary && statusIcon != null) ...[
                          const SizedBox(width: 4),
                          statusIcon,
                        ],
                      ],
                    ),
                  ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}

const double kChatBubbleLinkedGap = 4.0;
const double kChatBubbleSeparatedGap = 12.0;

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

List<TailEdge> _edgesForSecondary(bool hasPreviousLinked, bool hasNextLinked) {
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

    result.add(
      QaulChatBubbleDisplayItem(
        message: curr.copyWith(edges: edges),
        showTimestamp: showTimestamp,
        marginTop: marginTop,
      ),
    );
  }

  return result;
}
