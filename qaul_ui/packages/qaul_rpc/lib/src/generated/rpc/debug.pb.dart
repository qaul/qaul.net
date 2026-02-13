// This is a generated file - do not edit.
//
// Generated from rpc/debug.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

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

/// Libqaul RPC Debug Messages
class Debug extends $pb.GeneratedMessage {
  factory Debug({
    HeartbeatRequest? heartbeatRequest,
    HeartbeatResponse? heartbeatResponse,
    Panic? panic,
    LogToFile? logToFile,
    StoragePathRequest? storagePathRequest,
    StoragePathResponse? storagePathResponse,
    DeleteLibqaulLogsRequest? deleteLibqaulLogsRequest,
  }) {
    final result = create();
    if (heartbeatRequest != null) result.heartbeatRequest = heartbeatRequest;
    if (heartbeatResponse != null) result.heartbeatResponse = heartbeatResponse;
    if (panic != null) result.panic = panic;
    if (logToFile != null) result.logToFile = logToFile;
    if (storagePathRequest != null)
      result.storagePathRequest = storagePathRequest;
    if (storagePathResponse != null)
      result.storagePathResponse = storagePathResponse;
    if (deleteLibqaulLogsRequest != null)
      result.deleteLibqaulLogsRequest = deleteLibqaulLogsRequest;
    return result;
  }

  Debug._();

  factory Debug.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory Debug.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, Debug_Message> _Debug_MessageByTag = {
    1: Debug_Message.heartbeatRequest,
    2: Debug_Message.heartbeatResponse,
    3: Debug_Message.panic,
    4: Debug_Message.logToFile,
    5: Debug_Message.storagePathRequest,
    6: Debug_Message.storagePathResponse,
    7: Debug_Message.deleteLibqaulLogsRequest,
    0: Debug_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Debug',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.debug'),
      createEmptyInstance: create)
    ..oo(0, [1, 2, 3, 4, 5, 6, 7])
    ..aOM<HeartbeatRequest>(1, _omitFieldNames ? '' : 'heartbeatRequest',
        subBuilder: HeartbeatRequest.create)
    ..aOM<HeartbeatResponse>(2, _omitFieldNames ? '' : 'heartbeatResponse',
        subBuilder: HeartbeatResponse.create)
    ..aOM<Panic>(3, _omitFieldNames ? '' : 'panic', subBuilder: Panic.create)
    ..aOM<LogToFile>(4, _omitFieldNames ? '' : 'logToFile',
        subBuilder: LogToFile.create)
    ..aOM<StoragePathRequest>(5, _omitFieldNames ? '' : 'storagePathRequest',
        subBuilder: StoragePathRequest.create)
    ..aOM<StoragePathResponse>(6, _omitFieldNames ? '' : 'storagePathResponse',
        subBuilder: StoragePathResponse.create)
    ..aOM<DeleteLibqaulLogsRequest>(
        7, _omitFieldNames ? '' : 'deleteLibqaulLogsRequest',
        subBuilder: DeleteLibqaulLogsRequest.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Debug clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Debug copyWith(void Function(Debug) updates) =>
      super.copyWith((message) => updates(message as Debug)) as Debug;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Debug create() => Debug._();
  @$core.override
  Debug createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static Debug getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Debug>(create);
  static Debug? _defaultInstance;

  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  @$pb.TagNumber(6)
  @$pb.TagNumber(7)
  Debug_Message whichMessage() => _Debug_MessageByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  @$pb.TagNumber(6)
  @$pb.TagNumber(7)
  void clearMessage() => $_clearField($_whichOneof(0));

  /// request a heartbeat
  @$pb.TagNumber(1)
  HeartbeatRequest get heartbeatRequest => $_getN(0);
  @$pb.TagNumber(1)
  set heartbeatRequest(HeartbeatRequest value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasHeartbeatRequest() => $_has(0);
  @$pb.TagNumber(1)
  void clearHeartbeatRequest() => $_clearField(1);
  @$pb.TagNumber(1)
  HeartbeatRequest ensureHeartbeatRequest() => $_ensure(0);

  /// response to the heartbeat request
  @$pb.TagNumber(2)
  HeartbeatResponse get heartbeatResponse => $_getN(1);
  @$pb.TagNumber(2)
  set heartbeatResponse(HeartbeatResponse value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasHeartbeatResponse() => $_has(1);
  @$pb.TagNumber(2)
  void clearHeartbeatResponse() => $_clearField(2);
  @$pb.TagNumber(2)
  HeartbeatResponse ensureHeartbeatResponse() => $_ensure(1);

  /// libqaul panics immediately
  @$pb.TagNumber(3)
  Panic get panic => $_getN(2);
  @$pb.TagNumber(3)
  set panic(Panic value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasPanic() => $_has(2);
  @$pb.TagNumber(3)
  void clearPanic() => $_clearField(3);
  @$pb.TagNumber(3)
  Panic ensurePanic() => $_ensure(2);

  /// enable/disable logging to file
  @$pb.TagNumber(4)
  LogToFile get logToFile => $_getN(3);
  @$pb.TagNumber(4)
  set logToFile(LogToFile value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasLogToFile() => $_has(3);
  @$pb.TagNumber(4)
  void clearLogToFile() => $_clearField(4);
  @$pb.TagNumber(4)
  LogToFile ensureLogToFile() => $_ensure(3);

  /// Storage Path Request
  @$pb.TagNumber(5)
  StoragePathRequest get storagePathRequest => $_getN(4);
  @$pb.TagNumber(5)
  set storagePathRequest(StoragePathRequest value) => $_setField(5, value);
  @$pb.TagNumber(5)
  $core.bool hasStoragePathRequest() => $_has(4);
  @$pb.TagNumber(5)
  void clearStoragePathRequest() => $_clearField(5);
  @$pb.TagNumber(5)
  StoragePathRequest ensureStoragePathRequest() => $_ensure(4);

  /// Storage Path Response
  @$pb.TagNumber(6)
  StoragePathResponse get storagePathResponse => $_getN(5);
  @$pb.TagNumber(6)
  set storagePathResponse(StoragePathResponse value) => $_setField(6, value);
  @$pb.TagNumber(6)
  $core.bool hasStoragePathResponse() => $_has(5);
  @$pb.TagNumber(6)
  void clearStoragePathResponse() => $_clearField(6);
  @$pb.TagNumber(6)
  StoragePathResponse ensureStoragePathResponse() => $_ensure(5);

  /// Request for library to delete logs
  @$pb.TagNumber(7)
  DeleteLibqaulLogsRequest get deleteLibqaulLogsRequest => $_getN(6);
  @$pb.TagNumber(7)
  set deleteLibqaulLogsRequest(DeleteLibqaulLogsRequest value) =>
      $_setField(7, value);
  @$pb.TagNumber(7)
  $core.bool hasDeleteLibqaulLogsRequest() => $_has(6);
  @$pb.TagNumber(7)
  void clearDeleteLibqaulLogsRequest() => $_clearField(7);
  @$pb.TagNumber(7)
  DeleteLibqaulLogsRequest ensureDeleteLibqaulLogsRequest() => $_ensure(6);
}

/// Request a Heartbeat from Libqaul
///
/// The UI requests regular heartbeats from libqaul,
/// to check if libqaul is still alive
class HeartbeatRequest extends $pb.GeneratedMessage {
  factory HeartbeatRequest() => create();

  HeartbeatRequest._();

  factory HeartbeatRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory HeartbeatRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'HeartbeatRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.debug'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  HeartbeatRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  HeartbeatRequest copyWith(void Function(HeartbeatRequest) updates) =>
      super.copyWith((message) => updates(message as HeartbeatRequest))
          as HeartbeatRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static HeartbeatRequest create() => HeartbeatRequest._();
  @$core.override
  HeartbeatRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static HeartbeatRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<HeartbeatRequest>(create);
  static HeartbeatRequest? _defaultInstance;
}

/// Heartbeat Reply
///
/// Libqaul answers to the heartbeat request
/// with the heartbeat reply answer
class HeartbeatResponse extends $pb.GeneratedMessage {
  factory HeartbeatResponse() => create();

  HeartbeatResponse._();

  factory HeartbeatResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory HeartbeatResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'HeartbeatResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.debug'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  HeartbeatResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  HeartbeatResponse copyWith(void Function(HeartbeatResponse) updates) =>
      super.copyWith((message) => updates(message as HeartbeatResponse))
          as HeartbeatResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static HeartbeatResponse create() => HeartbeatResponse._();
  @$core.override
  HeartbeatResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static HeartbeatResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<HeartbeatResponse>(create);
  static HeartbeatResponse? _defaultInstance;
}

/// Panic
///
/// If libqaul receives this panic message, it
/// throws an error and panics immediatly.
///
/// This message is for debugging only.
class Panic extends $pb.GeneratedMessage {
  factory Panic() => create();

  Panic._();

  factory Panic.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory Panic.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Panic',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.debug'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Panic clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Panic copyWith(void Function(Panic) updates) =>
      super.copyWith((message) => updates(message as Panic)) as Panic;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Panic create() => Panic._();
  @$core.override
  Panic createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static Panic getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Panic>(create);
  static Panic? _defaultInstance;
}

/// LogToFile
///
/// If libqaul receives this enable message, it
/// start or stop to log error contents into error_xxx.log file.
class LogToFile extends $pb.GeneratedMessage {
  factory LogToFile({
    $core.bool? enable,
  }) {
    final result = create();
    if (enable != null) result.enable = enable;
    return result;
  }

  LogToFile._();

  factory LogToFile.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory LogToFile.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'LogToFile',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.debug'),
      createEmptyInstance: create)
    ..aOB(1, _omitFieldNames ? '' : 'enable')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  LogToFile clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  LogToFile copyWith(void Function(LogToFile) updates) =>
      super.copyWith((message) => updates(message as LogToFile)) as LogToFile;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static LogToFile create() => LogToFile._();
  @$core.override
  LogToFile createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static LogToFile getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<LogToFile>(create);
  static LogToFile? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get enable => $_getBF(0);
  @$pb.TagNumber(1)
  set enable($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasEnable() => $_has(0);
  @$pb.TagNumber(1)
  void clearEnable() => $_clearField(1);
}

/// StoragePathRequest
///
/// Return storage path
class StoragePathRequest extends $pb.GeneratedMessage {
  factory StoragePathRequest() => create();

  StoragePathRequest._();

  factory StoragePathRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory StoragePathRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'StoragePathRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.debug'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  StoragePathRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  StoragePathRequest copyWith(void Function(StoragePathRequest) updates) =>
      super.copyWith((message) => updates(message as StoragePathRequest))
          as StoragePathRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static StoragePathRequest create() => StoragePathRequest._();
  @$core.override
  StoragePathRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static StoragePathRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<StoragePathRequest>(create);
  static StoragePathRequest? _defaultInstance;
}

/// StoragePathResponse
///
/// Contains Storage Path
class StoragePathResponse extends $pb.GeneratedMessage {
  factory StoragePathResponse({
    $core.String? storagePath,
  }) {
    final result = create();
    if (storagePath != null) result.storagePath = storagePath;
    return result;
  }

  StoragePathResponse._();

  factory StoragePathResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory StoragePathResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'StoragePathResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.debug'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'storagePath')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  StoragePathResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  StoragePathResponse copyWith(void Function(StoragePathResponse) updates) =>
      super.copyWith((message) => updates(message as StoragePathResponse))
          as StoragePathResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static StoragePathResponse create() => StoragePathResponse._();
  @$core.override
  StoragePathResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static StoragePathResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<StoragePathResponse>(create);
  static StoragePathResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get storagePath => $_getSZ(0);
  @$pb.TagNumber(1)
  set storagePath($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasStoragePath() => $_has(0);
  @$pb.TagNumber(1)
  void clearStoragePath() => $_clearField(1);
}

/// DeleteLibqaulLogsRequest
///
/// Requests for the log folder to be wiped clean
class DeleteLibqaulLogsRequest extends $pb.GeneratedMessage {
  factory DeleteLibqaulLogsRequest() => create();

  DeleteLibqaulLogsRequest._();

  factory DeleteLibqaulLogsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory DeleteLibqaulLogsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DeleteLibqaulLogsRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.debug'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteLibqaulLogsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteLibqaulLogsRequest copyWith(
          void Function(DeleteLibqaulLogsRequest) updates) =>
      super.copyWith((message) => updates(message as DeleteLibqaulLogsRequest))
          as DeleteLibqaulLogsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static DeleteLibqaulLogsRequest create() => DeleteLibqaulLogsRequest._();
  @$core.override
  DeleteLibqaulLogsRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static DeleteLibqaulLogsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DeleteLibqaulLogsRequest>(create);
  static DeleteLibqaulLogsRequest? _defaultInstance;
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
