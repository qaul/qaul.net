import 'dart:typed_data';

import 'package:collection/collection.dart';
import 'package:equatable/equatable.dart';
import 'package:fast_base58/fast_base58.dart';
import 'package:flutter/foundation.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

import '../../qaul_rpc.dart';
import '../generated/services/chat/chat.pb.dart';
import '../generated/services/filesharing/filesharing_net.pb.dart';
import '../generated/services/group/group_rpc.pb.dart';

enum ChatRoomUserRole { normal, admin, unknown }

enum InvitationState { sent, received, accepted, unknown }

enum MessageStatus { sending, sent, received, receivedByAll }

@immutable
class ChatRoom with EquatableMixin implements Comparable {
  const ChatRoom._({
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
  });

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

  factory ChatRoom.blank({required User otherUser}) {
    assert(otherUser.conversationId != null);
    return ChatRoom._(
        conversationId: otherUser.conversationId!, name: otherUser.name);
  }

  factory ChatRoom.fromRpcGroupInfo(GroupInfo g, List<User> users) {
    final members = <ChatRoomUser>[];

    for (final user in users) {
      final m = g.members.firstWhereOrNull((m) => m.userId.equals(user.id));
      if (m == null) continue;
      members.add(ChatRoomUser.fromUser(user, m));
    }

    return ChatRoom._(
      conversationId: Uint8List.fromList(g.groupId),
      name: g.groupName,
      createdAt: DateTime.fromMillisecondsSinceEpoch(g.createdAt.toInt()),
      revisionNumber: g.revision,
      isDirectChat: g.isDirectChat,
      members: members,
      unreadCount: g.unreadMessages,
      lastMessageTime: DateTime.fromMillisecondsSinceEpoch(g.lastMessageAt.toInt()),
      lastMessagePreview: MessageContent.fromBuffer(g.lastMessage),
      lastMessageSenderId: Uint8List.fromList(g.lastMessageSenderId),
    );
  }

  String get idBase58 => Base58Encode(conversationId);

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
  List<Object?> get props => [idBase58];

  @override
  String toString() {
    var room = 'ChatRoom(';
    room += 'id: $idBase58, name: $name, isDirect: $isDirectChat';
    if (messages != null) room += ', messages: $messages';
    if (members.isNotEmpty) ', members: $members';
    return '$room)';
  }

  ChatRoom copyWith({
    int? lastMessageIndex,
    String? name,
    DateTime? lastMessageTime,
    int? unreadCount,
    MessageContent? lastMessagePreview,
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

  ChatRoom mergeWithConversationList(ChatConversationList c) {
    assert(conversationId.equals(Uint8List.fromList(c.groupId)));
    return ChatRoom._(
      conversationId: conversationId,
      messages: c.messageList.map((e) => Message.fromChatMessage(e)).toList(),
      lastMessageIndex: lastMessageIndex,
      name: name,
      lastMessageTime: lastMessageTime,
      unreadCount: unreadCount,
      lastMessagePreview: lastMessagePreview,
      lastMessageSenderId: lastMessageSenderId,
      createdAt: createdAt,
      isDirectChat: isDirectChat,
      members: members,
    );
  }
}

@immutable
class ChatRoomUser extends User {
  ChatRoomUser._(
    User u, {
    required this.joinedAt,
    this.roomId,
    this.role = ChatRoomUserRole.unknown,
    this.invitationState = InvitationState.unknown,
  }) : super(
          name: u.name,
          id: u.id,
          conversationId: u.conversationId,
          keyBase58: u.keyBase58,
          availableTypes: u.availableTypes,
          isBlocked: u.isBlocked,
          isVerified: u.isVerified,
          status: u.status,
        );

  final Uint8List? roomId;
  final ChatRoomUserRole role;
  final DateTime joinedAt;
  final InvitationState invitationState;

  factory ChatRoomUser.fromUser(User u, GroupMember m, {Uint8List? roomId}) {
    return ChatRoomUser._(
      u,
      roomId: Uint8List.fromList(m.userId),
      joinedAt: DateTime.fromMillisecondsSinceEpoch(m.joinedAt.toInt()),
      role: m.role == GroupMemberRole.User
          ? ChatRoomUserRole.normal
          : m.role == GroupMemberRole.Admin
              ? ChatRoomUserRole.admin
              : ChatRoomUserRole.unknown,
      invitationState: m.state == GroupMemberState.Invited
          ? InvitationState.sent
          : m.state == GroupMemberState.Activated
              ? InvitationState.accepted
              : InvitationState.unknown,
    );
  }

  @override
  List<Object?> get props => [name, idBase58, role, roomId];
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
    this.status = MessageStatus.sending,
  });

  final Uint8List senderId;
  final Uint8List messageId;
  final int index;
  final MessageStatus status;
  final DateTime sentAt;
  final DateTime receivedAt;
  final MessageContent content;

  String get messageIdBase58 => Base58Encode(messageId);

  factory Message.fromChatMessage(ChatMessage m) {
    return Message(
      senderId: Uint8List.fromList(m.senderId),
      messageId: Uint8List.fromList(m.messageId),
      content: MessageContent.fromBuffer(
          m.content),
      index: m.index.toInt(),
      status: MessageStatus.values[m.status.value],
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

abstract class MessageContent extends Equatable {
  const MessageContent();

  factory MessageContent.fromBuffer(List<int> buffer) {
    final content = ChatContentMessage.fromBuffer(buffer);
    switch (content.whichMessage()) {
      case ChatContentMessage_Message.chatContent:
        return TextMessageContent(String.fromCharCodes(buffer));
      case ChatContentMessage_Message.fileContent:
        final file = FileSharingContainer.fromBuffer(buffer).fileInfo;
        return FileShareContent(
          historyIndex: file.startIndex,
          fileId: file.fileId.toStringUnsigned(),
          fileName: file.fileName,
          size: file.fileSize,
          description: file.fileDescr,
        );
      case ChatContentMessage_Message.groupEvent:
        final event = GroupEvent.fromBuffer(buffer);
        return GroupEventContent(
          userId: Uint8List.fromList(event.userId),
          type: _groupEventContentTypeFactory(t: event.eventType),
        );
      case ChatContentMessage_Message.notSet:
        print('here');
        return NoneMessageContent();
    }
  }
}

class NoneMessageContent extends MessageContent {
  @override
  List<Object?> get props => [];
}

class TextMessageContent extends MessageContent {
  const TextMessageContent(this.content);

  final String content;

  @override
  List<Object?> get props => [content];
}

enum GroupEventContentType { none, invited, joined, left, closed, removed }

GroupEventContentType _groupEventContentTypeFactory({
  required GroupEventType t,
}) {
  switch (t) {
    case GroupEventType.CLOSED:
      return GroupEventContentType.closed;
    case GroupEventType.DEFAULT:
      return GroupEventContentType.none;
    case GroupEventType.INVITED:
      return GroupEventContentType.invited;
    case GroupEventType.JOINED:
      return GroupEventContentType.joined;
    case GroupEventType.LEFT:
      return GroupEventContentType.left;
    case GroupEventType.REMOVED:
      return GroupEventContentType.removed;
  }
  throw UnimplementedError(
      '(_groupEventContentTypeFactory) unmpaped event type: $t');
}

class GroupEventContent extends MessageContent {
  const GroupEventContent({
    required this.userId,
    required this.type,
  });

  final Uint8List userId;
  final GroupEventContentType type;

  String get userIdBase58 => Base58Encode(userId);

  @override
  List<Object?> get props => [userIdBase58, type];

  GroupEventContent.fromJson(Map<String, dynamic> json)
      : userId = Uint8List.fromList(json['userId']),
        type = _typeFromString(json['type']);

  Map<String, dynamic> toJson() {
    return {'userId': userId.toList(), 'type': type.toString()};
  }

  static GroupEventContentType _typeFromString(String s) {
    for (var element in GroupEventContentType.values) {
      if (element.toString() == s) return element;
    }
    throw ArgumentError.value(s, 'GroupEventType', 'unable to deserialize');
  }
}

class FileShareContent extends MessageContent {
  const FileShareContent({
    required this.historyIndex,
    required this.fileId,
    required this.fileName,
    required this.size,
    required this.description,
  });

  final int historyIndex;
  final String fileId;
  final String fileName;
  final int size;
  final String description;

  @override
  List<Object?> get props => [fileId, fileName];

  String get extension => fileName.split('.').last;

  String filePath(Reader read) {
    var storagePath = read(libqaulLogsStoragePath)!.replaceAll('/logs', '');
    var uuid = read(defaultUserProvider)!.idBase58;

    return '$storagePath/$uuid/files/$fileId.$extension';
  }
}
