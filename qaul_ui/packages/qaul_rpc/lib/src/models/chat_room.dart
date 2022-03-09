import 'dart:typed_data';

import 'package:equatable/equatable.dart';
import 'package:fast_base58/fast_base58.dart';
import 'package:flutter/foundation.dart';

import '../generated/services/chat/chat.pb.dart';
import 'models.dart';

enum MessageStatus { nothing, sent, received }

@immutable
class ChatRoom with EquatableMixin implements Comparable {
  const ChatRoom._({
    required this.conversationId,
    this.lastMessageIndex,
    this.name,
    this.lastMessageTime,
    this.lastMessagePreview,
    this.messages,
    this.unreadCount = 0,
  });

  /// The ID of the other user
  final Uint8List conversationId;
  final int? lastMessageIndex;
  final String? name;
  final DateTime? lastMessageTime;
  final int unreadCount;
  final String? lastMessagePreview;
  final List<Message>? messages;

  factory ChatRoom.blank({required User user, required User otherUser}) {
    return ChatRoom._(conversationId: otherUser.id, name: otherUser.name);
  }

  factory ChatRoom.fromOverview(ChatOverview overview) {
    return ChatRoom._(
      conversationId: Uint8List.fromList(overview.conversationId),
      name: overview.name,
      lastMessageIndex: overview.lastMessageIndex,
      lastMessageTime: DateTime.fromMillisecondsSinceEpoch(
        overview.lastMessageAt.toInt(),
      ),
      unreadCount: overview.unread,
      lastMessagePreview: overview.content,
    );
  }

  factory ChatRoom.fromConversationList(ChatConversationList conversationList) {
    return ChatRoom._(
      conversationId: Uint8List.fromList(conversationList.conversationId),
      messages: conversationList.messageList.map((e) => Message.fromChatMessage(e)).toList(),
    );
  }

  String get idBase58 => Base58Encode(conversationId);

  @override
  int compareTo(dynamic other) {
    assert(
      runtimeType == other.runtimeType,
      "The sorting algorithm must not compare incomparable keys, since they don't "
      'know how to order themselves relative to each other. Comparing $this with $other',
    );
    if (other is ChatRoom) {
      if (other.lastMessageTime == null && lastMessageTime == null) return 0;
      if (other.lastMessageTime == null) return 1;
      if (lastMessageTime == null) return -1;
    }
    return (other as ChatRoom).lastMessageTime!.compareTo(lastMessageTime!);
  }

  @override
  List<Object?> get props => [conversationId];

  ChatRoom copyWith({
    int? lastMessageIndex,
    String? name,
    DateTime? lastMessageTime,
    int? unreadCount,
    String? lastMessagePreview,
    List<Message>? messages,
  }) {
    return ChatRoom._(
      conversationId: conversationId,
      lastMessageIndex: lastMessageIndex ?? this.lastMessageIndex,
      name: name ?? this.name,
      lastMessageTime: lastMessageTime ?? this.lastMessageTime,
      unreadCount: unreadCount ?? this.unreadCount,
      lastMessagePreview: lastMessagePreview ?? this.lastMessagePreview,
      messages: messages ?? this.messages,
    );
  }
}

@immutable
class Message with EquatableMixin implements Comparable<Message> {
  const Message({
    required this.senderId,
    required this.messageId,
    required this.content,
    required this.index,
    required this.sentAt,
    required this.receivedAt,
    this.status = MessageStatus.nothing,
  });

  final Uint8List senderId;
  final Uint8List messageId;
  final int index;
  final MessageStatus status;
  final DateTime sentAt;
  final DateTime receivedAt;
  final String content;

  String get messageIdBase58 => Base58Encode(messageId);

  factory Message.fromChatMessage(ChatMessage m) {
    return Message(
      senderId: Uint8List.fromList(m.senderId),
      messageId: Uint8List.fromList(m.messageId),
      content: m.content,
      index: m.index,
      status: MessageStatus.values[m.status],
      sentAt: DateTime.fromMillisecondsSinceEpoch(m.sentAt.toInt()),
      receivedAt: DateTime.fromMillisecondsSinceEpoch(m.receivedAt.toInt()),
    );
  }

  @override
  int compareTo(dynamic other) {
    assert(
      runtimeType == other.runtimeType,
      "The sorting algorithm must not compare incomparable keys, since they don't "
      'know how to order themselves relative to each other. Comparing $this with $other',
    );
    return (other as Message).sentAt.compareTo(sentAt);
  }

  @override
  List<Object?> get props => [senderId, messageId, content];
}
