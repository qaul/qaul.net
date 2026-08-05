import 'package:flutter/material.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:widgetbook_annotation/widgetbook_annotation.dart' as widgetbook;

import '../../../../support/widgetbook_preview.dart';

final _clock = DateTime(2026, 4, 12, 14, 30);

@widgetbook.UseCase(
  name: 'Outgoing — own message',
  type: ChatReplyBubblePreview,
  path: 'design_components/chat/bubble',
)
Widget buildOutgoingReplyToOwnMessageUseCase(BuildContext context) {
  return widgetbookChatComponentFrame(
    context,
    alignment: Alignment.centerLeft,
    child: QaulChatBubble(
      message: QaulChatBubbleMessage(
        content: 'As I said earlier...',
        sentAt: _clock.subtract(const Duration(minutes: 8)),
        receivedAt: _clock.subtract(const Duration(minutes: 8)),
        status: MessageStatus.read,
        messageType: MessageType.primary,
        edges: const [TailEdge.bottomEnd],
        senderIdBase58: 'me',
        replyPreview: const ChatReplyPreviewData(
          author: 'You',
          content: 'This is about quoting myself, which is actually possible.',
        ),
      ),
      clock: _clock,
      showTimestamp: true,
    ),
  );
}

@widgetbook.UseCase(
  name: 'Outgoing — another user',
  type: ChatReplyBubblePreview,
  path: 'design_components/chat/bubble',
)
Widget buildOutgoingReplyToAnotherUserUseCase(BuildContext context) {
  return widgetbookChatComponentFrame(
    context,
    alignment: Alignment.centerLeft,
    child: QaulChatBubble(
      message: QaulChatBubbleMessage(
        content: 'I think this is the quote',
        sentAt: _clock.subtract(const Duration(minutes: 18)),
        receivedAt: _clock.subtract(const Duration(minutes: 18)),
        status: MessageStatus.read,
        messageType: MessageType.primary,
        edges: const [TailEdge.bottomEnd],
        senderIdBase58: 'me',
        replyPreview: const ChatReplyPreviewData(
          author: 'Group member 2',
          content:
              'This is a quote sent by another user in the group, and the '
              'message is long enough to wrap.',
        ),
      ),
      clock: _clock,
      showTimestamp: true,
    ),
  );
}

@widgetbook.UseCase(
  name: 'Incoming — own message',
  type: ChatReplyBubblePreview,
  path: 'design_components/chat/bubble',
)
Widget buildIncomingReplyToOwnMessageUseCase(BuildContext context) {
  return widgetbookChatComponentFrame(
    context,
    alignment: Alignment.centerLeft,
    child: QaulChatBubble(
      message: QaulChatBubbleMessage(
        content: 'That answers my question.',
        sentAt: _clock.subtract(const Duration(minutes: 28)),
        receivedAt: _clock.subtract(const Duration(minutes: 28)),
        status: MessageStatus.sent,
        messageType: MessageType.secondary,
        edges: const [TailEdge.bottomStart],
        senderIdBase58: 'them',
        replyPreview: const ChatReplyPreviewData(
          author: 'You',
          content: 'Can this work in private chats too?',
        ),
      ),
      clock: _clock,
      showTimestamp: true,
    ),
  );
}

@widgetbook.UseCase(
  name: 'Incoming — another user',
  type: ChatReplyBubblePreview,
  path: 'design_components/chat/bubble',
)
Widget buildIncomingReplyToAnotherUserUseCase(BuildContext context) {
  return widgetbookChatComponentFrame(
    context,
    alignment: Alignment.centerLeft,
    child: QaulChatBubble(
      message: QaulChatBubbleMessage(
        content: 'Written in the morning',
        sentAt: _clock.subtract(const Duration(minutes: 44)),
        receivedAt: _clock.subtract(const Duration(minutes: 44)),
        status: MessageStatus.sent,
        messageType: MessageType.secondary,
        edges: const [TailEdge.bottomStart],
        senderIdBase58: 'them',
        replyPreview: const ChatReplyPreviewData(
          author: 'MaxX',
          content:
              'This is a quote sent by another user and the message is too '
              'long to display without truncation.',
        ),
      ),
      clock: _clock,
      showTimestamp: true,
    ),
  );
}
