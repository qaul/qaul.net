// This is a generated file - do not edit.
//
// Generated from services/messaging/messaging.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'messaging.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'messaging.pbenum.dart';

/// qaul network messaging service
///
/// is responsible to distribute messages to users
/// the container contains the entire message with signature
class Container extends $pb.GeneratedMessage {
  factory Container({
    $core.List<$core.int>? signature,
    Envelope? envelope,
  }) {
    final result = create();
    if (signature != null) result.signature = signature;
    if (envelope != null) result.envelope = envelope;
    return result;
  }

  Container._();

  factory Container.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory Container.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Container',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.messaging'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'signature', $pb.PbFieldType.OY)
    ..aOM<Envelope>(2, _omitFieldNames ? '' : 'envelope',
        subBuilder: Envelope.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Container clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Container copyWith(void Function(Container) updates) =>
      super.copyWith((message) => updates(message as Container)) as Container;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Container create() => Container._();
  @$core.override
  Container createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static Container getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Container>(create);
  static Container? _defaultInstance;

  /// signed by sending user
  @$pb.TagNumber(1)
  $core.List<$core.int> get signature => $_getN(0);
  @$pb.TagNumber(1)
  set signature($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSignature() => $_has(0);
  @$pb.TagNumber(1)
  void clearSignature() => $_clearField(1);

  /// Message envelope
  @$pb.TagNumber(2)
  Envelope get envelope => $_getN(1);
  @$pb.TagNumber(2)
  set envelope(Envelope value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasEnvelope() => $_has(1);
  @$pb.TagNumber(2)
  void clearEnvelope() => $_clearField(2);
  @$pb.TagNumber(2)
  Envelope ensureEnvelope() => $_ensure(1);
}

/// message envelop with sender and receiver
class Envelope extends $pb.GeneratedMessage {
  factory Envelope({
    $core.List<$core.int>? senderId,
    $core.List<$core.int>? receiverId,
    $core.List<$core.int>? payload,
  }) {
    final result = create();
    if (senderId != null) result.senderId = senderId;
    if (receiverId != null) result.receiverId = receiverId;
    if (payload != null) result.payload = payload;
    return result;
  }

  Envelope._();

  factory Envelope.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory Envelope.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Envelope',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.messaging'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'senderId', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(
        2, _omitFieldNames ? '' : 'receiverId', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(
        3, _omitFieldNames ? '' : 'payload', $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Envelope clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Envelope copyWith(void Function(Envelope) updates) =>
      super.copyWith((message) => updates(message as Envelope)) as Envelope;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Envelope create() => Envelope._();
  @$core.override
  Envelope createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static Envelope getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Envelope>(create);
  static Envelope? _defaultInstance;

  /// the qaul ID of the sender
  @$pb.TagNumber(1)
  $core.List<$core.int> get senderId => $_getN(0);
  @$pb.TagNumber(1)
  set senderId($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSenderId() => $_has(0);
  @$pb.TagNumber(1)
  void clearSenderId() => $_clearField(1);

  /// the qaul ID of the receiver
  @$pb.TagNumber(2)
  $core.List<$core.int> get receiverId => $_getN(1);
  @$pb.TagNumber(2)
  set receiverId($core.List<$core.int> value) => $_setBytes(1, value);
  @$pb.TagNumber(2)
  $core.bool hasReceiverId() => $_has(1);
  @$pb.TagNumber(2)
  void clearReceiverId() => $_clearField(2);

  /// payload
  @$pb.TagNumber(3)
  $core.List<$core.int> get payload => $_getN(2);
  @$pb.TagNumber(3)
  set payload($core.List<$core.int> value) => $_setBytes(2, value);
  @$pb.TagNumber(3)
  $core.bool hasPayload() => $_has(2);
  @$pb.TagNumber(3)
  void clearPayload() => $_clearField(3);
}

enum EnvelopPayload_Payload { encrypted, dtn, notSet }

/// envelop payload
class EnvelopPayload extends $pb.GeneratedMessage {
  factory EnvelopPayload({
    Encrypted? encrypted,
    $core.List<$core.int>? dtn,
  }) {
    final result = create();
    if (encrypted != null) result.encrypted = encrypted;
    if (dtn != null) result.dtn = dtn;
    return result;
  }

  EnvelopPayload._();

  factory EnvelopPayload.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory EnvelopPayload.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, EnvelopPayload_Payload>
      _EnvelopPayload_PayloadByTag = {
    1: EnvelopPayload_Payload.encrypted,
    2: EnvelopPayload_Payload.dtn,
    0: EnvelopPayload_Payload.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'EnvelopPayload',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.messaging'),
      createEmptyInstance: create)
    ..oo(0, [1, 2])
    ..aOM<Encrypted>(1, _omitFieldNames ? '' : 'encrypted',
        subBuilder: Encrypted.create)
    ..a<$core.List<$core.int>>(
        2, _omitFieldNames ? '' : 'dtn', $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  EnvelopPayload clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  EnvelopPayload copyWith(void Function(EnvelopPayload) updates) =>
      super.copyWith((message) => updates(message as EnvelopPayload))
          as EnvelopPayload;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static EnvelopPayload create() => EnvelopPayload._();
  @$core.override
  EnvelopPayload createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static EnvelopPayload getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<EnvelopPayload>(create);
  static EnvelopPayload? _defaultInstance;

  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  EnvelopPayload_Payload whichPayload() =>
      _EnvelopPayload_PayloadByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  void clearPayload() => $_clearField($_whichOneof(0));

  /// encrypted message data
  @$pb.TagNumber(1)
  Encrypted get encrypted => $_getN(0);
  @$pb.TagNumber(1)
  set encrypted(Encrypted value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasEncrypted() => $_has(0);
  @$pb.TagNumber(1)
  void clearEncrypted() => $_clearField(1);
  @$pb.TagNumber(1)
  Encrypted ensureEncrypted() => $_ensure(0);

  /// DTN message
  @$pb.TagNumber(2)
  $core.List<$core.int> get dtn => $_getN(1);
  @$pb.TagNumber(2)
  set dtn($core.List<$core.int> value) => $_setBytes(1, value);
  @$pb.TagNumber(2)
  $core.bool hasDtn() => $_has(1);
  @$pb.TagNumber(2)
  void clearDtn() => $_clearField(2);
}

/// encrypted message data
class Encrypted extends $pb.GeneratedMessage {
  factory Encrypted({
    CryptoState? state,
    $core.int? sessionId,
    $core.Iterable<Data>? data,
  }) {
    final result = create();
    if (state != null) result.state = state;
    if (sessionId != null) result.sessionId = sessionId;
    if (data != null) result.data.addAll(data);
    return result;
  }

  Encrypted._();

  factory Encrypted.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory Encrypted.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Encrypted',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.messaging'),
      createEmptyInstance: create)
    ..aE<CryptoState>(1, _omitFieldNames ? '' : 'state',
        enumValues: CryptoState.values)
    ..aI(2, _omitFieldNames ? '' : 'sessionId', fieldType: $pb.PbFieldType.OU3)
    ..pPM<Data>(3, _omitFieldNames ? '' : 'data', subBuilder: Data.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Encrypted clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Encrypted copyWith(void Function(Encrypted) updates) =>
      super.copyWith((message) => updates(message as Encrypted)) as Encrypted;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Encrypted create() => Encrypted._();
  @$core.override
  Encrypted createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static Encrypted getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Encrypted>(create);
  static Encrypted? _defaultInstance;

  /// state of the crypto session
  @$pb.TagNumber(1)
  CryptoState get state => $_getN(0);
  @$pb.TagNumber(1)
  set state(CryptoState value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasState() => $_has(0);
  @$pb.TagNumber(1)
  void clearState() => $_clearField(1);

  /// crypto session id
  @$pb.TagNumber(2)
  $core.int get sessionId => $_getIZ(1);
  @$pb.TagNumber(2)
  set sessionId($core.int value) => $_setUnsignedInt32(1, value);
  @$pb.TagNumber(2)
  $core.bool hasSessionId() => $_has(1);
  @$pb.TagNumber(2)
  void clearSessionId() => $_clearField(2);

  /// one or several Data messages
  /// of maximally 64KB each.
  @$pb.TagNumber(3)
  $pb.PbList<Data> get data => $_getList(2);
}

/// encrypted message data
class Data extends $pb.GeneratedMessage {
  factory Data({
    $fixnum.Int64? nonce,
    $core.List<$core.int>? data,
  }) {
    final result = create();
    if (nonce != null) result.nonce = nonce;
    if (data != null) result.data = data;
    return result;
  }

  Data._();

  factory Data.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory Data.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Data',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.messaging'),
      createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, _omitFieldNames ? '' : 'nonce', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.List<$core.int>>(
        2, _omitFieldNames ? '' : 'data', $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Data clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Data copyWith(void Function(Data) updates) =>
      super.copyWith((message) => updates(message as Data)) as Data;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Data create() => Data._();
  @$core.override
  Data createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static Data getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Data>(create);
  static Data? _defaultInstance;

  /// message nonce for encryption
  ///
  /// each nonce is only used once per key
  /// and increases by one fore each new data package.
  @$pb.TagNumber(1)
  $fixnum.Int64 get nonce => $_getI64(0);
  @$pb.TagNumber(1)
  set nonce($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasNonce() => $_has(0);
  @$pb.TagNumber(1)
  void clearNonce() => $_clearField(1);

  /// the encrypted message data slice
  /// each data package contains maximally
  /// 64KB
  @$pb.TagNumber(2)
  $core.List<$core.int> get data => $_getN(1);
  @$pb.TagNumber(2)
  set data($core.List<$core.int> value) => $_setBytes(1, value);
  @$pb.TagNumber(2)
  $core.bool hasData() => $_has(1);
  @$pb.TagNumber(2)
  void clearData() => $_clearField(2);
}

enum Messaging_Message {
  confirmationMessage,
  dtnResponse,
  cryptoService,
  rtcStreamMessage,
  groupInviteMessage,
  commonMessage,
  notSet
}

/// messaging unified message
class Messaging extends $pb.GeneratedMessage {
  factory Messaging({
    Confirmation? confirmationMessage,
    DtnResponse? dtnResponse,
    CryptoService? cryptoService,
    RtcStreamMessage? rtcStreamMessage,
    GroupInviteMessage? groupInviteMessage,
    CommonMessage? commonMessage,
  }) {
    final result = create();
    if (confirmationMessage != null)
      result.confirmationMessage = confirmationMessage;
    if (dtnResponse != null) result.dtnResponse = dtnResponse;
    if (cryptoService != null) result.cryptoService = cryptoService;
    if (rtcStreamMessage != null) result.rtcStreamMessage = rtcStreamMessage;
    if (groupInviteMessage != null)
      result.groupInviteMessage = groupInviteMessage;
    if (commonMessage != null) result.commonMessage = commonMessage;
    return result;
  }

  Messaging._();

  factory Messaging.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory Messaging.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, Messaging_Message> _Messaging_MessageByTag =
      {
    1: Messaging_Message.confirmationMessage,
    2: Messaging_Message.dtnResponse,
    3: Messaging_Message.cryptoService,
    4: Messaging_Message.rtcStreamMessage,
    5: Messaging_Message.groupInviteMessage,
    6: Messaging_Message.commonMessage,
    0: Messaging_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Messaging',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.messaging'),
      createEmptyInstance: create)
    ..oo(0, [1, 2, 3, 4, 5, 6])
    ..aOM<Confirmation>(1, _omitFieldNames ? '' : 'confirmationMessage',
        subBuilder: Confirmation.create)
    ..aOM<DtnResponse>(2, _omitFieldNames ? '' : 'dtnResponse',
        subBuilder: DtnResponse.create)
    ..aOM<CryptoService>(3, _omitFieldNames ? '' : 'cryptoService',
        subBuilder: CryptoService.create)
    ..aOM<RtcStreamMessage>(4, _omitFieldNames ? '' : 'rtcStreamMessage',
        subBuilder: RtcStreamMessage.create)
    ..aOM<GroupInviteMessage>(5, _omitFieldNames ? '' : 'groupInviteMessage',
        subBuilder: GroupInviteMessage.create)
    ..aOM<CommonMessage>(6, _omitFieldNames ? '' : 'commonMessage',
        subBuilder: CommonMessage.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Messaging clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Messaging copyWith(void Function(Messaging) updates) =>
      super.copyWith((message) => updates(message as Messaging)) as Messaging;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Messaging create() => Messaging._();
  @$core.override
  Messaging createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static Messaging getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Messaging>(create);
  static Messaging? _defaultInstance;

  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  @$pb.TagNumber(6)
  Messaging_Message whichMessage() => _Messaging_MessageByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  @$pb.TagNumber(6)
  void clearMessage() => $_clearField($_whichOneof(0));

  /// confirm chat message
  @$pb.TagNumber(1)
  Confirmation get confirmationMessage => $_getN(0);
  @$pb.TagNumber(1)
  set confirmationMessage(Confirmation value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasConfirmationMessage() => $_has(0);
  @$pb.TagNumber(1)
  void clearConfirmationMessage() => $_clearField(1);
  @$pb.TagNumber(1)
  Confirmation ensureConfirmationMessage() => $_ensure(0);

  /// dtn response message
  @$pb.TagNumber(2)
  DtnResponse get dtnResponse => $_getN(1);
  @$pb.TagNumber(2)
  set dtnResponse(DtnResponse value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasDtnResponse() => $_has(1);
  @$pb.TagNumber(2)
  void clearDtnResponse() => $_clearField(2);
  @$pb.TagNumber(2)
  DtnResponse ensureDtnResponse() => $_ensure(1);

  /// crypto service
  @$pb.TagNumber(3)
  CryptoService get cryptoService => $_getN(2);
  @$pb.TagNumber(3)
  set cryptoService(CryptoService value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasCryptoService() => $_has(2);
  @$pb.TagNumber(3)
  void clearCryptoService() => $_clearField(3);
  @$pb.TagNumber(3)
  CryptoService ensureCryptoService() => $_ensure(2);

  /// rtc stream
  @$pb.TagNumber(4)
  RtcStreamMessage get rtcStreamMessage => $_getN(3);
  @$pb.TagNumber(4)
  set rtcStreamMessage(RtcStreamMessage value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasRtcStreamMessage() => $_has(3);
  @$pb.TagNumber(4)
  void clearRtcStreamMessage() => $_clearField(4);
  @$pb.TagNumber(4)
  RtcStreamMessage ensureRtcStreamMessage() => $_ensure(3);

  /// group invite messages
  @$pb.TagNumber(5)
  GroupInviteMessage get groupInviteMessage => $_getN(4);
  @$pb.TagNumber(5)
  set groupInviteMessage(GroupInviteMessage value) => $_setField(5, value);
  @$pb.TagNumber(5)
  $core.bool hasGroupInviteMessage() => $_has(4);
  @$pb.TagNumber(5)
  void clearGroupInviteMessage() => $_clearField(5);
  @$pb.TagNumber(5)
  GroupInviteMessage ensureGroupInviteMessage() => $_ensure(4);

  /// common message
  @$pb.TagNumber(6)
  CommonMessage get commonMessage => $_getN(5);
  @$pb.TagNumber(6)
  set commonMessage(CommonMessage value) => $_setField(6, value);
  @$pb.TagNumber(6)
  $core.bool hasCommonMessage() => $_has(5);
  @$pb.TagNumber(6)
  void clearCommonMessage() => $_clearField(6);
  @$pb.TagNumber(6)
  CommonMessage ensureCommonMessage() => $_ensure(5);
}

/// message received confirmation
///
/// every message that was received by a user
/// sends an acknowledgment package, to the sender
/// to confirm the receive.
class Confirmation extends $pb.GeneratedMessage {
  factory Confirmation({
    $core.List<$core.int>? signature,
    $fixnum.Int64? receivedAt,
  }) {
    final result = create();
    if (signature != null) result.signature = signature;
    if (receivedAt != null) result.receivedAt = receivedAt;
    return result;
  }

  Confirmation._();

  factory Confirmation.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory Confirmation.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Confirmation',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.messaging'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'signature', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(
        2, _omitFieldNames ? '' : 'receivedAt', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Confirmation clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Confirmation copyWith(void Function(Confirmation) updates) =>
      super.copyWith((message) => updates(message as Confirmation))
          as Confirmation;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Confirmation create() => Confirmation._();
  @$core.override
  Confirmation createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static Confirmation getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<Confirmation>(create);
  static Confirmation? _defaultInstance;

  /// message ID
  @$pb.TagNumber(1)
  $core.List<$core.int> get signature => $_getN(0);
  @$pb.TagNumber(1)
  set signature($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSignature() => $_has(0);
  @$pb.TagNumber(1)
  void clearSignature() => $_clearField(1);

  /// received at timestamp
  @$pb.TagNumber(2)
  $fixnum.Int64 get receivedAt => $_getI64(1);
  @$pb.TagNumber(2)
  set receivedAt($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasReceivedAt() => $_has(1);
  @$pb.TagNumber(2)
  void clearReceivedAt() => $_clearField(2);
}

/// Crypto Service Message
///
/// This message is for crypto specific tasks,
/// such as completing a handshake.
class CryptoService extends $pb.GeneratedMessage {
  factory CryptoService({
    $core.List<$core.int>? content,
  }) {
    final result = create();
    if (content != null) result.content = content;
    return result;
  }

  CryptoService._();

  factory CryptoService.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory CryptoService.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CryptoService',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.messaging'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'content', $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CryptoService clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CryptoService copyWith(void Function(CryptoService) updates) =>
      super.copyWith((message) => updates(message as CryptoService))
          as CryptoService;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static CryptoService create() => CryptoService._();
  @$core.override
  CryptoService createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static CryptoService getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CryptoService>(create);
  static CryptoService? _defaultInstance;

  /// message data
  @$pb.TagNumber(1)
  $core.List<$core.int> get content => $_getN(0);
  @$pb.TagNumber(1)
  set content($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasContent() => $_has(0);
  @$pb.TagNumber(1)
  void clearContent() => $_clearField(1);
}

/// rtc stream mesasge
class RtcStreamMessage extends $pb.GeneratedMessage {
  factory RtcStreamMessage({
    $core.List<$core.int>? content,
  }) {
    final result = create();
    if (content != null) result.content = content;
    return result;
  }

  RtcStreamMessage._();

  factory RtcStreamMessage.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory RtcStreamMessage.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RtcStreamMessage',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.messaging'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'content', $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RtcStreamMessage clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RtcStreamMessage copyWith(void Function(RtcStreamMessage) updates) =>
      super.copyWith((message) => updates(message as RtcStreamMessage))
          as RtcStreamMessage;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RtcStreamMessage create() => RtcStreamMessage._();
  @$core.override
  RtcStreamMessage createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static RtcStreamMessage getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RtcStreamMessage>(create);
  static RtcStreamMessage? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get content => $_getN(0);
  @$pb.TagNumber(1)
  set content($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasContent() => $_has(0);
  @$pb.TagNumber(1)
  void clearContent() => $_clearField(1);
}

/// group invite message
class GroupInviteMessage extends $pb.GeneratedMessage {
  factory GroupInviteMessage({
    $core.List<$core.int>? content,
  }) {
    final result = create();
    if (content != null) result.content = content;
    return result;
  }

  GroupInviteMessage._();

  factory GroupInviteMessage.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory GroupInviteMessage.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GroupInviteMessage',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.messaging'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'content', $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupInviteMessage clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupInviteMessage copyWith(void Function(GroupInviteMessage) updates) =>
      super.copyWith((message) => updates(message as GroupInviteMessage))
          as GroupInviteMessage;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GroupInviteMessage create() => GroupInviteMessage._();
  @$core.override
  GroupInviteMessage createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static GroupInviteMessage getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GroupInviteMessage>(create);
  static GroupInviteMessage? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get content => $_getN(0);
  @$pb.TagNumber(1)
  set content($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasContent() => $_has(0);
  @$pb.TagNumber(1)
  void clearContent() => $_clearField(1);
}

enum CommonMessage_Payload {
  chatMessage,
  fileMessage,
  groupMessage,
  rtcMessage,
  notSet
}

/// common message
class CommonMessage extends $pb.GeneratedMessage {
  factory CommonMessage({
    $core.List<$core.int>? messageId,
    $core.List<$core.int>? groupId,
    $fixnum.Int64? sentAt,
    ChatMessage? chatMessage,
    FileMessage? fileMessage,
    GroupMessage? groupMessage,
    RtcMessage? rtcMessage,
  }) {
    final result = create();
    if (messageId != null) result.messageId = messageId;
    if (groupId != null) result.groupId = groupId;
    if (sentAt != null) result.sentAt = sentAt;
    if (chatMessage != null) result.chatMessage = chatMessage;
    if (fileMessage != null) result.fileMessage = fileMessage;
    if (groupMessage != null) result.groupMessage = groupMessage;
    if (rtcMessage != null) result.rtcMessage = rtcMessage;
    return result;
  }

  CommonMessage._();

  factory CommonMessage.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory CommonMessage.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, CommonMessage_Payload>
      _CommonMessage_PayloadByTag = {
    4: CommonMessage_Payload.chatMessage,
    5: CommonMessage_Payload.fileMessage,
    6: CommonMessage_Payload.groupMessage,
    7: CommonMessage_Payload.rtcMessage,
    0: CommonMessage_Payload.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CommonMessage',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.messaging'),
      createEmptyInstance: create)
    ..oo(0, [4, 5, 6, 7])
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'messageId', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(
        2, _omitFieldNames ? '' : 'groupId', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(3, _omitFieldNames ? '' : 'sentAt', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOM<ChatMessage>(4, _omitFieldNames ? '' : 'chatMessage',
        subBuilder: ChatMessage.create)
    ..aOM<FileMessage>(5, _omitFieldNames ? '' : 'fileMessage',
        subBuilder: FileMessage.create)
    ..aOM<GroupMessage>(6, _omitFieldNames ? '' : 'groupMessage',
        subBuilder: GroupMessage.create)
    ..aOM<RtcMessage>(7, _omitFieldNames ? '' : 'rtcMessage',
        subBuilder: RtcMessage.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CommonMessage clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CommonMessage copyWith(void Function(CommonMessage) updates) =>
      super.copyWith((message) => updates(message as CommonMessage))
          as CommonMessage;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static CommonMessage create() => CommonMessage._();
  @$core.override
  CommonMessage createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static CommonMessage getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CommonMessage>(create);
  static CommonMessage? _defaultInstance;

  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  @$pb.TagNumber(6)
  @$pb.TagNumber(7)
  CommonMessage_Payload whichPayload() =>
      _CommonMessage_PayloadByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  @$pb.TagNumber(6)
  @$pb.TagNumber(7)
  void clearPayload() => $_clearField($_whichOneof(0));

  /// message ID
  @$pb.TagNumber(1)
  $core.List<$core.int> get messageId => $_getN(0);
  @$pb.TagNumber(1)
  set messageId($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasMessageId() => $_has(0);
  @$pb.TagNumber(1)
  void clearMessageId() => $_clearField(1);

  /// group id
  @$pb.TagNumber(2)
  $core.List<$core.int> get groupId => $_getN(1);
  @$pb.TagNumber(2)
  set groupId($core.List<$core.int> value) => $_setBytes(1, value);
  @$pb.TagNumber(2)
  $core.bool hasGroupId() => $_has(1);
  @$pb.TagNumber(2)
  void clearGroupId() => $_clearField(2);

  /// sent at timestamp
  @$pb.TagNumber(3)
  $fixnum.Int64 get sentAt => $_getI64(2);
  @$pb.TagNumber(3)
  set sentAt($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasSentAt() => $_has(2);
  @$pb.TagNumber(3)
  void clearSentAt() => $_clearField(3);

  /// chat message
  @$pb.TagNumber(4)
  ChatMessage get chatMessage => $_getN(3);
  @$pb.TagNumber(4)
  set chatMessage(ChatMessage value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasChatMessage() => $_has(3);
  @$pb.TagNumber(4)
  void clearChatMessage() => $_clearField(4);
  @$pb.TagNumber(4)
  ChatMessage ensureChatMessage() => $_ensure(3);

  /// file message
  @$pb.TagNumber(5)
  FileMessage get fileMessage => $_getN(4);
  @$pb.TagNumber(5)
  set fileMessage(FileMessage value) => $_setField(5, value);
  @$pb.TagNumber(5)
  $core.bool hasFileMessage() => $_has(4);
  @$pb.TagNumber(5)
  void clearFileMessage() => $_clearField(5);
  @$pb.TagNumber(5)
  FileMessage ensureFileMessage() => $_ensure(4);

  /// group message
  @$pb.TagNumber(6)
  GroupMessage get groupMessage => $_getN(5);
  @$pb.TagNumber(6)
  set groupMessage(GroupMessage value) => $_setField(6, value);
  @$pb.TagNumber(6)
  $core.bool hasGroupMessage() => $_has(5);
  @$pb.TagNumber(6)
  void clearGroupMessage() => $_clearField(6);
  @$pb.TagNumber(6)
  GroupMessage ensureGroupMessage() => $_ensure(5);

  /// rtc message
  @$pb.TagNumber(7)
  RtcMessage get rtcMessage => $_getN(6);
  @$pb.TagNumber(7)
  set rtcMessage(RtcMessage value) => $_setField(7, value);
  @$pb.TagNumber(7)
  $core.bool hasRtcMessage() => $_has(6);
  @$pb.TagNumber(7)
  void clearRtcMessage() => $_clearField(7);
  @$pb.TagNumber(7)
  RtcMessage ensureRtcMessage() => $_ensure(6);
}

/// chat content
class ChatMessage extends $pb.GeneratedMessage {
  factory ChatMessage({
    $core.String? content,
  }) {
    final result = create();
    if (content != null) result.content = content;
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
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.messaging'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'content')
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

  /// content
  @$pb.TagNumber(1)
  $core.String get content => $_getSZ(0);
  @$pb.TagNumber(1)
  set content($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasContent() => $_has(0);
  @$pb.TagNumber(1)
  void clearContent() => $_clearField(1);
}

/// file message
class FileMessage extends $pb.GeneratedMessage {
  factory FileMessage({
    $core.List<$core.int>? content,
  }) {
    final result = create();
    if (content != null) result.content = content;
    return result;
  }

  FileMessage._();

  factory FileMessage.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory FileMessage.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'FileMessage',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.messaging'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'content', $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  FileMessage clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  FileMessage copyWith(void Function(FileMessage) updates) =>
      super.copyWith((message) => updates(message as FileMessage))
          as FileMessage;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FileMessage create() => FileMessage._();
  @$core.override
  FileMessage createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static FileMessage getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<FileMessage>(create);
  static FileMessage? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get content => $_getN(0);
  @$pb.TagNumber(1)
  set content($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasContent() => $_has(0);
  @$pb.TagNumber(1)
  void clearContent() => $_clearField(1);
}

/// group message
class GroupMessage extends $pb.GeneratedMessage {
  factory GroupMessage({
    $core.List<$core.int>? content,
  }) {
    final result = create();
    if (content != null) result.content = content;
    return result;
  }

  GroupMessage._();

  factory GroupMessage.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory GroupMessage.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GroupMessage',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.messaging'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'content', $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupMessage clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupMessage copyWith(void Function(GroupMessage) updates) =>
      super.copyWith((message) => updates(message as GroupMessage))
          as GroupMessage;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GroupMessage create() => GroupMessage._();
  @$core.override
  GroupMessage createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static GroupMessage getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GroupMessage>(create);
  static GroupMessage? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get content => $_getN(0);
  @$pb.TagNumber(1)
  set content($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasContent() => $_has(0);
  @$pb.TagNumber(1)
  void clearContent() => $_clearField(1);
}

/// rtc message
class RtcMessage extends $pb.GeneratedMessage {
  factory RtcMessage({
    $core.List<$core.int>? content,
  }) {
    final result = create();
    if (content != null) result.content = content;
    return result;
  }

  RtcMessage._();

  factory RtcMessage.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory RtcMessage.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RtcMessage',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.messaging'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'content', $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RtcMessage clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RtcMessage copyWith(void Function(RtcMessage) updates) =>
      super.copyWith((message) => updates(message as RtcMessage)) as RtcMessage;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RtcMessage create() => RtcMessage._();
  @$core.override
  RtcMessage createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static RtcMessage getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RtcMessage>(create);
  static RtcMessage? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get content => $_getN(0);
  @$pb.TagNumber(1)
  set content($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasContent() => $_has(0);
  @$pb.TagNumber(1)
  void clearContent() => $_clearField(1);
}

enum Dtn_Message { container, response, notSet }

/// DTN message
class Dtn extends $pb.GeneratedMessage {
  factory Dtn({
    $core.List<$core.int>? container,
    DtnResponse? response,
  }) {
    final result = create();
    if (container != null) result.container = container;
    if (response != null) result.response = response;
    return result;
  }

  Dtn._();

  factory Dtn.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory Dtn.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, Dtn_Message> _Dtn_MessageByTag = {
    1: Dtn_Message.container,
    2: Dtn_Message.response,
    0: Dtn_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Dtn',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.messaging'),
      createEmptyInstance: create)
    ..oo(0, [1, 2])
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'container', $pb.PbFieldType.OY)
    ..aOM<DtnResponse>(2, _omitFieldNames ? '' : 'response',
        subBuilder: DtnResponse.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Dtn clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Dtn copyWith(void Function(Dtn) updates) =>
      super.copyWith((message) => updates(message as Dtn)) as Dtn;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Dtn create() => Dtn._();
  @$core.override
  Dtn createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static Dtn getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Dtn>(create);
  static Dtn? _defaultInstance;

  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  Dtn_Message whichMessage() => _Dtn_MessageByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  void clearMessage() => $_clearField($_whichOneof(0));

  /// message container
  @$pb.TagNumber(1)
  $core.List<$core.int> get container => $_getN(0);
  @$pb.TagNumber(1)
  set container($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasContainer() => $_has(0);
  @$pb.TagNumber(1)
  void clearContainer() => $_clearField(1);

  /// message received response
  @$pb.TagNumber(2)
  DtnResponse get response => $_getN(1);
  @$pb.TagNumber(2)
  set response(DtnResponse value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasResponse() => $_has(1);
  @$pb.TagNumber(2)
  void clearResponse() => $_clearField(2);
  @$pb.TagNumber(2)
  DtnResponse ensureResponse() => $_ensure(1);
}

/// DTN response
class DtnResponse extends $pb.GeneratedMessage {
  factory DtnResponse({
    DtnResponse_ResponseType? responseType,
    $core.List<$core.int>? signature,
    DtnResponse_Reason? reason,
  }) {
    final result = create();
    if (responseType != null) result.responseType = responseType;
    if (signature != null) result.signature = signature;
    if (reason != null) result.reason = reason;
    return result;
  }

  DtnResponse._();

  factory DtnResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory DtnResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DtnResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.messaging'),
      createEmptyInstance: create)
    ..aE<DtnResponse_ResponseType>(1, _omitFieldNames ? '' : 'responseType',
        enumValues: DtnResponse_ResponseType.values)
    ..a<$core.List<$core.int>>(
        2, _omitFieldNames ? '' : 'signature', $pb.PbFieldType.OY)
    ..aE<DtnResponse_Reason>(3, _omitFieldNames ? '' : 'reason',
        enumValues: DtnResponse_Reason.values)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DtnResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DtnResponse copyWith(void Function(DtnResponse) updates) =>
      super.copyWith((message) => updates(message as DtnResponse))
          as DtnResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static DtnResponse create() => DtnResponse._();
  @$core.override
  DtnResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static DtnResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DtnResponse>(create);
  static DtnResponse? _defaultInstance;

  /// the type of the message
  @$pb.TagNumber(1)
  DtnResponse_ResponseType get responseType => $_getN(0);
  @$pb.TagNumber(1)
  set responseType(DtnResponse_ResponseType value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasResponseType() => $_has(0);
  @$pb.TagNumber(1)
  void clearResponseType() => $_clearField(1);

  /// message signature reference
  @$pb.TagNumber(2)
  $core.List<$core.int> get signature => $_getN(1);
  @$pb.TagNumber(2)
  set signature($core.List<$core.int> value) => $_setBytes(1, value);
  @$pb.TagNumber(2)
  $core.bool hasSignature() => $_has(1);
  @$pb.TagNumber(2)
  void clearSignature() => $_clearField(2);

  /// reason of rejection
  @$pb.TagNumber(3)
  DtnResponse_Reason get reason => $_getN(2);
  @$pb.TagNumber(3)
  set reason(DtnResponse_Reason value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasReason() => $_has(2);
  @$pb.TagNumber(3)
  void clearReason() => $_clearField(3);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
