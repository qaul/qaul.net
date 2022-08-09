import 'dart:typed_data';

import 'package:collection/collection.dart';
import 'package:equatable/equatable.dart';
import 'package:fast_base58/fast_base58.dart';
import 'package:flutter/foundation.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:logging/logging.dart';

import '../../qaul_rpc.dart';
import '../generated/services/chat/chat.pb.dart';
import '../generated/services/group/group_rpc.pb.dart';

enum ChatRoomUserRole { normal, admin, unknown }

enum InvitationState { sent, received, accepted, unknown }

enum MessageStatus { nothing, sent, received }

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
    this.isGroupMessage = false,
    this.createdAt,
    this.members = const [],
  });

  /// The ID of the other user
  final Uint8List conversationId;
  final Uint8List? lastMessageSenderId;
  final int? lastMessageIndex;
  final String? name;
  final DateTime? lastMessageTime;
  final int unreadCount;
  final MessageContent? lastMessagePreview;
  final List<Message>? messages;
  final bool isGroupMessage;
  final DateTime? createdAt;
  final List<ChatRoomUser> members;

  factory ChatRoom.blank({required User user, required User otherUser}) {
    return ChatRoom._(conversationId: otherUser.id, name: otherUser.name);
  }

  factory ChatRoom.fromOverview(ChatOverview overview) {
    return ChatRoom._(
      conversationId: Uint8List.fromList(overview.conversationId),
      name: overview.name,
      lastMessageSenderId: Uint8List.fromList(overview.lastMessageSenderId),
      lastMessageIndex: overview.lastMessageIndex,
      lastMessageTime: DateTime.fromMillisecondsSinceEpoch(
        overview.lastMessageAt.toInt(),
      ),
      unreadCount: overview.unread,
      lastMessagePreview: MessageContent.fromBuffer(overview.content),
    );
  }

  factory ChatRoom.fromConversationList(ChatConversationList conversationList) {
    return ChatRoom._(
      conversationId: Uint8List.fromList(conversationList.conversationId),
      messages: conversationList.messageList
          .map((e) => Message.fromChatMessage(e))
          .toList(),
    );
  }

  factory ChatRoom.fromChatGroupList(ChatGroupList groupList) {
    return ChatRoom._(
      conversationId: Uint8List.fromList(groupList.groupId),
      messages:
          groupList.messageList.map((e) => Message.fromChatMessage(e)).toList(),
      isGroupMessage: true,
    );
  }

  factory ChatRoom.fromGroupInfo(GroupInfo g, List<User> users) {
    final members = <ChatRoomUser>[];

    for (final user in users) {
      final m = g.members.firstWhereOrNull((m) => m.userId.equals(user.id));
      if (m == null) continue;
      members.add(ChatRoomUser.fromUser(user, m));
    }

    return ChatRoom._(
      conversationId: g.id,
      name: g.groupName,
      createdAt: g.createdAt,
      members: members,
    );
  }

  String get idBase58 => Base58Encode(conversationId);

  bool get isGroupChatRoom => isGroupMessage || members.isNotEmpty;

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
          idBase58: u.idBase58,
          key: u.key,
          keyType: u.keyType,
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
      role: m.role == 0
          ? ChatRoomUserRole.normal
          : m.role == 255
              ? ChatRoomUserRole.admin
              : ChatRoomUserRole.unknown,
      invitationState: m.state == 1
          ? InvitationState.sent
          : m.state == 2
              ? InvitationState.received
              : m.state == 3
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
    this.status = MessageStatus.nothing,
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
      content: MessageContent.fromBuffer(m.content),
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

@immutable
class GroupInfo extends Equatable {
  final Uint8List id;
  final String groupName;
  final DateTime createdAt;
  final List<GroupMember> members;

  const GroupInfo({
    required this.id,
    required this.groupName,
    required this.createdAt,
    required this.members,
  });

  @override
  List<Object?> get props => [groupName, createdAt];
}

abstract class MessageContent extends Equatable {
  const MessageContent();

  factory MessageContent.fromBuffer(List<int> buffer) {
    final content = ChatMessageContent.fromBuffer(buffer);
    switch (content.whichContent()) {
      case ChatMessageContent_Content.chatContent:
        return TextMessageContent(content.ensureChatContent().content);
      case ChatMessageContent_Content.groupInviteContent:
        final invite = content.ensureGroupInviteContent();
        return GroupInviteContent(
          groupId: Uint8List.fromList(invite.groupId),
          groupName: invite.groupName,
          createdAt:
              DateTime.fromMillisecondsSinceEpoch(invite.createdAt.toInt()),
          numOfMembers: invite.memberCount,
          adminId: Uint8List.fromList(invite.adminId),
        );
      case ChatMessageContent_Content.fileContent:
        final fileContent = content.ensureFileContent();
        return FileShareContent(
          historyIndex: fileContent.historyIndex.toInt(),
          fileId: fileContent.fileId.toStringUnsigned(),
          fileName: fileContent.fileName,
          size: fileContent.fileSize,
          description: fileContent.fileDescr,
        );
      case ChatMessageContent_Content.groupInviteReplyContent:
      case ChatMessageContent_Content.notSet:
      default:
        Logger.root.warning(
            '(_messageContentFactory) Unmapped content type: ${content.whichContent()}');
    }
    throw UnimplementedError('');
  }
}

class TextMessageContent extends MessageContent {
  const TextMessageContent(this.content);

  final String content;

  @override
  List<Object?> get props => [content];
}

class GroupInviteContent extends MessageContent {
  const GroupInviteContent({
    required this.groupId,
    required this.groupName,
    required this.createdAt,
    required this.numOfMembers,
    required this.adminId,
  });

  final Uint8List groupId;
  final String groupName;
  final DateTime createdAt;
  final int numOfMembers;
  final Uint8List adminId;

  @override
  List<Object?> get props => [groupName, createdAt];

  GroupInviteContent.fromJson(Map<String, dynamic> json)
      : groupId = Uint8List.fromList(json['groupId']),
        groupName = json['groupName'],
        createdAt = DateTime.fromMillisecondsSinceEpoch(json['createdAt']),
        numOfMembers = json['numOfMembers'],
        adminId = Uint8List.fromList(json['adminId']);

  Map<String, dynamic> toJson() {
    return {
      'groupId': groupId.toList(),
      'groupName': groupName,
      'createdAt': createdAt.millisecondsSinceEpoch,
      'numOfMembers': numOfMembers,
      'adminId': adminId.toList(),
    };
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

  String filePath(Reader read) {
    var storagePath = read(libqaulLogsStoragePath)!.replaceAll('/logs', '');
    var uuid = read(defaultUserProvider)!.idBase58;
    var ext = fileName.split('.').last;

    return '$storagePath/$uuid/files/$fileId.$ext';
  }
}
