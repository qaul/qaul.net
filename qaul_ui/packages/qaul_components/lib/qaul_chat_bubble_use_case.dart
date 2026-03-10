import 'package:flutter/material.dart';
import 'package:widgetbook_annotation/widgetbook_annotation.dart' as widgetbook;

import 'qaul_components.dart';
import 'widgets/qaul_chat_bubble.dart';

@widgetbook.UseCase(name: 'Conversation preview', type: QaulChatBubble)
Widget buildChatBubbleConversationUseCase(BuildContext context) {
  final now = DateTime(2026, 4, 18, 19, 23);
  final earlier = now.subtract(const Duration(minutes: 70));
  final muchEarlier = now.subtract(const Duration(hours: 3));

  // Edges are ignored; computeChatBubbleDisplayItems sets them from sentAt + messageType.
  // Linked = same sender + same minute (sentAt). 4px between linked, 12px between separated.
  // Timestamp only on last message of each linked group.
  final rawMessages = [
    QaulChatBubbleMessage(
      content: 'Hello in 16px 300 font',
      sentAt: earlier,
      receivedAt: earlier,
      status: MessageStatus.read,
      messageType: MessageType.primary,
      edges: const [],
    ),
    QaulChatBubbleMessage(
      content:
          'This is a longer message with no own timestamp followed by another message with timestamp',
      sentAt: earlier,
      receivedAt: earlier,
      status: MessageStatus.sent,
      messageType: MessageType.primary,
      edges: const [],
    ),
    QaulChatBubbleMessage(
      content: 'This one is it',
      sentAt: earlier,
      receivedAt: earlier,
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
      sentAt: now.subtract(const Duration(minutes: 2)),
      receivedAt: now.subtract(const Duration(minutes: 2)),
      status: MessageStatus.read,
      messageType: MessageType.primary,
      edges: const [],
    ),
    QaulChatBubbleMessage(
      content: 'Longer message from the chatpartner',
      sentAt: now,
      receivedAt: now,
      status: MessageStatus.sent,
      messageType: MessageType.secondary,
      edges: const [],
    ),
    QaulChatBubbleMessage(
      content: 'followed by one with time',
      sentAt: now,
      receivedAt: now,
      status: MessageStatus.sent,
      messageType: MessageType.secondary,
      edges: const [],
    ),
  ];

  final items = computeChatBubbleDisplayItems(rawMessages);

  return Container(
    color: Colors.black,
    padding: const EdgeInsets.all(16),
    child: Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const SizedBox(height: 16),
        for (final item in items)
          Padding(
            padding: EdgeInsets.only(top: item.marginTop),
            child: QaulChatBubble(
              message: item.message,
              showTimestamp: item.showTimestamp,
            ),
          ),
      ],
    ),
  );
}
