import 'package:flutter/material.dart';
import 'package:widgetbook_annotation/widgetbook_annotation.dart' as widgetbook;

import 'qaul_components.dart';
import 'widgets/qaul_chat_bubble.dart';

String _labelForDate(DateTime date, int index, List<String> labels) {
  if (index < labels.length) {
    return labels[index];
  }
  return '${date.day}';
}

@widgetbook.UseCase(name: 'Conversation preview', type: QaulChatBubble)
Widget buildChatBubbleConversationUseCase(BuildContext context) {
  final now = DateTime.now();
  final yesterday = now.subtract(const Duration(days: 1));

  final friday17 = yesterday.subtract(const Duration(days: 2)).copyWith(
    hour: 16,
    minute: 13,
  );
  final muchEarlier = yesterday.subtract(const Duration(hours: 3));

  const dateLabels = ['Friday, April 17, 2026 ', 'Saturday, April 18, 2026 '];

  final rawMessages = [
    QaulChatBubbleMessage(
      content: 'Hello in 16px 300 font',
      sentAt: friday17,
      receivedAt: friday17,
      status: MessageStatus.read,
      messageType: MessageType.primary,
      edges: const [],
    ),
    QaulChatBubbleMessage(
      content:
          'This is a longer message with no own timestamp followed by another message with timestamp',
      sentAt: friday17,
      receivedAt: friday17,
      status: MessageStatus.sent,
      messageType: MessageType.primary,
      edges: const [],
    ),
    QaulChatBubbleMessage(
      content: 'This one is it',
      sentAt: friday17,
      receivedAt: friday17,
      status: MessageStatus.read,
      messageType: MessageType.primary,
      edges: const [],
    ),
    QaulChatBubbleMessage(
      content: 'Chatpartner is answering',
      sentAt: muchEarlier,
      receivedAt: muchEarlier,
      status: MessageStatus.sent,
      messageType: MessageType.secondary,
      edges: const [],
    ),
    QaulChatBubbleMessage(
      content: 'Another answer',
      sentAt: muchEarlier.add(const Duration(minutes: 20)),
      receivedAt: muchEarlier.add(const Duration(minutes: 20)),
      status: MessageStatus.sent,
      messageType: MessageType.secondary,
      edges: const [],
    ),
    QaulChatBubbleMessage(
      content: 'Message',
      sentAt: DateTime(
        yesterday.year,
        yesterday.month,
        yesterday.day,
        19,
        23,
      ),
      receivedAt: DateTime(
        yesterday.year,
        yesterday.month,
        yesterday.day,
        19,
        23,
      ),
      status: MessageStatus.read,
      messageType: MessageType.primary,
      edges: const [],
    ),
    QaulChatBubbleMessage(
      content: 'Message with delay',
      sentAt: DateTime(
        yesterday.year,
        yesterday.month,
        yesterday.day,
        12,
        14,
      ),
      receivedAt: DateTime(
        yesterday.year,
        yesterday.month,
        yesterday.day,
        12,
        14,
      ),
      status: MessageStatus.sent,
      messageType: MessageType.primary,
      edges: const [],
    ),
    QaulChatBubbleMessage(
      content: 'Longer message from the chatpartner',
      sentAt: DateTime(
        yesterday.year,
        yesterday.month,
        yesterday.day,
        21,
        19,
      ),
      receivedAt: DateTime(
        yesterday.year,
        yesterday.month,
        yesterday.day,
        21,
        19,
      ),
      status: MessageStatus.sent,
      messageType: MessageType.secondary,
      edges: const [],
    ),
    QaulChatBubbleMessage(
      content: 'followed by one with time',
      sentAt: DateTime(
        yesterday.year,
        yesterday.month,
        yesterday.day,
        21,
        19,
      ),
      receivedAt: DateTime(
        yesterday.year,
        yesterday.month,
        yesterday.day,
        21,
        19,
      ),
      status: MessageStatus.sent,
      messageType: MessageType.secondary,
      edges: const [],
    ),
    // 3 novas mensagens: 12 min (read), 1 min (sent), Now (notSent)
    QaulChatBubbleMessage(
      content: 'Out and delivered',
      sentAt: now.subtract(const Duration(minutes: 12)),
      receivedAt: now.subtract(const Duration(minutes: 12)),
      status: MessageStatus.read,
      messageType: MessageType.primary,
      edges: const [],
    ),
    QaulChatBubbleMessage(
      content: 'Out but not delivered yet',
      sentAt: now.subtract(const Duration(minutes: 1)),
      receivedAt: now.subtract(const Duration(minutes: 1)),
      status: MessageStatus.sent,
      messageType: MessageType.primary,
      edges: const [],
    ),
    QaulChatBubbleMessage(
      content: 'New Message not out',
      sentAt: now,
      receivedAt: now,
      status: MessageStatus.notSent,
      messageType: MessageType.primary,
      edges: const [],
    ),
  ];

  final items = computeChatBubbleDisplayItems(rawMessages);

  DateTime? previousDate;
  var dateLabelIndex = 0;
  final children = <Widget>[
    const SizedBox(height: 16),
  ];

  for (final item in items) {
    final messageDate = DateTime(
      item.message.sentAt.year,
      item.message.sentAt.month,
      item.message.sentAt.day,
    );
    bool addedSeparator = false;
    if (previousDate != null && previousDate != messageDate) {
      children.add(
        Padding(
          padding: const EdgeInsets.only(top: 16, bottom: 16.5),
          child: Center(
            child: Text(
              _labelForDate(messageDate, dateLabelIndex++, dateLabels),
              style: TextStyle(
                fontSize: 12,
                height: 1.2,
                color: Colors.white.withValues(alpha: 0.7),
              ),
            ),
          ),
        ),
      );
      addedSeparator = true;
    } else if (previousDate == null) {
      children.add(
        Padding(
          padding: const EdgeInsets.only(bottom: 16.5),
          child: Center(
            child: Text(
              _labelForDate(messageDate, dateLabelIndex++, dateLabels),
              style: TextStyle(
                fontSize: 12,
                height: 1.2,
                color: Colors.white.withValues(alpha: 0.7),
              ),
            ),
          ),
        ),
      );
      addedSeparator = true;
    }
    previousDate = messageDate;

    children.add(
      Padding(
        padding: EdgeInsets.only(top: addedSeparator ? 0 : item.marginTop),
        child: QaulChatBubble(
          message: item.message,
          showTimestamp: item.showTimestamp,
        ),
      ),
    );
  }

  return Container(
    color: Colors.black,
    padding: const EdgeInsets.all(16),
    child: Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: children,
    ),
  );
}
