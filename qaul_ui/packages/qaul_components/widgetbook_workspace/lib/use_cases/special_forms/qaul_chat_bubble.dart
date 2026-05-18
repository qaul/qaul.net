import 'package:flutter/material.dart';
import 'package:qaul_components/design_components/chat/qaul_chat_bubble.dart';
import 'package:widgetbook_annotation/widgetbook_annotation.dart' as widgetbook;

final _clock = DateTime(2026, 4, 12, 14, 30);

Widget _bubbleScaffold(Widget child) {
  return Container(
    color: Colors.black,
    padding: const EdgeInsets.all(16),
    child: Center(
      child: ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: 500),
        child: Align(
          alignment: Alignment.centerLeft,
          child: child,
        ),
      ),
    ),
  );
}

@widgetbook.UseCase(name: 'Outgoing — sent', type: QaulChatBubble)
Widget buildOutgoingSentUseCase(BuildContext context) {
  return _bubbleScaffold(
    QaulChatBubble(
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

@widgetbook.UseCase(name: 'Outgoing — read', type: QaulChatBubble)
Widget buildOutgoingReadUseCase(BuildContext context) {
  return _bubbleScaffold(
    QaulChatBubble(
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

@widgetbook.UseCase(name: 'Outgoing — not sent', type: QaulChatBubble)
Widget buildOutgoingNotSentUseCase(BuildContext context) {
  return _bubbleScaffold(
    QaulChatBubble(
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

@widgetbook.UseCase(name: 'Incoming — short', type: QaulChatBubble)
Widget buildIncomingShortUseCase(BuildContext context) {
  return _bubbleScaffold(
    QaulChatBubble(
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

@widgetbook.UseCase(name: 'Incoming — long', type: QaulChatBubble)
Widget buildIncomingLongUseCase(BuildContext context) {
  return _bubbleScaffold(
    QaulChatBubble(
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
