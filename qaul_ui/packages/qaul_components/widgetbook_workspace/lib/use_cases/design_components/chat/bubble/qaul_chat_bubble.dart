import 'package:flutter/material.dart';
import 'package:qaul_components/design_components/chat/qaul_chat_bubble.dart';
import 'package:widgetbook_annotation/widgetbook_annotation.dart' as widgetbook;

import '../../../../support/widgetbook_preview.dart';

final _clock = DateTime(2026, 4, 12, 14, 30);

@widgetbook.UseCase(
  name: 'Outgoing — sent',
  type: QaulChatBubble,
  path: 'design_components/chat/bubble',
)
Widget buildOutgoingSentUseCase(BuildContext context) {
  return widgetbookChatComponentFrame(
    context,
    alignment: Alignment.centerLeft,
    child: QaulChatBubble(
      message: QaulChatBubbleMessage(
        content: 'Out but not delivered yet',
        sentAt: _clock.subtract(const Duration(minutes: 1)),
        receivedAt: _clock.subtract(const Duration(minutes: 1)),
        status: MessageStatus.sent,
        messageType: MessageType.primary,
        edges: const [TailEdge.bottomEnd],
        senderIdBase58: 'me',
      ),
      clock: _clock,
      showTimestamp: true,
    ),
  );
}

@widgetbook.UseCase(
  name: 'Outgoing — read',
  type: QaulChatBubble,
  path: 'design_components/chat/bubble',
)
Widget buildOutgoingReadUseCase(BuildContext context) {
  return widgetbookChatComponentFrame(
    context,
    alignment: Alignment.centerLeft,
    child: QaulChatBubble(
      message: QaulChatBubbleMessage(
        content: 'Out and delivered',
        sentAt: _clock.subtract(const Duration(minutes: 12)),
        receivedAt: _clock.subtract(const Duration(minutes: 12)),
        status: MessageStatus.read,
        messageType: MessageType.primary,
        edges: const [TailEdge.bottomEnd],
        senderIdBase58: 'me',
      ),
      clock: _clock,
      showTimestamp: true,
    ),
  );
}

@widgetbook.UseCase(
  name: 'Outgoing — not sent',
  type: QaulChatBubble,
  path: 'design_components/chat/bubble',
)
Widget buildOutgoingNotSentUseCase(BuildContext context) {
  return widgetbookChatComponentFrame(
    context,
    alignment: Alignment.centerLeft,
    child: QaulChatBubble(
      message: QaulChatBubbleMessage(
        content: 'New Message not out',
        sentAt: _clock,
        receivedAt: _clock,
        status: MessageStatus.notSent,
        messageType: MessageType.primary,
        edges: const [TailEdge.bottomEnd],
        senderIdBase58: 'me',
      ),
      clock: _clock,
      showTimestamp: true,
    ),
  );
}

@widgetbook.UseCase(
  name: 'Incoming — short',
  type: QaulChatBubble,
  path: 'design_components/chat/bubble',
)
Widget buildIncomingShortUseCase(BuildContext context) {
  return widgetbookChatComponentFrame(
    context,
    alignment: Alignment.centerLeft,
    child: QaulChatBubble(
      message: QaulChatBubbleMessage(
        content: 'Hi!',
        sentAt: _clock.subtract(const Duration(minutes: 5)),
        receivedAt: _clock.subtract(const Duration(minutes: 5)),
        status: MessageStatus.sent,
        messageType: MessageType.secondary,
        edges: const [TailEdge.bottomStart],
        senderIdBase58: 'them',
      ),
      clock: _clock,
      showTimestamp: true,
    ),
  );
}

@widgetbook.UseCase(
  name: 'Incoming — long',
  type: QaulChatBubble,
  path: 'design_components/chat/bubble',
)
Widget buildIncomingLongUseCase(BuildContext context) {
  return widgetbookChatComponentFrame(
    context,
    alignment: Alignment.centerLeft,
    child: QaulChatBubble(
      message: QaulChatBubbleMessage(
        content:
            'This is a longer incoming message from the chat partner that wraps across multiple lines so the designer can validate line height, padding, and timestamp placement.',
        sentAt: _clock.subtract(const Duration(minutes: 30)),
        receivedAt: _clock.subtract(const Duration(minutes: 30)),
        status: MessageStatus.sent,
        messageType: MessageType.secondary,
        edges: const [TailEdge.bottomStart],
        senderIdBase58: 'them',
      ),
      clock: _clock,
      showTimestamp: true,
    ),
  );
}

class _LongPressSelectionPreview extends StatefulWidget {
  const _LongPressSelectionPreview();

  @override
  State<_LongPressSelectionPreview> createState() =>
      _LongPressSelectionPreviewState();
}

class _LongPressSelectionPreviewState
    extends State<_LongPressSelectionPreview> {
  var _isSelected = false;

  @override
  Widget build(BuildContext context) {
    return widgetbookChatComponentFrame(
      context,
      alignment: Alignment.centerLeft,
      child: GestureDetector(
        key: const ValueKey('widgetbook-pressable-chat-bubble'),
        behavior: HitTestBehavior.opaque,
        onLongPress: () => setState(() => _isSelected = true),
        onTap: _isSelected ? () => setState(() => _isSelected = false) : null,
        child: QaulChatBubble(
          message: QaulChatBubbleMessage(
            content: 'Another answer',
            sentAt: _clock.subtract(const Duration(minutes: 1)),
            receivedAt: _clock.subtract(const Duration(minutes: 1)),
            status: MessageStatus.sent,
            messageType: MessageType.secondary,
            edges: const [],
            senderIdBase58: 'group-member-2',
            senderDisplayName: 'Groupmember 2',
            senderDisplayNameColor: const Color(0xFFFF9800),
          ),
          clock: _clock,
          showTimestamp: true,
          isSelected: _isSelected,
        ),
      ),
    );
  }
}

@widgetbook.UseCase(
  name: 'Interactive — long press',
  type: QaulChatBubble,
  path: 'design_components/chat/bubble',
)
Widget buildInteractiveLongPressUseCase(BuildContext context) {
  return const _LongPressSelectionPreview();
}
