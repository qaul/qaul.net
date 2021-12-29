///
//  Generated code. Do not modify.
//  source: services/chat/chat.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

enum Chat_Message {
  overviewRequest, 
  overviewList, 
  conversationRequest, 
  conversationList, 
  send, 
  notSet
}

class Chat extends $pb.GeneratedMessage {
  static const $core.Map<$core.int, Chat_Message> _Chat_MessageByTag = {
    1 : Chat_Message.overviewRequest,
    2 : Chat_Message.overviewList,
    3 : Chat_Message.conversationRequest,
    4 : Chat_Message.conversationList,
    5 : Chat_Message.send,
    0 : Chat_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Chat', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.chat'), createEmptyInstance: create)
    ..oo(0, [1, 2, 3, 4, 5])
    ..aOM<ChatOverviewRequest>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'overviewRequest', subBuilder: ChatOverviewRequest.create)
    ..aOM<ChatOverviewList>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'overviewList', subBuilder: ChatOverviewList.create)
    ..aOM<ChatConversationRequest>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'conversationRequest', subBuilder: ChatConversationRequest.create)
    ..aOM<ChatConversationList>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'conversationList', subBuilder: ChatConversationList.create)
    ..aOM<ChatMessageSend>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'send', subBuilder: ChatMessageSend.create)
    ..hasRequiredFields = false
  ;

  Chat._() : super();
  factory Chat({
    ChatOverviewRequest? overviewRequest,
    ChatOverviewList? overviewList,
    ChatConversationRequest? conversationRequest,
    ChatConversationList? conversationList,
    ChatMessageSend? send,
  }) {
    final _result = create();
    if (overviewRequest != null) {
      _result.overviewRequest = overviewRequest;
    }
    if (overviewList != null) {
      _result.overviewList = overviewList;
    }
    if (conversationRequest != null) {
      _result.conversationRequest = conversationRequest;
    }
    if (conversationList != null) {
      _result.conversationList = conversationList;
    }
    if (send != null) {
      _result.send = send;
    }
    return _result;
  }
  factory Chat.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Chat.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Chat clone() => Chat()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Chat copyWith(void Function(Chat) updates) => super.copyWith((message) => updates(message as Chat)) as Chat; // ignore: deprecated_member_use
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

  @$pb.TagNumber(1)
  ChatOverviewRequest get overviewRequest => $_getN(0);
  @$pb.TagNumber(1)
  set overviewRequest(ChatOverviewRequest v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasOverviewRequest() => $_has(0);
  @$pb.TagNumber(1)
  void clearOverviewRequest() => clearField(1);
  @$pb.TagNumber(1)
  ChatOverviewRequest ensureOverviewRequest() => $_ensure(0);

  @$pb.TagNumber(2)
  ChatOverviewList get overviewList => $_getN(1);
  @$pb.TagNumber(2)
  set overviewList(ChatOverviewList v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasOverviewList() => $_has(1);
  @$pb.TagNumber(2)
  void clearOverviewList() => clearField(2);
  @$pb.TagNumber(2)
  ChatOverviewList ensureOverviewList() => $_ensure(1);

  @$pb.TagNumber(3)
  ChatConversationRequest get conversationRequest => $_getN(2);
  @$pb.TagNumber(3)
  set conversationRequest(ChatConversationRequest v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasConversationRequest() => $_has(2);
  @$pb.TagNumber(3)
  void clearConversationRequest() => clearField(3);
  @$pb.TagNumber(3)
  ChatConversationRequest ensureConversationRequest() => $_ensure(2);

  @$pb.TagNumber(4)
  ChatConversationList get conversationList => $_getN(3);
  @$pb.TagNumber(4)
  set conversationList(ChatConversationList v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasConversationList() => $_has(3);
  @$pb.TagNumber(4)
  void clearConversationList() => clearField(4);
  @$pb.TagNumber(4)
  ChatConversationList ensureConversationList() => $_ensure(3);

  @$pb.TagNumber(5)
  ChatMessageSend get send => $_getN(4);
  @$pb.TagNumber(5)
  set send(ChatMessageSend v) { setField(5, v); }
  @$pb.TagNumber(5)
  $core.bool hasSend() => $_has(4);
  @$pb.TagNumber(5)
  void clearSend() => clearField(5);
  @$pb.TagNumber(5)
  ChatMessageSend ensureSend() => $_ensure(4);
}

class ChatOverviewRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ChatOverviewRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.chat'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChatOverviewRequest._() : super();
  factory ChatOverviewRequest() => create();
  factory ChatOverviewRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChatOverviewRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChatOverviewRequest clone() => ChatOverviewRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChatOverviewRequest copyWith(void Function(ChatOverviewRequest) updates) => super.copyWith((message) => updates(message as ChatOverviewRequest)) as ChatOverviewRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChatOverviewRequest create() => ChatOverviewRequest._();
  ChatOverviewRequest createEmptyInstance() => create();
  static $pb.PbList<ChatOverviewRequest> createRepeated() => $pb.PbList<ChatOverviewRequest>();
  @$core.pragma('dart2js:noInline')
  static ChatOverviewRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChatOverviewRequest>(create);
  static ChatOverviewRequest? _defaultInstance;
}

class ChatOverviewList extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ChatOverviewList', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.chat'), createEmptyInstance: create)
    ..pc<ChatConversation>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'conversationList', $pb.PbFieldType.PM, subBuilder: ChatConversation.create)
    ..hasRequiredFields = false
  ;

  ChatOverviewList._() : super();
  factory ChatOverviewList({
    $core.Iterable<ChatConversation>? conversationList,
  }) {
    final _result = create();
    if (conversationList != null) {
      _result.conversationList.addAll(conversationList);
    }
    return _result;
  }
  factory ChatOverviewList.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChatOverviewList.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChatOverviewList clone() => ChatOverviewList()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChatOverviewList copyWith(void Function(ChatOverviewList) updates) => super.copyWith((message) => updates(message as ChatOverviewList)) as ChatOverviewList; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChatOverviewList create() => ChatOverviewList._();
  ChatOverviewList createEmptyInstance() => create();
  static $pb.PbList<ChatOverviewList> createRepeated() => $pb.PbList<ChatOverviewList>();
  @$core.pragma('dart2js:noInline')
  static ChatOverviewList getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChatOverviewList>(create);
  static ChatOverviewList? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<ChatConversation> get conversationList => $_getList(0);
}

class ChatConversation extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ChatConversation', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.chat'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'conversationId', $pb.PbFieldType.OY)
    ..a<$core.int>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'lastMessageIndex', $pb.PbFieldType.OU3)
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'name')
    ..a<$fixnum.Int64>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'lastMessageAt', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.int>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'unread', $pb.PbFieldType.O3)
    ..aOS(6, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'content')
    ..hasRequiredFields = false
  ;

  ChatConversation._() : super();
  factory ChatConversation({
    $core.List<$core.int>? conversationId,
    $core.int? lastMessageIndex,
    $core.String? name,
    $fixnum.Int64? lastMessageAt,
    $core.int? unread,
    $core.String? content,
  }) {
    final _result = create();
    if (conversationId != null) {
      _result.conversationId = conversationId;
    }
    if (lastMessageIndex != null) {
      _result.lastMessageIndex = lastMessageIndex;
    }
    if (name != null) {
      _result.name = name;
    }
    if (lastMessageAt != null) {
      _result.lastMessageAt = lastMessageAt;
    }
    if (unread != null) {
      _result.unread = unread;
    }
    if (content != null) {
      _result.content = content;
    }
    return _result;
  }
  factory ChatConversation.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChatConversation.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChatConversation clone() => ChatConversation()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChatConversation copyWith(void Function(ChatConversation) updates) => super.copyWith((message) => updates(message as ChatConversation)) as ChatConversation; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChatConversation create() => ChatConversation._();
  ChatConversation createEmptyInstance() => create();
  static $pb.PbList<ChatConversation> createRepeated() => $pb.PbList<ChatConversation>();
  @$core.pragma('dart2js:noInline')
  static ChatConversation getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChatConversation>(create);
  static ChatConversation? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get conversationId => $_getN(0);
  @$pb.TagNumber(1)
  set conversationId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasConversationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearConversationId() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get lastMessageIndex => $_getIZ(1);
  @$pb.TagNumber(2)
  set lastMessageIndex($core.int v) { $_setUnsignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasLastMessageIndex() => $_has(1);
  @$pb.TagNumber(2)
  void clearLastMessageIndex() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get name => $_getSZ(2);
  @$pb.TagNumber(3)
  set name($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasName() => $_has(2);
  @$pb.TagNumber(3)
  void clearName() => clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get lastMessageAt => $_getI64(3);
  @$pb.TagNumber(4)
  set lastMessageAt($fixnum.Int64 v) { $_setInt64(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasLastMessageAt() => $_has(3);
  @$pb.TagNumber(4)
  void clearLastMessageAt() => clearField(4);

  @$pb.TagNumber(5)
  $core.int get unread => $_getIZ(4);
  @$pb.TagNumber(5)
  set unread($core.int v) { $_setSignedInt32(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasUnread() => $_has(4);
  @$pb.TagNumber(5)
  void clearUnread() => clearField(5);

  @$pb.TagNumber(6)
  $core.String get content => $_getSZ(5);
  @$pb.TagNumber(6)
  set content($core.String v) { $_setString(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasContent() => $_has(5);
  @$pb.TagNumber(6)
  void clearContent() => clearField(6);
}

class ChatConversationRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ChatConversationRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.chat'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'conversationId', $pb.PbFieldType.OY)
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'lastReceived')
    ..hasRequiredFields = false
  ;

  ChatConversationRequest._() : super();
  factory ChatConversationRequest({
    $core.List<$core.int>? conversationId,
    $core.String? lastReceived,
  }) {
    final _result = create();
    if (conversationId != null) {
      _result.conversationId = conversationId;
    }
    if (lastReceived != null) {
      _result.lastReceived = lastReceived;
    }
    return _result;
  }
  factory ChatConversationRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChatConversationRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChatConversationRequest clone() => ChatConversationRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChatConversationRequest copyWith(void Function(ChatConversationRequest) updates) => super.copyWith((message) => updates(message as ChatConversationRequest)) as ChatConversationRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChatConversationRequest create() => ChatConversationRequest._();
  ChatConversationRequest createEmptyInstance() => create();
  static $pb.PbList<ChatConversationRequest> createRepeated() => $pb.PbList<ChatConversationRequest>();
  @$core.pragma('dart2js:noInline')
  static ChatConversationRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChatConversationRequest>(create);
  static ChatConversationRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get conversationId => $_getN(0);
  @$pb.TagNumber(1)
  set conversationId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasConversationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearConversationId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get lastReceived => $_getSZ(1);
  @$pb.TagNumber(2)
  set lastReceived($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasLastReceived() => $_has(1);
  @$pb.TagNumber(2)
  void clearLastReceived() => clearField(2);
}

class ChatConversationList extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ChatConversationList', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.chat'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'conversationId', $pb.PbFieldType.OY)
    ..pc<ChatMessage>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'messageList', $pb.PbFieldType.PM, subBuilder: ChatMessage.create)
    ..hasRequiredFields = false
  ;

  ChatConversationList._() : super();
  factory ChatConversationList({
    $core.List<$core.int>? conversationId,
    $core.Iterable<ChatMessage>? messageList,
  }) {
    final _result = create();
    if (conversationId != null) {
      _result.conversationId = conversationId;
    }
    if (messageList != null) {
      _result.messageList.addAll(messageList);
    }
    return _result;
  }
  factory ChatConversationList.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChatConversationList.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChatConversationList clone() => ChatConversationList()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChatConversationList copyWith(void Function(ChatConversationList) updates) => super.copyWith((message) => updates(message as ChatConversationList)) as ChatConversationList; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChatConversationList create() => ChatConversationList._();
  ChatConversationList createEmptyInstance() => create();
  static $pb.PbList<ChatConversationList> createRepeated() => $pb.PbList<ChatConversationList>();
  @$core.pragma('dart2js:noInline')
  static ChatConversationList getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChatConversationList>(create);
  static ChatConversationList? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get conversationId => $_getN(0);
  @$pb.TagNumber(1)
  set conversationId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasConversationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearConversationId() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<ChatMessage> get messageList => $_getList(1);
}

class ChatMessage extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ChatMessage', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.chat'), createEmptyInstance: create)
    ..a<$core.int>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'index', $pb.PbFieldType.OU3)
    ..a<$core.List<$core.int>>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'senderId', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'messageId', $pb.PbFieldType.OY)
    ..a<$core.int>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'status', $pb.PbFieldType.O3)
    ..a<$fixnum.Int64>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'sentAt', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(6, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'receivedAt', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOS(7, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'content')
    ..hasRequiredFields = false
  ;

  ChatMessage._() : super();
  factory ChatMessage({
    $core.int? index,
    $core.List<$core.int>? senderId,
    $core.List<$core.int>? messageId,
    $core.int? status,
    $fixnum.Int64? sentAt,
    $fixnum.Int64? receivedAt,
    $core.String? content,
  }) {
    final _result = create();
    if (index != null) {
      _result.index = index;
    }
    if (senderId != null) {
      _result.senderId = senderId;
    }
    if (messageId != null) {
      _result.messageId = messageId;
    }
    if (status != null) {
      _result.status = status;
    }
    if (sentAt != null) {
      _result.sentAt = sentAt;
    }
    if (receivedAt != null) {
      _result.receivedAt = receivedAt;
    }
    if (content != null) {
      _result.content = content;
    }
    return _result;
  }
  factory ChatMessage.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChatMessage.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChatMessage clone() => ChatMessage()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChatMessage copyWith(void Function(ChatMessage) updates) => super.copyWith((message) => updates(message as ChatMessage)) as ChatMessage; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChatMessage create() => ChatMessage._();
  ChatMessage createEmptyInstance() => create();
  static $pb.PbList<ChatMessage> createRepeated() => $pb.PbList<ChatMessage>();
  @$core.pragma('dart2js:noInline')
  static ChatMessage getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChatMessage>(create);
  static ChatMessage? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get index => $_getIZ(0);
  @$pb.TagNumber(1)
  set index($core.int v) { $_setUnsignedInt32(0, v); }
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
  $core.int get status => $_getIZ(3);
  @$pb.TagNumber(4)
  set status($core.int v) { $_setSignedInt32(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasStatus() => $_has(3);
  @$pb.TagNumber(4)
  void clearStatus() => clearField(4);

  @$pb.TagNumber(5)
  $fixnum.Int64 get sentAt => $_getI64(4);
  @$pb.TagNumber(5)
  set sentAt($fixnum.Int64 v) { $_setInt64(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasSentAt() => $_has(4);
  @$pb.TagNumber(5)
  void clearSentAt() => clearField(5);

  @$pb.TagNumber(6)
  $fixnum.Int64 get receivedAt => $_getI64(5);
  @$pb.TagNumber(6)
  set receivedAt($fixnum.Int64 v) { $_setInt64(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasReceivedAt() => $_has(5);
  @$pb.TagNumber(6)
  void clearReceivedAt() => clearField(6);

  @$pb.TagNumber(7)
  $core.String get content => $_getSZ(6);
  @$pb.TagNumber(7)
  set content($core.String v) { $_setString(6, v); }
  @$pb.TagNumber(7)
  $core.bool hasContent() => $_has(6);
  @$pb.TagNumber(7)
  void clearContent() => clearField(7);
}

class ChatMessageSend extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ChatMessageSend', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.chat'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'conversationId', $pb.PbFieldType.OY)
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'content')
    ..hasRequiredFields = false
  ;

  ChatMessageSend._() : super();
  factory ChatMessageSend({
    $core.List<$core.int>? conversationId,
    $core.String? content,
  }) {
    final _result = create();
    if (conversationId != null) {
      _result.conversationId = conversationId;
    }
    if (content != null) {
      _result.content = content;
    }
    return _result;
  }
  factory ChatMessageSend.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChatMessageSend.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChatMessageSend clone() => ChatMessageSend()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChatMessageSend copyWith(void Function(ChatMessageSend) updates) => super.copyWith((message) => updates(message as ChatMessageSend)) as ChatMessageSend; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChatMessageSend create() => ChatMessageSend._();
  ChatMessageSend createEmptyInstance() => create();
  static $pb.PbList<ChatMessageSend> createRepeated() => $pb.PbList<ChatMessageSend>();
  @$core.pragma('dart2js:noInline')
  static ChatMessageSend getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChatMessageSend>(create);
  static ChatMessageSend? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get conversationId => $_getN(0);
  @$pb.TagNumber(1)
  set conversationId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasConversationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearConversationId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get content => $_getSZ(1);
  @$pb.TagNumber(2)
  set content($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasContent() => $_has(1);
  @$pb.TagNumber(2)
  void clearContent() => clearField(2);
}

