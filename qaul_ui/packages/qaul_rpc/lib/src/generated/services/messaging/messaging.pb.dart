//
//  Generated code. Do not modify.
//  source: services/messaging/messaging.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'messaging.pbenum.dart';

export 'messaging.pbenum.dart';

class Container extends $pb.GeneratedMessage {
  factory Container() => create();
  Container._() : super();
  factory Container.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Container.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Container', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.messaging'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'signature', $pb.PbFieldType.OY)
    ..aOM<Envelope>(2, _omitFieldNames ? '' : 'envelope', subBuilder: Envelope.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Container clone() => Container()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Container copyWith(void Function(Container) updates) => super.copyWith((message) => updates(message as Container)) as Container;

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
  factory Envelope() => create();
  Envelope._() : super();
  factory Envelope.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Envelope.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Envelope', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.messaging'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'senderId', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(2, _omitFieldNames ? '' : 'receiverId', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(3, _omitFieldNames ? '' : 'payload', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Envelope clone() => Envelope()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Envelope copyWith(void Function(Envelope) updates) => super.copyWith((message) => updates(message as Envelope)) as Envelope;

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
  $core.List<$core.int> get payload => $_getN(2);
  @$pb.TagNumber(3)
  set payload($core.List<$core.int> v) { $_setBytes(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasPayload() => $_has(2);
  @$pb.TagNumber(3)
  void clearPayload() => clearField(3);
}

enum EnvelopPayload_Payload {
  encrypted, 
  dtn, 
  notSet
}

class EnvelopPayload extends $pb.GeneratedMessage {
  factory EnvelopPayload() => create();
  EnvelopPayload._() : super();
  factory EnvelopPayload.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory EnvelopPayload.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static const $core.Map<$core.int, EnvelopPayload_Payload> _EnvelopPayload_PayloadByTag = {
    1 : EnvelopPayload_Payload.encrypted,
    2 : EnvelopPayload_Payload.dtn,
    0 : EnvelopPayload_Payload.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'EnvelopPayload', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.messaging'), createEmptyInstance: create)
    ..oo(0, [1, 2])
    ..aOM<Encrypted>(1, _omitFieldNames ? '' : 'encrypted', subBuilder: Encrypted.create)
    ..a<$core.List<$core.int>>(2, _omitFieldNames ? '' : 'dtn', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  EnvelopPayload clone() => EnvelopPayload()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  EnvelopPayload copyWith(void Function(EnvelopPayload) updates) => super.copyWith((message) => updates(message as EnvelopPayload)) as EnvelopPayload;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static EnvelopPayload create() => EnvelopPayload._();
  EnvelopPayload createEmptyInstance() => create();
  static $pb.PbList<EnvelopPayload> createRepeated() => $pb.PbList<EnvelopPayload>();
  @$core.pragma('dart2js:noInline')
  static EnvelopPayload getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<EnvelopPayload>(create);
  static EnvelopPayload? _defaultInstance;

  EnvelopPayload_Payload whichPayload() => _EnvelopPayload_PayloadByTag[$_whichOneof(0)]!;
  void clearPayload() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  Encrypted get encrypted => $_getN(0);
  @$pb.TagNumber(1)
  set encrypted(Encrypted v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasEncrypted() => $_has(0);
  @$pb.TagNumber(1)
  void clearEncrypted() => clearField(1);
  @$pb.TagNumber(1)
  Encrypted ensureEncrypted() => $_ensure(0);

  @$pb.TagNumber(2)
  $core.List<$core.int> get dtn => $_getN(1);
  @$pb.TagNumber(2)
  set dtn($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasDtn() => $_has(1);
  @$pb.TagNumber(2)
  void clearDtn() => clearField(2);
}

class Encrypted extends $pb.GeneratedMessage {
  factory Encrypted() => create();
  Encrypted._() : super();
  factory Encrypted.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Encrypted.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Encrypted', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.messaging'), createEmptyInstance: create)
    ..e<CryptoState>(1, _omitFieldNames ? '' : 'state', $pb.PbFieldType.OE, defaultOrMaker: CryptoState.NONE, valueOf: CryptoState.valueOf, enumValues: CryptoState.values)
    ..a<$core.int>(2, _omitFieldNames ? '' : 'sessionId', $pb.PbFieldType.OU3)
    ..pc<Data>(3, _omitFieldNames ? '' : 'data', $pb.PbFieldType.PM, subBuilder: Data.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Encrypted clone() => Encrypted()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Encrypted copyWith(void Function(Encrypted) updates) => super.copyWith((message) => updates(message as Encrypted)) as Encrypted;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Encrypted create() => Encrypted._();
  Encrypted createEmptyInstance() => create();
  static $pb.PbList<Encrypted> createRepeated() => $pb.PbList<Encrypted>();
  @$core.pragma('dart2js:noInline')
  static Encrypted getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Encrypted>(create);
  static Encrypted? _defaultInstance;

  @$pb.TagNumber(1)
  CryptoState get state => $_getN(0);
  @$pb.TagNumber(1)
  set state(CryptoState v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasState() => $_has(0);
  @$pb.TagNumber(1)
  void clearState() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get sessionId => $_getIZ(1);
  @$pb.TagNumber(2)
  set sessionId($core.int v) { $_setUnsignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasSessionId() => $_has(1);
  @$pb.TagNumber(2)
  void clearSessionId() => clearField(2);

  @$pb.TagNumber(3)
  $core.List<Data> get data => $_getList(2);
}

class Data extends $pb.GeneratedMessage {
  factory Data() => create();
  Data._() : super();
  factory Data.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Data.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Data', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.messaging'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, _omitFieldNames ? '' : 'nonce', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.List<$core.int>>(2, _omitFieldNames ? '' : 'data', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Data clone() => Data()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Data copyWith(void Function(Data) updates) => super.copyWith((message) => updates(message as Data)) as Data;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Data create() => Data._();
  Data createEmptyInstance() => create();
  static $pb.PbList<Data> createRepeated() => $pb.PbList<Data>();
  @$core.pragma('dart2js:noInline')
  static Data getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Data>(create);
  static Data? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get nonce => $_getI64(0);
  @$pb.TagNumber(1)
  set nonce($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasNonce() => $_has(0);
  @$pb.TagNumber(1)
  void clearNonce() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get data => $_getN(1);
  @$pb.TagNumber(2)
  set data($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasData() => $_has(1);
  @$pb.TagNumber(2)
  void clearData() => clearField(2);
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

class Messaging extends $pb.GeneratedMessage {
  factory Messaging() => create();
  Messaging._() : super();
  factory Messaging.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Messaging.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static const $core.Map<$core.int, Messaging_Message> _Messaging_MessageByTag = {
    1 : Messaging_Message.confirmationMessage,
    2 : Messaging_Message.dtnResponse,
    3 : Messaging_Message.cryptoService,
    4 : Messaging_Message.rtcStreamMessage,
    5 : Messaging_Message.groupInviteMessage,
    6 : Messaging_Message.commonMessage,
    0 : Messaging_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Messaging', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.messaging'), createEmptyInstance: create)
    ..oo(0, [1, 2, 3, 4, 5, 6])
    ..aOM<Confirmation>(1, _omitFieldNames ? '' : 'confirmationMessage', subBuilder: Confirmation.create)
    ..aOM<DtnResponse>(2, _omitFieldNames ? '' : 'dtnResponse', subBuilder: DtnResponse.create)
    ..aOM<CryptoService>(3, _omitFieldNames ? '' : 'cryptoService', subBuilder: CryptoService.create)
    ..aOM<RtcStreamMessage>(4, _omitFieldNames ? '' : 'rtcStreamMessage', subBuilder: RtcStreamMessage.create)
    ..aOM<GroupInviteMessage>(5, _omitFieldNames ? '' : 'groupInviteMessage', subBuilder: GroupInviteMessage.create)
    ..aOM<CommonMessage>(6, _omitFieldNames ? '' : 'commonMessage', subBuilder: CommonMessage.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Messaging clone() => Messaging()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Messaging copyWith(void Function(Messaging) updates) => super.copyWith((message) => updates(message as Messaging)) as Messaging;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Messaging create() => Messaging._();
  Messaging createEmptyInstance() => create();
  static $pb.PbList<Messaging> createRepeated() => $pb.PbList<Messaging>();
  @$core.pragma('dart2js:noInline')
  static Messaging getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Messaging>(create);
  static Messaging? _defaultInstance;

  Messaging_Message whichMessage() => _Messaging_MessageByTag[$_whichOneof(0)]!;
  void clearMessage() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  Confirmation get confirmationMessage => $_getN(0);
  @$pb.TagNumber(1)
  set confirmationMessage(Confirmation v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasConfirmationMessage() => $_has(0);
  @$pb.TagNumber(1)
  void clearConfirmationMessage() => clearField(1);
  @$pb.TagNumber(1)
  Confirmation ensureConfirmationMessage() => $_ensure(0);

  @$pb.TagNumber(2)
  DtnResponse get dtnResponse => $_getN(1);
  @$pb.TagNumber(2)
  set dtnResponse(DtnResponse v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasDtnResponse() => $_has(1);
  @$pb.TagNumber(2)
  void clearDtnResponse() => clearField(2);
  @$pb.TagNumber(2)
  DtnResponse ensureDtnResponse() => $_ensure(1);

  @$pb.TagNumber(3)
  CryptoService get cryptoService => $_getN(2);
  @$pb.TagNumber(3)
  set cryptoService(CryptoService v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasCryptoService() => $_has(2);
  @$pb.TagNumber(3)
  void clearCryptoService() => clearField(3);
  @$pb.TagNumber(3)
  CryptoService ensureCryptoService() => $_ensure(2);

  @$pb.TagNumber(4)
  RtcStreamMessage get rtcStreamMessage => $_getN(3);
  @$pb.TagNumber(4)
  set rtcStreamMessage(RtcStreamMessage v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasRtcStreamMessage() => $_has(3);
  @$pb.TagNumber(4)
  void clearRtcStreamMessage() => clearField(4);
  @$pb.TagNumber(4)
  RtcStreamMessage ensureRtcStreamMessage() => $_ensure(3);

  @$pb.TagNumber(5)
  GroupInviteMessage get groupInviteMessage => $_getN(4);
  @$pb.TagNumber(5)
  set groupInviteMessage(GroupInviteMessage v) { setField(5, v); }
  @$pb.TagNumber(5)
  $core.bool hasGroupInviteMessage() => $_has(4);
  @$pb.TagNumber(5)
  void clearGroupInviteMessage() => clearField(5);
  @$pb.TagNumber(5)
  GroupInviteMessage ensureGroupInviteMessage() => $_ensure(4);

  @$pb.TagNumber(6)
  CommonMessage get commonMessage => $_getN(5);
  @$pb.TagNumber(6)
  set commonMessage(CommonMessage v) { setField(6, v); }
  @$pb.TagNumber(6)
  $core.bool hasCommonMessage() => $_has(5);
  @$pb.TagNumber(6)
  void clearCommonMessage() => clearField(6);
  @$pb.TagNumber(6)
  CommonMessage ensureCommonMessage() => $_ensure(5);
}

class Confirmation extends $pb.GeneratedMessage {
  factory Confirmation() => create();
  Confirmation._() : super();
  factory Confirmation.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Confirmation.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Confirmation', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.messaging'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'signature', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(2, _omitFieldNames ? '' : 'receivedAt', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Confirmation clone() => Confirmation()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Confirmation copyWith(void Function(Confirmation) updates) => super.copyWith((message) => updates(message as Confirmation)) as Confirmation;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Confirmation create() => Confirmation._();
  Confirmation createEmptyInstance() => create();
  static $pb.PbList<Confirmation> createRepeated() => $pb.PbList<Confirmation>();
  @$core.pragma('dart2js:noInline')
  static Confirmation getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Confirmation>(create);
  static Confirmation? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get signature => $_getN(0);
  @$pb.TagNumber(1)
  set signature($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasSignature() => $_has(0);
  @$pb.TagNumber(1)
  void clearSignature() => clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get receivedAt => $_getI64(1);
  @$pb.TagNumber(2)
  set receivedAt($fixnum.Int64 v) { $_setInt64(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasReceivedAt() => $_has(1);
  @$pb.TagNumber(2)
  void clearReceivedAt() => clearField(2);
}

class CryptoService extends $pb.GeneratedMessage {
  factory CryptoService() => create();
  CryptoService._() : super();
  factory CryptoService.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory CryptoService.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'CryptoService', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.messaging'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'content', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  CryptoService clone() => CryptoService()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  CryptoService copyWith(void Function(CryptoService) updates) => super.copyWith((message) => updates(message as CryptoService)) as CryptoService;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static CryptoService create() => CryptoService._();
  CryptoService createEmptyInstance() => create();
  static $pb.PbList<CryptoService> createRepeated() => $pb.PbList<CryptoService>();
  @$core.pragma('dart2js:noInline')
  static CryptoService getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<CryptoService>(create);
  static CryptoService? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get content => $_getN(0);
  @$pb.TagNumber(1)
  set content($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasContent() => $_has(0);
  @$pb.TagNumber(1)
  void clearContent() => clearField(1);
}

class RtcStreamMessage extends $pb.GeneratedMessage {
  factory RtcStreamMessage() => create();
  RtcStreamMessage._() : super();
  factory RtcStreamMessage.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RtcStreamMessage.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'RtcStreamMessage', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.messaging'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'content', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RtcStreamMessage clone() => RtcStreamMessage()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RtcStreamMessage copyWith(void Function(RtcStreamMessage) updates) => super.copyWith((message) => updates(message as RtcStreamMessage)) as RtcStreamMessage;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RtcStreamMessage create() => RtcStreamMessage._();
  RtcStreamMessage createEmptyInstance() => create();
  static $pb.PbList<RtcStreamMessage> createRepeated() => $pb.PbList<RtcStreamMessage>();
  @$core.pragma('dart2js:noInline')
  static RtcStreamMessage getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RtcStreamMessage>(create);
  static RtcStreamMessage? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get content => $_getN(0);
  @$pb.TagNumber(1)
  set content($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasContent() => $_has(0);
  @$pb.TagNumber(1)
  void clearContent() => clearField(1);
}

class GroupInviteMessage extends $pb.GeneratedMessage {
  factory GroupInviteMessage() => create();
  GroupInviteMessage._() : super();
  factory GroupInviteMessage.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupInviteMessage.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GroupInviteMessage', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.messaging'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'content', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GroupInviteMessage clone() => GroupInviteMessage()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GroupInviteMessage copyWith(void Function(GroupInviteMessage) updates) => super.copyWith((message) => updates(message as GroupInviteMessage)) as GroupInviteMessage;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GroupInviteMessage create() => GroupInviteMessage._();
  GroupInviteMessage createEmptyInstance() => create();
  static $pb.PbList<GroupInviteMessage> createRepeated() => $pb.PbList<GroupInviteMessage>();
  @$core.pragma('dart2js:noInline')
  static GroupInviteMessage getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupInviteMessage>(create);
  static GroupInviteMessage? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get content => $_getN(0);
  @$pb.TagNumber(1)
  set content($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasContent() => $_has(0);
  @$pb.TagNumber(1)
  void clearContent() => clearField(1);
}

enum CommonMessage_Payload {
  chatMessage, 
  fileMessage, 
  groupMessage, 
  rtcMessage, 
  notSet
}

class CommonMessage extends $pb.GeneratedMessage {
  factory CommonMessage() => create();
  CommonMessage._() : super();
  factory CommonMessage.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory CommonMessage.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static const $core.Map<$core.int, CommonMessage_Payload> _CommonMessage_PayloadByTag = {
    4 : CommonMessage_Payload.chatMessage,
    5 : CommonMessage_Payload.fileMessage,
    6 : CommonMessage_Payload.groupMessage,
    7 : CommonMessage_Payload.rtcMessage,
    0 : CommonMessage_Payload.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'CommonMessage', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.messaging'), createEmptyInstance: create)
    ..oo(0, [4, 5, 6, 7])
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'messageId', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(2, _omitFieldNames ? '' : 'groupId', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(3, _omitFieldNames ? '' : 'sentAt', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOM<ChatMessage>(4, _omitFieldNames ? '' : 'chatMessage', subBuilder: ChatMessage.create)
    ..aOM<FileMessage>(5, _omitFieldNames ? '' : 'fileMessage', subBuilder: FileMessage.create)
    ..aOM<GroupMessage>(6, _omitFieldNames ? '' : 'groupMessage', subBuilder: GroupMessage.create)
    ..aOM<RtcMessage>(7, _omitFieldNames ? '' : 'rtcMessage', subBuilder: RtcMessage.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  CommonMessage clone() => CommonMessage()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  CommonMessage copyWith(void Function(CommonMessage) updates) => super.copyWith((message) => updates(message as CommonMessage)) as CommonMessage;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static CommonMessage create() => CommonMessage._();
  CommonMessage createEmptyInstance() => create();
  static $pb.PbList<CommonMessage> createRepeated() => $pb.PbList<CommonMessage>();
  @$core.pragma('dart2js:noInline')
  static CommonMessage getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<CommonMessage>(create);
  static CommonMessage? _defaultInstance;

  CommonMessage_Payload whichPayload() => _CommonMessage_PayloadByTag[$_whichOneof(0)]!;
  void clearPayload() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  $core.List<$core.int> get messageId => $_getN(0);
  @$pb.TagNumber(1)
  set messageId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasMessageId() => $_has(0);
  @$pb.TagNumber(1)
  void clearMessageId() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get groupId => $_getN(1);
  @$pb.TagNumber(2)
  set groupId($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasGroupId() => $_has(1);
  @$pb.TagNumber(2)
  void clearGroupId() => clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get sentAt => $_getI64(2);
  @$pb.TagNumber(3)
  set sentAt($fixnum.Int64 v) { $_setInt64(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasSentAt() => $_has(2);
  @$pb.TagNumber(3)
  void clearSentAt() => clearField(3);

  @$pb.TagNumber(4)
  ChatMessage get chatMessage => $_getN(3);
  @$pb.TagNumber(4)
  set chatMessage(ChatMessage v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasChatMessage() => $_has(3);
  @$pb.TagNumber(4)
  void clearChatMessage() => clearField(4);
  @$pb.TagNumber(4)
  ChatMessage ensureChatMessage() => $_ensure(3);

  @$pb.TagNumber(5)
  FileMessage get fileMessage => $_getN(4);
  @$pb.TagNumber(5)
  set fileMessage(FileMessage v) { setField(5, v); }
  @$pb.TagNumber(5)
  $core.bool hasFileMessage() => $_has(4);
  @$pb.TagNumber(5)
  void clearFileMessage() => clearField(5);
  @$pb.TagNumber(5)
  FileMessage ensureFileMessage() => $_ensure(4);

  @$pb.TagNumber(6)
  GroupMessage get groupMessage => $_getN(5);
  @$pb.TagNumber(6)
  set groupMessage(GroupMessage v) { setField(6, v); }
  @$pb.TagNumber(6)
  $core.bool hasGroupMessage() => $_has(5);
  @$pb.TagNumber(6)
  void clearGroupMessage() => clearField(6);
  @$pb.TagNumber(6)
  GroupMessage ensureGroupMessage() => $_ensure(5);

  @$pb.TagNumber(7)
  RtcMessage get rtcMessage => $_getN(6);
  @$pb.TagNumber(7)
  set rtcMessage(RtcMessage v) { setField(7, v); }
  @$pb.TagNumber(7)
  $core.bool hasRtcMessage() => $_has(6);
  @$pb.TagNumber(7)
  void clearRtcMessage() => clearField(7);
  @$pb.TagNumber(7)
  RtcMessage ensureRtcMessage() => $_ensure(6);
}

class ChatMessage extends $pb.GeneratedMessage {
  factory ChatMessage() => create();
  ChatMessage._() : super();
  factory ChatMessage.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChatMessage.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ChatMessage', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.messaging'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'content')
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
  $core.String get content => $_getSZ(0);
  @$pb.TagNumber(1)
  set content($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasContent() => $_has(0);
  @$pb.TagNumber(1)
  void clearContent() => clearField(1);
}

class FileMessage extends $pb.GeneratedMessage {
  factory FileMessage() => create();
  FileMessage._() : super();
  factory FileMessage.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FileMessage.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'FileMessage', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.messaging'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'content', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  FileMessage clone() => FileMessage()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  FileMessage copyWith(void Function(FileMessage) updates) => super.copyWith((message) => updates(message as FileMessage)) as FileMessage;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FileMessage create() => FileMessage._();
  FileMessage createEmptyInstance() => create();
  static $pb.PbList<FileMessage> createRepeated() => $pb.PbList<FileMessage>();
  @$core.pragma('dart2js:noInline')
  static FileMessage getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FileMessage>(create);
  static FileMessage? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get content => $_getN(0);
  @$pb.TagNumber(1)
  set content($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasContent() => $_has(0);
  @$pb.TagNumber(1)
  void clearContent() => clearField(1);
}

class GroupMessage extends $pb.GeneratedMessage {
  factory GroupMessage() => create();
  GroupMessage._() : super();
  factory GroupMessage.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupMessage.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GroupMessage', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.messaging'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'content', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GroupMessage clone() => GroupMessage()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GroupMessage copyWith(void Function(GroupMessage) updates) => super.copyWith((message) => updates(message as GroupMessage)) as GroupMessage;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GroupMessage create() => GroupMessage._();
  GroupMessage createEmptyInstance() => create();
  static $pb.PbList<GroupMessage> createRepeated() => $pb.PbList<GroupMessage>();
  @$core.pragma('dart2js:noInline')
  static GroupMessage getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupMessage>(create);
  static GroupMessage? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get content => $_getN(0);
  @$pb.TagNumber(1)
  set content($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasContent() => $_has(0);
  @$pb.TagNumber(1)
  void clearContent() => clearField(1);
}

class RtcMessage extends $pb.GeneratedMessage {
  factory RtcMessage() => create();
  RtcMessage._() : super();
  factory RtcMessage.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RtcMessage.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'RtcMessage', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.messaging'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'content', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RtcMessage clone() => RtcMessage()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RtcMessage copyWith(void Function(RtcMessage) updates) => super.copyWith((message) => updates(message as RtcMessage)) as RtcMessage;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RtcMessage create() => RtcMessage._();
  RtcMessage createEmptyInstance() => create();
  static $pb.PbList<RtcMessage> createRepeated() => $pb.PbList<RtcMessage>();
  @$core.pragma('dart2js:noInline')
  static RtcMessage getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RtcMessage>(create);
  static RtcMessage? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get content => $_getN(0);
  @$pb.TagNumber(1)
  set content($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasContent() => $_has(0);
  @$pb.TagNumber(1)
  void clearContent() => clearField(1);
}

enum Dtn_Message {
  container, 
  response, 
  notSet
}

class Dtn extends $pb.GeneratedMessage {
  factory Dtn() => create();
  Dtn._() : super();
  factory Dtn.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Dtn.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static const $core.Map<$core.int, Dtn_Message> _Dtn_MessageByTag = {
    1 : Dtn_Message.container,
    2 : Dtn_Message.response,
    0 : Dtn_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Dtn', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.messaging'), createEmptyInstance: create)
    ..oo(0, [1, 2])
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'container', $pb.PbFieldType.OY)
    ..aOM<DtnResponse>(2, _omitFieldNames ? '' : 'response', subBuilder: DtnResponse.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Dtn clone() => Dtn()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Dtn copyWith(void Function(Dtn) updates) => super.copyWith((message) => updates(message as Dtn)) as Dtn;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Dtn create() => Dtn._();
  Dtn createEmptyInstance() => create();
  static $pb.PbList<Dtn> createRepeated() => $pb.PbList<Dtn>();
  @$core.pragma('dart2js:noInline')
  static Dtn getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Dtn>(create);
  static Dtn? _defaultInstance;

  Dtn_Message whichMessage() => _Dtn_MessageByTag[$_whichOneof(0)]!;
  void clearMessage() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  $core.List<$core.int> get container => $_getN(0);
  @$pb.TagNumber(1)
  set container($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasContainer() => $_has(0);
  @$pb.TagNumber(1)
  void clearContainer() => clearField(1);

  @$pb.TagNumber(2)
  DtnResponse get response => $_getN(1);
  @$pb.TagNumber(2)
  set response(DtnResponse v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasResponse() => $_has(1);
  @$pb.TagNumber(2)
  void clearResponse() => clearField(2);
  @$pb.TagNumber(2)
  DtnResponse ensureResponse() => $_ensure(1);
}

class DtnResponse extends $pb.GeneratedMessage {
  factory DtnResponse() => create();
  DtnResponse._() : super();
  factory DtnResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory DtnResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'DtnResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.messaging'), createEmptyInstance: create)
    ..e<DtnResponse_ResponseType>(1, _omitFieldNames ? '' : 'responseType', $pb.PbFieldType.OE, defaultOrMaker: DtnResponse_ResponseType.ACCEPTED, valueOf: DtnResponse_ResponseType.valueOf, enumValues: DtnResponse_ResponseType.values)
    ..a<$core.List<$core.int>>(2, _omitFieldNames ? '' : 'signature', $pb.PbFieldType.OY)
    ..e<DtnResponse_Reason>(3, _omitFieldNames ? '' : 'reason', $pb.PbFieldType.OE, defaultOrMaker: DtnResponse_Reason.NONE, valueOf: DtnResponse_Reason.valueOf, enumValues: DtnResponse_Reason.values)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  DtnResponse clone() => DtnResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  DtnResponse copyWith(void Function(DtnResponse) updates) => super.copyWith((message) => updates(message as DtnResponse)) as DtnResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static DtnResponse create() => DtnResponse._();
  DtnResponse createEmptyInstance() => create();
  static $pb.PbList<DtnResponse> createRepeated() => $pb.PbList<DtnResponse>();
  @$core.pragma('dart2js:noInline')
  static DtnResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DtnResponse>(create);
  static DtnResponse? _defaultInstance;

  @$pb.TagNumber(1)
  DtnResponse_ResponseType get responseType => $_getN(0);
  @$pb.TagNumber(1)
  set responseType(DtnResponse_ResponseType v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasResponseType() => $_has(0);
  @$pb.TagNumber(1)
  void clearResponseType() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get signature => $_getN(1);
  @$pb.TagNumber(2)
  set signature($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasSignature() => $_has(1);
  @$pb.TagNumber(2)
  void clearSignature() => clearField(2);

  @$pb.TagNumber(3)
  DtnResponse_Reason get reason => $_getN(2);
  @$pb.TagNumber(3)
  set reason(DtnResponse_Reason v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasReason() => $_has(2);
  @$pb.TagNumber(3)
  void clearReason() => clearField(3);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
