///
//  Generated code. Do not modify.
//  source: rpc/debug.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

enum Debug_Message {
  heartbeatRequest, 
  heartbeatResponse, 
  panic, 
  notSet
}

class Debug extends $pb.GeneratedMessage {
  static const $core.Map<$core.int, Debug_Message> _Debug_MessageByTag = {
    1 : Debug_Message.heartbeatRequest,
    2 : Debug_Message.heartbeatResponse,
    3 : Debug_Message.panic,
    0 : Debug_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Debug', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.debug'), createEmptyInstance: create)
    ..oo(0, [1, 2, 3])
    ..aOM<HeartbeatRequest>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'heartbeatRequest', subBuilder: HeartbeatRequest.create)
    ..aOM<HeartbeatResponse>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'heartbeatResponse', subBuilder: HeartbeatResponse.create)
    ..aOM<Panic>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'panic', subBuilder: Panic.create)
    ..hasRequiredFields = false
  ;

  Debug._() : super();
  factory Debug({
    HeartbeatRequest? heartbeatRequest,
    HeartbeatResponse? heartbeatResponse,
    Panic? panic,
  }) {
    final _result = create();
    if (heartbeatRequest != null) {
      _result.heartbeatRequest = heartbeatRequest;
    }
    if (heartbeatResponse != null) {
      _result.heartbeatResponse = heartbeatResponse;
    }
    if (panic != null) {
      _result.panic = panic;
    }
    return _result;
  }
  factory Debug.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Debug.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Debug clone() => Debug()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Debug copyWith(void Function(Debug) updates) => super.copyWith((message) => updates(message as Debug)) as Debug; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Debug create() => Debug._();
  Debug createEmptyInstance() => create();
  static $pb.PbList<Debug> createRepeated() => $pb.PbList<Debug>();
  @$core.pragma('dart2js:noInline')
  static Debug getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Debug>(create);
  static Debug? _defaultInstance;

  Debug_Message whichMessage() => _Debug_MessageByTag[$_whichOneof(0)]!;
  void clearMessage() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  HeartbeatRequest get heartbeatRequest => $_getN(0);
  @$pb.TagNumber(1)
  set heartbeatRequest(HeartbeatRequest v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasHeartbeatRequest() => $_has(0);
  @$pb.TagNumber(1)
  void clearHeartbeatRequest() => clearField(1);
  @$pb.TagNumber(1)
  HeartbeatRequest ensureHeartbeatRequest() => $_ensure(0);

  @$pb.TagNumber(2)
  HeartbeatResponse get heartbeatResponse => $_getN(1);
  @$pb.TagNumber(2)
  set heartbeatResponse(HeartbeatResponse v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasHeartbeatResponse() => $_has(1);
  @$pb.TagNumber(2)
  void clearHeartbeatResponse() => clearField(2);
  @$pb.TagNumber(2)
  HeartbeatResponse ensureHeartbeatResponse() => $_ensure(1);

  @$pb.TagNumber(3)
  Panic get panic => $_getN(2);
  @$pb.TagNumber(3)
  set panic(Panic v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasPanic() => $_has(2);
  @$pb.TagNumber(3)
  void clearPanic() => clearField(3);
  @$pb.TagNumber(3)
  Panic ensurePanic() => $_ensure(2);
}

class HeartbeatRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'HeartbeatRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.debug'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  HeartbeatRequest._() : super();
  factory HeartbeatRequest() => create();
  factory HeartbeatRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory HeartbeatRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  HeartbeatRequest clone() => HeartbeatRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  HeartbeatRequest copyWith(void Function(HeartbeatRequest) updates) => super.copyWith((message) => updates(message as HeartbeatRequest)) as HeartbeatRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static HeartbeatRequest create() => HeartbeatRequest._();
  HeartbeatRequest createEmptyInstance() => create();
  static $pb.PbList<HeartbeatRequest> createRepeated() => $pb.PbList<HeartbeatRequest>();
  @$core.pragma('dart2js:noInline')
  static HeartbeatRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<HeartbeatRequest>(create);
  static HeartbeatRequest? _defaultInstance;
}

class HeartbeatResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'HeartbeatResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.debug'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  HeartbeatResponse._() : super();
  factory HeartbeatResponse() => create();
  factory HeartbeatResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory HeartbeatResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  HeartbeatResponse clone() => HeartbeatResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  HeartbeatResponse copyWith(void Function(HeartbeatResponse) updates) => super.copyWith((message) => updates(message as HeartbeatResponse)) as HeartbeatResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static HeartbeatResponse create() => HeartbeatResponse._();
  HeartbeatResponse createEmptyInstance() => create();
  static $pb.PbList<HeartbeatResponse> createRepeated() => $pb.PbList<HeartbeatResponse>();
  @$core.pragma('dart2js:noInline')
  static HeartbeatResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<HeartbeatResponse>(create);
  static HeartbeatResponse? _defaultInstance;
}

class Panic extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Panic', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.debug'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  Panic._() : super();
  factory Panic() => create();
  factory Panic.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Panic.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Panic clone() => Panic()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Panic copyWith(void Function(Panic) updates) => super.copyWith((message) => updates(message as Panic)) as Panic; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Panic create() => Panic._();
  Panic createEmptyInstance() => create();
  static $pb.PbList<Panic> createRepeated() => $pb.PbList<Panic>();
  @$core.pragma('dart2js:noInline')
  static Panic getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Panic>(create);
  static Panic? _defaultInstance;
}

