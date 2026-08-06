library chat_room;

import 'dart:math' as math;

import 'package:collection/collection.dart';
import 'package:equatable/equatable.dart';
import 'package:fast_base58/fast_base58.dart';
import 'package:flutter/foundation.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:hooks_riverpod/legacy.dart';

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

  factory ChatRoom.blank({required User otherUser, required User localUser}) {
    return ChatRoom(
      conversationId: _directConversationId(localUser, otherUser),
      name: otherUser.name,
      members: [
        ChatRoomUser(
          otherUser,
          joinedAt: DateTime.utc(1970),
        ),
      ],
    );
  }

  static Uint8List _directConversationId(User localUser, User otherUser) {
    final localQ8id = localUser.id.sublist(6, 14);
    final otherQ8id = otherUser.id.sublist(6, 14);
    final first = _compareBytes(localQ8id, otherQ8id) <= 0
        ? localQ8id
        : otherQ8id;
    final second = identical(first, localQ8id) ? otherQ8id : localQ8id;
    return Uint8List.fromList([...first, ...second]);
  }

  static int _compareBytes(List<int> a, List<int> b) {
    for (var i = 0; i < a.length && i < b.length; i++) {
      final diff = a[i].compareTo(b[i]);
      if (diff != 0) return diff;
    }
    return a.length.compareTo(b.length);
  }

  factory ChatRoom.fromRpcGroupInfo(GroupInfo g, List<User> users) {
    final usersById = <String, User>{for (final u in users) u.idBase58: u};
    final members = g.members.map((m) {
      final memberId = Uint8List.fromList(m.userId);
      final user = usersById[Base58Encode(memberId)] ??
          User(name: 'Name Undefined', id: memberId);
      return ChatRoomUser.fromUser(user, m);
    }).toList();

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

  bool get isDraftDirectChat =>
      isDirectChat &&
      createdAt == null &&
      lastMessageIndex == null &&
      messages == null;

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
  List<Object?> get props => [idBase58, lastMessageIndex, messages];

  @override
  String toString() {
    var room = 'ChatRoom(';
    room += 'id: $idBase58, name: $name, isDirect: $isDirectChat';
    if (messages != null) room += ', messages: $messages';
    if (members.isNotEmpty) ', members: $members';
    return '$room)';
  }

  ChatRoom copyWith({
    Uint8List? conversationId,
    Uint8List? lastMessageSenderId,
    int? lastMessageIndex,
    String? name,
    DateTime? lastMessageTime,
    MessageContent? lastMessagePreview,
    List<Message>? messages,
    int? unreadCount,
    DateTime? createdAt,
    bool? isDirectChat,
    List<ChatRoomUser>? members,
    int? revisionNumber,
    ChatRoomStatus? status,
  }) =>
      ChatRoom(
        conversationId: conversationId ?? this.conversationId,
        lastMessageSenderId: lastMessageSenderId ?? this.lastMessageSenderId,
        lastMessageIndex: lastMessageIndex ?? this.lastMessageIndex,
        name: name ?? this.name,
        lastMessageTime: lastMessageTime ?? this.lastMessageTime,
        lastMessagePreview: lastMessagePreview ?? this.lastMessagePreview,
        messages: messages ?? this.messages,
        unreadCount: unreadCount ?? this.unreadCount,
        createdAt: createdAt ?? this.createdAt,
        isDirectChat: isDirectChat ?? this.isDirectChat,
        members: members ?? this.members,
        revisionNumber: revisionNumber ?? this.revisionNumber,
        status: status ?? this.status,
      );

  ChatRoom copyWithMessages(ChatConversationList c) {
    assert(conversationId.equals(Uint8List.fromList(c.groupId)));
    return copyWith(
      messages: c.messageList.map((e) => Message.fromChatMessage(e)).toList(),
      lastMessageIndex: c.messageList.fold<int>(0, maxIndex),
    );
  }

  int maxIndex(int curr, ChatMessage c) => math.max(curr, c.index.toInt());
}

class PaginatedChatRooms {
  PaginatedChatRooms({
    required this.rooms,
    this.pagination,
  });

  final List<ChatRoom> rooms;
  final PaginationState? pagination;
}
