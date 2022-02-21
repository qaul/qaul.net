import 'dart:typed_data';

import 'package:equatable/equatable.dart';
import 'package:fast_base58/fast_base58.dart';
import 'package:flutter/foundation.dart';

import '../generated/services/chat/chat.pb.dart';

enum MessageStatus { nothing, sent, received }

@immutable
class ChatRoom extends Equatable {
  const ChatRoom._({
    required this.conversationId,
    this.lastMessageIndex,
    this.name,
    this.lastMessageTime,
    this.lastMessagePreview,
    this.messages,
    this.unreadCount = 0,
  });

  final Uint8List conversationId;
  final int? lastMessageIndex;
  final String? name;
  final DateTime? lastMessageTime;
  final int unreadCount;
  final String? lastMessagePreview;
  final List<Message>? messages;

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
class Message extends Equatable {
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
  List<Object?> get props => [senderId, messageId, content];
}
