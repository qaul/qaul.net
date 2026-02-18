// This is a generated file - do not edit.
//
// Generated from connections/ble/ble_rpc.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

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

/// BLE RPC Message Container
///
/// Union of all messages that can be sent or received
/// via RPC between the UI and libqaul
class Ble extends $pb.GeneratedMessage {
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
    final result = create();
    if (infoRequest != null) result.infoRequest = infoRequest;
    if (infoResponse != null) result.infoResponse = infoResponse;
    if (startRequest != null) result.startRequest = startRequest;
    if (stopRequest != null) result.stopRequest = stopRequest;
    if (discoveredRequest != null) result.discoveredRequest = discoveredRequest;
    if (discoveredResponse != null)
      result.discoveredResponse = discoveredResponse;
    if (rightsRequest != null) result.rightsRequest = rightsRequest;
    if (rightsResult != null) result.rightsResult = rightsResult;
    return result;
  }

  Ble._();

  factory Ble.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory Ble.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, Ble_Message> _Ble_MessageByTag = {
    1: Ble_Message.infoRequest,
    2: Ble_Message.infoResponse,
    3: Ble_Message.startRequest,
    4: Ble_Message.stopRequest,
    5: Ble_Message.discoveredRequest,
    6: Ble_Message.discoveredResponse,
    7: Ble_Message.rightsRequest,
    8: Ble_Message.rightsResult,
    0: Ble_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Ble',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.ble'),
      createEmptyInstance: create)
    ..oo(0, [1, 2, 3, 4, 5, 6, 7, 8])
    ..aOM<InfoRequest>(1, _omitFieldNames ? '' : 'infoRequest',
        subBuilder: InfoRequest.create)
    ..aOM<InfoResponse>(2, _omitFieldNames ? '' : 'infoResponse',
        subBuilder: InfoResponse.create)
    ..aOM<StartRequest>(3, _omitFieldNames ? '' : 'startRequest',
        subBuilder: StartRequest.create)
    ..aOM<StopRequest>(4, _omitFieldNames ? '' : 'stopRequest',
        subBuilder: StopRequest.create)
    ..aOM<DiscoveredRequest>(5, _omitFieldNames ? '' : 'discoveredRequest',
        subBuilder: DiscoveredRequest.create)
    ..aOM<DiscoveredResponse>(6, _omitFieldNames ? '' : 'discoveredResponse',
        subBuilder: DiscoveredResponse.create)
    ..aOM<RightsRequest>(7, _omitFieldNames ? '' : 'rightsRequest',
        subBuilder: RightsRequest.create)
    ..aOM<RightsResult>(8, _omitFieldNames ? '' : 'rightsResult',
        subBuilder: RightsResult.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Ble clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Ble copyWith(void Function(Ble) updates) =>
      super.copyWith((message) => updates(message as Ble)) as Ble;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Ble create() => Ble._();
  @$core.override
  Ble createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static Ble getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Ble>(create);
  static Ble? _defaultInstance;

  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  @$pb.TagNumber(6)
  @$pb.TagNumber(7)
  @$pb.TagNumber(8)
  Ble_Message whichMessage() => _Ble_MessageByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  @$pb.TagNumber(6)
  @$pb.TagNumber(7)
  @$pb.TagNumber(8)
  void clearMessage() => $_clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  InfoRequest get infoRequest => $_getN(0);
  @$pb.TagNumber(1)
  set infoRequest(InfoRequest value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasInfoRequest() => $_has(0);
  @$pb.TagNumber(1)
  void clearInfoRequest() => $_clearField(1);
  @$pb.TagNumber(1)
  InfoRequest ensureInfoRequest() => $_ensure(0);

  @$pb.TagNumber(2)
  InfoResponse get infoResponse => $_getN(1);
  @$pb.TagNumber(2)
  set infoResponse(InfoResponse value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasInfoResponse() => $_has(1);
  @$pb.TagNumber(2)
  void clearInfoResponse() => $_clearField(2);
  @$pb.TagNumber(2)
  InfoResponse ensureInfoResponse() => $_ensure(1);

  @$pb.TagNumber(3)
  StartRequest get startRequest => $_getN(2);
  @$pb.TagNumber(3)
  set startRequest(StartRequest value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasStartRequest() => $_has(2);
  @$pb.TagNumber(3)
  void clearStartRequest() => $_clearField(3);
  @$pb.TagNumber(3)
  StartRequest ensureStartRequest() => $_ensure(2);

  @$pb.TagNumber(4)
  StopRequest get stopRequest => $_getN(3);
  @$pb.TagNumber(4)
  set stopRequest(StopRequest value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasStopRequest() => $_has(3);
  @$pb.TagNumber(4)
  void clearStopRequest() => $_clearField(4);
  @$pb.TagNumber(4)
  StopRequest ensureStopRequest() => $_ensure(3);

  @$pb.TagNumber(5)
  DiscoveredRequest get discoveredRequest => $_getN(4);
  @$pb.TagNumber(5)
  set discoveredRequest(DiscoveredRequest value) => $_setField(5, value);
  @$pb.TagNumber(5)
  $core.bool hasDiscoveredRequest() => $_has(4);
  @$pb.TagNumber(5)
  void clearDiscoveredRequest() => $_clearField(5);
  @$pb.TagNumber(5)
  DiscoveredRequest ensureDiscoveredRequest() => $_ensure(4);

  @$pb.TagNumber(6)
  DiscoveredResponse get discoveredResponse => $_getN(5);
  @$pb.TagNumber(6)
  set discoveredResponse(DiscoveredResponse value) => $_setField(6, value);
  @$pb.TagNumber(6)
  $core.bool hasDiscoveredResponse() => $_has(5);
  @$pb.TagNumber(6)
  void clearDiscoveredResponse() => $_clearField(6);
  @$pb.TagNumber(6)
  DiscoveredResponse ensureDiscoveredResponse() => $_ensure(5);

  @$pb.TagNumber(7)
  RightsRequest get rightsRequest => $_getN(6);
  @$pb.TagNumber(7)
  set rightsRequest(RightsRequest value) => $_setField(7, value);
  @$pb.TagNumber(7)
  $core.bool hasRightsRequest() => $_has(6);
  @$pb.TagNumber(7)
  void clearRightsRequest() => $_clearField(7);
  @$pb.TagNumber(7)
  RightsRequest ensureRightsRequest() => $_ensure(6);

  @$pb.TagNumber(8)
  RightsResult get rightsResult => $_getN(7);
  @$pb.TagNumber(8)
  set rightsResult(RightsResult value) => $_setField(8, value);
  @$pb.TagNumber(8)
  $core.bool hasRightsResult() => $_has(7);
  @$pb.TagNumber(8)
  void clearRightsResult() => $_clearField(8);
  @$pb.TagNumber(8)
  RightsResult ensureRightsResult() => $_ensure(7);
}

/// UI request for information on devices and module status
class InfoRequest extends $pb.GeneratedMessage {
  factory InfoRequest() => create();

  InfoRequest._();

  factory InfoRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory InfoRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'InfoRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.ble'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  InfoRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  InfoRequest copyWith(void Function(InfoRequest) updates) =>
      super.copyWith((message) => updates(message as InfoRequest))
          as InfoRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static InfoRequest create() => InfoRequest._();
  @$core.override
  InfoRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static InfoRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<InfoRequest>(create);
  static InfoRequest? _defaultInstance;
}

/// BLE Info Response Message
///
/// Contains information on the status of the module,
/// as well as all available BLE devices
class InfoResponse extends $pb.GeneratedMessage {
  factory InfoResponse({
    $core.List<$core.int>? smallId,
    $core.String? status,
    $core.List<$core.int>? deviceInfo,
  }) {
    final result = create();
    if (smallId != null) result.smallId = smallId;
    if (status != null) result.status = status;
    if (deviceInfo != null) result.deviceInfo = deviceInfo;
    return result;
  }

  InfoResponse._();

  factory InfoResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory InfoResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'InfoResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.ble'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'smallId', $pb.PbFieldType.OY)
    ..aOS(2, _omitFieldNames ? '' : 'status')
    ..a<$core.List<$core.int>>(
        3, _omitFieldNames ? '' : 'deviceInfo', $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  InfoResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  InfoResponse copyWith(void Function(InfoResponse) updates) =>
      super.copyWith((message) => updates(message as InfoResponse))
          as InfoResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static InfoResponse create() => InfoResponse._();
  @$core.override
  InfoResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static InfoResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<InfoResponse>(create);
  static InfoResponse? _defaultInstance;

  /// the small 16 byte BLE id
  @$pb.TagNumber(1)
  $core.List<$core.int> get smallId => $_getN(0);
  @$pb.TagNumber(1)
  set smallId($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSmallId() => $_has(0);
  @$pb.TagNumber(1)
  void clearSmallId() => $_clearField(1);

  /// status of the module
  @$pb.TagNumber(2)
  $core.String get status => $_getSZ(1);
  @$pb.TagNumber(2)
  set status($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasStatus() => $_has(1);
  @$pb.TagNumber(2)
  void clearStatus() => $_clearField(2);

  /// devices
  @$pb.TagNumber(3)
  $core.List<$core.int> get deviceInfo => $_getN(2);
  @$pb.TagNumber(3)
  set deviceInfo($core.List<$core.int> value) => $_setBytes(2, value);
  @$pb.TagNumber(3)
  $core.bool hasDeviceInfo() => $_has(2);
  @$pb.TagNumber(3)
  void clearDeviceInfo() => $_clearField(3);
}

/// Request BLE module to start
///
/// Start message sent from UI to libqaul.
///
/// This message only has an effect if the module
/// has not already started.
class StartRequest extends $pb.GeneratedMessage {
  factory StartRequest() => create();

  StartRequest._();

  factory StartRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory StartRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'StartRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.ble'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  StartRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  StartRequest copyWith(void Function(StartRequest) updates) =>
      super.copyWith((message) => updates(message as StartRequest))
          as StartRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static StartRequest create() => StartRequest._();
  @$core.override
  StartRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static StartRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<StartRequest>(create);
  static StartRequest? _defaultInstance;
}

/// Request BLE module to stop
///
/// Stop message sent from UI to libqaul.
///
/// This message only has an effect if the module
/// was started earlier and is running.
class StopRequest extends $pb.GeneratedMessage {
  factory StopRequest() => create();

  StopRequest._();

  factory StopRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory StopRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'StopRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.ble'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  StopRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  StopRequest copyWith(void Function(StopRequest) updates) =>
      super.copyWith((message) => updates(message as StopRequest))
          as StopRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static StopRequest create() => StopRequest._();
  @$core.override
  StopRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static StopRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<StopRequest>(create);
  static StopRequest? _defaultInstance;
}

/// Request Discovered Nodes on BLE
///
/// Message sent from UI to libqaul.
class DiscoveredRequest extends $pb.GeneratedMessage {
  factory DiscoveredRequest() => create();

  DiscoveredRequest._();

  factory DiscoveredRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory DiscoveredRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DiscoveredRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.ble'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DiscoveredRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DiscoveredRequest copyWith(void Function(DiscoveredRequest) updates) =>
      super.copyWith((message) => updates(message as DiscoveredRequest))
          as DiscoveredRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static DiscoveredRequest create() => DiscoveredRequest._();
  @$core.override
  DiscoveredRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static DiscoveredRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DiscoveredRequest>(create);
  static DiscoveredRequest? _defaultInstance;
}

/// All Discovered Nodes
///
/// Answer from libqaul to UI on DiscoveredRequest
class DiscoveredResponse extends $pb.GeneratedMessage {
  factory DiscoveredResponse({
    $core.int? nodesCount,
    $core.int? toConfirmCount,
  }) {
    final result = create();
    if (nodesCount != null) result.nodesCount = nodesCount;
    if (toConfirmCount != null) result.toConfirmCount = toConfirmCount;
    return result;
  }

  DiscoveredResponse._();

  factory DiscoveredResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory DiscoveredResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DiscoveredResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.ble'),
      createEmptyInstance: create)
    ..aI(1, _omitFieldNames ? '' : 'nodesCount', fieldType: $pb.PbFieldType.OU3)
    ..aI(2, _omitFieldNames ? '' : 'toConfirmCount',
        fieldType: $pb.PbFieldType.OU3)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DiscoveredResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DiscoveredResponse copyWith(void Function(DiscoveredResponse) updates) =>
      super.copyWith((message) => updates(message as DiscoveredResponse))
          as DiscoveredResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static DiscoveredResponse create() => DiscoveredResponse._();
  @$core.override
  DiscoveredResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static DiscoveredResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DiscoveredResponse>(create);
  static DiscoveredResponse? _defaultInstance;

  /// number of nodes in discovery table
  @$pb.TagNumber(1)
  $core.int get nodesCount => $_getIZ(0);
  @$pb.TagNumber(1)
  set nodesCount($core.int value) => $_setUnsignedInt32(0, value);
  @$pb.TagNumber(1)
  $core.bool hasNodesCount() => $_has(0);
  @$pb.TagNumber(1)
  void clearNodesCount() => $_clearField(1);

  /// number of nodes in to_confirm table
  @$pb.TagNumber(2)
  $core.int get toConfirmCount => $_getIZ(1);
  @$pb.TagNumber(2)
  set toConfirmCount($core.int value) => $_setUnsignedInt32(1, value);
  @$pb.TagNumber(2)
  $core.bool hasToConfirmCount() => $_has(1);
  @$pb.TagNumber(2)
  void clearToConfirmCount() => $_clearField(2);
}

/// Request Rights
class RightsRequest extends $pb.GeneratedMessage {
  factory RightsRequest() => create();

  RightsRequest._();

  factory RightsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory RightsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RightsRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.ble'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RightsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RightsRequest copyWith(void Function(RightsRequest) updates) =>
      super.copyWith((message) => updates(message as RightsRequest))
          as RightsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RightsRequest create() => RightsRequest._();
  @$core.override
  RightsRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static RightsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RightsRequest>(create);
  static RightsRequest? _defaultInstance;
}

/// Rights Request Results
class RightsResult extends $pb.GeneratedMessage {
  factory RightsResult({
    $core.bool? rightsGranted,
  }) {
    final result = create();
    if (rightsGranted != null) result.rightsGranted = rightsGranted;
    return result;
  }

  RightsResult._();

  factory RightsResult.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory RightsResult.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RightsResult',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.ble'),
      createEmptyInstance: create)
    ..aOB(1, _omitFieldNames ? '' : 'rightsGranted')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RightsResult clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RightsResult copyWith(void Function(RightsResult) updates) =>
      super.copyWith((message) => updates(message as RightsResult))
          as RightsResult;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RightsResult create() => RightsResult._();
  @$core.override
  RightsResult createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static RightsResult getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RightsResult>(create);
  static RightsResult? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get rightsGranted => $_getBF(0);
  @$pb.TagNumber(1)
  set rightsGranted($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasRightsGranted() => $_has(0);
  @$pb.TagNumber(1)
  void clearRightsGranted() => $_clearField(1);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
