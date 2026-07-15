import 'package:flutter/material.dart';
import 'package:qaul_components/qaul_components.dart';

const chatFixtureCurrentUser = ChatUser(id: 'me', name: 'Me');
const chatFixturePeer = ChatUser(id: 'maxx', name: 'MaxX');

Widget chatFixtureAvatar({
  required String initials,
  Color backgroundColor = const Color(0xFFD35400),
}) {
  return CircleAvatar(
    backgroundColor: backgroundColor,
    foregroundColor: Colors.white,
    child: Text(
      initials,
      style: const TextStyle(
        fontFamily: 'Roboto',
        fontWeight: FontWeight.w400,
        fontSize: 18,
      ),
    ),
  );
}

List<ChatMessage> buildDirectChatFixtureMessages({
  required DateTime clock,
  ChatUser currentUser = chatFixtureCurrentUser,
  ChatUser peer = chatFixturePeer,
  bool includeDelayedMessage = true,
  bool includeUnsentMessage = false,
}) {
  final today = DateTime(clock.year, clock.month, clock.day);
  final yesterday = today.subtract(const Duration(days: 1));

  TextChatMessage message({
    required String id,
    required ChatUser sender,
    required String content,
    required DateTime sentAt,
    MessageStatus status = MessageStatus.sent,
    DateTime? receivedAt,
  }) {
    return TextChatMessage(
      id: id,
      sender: sender,
      content: content,
      sentAt: sentAt,
      receivedAt: receivedAt ?? sentAt,
      status: status,
    );
  }

  return [
    message(
      id: 'direct-1',
      sender: currentUser,
      content: 'Hello in 16px 300 font',
      sentAt: yesterday.copyWith(hour: 16, minute: 13),
      status: MessageStatus.read,
    ),
    message(
      id: 'direct-2',
      sender: currentUser,
      content:
          'This is a longer message with no own timestamp followed by another message with timestamp',
      sentAt: yesterday.copyWith(hour: 16, minute: 13),
    ),
    message(
      id: 'direct-3',
      sender: currentUser,
      content: 'This one is it',
      sentAt: yesterday.copyWith(hour: 16, minute: 20),
      status: MessageStatus.read,
    ),
    message(
      id: 'direct-4',
      sender: peer,
      content: 'Chatpartner is answering',
      sentAt: yesterday.copyWith(hour: 18, minute: 9),
    ),
    message(
      id: 'direct-5',
      sender: peer,
      content: 'Another answer',
      sentAt: yesterday.copyWith(hour: 18, minute: 29),
    ),
    message(
      id: 'direct-6',
      sender: currentUser,
      content: 'Message',
      sentAt: yesterday.copyWith(hour: 19, minute: 23),
      status: MessageStatus.read,
    ),
    message(
      id: 'direct-7',
      sender: peer,
      content: 'Longer message from the chatpartner',
      sentAt: yesterday.copyWith(hour: 21, minute: 19),
    ),
    message(
      id: 'direct-8',
      sender: peer,
      content: 'followed by one with time',
      sentAt: yesterday.copyWith(hour: 21, minute: 39),
    ),
    if (includeDelayedMessage)
      message(
        id: 'direct-9',
        sender: currentUser,
        content: 'Message with delay',
        sentAt: yesterday.copyWith(hour: 22, minute: 14),
        receivedAt: today.copyWith(hour: 12, minute: 14),
        status: MessageStatus.read,
      ),
    message(
      id: 'direct-10',
      sender: currentUser,
      content: 'Out and delivered',
      sentAt: clock.subtract(const Duration(minutes: 12)),
      status: MessageStatus.read,
    ),
    message(
      id: 'direct-11',
      sender: currentUser,
      content: 'Out but not delivered yet',
      sentAt: clock.subtract(const Duration(minutes: 1)),
    ),
    if (includeUnsentMessage)
      message(
        id: 'direct-12',
        sender: currentUser,
        content: 'New Message not out',
        sentAt: clock,
        status: MessageStatus.notSent,
      ),
  ];
}
