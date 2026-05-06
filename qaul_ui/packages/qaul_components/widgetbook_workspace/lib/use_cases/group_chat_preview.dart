import 'package:flutter/material.dart';
import 'package:qaul_components/qaul_components.dart';

List<QaulChatBubbleMessage> buildGroupPreviewRawMessages(DateTime clock) {
  final today = DateTime(clock.year, clock.month, clock.day);
  final yesterday = today.subtract(const Duration(days: 1));

  return [
    QaulChatBubbleMessage(
      content: 'Hello in 16px 300 font',
      sentAt: yesterday.copyWith(hour: 16, minute: 13),
      receivedAt: yesterday.copyWith(hour: 16, minute: 13),
      status: MessageStatus.read,
      messageType: MessageType.primary,
      edges: const [],
      senderIdBase58: 'me',
    ),
    QaulChatBubbleMessage(
      content:
          'This is a longer message with no own timestamp followed by another message with timestamp',
      sentAt: yesterday.copyWith(hour: 16, minute: 13),
      receivedAt: yesterday.copyWith(hour: 16, minute: 13),
      status: MessageStatus.sent,
      messageType: MessageType.primary,
      edges: const [],
      senderIdBase58: 'me',
    ),
    QaulChatBubbleMessage(
      content: 'This one is it',
      sentAt: yesterday.copyWith(hour: 16, minute: 13),
      receivedAt: yesterday.copyWith(hour: 16, minute: 13),
      status: MessageStatus.read,
      messageType: MessageType.primary,
      edges: const [],
      senderIdBase58: 'me',
    ),
    QaulChatBubbleMessage(
      content: 'Chatpartner is answering',
      sentAt: yesterday.copyWith(hour: 18, minute: 9),
      receivedAt: yesterday.copyWith(hour: 18, minute: 9),
      status: MessageStatus.sent,
      messageType: MessageType.secondary,
      edges: const [],
      senderIdBase58: 'user-gm',
    ),
    QaulChatBubbleMessage(
      content: 'Another answer',
      sentAt: yesterday.copyWith(hour: 18, minute: 29),
      receivedAt: yesterday.copyWith(hour: 18, minute: 29),
      status: MessageStatus.sent,
      messageType: MessageType.secondary,
      edges: const [],
      senderIdBase58: 'user-g2',
    ),
    QaulChatBubbleMessage(
      content: 'Message',
      sentAt: yesterday.copyWith(hour: 19, minute: 23),
      receivedAt: yesterday.copyWith(hour: 19, minute: 23),
      status: MessageStatus.read,
      messageType: MessageType.primary,
      edges: const [],
      senderIdBase58: 'me',
    ),
    QaulChatBubbleMessage(
      content: 'Longer message from the chatpartner',
      sentAt: yesterday.copyWith(hour: 21, minute: 19),
      receivedAt: yesterday.copyWith(hour: 21, minute: 19),
      status: MessageStatus.sent,
      messageType: MessageType.secondary,
      edges: const [],
      senderIdBase58: 'user-tm',
    ),
    QaulChatBubbleMessage(
      content: 'followed by one with time',
      sentAt: yesterday.copyWith(hour: 21, minute: 19),
      receivedAt: yesterday.copyWith(hour: 21, minute: 19),
      status: MessageStatus.sent,
      messageType: MessageType.secondary,
      edges: const [],
      senderIdBase58: 'user-tm',
    ),
    QaulChatBubbleMessage(
      content: 'Message with delay',
      sentAt: yesterday.copyWith(hour: 22, minute: 14),
      receivedAt: today.copyWith(hour: 12, minute: 30),
      status: MessageStatus.read,
      messageType: MessageType.secondary,
      edges: const [],
      senderIdBase58: 'user-tm',
    ),
    QaulChatBubbleMessage(
      content: 'Written in the morning',
      sentAt: today.copyWith(hour: 8, minute: 9),
      receivedAt: today.copyWith(hour: 8, minute: 9),
      status: MessageStatus.sent,
      messageType: MessageType.secondary,
      edges: const [],
      senderIdBase58: 'user-tm',
    ),
    QaulChatBubbleMessage(
      content: 'Followed by one late night',
      sentAt: today.copyWith(hour: 23, minute: 39),
      receivedAt: today.copyWith(hour: 23, minute: 39),
      status: MessageStatus.sent,
      messageType: MessageType.secondary,
      edges: const [],
      senderIdBase58: 'user-tm',
    ),
  ];
}

const _previewSenders = <String, QaulGroupMessageSender>{
  'user-gm': QaulGroupMessageSender(
    idBase58: 'user-gm',
    name: 'Group Member',
    isConnected: true,
  ),
  'user-g2': QaulGroupMessageSender(
    idBase58: 'user-g2',
    name: 'Groupmember 2',
    isConnected: true,
  ),
  'user-tm': QaulGroupMessageSender(
    idBase58: 'user-tm',
    name: 'Third Member',
    isConnected: true,
  ),
};

Widget _groupPreviewTextRow({
  required List<QaulChatBubbleDisplayItem> items,
  required int index,
  required DateTime clock,
}) {
  final item = items[index];
  final isPrimary = item.message.messageType == MessageType.primary;
  final senderId = item.message.senderIdBase58;
  final sender =
      isPrimary ? null : (senderId == null ? null : _previewSenders[senderId]);

  return ChatMessageRenderer.renderText(
    presentation: MessagePresentation.forSequentialTextBubble(
      item: item,
      index: index,
      items: items,
      isPrimary: isPrimary,
      sender: sender,
    ),
    mode: ChatRenderMode.group,
    clock: clock,
  );
}

class GroupChatPreview extends StatelessWidget {
  const GroupChatPreview({
    super.key,
    required this.clock,
    this.padding = const EdgeInsets.fromLTRB(16, 16, 16, 24),
  });

  final DateTime clock;
  final EdgeInsets padding;

  @override
  Widget build(BuildContext context) {
    final messages = buildGroupPreviewRawMessages(clock);
    final items = computeChatBubbleDisplayItems(
      messages,
      layoutMode: ChatRenderMode.group,
    );
    final isDark = Theme.of(context).brightness == Brightness.dark;
    final background = isDark
        ? Colors.black
        : Theme.of(context).colorScheme.surfaceContainerHighest;
    final dateColor = isDark ? Colors.white70 : Colors.black54;

    return ColoredBox(
      color: background,
      child: Padding(
        padding: padding,
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            for (var i = 0; i < items.length; i++) ...[
              if (items[i].message.content == 'Written in the morning')
                Padding(
                  padding: const EdgeInsets.symmetric(vertical: 16),
                  child: Center(
                    child: Text(
                      'Saturday, April 18, 2026',
                      style: TextStyle(
                        fontSize: 12,
                        height: 1.2,
                        color: dateColor,
                      ),
                    ),
                  ),
                ),
              _groupPreviewTextRow(
                items: items,
                index: i,
                clock: clock,
              ),
            ],
          ],
        ),
      ),
    );
  }
}
