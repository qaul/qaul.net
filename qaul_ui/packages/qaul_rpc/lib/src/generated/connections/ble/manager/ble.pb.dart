///
//  Generated code. Do not modify.
//  source: connections/ble/manager/ble.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'ble.pbenum.dart';

export 'ble.pbenum.dart';

enum Ble_Message {
  infoRequest, 
  infoResponse, 
  startRequest, 
  startResult, 
  advertisingSet, 
  advertisingSend, 
  advertisingReceived, 
  directSend, 
  directReceived, 
  notSet
}

class Ble extends $pb.GeneratedMessage {
  static const $core.Map<$core.int, Ble_Message> _Ble_MessageByTag = {
    1 : Ble_Message.infoRequest,
    2 : Ble_Message.infoResponse,
    3 : Ble_Message.startRequest,
    4 : Ble_Message.startResult,
    5 : Ble_Message.advertisingSet,
    6 : Ble_Message.advertisingSend,
    7 : Ble_Message.advertisingReceived,
    8 : Ble_Message.directSend,
    9 : Ble_Message.directReceived,
    0 : Ble_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Ble', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.sys.ble'), createEmptyInstance: create)
    ..oo(0, [1, 2, 3, 4, 5, 6, 7, 8, 9])
    ..aOM<BleInfoRequest>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'infoRequest', subBuilder: BleInfoRequest.create)
    ..aOM<BleInfoResponse>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'infoResponse', subBuilder: BleInfoResponse.create)
    ..aOM<BleStartRequest>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'startRequest', subBuilder: BleStartRequest.create)
    ..aOM<BleStartResult>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'startResult', subBuilder: BleStartResult.create)
    ..aOM<BleAdvertisingSet>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'advertisingSet', subBuilder: BleAdvertisingSet.create)
    ..aOM<BleAdvertisingSend>(6, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'advertisingSend', subBuilder: BleAdvertisingSend.create)
    ..aOM<BleAdvertisingReceived>(7, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'advertisingReceived', subBuilder: BleAdvertisingReceived.create)
    ..aOM<BleDirectSend>(8, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'directSend', subBuilder: BleDirectSend.create)
    ..aOM<BleDirectReceived>(9, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'directReceived', subBuilder: BleDirectReceived.create)
    ..hasRequiredFields = false
  ;

  Ble._() : super();
  factory Ble({
    BleInfoRequest? infoRequest,
    BleInfoResponse? infoResponse,
    BleStartRequest? startRequest,
    BleStartResult? startResult,
    BleAdvertisingSet? advertisingSet,
    BleAdvertisingSend? advertisingSend,
    BleAdvertisingReceived? advertisingReceived,
    BleDirectSend? directSend,
    BleDirectReceived? directReceived,
  }) {
    final _result = create();
    if (infoRequest != null) {
      _result.infoRequest = infoRequest;
    }
    if (infoResponse != null) {
      _result.infoResponse = infoResponse;
    }
    if (startRequest != null) {
      _result.startRequest = startRequest;
    }
    if (startResult != null) {
      _result.startResult = startResult;
    }
    if (advertisingSet != null) {
      _result.advertisingSet = advertisingSet;
    }
    if (advertisingSend != null) {
      _result.advertisingSend = advertisingSend;
    }
    if (advertisingReceived != null) {
      _result.advertisingReceived = advertisingReceived;
    }
    if (directSend != null) {
      _result.directSend = directSend;
    }
    if (directReceived != null) {
      _result.directReceived = directReceived;
    }
    return _result;
  }
  factory Ble.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Ble.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Ble clone() => Ble()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Ble copyWith(void Function(Ble) updates) => super.copyWith((message) => updates(message as Ble)) as Ble; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Ble create() => Ble._();
  Ble createEmptyInstance() => create();
  static $pb.PbList<Ble> createRepeated() => $pb.PbList<Ble>();
  @$core.pragma('dart2js:noInline')
  static Ble getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Ble>(create);
  static Ble? _defaultInstance;

  Ble_Message whichMessage() => _Ble_MessageByTag[$_whichOneof(0)]!;
  void clearMessage() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  BleInfoRequest get infoRequest => $_getN(0);
  @$pb.TagNumber(1)
  set infoRequest(BleInfoRequest v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasInfoRequest() => $_has(0);
  @$pb.TagNumber(1)
  void clearInfoRequest() => clearField(1);
  @$pb.TagNumber(1)
  BleInfoRequest ensureInfoRequest() => $_ensure(0);

  @$pb.TagNumber(2)
  BleInfoResponse get infoResponse => $_getN(1);
  @$pb.TagNumber(2)
  set infoResponse(BleInfoResponse v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasInfoResponse() => $_has(1);
  @$pb.TagNumber(2)
  void clearInfoResponse() => clearField(2);
  @$pb.TagNumber(2)
  BleInfoResponse ensureInfoResponse() => $_ensure(1);

  @$pb.TagNumber(3)
  BleStartRequest get startRequest => $_getN(2);
  @$pb.TagNumber(3)
  set startRequest(BleStartRequest v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasStartRequest() => $_has(2);
  @$pb.TagNumber(3)
  void clearStartRequest() => clearField(3);
  @$pb.TagNumber(3)
  BleStartRequest ensureStartRequest() => $_ensure(2);

  @$pb.TagNumber(4)
  BleStartResult get startResult => $_getN(3);
  @$pb.TagNumber(4)
  set startResult(BleStartResult v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasStartResult() => $_has(3);
  @$pb.TagNumber(4)
  void clearStartResult() => clearField(4);
  @$pb.TagNumber(4)
  BleStartResult ensureStartResult() => $_ensure(3);

  @$pb.TagNumber(5)
  BleAdvertisingSet get advertisingSet => $_getN(4);
  @$pb.TagNumber(5)
  set advertisingSet(BleAdvertisingSet v) { setField(5, v); }
  @$pb.TagNumber(5)
  $core.bool hasAdvertisingSet() => $_has(4);
  @$pb.TagNumber(5)
  void clearAdvertisingSet() => clearField(5);
  @$pb.TagNumber(5)
  BleAdvertisingSet ensureAdvertisingSet() => $_ensure(4);

  @$pb.TagNumber(6)
  BleAdvertisingSend get advertisingSend => $_getN(5);
  @$pb.TagNumber(6)
  set advertisingSend(BleAdvertisingSend v) { setField(6, v); }
  @$pb.TagNumber(6)
  $core.bool hasAdvertisingSend() => $_has(5);
  @$pb.TagNumber(6)
  void clearAdvertisingSend() => clearField(6);
  @$pb.TagNumber(6)
  BleAdvertisingSend ensureAdvertisingSend() => $_ensure(5);

  @$pb.TagNumber(7)
  BleAdvertisingReceived get advertisingReceived => $_getN(6);
  @$pb.TagNumber(7)
  set advertisingReceived(BleAdvertisingReceived v) { setField(7, v); }
  @$pb.TagNumber(7)
  $core.bool hasAdvertisingReceived() => $_has(6);
  @$pb.TagNumber(7)
  void clearAdvertisingReceived() => clearField(7);
  @$pb.TagNumber(7)
  BleAdvertisingReceived ensureAdvertisingReceived() => $_ensure(6);

  @$pb.TagNumber(8)
  BleDirectSend get directSend => $_getN(7);
  @$pb.TagNumber(8)
  set directSend(BleDirectSend v) { setField(8, v); }
  @$pb.TagNumber(8)
  $core.bool hasDirectSend() => $_has(7);
  @$pb.TagNumber(8)
  void clearDirectSend() => clearField(8);
  @$pb.TagNumber(8)
  BleDirectSend ensureDirectSend() => $_ensure(7);

  @$pb.TagNumber(9)
  BleDirectReceived get directReceived => $_getN(8);
  @$pb.TagNumber(9)
  set directReceived(BleDirectReceived v) { setField(9, v); }
  @$pb.TagNumber(9)
  $core.bool hasDirectReceived() => $_has(8);
  @$pb.TagNumber(9)
  void clearDirectReceived() => clearField(9);
  @$pb.TagNumber(9)
  BleDirectReceived ensureDirectReceived() => $_ensure(8);
}

class BleInfoRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'BleInfoRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.sys.ble'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  BleInfoRequest._() : super();
  factory BleInfoRequest() => create();
  factory BleInfoRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory BleInfoRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  BleInfoRequest clone() => BleInfoRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  BleInfoRequest copyWith(void Function(BleInfoRequest) updates) => super.copyWith((message) => updates(message as BleInfoRequest)) as BleInfoRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static BleInfoRequest create() => BleInfoRequest._();
  BleInfoRequest createEmptyInstance() => create();
  static $pb.PbList<BleInfoRequest> createRepeated() => $pb.PbList<BleInfoRequest>();
  @$core.pragma('dart2js:noInline')
  static BleInfoRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<BleInfoRequest>(create);
  static BleInfoRequest? _defaultInstance;
}

class BleInfoResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'BleInfoResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.sys.ble'), createEmptyInstance: create)
    ..pc<BleDeviceInfo>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'device', $pb.PbFieldType.PM, subBuilder: BleDeviceInfo.create)
    ..hasRequiredFields = false
  ;

  BleInfoResponse._() : super();
  factory BleInfoResponse({
    $core.Iterable<BleDeviceInfo>? device,
  }) {
    final _result = create();
    if (device != null) {
      _result.device.addAll(device);
    }
    return _result;
  }
  factory BleInfoResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory BleInfoResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  BleInfoResponse clone() => BleInfoResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  BleInfoResponse copyWith(void Function(BleInfoResponse) updates) => super.copyWith((message) => updates(message as BleInfoResponse)) as BleInfoResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static BleInfoResponse create() => BleInfoResponse._();
  BleInfoResponse createEmptyInstance() => create();
  static $pb.PbList<BleInfoResponse> createRepeated() => $pb.PbList<BleInfoResponse>();
  @$core.pragma('dart2js:noInline')
  static BleInfoResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<BleInfoResponse>(create);
  static BleInfoResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<BleDeviceInfo> get device => $_getList(0);
}

class BleDeviceInfo extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'BleDeviceInfo', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.sys.ble'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'id')
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'name')
    ..aOB(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'powered')
    ..aOB(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'bleSupport')
    ..aOB(7, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'adv251', protoName: 'adv_251')
    ..aOB(8, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'advExtended')
    ..a<$core.int>(9, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'advExtendedBytes', $pb.PbFieldType.OU3)
    ..aOB(10, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'adv1m', protoName: 'adv_1m')
    ..aOB(11, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'adv2m', protoName: 'adv_2m')
    ..aOB(12, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'advCoded')
    ..aOB(13, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'leAudio')
    ..hasRequiredFields = false
  ;

  BleDeviceInfo._() : super();
  factory BleDeviceInfo({
    $core.String? id,
    $core.String? name,
    $core.bool? powered,
    $core.bool? bleSupport,
    $core.bool? adv251,
    $core.bool? advExtended,
    $core.int? advExtendedBytes,
    $core.bool? adv1m,
    $core.bool? adv2m,
    $core.bool? advCoded,
    $core.bool? leAudio,
  }) {
    final _result = create();
    if (id != null) {
      _result.id = id;
    }
    if (name != null) {
      _result.name = name;
    }
    if (powered != null) {
      _result.powered = powered;
    }
    if (bleSupport != null) {
      _result.bleSupport = bleSupport;
    }
    if (adv251 != null) {
      _result.adv251 = adv251;
    }
    if (advExtended != null) {
      _result.advExtended = advExtended;
    }
    if (advExtendedBytes != null) {
      _result.advExtendedBytes = advExtendedBytes;
    }
    if (adv1m != null) {
      _result.adv1m = adv1m;
    }
    if (adv2m != null) {
      _result.adv2m = adv2m;
    }
    if (advCoded != null) {
      _result.advCoded = advCoded;
    }
    if (leAudio != null) {
      _result.leAudio = leAudio;
    }
    return _result;
  }
  factory BleDeviceInfo.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory BleDeviceInfo.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  BleDeviceInfo clone() => BleDeviceInfo()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  BleDeviceInfo copyWith(void Function(BleDeviceInfo) updates) => super.copyWith((message) => updates(message as BleDeviceInfo)) as BleDeviceInfo; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static BleDeviceInfo create() => BleDeviceInfo._();
  BleDeviceInfo createEmptyInstance() => create();
  static $pb.PbList<BleDeviceInfo> createRepeated() => $pb.PbList<BleDeviceInfo>();
  @$core.pragma('dart2js:noInline')
  static BleDeviceInfo getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<BleDeviceInfo>(create);
  static BleDeviceInfo? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get id => $_getSZ(0);
  @$pb.TagNumber(1)
  set id($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get name => $_getSZ(1);
  @$pb.TagNumber(2)
  set name($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasName() => $_has(1);
  @$pb.TagNumber(2)
  void clearName() => clearField(2);

  @$pb.TagNumber(3)
  $core.bool get powered => $_getBF(2);
  @$pb.TagNumber(3)
  set powered($core.bool v) { $_setBool(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasPowered() => $_has(2);
  @$pb.TagNumber(3)
  void clearPowered() => clearField(3);

  @$pb.TagNumber(4)
  $core.bool get bleSupport => $_getBF(3);
  @$pb.TagNumber(4)
  set bleSupport($core.bool v) { $_setBool(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasBleSupport() => $_has(3);
  @$pb.TagNumber(4)
  void clearBleSupport() => clearField(4);

  @$pb.TagNumber(7)
  $core.bool get adv251 => $_getBF(4);
  @$pb.TagNumber(7)
  set adv251($core.bool v) { $_setBool(4, v); }
  @$pb.TagNumber(7)
  $core.bool hasAdv251() => $_has(4);
  @$pb.TagNumber(7)
  void clearAdv251() => clearField(7);

  @$pb.TagNumber(8)
  $core.bool get advExtended => $_getBF(5);
  @$pb.TagNumber(8)
  set advExtended($core.bool v) { $_setBool(5, v); }
  @$pb.TagNumber(8)
  $core.bool hasAdvExtended() => $_has(5);
  @$pb.TagNumber(8)
  void clearAdvExtended() => clearField(8);

  @$pb.TagNumber(9)
  $core.int get advExtendedBytes => $_getIZ(6);
  @$pb.TagNumber(9)
  set advExtendedBytes($core.int v) { $_setUnsignedInt32(6, v); }
  @$pb.TagNumber(9)
  $core.bool hasAdvExtendedBytes() => $_has(6);
  @$pb.TagNumber(9)
  void clearAdvExtendedBytes() => clearField(9);

  @$pb.TagNumber(10)
  $core.bool get adv1m => $_getBF(7);
  @$pb.TagNumber(10)
  set adv1m($core.bool v) { $_setBool(7, v); }
  @$pb.TagNumber(10)
  $core.bool hasAdv1m() => $_has(7);
  @$pb.TagNumber(10)
  void clearAdv1m() => clearField(10);

  @$pb.TagNumber(11)
  $core.bool get adv2m => $_getBF(8);
  @$pb.TagNumber(11)
  set adv2m($core.bool v) { $_setBool(8, v); }
  @$pb.TagNumber(11)
  $core.bool hasAdv2m() => $_has(8);
  @$pb.TagNumber(11)
  void clearAdv2m() => clearField(11);

  @$pb.TagNumber(12)
  $core.bool get advCoded => $_getBF(9);
  @$pb.TagNumber(12)
  set advCoded($core.bool v) { $_setBool(9, v); }
  @$pb.TagNumber(12)
  $core.bool hasAdvCoded() => $_has(9);
  @$pb.TagNumber(12)
  void clearAdvCoded() => clearField(12);

  @$pb.TagNumber(13)
  $core.bool get leAudio => $_getBF(10);
  @$pb.TagNumber(13)
  set leAudio($core.bool v) { $_setBool(10, v); }
  @$pb.TagNumber(13)
  $core.bool hasLeAudio() => $_has(10);
  @$pb.TagNumber(13)
  void clearLeAudio() => clearField(13);
}

class BleStartRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'BleStartRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.sys.ble'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  BleStartRequest._() : super();
  factory BleStartRequest() => create();
  factory BleStartRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory BleStartRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  BleStartRequest clone() => BleStartRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  BleStartRequest copyWith(void Function(BleStartRequest) updates) => super.copyWith((message) => updates(message as BleStartRequest)) as BleStartRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static BleStartRequest create() => BleStartRequest._();
  BleStartRequest createEmptyInstance() => create();
  static $pb.PbList<BleStartRequest> createRepeated() => $pb.PbList<BleStartRequest>();
  @$core.pragma('dart2js:noInline')
  static BleStartRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<BleStartRequest>(create);
  static BleStartRequest? _defaultInstance;
}

class BleStartResult extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'BleStartResult', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.sys.ble'), createEmptyInstance: create)
    ..aOB(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'success')
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'errorMessage')
    ..aOB(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'unknonwError')
    ..aOB(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'noRights')
    ..hasRequiredFields = false
  ;

  BleStartResult._() : super();
  factory BleStartResult({
    $core.bool? success,
    $core.String? errorMessage,
    $core.bool? unknonwError,
    $core.bool? noRights,
  }) {
    final _result = create();
    if (success != null) {
      _result.success = success;
    }
    if (errorMessage != null) {
      _result.errorMessage = errorMessage;
    }
    if (unknonwError != null) {
      _result.unknonwError = unknonwError;
    }
    if (noRights != null) {
      _result.noRights = noRights;
    }
    return _result;
  }
  factory BleStartResult.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory BleStartResult.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  BleStartResult clone() => BleStartResult()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  BleStartResult copyWith(void Function(BleStartResult) updates) => super.copyWith((message) => updates(message as BleStartResult)) as BleStartResult; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static BleStartResult create() => BleStartResult._();
  BleStartResult createEmptyInstance() => create();
  static $pb.PbList<BleStartResult> createRepeated() => $pb.PbList<BleStartResult>();
  @$core.pragma('dart2js:noInline')
  static BleStartResult getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<BleStartResult>(create);
  static BleStartResult? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get success => $_getBF(0);
  @$pb.TagNumber(1)
  set success($core.bool v) { $_setBool(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasSuccess() => $_has(0);
  @$pb.TagNumber(1)
  void clearSuccess() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get errorMessage => $_getSZ(1);
  @$pb.TagNumber(2)
  set errorMessage($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasErrorMessage() => $_has(1);
  @$pb.TagNumber(2)
  void clearErrorMessage() => clearField(2);

  @$pb.TagNumber(3)
  $core.bool get unknonwError => $_getBF(2);
  @$pb.TagNumber(3)
  set unknonwError($core.bool v) { $_setBool(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasUnknonwError() => $_has(2);
  @$pb.TagNumber(3)
  void clearUnknonwError() => clearField(3);

  @$pb.TagNumber(4)
  $core.bool get noRights => $_getBF(3);
  @$pb.TagNumber(4)
  set noRights($core.bool v) { $_setBool(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasNoRights() => $_has(3);
  @$pb.TagNumber(4)
  void clearNoRights() => clearField(4);
}

class BleAdvertisingSet extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'BleAdvertisingSet', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.sys.ble'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'data', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  BleAdvertisingSet._() : super();
  factory BleAdvertisingSet({
    $core.List<$core.int>? data,
  }) {
    final _result = create();
    if (data != null) {
      _result.data = data;
    }
    return _result;
  }
  factory BleAdvertisingSet.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory BleAdvertisingSet.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  BleAdvertisingSet clone() => BleAdvertisingSet()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  BleAdvertisingSet copyWith(void Function(BleAdvertisingSet) updates) => super.copyWith((message) => updates(message as BleAdvertisingSet)) as BleAdvertisingSet; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static BleAdvertisingSet create() => BleAdvertisingSet._();
  BleAdvertisingSet createEmptyInstance() => create();
  static $pb.PbList<BleAdvertisingSet> createRepeated() => $pb.PbList<BleAdvertisingSet>();
  @$core.pragma('dart2js:noInline')
  static BleAdvertisingSet getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<BleAdvertisingSet>(create);
  static BleAdvertisingSet? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get data => $_getN(0);
  @$pb.TagNumber(1)
  set data($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasData() => $_has(0);
  @$pb.TagNumber(1)
  void clearData() => clearField(1);
}

class BleAdvertisingSend extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'BleAdvertisingSend', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.sys.ble'), createEmptyInstance: create)
    ..e<BleMode>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'mode', $pb.PbFieldType.OE, defaultOrMaker: BleMode.legacy, valueOf: BleMode.valueOf, enumValues: BleMode.values)
    ..a<$core.List<$core.int>>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'data', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  BleAdvertisingSend._() : super();
  factory BleAdvertisingSend({
    BleMode? mode,
    $core.List<$core.int>? data,
  }) {
    final _result = create();
    if (mode != null) {
      _result.mode = mode;
    }
    if (data != null) {
      _result.data = data;
    }
    return _result;
  }
  factory BleAdvertisingSend.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory BleAdvertisingSend.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  BleAdvertisingSend clone() => BleAdvertisingSend()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  BleAdvertisingSend copyWith(void Function(BleAdvertisingSend) updates) => super.copyWith((message) => updates(message as BleAdvertisingSend)) as BleAdvertisingSend; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static BleAdvertisingSend create() => BleAdvertisingSend._();
  BleAdvertisingSend createEmptyInstance() => create();
  static $pb.PbList<BleAdvertisingSend> createRepeated() => $pb.PbList<BleAdvertisingSend>();
  @$core.pragma('dart2js:noInline')
  static BleAdvertisingSend getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<BleAdvertisingSend>(create);
  static BleAdvertisingSend? _defaultInstance;

  @$pb.TagNumber(1)
  BleMode get mode => $_getN(0);
  @$pb.TagNumber(1)
  set mode(BleMode v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasMode() => $_has(0);
  @$pb.TagNumber(1)
  void clearMode() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get data => $_getN(1);
  @$pb.TagNumber(2)
  set data($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasData() => $_has(1);
  @$pb.TagNumber(2)
  void clearData() => clearField(2);
}

class BleAdvertisingReceived extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'BleAdvertisingReceived', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.sys.ble'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'id', $pb.PbFieldType.OY)
    ..a<$core.int>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'rssi', $pb.PbFieldType.O3)
    ..e<BleMode>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'mode', $pb.PbFieldType.OE, defaultOrMaker: BleMode.legacy, valueOf: BleMode.valueOf, enumValues: BleMode.values)
    ..a<$core.List<$core.int>>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'data', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  BleAdvertisingReceived._() : super();
  factory BleAdvertisingReceived({
    $core.List<$core.int>? id,
    $core.int? rssi,
    BleMode? mode,
    $core.List<$core.int>? data,
  }) {
    final _result = create();
    if (id != null) {
      _result.id = id;
    }
    if (rssi != null) {
      _result.rssi = rssi;
    }
    if (mode != null) {
      _result.mode = mode;
    }
    if (data != null) {
      _result.data = data;
    }
    return _result;
  }
  factory BleAdvertisingReceived.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory BleAdvertisingReceived.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  BleAdvertisingReceived clone() => BleAdvertisingReceived()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  BleAdvertisingReceived copyWith(void Function(BleAdvertisingReceived) updates) => super.copyWith((message) => updates(message as BleAdvertisingReceived)) as BleAdvertisingReceived; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static BleAdvertisingReceived create() => BleAdvertisingReceived._();
  BleAdvertisingReceived createEmptyInstance() => create();
  static $pb.PbList<BleAdvertisingReceived> createRepeated() => $pb.PbList<BleAdvertisingReceived>();
  @$core.pragma('dart2js:noInline')
  static BleAdvertisingReceived getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<BleAdvertisingReceived>(create);
  static BleAdvertisingReceived? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get id => $_getN(0);
  @$pb.TagNumber(1)
  set id($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get rssi => $_getIZ(1);
  @$pb.TagNumber(2)
  set rssi($core.int v) { $_setSignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasRssi() => $_has(1);
  @$pb.TagNumber(2)
  void clearRssi() => clearField(2);

  @$pb.TagNumber(3)
  BleMode get mode => $_getN(2);
  @$pb.TagNumber(3)
  set mode(BleMode v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasMode() => $_has(2);
  @$pb.TagNumber(3)
  void clearMode() => clearField(3);

  @$pb.TagNumber(4)
  $core.List<$core.int> get data => $_getN(3);
  @$pb.TagNumber(4)
  set data($core.List<$core.int> v) { $_setBytes(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasData() => $_has(3);
  @$pb.TagNumber(4)
  void clearData() => clearField(4);
}

class BleDirectSend extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'BleDirectSend', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.sys.ble'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'id', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'to', $pb.PbFieldType.OY)
    ..e<BleMode>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'mode', $pb.PbFieldType.OE, defaultOrMaker: BleMode.legacy, valueOf: BleMode.valueOf, enumValues: BleMode.values)
    ..a<$core.List<$core.int>>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'data', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  BleDirectSend._() : super();
  factory BleDirectSend({
    $core.List<$core.int>? id,
    $core.List<$core.int>? to,
    BleMode? mode,
    $core.List<$core.int>? data,
  }) {
    final _result = create();
    if (id != null) {
      _result.id = id;
    }
    if (to != null) {
      _result.to = to;
    }
    if (mode != null) {
      _result.mode = mode;
    }
    if (data != null) {
      _result.data = data;
    }
    return _result;
  }
  factory BleDirectSend.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory BleDirectSend.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  BleDirectSend clone() => BleDirectSend()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  BleDirectSend copyWith(void Function(BleDirectSend) updates) => super.copyWith((message) => updates(message as BleDirectSend)) as BleDirectSend; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static BleDirectSend create() => BleDirectSend._();
  BleDirectSend createEmptyInstance() => create();
  static $pb.PbList<BleDirectSend> createRepeated() => $pb.PbList<BleDirectSend>();
  @$core.pragma('dart2js:noInline')
  static BleDirectSend getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<BleDirectSend>(create);
  static BleDirectSend? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get id => $_getN(0);
  @$pb.TagNumber(1)
  set id($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get to => $_getN(1);
  @$pb.TagNumber(2)
  set to($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasTo() => $_has(1);
  @$pb.TagNumber(2)
  void clearTo() => clearField(2);

  @$pb.TagNumber(3)
  BleMode get mode => $_getN(2);
  @$pb.TagNumber(3)
  set mode(BleMode v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasMode() => $_has(2);
  @$pb.TagNumber(3)
  void clearMode() => clearField(3);

  @$pb.TagNumber(4)
  $core.List<$core.int> get data => $_getN(3);
  @$pb.TagNumber(4)
  set data($core.List<$core.int> v) { $_setBytes(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasData() => $_has(3);
  @$pb.TagNumber(4)
  void clearData() => clearField(4);
}

class BleDirectSendResult extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'BleDirectSendResult', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.sys.ble'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'id', $pb.PbFieldType.OY)
    ..aOB(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'success')
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'errorMessage')
    ..hasRequiredFields = false
  ;

  BleDirectSendResult._() : super();
  factory BleDirectSendResult({
    $core.List<$core.int>? id,
    $core.bool? success,
    $core.String? errorMessage,
  }) {
    final _result = create();
    if (id != null) {
      _result.id = id;
    }
    if (success != null) {
      _result.success = success;
    }
    if (errorMessage != null) {
      _result.errorMessage = errorMessage;
    }
    return _result;
  }
  factory BleDirectSendResult.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory BleDirectSendResult.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  BleDirectSendResult clone() => BleDirectSendResult()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  BleDirectSendResult copyWith(void Function(BleDirectSendResult) updates) => super.copyWith((message) => updates(message as BleDirectSendResult)) as BleDirectSendResult; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static BleDirectSendResult create() => BleDirectSendResult._();
  BleDirectSendResult createEmptyInstance() => create();
  static $pb.PbList<BleDirectSendResult> createRepeated() => $pb.PbList<BleDirectSendResult>();
  @$core.pragma('dart2js:noInline')
  static BleDirectSendResult getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<BleDirectSendResult>(create);
  static BleDirectSendResult? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get id => $_getN(0);
  @$pb.TagNumber(1)
  set id($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => clearField(1);

  @$pb.TagNumber(2)
  $core.bool get success => $_getBF(1);
  @$pb.TagNumber(2)
  set success($core.bool v) { $_setBool(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasSuccess() => $_has(1);
  @$pb.TagNumber(2)
  void clearSuccess() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get errorMessage => $_getSZ(2);
  @$pb.TagNumber(3)
  set errorMessage($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasErrorMessage() => $_has(2);
  @$pb.TagNumber(3)
  void clearErrorMessage() => clearField(3);
}

class BleDirectReceived extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'BleDirectReceived', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.sys.ble'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'from', $pb.PbFieldType.OY)
    ..a<$core.int>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'rssi', $pb.PbFieldType.O3)
    ..e<BleMode>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'mode', $pb.PbFieldType.OE, defaultOrMaker: BleMode.legacy, valueOf: BleMode.valueOf, enumValues: BleMode.values)
    ..a<$core.List<$core.int>>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'data', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  BleDirectReceived._() : super();
  factory BleDirectReceived({
    $core.List<$core.int>? from,
    $core.int? rssi,
    BleMode? mode,
    $core.List<$core.int>? data,
  }) {
    final _result = create();
    if (from != null) {
      _result.from = from;
    }
    if (rssi != null) {
      _result.rssi = rssi;
    }
    if (mode != null) {
      _result.mode = mode;
    }
    if (data != null) {
      _result.data = data;
    }
    return _result;
  }
  factory BleDirectReceived.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory BleDirectReceived.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  BleDirectReceived clone() => BleDirectReceived()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  BleDirectReceived copyWith(void Function(BleDirectReceived) updates) => super.copyWith((message) => updates(message as BleDirectReceived)) as BleDirectReceived; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static BleDirectReceived create() => BleDirectReceived._();
  BleDirectReceived createEmptyInstance() => create();
  static $pb.PbList<BleDirectReceived> createRepeated() => $pb.PbList<BleDirectReceived>();
  @$core.pragma('dart2js:noInline')
  static BleDirectReceived getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<BleDirectReceived>(create);
  static BleDirectReceived? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get from => $_getN(0);
  @$pb.TagNumber(1)
  set from($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasFrom() => $_has(0);
  @$pb.TagNumber(1)
  void clearFrom() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get rssi => $_getIZ(1);
  @$pb.TagNumber(2)
  set rssi($core.int v) { $_setSignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasRssi() => $_has(1);
  @$pb.TagNumber(2)
  void clearRssi() => clearField(2);

  @$pb.TagNumber(3)
  BleMode get mode => $_getN(2);
  @$pb.TagNumber(3)
  set mode(BleMode v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasMode() => $_has(2);
  @$pb.TagNumber(3)
  void clearMode() => clearField(3);

  @$pb.TagNumber(4)
  $core.List<$core.int> get data => $_getN(3);
  @$pb.TagNumber(4)
  set data($core.List<$core.int> v) { $_setBytes(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasData() => $_has(3);
  @$pb.TagNumber(4)
  void clearData() => clearField(4);
}

