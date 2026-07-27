import 'package:flutter/material.dart';
import 'package:qaul_components/qaul_components.dart';

const chatFixtureCurrentUser = ChatUser(id: 'me', name: 'Me');
const chatFixturePeer = ChatUser(id: 'maxx', name: 'MaxX');

Widget chatFixtureAvatar({required String initials}) {
  return CircleAvatar(
    backgroundColor: const Color(0xFFD35400),
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

/// Builds a [TextChatMessage] with `receivedAt` defaulting to `sentAt`, which
/// is the common case for fixture data (no delivery delay).
TextChatMessage chatFixtureTextMessage({
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

/// The canonical 1-1 conversation shown across the chat design stories.
///
/// Ordered ascending by `sentAt` and deliberately covering the presentation
/// cases `computeChatMessagePresentation` keys on:
///
/// * `direct-1..3` share the minute 16:13 — one linked outgoing cluster where
///   only the last bubble carries a timestamp (see `directChatBubblesShareMinute`).
/// * `direct-7..8` share the minute 21:19 — the same rule for incoming bubbles.
/// * `direct-9` is delivered a day after it was sent (`receivedAt` > `sentAt`).
/// * `direct-10..13` cover read / sent / unsent status and markdown content.
///
/// Do not spread the shared-minute pairs apart: doing so silently removes the
/// bubble-linking scenario from every story that renders this list.
List<ChatMessage> buildDirectChatFixtureMessages({
  required DateTime clock,
  ChatUser currentUser = chatFixtureCurrentUser,
  ChatUser peer = chatFixturePeer,
}) {
  final today = DateTime(clock.year, clock.month, clock.day);
  final yesterday = today.subtract(const Duration(days: 1));

  return [
    chatFixtureTextMessage(
      id: 'direct-1',
      sender: currentUser,
      content: 'Hello in 16px 300 font',
      sentAt: yesterday.copyWith(hour: 16, minute: 13),
      status: MessageStatus.read,
    ),
    chatFixtureTextMessage(
      id: 'direct-2',
      sender: currentUser,
      content:
          'This is a longer message with no own timestamp followed by another message with timestamp',
      sentAt: yesterday.copyWith(hour: 16, minute: 13),
    ),
    chatFixtureTextMessage(
      id: 'direct-3',
      sender: currentUser,
      content: 'This one is it',
      sentAt: yesterday.copyWith(hour: 16, minute: 13),
      status: MessageStatus.read,
    ),
    chatFixtureTextMessage(
      id: 'direct-4',
      sender: peer,
      content: 'Chatpartner is answering',
      sentAt: yesterday.copyWith(hour: 18, minute: 9),
    ),
    chatFixtureTextMessage(
      id: 'direct-5',
      sender: peer,
      content: 'Another answer',
      sentAt: yesterday.copyWith(hour: 18, minute: 29),
    ),
    chatFixtureTextMessage(
      id: 'direct-6',
      sender: currentUser,
      content: 'Message',
      sentAt: yesterday.copyWith(hour: 19, minute: 23),
      status: MessageStatus.read,
    ),
    chatFixtureTextMessage(
      id: 'direct-7',
      sender: peer,
      content: 'Longer message from the chatpartner',
      sentAt: yesterday.copyWith(hour: 21, minute: 19),
    ),
    chatFixtureTextMessage(
      id: 'direct-8',
      sender: peer,
      content: 'followed by one with time',
      sentAt: yesterday.copyWith(hour: 21, minute: 19),
    ),
    chatFixtureTextMessage(
      id: 'direct-9',
      sender: currentUser,
      content: 'Message with delay',
      sentAt: yesterday.copyWith(hour: 22, minute: 14),
      receivedAt: today.copyWith(hour: 12, minute: 14),
      status: MessageStatus.read,
    ),
    chatFixtureTextMessage(
      id: 'direct-10',
      sender: currentUser,
      content: 'Out and delivered',
      sentAt: clock.subtract(const Duration(minutes: 12)),
      status: MessageStatus.read,
    ),
    chatFixtureTextMessage(
      id: 'direct-11',
      sender: currentUser,
      content: 'Out but not delivered yet',
      sentAt: clock.subtract(const Duration(minutes: 1)),
    ),
    chatFixtureTextMessage(
      id: 'direct-12',
      sender: peer,
      content: '**Markdown** _preview_ message for bubble spacing context',
      sentAt: clock.subtract(const Duration(seconds: 20)),
    ),
    chatFixtureTextMessage(
      id: 'direct-13',
      sender: currentUser,
      content: 'New Message not out',
      sentAt: clock,
      status: MessageStatus.notSent,
    ),
  ];
}
