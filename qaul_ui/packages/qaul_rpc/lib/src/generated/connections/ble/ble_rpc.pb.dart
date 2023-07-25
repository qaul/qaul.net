///
//  Generated code. Do not modify.
//  source: connections/ble/ble_rpc.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

enum Ble_Message {
  infoRequest, 
  infoResponse, 
  startRequest, 
  stopRequest, 
  discoveredRequest, 
  discoveredResponse, 
  rightsRequest, 
  rightsResult, 
  notSet
}

class Ble extends $pb.GeneratedMessage {
  static const $core.Map<$core.int, Ble_Message> _Ble_MessageByTag = {
    1 : Ble_Message.infoRequest,
    2 : Ble_Message.infoResponse,
    3 : Ble_Message.startRequest,
    4 : Ble_Message.stopRequest,
    5 : Ble_Message.discoveredRequest,
    6 : Ble_Message.discoveredResponse,
    7 : Ble_Message.rightsRequest,
    8 : Ble_Message.rightsResult,
    0 : Ble_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Ble', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.ble'), createEmptyInstance: create)
    ..oo(0, [1, 2, 3, 4, 5, 6, 7, 8])
    ..aOM<InfoRequest>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'infoRequest', subBuilder: InfoRequest.create)
    ..aOM<InfoResponse>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'infoResponse', subBuilder: InfoResponse.create)
    ..aOM<StartRequest>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'startRequest', subBuilder: StartRequest.create)
    ..aOM<StopRequest>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'stopRequest', subBuilder: StopRequest.create)
    ..aOM<DiscoveredRequest>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'discoveredRequest', subBuilder: DiscoveredRequest.create)
    ..aOM<DiscoveredResponse>(6, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'discoveredResponse', subBuilder: DiscoveredResponse.create)
    ..aOM<RightsRequest>(7, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'rightsRequest', subBuilder: RightsRequest.create)
    ..aOM<RightsResult>(8, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'rightsResult', subBuilder: RightsResult.create)
    ..hasRequiredFields = false
  ;

  Ble._() : super();
  factory Ble({
    InfoRequest? infoRequest,
    InfoResponse? infoResponse,
    StartRequest? startRequest,
    StopRequest? stopRequest,
    DiscoveredRequest? discoveredRequest,
    DiscoveredResponse? discoveredResponse,
    RightsRequest? rightsRequest,
    RightsResult? rightsResult,
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
    if (stopRequest != null) {
      _result.stopRequest = stopRequest;
    }
    if (discoveredRequest != null) {
      _result.discoveredRequest = discoveredRequest;
    }
    if (discoveredResponse != null) {
      _result.discoveredResponse = discoveredResponse;
    }
    if (rightsRequest != null) {
      _result.rightsRequest = rightsRequest;
    }
    if (rightsResult != null) {
      _result.rightsResult = rightsResult;
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
  InfoRequest get infoRequest => $_getN(0);
  @$pb.TagNumber(1)
  set infoRequest(InfoRequest v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasInfoRequest() => $_has(0);
  @$pb.TagNumber(1)
  void clearInfoRequest() => clearField(1);
  @$pb.TagNumber(1)
  InfoRequest ensureInfoRequest() => $_ensure(0);

  @$pb.TagNumber(2)
  InfoResponse get infoResponse => $_getN(1);
  @$pb.TagNumber(2)
  set infoResponse(InfoResponse v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasInfoResponse() => $_has(1);
  @$pb.TagNumber(2)
  void clearInfoResponse() => clearField(2);
  @$pb.TagNumber(2)
  InfoResponse ensureInfoResponse() => $_ensure(1);

  @$pb.TagNumber(3)
  StartRequest get startRequest => $_getN(2);
  @$pb.TagNumber(3)
  set startRequest(StartRequest v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasStartRequest() => $_has(2);
  @$pb.TagNumber(3)
  void clearStartRequest() => clearField(3);
  @$pb.TagNumber(3)
  StartRequest ensureStartRequest() => $_ensure(2);

  @$pb.TagNumber(4)
  StopRequest get stopRequest => $_getN(3);
  @$pb.TagNumber(4)
  set stopRequest(StopRequest v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasStopRequest() => $_has(3);
  @$pb.TagNumber(4)
  void clearStopRequest() => clearField(4);
  @$pb.TagNumber(4)
  StopRequest ensureStopRequest() => $_ensure(3);

  @$pb.TagNumber(5)
  DiscoveredRequest get discoveredRequest => $_getN(4);
  @$pb.TagNumber(5)
  set discoveredRequest(DiscoveredRequest v) { setField(5, v); }
  @$pb.TagNumber(5)
  $core.bool hasDiscoveredRequest() => $_has(4);
  @$pb.TagNumber(5)
  void clearDiscoveredRequest() => clearField(5);
  @$pb.TagNumber(5)
  DiscoveredRequest ensureDiscoveredRequest() => $_ensure(4);

  @$pb.TagNumber(6)
  DiscoveredResponse get discoveredResponse => $_getN(5);
  @$pb.TagNumber(6)
  set discoveredResponse(DiscoveredResponse v) { setField(6, v); }
  @$pb.TagNumber(6)
  $core.bool hasDiscoveredResponse() => $_has(5);
  @$pb.TagNumber(6)
  void clearDiscoveredResponse() => clearField(6);
  @$pb.TagNumber(6)
  DiscoveredResponse ensureDiscoveredResponse() => $_ensure(5);

  @$pb.TagNumber(7)
  RightsRequest get rightsRequest => $_getN(6);
  @$pb.TagNumber(7)
  set rightsRequest(RightsRequest v) { setField(7, v); }
  @$pb.TagNumber(7)
  $core.bool hasRightsRequest() => $_has(6);
  @$pb.TagNumber(7)
  void clearRightsRequest() => clearField(7);
  @$pb.TagNumber(7)
  RightsRequest ensureRightsRequest() => $_ensure(6);

  @$pb.TagNumber(8)
  RightsResult get rightsResult => $_getN(7);
  @$pb.TagNumber(8)
  set rightsResult(RightsResult v) { setField(8, v); }
  @$pb.TagNumber(8)
  $core.bool hasRightsResult() => $_has(7);
  @$pb.TagNumber(8)
  void clearRightsResult() => clearField(8);
  @$pb.TagNumber(8)
  RightsResult ensureRightsResult() => $_ensure(7);
}

class InfoRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'InfoRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.ble'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  InfoRequest._() : super();
  factory InfoRequest() => create();
  factory InfoRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory InfoRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  InfoRequest clone() => InfoRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  InfoRequest copyWith(void Function(InfoRequest) updates) => super.copyWith((message) => updates(message as InfoRequest)) as InfoRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static InfoRequest create() => InfoRequest._();
  InfoRequest createEmptyInstance() => create();
  static $pb.PbList<InfoRequest> createRepeated() => $pb.PbList<InfoRequest>();
  @$core.pragma('dart2js:noInline')
  static InfoRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<InfoRequest>(create);
  static InfoRequest? _defaultInstance;
}

class InfoResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'InfoResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.ble'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'smallId', $pb.PbFieldType.OY)
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'status')
    ..a<$core.List<$core.int>>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'deviceInfo', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  InfoResponse._() : super();
  factory InfoResponse({
    $core.List<$core.int>? smallId,
    $core.String? status,
    $core.List<$core.int>? deviceInfo,
  }) {
    final _result = create();
    if (smallId != null) {
      _result.smallId = smallId;
    }
    if (status != null) {
      _result.status = status;
    }
    if (deviceInfo != null) {
      _result.deviceInfo = deviceInfo;
    }
    return _result;
  }
  factory InfoResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory InfoResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  InfoResponse clone() => InfoResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  InfoResponse copyWith(void Function(InfoResponse) updates) => super.copyWith((message) => updates(message as InfoResponse)) as InfoResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static InfoResponse create() => InfoResponse._();
  InfoResponse createEmptyInstance() => create();
  static $pb.PbList<InfoResponse> createRepeated() => $pb.PbList<InfoResponse>();
  @$core.pragma('dart2js:noInline')
  static InfoResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<InfoResponse>(create);
  static InfoResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get smallId => $_getN(0);
  @$pb.TagNumber(1)
  set smallId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasSmallId() => $_has(0);
  @$pb.TagNumber(1)
  void clearSmallId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get status => $_getSZ(1);
  @$pb.TagNumber(2)
  set status($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasStatus() => $_has(1);
  @$pb.TagNumber(2)
  void clearStatus() => clearField(2);

  @$pb.TagNumber(3)
  $core.List<$core.int> get deviceInfo => $_getN(2);
  @$pb.TagNumber(3)
  set deviceInfo($core.List<$core.int> v) { $_setBytes(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasDeviceInfo() => $_has(2);
  @$pb.TagNumber(3)
  void clearDeviceInfo() => clearField(3);
}

class StartRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'StartRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.ble'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  StartRequest._() : super();
  factory StartRequest() => create();
  factory StartRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory StartRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  StartRequest clone() => StartRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  StartRequest copyWith(void Function(StartRequest) updates) => super.copyWith((message) => updates(message as StartRequest)) as StartRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static StartRequest create() => StartRequest._();
  StartRequest createEmptyInstance() => create();
  static $pb.PbList<StartRequest> createRepeated() => $pb.PbList<StartRequest>();
  @$core.pragma('dart2js:noInline')
  static StartRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<StartRequest>(create);
  static StartRequest? _defaultInstance;
}

class StopRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'StopRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.ble'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  StopRequest._() : super();
  factory StopRequest() => create();
  factory StopRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory StopRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  StopRequest clone() => StopRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  StopRequest copyWith(void Function(StopRequest) updates) => super.copyWith((message) => updates(message as StopRequest)) as StopRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static StopRequest create() => StopRequest._();
  StopRequest createEmptyInstance() => create();
  static $pb.PbList<StopRequest> createRepeated() => $pb.PbList<StopRequest>();
  @$core.pragma('dart2js:noInline')
  static StopRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<StopRequest>(create);
  static StopRequest? _defaultInstance;
}

class DiscoveredRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'DiscoveredRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.ble'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  DiscoveredRequest._() : super();
  factory DiscoveredRequest() => create();
  factory DiscoveredRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory DiscoveredRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  DiscoveredRequest clone() => DiscoveredRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  DiscoveredRequest copyWith(void Function(DiscoveredRequest) updates) => super.copyWith((message) => updates(message as DiscoveredRequest)) as DiscoveredRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static DiscoveredRequest create() => DiscoveredRequest._();
  DiscoveredRequest createEmptyInstance() => create();
  static $pb.PbList<DiscoveredRequest> createRepeated() => $pb.PbList<DiscoveredRequest>();
  @$core.pragma('dart2js:noInline')
  static DiscoveredRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DiscoveredRequest>(create);
  static DiscoveredRequest? _defaultInstance;
}

class DiscoveredResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'DiscoveredResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.ble'), createEmptyInstance: create)
    ..a<$core.int>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'nodesCount', $pb.PbFieldType.OU3)
    ..a<$core.int>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'toConfirmCount', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  DiscoveredResponse._() : super();
  factory DiscoveredResponse({
    $core.int? nodesCount,
    $core.int? toConfirmCount,
  }) {
    final _result = create();
    if (nodesCount != null) {
      _result.nodesCount = nodesCount;
    }
    if (toConfirmCount != null) {
      _result.toConfirmCount = toConfirmCount;
    }
    return _result;
  }
  factory DiscoveredResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory DiscoveredResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  DiscoveredResponse clone() => DiscoveredResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  DiscoveredResponse copyWith(void Function(DiscoveredResponse) updates) => super.copyWith((message) => updates(message as DiscoveredResponse)) as DiscoveredResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static DiscoveredResponse create() => DiscoveredResponse._();
  DiscoveredResponse createEmptyInstance() => create();
  static $pb.PbList<DiscoveredResponse> createRepeated() => $pb.PbList<DiscoveredResponse>();
  @$core.pragma('dart2js:noInline')
  static DiscoveredResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DiscoveredResponse>(create);
  static DiscoveredResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get nodesCount => $_getIZ(0);
  @$pb.TagNumber(1)
  set nodesCount($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasNodesCount() => $_has(0);
  @$pb.TagNumber(1)
  void clearNodesCount() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get toConfirmCount => $_getIZ(1);
  @$pb.TagNumber(2)
  set toConfirmCount($core.int v) { $_setUnsignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasToConfirmCount() => $_has(1);
  @$pb.TagNumber(2)
  void clearToConfirmCount() => clearField(2);
}

class RightsRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'RightsRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.ble'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  RightsRequest._() : super();
  factory RightsRequest() => create();
  factory RightsRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RightsRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RightsRequest clone() => RightsRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RightsRequest copyWith(void Function(RightsRequest) updates) => super.copyWith((message) => updates(message as RightsRequest)) as RightsRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static RightsRequest create() => RightsRequest._();
  RightsRequest createEmptyInstance() => create();
  static $pb.PbList<RightsRequest> createRepeated() => $pb.PbList<RightsRequest>();
  @$core.pragma('dart2js:noInline')
  static RightsRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RightsRequest>(create);
  static RightsRequest? _defaultInstance;
}

class RightsResult extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'RightsResult', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.ble'), createEmptyInstance: create)
    ..aOB(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'rightsGranted')
    ..hasRequiredFields = false
  ;

  RightsResult._() : super();
  factory RightsResult({
    $core.bool? rightsGranted,
  }) {
    final _result = create();
    if (rightsGranted != null) {
      _result.rightsGranted = rightsGranted;
    }
    return _result;
  }
  factory RightsResult.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RightsResult.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RightsResult clone() => RightsResult()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RightsResult copyWith(void Function(RightsResult) updates) => super.copyWith((message) => updates(message as RightsResult)) as RightsResult; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static RightsResult create() => RightsResult._();
  RightsResult createEmptyInstance() => create();
  static $pb.PbList<RightsResult> createRepeated() => $pb.PbList<RightsResult>();
  @$core.pragma('dart2js:noInline')
  static RightsResult getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RightsResult>(create);
  static RightsResult? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get rightsGranted => $_getBF(0);
  @$pb.TagNumber(1)
  set rightsGranted($core.bool v) { $_setBool(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasRightsGranted() => $_has(0);
  @$pb.TagNumber(1)
  void clearRightsGranted() => clearField(1);
}

