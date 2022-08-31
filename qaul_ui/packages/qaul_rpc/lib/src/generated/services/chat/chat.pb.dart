///
//  Generated code. Do not modify.
//  source: services/chat/chat.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'chat.pbenum.dart';

export 'chat.pbenum.dart';

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
    ..pc<ChatOverview>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'overviewList', $pb.PbFieldType.PM, subBuilder: ChatOverview.create)
    ..hasRequiredFields = false
  ;

  ChatOverviewList._() : super();
  factory ChatOverviewList({
    $core.Iterable<ChatOverview>? overviewList,
  }) {
    final _result = create();
    if (overviewList != null) {
      _result.overviewList.addAll(overviewList);
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
  $core.List<ChatOverview> get overviewList => $_getList(0);
}

class ChatOverview extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ChatOverview', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.chat'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'conversationId', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'lastMessageIndex', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'name')
    ..a<$fixnum.Int64>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'lastMessageAt', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.int>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'unread', $pb.PbFieldType.O3)
    ..e<ChatContentType>(6, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'contentType', $pb.PbFieldType.OE, defaultOrMaker: ChatContentType.NONE, valueOf: ChatContentType.valueOf, enumValues: ChatContentType.values)
    ..a<$core.List<$core.int>>(7, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'content', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(8, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'lastMessageSenderId', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  ChatOverview._() : super();
  factory ChatOverview({
    $core.List<$core.int>? conversationId,
    $fixnum.Int64? lastMessageIndex,
    $core.String? name,
    $fixnum.Int64? lastMessageAt,
    $core.int? unread,
    ChatContentType? contentType,
    $core.List<$core.int>? content,
    $core.List<$core.int>? lastMessageSenderId,
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
    if (contentType != null) {
      _result.contentType = contentType;
    }
    if (content != null) {
      _result.content = content;
    }
    if (lastMessageSenderId != null) {
      _result.lastMessageSenderId = lastMessageSenderId;
    }
    return _result;
  }
  factory ChatOverview.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChatOverview.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChatOverview clone() => ChatOverview()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChatOverview copyWith(void Function(ChatOverview) updates) => super.copyWith((message) => updates(message as ChatOverview)) as ChatOverview; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChatOverview create() => ChatOverview._();
  ChatOverview createEmptyInstance() => create();
  static $pb.PbList<ChatOverview> createRepeated() => $pb.PbList<ChatOverview>();
  @$core.pragma('dart2js:noInline')
  static ChatOverview getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChatOverview>(create);
  static ChatOverview? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get conversationId => $_getN(0);
  @$pb.TagNumber(1)
  set conversationId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasConversationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearConversationId() => clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get lastMessageIndex => $_getI64(1);
  @$pb.TagNumber(2)
  set lastMessageIndex($fixnum.Int64 v) { $_setInt64(1, v); }
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
  ChatContentType get contentType => $_getN(5);
  @$pb.TagNumber(6)
  set contentType(ChatContentType v) { setField(6, v); }
  @$pb.TagNumber(6)
  $core.bool hasContentType() => $_has(5);
  @$pb.TagNumber(6)
  void clearContentType() => clearField(6);

  @$pb.TagNumber(7)
  $core.List<$core.int> get content => $_getN(6);
  @$pb.TagNumber(7)
  set content($core.List<$core.int> v) { $_setBytes(6, v); }
  @$pb.TagNumber(7)
  $core.bool hasContent() => $_has(6);
  @$pb.TagNumber(7)
  void clearContent() => clearField(7);

  @$pb.TagNumber(8)
  $core.List<$core.int> get lastMessageSenderId => $_getN(7);
  @$pb.TagNumber(8)
  set lastMessageSenderId($core.List<$core.int> v) { $_setBytes(7, v); }
  @$pb.TagNumber(8)
  $core.bool hasLastMessageSenderId() => $_has(7);
  @$pb.TagNumber(8)
  void clearLastMessageSenderId() => clearField(8);
}

class ChatConversationRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ChatConversationRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.chat'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'conversationId', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'lastIndex', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  ChatConversationRequest._() : super();
  factory ChatConversationRequest({
    $core.List<$core.int>? conversationId,
    $fixnum.Int64? lastIndex,
  }) {
    final _result = create();
    if (conversationId != null) {
      _result.conversationId = conversationId;
    }
    if (lastIndex != null) {
      _result.lastIndex = lastIndex;
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
  $fixnum.Int64 get lastIndex => $_getI64(1);
  @$pb.TagNumber(2)
  set lastIndex($fixnum.Int64 v) { $_setInt64(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasLastIndex() => $_has(1);
  @$pb.TagNumber(2)
  void clearLastIndex() => clearField(2);
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
    ..a<$fixnum.Int64>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'index', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.List<$core.int>>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'senderId', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'messageId', $pb.PbFieldType.OY)
    ..e<MessageStatus>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'status', $pb.PbFieldType.OE, defaultOrMaker: MessageStatus.SENDING, valueOf: MessageStatus.valueOf, enumValues: MessageStatus.values)
    ..a<$core.List<$core.int>>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'conversationId', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(6, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'sentAt', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(7, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'receivedAt', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..e<ChatContentType>(8, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'contentType', $pb.PbFieldType.OE, defaultOrMaker: ChatContentType.NONE, valueOf: ChatContentType.valueOf, enumValues: ChatContentType.values)
    ..a<$core.List<$core.int>>(9, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'content', $pb.PbFieldType.OY)
    ..pc<MessageReceptionConfirmed>(10, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'messageReceptionConfirmed', $pb.PbFieldType.PM, subBuilder: MessageReceptionConfirmed.create)
    ..hasRequiredFields = false
  ;

  ChatMessage._() : super();
  factory ChatMessage({
    $fixnum.Int64? index,
    $core.List<$core.int>? senderId,
    $core.List<$core.int>? messageId,
    MessageStatus? status,
    $core.List<$core.int>? conversationId,
    $fixnum.Int64? sentAt,
    $fixnum.Int64? receivedAt,
    ChatContentType? contentType,
    $core.List<$core.int>? content,
    $core.Iterable<MessageReceptionConfirmed>? messageReceptionConfirmed,
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
    if (conversationId != null) {
      _result.conversationId = conversationId;
    }
    if (sentAt != null) {
      _result.sentAt = sentAt;
    }
    if (receivedAt != null) {
      _result.receivedAt = receivedAt;
    }
    if (contentType != null) {
      _result.contentType = contentType;
    }
    if (content != null) {
      _result.content = content;
    }
    if (messageReceptionConfirmed != null) {
      _result.messageReceptionConfirmed.addAll(messageReceptionConfirmed);
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
  $core.List<$core.int> get conversationId => $_getN(4);
  @$pb.TagNumber(5)
  set conversationId($core.List<$core.int> v) { $_setBytes(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasConversationId() => $_has(4);
  @$pb.TagNumber(5)
  void clearConversationId() => clearField(5);

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
  ChatContentType get contentType => $_getN(7);
  @$pb.TagNumber(8)
  set contentType(ChatContentType v) { setField(8, v); }
  @$pb.TagNumber(8)
  $core.bool hasContentType() => $_has(7);
  @$pb.TagNumber(8)
  void clearContentType() => clearField(8);

  @$pb.TagNumber(9)
  $core.List<$core.int> get content => $_getN(8);
  @$pb.TagNumber(9)
  set content($core.List<$core.int> v) { $_setBytes(8, v); }
  @$pb.TagNumber(9)
  $core.bool hasContent() => $_has(8);
  @$pb.TagNumber(9)
  void clearContent() => clearField(9);

  @$pb.TagNumber(10)
  $core.List<MessageReceptionConfirmed> get messageReceptionConfirmed => $_getList(9);
}

class MessageReceptionConfirmed extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'MessageReceptionConfirmed', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.chat'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'userId', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'confirmedAt', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  MessageReceptionConfirmed._() : super();
  factory MessageReceptionConfirmed({
    $core.List<$core.int>? userId,
    $fixnum.Int64? confirmedAt,
  }) {
    final _result = create();
    if (userId != null) {
      _result.userId = userId;
    }
    if (confirmedAt != null) {
      _result.confirmedAt = confirmedAt;
    }
    return _result;
  }
  factory MessageReceptionConfirmed.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory MessageReceptionConfirmed.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  MessageReceptionConfirmed clone() => MessageReceptionConfirmed()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  MessageReceptionConfirmed copyWith(void Function(MessageReceptionConfirmed) updates) => super.copyWith((message) => updates(message as MessageReceptionConfirmed)) as MessageReceptionConfirmed; // ignore: deprecated_member_use
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

class ChatContent extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ChatContent', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.chat'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'text')
    ..hasRequiredFields = false
  ;

  ChatContent._() : super();
  factory ChatContent({
    $core.String? text,
  }) {
    final _result = create();
    if (text != null) {
      _result.text = text;
    }
    return _result;
  }
  factory ChatContent.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChatContent.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChatContent clone() => ChatContent()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChatContent copyWith(void Function(ChatContent) updates) => super.copyWith((message) => updates(message as ChatContent)) as ChatContent; // ignore: deprecated_member_use
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
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'FileContent', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.chat'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fileId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fileName')
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fileExtension')
    ..a<$core.int>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fileSize', $pb.PbFieldType.OU3)
    ..aOS(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fileDescription')
    ..hasRequiredFields = false
  ;

  FileContent._() : super();
  factory FileContent({
    $fixnum.Int64? fileId,
    $core.String? fileName,
    $core.String? fileExtension,
    $core.int? fileSize,
    $core.String? fileDescription,
  }) {
    final _result = create();
    if (fileId != null) {
      _result.fileId = fileId;
    }
    if (fileName != null) {
      _result.fileName = fileName;
    }
    if (fileExtension != null) {
      _result.fileExtension = fileExtension;
    }
    if (fileSize != null) {
      _result.fileSize = fileSize;
    }
    if (fileDescription != null) {
      _result.fileDescription = fileDescription;
    }
    return _result;
  }
  factory FileContent.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FileContent.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  FileContent clone() => FileContent()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  FileContent copyWith(void Function(FileContent) updates) => super.copyWith((message) => updates(message as FileContent)) as FileContent; // ignore: deprecated_member_use
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
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GroupEvent', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.chat'), createEmptyInstance: create)
    ..e<GroupEventType>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'eventType', $pb.PbFieldType.OE, defaultOrMaker: GroupEventType.DEFAULT, valueOf: GroupEventType.valueOf, enumValues: GroupEventType.values)
    ..a<$core.List<$core.int>>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'userId', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  GroupEvent._() : super();
  factory GroupEvent({
    GroupEventType? eventType,
    $core.List<$core.int>? userId,
  }) {
    final _result = create();
    if (eventType != null) {
      _result.eventType = eventType;
    }
    if (userId != null) {
      _result.userId = userId;
    }
    return _result;
  }
  factory GroupEvent.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupEvent.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GroupEvent clone() => GroupEvent()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GroupEvent copyWith(void Function(GroupEvent) updates) => super.copyWith((message) => updates(message as GroupEvent)) as GroupEvent; // ignore: deprecated_member_use
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

