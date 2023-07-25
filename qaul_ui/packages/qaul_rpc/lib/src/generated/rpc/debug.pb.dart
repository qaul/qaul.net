///
//  Generated code. Do not modify.
//  source: rpc/debug.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

enum Debug_Message {
  heartbeatRequest, 
  heartbeatResponse, 
  panic, 
  logToFile, 
  storagePathRequest, 
  storagePathResponse, 
  deleteLibqaulLogsRequest, 
  notSet
}

class Debug extends $pb.GeneratedMessage {
  static const $core.Map<$core.int, Debug_Message> _Debug_MessageByTag = {
    1 : Debug_Message.heartbeatRequest,
    2 : Debug_Message.heartbeatResponse,
    3 : Debug_Message.panic,
    4 : Debug_Message.logToFile,
    5 : Debug_Message.storagePathRequest,
    6 : Debug_Message.storagePathResponse,
    7 : Debug_Message.deleteLibqaulLogsRequest,
    0 : Debug_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Debug', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.debug'), createEmptyInstance: create)
    ..oo(0, [1, 2, 3, 4, 5, 6, 7])
    ..aOM<HeartbeatRequest>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'heartbeatRequest', subBuilder: HeartbeatRequest.create)
    ..aOM<HeartbeatResponse>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'heartbeatResponse', subBuilder: HeartbeatResponse.create)
    ..aOM<Panic>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'panic', subBuilder: Panic.create)
    ..aOM<LogToFile>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'logToFile', subBuilder: LogToFile.create)
    ..aOM<StoragePathRequest>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'storagePathRequest', subBuilder: StoragePathRequest.create)
    ..aOM<StoragePathResponse>(6, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'storagePathResponse', subBuilder: StoragePathResponse.create)
    ..aOM<DeleteLibqaulLogsRequest>(7, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'deleteLibqaulLogsRequest', subBuilder: DeleteLibqaulLogsRequest.create)
    ..hasRequiredFields = false
  ;

  Debug._() : super();
  factory Debug({
    HeartbeatRequest? heartbeatRequest,
    HeartbeatResponse? heartbeatResponse,
    Panic? panic,
    LogToFile? logToFile,
    StoragePathRequest? storagePathRequest,
    StoragePathResponse? storagePathResponse,
    DeleteLibqaulLogsRequest? deleteLibqaulLogsRequest,
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
    if (logToFile != null) {
      _result.logToFile = logToFile;
    }
    if (storagePathRequest != null) {
      _result.storagePathRequest = storagePathRequest;
    }
    if (storagePathResponse != null) {
      _result.storagePathResponse = storagePathResponse;
    }
    if (deleteLibqaulLogsRequest != null) {
      _result.deleteLibqaulLogsRequest = deleteLibqaulLogsRequest;
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

  @$pb.TagNumber(4)
  LogToFile get logToFile => $_getN(3);
  @$pb.TagNumber(4)
  set logToFile(LogToFile v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasLogToFile() => $_has(3);
  @$pb.TagNumber(4)
  void clearLogToFile() => clearField(4);
  @$pb.TagNumber(4)
  LogToFile ensureLogToFile() => $_ensure(3);

  @$pb.TagNumber(5)
  StoragePathRequest get storagePathRequest => $_getN(4);
  @$pb.TagNumber(5)
  set storagePathRequest(StoragePathRequest v) { setField(5, v); }
  @$pb.TagNumber(5)
  $core.bool hasStoragePathRequest() => $_has(4);
  @$pb.TagNumber(5)
  void clearStoragePathRequest() => clearField(5);
  @$pb.TagNumber(5)
  StoragePathRequest ensureStoragePathRequest() => $_ensure(4);

  @$pb.TagNumber(6)
  StoragePathResponse get storagePathResponse => $_getN(5);
  @$pb.TagNumber(6)
  set storagePathResponse(StoragePathResponse v) { setField(6, v); }
  @$pb.TagNumber(6)
  $core.bool hasStoragePathResponse() => $_has(5);
  @$pb.TagNumber(6)
  void clearStoragePathResponse() => clearField(6);
  @$pb.TagNumber(6)
  StoragePathResponse ensureStoragePathResponse() => $_ensure(5);

  @$pb.TagNumber(7)
  DeleteLibqaulLogsRequest get deleteLibqaulLogsRequest => $_getN(6);
  @$pb.TagNumber(7)
  set deleteLibqaulLogsRequest(DeleteLibqaulLogsRequest v) { setField(7, v); }
  @$pb.TagNumber(7)
  $core.bool hasDeleteLibqaulLogsRequest() => $_has(6);
  @$pb.TagNumber(7)
  void clearDeleteLibqaulLogsRequest() => clearField(7);
  @$pb.TagNumber(7)
  DeleteLibqaulLogsRequest ensureDeleteLibqaulLogsRequest() => $_ensure(6);
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

class LogToFile extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'LogToFile', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.debug'), createEmptyInstance: create)
    ..aOB(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'enable')
    ..hasRequiredFields = false
  ;

  LogToFile._() : super();
  factory LogToFile({
    $core.bool? enable,
  }) {
    final _result = create();
    if (enable != null) {
      _result.enable = enable;
    }
    return _result;
  }
  factory LogToFile.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory LogToFile.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  LogToFile clone() => LogToFile()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  LogToFile copyWith(void Function(LogToFile) updates) => super.copyWith((message) => updates(message as LogToFile)) as LogToFile; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static LogToFile create() => LogToFile._();
  LogToFile createEmptyInstance() => create();
  static $pb.PbList<LogToFile> createRepeated() => $pb.PbList<LogToFile>();
  @$core.pragma('dart2js:noInline')
  static LogToFile getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<LogToFile>(create);
  static LogToFile? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get enable => $_getBF(0);
  @$pb.TagNumber(1)
  set enable($core.bool v) { $_setBool(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasEnable() => $_has(0);
  @$pb.TagNumber(1)
  void clearEnable() => clearField(1);
}

class StoragePathRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'StoragePathRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.debug'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  StoragePathRequest._() : super();
  factory StoragePathRequest() => create();
  factory StoragePathRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory StoragePathRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  StoragePathRequest clone() => StoragePathRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  StoragePathRequest copyWith(void Function(StoragePathRequest) updates) => super.copyWith((message) => updates(message as StoragePathRequest)) as StoragePathRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static StoragePathRequest create() => StoragePathRequest._();
  StoragePathRequest createEmptyInstance() => create();
  static $pb.PbList<StoragePathRequest> createRepeated() => $pb.PbList<StoragePathRequest>();
  @$core.pragma('dart2js:noInline')
  static StoragePathRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<StoragePathRequest>(create);
  static StoragePathRequest? _defaultInstance;
}

class StoragePathResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'StoragePathResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.debug'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'storagePath')
    ..hasRequiredFields = false
  ;

  StoragePathResponse._() : super();
  factory StoragePathResponse({
    $core.String? storagePath,
  }) {
    final _result = create();
    if (storagePath != null) {
      _result.storagePath = storagePath;
    }
    return _result;
  }
  factory StoragePathResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory StoragePathResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  StoragePathResponse clone() => StoragePathResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  StoragePathResponse copyWith(void Function(StoragePathResponse) updates) => super.copyWith((message) => updates(message as StoragePathResponse)) as StoragePathResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static StoragePathResponse create() => StoragePathResponse._();
  StoragePathResponse createEmptyInstance() => create();
  static $pb.PbList<StoragePathResponse> createRepeated() => $pb.PbList<StoragePathResponse>();
  @$core.pragma('dart2js:noInline')
  static StoragePathResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<StoragePathResponse>(create);
  static StoragePathResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get storagePath => $_getSZ(0);
  @$pb.TagNumber(1)
  set storagePath($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasStoragePath() => $_has(0);
  @$pb.TagNumber(1)
  void clearStoragePath() => clearField(1);
}

class DeleteLibqaulLogsRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'DeleteLibqaulLogsRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.debug'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  DeleteLibqaulLogsRequest._() : super();
  factory DeleteLibqaulLogsRequest() => create();
  factory DeleteLibqaulLogsRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory DeleteLibqaulLogsRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  DeleteLibqaulLogsRequest clone() => DeleteLibqaulLogsRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  DeleteLibqaulLogsRequest copyWith(void Function(DeleteLibqaulLogsRequest) updates) => super.copyWith((message) => updates(message as DeleteLibqaulLogsRequest)) as DeleteLibqaulLogsRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static DeleteLibqaulLogsRequest create() => DeleteLibqaulLogsRequest._();
  DeleteLibqaulLogsRequest createEmptyInstance() => create();
  static $pb.PbList<DeleteLibqaulLogsRequest> createRepeated() => $pb.PbList<DeleteLibqaulLogsRequest>();
  @$core.pragma('dart2js:noInline')
  static DeleteLibqaulLogsRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DeleteLibqaulLogsRequest>(create);
  static DeleteLibqaulLogsRequest? _defaultInstance;
}

