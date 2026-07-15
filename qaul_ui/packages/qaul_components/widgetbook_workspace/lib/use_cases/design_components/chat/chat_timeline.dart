import 'package:flutter/material.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:widgetbook_annotation/widgetbook_annotation.dart' as widgetbook;

import '../../../support/chat_fixtures.dart';
import '../../../support/widgetbook_preview.dart';

final _previewClock = DateTime(2026, 4, 12, 14, 30);

@widgetbook.UseCase(name: 'Direct chat preview', type: ChatTimeline)
Widget buildChatRoomPreviewUseCase(BuildContext context) {
  return widgetbookFullScreenChatPreview(
    context,
    ChatTimeline.direct(
      currentUser: chatFixtureCurrentUser,
      messages: buildDirectChatFixtureMessages(
        clock: _previewClock,
        includeDelayedMessage: false,
        includeUnsentMessage: true,
      ),
      clock: _previewClock,
    ),
  );
}

final _groupPreviewClock = DateTime(2026, 4, 18, 22, 30);

List<ChatMessage> _buildGroupMessages() {
  final clock = _groupPreviewClock;
  final today = DateTime(clock.year, clock.month, clock.day);
  final yesterday = today.subtract(const Duration(days: 1));

  const me = ChatUser(id: 'me', name: 'Me');
  const groupMember = ChatUser(id: 'user-gm', name: 'Group Member');
  const groupMember2 = ChatUser(id: 'user-g2', name: 'Groupmember 2');
  const thirdMember = ChatUser(id: 'user-tm', name: 'Third Member');

  return [
    TextChatMessage(
      id: 'gmsg-1',
      sender: me,
      content: 'Hello in 16px 300 font',
      sentAt: yesterday.copyWith(hour: 16, minute: 13),
      receivedAt: yesterday.copyWith(hour: 16, minute: 13),
      status: MessageStatus.read,
    ),
    TextChatMessage(
      id: 'gmsg-2',
      sender: me,
      content:
          'This is a longer message with no own timestamp followed by another message with timestamp',
      sentAt: yesterday.copyWith(hour: 16, minute: 13),
      receivedAt: yesterday.copyWith(hour: 16, minute: 13),
      status: MessageStatus.sent,
    ),
    TextChatMessage(
      id: 'gmsg-3',
      sender: me,
      content: 'This one is it',
      sentAt: yesterday.copyWith(hour: 16, minute: 13),
      receivedAt: yesterday.copyWith(hour: 16, minute: 13),
      status: MessageStatus.read,
    ),
    TextChatMessage(
      id: 'gmsg-4',
      sender: groupMember,
      content: 'Chatpartner is answering',
      sentAt: yesterday.copyWith(hour: 18, minute: 9),
      receivedAt: yesterday.copyWith(hour: 18, minute: 9),
      status: MessageStatus.sent,
    ),
    TextChatMessage(
      id: 'gmsg-5',
      sender: groupMember2,
      content: 'Another answer',
      sentAt: yesterday.copyWith(hour: 18, minute: 29),
      receivedAt: yesterday.copyWith(hour: 18, minute: 29),
      status: MessageStatus.sent,
    ),
    TextChatMessage(
      id: 'gmsg-6',
      sender: me,
      content: 'Message',
      sentAt: yesterday.copyWith(hour: 19, minute: 23),
      receivedAt: yesterday.copyWith(hour: 19, minute: 23),
      status: MessageStatus.read,
    ),
    TextChatMessage(
      id: 'gmsg-7',
      sender: thirdMember,
      content: 'Longer message from the chatpartner',
      sentAt: yesterday.copyWith(hour: 21, minute: 19),
      receivedAt: yesterday.copyWith(hour: 21, minute: 19),
      status: MessageStatus.sent,
    ),
    TextChatMessage(
      id: 'gmsg-8',
      sender: thirdMember,
      content: 'followed by one with time',
      sentAt: yesterday.copyWith(hour: 21, minute: 19),
      receivedAt: yesterday.copyWith(hour: 21, minute: 19),
      status: MessageStatus.sent,
    ),
    TextChatMessage(
      id: 'gmsg-9',
      sender: thirdMember,
      content: 'Message with delay',
      sentAt: yesterday.copyWith(hour: 22, minute: 14),
      receivedAt: today.copyWith(hour: 12, minute: 30),
      status: MessageStatus.read,
    ),
    const MetaChatMessage(id: 'meta-1', label: 'Group Member joined the group'),
    TextChatMessage(
      id: 'gmsg-10',
      sender: thirdMember,
      content: 'Written in the morning',
      sentAt: today.copyWith(hour: 8, minute: 9),
      receivedAt: today.copyWith(hour: 8, minute: 9),
      status: MessageStatus.sent,
    ),
    TextChatMessage(
      id: 'gmsg-11',
      sender: thirdMember,
      content: 'Followed by one late night',
      sentAt: today.copyWith(hour: 23, minute: 39),
      receivedAt: today.copyWith(hour: 23, minute: 39),
      status: MessageStatus.sent,
    ),
  ];
}

@widgetbook.UseCase(name: 'Group chat preview', type: ChatTimeline)
Widget buildGroupChatUseCase(BuildContext context) {
  const me = ChatUser(id: 'me', name: 'Me');

  return widgetbookFullScreenChatPreview(
    context,
    ChatTimeline.group(
      currentUser: me,
      messages: _buildGroupMessages(),
      clock: _groupPreviewClock,
    ),
  );
}
