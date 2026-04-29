import 'package:flutter/material.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:qaul_components/widgets/chat_message.dart';
import 'package:qaul_components/widgets/chat_room.dart';
import 'package:widgetbook_annotation/widgetbook_annotation.dart' as widgetbook;

final _previewClock = DateTime(2026, 4, 12, 14, 30);

/// Date headers in Widgetbook only: vertical space above/below so they clear
/// nearby bubble timestamps (app spacing unchanged).
class _WidgetbookPaddedDateMeta extends ChatMessage {
  const _WidgetbookPaddedDateMeta({super.key, required this.date});

  final DateTime date;

  static const _padding = EdgeInsets.symmetric(vertical: 16);

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: _padding,
      child: RoomMetaMessage.date(date: date),
    );
  }
}

List<ChatMessage> _buildPreviewMessages() {
  final clock = _previewClock;
  final today = DateTime(clock.year, clock.month, clock.day);
  final yesterday = today.subtract(const Duration(days: 1));

  return [
    _WidgetbookPaddedDateMeta(date: yesterday),
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
    _WidgetbookPaddedDateMeta(date: today),
    QaulChatBubbleMessage(
      content: 'Out and delivered',
      sentAt: clock.subtract(const Duration(minutes: 12)),
      receivedAt: clock.subtract(const Duration(minutes: 12)),
      status: MessageStatus.read,
      messageType: MessageType.primary,
      edges: const [],
    ),
    QaulChatBubbleMessage(
      content: 'Out but not delivered yet',
      sentAt: clock.subtract(const Duration(minutes: 1)),
      receivedAt: clock.subtract(const Duration(minutes: 1)),
      status: MessageStatus.sent,
      messageType: MessageType.primary,
      edges: const [],
    ),
    QaulChatBubbleMessage(
      content: 'New Message not out',
      sentAt: clock,
      receivedAt: clock,
      status: MessageStatus.notSent,
      messageType: MessageType.primary,
      edges: const [],
    ),
  ];
}

Widget _buildScaffoldedRoom({required double maxWidth}) {
  return Container(
    color: Colors.black,
    child: Center(
      child: ConstrainedBox(
        constraints: BoxConstraints(maxWidth: maxWidth),
        child: ChatRoom(
          messages: _buildPreviewMessages(),
          clock: _previewClock,
        ),
      ),
    ),
  );
}

@widgetbook.UseCase(name: 'Portrait', type: ChatRoom)
Widget buildChatRoomPortraitUseCase(BuildContext context) {
  return _buildScaffoldedRoom(maxWidth: 402);
}

@widgetbook.UseCase(name: 'Landscape', type: ChatRoom)
Widget buildChatRoomLandscapeUseCase(BuildContext context) {
  return _buildScaffoldedRoom(maxWidth: 900);
}
