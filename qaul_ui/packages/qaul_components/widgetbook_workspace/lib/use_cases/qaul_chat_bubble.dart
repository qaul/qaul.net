import 'package:flutter/material.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:widgetbook_annotation/widgetbook_annotation.dart' as widgetbook;

@widgetbook.UseCase(name: 'Conversation preview', type: QaulChatBubble)
Widget buildChatBubbleConversationUseCase(BuildContext context) {
  const dateLabels = ['Friday, April 17, 2026 ', 'Saturday, April 18, 2026 '];

  final now = DateTime.now();
  final today = DateTime(now.year, now.month, now.day);
  final yesterday = today.subtract(const Duration(days: 1));

  final rawMessages = [
    QaulChatBubbleMessage(
      content: 'Hello in 16px 300 font',
      sentAt: yesterday.copyWith(hour: 16, minute: 13),
      receivedAt: yesterday.copyWith(hour: 16, minute: 13),
      status: MessageStatus.read,
      messageType: MessageType.primary,
      edges: const [],
    ),
    QaulChatBubbleMessage(
      content:
          'This is a longer message with no own timestamp followed by another message with timestamp',
      sentAt: yesterday.copyWith(hour: 16, minute: 13),
      receivedAt: yesterday.copyWith(hour: 16, minute: 13),
      status: MessageStatus.sent,
      messageType: MessageType.primary,
      edges: const [],
    ),
    QaulChatBubbleMessage(
      content: 'This one is it',
      sentAt: yesterday.copyWith(hour: 16, minute: 13),
      receivedAt: yesterday.copyWith(hour: 16, minute: 13),
      status: MessageStatus.read,
      messageType: MessageType.primary,
      edges: const [],
    ),
    QaulChatBubbleMessage(
      content: 'Chatpartner is answering',
      sentAt: yesterday.copyWith(hour: 18, minute: 9),
      receivedAt: yesterday.copyWith(hour: 18, minute: 9),
      status: MessageStatus.sent,
      messageType: MessageType.secondary,
      edges: const [],
    ),
    QaulChatBubbleMessage(
      content: 'Another answer',
      sentAt: yesterday.copyWith(hour: 18, minute: 29),
      receivedAt: yesterday.copyWith(hour: 18, minute: 29),
      status: MessageStatus.sent,
      messageType: MessageType.secondary,
      edges: const [],
    ),
    QaulChatBubbleMessage(
      content: 'Message',
      sentAt: yesterday.copyWith(hour: 19, minute: 23),
      receivedAt: yesterday.copyWith(hour: 19, minute: 23),
      status: MessageStatus.read,
      messageType: MessageType.primary,
      edges: const [],
    ),
    QaulChatBubbleMessage(
      content: 'Longer message from the chatpartner',
      sentAt: yesterday.copyWith(hour: 21, minute: 19),
      receivedAt: yesterday.copyWith(hour: 21, minute: 19),
      status: MessageStatus.sent,
      messageType: MessageType.secondary,
      edges: const [],
    ),
    QaulChatBubbleMessage(
      content: 'followed by one with time',
      sentAt: yesterday.copyWith(hour: 21, minute: 19),
      receivedAt: yesterday.copyWith(hour: 21, minute: 19),
      status: MessageStatus.sent,
      messageType: MessageType.secondary,
      edges: const [],
    ),
    QaulChatBubbleMessage(
      content: 'Message with delay',
      sentAt: today
          .subtract(const Duration(days: 4))
          .copyWith(hour: 12, minute: 14),
      receivedAt: today.copyWith(hour: 12, minute: 30),
      status: MessageStatus.read,
      messageType: MessageType.primary,
      edges: const [],
    ),
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

  var dateLabelIndex = 0;
  final children = <Widget>[
    const SizedBox(height: 16),
  ];

  for (final item in items) {
    bool addedSeparator = false;
    if (dateLabelIndex == 0) {
      children.add(
        Padding(
          padding: const EdgeInsets.only(top: 16, bottom: 16.5),
          child: Center(
            child: Text(
              dateLabels[dateLabelIndex++],
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
    } else if (item.message.content == 'Out and delivered' &&
        dateLabelIndex == 1) {
      children.add(
        Padding(
          padding: const EdgeInsets.only(top: 16, bottom: 16.5),
          child: Center(
            child: Text(
              dateLabels[dateLabelIndex++],
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
    child: Center(
      child: ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: 500),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: children,
        ),
      ),
    ),
  );
}
