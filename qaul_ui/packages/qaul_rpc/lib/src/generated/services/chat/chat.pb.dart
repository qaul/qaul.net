// This is a generated file - do not edit.
//
// Generated from services/chat/chat.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'chat.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'chat.pbenum.dart';

enum Chat_Message { conversationRequest, conversationList, send, notSet }

/// Chat service RPC message container
class Chat extends $pb.GeneratedMessage {
  factory Chat({
    ChatConversationRequest? conversationRequest,
    ChatConversationList? conversationList,
    ChatMessageSend? send,
  }) {
    final result = create();
    if (conversationRequest != null)
      result.conversationRequest = conversationRequest;
    if (conversationList != null) result.conversationList = conversationList;
    if (send != null) result.send = send;
    return result;
  }

  Chat._();

  factory Chat.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory Chat.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, Chat_Message> _Chat_MessageByTag = {
    3: Chat_Message.conversationRequest,
    4: Chat_Message.conversationList,
    5: Chat_Message.send,
    0: Chat_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Chat',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.chat'),
      createEmptyInstance: create)
    ..oo(0, [3, 4, 5])
    ..aOM<ChatConversationRequest>(
        3, _omitFieldNames ? '' : 'conversationRequest',
        subBuilder: ChatConversationRequest.create)
    ..aOM<ChatConversationList>(4, _omitFieldNames ? '' : 'conversationList',
        subBuilder: ChatConversationList.create)
    ..aOM<ChatMessageSend>(5, _omitFieldNames ? '' : 'send',
        subBuilder: ChatMessageSend.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Chat clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Chat copyWith(void Function(Chat) updates) =>
      super.copyWith((message) => updates(message as Chat)) as Chat;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Chat create() => Chat._();
  @$core.override
  Chat createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static Chat getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Chat>(create);
  static Chat? _defaultInstance;

  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  Chat_Message whichMessage() => _Chat_MessageByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  void clearMessage() => $_clearField($_whichOneof(0));

  /// request a specific conversation
  @$pb.TagNumber(3)
  ChatConversationRequest get conversationRequest => $_getN(0);
  @$pb.TagNumber(3)
  set conversationRequest(ChatConversationRequest value) =>
      $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasConversationRequest() => $_has(0);
  @$pb.TagNumber(3)
  void clearConversationRequest() => $_clearField(3);
  @$pb.TagNumber(3)
  ChatConversationRequest ensureConversationRequest() => $_ensure(0);

  /// list of a chat conversation
  @$pb.TagNumber(4)
  ChatConversationList get conversationList => $_getN(1);
  @$pb.TagNumber(4)
  set conversationList(ChatConversationList value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasConversationList() => $_has(1);
  @$pb.TagNumber(4)
  void clearConversationList() => $_clearField(4);
  @$pb.TagNumber(4)
  ChatConversationList ensureConversationList() => $_ensure(1);

  /// send a new chat message
  @$pb.TagNumber(5)
  ChatMessageSend get send => $_getN(2);
  @$pb.TagNumber(5)
  set send(ChatMessageSend value) => $_setField(5, value);
  @$pb.TagNumber(5)
  $core.bool hasSend() => $_has(2);
  @$pb.TagNumber(5)
  void clearSend() => $_clearField(5);
  @$pb.TagNumber(5)
  ChatMessageSend ensureSend() => $_ensure(2);
}

/// request messages of a specific chat conversation
class ChatConversationRequest extends $pb.GeneratedMessage {
  factory ChatConversationRequest({
    $core.List<$core.int>? groupId,
    $fixnum.Int64? lastIndex,
  }) {
    final result = create();
    if (groupId != null) result.groupId = groupId;
    if (lastIndex != null) result.lastIndex = lastIndex;
    return result;
  }

  ChatConversationRequest._();

  factory ChatConversationRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ChatConversationRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ChatConversationRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.chat'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'groupId', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(
        2, _omitFieldNames ? '' : 'lastIndex', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ChatConversationRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ChatConversationRequest copyWith(
          void Function(ChatConversationRequest) updates) =>
      super.copyWith((message) => updates(message as ChatConversationRequest))
          as ChatConversationRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ChatConversationRequest create() => ChatConversationRequest._();
  @$core.override
  ChatConversationRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ChatConversationRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ChatConversationRequest>(create);
  static ChatConversationRequest? _defaultInstance;

  /// group id
  @$pb.TagNumber(1)
  $core.List<$core.int> get groupId => $_getN(0);
  @$pb.TagNumber(1)
  set groupId($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => $_clearField(1);

  /// send only changes that are newer than the last received
  @$pb.TagNumber(2)
  $fixnum.Int64 get lastIndex => $_getI64(1);
  @$pb.TagNumber(2)
  set lastIndex($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasLastIndex() => $_has(1);
  @$pb.TagNumber(2)
  void clearLastIndex() => $_clearField(2);
}

/// list of chat messages of a specific conversation
class ChatConversationList extends $pb.GeneratedMessage {
  factory ChatConversationList({
    $core.List<$core.int>? groupId,
    $core.Iterable<ChatMessage>? messageList,
  }) {
    final result = create();
    if (groupId != null) result.groupId = groupId;
    if (messageList != null) result.messageList.addAll(messageList);
    return result;
  }

  ChatConversationList._();

  factory ChatConversationList.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ChatConversationList.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ChatConversationList',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.chat'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'groupId', $pb.PbFieldType.OY)
    ..pPM<ChatMessage>(2, _omitFieldNames ? '' : 'messageList',
        subBuilder: ChatMessage.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ChatConversationList clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ChatConversationList copyWith(void Function(ChatConversationList) updates) =>
      super.copyWith((message) => updates(message as ChatConversationList))
          as ChatConversationList;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ChatConversationList create() => ChatConversationList._();
  @$core.override
  ChatConversationList createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ChatConversationList getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ChatConversationList>(create);
  static ChatConversationList? _defaultInstance;

  /// group id
  @$pb.TagNumber(1)
  $core.List<$core.int> get groupId => $_getN(0);
  @$pb.TagNumber(1)
  set groupId($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => $_clearField(1);

  /// several messages
  @$pb.TagNumber(2)
  $pb.PbList<ChatMessage> get messageList => $_getList(1);
}

/// a single chat message
class ChatMessage extends $pb.GeneratedMessage {
  factory ChatMessage({
    $fixnum.Int64? index,
    $core.List<$core.int>? senderId,
    $core.List<$core.int>? messageId,
    MessageStatus? status,
    $core.List<$core.int>? groupId,
    $fixnum.Int64? sentAt,
    $fixnum.Int64? receivedAt,
    $core.List<$core.int>? content,
    $core.Iterable<MessageReceptionConfirmed>? messageReceptionConfirmed,
  }) {
    final result = create();
    if (index != null) result.index = index;
    if (senderId != null) result.senderId = senderId;
    if (messageId != null) result.messageId = messageId;
    if (status != null) result.status = status;
    if (groupId != null) result.groupId = groupId;
    if (sentAt != null) result.sentAt = sentAt;
    if (receivedAt != null) result.receivedAt = receivedAt;
    if (content != null) result.content = content;
    if (messageReceptionConfirmed != null)
      result.messageReceptionConfirmed.addAll(messageReceptionConfirmed);
    return result;
  }

  ChatMessage._();

  factory ChatMessage.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ChatMessage.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ChatMessage',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.chat'),
      createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, _omitFieldNames ? '' : 'index', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.List<$core.int>>(
        2, _omitFieldNames ? '' : 'senderId', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(
        3, _omitFieldNames ? '' : 'messageId', $pb.PbFieldType.OY)
    ..aE<MessageStatus>(4, _omitFieldNames ? '' : 'status',
        enumValues: MessageStatus.values)
    ..a<$core.List<$core.int>>(
        5, _omitFieldNames ? '' : 'groupId', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(6, _omitFieldNames ? '' : 'sentAt', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(
        7, _omitFieldNames ? '' : 'receivedAt', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.List<$core.int>>(
        8, _omitFieldNames ? '' : 'content', $pb.PbFieldType.OY)
    ..pPM<MessageReceptionConfirmed>(
        10, _omitFieldNames ? '' : 'messageReceptionConfirmed',
        subBuilder: MessageReceptionConfirmed.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ChatMessage clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ChatMessage copyWith(void Function(ChatMessage) updates) =>
      super.copyWith((message) => updates(message as ChatMessage))
          as ChatMessage;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ChatMessage create() => ChatMessage._();
  @$core.override
  ChatMessage createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ChatMessage getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ChatMessage>(create);
  static ChatMessage? _defaultInstance;

  /// index
  @$pb.TagNumber(1)
  $fixnum.Int64 get index => $_getI64(0);
  @$pb.TagNumber(1)
  set index($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasIndex() => $_has(0);
  @$pb.TagNumber(1)
  void clearIndex() => $_clearField(1);

  /// id of the sending user
  @$pb.TagNumber(2)
  $core.List<$core.int> get senderId => $_getN(1);
  @$pb.TagNumber(2)
  set senderId($core.List<$core.int> value) => $_setBytes(1, value);
  @$pb.TagNumber(2)
  $core.bool hasSenderId() => $_has(1);
  @$pb.TagNumber(2)
  void clearSenderId() => $_clearField(2);

  /// message id or member id
  @$pb.TagNumber(3)
  $core.List<$core.int> get messageId => $_getN(2);
  @$pb.TagNumber(3)
  set messageId($core.List<$core.int> value) => $_setBytes(2, value);
  @$pb.TagNumber(3)
  $core.bool hasMessageId() => $_has(2);
  @$pb.TagNumber(3)
  void clearMessageId() => $_clearField(3);

  /// message status
  @$pb.TagNumber(4)
  MessageStatus get status => $_getN(3);
  @$pb.TagNumber(4)
  set status(MessageStatus value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasStatus() => $_has(3);
  @$pb.TagNumber(4)
  void clearStatus() => $_clearField(4);

  /// group id
  @$pb.TagNumber(5)
  $core.List<$core.int> get groupId => $_getN(4);
  @$pb.TagNumber(5)
  set groupId($core.List<$core.int> value) => $_setBytes(4, value);
  @$pb.TagNumber(5)
  $core.bool hasGroupId() => $_has(4);
  @$pb.TagNumber(5)
  void clearGroupId() => $_clearField(5);

  /// time when the message was sent
  @$pb.TagNumber(6)
  $fixnum.Int64 get sentAt => $_getI64(5);
  @$pb.TagNumber(6)
  set sentAt($fixnum.Int64 value) => $_setInt64(5, value);
  @$pb.TagNumber(6)
  $core.bool hasSentAt() => $_has(5);
  @$pb.TagNumber(6)
  void clearSentAt() => $_clearField(6);

  /// time when the message was received
  @$pb.TagNumber(7)
  $fixnum.Int64 get receivedAt => $_getI64(6);
  @$pb.TagNumber(7)
  set receivedAt($fixnum.Int64 value) => $_setInt64(6, value);
  @$pb.TagNumber(7)
  $core.bool hasReceivedAt() => $_has(6);
  @$pb.TagNumber(7)
  void clearReceivedAt() => $_clearField(7);

  /// chat content message
  @$pb.TagNumber(8)
  $core.List<$core.int> get content => $_getN(7);
  @$pb.TagNumber(8)
  set content($core.List<$core.int> value) => $_setBytes(7, value);
  @$pb.TagNumber(8)
  $core.bool hasContent() => $_has(7);
  @$pb.TagNumber(8)
  void clearContent() => $_clearField(8);

  /// message reception confirmed
  ///
  /// When a user receives a message, sent by us,
  /// the user is confirming the reception of this message.
  /// We are only getting this confirmation if we are the sender of this
  /// message.
  @$pb.TagNumber(10)
  $pb.PbList<MessageReceptionConfirmed> get messageReceptionConfirmed =>
      $_getList(8);
}

/// message reception confirmed
class MessageReceptionConfirmed extends $pb.GeneratedMessage {
  factory MessageReceptionConfirmed({
    $core.List<$core.int>? userId,
    $fixnum.Int64? confirmedAt,
  }) {
    final result = create();
    if (userId != null) result.userId = userId;
    if (confirmedAt != null) result.confirmedAt = confirmedAt;
    return result;
  }

  MessageReceptionConfirmed._();

  factory MessageReceptionConfirmed.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory MessageReceptionConfirmed.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MessageReceptionConfirmed',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.chat'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'userId', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(
        2, _omitFieldNames ? '' : 'confirmedAt', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MessageReceptionConfirmed clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MessageReceptionConfirmed copyWith(
          void Function(MessageReceptionConfirmed) updates) =>
      super.copyWith((message) => updates(message as MessageReceptionConfirmed))
          as MessageReceptionConfirmed;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static MessageReceptionConfirmed create() => MessageReceptionConfirmed._();
  @$core.override
  MessageReceptionConfirmed createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static MessageReceptionConfirmed getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<MessageReceptionConfirmed>(create);
  static MessageReceptionConfirmed? _defaultInstance;

  /// user id
  @$pb.TagNumber(1)
  $core.List<$core.int> get userId => $_getN(0);
  @$pb.TagNumber(1)
  set userId($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasUserId() => $_has(0);
  @$pb.TagNumber(1)
  void clearUserId() => $_clearField(1);

  /// time of confirmation
  @$pb.TagNumber(2)
  $fixnum.Int64 get confirmedAt => $_getI64(1);
  @$pb.TagNumber(2)
  set confirmedAt($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasConfirmedAt() => $_has(1);
  @$pb.TagNumber(2)
  void clearConfirmedAt() => $_clearField(2);
}

enum ChatContentMessage_Message { chatContent, fileContent, groupEvent, notSet }

/// chat content message
class ChatContentMessage extends $pb.GeneratedMessage {
  factory ChatContentMessage({
    ChatContent? chatContent,
    FileContent? fileContent,
    GroupEvent? groupEvent,
  }) {
    final result = create();
    if (chatContent != null) result.chatContent = chatContent;
    if (fileContent != null) result.fileContent = fileContent;
    if (groupEvent != null) result.groupEvent = groupEvent;
    return result;
  }

  ChatContentMessage._();

  factory ChatContentMessage.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ChatContentMessage.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, ChatContentMessage_Message>
      _ChatContentMessage_MessageByTag = {
    1: ChatContentMessage_Message.chatContent,
    2: ChatContentMessage_Message.fileContent,
    3: ChatContentMessage_Message.groupEvent,
    0: ChatContentMessage_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ChatContentMessage',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.chat'),
      createEmptyInstance: create)
    ..oo(0, [1, 2, 3])
    ..aOM<ChatContent>(1, _omitFieldNames ? '' : 'chatContent',
        subBuilder: ChatContent.create)
    ..aOM<FileContent>(2, _omitFieldNames ? '' : 'fileContent',
        subBuilder: FileContent.create)
    ..aOM<GroupEvent>(3, _omitFieldNames ? '' : 'groupEvent',
        subBuilder: GroupEvent.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ChatContentMessage clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ChatContentMessage copyWith(void Function(ChatContentMessage) updates) =>
      super.copyWith((message) => updates(message as ChatContentMessage))
          as ChatContentMessage;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ChatContentMessage create() => ChatContentMessage._();
  @$core.override
  ChatContentMessage createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ChatContentMessage getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ChatContentMessage>(create);
  static ChatContentMessage? _defaultInstance;

  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  ChatContentMessage_Message whichMessage() =>
      _ChatContentMessage_MessageByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  void clearMessage() => $_clearField($_whichOneof(0));

  /// a chat content message
  @$pb.TagNumber(1)
  ChatContent get chatContent => $_getN(0);
  @$pb.TagNumber(1)
  set chatContent(ChatContent value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasChatContent() => $_has(0);
  @$pb.TagNumber(1)
  void clearChatContent() => $_clearField(1);
  @$pb.TagNumber(1)
  ChatContent ensureChatContent() => $_ensure(0);

  /// a file content message
  @$pb.TagNumber(2)
  FileContent get fileContent => $_getN(1);
  @$pb.TagNumber(2)
  set fileContent(FileContent value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasFileContent() => $_has(1);
  @$pb.TagNumber(2)
  void clearFileContent() => $_clearField(2);
  @$pb.TagNumber(2)
  FileContent ensureFileContent() => $_ensure(1);

  /// a group event information
  @$pb.TagNumber(3)
  GroupEvent get groupEvent => $_getN(2);
  @$pb.TagNumber(3)
  set groupEvent(GroupEvent value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasGroupEvent() => $_has(2);
  @$pb.TagNumber(3)
  void clearGroupEvent() => $_clearField(3);
  @$pb.TagNumber(3)
  GroupEvent ensureGroupEvent() => $_ensure(2);
}

/// chat content
class ChatContent extends $pb.GeneratedMessage {
  factory ChatContent({
    $core.String? text,
  }) {
    final result = create();
    if (text != null) result.text = text;
    return result;
  }

  ChatContent._();

  factory ChatContent.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ChatContent.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ChatContent',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.chat'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'text')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ChatContent clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ChatContent copyWith(void Function(ChatContent) updates) =>
      super.copyWith((message) => updates(message as ChatContent))
          as ChatContent;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ChatContent create() => ChatContent._();
  @$core.override
  ChatContent createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ChatContent getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ChatContent>(create);
  static ChatContent? _defaultInstance;

  /// message text
  @$pb.TagNumber(1)
  $core.String get text => $_getSZ(0);
  @$pb.TagNumber(1)
  set text($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasText() => $_has(0);
  @$pb.TagNumber(1)
  void clearText() => $_clearField(1);
}

/// file content
class FileContent extends $pb.GeneratedMessage {
  factory FileContent({
    $fixnum.Int64? fileId,
    $core.String? fileName,
    $core.String? fileExtension,
    $core.int? fileSize,
    $core.String? fileDescription,
  }) {
    final result = create();
    if (fileId != null) result.fileId = fileId;
    if (fileName != null) result.fileName = fileName;
    if (fileExtension != null) result.fileExtension = fileExtension;
    if (fileSize != null) result.fileSize = fileSize;
    if (fileDescription != null) result.fileDescription = fileDescription;
    return result;
  }

  FileContent._();

  factory FileContent.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory FileContent.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'FileContent',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.chat'),
      createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, _omitFieldNames ? '' : 'fileId', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOS(2, _omitFieldNames ? '' : 'fileName')
    ..aOS(3, _omitFieldNames ? '' : 'fileExtension')
    ..aI(4, _omitFieldNames ? '' : 'fileSize', fieldType: $pb.PbFieldType.OU3)
    ..aOS(5, _omitFieldNames ? '' : 'fileDescription')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  FileContent clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  FileContent copyWith(void Function(FileContent) updates) =>
      super.copyWith((message) => updates(message as FileContent))
          as FileContent;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FileContent create() => FileContent._();
  @$core.override
  FileContent createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static FileContent getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<FileContent>(create);
  static FileContent? _defaultInstance;

  /// file id
  @$pb.TagNumber(1)
  $fixnum.Int64 get fileId => $_getI64(0);
  @$pb.TagNumber(1)
  set fileId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasFileId() => $_has(0);
  @$pb.TagNumber(1)
  void clearFileId() => $_clearField(1);

  /// file name
  @$pb.TagNumber(2)
  $core.String get fileName => $_getSZ(1);
  @$pb.TagNumber(2)
  set fileName($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasFileName() => $_has(1);
  @$pb.TagNumber(2)
  void clearFileName() => $_clearField(2);

  /// file extension
  @$pb.TagNumber(3)
  $core.String get fileExtension => $_getSZ(2);
  @$pb.TagNumber(3)
  set fileExtension($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasFileExtension() => $_has(2);
  @$pb.TagNumber(3)
  void clearFileExtension() => $_clearField(3);

  /// file size
  @$pb.TagNumber(4)
  $core.int get fileSize => $_getIZ(3);
  @$pb.TagNumber(4)
  set fileSize($core.int value) => $_setUnsignedInt32(3, value);
  @$pb.TagNumber(4)
  $core.bool hasFileSize() => $_has(3);
  @$pb.TagNumber(4)
  void clearFileSize() => $_clearField(4);

  /// file description
  @$pb.TagNumber(5)
  $core.String get fileDescription => $_getSZ(4);
  @$pb.TagNumber(5)
  set fileDescription($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasFileDescription() => $_has(4);
  @$pb.TagNumber(5)
  void clearFileDescription() => $_clearField(5);
}

/// Group event information
/// this message is purely informational
class GroupEvent extends $pb.GeneratedMessage {
  factory GroupEvent({
    GroupEventType? eventType,
    $core.List<$core.int>? userId,
  }) {
    final result = create();
    if (eventType != null) result.eventType = eventType;
    if (userId != null) result.userId = userId;
    return result;
  }

  GroupEvent._();

  factory GroupEvent.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory GroupEvent.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GroupEvent',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.chat'),
      createEmptyInstance: create)
    ..aE<GroupEventType>(1, _omitFieldNames ? '' : 'eventType',
        enumValues: GroupEventType.values)
    ..a<$core.List<$core.int>>(
        2, _omitFieldNames ? '' : 'userId', $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupEvent clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupEvent copyWith(void Function(GroupEvent) updates) =>
      super.copyWith((message) => updates(message as GroupEvent)) as GroupEvent;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GroupEvent create() => GroupEvent._();
  @$core.override
  GroupEvent createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static GroupEvent getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GroupEvent>(create);
  static GroupEvent? _defaultInstance;

  /// group event type
  @$pb.TagNumber(1)
  GroupEventType get eventType => $_getN(0);
  @$pb.TagNumber(1)
  set eventType(GroupEventType value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasEventType() => $_has(0);
  @$pb.TagNumber(1)
  void clearEventType() => $_clearField(1);

  /// user ID of user joined or left
  @$pb.TagNumber(2)
  $core.List<$core.int> get userId => $_getN(1);
  @$pb.TagNumber(2)
  set userId($core.List<$core.int> value) => $_setBytes(1, value);
  @$pb.TagNumber(2)
  $core.bool hasUserId() => $_has(1);
  @$pb.TagNumber(2)
  void clearUserId() => $_clearField(2);
}

/// send chat message
class ChatMessageSend extends $pb.GeneratedMessage {
  factory ChatMessageSend({
    $core.List<$core.int>? groupId,
    $core.String? content,
  }) {
    final result = create();
    if (groupId != null) result.groupId = groupId;
    if (content != null) result.content = content;
    return result;
  }

  ChatMessageSend._();

  factory ChatMessageSend.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ChatMessageSend.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ChatMessageSend',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.chat'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'groupId', $pb.PbFieldType.OY)
    ..aOS(2, _omitFieldNames ? '' : 'content')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ChatMessageSend clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ChatMessageSend copyWith(void Function(ChatMessageSend) updates) =>
      super.copyWith((message) => updates(message as ChatMessageSend))
          as ChatMessageSend;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ChatMessageSend create() => ChatMessageSend._();
  @$core.override
  ChatMessageSend createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ChatMessageSend getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ChatMessageSend>(create);
  static ChatMessageSend? _defaultInstance;

  /// group id to which this message is sent
  @$pb.TagNumber(1)
  $core.List<$core.int> get groupId => $_getN(0);
  @$pb.TagNumber(1)
  set groupId($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => $_clearField(1);

  /// content of the message
  @$pb.TagNumber(2)
  $core.String get content => $_getSZ(1);
  @$pb.TagNumber(2)
  set content($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasContent() => $_has(1);
  @$pb.TagNumber(2)
  void clearContent() => $_clearField(2);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
