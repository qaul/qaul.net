import 'package:flutter/material.dart';

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

enum TailEdge { topStart, topEnd, bottomStart, bottomEnd }

enum MessageStatus { notSent, sent, read }

enum MessageType { primary, secondary }

// ---------------------------------------------------------------------------
// Public constants & style
// ---------------------------------------------------------------------------

const double kChatBubbleWidthBreakpoint = 500.0;

class ChatBubbleStyle {
  static const maxBubbleWidthMobile = 272.0;
  static const maxBubbleWidthExtended = 392.0;
  static const minBubbleWidth = 32.0;

  static const horizontalPadding = 10.0;
  static const verticalPadding = 6.0;

  static const gapBetweenTextAndDate = 4.0;

  static const gapBetweenTimeAndStatusIcon = 3.0;

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
    letterSpacing: 0,
    color: Colors.white70,
  );
}

// ---------------------------------------------------------------------------
// QaulChatBubbleMessage
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Time label (pure: depends only on message + clock)
// ---------------------------------------------------------------------------

int _daysBetween(DateTime from, DateTime to) {
  final fromDate = DateTime(from.year, from.month, from.day);
  final toDate = DateTime(to.year, to.month, to.day);
  return toDate.difference(fromDate).inDays;
}

/// Formats the timestamp line for a bubble. Same [message] and [clock]
/// always produce the same string (no hidden [DateTime.now]).
String formatQaulChatBubbleTime(QaulChatBubbleMessage message, DateTime clock) {
  final isPrimary = message.messageType == MessageType.primary;
  final baseTime = isPrimary ? message.sentAt : message.receivedAt;
  final diffFromClock = clock.difference(baseTime);

  String timeLabel;
  if (diffFromClock.inMinutes < 1) {
    timeLabel = 'Now';
  } else if (diffFromClock.inMinutes < 60) {
    timeLabel = '${diffFromClock.inMinutes} min';
  } else {
    final h = baseTime.hour.toString().padLeft(2, '0');
    final m = baseTime.minute.toString().padLeft(2, '0');
    timeLabel = '$h:$m';
  }

  if (message.status != MessageStatus.read) return timeLabel;

  final days = _daysBetween(message.sentAt, message.receivedAt);
  if (days < 1) return timeLabel;

  if (isPrimary) {
    final dayText = days == 1 ? '1 day' : '$days days';
    return '$timeLabel + $dayText';
  } else {
    final dayText = days == 1 ? '1 day ago' : '$days days ago';
    return '$timeLabel $dayText';
  }
}

// ---------------------------------------------------------------------------
// QaulChatBubble widget
// ---------------------------------------------------------------------------

class QaulChatBubble extends StatelessWidget {
  const QaulChatBubble({
    super.key,
    required this.message,
    required this.clock,
    this.showTimestamp = true,
  });

  final QaulChatBubbleMessage message;

  /// Reference instant for relative labels ("Now", "N min"). Pass a fixed
  /// value in tests; in the app, typically [DateTime.now] at the call site.
  final DateTime clock;

  final bool showTimestamp;

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

    final width = MediaQuery.sizeOf(context).width;
    final isMobile = width < kChatBubbleWidthBreakpoint;
    final maxBubbleWidth = isMobile
        ? ChatBubbleStyle.maxBubbleWidthMobile
        : ChatBubbleStyle.maxBubbleWidthExtended;
    final maxTextWidth =
        maxBubbleWidth - ChatBubbleStyle.horizontalPadding * 2;

    final backgroundColor = isPrimary
        ? ChatBubbleStyle.primaryColor
        : ChatBubbleStyle.secondaryColor;

    const textColor = Colors.white;

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

    final timeLabel = formatQaulChatBubbleTime(message, clock);
    final statusIcon = _buildStatusIcon(textColor);

    return Align(
      alignment: isPrimary ? Alignment.centerRight : Alignment.centerLeft,
      child: ConstrainedBox(
        constraints: BoxConstraints(
          minWidth: ChatBubbleStyle.minBubbleWidth,
          maxWidth: maxBubbleWidth,
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
            child: ConstrainedBox(
              constraints: BoxConstraints(maxWidth: maxTextWidth),
              child: LayoutBuilder(
                builder: (context, constraints) {
                  final content = message.content.trim();
                  final messageSpan = TextSpan(
                    style: ChatBubbleStyle.textStyle,
                    text: content,
                  );
                  final gap = ChatBubbleStyle.gapBetweenTextAndDate;
                  final timeLabelPainter = TextPainter(
                    text: TextSpan(
                      text: timeLabel,
                      style: ChatBubbleStyle.timeStyle,
                    ),
                    textDirection: TextDirection.ltr,
                  );
                  timeLabelPainter.layout();
                  var timeBlockWidth = timeLabelPainter.width +
                      ChatBubbleStyle.gapBetweenTimeAndStatusIcon;
                  if (isPrimary && statusIcon != null) {
                    timeBlockWidth += 14.0;
                  }
                  final maxMessageWidth =
                      (constraints.maxWidth - gap - timeBlockWidth)
                          .clamp(0.0, double.infinity);

                  final painter = TextPainter(
                    text: messageSpan,
                    textDirection: TextDirection.ltr,
                  );
                  painter.layout(maxWidth: maxMessageWidth);
                  final lineHeight = ChatBubbleStyle.textStyle.fontSize! *
                      (ChatBubbleStyle.textStyle.height ?? 1.2);
                  final fitsOnOneLine = painter.height <= lineHeight * 1.1;

                  final timeRow = Row(
                    mainAxisSize: MainAxisSize.min,
                    mainAxisAlignment: MainAxisAlignment.end,
                    children: [
                      Text(timeLabel, style: ChatBubbleStyle.timeStyle),
                      const SizedBox(
                          width:
                              ChatBubbleStyle.gapBetweenTimeAndStatusIcon),
                      if (isPrimary && statusIcon != null) statusIcon,
                    ],
                  );

                  if (!showTimestamp) {
                    return RichText(
                      textAlign: TextAlign.left,
                      textWidthBasis: TextWidthBasis.longestLine,
                      text: messageSpan,
                    );
                  }

                  if (fitsOnOneLine) {
                    return Row(
                      crossAxisAlignment: CrossAxisAlignment.end,
                      mainAxisSize: MainAxisSize.min,
                      children: [
                        Flexible(
                          child: RichText(
                            textAlign: TextAlign.left,
                            textWidthBasis: TextWidthBasis.longestLine,
                            text: messageSpan,
                          ),
                        ),
                        SizedBox(width: gap),
                        timeRow,
                      ],
                    );
                  }

                  return Column(
                    mainAxisSize: MainAxisSize.min,
                    crossAxisAlignment: isPrimary
                        ? CrossAxisAlignment.end
                        : CrossAxisAlignment.start,
                    children: [
                      RichText(
                        textAlign: TextAlign.left,
                        textWidthBasis: TextWidthBasis.longestLine,
                        text: messageSpan,
                      ),
                      Padding(
                        padding: EdgeInsets.only(top: gap),
                        child: timeRow,
                      ),
                    ],
                  );
                },
              ),
            ),
          ),
        ),
      ),
    );
  }
}

// ---------------------------------------------------------------------------
// Display item & layout constants
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Public helpers
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Private helpers
// ---------------------------------------------------------------------------

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
