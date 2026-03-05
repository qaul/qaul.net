// This is a generated file - do not edit.
//
// Generated from connections/ble/ble_net.proto.

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

enum BleMessage_Message {
  info,
  feed,
  messaging,
  identification,
  encrypted,
  handshake,
  notSet
}

/// BLE network communication message
class BleMessage extends $pb.GeneratedMessage {
  factory BleMessage({
    $core.List<$core.int>? info,
    $core.List<$core.int>? feed,
    $core.List<$core.int>? messaging,
    Identification? identification,
    EncryptedBleTransport? encrypted,
    NoiseHandshake? handshake,
  }) {
    final result = create();
    if (info != null) result.info = info;
    if (feed != null) result.feed = feed;
    if (messaging != null) result.messaging = messaging;
    if (identification != null) result.identification = identification;
    if (encrypted != null) result.encrypted = encrypted;
    if (handshake != null) result.handshake = handshake;
    return result;
  }

  BleMessage._();

  factory BleMessage.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory BleMessage.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, BleMessage_Message>
      _BleMessage_MessageByTag = {
    1: BleMessage_Message.info,
    2: BleMessage_Message.feed,
    3: BleMessage_Message.messaging,
    4: BleMessage_Message.identification,
    5: BleMessage_Message.encrypted,
    6: BleMessage_Message.handshake,
    0: BleMessage_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BleMessage',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.ble'),
      createEmptyInstance: create)
    ..oo(0, [1, 2, 3, 4, 5, 6])
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'info', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(
        2, _omitFieldNames ? '' : 'feed', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(
        3, _omitFieldNames ? '' : 'messaging', $pb.PbFieldType.OY)
    ..aOM<Identification>(4, _omitFieldNames ? '' : 'identification',
        subBuilder: Identification.create)
    ..aOM<EncryptedBleTransport>(5, _omitFieldNames ? '' : 'encrypted',
        subBuilder: EncryptedBleTransport.create)
    ..aOM<NoiseHandshake>(6, _omitFieldNames ? '' : 'handshake',
        subBuilder: NoiseHandshake.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BleMessage clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BleMessage copyWith(void Function(BleMessage) updates) =>
      super.copyWith((message) => updates(message as BleMessage)) as BleMessage;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static BleMessage create() => BleMessage._();
  @$core.override
  BleMessage createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static BleMessage getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BleMessage>(create);
  static BleMessage? _defaultInstance;

  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  @$pb.TagNumber(6)
  BleMessage_Message whichMessage() =>
      _BleMessage_MessageByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  @$pb.TagNumber(6)
  void clearMessage() => $_clearField($_whichOneof(0));

  /// info message
  @$pb.TagNumber(1)
  $core.List<$core.int> get info => $_getN(0);
  @$pb.TagNumber(1)
  set info($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasInfo() => $_has(0);
  @$pb.TagNumber(1)
  void clearInfo() => $_clearField(1);

  /// feed message
  @$pb.TagNumber(2)
  $core.List<$core.int> get feed => $_getN(1);
  @$pb.TagNumber(2)
  set feed($core.List<$core.int> value) => $_setBytes(1, value);
  @$pb.TagNumber(2)
  $core.bool hasFeed() => $_has(1);
  @$pb.TagNumber(2)
  void clearFeed() => $_clearField(2);

  /// messaging message
  @$pb.TagNumber(3)
  $core.List<$core.int> get messaging => $_getN(2);
  @$pb.TagNumber(3)
  set messaging($core.List<$core.int> value) => $_setBytes(2, value);
  @$pb.TagNumber(3)
  $core.bool hasMessaging() => $_has(2);
  @$pb.TagNumber(3)
  void clearMessaging() => $_clearField(3);

  /// identification request
  @$pb.TagNumber(4)
  Identification get identification => $_getN(3);
  @$pb.TagNumber(4)
  set identification(Identification value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasIdentification() => $_has(3);
  @$pb.TagNumber(4)
  void clearIdentification() => $_clearField(4);
  @$pb.TagNumber(4)
  Identification ensureIdentification() => $_ensure(3);

  /// encrypted transport message
  @$pb.TagNumber(5)
  EncryptedBleTransport get encrypted => $_getN(4);
  @$pb.TagNumber(5)
  set encrypted(EncryptedBleTransport value) => $_setField(5, value);
  @$pb.TagNumber(5)
  $core.bool hasEncrypted() => $_has(4);
  @$pb.TagNumber(5)
  void clearEncrypted() => $_clearField(5);
  @$pb.TagNumber(5)
  EncryptedBleTransport ensureEncrypted() => $_ensure(4);

  /// noise handshake message
  @$pb.TagNumber(6)
  NoiseHandshake get handshake => $_getN(5);
  @$pb.TagNumber(6)
  set handshake(NoiseHandshake value) => $_setField(6, value);
  @$pb.TagNumber(6)
  $core.bool hasHandshake() => $_has(5);
  @$pb.TagNumber(6)
  void clearHandshake() => $_clearField(6);
  @$pb.TagNumber(6)
  NoiseHandshake ensureHandshake() => $_ensure(5);
}

/// Identfication Request
class Identification extends $pb.GeneratedMessage {
  factory Identification({
    $core.bool? request,
    NodeIdentification? node,
  }) {
    final result = create();
    if (request != null) result.request = request;
    if (node != null) result.node = node;
    return result;
  }

  Identification._();

  factory Identification.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory Identification.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Identification',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.ble'),
      createEmptyInstance: create)
    ..aOB(1, _omitFieldNames ? '' : 'request')
    ..aOM<NodeIdentification>(2, _omitFieldNames ? '' : 'node',
        subBuilder: NodeIdentification.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Identification clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Identification copyWith(void Function(Identification) updates) =>
      super.copyWith((message) => updates(message as Identification))
          as Identification;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Identification create() => Identification._();
  @$core.override
  Identification createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static Identification getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<Identification>(create);
  static Identification? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get request => $_getBF(0);
  @$pb.TagNumber(1)
  set request($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasRequest() => $_has(0);
  @$pb.TagNumber(1)
  void clearRequest() => $_clearField(1);

  @$pb.TagNumber(2)
  NodeIdentification get node => $_getN(1);
  @$pb.TagNumber(2)
  set node(NodeIdentification value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasNode() => $_has(1);
  @$pb.TagNumber(2)
  void clearNode() => $_clearField(2);
  @$pb.TagNumber(2)
  NodeIdentification ensureNode() => $_ensure(1);
}

/// Identity Information
class NodeIdentification extends $pb.GeneratedMessage {
  factory NodeIdentification({
    $core.List<$core.int>? id,
  }) {
    final result = create();
    if (id != null) result.id = id;
    return result;
  }

  NodeIdentification._();

  factory NodeIdentification.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory NodeIdentification.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'NodeIdentification',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.ble'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'id', $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NodeIdentification clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NodeIdentification copyWith(void Function(NodeIdentification) updates) =>
      super.copyWith((message) => updates(message as NodeIdentification))
          as NodeIdentification;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NodeIdentification create() => NodeIdentification._();
  @$core.override
  NodeIdentification createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static NodeIdentification getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<NodeIdentification>(create);
  static NodeIdentification? _defaultInstance;

  /// Node ID
  @$pb.TagNumber(1)
  $core.List<$core.int> get id => $_getN(0);
  @$pb.TagNumber(1)
  set id($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

/// Encrypted transport wrapper for BLE messages
class EncryptedBleTransport extends $pb.GeneratedMessage {
  factory EncryptedBleTransport({
    $core.int? sessionId,
    $fixnum.Int64? nonce,
    $core.List<$core.int>? ciphertext,
  }) {
    final result = create();
    if (sessionId != null) result.sessionId = sessionId;
    if (nonce != null) result.nonce = nonce;
    if (ciphertext != null) result.ciphertext = ciphertext;
    return result;
  }

  EncryptedBleTransport._();

  factory EncryptedBleTransport.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory EncryptedBleTransport.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'EncryptedBleTransport',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.ble'),
      createEmptyInstance: create)
    ..aI(1, _omitFieldNames ? '' : 'sessionId', fieldType: $pb.PbFieldType.OU3)
    ..a<$fixnum.Int64>(2, _omitFieldNames ? '' : 'nonce', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.List<$core.int>>(
        3, _omitFieldNames ? '' : 'ciphertext', $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  EncryptedBleTransport clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  EncryptedBleTransport copyWith(
          void Function(EncryptedBleTransport) updates) =>
      super.copyWith((message) => updates(message as EncryptedBleTransport))
          as EncryptedBleTransport;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static EncryptedBleTransport create() => EncryptedBleTransport._();
  @$core.override
  EncryptedBleTransport createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static EncryptedBleTransport getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<EncryptedBleTransport>(create);
  static EncryptedBleTransport? _defaultInstance;

  /// Session identifier
  @$pb.TagNumber(1)
  $core.int get sessionId => $_getIZ(0);
  @$pb.TagNumber(1)
  set sessionId($core.int value) => $_setUnsignedInt32(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSessionId() => $_has(0);
  @$pb.TagNumber(1)
  void clearSessionId() => $_clearField(1);

  /// Message nonce (counter)
  @$pb.TagNumber(2)
  $fixnum.Int64 get nonce => $_getI64(1);
  @$pb.TagNumber(2)
  set nonce($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasNonce() => $_has(1);
  @$pb.TagNumber(2)
  void clearNonce() => $_clearField(2);

  /// Encrypted ciphertext
  @$pb.TagNumber(3)
  $core.List<$core.int> get ciphertext => $_getN(2);
  @$pb.TagNumber(3)
  set ciphertext($core.List<$core.int> value) => $_setBytes(2, value);
  @$pb.TagNumber(3)
  $core.bool hasCiphertext() => $_has(2);
  @$pb.TagNumber(3)
  void clearCiphertext() => $_clearField(3);
}

/// Noise protocol handshake message
class NoiseHandshake extends $pb.GeneratedMessage {
  factory NoiseHandshake({
    $core.int? sessionId,
    $core.int? messageNumber,
    $core.List<$core.int>? payload,
  }) {
    final result = create();
    if (sessionId != null) result.sessionId = sessionId;
    if (messageNumber != null) result.messageNumber = messageNumber;
    if (payload != null) result.payload = payload;
    return result;
  }

  NoiseHandshake._();

  factory NoiseHandshake.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory NoiseHandshake.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'NoiseHandshake',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.ble'),
      createEmptyInstance: create)
    ..aI(1, _omitFieldNames ? '' : 'sessionId', fieldType: $pb.PbFieldType.OU3)
    ..aI(2, _omitFieldNames ? '' : 'messageNumber',
        fieldType: $pb.PbFieldType.OU3)
    ..a<$core.List<$core.int>>(
        3, _omitFieldNames ? '' : 'payload', $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NoiseHandshake clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NoiseHandshake copyWith(void Function(NoiseHandshake) updates) =>
      super.copyWith((message) => updates(message as NoiseHandshake))
          as NoiseHandshake;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NoiseHandshake create() => NoiseHandshake._();
  @$core.override
  NoiseHandshake createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static NoiseHandshake getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<NoiseHandshake>(create);
  static NoiseHandshake? _defaultInstance;

  /// Session identifier
  @$pb.TagNumber(1)
  $core.int get sessionId => $_getIZ(0);
  @$pb.TagNumber(1)
  set sessionId($core.int value) => $_setUnsignedInt32(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSessionId() => $_has(0);
  @$pb.TagNumber(1)
  void clearSessionId() => $_clearField(1);

  /// Handshake message number (1 or 2)
  @$pb.TagNumber(2)
  $core.int get messageNumber => $_getIZ(1);
  @$pb.TagNumber(2)
  set messageNumber($core.int value) => $_setUnsignedInt32(1, value);
  @$pb.TagNumber(2)
  $core.bool hasMessageNumber() => $_has(1);
  @$pb.TagNumber(2)
  void clearMessageNumber() => $_clearField(2);

  /// Handshake payload
  @$pb.TagNumber(3)
  $core.List<$core.int> get payload => $_getN(2);
  @$pb.TagNumber(3)
  set payload($core.List<$core.int> value) => $_setBytes(2, value);
  @$pb.TagNumber(3)
  $core.bool hasPayload() => $_has(2);
  @$pb.TagNumber(3)
  void clearPayload() => $_clearField(3);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
