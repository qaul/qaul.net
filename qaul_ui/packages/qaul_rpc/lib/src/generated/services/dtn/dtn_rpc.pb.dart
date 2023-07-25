///
//  Generated code. Do not modify.
//  source: services/dtn/dtn_rpc.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

enum DTN_Message {
  dtnStateRequest, 
  dtnStateResponse, 
  dtnConfigRequest, 
  dtnConfigResponse, 
  dtnAddUserRequest, 
  dtnAddUserResponse, 
  dtnRemoveUserRequest, 
  dtnRemoveUserResponse, 
  dtnSetTotalSizeRequest, 
  dtnSetTotalSizeResponse, 
  notSet
}

class DTN extends $pb.GeneratedMessage {
  static const $core.Map<$core.int, DTN_Message> _DTN_MessageByTag = {
    1 : DTN_Message.dtnStateRequest,
    2 : DTN_Message.dtnStateResponse,
    3 : DTN_Message.dtnConfigRequest,
    4 : DTN_Message.dtnConfigResponse,
    5 : DTN_Message.dtnAddUserRequest,
    6 : DTN_Message.dtnAddUserResponse,
    7 : DTN_Message.dtnRemoveUserRequest,
    8 : DTN_Message.dtnRemoveUserResponse,
    9 : DTN_Message.dtnSetTotalSizeRequest,
    10 : DTN_Message.dtnSetTotalSizeResponse,
    0 : DTN_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'DTN', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.dtn'), createEmptyInstance: create)
    ..oo(0, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
    ..aOM<DtnStateRequest>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'dtnStateRequest', subBuilder: DtnStateRequest.create)
    ..aOM<DtnStateResponse>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'dtnStateResponse', subBuilder: DtnStateResponse.create)
    ..aOM<DtnConfigRequest>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'dtnConfigRequest', subBuilder: DtnConfigRequest.create)
    ..aOM<DtnConfigResponse>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'dtnConfigResponse', subBuilder: DtnConfigResponse.create)
    ..aOM<DtnAddUserRequest>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'dtnAddUserRequest', subBuilder: DtnAddUserRequest.create)
    ..aOM<DtnAddUserResponse>(6, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'dtnAddUserResponse', subBuilder: DtnAddUserResponse.create)
    ..aOM<DtnRemoveUserRequest>(7, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'dtnRemoveUserRequest', subBuilder: DtnRemoveUserRequest.create)
    ..aOM<DtnRemoveUserResponse>(8, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'dtnRemoveUserResponse', subBuilder: DtnRemoveUserResponse.create)
    ..aOM<DtnSetTotalSizeRequest>(9, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'dtnSetTotalSizeRequest', subBuilder: DtnSetTotalSizeRequest.create)
    ..aOM<DtnSetTotalSizeResponse>(10, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'dtnSetTotalSizeResponse', subBuilder: DtnSetTotalSizeResponse.create)
    ..hasRequiredFields = false
  ;

  DTN._() : super();
  factory DTN({
    DtnStateRequest? dtnStateRequest,
    DtnStateResponse? dtnStateResponse,
    DtnConfigRequest? dtnConfigRequest,
    DtnConfigResponse? dtnConfigResponse,
    DtnAddUserRequest? dtnAddUserRequest,
    DtnAddUserResponse? dtnAddUserResponse,
    DtnRemoveUserRequest? dtnRemoveUserRequest,
    DtnRemoveUserResponse? dtnRemoveUserResponse,
    DtnSetTotalSizeRequest? dtnSetTotalSizeRequest,
    DtnSetTotalSizeResponse? dtnSetTotalSizeResponse,
  }) {
    final _result = create();
    if (dtnStateRequest != null) {
      _result.dtnStateRequest = dtnStateRequest;
    }
    if (dtnStateResponse != null) {
      _result.dtnStateResponse = dtnStateResponse;
    }
    if (dtnConfigRequest != null) {
      _result.dtnConfigRequest = dtnConfigRequest;
    }
    if (dtnConfigResponse != null) {
      _result.dtnConfigResponse = dtnConfigResponse;
    }
    if (dtnAddUserRequest != null) {
      _result.dtnAddUserRequest = dtnAddUserRequest;
    }
    if (dtnAddUserResponse != null) {
      _result.dtnAddUserResponse = dtnAddUserResponse;
    }
    if (dtnRemoveUserRequest != null) {
      _result.dtnRemoveUserRequest = dtnRemoveUserRequest;
    }
    if (dtnRemoveUserResponse != null) {
      _result.dtnRemoveUserResponse = dtnRemoveUserResponse;
    }
    if (dtnSetTotalSizeRequest != null) {
      _result.dtnSetTotalSizeRequest = dtnSetTotalSizeRequest;
    }
    if (dtnSetTotalSizeResponse != null) {
      _result.dtnSetTotalSizeResponse = dtnSetTotalSizeResponse;
    }
    return _result;
  }
  factory DTN.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory DTN.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  DTN clone() => DTN()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  DTN copyWith(void Function(DTN) updates) => super.copyWith((message) => updates(message as DTN)) as DTN; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static DTN create() => DTN._();
  DTN createEmptyInstance() => create();
  static $pb.PbList<DTN> createRepeated() => $pb.PbList<DTN>();
  @$core.pragma('dart2js:noInline')
  static DTN getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DTN>(create);
  static DTN? _defaultInstance;

  DTN_Message whichMessage() => _DTN_MessageByTag[$_whichOneof(0)]!;
  void clearMessage() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  DtnStateRequest get dtnStateRequest => $_getN(0);
  @$pb.TagNumber(1)
  set dtnStateRequest(DtnStateRequest v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasDtnStateRequest() => $_has(0);
  @$pb.TagNumber(1)
  void clearDtnStateRequest() => clearField(1);
  @$pb.TagNumber(1)
  DtnStateRequest ensureDtnStateRequest() => $_ensure(0);

  @$pb.TagNumber(2)
  DtnStateResponse get dtnStateResponse => $_getN(1);
  @$pb.TagNumber(2)
  set dtnStateResponse(DtnStateResponse v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasDtnStateResponse() => $_has(1);
  @$pb.TagNumber(2)
  void clearDtnStateResponse() => clearField(2);
  @$pb.TagNumber(2)
  DtnStateResponse ensureDtnStateResponse() => $_ensure(1);

  @$pb.TagNumber(3)
  DtnConfigRequest get dtnConfigRequest => $_getN(2);
  @$pb.TagNumber(3)
  set dtnConfigRequest(DtnConfigRequest v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasDtnConfigRequest() => $_has(2);
  @$pb.TagNumber(3)
  void clearDtnConfigRequest() => clearField(3);
  @$pb.TagNumber(3)
  DtnConfigRequest ensureDtnConfigRequest() => $_ensure(2);

  @$pb.TagNumber(4)
  DtnConfigResponse get dtnConfigResponse => $_getN(3);
  @$pb.TagNumber(4)
  set dtnConfigResponse(DtnConfigResponse v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasDtnConfigResponse() => $_has(3);
  @$pb.TagNumber(4)
  void clearDtnConfigResponse() => clearField(4);
  @$pb.TagNumber(4)
  DtnConfigResponse ensureDtnConfigResponse() => $_ensure(3);

  @$pb.TagNumber(5)
  DtnAddUserRequest get dtnAddUserRequest => $_getN(4);
  @$pb.TagNumber(5)
  set dtnAddUserRequest(DtnAddUserRequest v) { setField(5, v); }
  @$pb.TagNumber(5)
  $core.bool hasDtnAddUserRequest() => $_has(4);
  @$pb.TagNumber(5)
  void clearDtnAddUserRequest() => clearField(5);
  @$pb.TagNumber(5)
  DtnAddUserRequest ensureDtnAddUserRequest() => $_ensure(4);

  @$pb.TagNumber(6)
  DtnAddUserResponse get dtnAddUserResponse => $_getN(5);
  @$pb.TagNumber(6)
  set dtnAddUserResponse(DtnAddUserResponse v) { setField(6, v); }
  @$pb.TagNumber(6)
  $core.bool hasDtnAddUserResponse() => $_has(5);
  @$pb.TagNumber(6)
  void clearDtnAddUserResponse() => clearField(6);
  @$pb.TagNumber(6)
  DtnAddUserResponse ensureDtnAddUserResponse() => $_ensure(5);

  @$pb.TagNumber(7)
  DtnRemoveUserRequest get dtnRemoveUserRequest => $_getN(6);
  @$pb.TagNumber(7)
  set dtnRemoveUserRequest(DtnRemoveUserRequest v) { setField(7, v); }
  @$pb.TagNumber(7)
  $core.bool hasDtnRemoveUserRequest() => $_has(6);
  @$pb.TagNumber(7)
  void clearDtnRemoveUserRequest() => clearField(7);
  @$pb.TagNumber(7)
  DtnRemoveUserRequest ensureDtnRemoveUserRequest() => $_ensure(6);

  @$pb.TagNumber(8)
  DtnRemoveUserResponse get dtnRemoveUserResponse => $_getN(7);
  @$pb.TagNumber(8)
  set dtnRemoveUserResponse(DtnRemoveUserResponse v) { setField(8, v); }
  @$pb.TagNumber(8)
  $core.bool hasDtnRemoveUserResponse() => $_has(7);
  @$pb.TagNumber(8)
  void clearDtnRemoveUserResponse() => clearField(8);
  @$pb.TagNumber(8)
  DtnRemoveUserResponse ensureDtnRemoveUserResponse() => $_ensure(7);

  @$pb.TagNumber(9)
  DtnSetTotalSizeRequest get dtnSetTotalSizeRequest => $_getN(8);
  @$pb.TagNumber(9)
  set dtnSetTotalSizeRequest(DtnSetTotalSizeRequest v) { setField(9, v); }
  @$pb.TagNumber(9)
  $core.bool hasDtnSetTotalSizeRequest() => $_has(8);
  @$pb.TagNumber(9)
  void clearDtnSetTotalSizeRequest() => clearField(9);
  @$pb.TagNumber(9)
  DtnSetTotalSizeRequest ensureDtnSetTotalSizeRequest() => $_ensure(8);

  @$pb.TagNumber(10)
  DtnSetTotalSizeResponse get dtnSetTotalSizeResponse => $_getN(9);
  @$pb.TagNumber(10)
  set dtnSetTotalSizeResponse(DtnSetTotalSizeResponse v) { setField(10, v); }
  @$pb.TagNumber(10)
  $core.bool hasDtnSetTotalSizeResponse() => $_has(9);
  @$pb.TagNumber(10)
  void clearDtnSetTotalSizeResponse() => clearField(10);
  @$pb.TagNumber(10)
  DtnSetTotalSizeResponse ensureDtnSetTotalSizeResponse() => $_ensure(9);
}

class DtnStateRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'DtnStateRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.dtn'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  DtnStateRequest._() : super();
  factory DtnStateRequest() => create();
  factory DtnStateRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory DtnStateRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  DtnStateRequest clone() => DtnStateRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  DtnStateRequest copyWith(void Function(DtnStateRequest) updates) => super.copyWith((message) => updates(message as DtnStateRequest)) as DtnStateRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static DtnStateRequest create() => DtnStateRequest._();
  DtnStateRequest createEmptyInstance() => create();
  static $pb.PbList<DtnStateRequest> createRepeated() => $pb.PbList<DtnStateRequest>();
  @$core.pragma('dart2js:noInline')
  static DtnStateRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DtnStateRequest>(create);
  static DtnStateRequest? _defaultInstance;
}

class DtnStateResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'DtnStateResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.dtn'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'usedSize', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.int>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'dtnMessageCount', $pb.PbFieldType.OU3)
    ..a<$core.int>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'unconfirmedCount', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  DtnStateResponse._() : super();
  factory DtnStateResponse({
    $fixnum.Int64? usedSize,
    $core.int? dtnMessageCount,
    $core.int? unconfirmedCount,
  }) {
    final _result = create();
    if (usedSize != null) {
      _result.usedSize = usedSize;
    }
    if (dtnMessageCount != null) {
      _result.dtnMessageCount = dtnMessageCount;
    }
    if (unconfirmedCount != null) {
      _result.unconfirmedCount = unconfirmedCount;
    }
    return _result;
  }
  factory DtnStateResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory DtnStateResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  DtnStateResponse clone() => DtnStateResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  DtnStateResponse copyWith(void Function(DtnStateResponse) updates) => super.copyWith((message) => updates(message as DtnStateResponse)) as DtnStateResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static DtnStateResponse create() => DtnStateResponse._();
  DtnStateResponse createEmptyInstance() => create();
  static $pb.PbList<DtnStateResponse> createRepeated() => $pb.PbList<DtnStateResponse>();
  @$core.pragma('dart2js:noInline')
  static DtnStateResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DtnStateResponse>(create);
  static DtnStateResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get usedSize => $_getI64(0);
  @$pb.TagNumber(1)
  set usedSize($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasUsedSize() => $_has(0);
  @$pb.TagNumber(1)
  void clearUsedSize() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get dtnMessageCount => $_getIZ(1);
  @$pb.TagNumber(2)
  set dtnMessageCount($core.int v) { $_setUnsignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasDtnMessageCount() => $_has(1);
  @$pb.TagNumber(2)
  void clearDtnMessageCount() => clearField(2);

  @$pb.TagNumber(3)
  $core.int get unconfirmedCount => $_getIZ(2);
  @$pb.TagNumber(3)
  set unconfirmedCount($core.int v) { $_setUnsignedInt32(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasUnconfirmedCount() => $_has(2);
  @$pb.TagNumber(3)
  void clearUnconfirmedCount() => clearField(3);
}

class DtnConfigRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'DtnConfigRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.dtn'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  DtnConfigRequest._() : super();
  factory DtnConfigRequest() => create();
  factory DtnConfigRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory DtnConfigRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  DtnConfigRequest clone() => DtnConfigRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  DtnConfigRequest copyWith(void Function(DtnConfigRequest) updates) => super.copyWith((message) => updates(message as DtnConfigRequest)) as DtnConfigRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static DtnConfigRequest create() => DtnConfigRequest._();
  DtnConfigRequest createEmptyInstance() => create();
  static $pb.PbList<DtnConfigRequest> createRepeated() => $pb.PbList<DtnConfigRequest>();
  @$core.pragma('dart2js:noInline')
  static DtnConfigRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DtnConfigRequest>(create);
  static DtnConfigRequest? _defaultInstance;
}

class DtnConfigResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'DtnConfigResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.dtn'), createEmptyInstance: create)
    ..a<$core.int>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'totalSize', $pb.PbFieldType.OU3)
    ..p<$core.List<$core.int>>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'users', $pb.PbFieldType.PY)
    ..hasRequiredFields = false
  ;

  DtnConfigResponse._() : super();
  factory DtnConfigResponse({
    $core.int? totalSize,
    $core.Iterable<$core.List<$core.int>>? users,
  }) {
    final _result = create();
    if (totalSize != null) {
      _result.totalSize = totalSize;
    }
    if (users != null) {
      _result.users.addAll(users);
    }
    return _result;
  }
  factory DtnConfigResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory DtnConfigResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  DtnConfigResponse clone() => DtnConfigResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  DtnConfigResponse copyWith(void Function(DtnConfigResponse) updates) => super.copyWith((message) => updates(message as DtnConfigResponse)) as DtnConfigResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static DtnConfigResponse create() => DtnConfigResponse._();
  DtnConfigResponse createEmptyInstance() => create();
  static $pb.PbList<DtnConfigResponse> createRepeated() => $pb.PbList<DtnConfigResponse>();
  @$core.pragma('dart2js:noInline')
  static DtnConfigResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DtnConfigResponse>(create);
  static DtnConfigResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get totalSize => $_getIZ(0);
  @$pb.TagNumber(1)
  set totalSize($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasTotalSize() => $_has(0);
  @$pb.TagNumber(1)
  void clearTotalSize() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.List<$core.int>> get users => $_getList(1);
}

class DtnAddUserRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'DtnAddUserRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.dtn'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'userId', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  DtnAddUserRequest._() : super();
  factory DtnAddUserRequest({
    $core.List<$core.int>? userId,
  }) {
    final _result = create();
    if (userId != null) {
      _result.userId = userId;
    }
    return _result;
  }
  factory DtnAddUserRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory DtnAddUserRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  DtnAddUserRequest clone() => DtnAddUserRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  DtnAddUserRequest copyWith(void Function(DtnAddUserRequest) updates) => super.copyWith((message) => updates(message as DtnAddUserRequest)) as DtnAddUserRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static DtnAddUserRequest create() => DtnAddUserRequest._();
  DtnAddUserRequest createEmptyInstance() => create();
  static $pb.PbList<DtnAddUserRequest> createRepeated() => $pb.PbList<DtnAddUserRequest>();
  @$core.pragma('dart2js:noInline')
  static DtnAddUserRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DtnAddUserRequest>(create);
  static DtnAddUserRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get userId => $_getN(0);
  @$pb.TagNumber(1)
  set userId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasUserId() => $_has(0);
  @$pb.TagNumber(1)
  void clearUserId() => clearField(1);
}

class DtnAddUserResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'DtnAddUserResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.dtn'), createEmptyInstance: create)
    ..aOB(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'status')
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'message')
    ..hasRequiredFields = false
  ;

  DtnAddUserResponse._() : super();
  factory DtnAddUserResponse({
    $core.bool? status,
    $core.String? message,
  }) {
    final _result = create();
    if (status != null) {
      _result.status = status;
    }
    if (message != null) {
      _result.message = message;
    }
    return _result;
  }
  factory DtnAddUserResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory DtnAddUserResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  DtnAddUserResponse clone() => DtnAddUserResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  DtnAddUserResponse copyWith(void Function(DtnAddUserResponse) updates) => super.copyWith((message) => updates(message as DtnAddUserResponse)) as DtnAddUserResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static DtnAddUserResponse create() => DtnAddUserResponse._();
  DtnAddUserResponse createEmptyInstance() => create();
  static $pb.PbList<DtnAddUserResponse> createRepeated() => $pb.PbList<DtnAddUserResponse>();
  @$core.pragma('dart2js:noInline')
  static DtnAddUserResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DtnAddUserResponse>(create);
  static DtnAddUserResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get status => $_getBF(0);
  @$pb.TagNumber(1)
  set status($core.bool v) { $_setBool(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasStatus() => $_has(0);
  @$pb.TagNumber(1)
  void clearStatus() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get message => $_getSZ(1);
  @$pb.TagNumber(2)
  set message($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasMessage() => $_has(1);
  @$pb.TagNumber(2)
  void clearMessage() => clearField(2);
}

class DtnRemoveUserRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'DtnRemoveUserRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.dtn'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'userId', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  DtnRemoveUserRequest._() : super();
  factory DtnRemoveUserRequest({
    $core.List<$core.int>? userId,
  }) {
    final _result = create();
    if (userId != null) {
      _result.userId = userId;
    }
    return _result;
  }
  factory DtnRemoveUserRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory DtnRemoveUserRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  DtnRemoveUserRequest clone() => DtnRemoveUserRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  DtnRemoveUserRequest copyWith(void Function(DtnRemoveUserRequest) updates) => super.copyWith((message) => updates(message as DtnRemoveUserRequest)) as DtnRemoveUserRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static DtnRemoveUserRequest create() => DtnRemoveUserRequest._();
  DtnRemoveUserRequest createEmptyInstance() => create();
  static $pb.PbList<DtnRemoveUserRequest> createRepeated() => $pb.PbList<DtnRemoveUserRequest>();
  @$core.pragma('dart2js:noInline')
  static DtnRemoveUserRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DtnRemoveUserRequest>(create);
  static DtnRemoveUserRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get userId => $_getN(0);
  @$pb.TagNumber(1)
  set userId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasUserId() => $_has(0);
  @$pb.TagNumber(1)
  void clearUserId() => clearField(1);
}

class DtnRemoveUserResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'DtnRemoveUserResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.dtn'), createEmptyInstance: create)
    ..aOB(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'status')
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'message')
    ..hasRequiredFields = false
  ;

  DtnRemoveUserResponse._() : super();
  factory DtnRemoveUserResponse({
    $core.bool? status,
    $core.String? message,
  }) {
    final _result = create();
    if (status != null) {
      _result.status = status;
    }
    if (message != null) {
      _result.message = message;
    }
    return _result;
  }
  factory DtnRemoveUserResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory DtnRemoveUserResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  DtnRemoveUserResponse clone() => DtnRemoveUserResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  DtnRemoveUserResponse copyWith(void Function(DtnRemoveUserResponse) updates) => super.copyWith((message) => updates(message as DtnRemoveUserResponse)) as DtnRemoveUserResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static DtnRemoveUserResponse create() => DtnRemoveUserResponse._();
  DtnRemoveUserResponse createEmptyInstance() => create();
  static $pb.PbList<DtnRemoveUserResponse> createRepeated() => $pb.PbList<DtnRemoveUserResponse>();
  @$core.pragma('dart2js:noInline')
  static DtnRemoveUserResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DtnRemoveUserResponse>(create);
  static DtnRemoveUserResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get status => $_getBF(0);
  @$pb.TagNumber(1)
  set status($core.bool v) { $_setBool(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasStatus() => $_has(0);
  @$pb.TagNumber(1)
  void clearStatus() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get message => $_getSZ(1);
  @$pb.TagNumber(2)
  set message($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasMessage() => $_has(1);
  @$pb.TagNumber(2)
  void clearMessage() => clearField(2);
}

class DtnSetTotalSizeRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'DtnSetTotalSizeRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.dtn'), createEmptyInstance: create)
    ..a<$core.int>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'totalSize', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  DtnSetTotalSizeRequest._() : super();
  factory DtnSetTotalSizeRequest({
    $core.int? totalSize,
  }) {
    final _result = create();
    if (totalSize != null) {
      _result.totalSize = totalSize;
    }
    return _result;
  }
  factory DtnSetTotalSizeRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory DtnSetTotalSizeRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  DtnSetTotalSizeRequest clone() => DtnSetTotalSizeRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  DtnSetTotalSizeRequest copyWith(void Function(DtnSetTotalSizeRequest) updates) => super.copyWith((message) => updates(message as DtnSetTotalSizeRequest)) as DtnSetTotalSizeRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static DtnSetTotalSizeRequest create() => DtnSetTotalSizeRequest._();
  DtnSetTotalSizeRequest createEmptyInstance() => create();
  static $pb.PbList<DtnSetTotalSizeRequest> createRepeated() => $pb.PbList<DtnSetTotalSizeRequest>();
  @$core.pragma('dart2js:noInline')
  static DtnSetTotalSizeRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DtnSetTotalSizeRequest>(create);
  static DtnSetTotalSizeRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get totalSize => $_getIZ(0);
  @$pb.TagNumber(1)
  set totalSize($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasTotalSize() => $_has(0);
  @$pb.TagNumber(1)
  void clearTotalSize() => clearField(1);
}

class DtnSetTotalSizeResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'DtnSetTotalSizeResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.dtn'), createEmptyInstance: create)
    ..aOB(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'status')
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'message')
    ..hasRequiredFields = false
  ;

  DtnSetTotalSizeResponse._() : super();
  factory DtnSetTotalSizeResponse({
    $core.bool? status,
    $core.String? message,
  }) {
    final _result = create();
    if (status != null) {
      _result.status = status;
    }
    if (message != null) {
      _result.message = message;
    }
    return _result;
  }
  factory DtnSetTotalSizeResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory DtnSetTotalSizeResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  DtnSetTotalSizeResponse clone() => DtnSetTotalSizeResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  DtnSetTotalSizeResponse copyWith(void Function(DtnSetTotalSizeResponse) updates) => super.copyWith((message) => updates(message as DtnSetTotalSizeResponse)) as DtnSetTotalSizeResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static DtnSetTotalSizeResponse create() => DtnSetTotalSizeResponse._();
  DtnSetTotalSizeResponse createEmptyInstance() => create();
  static $pb.PbList<DtnSetTotalSizeResponse> createRepeated() => $pb.PbList<DtnSetTotalSizeResponse>();
  @$core.pragma('dart2js:noInline')
  static DtnSetTotalSizeResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DtnSetTotalSizeResponse>(create);
  static DtnSetTotalSizeResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get status => $_getBF(0);
  @$pb.TagNumber(1)
  set status($core.bool v) { $_setBool(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasStatus() => $_has(0);
  @$pb.TagNumber(1)
  void clearStatus() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get message => $_getSZ(1);
  @$pb.TagNumber(2)
  set message($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasMessage() => $_has(1);
  @$pb.TagNumber(2)
  void clearMessage() => clearField(2);
}

