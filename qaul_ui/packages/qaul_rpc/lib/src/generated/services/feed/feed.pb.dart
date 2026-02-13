// This is a generated file - do not edit.
//
// Generated from services/feed/feed.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

enum Feed_Message { received, send, request, notSet }

/// Feed service RPC message container
class Feed extends $pb.GeneratedMessage {
  factory Feed({
    FeedMessageList? received,
    SendMessage? send,
    FeedMessageRequest? request,
  }) {
    final result = create();
    if (received != null) result.received = received;
    if (send != null) result.send = send;
    if (request != null) result.request = request;
    return result;
  }

  Feed._();

  factory Feed.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory Feed.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, Feed_Message> _Feed_MessageByTag = {
    1: Feed_Message.received,
    2: Feed_Message.send,
    3: Feed_Message.request,
    0: Feed_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Feed',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.feed'),
      createEmptyInstance: create)
    ..oo(0, [1, 2, 3])
    ..aOM<FeedMessageList>(1, _omitFieldNames ? '' : 'received',
        subBuilder: FeedMessageList.create)
    ..aOM<SendMessage>(2, _omitFieldNames ? '' : 'send',
        subBuilder: SendMessage.create)
    ..aOM<FeedMessageRequest>(3, _omitFieldNames ? '' : 'request',
        subBuilder: FeedMessageRequest.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Feed clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Feed copyWith(void Function(Feed) updates) =>
      super.copyWith((message) => updates(message as Feed)) as Feed;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Feed create() => Feed._();
  @$core.override
  Feed createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static Feed getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Feed>(create);
  static Feed? _defaultInstance;

  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  Feed_Message whichMessage() => _Feed_MessageByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  void clearMessage() => $_clearField($_whichOneof(0));

  /// received messages
  @$pb.TagNumber(1)
  FeedMessageList get received => $_getN(0);
  @$pb.TagNumber(1)
  set received(FeedMessageList value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasReceived() => $_has(0);
  @$pb.TagNumber(1)
  void clearReceived() => $_clearField(1);
  @$pb.TagNumber(1)
  FeedMessageList ensureReceived() => $_ensure(0);

  /// send a new feed message
  @$pb.TagNumber(2)
  SendMessage get send => $_getN(1);
  @$pb.TagNumber(2)
  set send(SendMessage value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasSend() => $_has(1);
  @$pb.TagNumber(2)
  void clearSend() => $_clearField(2);
  @$pb.TagNumber(2)
  SendMessage ensureSend() => $_ensure(1);

  /// request received messages
  @$pb.TagNumber(3)
  FeedMessageRequest get request => $_getN(2);
  @$pb.TagNumber(3)
  set request(FeedMessageRequest value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasRequest() => $_has(2);
  @$pb.TagNumber(3)
  void clearRequest() => $_clearField(3);
  @$pb.TagNumber(3)
  FeedMessageRequest ensureRequest() => $_ensure(2);
}

/// request feed messages
class FeedMessageRequest extends $pb.GeneratedMessage {
  factory FeedMessageRequest({
    $core.List<$core.int>? lastReceived,
    $fixnum.Int64? lastIndex,
  }) {
    final result = create();
    if (lastReceived != null) result.lastReceived = lastReceived;
    if (lastIndex != null) result.lastIndex = lastIndex;
    return result;
  }

  FeedMessageRequest._();

  factory FeedMessageRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory FeedMessageRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'FeedMessageRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.feed'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'lastReceived', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(
        2, _omitFieldNames ? '' : 'lastIndex', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  FeedMessageRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  FeedMessageRequest copyWith(void Function(FeedMessageRequest) updates) =>
      super.copyWith((message) => updates(message as FeedMessageRequest))
          as FeedMessageRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FeedMessageRequest create() => FeedMessageRequest._();
  @$core.override
  FeedMessageRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static FeedMessageRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<FeedMessageRequest>(create);
  static FeedMessageRequest? _defaultInstance;

  /// DEPRECATED
  @$pb.TagNumber(1)
  $core.List<$core.int> get lastReceived => $_getN(0);
  @$pb.TagNumber(1)
  set lastReceived($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasLastReceived() => $_has(0);
  @$pb.TagNumber(1)
  void clearLastReceived() => $_clearField(1);

  /// Index of the last message received
  ///
  /// The message index is a continues numbering
  /// of incoming messages in the database of the node.
  ///
  /// When this variable is set, only
  /// newer messages will be sent.
  /// Default value is 0, when the value
  /// is 0, all feed messages will be sent.
  @$pb.TagNumber(2)
  $fixnum.Int64 get lastIndex => $_getI64(1);
  @$pb.TagNumber(2)
  set lastIndex($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasLastIndex() => $_has(1);
  @$pb.TagNumber(2)
  void clearLastIndex() => $_clearField(2);
}

/// List of feed messages
class FeedMessageList extends $pb.GeneratedMessage {
  factory FeedMessageList({
    $core.Iterable<FeedMessage>? feedMessage,
  }) {
    final result = create();
    if (feedMessage != null) result.feedMessage.addAll(feedMessage);
    return result;
  }

  FeedMessageList._();

  factory FeedMessageList.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory FeedMessageList.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'FeedMessageList',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.feed'),
      createEmptyInstance: create)
    ..pPM<FeedMessage>(1, _omitFieldNames ? '' : 'feedMessage',
        subBuilder: FeedMessage.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  FeedMessageList clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  FeedMessageList copyWith(void Function(FeedMessageList) updates) =>
      super.copyWith((message) => updates(message as FeedMessageList))
          as FeedMessageList;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FeedMessageList create() => FeedMessageList._();
  @$core.override
  FeedMessageList createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static FeedMessageList getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<FeedMessageList>(create);
  static FeedMessageList? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<FeedMessage> get feedMessage => $_getList(0);
}

/// A single feed message
class FeedMessage extends $pb.GeneratedMessage {
  factory FeedMessage({
    $core.List<$core.int>? senderId,
    $core.String? senderIdBase58,
    $core.List<$core.int>? messageId,
    $core.String? messageIdBase58,
    $core.String? timeSent,
    $core.String? timeReceived,
    $core.String? content,
    $fixnum.Int64? index,
    $fixnum.Int64? timestampSent,
    $fixnum.Int64? timestampReceived,
  }) {
    final result = create();
    if (senderId != null) result.senderId = senderId;
    if (senderIdBase58 != null) result.senderIdBase58 = senderIdBase58;
    if (messageId != null) result.messageId = messageId;
    if (messageIdBase58 != null) result.messageIdBase58 = messageIdBase58;
    if (timeSent != null) result.timeSent = timeSent;
    if (timeReceived != null) result.timeReceived = timeReceived;
    if (content != null) result.content = content;
    if (index != null) result.index = index;
    if (timestampSent != null) result.timestampSent = timestampSent;
    if (timestampReceived != null) result.timestampReceived = timestampReceived;
    return result;
  }

  FeedMessage._();

  factory FeedMessage.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory FeedMessage.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'FeedMessage',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.feed'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'senderId', $pb.PbFieldType.OY)
    ..aOS(2, _omitFieldNames ? '' : 'senderIdBase58')
    ..a<$core.List<$core.int>>(
        3, _omitFieldNames ? '' : 'messageId', $pb.PbFieldType.OY)
    ..aOS(4, _omitFieldNames ? '' : 'messageIdBase58')
    ..aOS(5, _omitFieldNames ? '' : 'timeSent')
    ..aOS(6, _omitFieldNames ? '' : 'timeReceived')
    ..aOS(7, _omitFieldNames ? '' : 'content')
    ..a<$fixnum.Int64>(8, _omitFieldNames ? '' : 'index', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(
        9, _omitFieldNames ? '' : 'timestampSent', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(
        10, _omitFieldNames ? '' : 'timestampReceived', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  FeedMessage clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  FeedMessage copyWith(void Function(FeedMessage) updates) =>
      super.copyWith((message) => updates(message as FeedMessage))
          as FeedMessage;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FeedMessage create() => FeedMessage._();
  @$core.override
  FeedMessage createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static FeedMessage getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<FeedMessage>(create);
  static FeedMessage? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get senderId => $_getN(0);
  @$pb.TagNumber(1)
  set senderId($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSenderId() => $_has(0);
  @$pb.TagNumber(1)
  void clearSenderId() => $_clearField(1);

  /// DEPRECATED
  @$pb.TagNumber(2)
  $core.String get senderIdBase58 => $_getSZ(1);
  @$pb.TagNumber(2)
  set senderIdBase58($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasSenderIdBase58() => $_has(1);
  @$pb.TagNumber(2)
  void clearSenderIdBase58() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.List<$core.int> get messageId => $_getN(2);
  @$pb.TagNumber(3)
  set messageId($core.List<$core.int> value) => $_setBytes(2, value);
  @$pb.TagNumber(3)
  $core.bool hasMessageId() => $_has(2);
  @$pb.TagNumber(3)
  void clearMessageId() => $_clearField(3);

  /// DEPRECATED
  @$pb.TagNumber(4)
  $core.String get messageIdBase58 => $_getSZ(3);
  @$pb.TagNumber(4)
  set messageIdBase58($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasMessageIdBase58() => $_has(3);
  @$pb.TagNumber(4)
  void clearMessageIdBase58() => $_clearField(4);

  /// DEPRECATED
  @$pb.TagNumber(5)
  $core.String get timeSent => $_getSZ(4);
  @$pb.TagNumber(5)
  set timeSent($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasTimeSent() => $_has(4);
  @$pb.TagNumber(5)
  void clearTimeSent() => $_clearField(5);

  /// DEPRECATED
  @$pb.TagNumber(6)
  $core.String get timeReceived => $_getSZ(5);
  @$pb.TagNumber(6)
  set timeReceived($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasTimeReceived() => $_has(5);
  @$pb.TagNumber(6)
  void clearTimeReceived() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get content => $_getSZ(6);
  @$pb.TagNumber(7)
  set content($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasContent() => $_has(6);
  @$pb.TagNumber(7)
  void clearContent() => $_clearField(7);

  @$pb.TagNumber(8)
  $fixnum.Int64 get index => $_getI64(7);
  @$pb.TagNumber(8)
  set index($fixnum.Int64 value) => $_setInt64(7, value);
  @$pb.TagNumber(8)
  $core.bool hasIndex() => $_has(7);
  @$pb.TagNumber(8)
  void clearIndex() => $_clearField(8);

  @$pb.TagNumber(9)
  $fixnum.Int64 get timestampSent => $_getI64(8);
  @$pb.TagNumber(9)
  set timestampSent($fixnum.Int64 value) => $_setInt64(8, value);
  @$pb.TagNumber(9)
  $core.bool hasTimestampSent() => $_has(8);
  @$pb.TagNumber(9)
  void clearTimestampSent() => $_clearField(9);

  @$pb.TagNumber(10)
  $fixnum.Int64 get timestampReceived => $_getI64(9);
  @$pb.TagNumber(10)
  set timestampReceived($fixnum.Int64 value) => $_setInt64(9, value);
  @$pb.TagNumber(10)
  $core.bool hasTimestampReceived() => $_has(9);
  @$pb.TagNumber(10)
  void clearTimestampReceived() => $_clearField(10);
}

/// send feed message
class SendMessage extends $pb.GeneratedMessage {
  factory SendMessage({
    $core.String? content,
  }) {
    final result = create();
    if (content != null) result.content = content;
    return result;
  }

  SendMessage._();

  factory SendMessage.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory SendMessage.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SendMessage',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.feed'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'content')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SendMessage clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SendMessage copyWith(void Function(SendMessage) updates) =>
      super.copyWith((message) => updates(message as SendMessage))
          as SendMessage;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SendMessage create() => SendMessage._();
  @$core.override
  SendMessage createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static SendMessage getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SendMessage>(create);
  static SendMessage? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get content => $_getSZ(0);
  @$pb.TagNumber(1)
  set content($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasContent() => $_has(0);
  @$pb.TagNumber(1)
  void clearContent() => $_clearField(1);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
