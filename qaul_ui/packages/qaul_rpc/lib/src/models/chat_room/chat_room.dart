library chat_room;

import 'dart:math' as math;

import 'package:collection/collection.dart';
import 'package:equatable/equatable.dart';
import 'package:fast_base58/fast_base58.dart';
import 'package:flutter/foundation.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

import '../../../qaul_rpc.dart';
import '../../generated/services/chat/chat.pb.dart';
import '../../generated/services/group/group_rpc.pb.dart';
import '../../utils.dart';

part 'chat_room_list_notifier.dart';

part 'chat_room_user.dart';

part 'enums.dart';

part 'message/message.dart';

part 'message/message_content.dart';

part 'providers.dart';

@immutable
class ChatRoom with EquatableMixin implements Comparable {
  ChatRoom({
    required this.conversationId,
    this.lastMessageSenderId,
    this.lastMessageIndex,
    this.name,
    this.lastMessageTime,
    this.lastMessagePreview,
    this.messages,
    this.unreadCount = 0,
    this.createdAt,
    this.isDirectChat = true,
    this.members = const [],
    this.revisionNumber = 0,
    this.status = ChatRoomStatus.active,
  }) : idBase58 = Base58Encode(conversationId);

  final Uint8List conversationId;
  final Uint8List? lastMessageSenderId;
  final int? lastMessageIndex;
  final String? name;
  final DateTime? lastMessageTime;
  final int unreadCount;
  final MessageContent? lastMessagePreview;
  final List<Message>? messages;
  final DateTime? createdAt;
  final bool isDirectChat;
  final List<ChatRoomUser> members;
  final int revisionNumber;
  final ChatRoomStatus status;

  final String idBase58;

  factory ChatRoom.blank({required User otherUser}) {
    assert(otherUser.conversationId != null);
    return ChatRoom(
        conversationId: otherUser.conversationId!, name: otherUser.name);
  }

  factory ChatRoom.fromRpcGroupInfo(GroupInfo g, List<User> users) {
    final members = <ChatRoomUser>[];

    for (final user in users) {
      final m = g.members.firstWhereOrNull((m) => m.userId.equals(user.id));
      if (m == null) continue;
      members.add(ChatRoomUser.fromUser(user, m));
    }

    return ChatRoom(
      conversationId: Uint8List.fromList(g.groupId),
      name: g.groupName,
      createdAt: DateTime.fromMillisecondsSinceEpoch(g.createdAt.toInt()),
      revisionNumber: g.revision,
      isDirectChat: g.isDirectChat,
      members: members,
      unreadCount: g.unreadMessages,
      lastMessageTime:
          DateTime.fromMillisecondsSinceEpoch(g.lastMessageAt.toInt()),
      lastMessagePreview: MessageContent.fromBuffer(g.lastMessage),
      lastMessageSenderId: Uint8List.fromList(g.lastMessageSenderId),
      status: _chatRoomStatusFactory(s: g.status),
    );
  }

  bool get isGroupChatRoom => !isDirectChat;

  String? get groupAdminIdBase58 => members
      .firstWhereOrNull((m) => m.role == ChatRoomUserRole.admin)
      ?.idBase58;

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
  List<Object?> get props => [idBase58, lastMessageIndex];

  @override
  String toString() {
    var room = 'ChatRoom(';
    room += 'id: $idBase58, name: $name, isDirect: $isDirectChat';
    if (messages != null) room += ', messages: $messages';
    if (members.isNotEmpty) ', members: $members';
    return '$room)';
  }

  ChatRoom mergeWithConversationList(ChatConversationList c) {
    assert(conversationId.equals(Uint8List.fromList(c.groupId)));
    return ChatRoom(
      conversationId: conversationId,
      messages: c.messageList.map((e) => Message.fromChatMessage(e)).toList(),
      lastMessageIndex: c.messageList.fold<int>(0, maxIndex),
      name: name,
      lastMessageTime: lastMessageTime,
      unreadCount: unreadCount,
      lastMessagePreview: lastMessagePreview,
      lastMessageSenderId: lastMessageSenderId,
      createdAt: createdAt,
      isDirectChat: isDirectChat,
      members: members,
      status: status,
    );
  }

  int maxIndex(int curr, ChatMessage c) => math.max(curr, c.index.toInt());
}
