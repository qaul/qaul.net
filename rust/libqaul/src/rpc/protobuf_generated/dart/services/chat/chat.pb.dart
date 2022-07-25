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
  chatGroupRequest, 
  chatGroupList, 
  notSet
}

class Chat extends $pb.GeneratedMessage {
  static const $core.Map<$core.int, Chat_Message> _Chat_MessageByTag = {
    1 : Chat_Message.overviewRequest,
    2 : Chat_Message.overviewList,
    3 : Chat_Message.conversationRequest,
    4 : Chat_Message.conversationList,
    5 : Chat_Message.send,
    6 : Chat_Message.chatGroupRequest,
    7 : Chat_Message.chatGroupList,
    0 : Chat_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Chat', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.chat'), createEmptyInstance: create)
    ..oo(0, [1, 2, 3, 4, 5, 6, 7])
    ..aOM<ChatOverviewRequest>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'overviewRequest', subBuilder: ChatOverviewRequest.create)
    ..aOM<ChatOverviewList>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'overviewList', subBuilder: ChatOverviewList.create)
    ..aOM<ChatConversationRequest>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'conversationRequest', subBuilder: ChatConversationRequest.create)
    ..aOM<ChatConversationList>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'conversationList', subBuilder: ChatConversationList.create)
    ..aOM<ChatMessageSend>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'send', subBuilder: ChatMessageSend.create)
    ..aOM<ChatGroupRequest>(6, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'chatGroupRequest', subBuilder: ChatGroupRequest.create)
    ..aOM<ChatGroupList>(7, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'chatGroupList', subBuilder: ChatGroupList.create)
    ..hasRequiredFields = false
  ;

  Chat._() : super();
  factory Chat({
    ChatOverviewRequest? overviewRequest,
    ChatOverviewList? overviewList,
    ChatConversationRequest? conversationRequest,
    ChatConversationList? conversationList,
    ChatMessageSend? send,
    ChatGroupRequest? chatGroupRequest,
    ChatGroupList? chatGroupList,
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
    if (chatGroupRequest != null) {
      _result.chatGroupRequest = chatGroupRequest;
    }
    if (chatGroupList != null) {
      _result.chatGroupList = chatGroupList;
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

  @$pb.TagNumber(6)
  ChatGroupRequest get chatGroupRequest => $_getN(5);
  @$pb.TagNumber(6)
  set chatGroupRequest(ChatGroupRequest v) { setField(6, v); }
  @$pb.TagNumber(6)
  $core.bool hasChatGroupRequest() => $_has(5);
  @$pb.TagNumber(6)
  void clearChatGroupRequest() => clearField(6);
  @$pb.TagNumber(6)
  ChatGroupRequest ensureChatGroupRequest() => $_ensure(5);

  @$pb.TagNumber(7)
  ChatGroupList get chatGroupList => $_getN(6);
  @$pb.TagNumber(7)
  set chatGroupList(ChatGroupList v) { setField(7, v); }
  @$pb.TagNumber(7)
  $core.bool hasChatGroupList() => $_has(6);
  @$pb.TagNumber(7)
  void clearChatGroupList() => clearField(7);
  @$pb.TagNumber(7)
  ChatGroupList ensureChatGroupList() => $_ensure(6);
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
    ..a<$core.int>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'lastMessageIndex', $pb.PbFieldType.OU3)
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'name')
    ..a<$fixnum.Int64>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'lastMessageAt', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.int>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'unread', $pb.PbFieldType.O3)
    ..a<$core.List<$core.int>>(6, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'content', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(7, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'lastMessageSenderId', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  ChatOverview._() : super();
  factory ChatOverview({
    $core.List<$core.int>? conversationId,
    $core.int? lastMessageIndex,
    $core.String? name,
    $fixnum.Int64? lastMessageAt,
    $core.int? unread,
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
  $core.List<$core.int> get content => $_getN(5);
  @$pb.TagNumber(6)
  set content($core.List<$core.int> v) { $_setBytes(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasContent() => $_has(5);
  @$pb.TagNumber(6)
  void clearContent() => clearField(6);

  @$pb.TagNumber(7)
  $core.List<$core.int> get lastMessageSenderId => $_getN(6);
  @$pb.TagNumber(7)
  set lastMessageSenderId($core.List<$core.int> v) { $_setBytes(6, v); }
  @$pb.TagNumber(7)
  $core.bool hasLastMessageSenderId() => $_has(6);
  @$pb.TagNumber(7)
  void clearLastMessageSenderId() => clearField(7);
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

class ChatGroupRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ChatGroupRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.chat'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupId', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'lastIndex', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  ChatGroupRequest._() : super();
  factory ChatGroupRequest({
    $core.List<$core.int>? groupId,
    $fixnum.Int64? lastIndex,
  }) {
    final _result = create();
    if (groupId != null) {
      _result.groupId = groupId;
    }
    if (lastIndex != null) {
      _result.lastIndex = lastIndex;
    }
    return _result;
  }
  factory ChatGroupRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChatGroupRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChatGroupRequest clone() => ChatGroupRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChatGroupRequest copyWith(void Function(ChatGroupRequest) updates) => super.copyWith((message) => updates(message as ChatGroupRequest)) as ChatGroupRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChatGroupRequest create() => ChatGroupRequest._();
  ChatGroupRequest createEmptyInstance() => create();
  static $pb.PbList<ChatGroupRequest> createRepeated() => $pb.PbList<ChatGroupRequest>();
  @$core.pragma('dart2js:noInline')
  static ChatGroupRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChatGroupRequest>(create);
  static ChatGroupRequest? _defaultInstance;

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

class ChatGroupList extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ChatGroupList', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.chat'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupId', $pb.PbFieldType.OY)
    ..pc<ChatMessage>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'messageList', $pb.PbFieldType.PM, subBuilder: ChatMessage.create)
    ..hasRequiredFields = false
  ;

  ChatGroupList._() : super();
  factory ChatGroupList({
    $core.List<$core.int>? groupId,
    $core.Iterable<ChatMessage>? messageList,
  }) {
    final _result = create();
    if (groupId != null) {
      _result.groupId = groupId;
    }
    if (messageList != null) {
      _result.messageList.addAll(messageList);
    }
    return _result;
  }
  factory ChatGroupList.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChatGroupList.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChatGroupList clone() => ChatGroupList()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChatGroupList copyWith(void Function(ChatGroupList) updates) => super.copyWith((message) => updates(message as ChatGroupList)) as ChatGroupList; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChatGroupList create() => ChatGroupList._();
  ChatGroupList createEmptyInstance() => create();
  static $pb.PbList<ChatGroupList> createRepeated() => $pb.PbList<ChatGroupList>();
  @$core.pragma('dart2js:noInline')
  static ChatGroupList getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChatGroupList>(create);
  static ChatGroupList? _defaultInstance;

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
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ChatMessage', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.chat'), createEmptyInstance: create)
    ..a<$core.int>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'index', $pb.PbFieldType.OU3)
    ..a<$core.List<$core.int>>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'senderId', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'messageId', $pb.PbFieldType.OY)
    ..a<$core.int>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'status', $pb.PbFieldType.OU3)
    ..aOB(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'isGroup')
    ..a<$core.List<$core.int>>(6, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'conversationId', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(7, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'sentAt', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(8, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'receivedAt', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.List<$core.int>>(9, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'content', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  ChatMessage._() : super();
  factory ChatMessage({
    $core.int? index,
    $core.List<$core.int>? senderId,
    $core.List<$core.int>? messageId,
    $core.int? status,
    $core.bool? isGroup,
    $core.List<$core.int>? conversationId,
    $fixnum.Int64? sentAt,
    $fixnum.Int64? receivedAt,
    $core.List<$core.int>? content,
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
    if (isGroup != null) {
      _result.isGroup = isGroup;
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
  set status($core.int v) { $_setUnsignedInt32(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasStatus() => $_has(3);
  @$pb.TagNumber(4)
  void clearStatus() => clearField(4);

  @$pb.TagNumber(5)
  $core.bool get isGroup => $_getBF(4);
  @$pb.TagNumber(5)
  set isGroup($core.bool v) { $_setBool(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasIsGroup() => $_has(4);
  @$pb.TagNumber(5)
  void clearIsGroup() => clearField(5);

  @$pb.TagNumber(6)
  $core.List<$core.int> get conversationId => $_getN(5);
  @$pb.TagNumber(6)
  set conversationId($core.List<$core.int> v) { $_setBytes(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasConversationId() => $_has(5);
  @$pb.TagNumber(6)
  void clearConversationId() => clearField(6);

  @$pb.TagNumber(7)
  $fixnum.Int64 get sentAt => $_getI64(6);
  @$pb.TagNumber(7)
  set sentAt($fixnum.Int64 v) { $_setInt64(6, v); }
  @$pb.TagNumber(7)
  $core.bool hasSentAt() => $_has(6);
  @$pb.TagNumber(7)
  void clearSentAt() => clearField(7);

  @$pb.TagNumber(8)
  $fixnum.Int64 get receivedAt => $_getI64(7);
  @$pb.TagNumber(8)
  set receivedAt($fixnum.Int64 v) { $_setInt64(7, v); }
  @$pb.TagNumber(8)
  $core.bool hasReceivedAt() => $_has(7);
  @$pb.TagNumber(8)
  void clearReceivedAt() => clearField(8);

  @$pb.TagNumber(9)
  $core.List<$core.int> get content => $_getN(8);
  @$pb.TagNumber(9)
  set content($core.List<$core.int> v) { $_setBytes(8, v); }
  @$pb.TagNumber(9)
  $core.bool hasContent() => $_has(8);
  @$pb.TagNumber(9)
  void clearContent() => clearField(9);
}

enum ChatMessageContent_Content {
  chatContent, 
  fileContent, 
  groupInviteContent, 
  groupInviteReplyContent, 
  notSet
}

class ChatMessageContent extends $pb.GeneratedMessage {
  static const $core.Map<$core.int, ChatMessageContent_Content> _ChatMessageContent_ContentByTag = {
    1 : ChatMessageContent_Content.chatContent,
    2 : ChatMessageContent_Content.fileContent,
    3 : ChatMessageContent_Content.groupInviteContent,
    4 : ChatMessageContent_Content.groupInviteReplyContent,
    0 : ChatMessageContent_Content.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ChatMessageContent', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.chat'), createEmptyInstance: create)
    ..oo(0, [1, 2, 3, 4])
    ..aOM<ChatContent>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'chatContent', subBuilder: ChatContent.create)
    ..aOM<FileShareContent>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fileContent', subBuilder: FileShareContent.create)
    ..aOM<GroupInviteContent>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupInviteContent', subBuilder: GroupInviteContent.create)
    ..aOM<GroupInviteReplyContent>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupInviteReplyContent', subBuilder: GroupInviteReplyContent.create)
    ..hasRequiredFields = false
  ;

  ChatMessageContent._() : super();
  factory ChatMessageContent({
    ChatContent? chatContent,
    FileShareContent? fileContent,
    GroupInviteContent? groupInviteContent,
    GroupInviteReplyContent? groupInviteReplyContent,
  }) {
    final _result = create();
    if (chatContent != null) {
      _result.chatContent = chatContent;
    }
    if (fileContent != null) {
      _result.fileContent = fileContent;
    }
    if (groupInviteContent != null) {
      _result.groupInviteContent = groupInviteContent;
    }
    if (groupInviteReplyContent != null) {
      _result.groupInviteReplyContent = groupInviteReplyContent;
    }
    return _result;
  }
  factory ChatMessageContent.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChatMessageContent.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChatMessageContent clone() => ChatMessageContent()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChatMessageContent copyWith(void Function(ChatMessageContent) updates) => super.copyWith((message) => updates(message as ChatMessageContent)) as ChatMessageContent; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChatMessageContent create() => ChatMessageContent._();
  ChatMessageContent createEmptyInstance() => create();
  static $pb.PbList<ChatMessageContent> createRepeated() => $pb.PbList<ChatMessageContent>();
  @$core.pragma('dart2js:noInline')
  static ChatMessageContent getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChatMessageContent>(create);
  static ChatMessageContent? _defaultInstance;

  ChatMessageContent_Content whichContent() => _ChatMessageContent_ContentByTag[$_whichOneof(0)]!;
  void clearContent() => clearField($_whichOneof(0));

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
  FileShareContent get fileContent => $_getN(1);
  @$pb.TagNumber(2)
  set fileContent(FileShareContent v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasFileContent() => $_has(1);
  @$pb.TagNumber(2)
  void clearFileContent() => clearField(2);
  @$pb.TagNumber(2)
  FileShareContent ensureFileContent() => $_ensure(1);

  @$pb.TagNumber(3)
  GroupInviteContent get groupInviteContent => $_getN(2);
  @$pb.TagNumber(3)
  set groupInviteContent(GroupInviteContent v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasGroupInviteContent() => $_has(2);
  @$pb.TagNumber(3)
  void clearGroupInviteContent() => clearField(3);
  @$pb.TagNumber(3)
  GroupInviteContent ensureGroupInviteContent() => $_ensure(2);

  @$pb.TagNumber(4)
  GroupInviteReplyContent get groupInviteReplyContent => $_getN(3);
  @$pb.TagNumber(4)
  set groupInviteReplyContent(GroupInviteReplyContent v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasGroupInviteReplyContent() => $_has(3);
  @$pb.TagNumber(4)
  void clearGroupInviteReplyContent() => clearField(4);
  @$pb.TagNumber(4)
  GroupInviteReplyContent ensureGroupInviteReplyContent() => $_ensure(3);
}

class ChatContent extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ChatContent', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.chat'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'content')
    ..hasRequiredFields = false
  ;

  ChatContent._() : super();
  factory ChatContent({
    $core.String? content,
  }) {
    final _result = create();
    if (content != null) {
      _result.content = content;
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
  $core.String get content => $_getSZ(0);
  @$pb.TagNumber(1)
  set content($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasContent() => $_has(0);
  @$pb.TagNumber(1)
  void clearContent() => clearField(1);
}

class FileShareContent extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'FileShareContent', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.chat'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'historyIndex', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fileId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fileName')
    ..a<$core.int>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fileSize', $pb.PbFieldType.OU3)
    ..aOS(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fileDescr')
    ..hasRequiredFields = false
  ;

  FileShareContent._() : super();
  factory FileShareContent({
    $fixnum.Int64? historyIndex,
    $fixnum.Int64? fileId,
    $core.String? fileName,
    $core.int? fileSize,
    $core.String? fileDescr,
  }) {
    final _result = create();
    if (historyIndex != null) {
      _result.historyIndex = historyIndex;
    }
    if (fileId != null) {
      _result.fileId = fileId;
    }
    if (fileName != null) {
      _result.fileName = fileName;
    }
    if (fileSize != null) {
      _result.fileSize = fileSize;
    }
    if (fileDescr != null) {
      _result.fileDescr = fileDescr;
    }
    return _result;
  }
  factory FileShareContent.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FileShareContent.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  FileShareContent clone() => FileShareContent()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  FileShareContent copyWith(void Function(FileShareContent) updates) => super.copyWith((message) => updates(message as FileShareContent)) as FileShareContent; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static FileShareContent create() => FileShareContent._();
  FileShareContent createEmptyInstance() => create();
  static $pb.PbList<FileShareContent> createRepeated() => $pb.PbList<FileShareContent>();
  @$core.pragma('dart2js:noInline')
  static FileShareContent getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FileShareContent>(create);
  static FileShareContent? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get historyIndex => $_getI64(0);
  @$pb.TagNumber(1)
  set historyIndex($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasHistoryIndex() => $_has(0);
  @$pb.TagNumber(1)
  void clearHistoryIndex() => clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get fileId => $_getI64(1);
  @$pb.TagNumber(2)
  set fileId($fixnum.Int64 v) { $_setInt64(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasFileId() => $_has(1);
  @$pb.TagNumber(2)
  void clearFileId() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get fileName => $_getSZ(2);
  @$pb.TagNumber(3)
  set fileName($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasFileName() => $_has(2);
  @$pb.TagNumber(3)
  void clearFileName() => clearField(3);

  @$pb.TagNumber(4)
  $core.int get fileSize => $_getIZ(3);
  @$pb.TagNumber(4)
  set fileSize($core.int v) { $_setUnsignedInt32(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasFileSize() => $_has(3);
  @$pb.TagNumber(4)
  void clearFileSize() => clearField(4);

  @$pb.TagNumber(5)
  $core.String get fileDescr => $_getSZ(4);
  @$pb.TagNumber(5)
  set fileDescr($core.String v) { $_setString(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasFileDescr() => $_has(4);
  @$pb.TagNumber(5)
  void clearFileDescr() => clearField(5);
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

class GroupInviteContent extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GroupInviteContent', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.chat'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupId', $pb.PbFieldType.OY)
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupName')
    ..a<$fixnum.Int64>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'createdAt', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.int>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'memberCount', $pb.PbFieldType.OU3)
    ..a<$core.List<$core.int>>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'adminId', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  GroupInviteContent._() : super();
  factory GroupInviteContent({
    $core.List<$core.int>? groupId,
    $core.String? groupName,
    $fixnum.Int64? createdAt,
    $core.int? memberCount,
    $core.List<$core.int>? adminId,
  }) {
    final _result = create();
    if (groupId != null) {
      _result.groupId = groupId;
    }
    if (groupName != null) {
      _result.groupName = groupName;
    }
    if (createdAt != null) {
      _result.createdAt = createdAt;
    }
    if (memberCount != null) {
      _result.memberCount = memberCount;
    }
    if (adminId != null) {
      _result.adminId = adminId;
    }
    return _result;
  }
  factory GroupInviteContent.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupInviteContent.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GroupInviteContent clone() => GroupInviteContent()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GroupInviteContent copyWith(void Function(GroupInviteContent) updates) => super.copyWith((message) => updates(message as GroupInviteContent)) as GroupInviteContent; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupInviteContent create() => GroupInviteContent._();
  GroupInviteContent createEmptyInstance() => create();
  static $pb.PbList<GroupInviteContent> createRepeated() => $pb.PbList<GroupInviteContent>();
  @$core.pragma('dart2js:noInline')
  static GroupInviteContent getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupInviteContent>(create);
  static GroupInviteContent? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get groupId => $_getN(0);
  @$pb.TagNumber(1)
  set groupId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get groupName => $_getSZ(1);
  @$pb.TagNumber(2)
  set groupName($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasGroupName() => $_has(1);
  @$pb.TagNumber(2)
  void clearGroupName() => clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get createdAt => $_getI64(2);
  @$pb.TagNumber(3)
  set createdAt($fixnum.Int64 v) { $_setInt64(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasCreatedAt() => $_has(2);
  @$pb.TagNumber(3)
  void clearCreatedAt() => clearField(3);

  @$pb.TagNumber(4)
  $core.int get memberCount => $_getIZ(3);
  @$pb.TagNumber(4)
  set memberCount($core.int v) { $_setUnsignedInt32(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasMemberCount() => $_has(3);
  @$pb.TagNumber(4)
  void clearMemberCount() => clearField(4);

  @$pb.TagNumber(5)
  $core.List<$core.int> get adminId => $_getN(4);
  @$pb.TagNumber(5)
  set adminId($core.List<$core.int> v) { $_setBytes(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasAdminId() => $_has(4);
  @$pb.TagNumber(5)
  void clearAdminId() => clearField(5);
}

class GroupInviteReplyContent extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GroupInviteReplyContent', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.chat'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupId', $pb.PbFieldType.OY)
    ..aOB(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'accept')
    ..hasRequiredFields = false
  ;

  GroupInviteReplyContent._() : super();
  factory GroupInviteReplyContent({
    $core.List<$core.int>? groupId,
    $core.bool? accept,
  }) {
    final _result = create();
    if (groupId != null) {
      _result.groupId = groupId;
    }
    if (accept != null) {
      _result.accept = accept;
    }
    return _result;
  }
  factory GroupInviteReplyContent.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupInviteReplyContent.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GroupInviteReplyContent clone() => GroupInviteReplyContent()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GroupInviteReplyContent copyWith(void Function(GroupInviteReplyContent) updates) => super.copyWith((message) => updates(message as GroupInviteReplyContent)) as GroupInviteReplyContent; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupInviteReplyContent create() => GroupInviteReplyContent._();
  GroupInviteReplyContent createEmptyInstance() => create();
  static $pb.PbList<GroupInviteReplyContent> createRepeated() => $pb.PbList<GroupInviteReplyContent>();
  @$core.pragma('dart2js:noInline')
  static GroupInviteReplyContent getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupInviteReplyContent>(create);
  static GroupInviteReplyContent? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get groupId => $_getN(0);
  @$pb.TagNumber(1)
  set groupId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => clearField(1);

  @$pb.TagNumber(2)
  $core.bool get accept => $_getBF(1);
  @$pb.TagNumber(2)
  set accept($core.bool v) { $_setBool(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasAccept() => $_has(1);
  @$pb.TagNumber(2)
  void clearAccept() => clearField(2);
}

