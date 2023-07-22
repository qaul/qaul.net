//
//  Generated code. Do not modify.
//  source: services/feed/feed.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

enum Feed_Message {
  received, 
  send, 
  request, 
  notSet
}

class Feed extends $pb.GeneratedMessage {
  factory Feed() => create();
  Feed._() : super();
  factory Feed.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Feed.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static const $core.Map<$core.int, Feed_Message> _Feed_MessageByTag = {
    1 : Feed_Message.received,
    2 : Feed_Message.send,
    3 : Feed_Message.request,
    0 : Feed_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Feed', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.feed'), createEmptyInstance: create)
    ..oo(0, [1, 2, 3])
    ..aOM<FeedMessageList>(1, _omitFieldNames ? '' : 'received', subBuilder: FeedMessageList.create)
    ..aOM<SendMessage>(2, _omitFieldNames ? '' : 'send', subBuilder: SendMessage.create)
    ..aOM<FeedMessageRequest>(3, _omitFieldNames ? '' : 'request', subBuilder: FeedMessageRequest.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Feed clone() => Feed()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Feed copyWith(void Function(Feed) updates) => super.copyWith((message) => updates(message as Feed)) as Feed;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Feed create() => Feed._();
  Feed createEmptyInstance() => create();
  static $pb.PbList<Feed> createRepeated() => $pb.PbList<Feed>();
  @$core.pragma('dart2js:noInline')
  static Feed getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Feed>(create);
  static Feed? _defaultInstance;

  Feed_Message whichMessage() => _Feed_MessageByTag[$_whichOneof(0)]!;
  void clearMessage() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  FeedMessageList get received => $_getN(0);
  @$pb.TagNumber(1)
  set received(FeedMessageList v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasReceived() => $_has(0);
  @$pb.TagNumber(1)
  void clearReceived() => clearField(1);
  @$pb.TagNumber(1)
  FeedMessageList ensureReceived() => $_ensure(0);

  @$pb.TagNumber(2)
  SendMessage get send => $_getN(1);
  @$pb.TagNumber(2)
  set send(SendMessage v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasSend() => $_has(1);
  @$pb.TagNumber(2)
  void clearSend() => clearField(2);
  @$pb.TagNumber(2)
  SendMessage ensureSend() => $_ensure(1);

  @$pb.TagNumber(3)
  FeedMessageRequest get request => $_getN(2);
  @$pb.TagNumber(3)
  set request(FeedMessageRequest v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasRequest() => $_has(2);
  @$pb.TagNumber(3)
  void clearRequest() => clearField(3);
  @$pb.TagNumber(3)
  FeedMessageRequest ensureRequest() => $_ensure(2);
}

class FeedMessageRequest extends $pb.GeneratedMessage {
  factory FeedMessageRequest() => create();
  FeedMessageRequest._() : super();
  factory FeedMessageRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FeedMessageRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'FeedMessageRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.feed'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'lastReceived', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(2, _omitFieldNames ? '' : 'lastIndex', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  FeedMessageRequest clone() => FeedMessageRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  FeedMessageRequest copyWith(void Function(FeedMessageRequest) updates) => super.copyWith((message) => updates(message as FeedMessageRequest)) as FeedMessageRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FeedMessageRequest create() => FeedMessageRequest._();
  FeedMessageRequest createEmptyInstance() => create();
  static $pb.PbList<FeedMessageRequest> createRepeated() => $pb.PbList<FeedMessageRequest>();
  @$core.pragma('dart2js:noInline')
  static FeedMessageRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FeedMessageRequest>(create);
  static FeedMessageRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get lastReceived => $_getN(0);
  @$pb.TagNumber(1)
  set lastReceived($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasLastReceived() => $_has(0);
  @$pb.TagNumber(1)
  void clearLastReceived() => clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get lastIndex => $_getI64(1);
  @$pb.TagNumber(2)
  set lastIndex($fixnum.Int64 v) { $_setInt64(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasLastIndex() => $_has(1);
  @$pb.TagNumber(2)
  void clearLastIndex() => clearField(2);
}

class FeedMessageList extends $pb.GeneratedMessage {
  factory FeedMessageList() => create();
  FeedMessageList._() : super();
  factory FeedMessageList.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FeedMessageList.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'FeedMessageList', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.feed'), createEmptyInstance: create)
    ..pc<FeedMessage>(1, _omitFieldNames ? '' : 'feedMessage', $pb.PbFieldType.PM, subBuilder: FeedMessage.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  FeedMessageList clone() => FeedMessageList()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  FeedMessageList copyWith(void Function(FeedMessageList) updates) => super.copyWith((message) => updates(message as FeedMessageList)) as FeedMessageList;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FeedMessageList create() => FeedMessageList._();
  FeedMessageList createEmptyInstance() => create();
  static $pb.PbList<FeedMessageList> createRepeated() => $pb.PbList<FeedMessageList>();
  @$core.pragma('dart2js:noInline')
  static FeedMessageList getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FeedMessageList>(create);
  static FeedMessageList? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<FeedMessage> get feedMessage => $_getList(0);
}

class FeedMessage extends $pb.GeneratedMessage {
  factory FeedMessage() => create();
  FeedMessage._() : super();
  factory FeedMessage.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FeedMessage.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'FeedMessage', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.feed'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'senderId', $pb.PbFieldType.OY)
    ..aOS(2, _omitFieldNames ? '' : 'senderIdBase58')
    ..a<$core.List<$core.int>>(3, _omitFieldNames ? '' : 'messageId', $pb.PbFieldType.OY)
    ..aOS(4, _omitFieldNames ? '' : 'messageIdBase58')
    ..aOS(5, _omitFieldNames ? '' : 'timeSent')
    ..aOS(6, _omitFieldNames ? '' : 'timeReceived')
    ..aOS(7, _omitFieldNames ? '' : 'content')
    ..a<$fixnum.Int64>(8, _omitFieldNames ? '' : 'index', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(9, _omitFieldNames ? '' : 'timestampSent', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(10, _omitFieldNames ? '' : 'timestampReceived', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  FeedMessage clone() => FeedMessage()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  FeedMessage copyWith(void Function(FeedMessage) updates) => super.copyWith((message) => updates(message as FeedMessage)) as FeedMessage;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FeedMessage create() => FeedMessage._();
  FeedMessage createEmptyInstance() => create();
  static $pb.PbList<FeedMessage> createRepeated() => $pb.PbList<FeedMessage>();
  @$core.pragma('dart2js:noInline')
  static FeedMessage getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FeedMessage>(create);
  static FeedMessage? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get senderId => $_getN(0);
  @$pb.TagNumber(1)
  set senderId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasSenderId() => $_has(0);
  @$pb.TagNumber(1)
  void clearSenderId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get senderIdBase58 => $_getSZ(1);
  @$pb.TagNumber(2)
  set senderIdBase58($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasSenderIdBase58() => $_has(1);
  @$pb.TagNumber(2)
  void clearSenderIdBase58() => clearField(2);

  @$pb.TagNumber(3)
  $core.List<$core.int> get messageId => $_getN(2);
  @$pb.TagNumber(3)
  set messageId($core.List<$core.int> v) { $_setBytes(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasMessageId() => $_has(2);
  @$pb.TagNumber(3)
  void clearMessageId() => clearField(3);

  @$pb.TagNumber(4)
  $core.String get messageIdBase58 => $_getSZ(3);
  @$pb.TagNumber(4)
  set messageIdBase58($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasMessageIdBase58() => $_has(3);
  @$pb.TagNumber(4)
  void clearMessageIdBase58() => clearField(4);

  @$pb.TagNumber(5)
  $core.String get timeSent => $_getSZ(4);
  @$pb.TagNumber(5)
  set timeSent($core.String v) { $_setString(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasTimeSent() => $_has(4);
  @$pb.TagNumber(5)
  void clearTimeSent() => clearField(5);

  @$pb.TagNumber(6)
  $core.String get timeReceived => $_getSZ(5);
  @$pb.TagNumber(6)
  set timeReceived($core.String v) { $_setString(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasTimeReceived() => $_has(5);
  @$pb.TagNumber(6)
  void clearTimeReceived() => clearField(6);

  @$pb.TagNumber(7)
  $core.String get content => $_getSZ(6);
  @$pb.TagNumber(7)
  set content($core.String v) { $_setString(6, v); }
  @$pb.TagNumber(7)
  $core.bool hasContent() => $_has(6);
  @$pb.TagNumber(7)
  void clearContent() => clearField(7);

  @$pb.TagNumber(8)
  $fixnum.Int64 get index => $_getI64(7);
  @$pb.TagNumber(8)
  set index($fixnum.Int64 v) { $_setInt64(7, v); }
  @$pb.TagNumber(8)
  $core.bool hasIndex() => $_has(7);
  @$pb.TagNumber(8)
  void clearIndex() => clearField(8);

  @$pb.TagNumber(9)
  $fixnum.Int64 get timestampSent => $_getI64(8);
  @$pb.TagNumber(9)
  set timestampSent($fixnum.Int64 v) { $_setInt64(8, v); }
  @$pb.TagNumber(9)
  $core.bool hasTimestampSent() => $_has(8);
  @$pb.TagNumber(9)
  void clearTimestampSent() => clearField(9);

  @$pb.TagNumber(10)
  $fixnum.Int64 get timestampReceived => $_getI64(9);
  @$pb.TagNumber(10)
  set timestampReceived($fixnum.Int64 v) { $_setInt64(9, v); }
  @$pb.TagNumber(10)
  $core.bool hasTimestampReceived() => $_has(9);
  @$pb.TagNumber(10)
  void clearTimestampReceived() => clearField(10);
}

class SendMessage extends $pb.GeneratedMessage {
  factory SendMessage() => create();
  SendMessage._() : super();
  factory SendMessage.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory SendMessage.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'SendMessage', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.feed'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'content')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  SendMessage clone() => SendMessage()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  SendMessage copyWith(void Function(SendMessage) updates) => super.copyWith((message) => updates(message as SendMessage)) as SendMessage;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SendMessage create() => SendMessage._();
  SendMessage createEmptyInstance() => create();
  static $pb.PbList<SendMessage> createRepeated() => $pb.PbList<SendMessage>();
  @$core.pragma('dart2js:noInline')
  static SendMessage getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SendMessage>(create);
  static SendMessage? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get content => $_getSZ(0);
  @$pb.TagNumber(1)
  set content($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasContent() => $_has(0);
  @$pb.TagNumber(1)
  void clearContent() => clearField(1);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
