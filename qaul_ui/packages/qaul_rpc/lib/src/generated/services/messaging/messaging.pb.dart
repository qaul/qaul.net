///
//  Generated code. Do not modify.
//  source: services/messaging/messaging.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

class Container extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Container', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.net.messaging'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'signature', $pb.PbFieldType.OY)
    ..aOM<Envelope>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'envelope', subBuilder: Envelope.create)
    ..hasRequiredFields = false
  ;

  Container._() : super();
  factory Container({
    $core.List<$core.int>? signature,
    Envelope? envelope,
  }) {
    final _result = create();
    if (signature != null) {
      _result.signature = signature;
    }
    if (envelope != null) {
      _result.envelope = envelope;
    }
    return _result;
  }
  factory Container.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Container.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Container clone() => Container()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Container copyWith(void Function(Container) updates) => super.copyWith((message) => updates(message as Container)) as Container; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Container create() => Container._();
  Container createEmptyInstance() => create();
  static $pb.PbList<Container> createRepeated() => $pb.PbList<Container>();
  @$core.pragma('dart2js:noInline')
  static Container getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Container>(create);
  static Container? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get signature => $_getN(0);
  @$pb.TagNumber(1)
  set signature($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasSignature() => $_has(0);
  @$pb.TagNumber(1)
  void clearSignature() => clearField(1);

  @$pb.TagNumber(2)
  Envelope get envelope => $_getN(1);
  @$pb.TagNumber(2)
  set envelope(Envelope v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasEnvelope() => $_has(1);
  @$pb.TagNumber(2)
  void clearEnvelope() => clearField(2);
  @$pb.TagNumber(2)
  Envelope ensureEnvelope() => $_ensure(1);
}

class Envelope extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Envelope', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.net.messaging'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'senderId', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'receiverId', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'data', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  Envelope._() : super();
  factory Envelope({
    $core.List<$core.int>? senderId,
    $core.List<$core.int>? receiverId,
    $core.List<$core.int>? data,
  }) {
    final _result = create();
    if (senderId != null) {
      _result.senderId = senderId;
    }
    if (receiverId != null) {
      _result.receiverId = receiverId;
    }
    if (data != null) {
      _result.data = data;
    }
    return _result;
  }
  factory Envelope.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Envelope.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Envelope clone() => Envelope()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Envelope copyWith(void Function(Envelope) updates) => super.copyWith((message) => updates(message as Envelope)) as Envelope; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Envelope create() => Envelope._();
  Envelope createEmptyInstance() => create();
  static $pb.PbList<Envelope> createRepeated() => $pb.PbList<Envelope>();
  @$core.pragma('dart2js:noInline')
  static Envelope getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Envelope>(create);
  static Envelope? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get senderId => $_getN(0);
  @$pb.TagNumber(1)
  set senderId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasSenderId() => $_has(0);
  @$pb.TagNumber(1)
  void clearSenderId() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get receiverId => $_getN(1);
  @$pb.TagNumber(2)
  set receiverId($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasReceiverId() => $_has(1);
  @$pb.TagNumber(2)
  void clearReceiverId() => clearField(2);

  @$pb.TagNumber(3)
  $core.List<$core.int> get data => $_getN(2);
  @$pb.TagNumber(3)
  set data($core.List<$core.int> v) { $_setBytes(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasData() => $_has(2);
  @$pb.TagNumber(3)
  void clearData() => clearField(3);
}

class Confirmation extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Confirmation', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.net.messaging'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'messageId', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'receivedAt', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  Confirmation._() : super();
  factory Confirmation({
    $core.List<$core.int>? messageId,
    $fixnum.Int64? receivedAt,
  }) {
    final _result = create();
    if (messageId != null) {
      _result.messageId = messageId;
    }
    if (receivedAt != null) {
      _result.receivedAt = receivedAt;
    }
    return _result;
  }
  factory Confirmation.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Confirmation.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Confirmation clone() => Confirmation()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Confirmation copyWith(void Function(Confirmation) updates) => super.copyWith((message) => updates(message as Confirmation)) as Confirmation; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Confirmation create() => Confirmation._();
  Confirmation createEmptyInstance() => create();
  static $pb.PbList<Confirmation> createRepeated() => $pb.PbList<Confirmation>();
  @$core.pragma('dart2js:noInline')
  static Confirmation getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Confirmation>(create);
  static Confirmation? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get messageId => $_getN(0);
  @$pb.TagNumber(1)
  set messageId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasMessageId() => $_has(0);
  @$pb.TagNumber(1)
  void clearMessageId() => clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get receivedAt => $_getI64(1);
  @$pb.TagNumber(2)
  set receivedAt($fixnum.Int64 v) { $_setInt64(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasReceivedAt() => $_has(1);
  @$pb.TagNumber(2)
  void clearReceivedAt() => clearField(2);
}

class ChatMessage extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ChatMessage', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.net.messaging'), createEmptyInstance: create)
    ..aOB(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'group')
    ..a<$core.List<$core.int>>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'conversationId', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'sentAt', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOS(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'content')
    ..hasRequiredFields = false
  ;

  ChatMessage._() : super();
  factory ChatMessage({
    $core.bool? group,
    $core.List<$core.int>? conversationId,
    $fixnum.Int64? sentAt,
    $core.String? content,
  }) {
    final _result = create();
    if (group != null) {
      _result.group = group;
    }
    if (conversationId != null) {
      _result.conversationId = conversationId;
    }
    if (sentAt != null) {
      _result.sentAt = sentAt;
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
  $core.bool get group => $_getBF(0);
  @$pb.TagNumber(1)
  set group($core.bool v) { $_setBool(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroup() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroup() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get conversationId => $_getN(1);
  @$pb.TagNumber(2)
  set conversationId($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasConversationId() => $_has(1);
  @$pb.TagNumber(2)
  void clearConversationId() => clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get sentAt => $_getI64(2);
  @$pb.TagNumber(3)
  set sentAt($fixnum.Int64 v) { $_setInt64(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasSentAt() => $_has(2);
  @$pb.TagNumber(3)
  void clearSentAt() => clearField(3);

  @$pb.TagNumber(4)
  $core.String get content => $_getSZ(3);
  @$pb.TagNumber(4)
  set content($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasContent() => $_has(3);
  @$pb.TagNumber(4)
  void clearContent() => clearField(4);
}

