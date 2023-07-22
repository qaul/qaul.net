//
//  Generated code. Do not modify.
//  source: services/chat/chat.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'chat.pbenum.dart';

export 'chat.pbenum.dart';

enum Chat_Message {
  conversationRequest, 
  conversationList, 
  send, 
  notSet
}

class Chat extends $pb.GeneratedMessage {
  factory Chat() => create();
  Chat._() : super();
  factory Chat.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Chat.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static const $core.Map<$core.int, Chat_Message> _Chat_MessageByTag = {
    3 : Chat_Message.conversationRequest,
    4 : Chat_Message.conversationList,
    5 : Chat_Message.send,
    0 : Chat_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Chat', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.chat'), createEmptyInstance: create)
    ..oo(0, [3, 4, 5])
    ..aOM<ChatConversationRequest>(3, _omitFieldNames ? '' : 'conversationRequest', subBuilder: ChatConversationRequest.create)
    ..aOM<ChatConversationList>(4, _omitFieldNames ? '' : 'conversationList', subBuilder: ChatConversationList.create)
    ..aOM<ChatMessageSend>(5, _omitFieldNames ? '' : 'send', subBuilder: ChatMessageSend.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Chat clone() => Chat()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Chat copyWith(void Function(Chat) updates) => super.copyWith((message) => updates(message as Chat)) as Chat;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Chat create() => Chat._();
  Chat createEmptyInstance() => create();
  static $pb.PbList<Chat> createRepeated() => $pb.PbList<Chat>();
  @$core.pragma('dart2js:noInline')
  static Chat getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Chat>(create);
  static Chat? _defaultInstance;

  Chat_Message whichMessage() => _Chat_MessageByTag[$_whichOneof(0)]!;
  void clearMessage() => clearField($_whichOneof(0));

  @$pb.TagNumber(3)
  ChatConversationRequest get conversationRequest => $_getN(0);
  @$pb.TagNumber(3)
  set conversationRequest(ChatConversationRequest v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasConversationRequest() => $_has(0);
  @$pb.TagNumber(3)
  void clearConversationRequest() => clearField(3);
  @$pb.TagNumber(3)
  ChatConversationRequest ensureConversationRequest() => $_ensure(0);

  @$pb.TagNumber(4)
  ChatConversationList get conversationList => $_getN(1);
  @$pb.TagNumber(4)
  set conversationList(ChatConversationList v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasConversationList() => $_has(1);
  @$pb.TagNumber(4)
  void clearConversationList() => clearField(4);
  @$pb.TagNumber(4)
  ChatConversationList ensureConversationList() => $_ensure(1);

  @$pb.TagNumber(5)
  ChatMessageSend get send => $_getN(2);
  @$pb.TagNumber(5)
  set send(ChatMessageSend v) { setField(5, v); }
  @$pb.TagNumber(5)
  $core.bool hasSend() => $_has(2);
  @$pb.TagNumber(5)
  void clearSend() => clearField(5);
  @$pb.TagNumber(5)
  ChatMessageSend ensureSend() => $_ensure(2);
}

class ChatConversationRequest extends $pb.GeneratedMessage {
  factory ChatConversationRequest() => create();
  ChatConversationRequest._() : super();
  factory ChatConversationRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChatConversationRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ChatConversationRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.chat'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'groupId', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(2, _omitFieldNames ? '' : 'lastIndex', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChatConversationRequest clone() => ChatConversationRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChatConversationRequest copyWith(void Function(ChatConversationRequest) updates) => super.copyWith((message) => updates(message as ChatConversationRequest)) as ChatConversationRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ChatConversationRequest create() => ChatConversationRequest._();
  ChatConversationRequest createEmptyInstance() => create();
  static $pb.PbList<ChatConversationRequest> createRepeated() => $pb.PbList<ChatConversationRequest>();
  @$core.pragma('dart2js:noInline')
  static ChatConversationRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChatConversationRequest>(create);
  static ChatConversationRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get groupId => $_getN(0);
  @$pb.TagNumber(1)
  set groupId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get lastIndex => $_getI64(1);
  @$pb.TagNumber(2)
  set lastIndex($fixnum.Int64 v) { $_setInt64(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasLastIndex() => $_has(1);
  @$pb.TagNumber(2)
  void clearLastIndex() => clearField(2);
}

class ChatConversationList extends $pb.GeneratedMessage {
  factory ChatConversationList() => create();
  ChatConversationList._() : super();
  factory ChatConversationList.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChatConversationList.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ChatConversationList', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.chat'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'groupId', $pb.PbFieldType.OY)
    ..pc<ChatMessage>(2, _omitFieldNames ? '' : 'messageList', $pb.PbFieldType.PM, subBuilder: ChatMessage.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChatConversationList clone() => ChatConversationList()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChatConversationList copyWith(void Function(ChatConversationList) updates) => super.copyWith((message) => updates(message as ChatConversationList)) as ChatConversationList;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ChatConversationList create() => ChatConversationList._();
  ChatConversationList createEmptyInstance() => create();
  static $pb.PbList<ChatConversationList> createRepeated() => $pb.PbList<ChatConversationList>();
  @$core.pragma('dart2js:noInline')
  static ChatConversationList getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChatConversationList>(create);
  static ChatConversationList? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get groupId => $_getN(0);
  @$pb.TagNumber(1)
  set groupId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<ChatMessage> get messageList => $_getList(1);
}

class ChatMessage extends $pb.GeneratedMessage {
  factory ChatMessage() => create();
  ChatMessage._() : super();
  factory ChatMessage.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChatMessage.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ChatMessage', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.chat'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, _omitFieldNames ? '' : 'index', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.List<$core.int>>(2, _omitFieldNames ? '' : 'senderId', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(3, _omitFieldNames ? '' : 'messageId', $pb.PbFieldType.OY)
    ..e<MessageStatus>(4, _omitFieldNames ? '' : 'status', $pb.PbFieldType.OE, defaultOrMaker: MessageStatus.SENDING, valueOf: MessageStatus.valueOf, enumValues: MessageStatus.values)
    ..a<$core.List<$core.int>>(5, _omitFieldNames ? '' : 'groupId', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(6, _omitFieldNames ? '' : 'sentAt', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(7, _omitFieldNames ? '' : 'receivedAt', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.List<$core.int>>(8, _omitFieldNames ? '' : 'content', $pb.PbFieldType.OY)
    ..pc<MessageReceptionConfirmed>(10, _omitFieldNames ? '' : 'messageReceptionConfirmed', $pb.PbFieldType.PM, subBuilder: MessageReceptionConfirmed.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChatMessage clone() => ChatMessage()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChatMessage copyWith(void Function(ChatMessage) updates) => super.copyWith((message) => updates(message as ChatMessage)) as ChatMessage;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ChatMessage create() => ChatMessage._();
  ChatMessage createEmptyInstance() => create();
  static $pb.PbList<ChatMessage> createRepeated() => $pb.PbList<ChatMessage>();
  @$core.pragma('dart2js:noInline')
  static ChatMessage getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChatMessage>(create);
  static ChatMessage? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get index => $_getI64(0);
  @$pb.TagNumber(1)
  set index($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasIndex() => $_has(0);
  @$pb.TagNumber(1)
  void clearIndex() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get senderId => $_getN(1);
  @$pb.TagNumber(2)
  set senderId($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasSenderId() => $_has(1);
  @$pb.TagNumber(2)
  void clearSenderId() => clearField(2);

  @$pb.TagNumber(3)
  $core.List<$core.int> get messageId => $_getN(2);
  @$pb.TagNumber(3)
  set messageId($core.List<$core.int> v) { $_setBytes(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasMessageId() => $_has(2);
  @$pb.TagNumber(3)
  void clearMessageId() => clearField(3);

  @$pb.TagNumber(4)
  MessageStatus get status => $_getN(3);
  @$pb.TagNumber(4)
  set status(MessageStatus v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasStatus() => $_has(3);
  @$pb.TagNumber(4)
  void clearStatus() => clearField(4);

  @$pb.TagNumber(5)
  $core.List<$core.int> get groupId => $_getN(4);
  @$pb.TagNumber(5)
  set groupId($core.List<$core.int> v) { $_setBytes(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasGroupId() => $_has(4);
  @$pb.TagNumber(5)
  void clearGroupId() => clearField(5);

  @$pb.TagNumber(6)
  $fixnum.Int64 get sentAt => $_getI64(5);
  @$pb.TagNumber(6)
  set sentAt($fixnum.Int64 v) { $_setInt64(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasSentAt() => $_has(5);
  @$pb.TagNumber(6)
  void clearSentAt() => clearField(6);

  @$pb.TagNumber(7)
  $fixnum.Int64 get receivedAt => $_getI64(6);
  @$pb.TagNumber(7)
  set receivedAt($fixnum.Int64 v) { $_setInt64(6, v); }
  @$pb.TagNumber(7)
  $core.bool hasReceivedAt() => $_has(6);
  @$pb.TagNumber(7)
  void clearReceivedAt() => clearField(7);

  @$pb.TagNumber(8)
  $core.List<$core.int> get content => $_getN(7);
  @$pb.TagNumber(8)
  set content($core.List<$core.int> v) { $_setBytes(7, v); }
  @$pb.TagNumber(8)
  $core.bool hasContent() => $_has(7);
  @$pb.TagNumber(8)
  void clearContent() => clearField(8);

  @$pb.TagNumber(10)
  $core.List<MessageReceptionConfirmed> get messageReceptionConfirmed => $_getList(8);
}

class MessageReceptionConfirmed extends $pb.GeneratedMessage {
  factory MessageReceptionConfirmed() => create();
  MessageReceptionConfirmed._() : super();
  factory MessageReceptionConfirmed.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory MessageReceptionConfirmed.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'MessageReceptionConfirmed', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.chat'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'userId', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(2, _omitFieldNames ? '' : 'confirmedAt', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  MessageReceptionConfirmed clone() => MessageReceptionConfirmed()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  MessageReceptionConfirmed copyWith(void Function(MessageReceptionConfirmed) updates) => super.copyWith((message) => updates(message as MessageReceptionConfirmed)) as MessageReceptionConfirmed;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static MessageReceptionConfirmed create() => MessageReceptionConfirmed._();
  MessageReceptionConfirmed createEmptyInstance() => create();
  static $pb.PbList<MessageReceptionConfirmed> createRepeated() => $pb.PbList<MessageReceptionConfirmed>();
  @$core.pragma('dart2js:noInline')
  static MessageReceptionConfirmed getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<MessageReceptionConfirmed>(create);
  static MessageReceptionConfirmed? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get userId => $_getN(0);
  @$pb.TagNumber(1)
  set userId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasUserId() => $_has(0);
  @$pb.TagNumber(1)
  void clearUserId() => clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get confirmedAt => $_getI64(1);
  @$pb.TagNumber(2)
  set confirmedAt($fixnum.Int64 v) { $_setInt64(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasConfirmedAt() => $_has(1);
  @$pb.TagNumber(2)
  void clearConfirmedAt() => clearField(2);
}

enum ChatContentMessage_Message {
  chatContent, 
  fileContent, 
  groupEvent, 
  notSet
}

class ChatContentMessage extends $pb.GeneratedMessage {
  factory ChatContentMessage() => create();
  ChatContentMessage._() : super();
  factory ChatContentMessage.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChatContentMessage.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static const $core.Map<$core.int, ChatContentMessage_Message> _ChatContentMessage_MessageByTag = {
    1 : ChatContentMessage_Message.chatContent,
    2 : ChatContentMessage_Message.fileContent,
    3 : ChatContentMessage_Message.groupEvent,
    0 : ChatContentMessage_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ChatContentMessage', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.chat'), createEmptyInstance: create)
    ..oo(0, [1, 2, 3])
    ..aOM<ChatContent>(1, _omitFieldNames ? '' : 'chatContent', subBuilder: ChatContent.create)
    ..aOM<FileContent>(2, _omitFieldNames ? '' : 'fileContent', subBuilder: FileContent.create)
    ..aOM<GroupEvent>(3, _omitFieldNames ? '' : 'groupEvent', subBuilder: GroupEvent.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChatContentMessage clone() => ChatContentMessage()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChatContentMessage copyWith(void Function(ChatContentMessage) updates) => super.copyWith((message) => updates(message as ChatContentMessage)) as ChatContentMessage;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ChatContentMessage create() => ChatContentMessage._();
  ChatContentMessage createEmptyInstance() => create();
  static $pb.PbList<ChatContentMessage> createRepeated() => $pb.PbList<ChatContentMessage>();
  @$core.pragma('dart2js:noInline')
  static ChatContentMessage getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChatContentMessage>(create);
  static ChatContentMessage? _defaultInstance;

  ChatContentMessage_Message whichMessage() => _ChatContentMessage_MessageByTag[$_whichOneof(0)]!;
  void clearMessage() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  ChatContent get chatContent => $_getN(0);
  @$pb.TagNumber(1)
  set chatContent(ChatContent v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasChatContent() => $_has(0);
  @$pb.TagNumber(1)
  void clearChatContent() => clearField(1);
  @$pb.TagNumber(1)
  ChatContent ensureChatContent() => $_ensure(0);

  @$pb.TagNumber(2)
  FileContent get fileContent => $_getN(1);
  @$pb.TagNumber(2)
  set fileContent(FileContent v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasFileContent() => $_has(1);
  @$pb.TagNumber(2)
  void clearFileContent() => clearField(2);
  @$pb.TagNumber(2)
  FileContent ensureFileContent() => $_ensure(1);

  @$pb.TagNumber(3)
  GroupEvent get groupEvent => $_getN(2);
  @$pb.TagNumber(3)
  set groupEvent(GroupEvent v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasGroupEvent() => $_has(2);
  @$pb.TagNumber(3)
  void clearGroupEvent() => clearField(3);
  @$pb.TagNumber(3)
  GroupEvent ensureGroupEvent() => $_ensure(2);
}

class ChatContent extends $pb.GeneratedMessage {
  factory ChatContent() => create();
  ChatContent._() : super();
  factory ChatContent.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChatContent.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ChatContent', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.chat'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'text')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChatContent clone() => ChatContent()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChatContent copyWith(void Function(ChatContent) updates) => super.copyWith((message) => updates(message as ChatContent)) as ChatContent;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ChatContent create() => ChatContent._();
  ChatContent createEmptyInstance() => create();
  static $pb.PbList<ChatContent> createRepeated() => $pb.PbList<ChatContent>();
  @$core.pragma('dart2js:noInline')
  static ChatContent getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChatContent>(create);
  static ChatContent? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get text => $_getSZ(0);
  @$pb.TagNumber(1)
  set text($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasText() => $_has(0);
  @$pb.TagNumber(1)
  void clearText() => clearField(1);
}

class FileContent extends $pb.GeneratedMessage {
  factory FileContent() => create();
  FileContent._() : super();
  factory FileContent.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FileContent.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'FileContent', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.chat'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, _omitFieldNames ? '' : 'fileId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOS(2, _omitFieldNames ? '' : 'fileName')
    ..aOS(3, _omitFieldNames ? '' : 'fileExtension')
    ..a<$core.int>(4, _omitFieldNames ? '' : 'fileSize', $pb.PbFieldType.OU3)
    ..aOS(5, _omitFieldNames ? '' : 'fileDescription')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  FileContent clone() => FileContent()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  FileContent copyWith(void Function(FileContent) updates) => super.copyWith((message) => updates(message as FileContent)) as FileContent;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FileContent create() => FileContent._();
  FileContent createEmptyInstance() => create();
  static $pb.PbList<FileContent> createRepeated() => $pb.PbList<FileContent>();
  @$core.pragma('dart2js:noInline')
  static FileContent getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FileContent>(create);
  static FileContent? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get fileId => $_getI64(0);
  @$pb.TagNumber(1)
  set fileId($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasFileId() => $_has(0);
  @$pb.TagNumber(1)
  void clearFileId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get fileName => $_getSZ(1);
  @$pb.TagNumber(2)
  set fileName($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasFileName() => $_has(1);
  @$pb.TagNumber(2)
  void clearFileName() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get fileExtension => $_getSZ(2);
  @$pb.TagNumber(3)
  set fileExtension($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasFileExtension() => $_has(2);
  @$pb.TagNumber(3)
  void clearFileExtension() => clearField(3);

  @$pb.TagNumber(4)
  $core.int get fileSize => $_getIZ(3);
  @$pb.TagNumber(4)
  set fileSize($core.int v) { $_setUnsignedInt32(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasFileSize() => $_has(3);
  @$pb.TagNumber(4)
  void clearFileSize() => clearField(4);

  @$pb.TagNumber(5)
  $core.String get fileDescription => $_getSZ(4);
  @$pb.TagNumber(5)
  set fileDescription($core.String v) { $_setString(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasFileDescription() => $_has(4);
  @$pb.TagNumber(5)
  void clearFileDescription() => clearField(5);
}

class GroupEvent extends $pb.GeneratedMessage {
  factory GroupEvent() => create();
  GroupEvent._() : super();
  factory GroupEvent.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupEvent.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GroupEvent', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.chat'), createEmptyInstance: create)
    ..e<GroupEventType>(1, _omitFieldNames ? '' : 'eventType', $pb.PbFieldType.OE, defaultOrMaker: GroupEventType.DEFAULT, valueOf: GroupEventType.valueOf, enumValues: GroupEventType.values)
    ..a<$core.List<$core.int>>(2, _omitFieldNames ? '' : 'userId', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GroupEvent clone() => GroupEvent()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GroupEvent copyWith(void Function(GroupEvent) updates) => super.copyWith((message) => updates(message as GroupEvent)) as GroupEvent;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GroupEvent create() => GroupEvent._();
  GroupEvent createEmptyInstance() => create();
  static $pb.PbList<GroupEvent> createRepeated() => $pb.PbList<GroupEvent>();
  @$core.pragma('dart2js:noInline')
  static GroupEvent getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupEvent>(create);
  static GroupEvent? _defaultInstance;

  @$pb.TagNumber(1)
  GroupEventType get eventType => $_getN(0);
  @$pb.TagNumber(1)
  set eventType(GroupEventType v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasEventType() => $_has(0);
  @$pb.TagNumber(1)
  void clearEventType() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get userId => $_getN(1);
  @$pb.TagNumber(2)
  set userId($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasUserId() => $_has(1);
  @$pb.TagNumber(2)
  void clearUserId() => clearField(2);
}

class ChatMessageSend extends $pb.GeneratedMessage {
  factory ChatMessageSend() => create();
  ChatMessageSend._() : super();
  factory ChatMessageSend.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChatMessageSend.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ChatMessageSend', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.chat'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'groupId', $pb.PbFieldType.OY)
    ..aOS(2, _omitFieldNames ? '' : 'content')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChatMessageSend clone() => ChatMessageSend()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChatMessageSend copyWith(void Function(ChatMessageSend) updates) => super.copyWith((message) => updates(message as ChatMessageSend)) as ChatMessageSend;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ChatMessageSend create() => ChatMessageSend._();
  ChatMessageSend createEmptyInstance() => create();
  static $pb.PbList<ChatMessageSend> createRepeated() => $pb.PbList<ChatMessageSend>();
  @$core.pragma('dart2js:noInline')
  static ChatMessageSend getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChatMessageSend>(create);
  static ChatMessageSend? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get groupId => $_getN(0);
  @$pb.TagNumber(1)
  set groupId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get content => $_getSZ(1);
  @$pb.TagNumber(2)
  set content($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasContent() => $_has(1);
  @$pb.TagNumber(2)
  void clearContent() => clearField(2);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
