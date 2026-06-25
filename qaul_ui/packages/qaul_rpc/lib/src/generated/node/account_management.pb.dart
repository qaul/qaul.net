// This is a generated file - do not edit.
//
// Generated from node/account_management.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:async' as $async;
import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import '../common/common.pb.dart' as $0;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

enum AccountManagement_Message {
  exportAccountRequest,
  exportAccountResponse,
  deleteAccountRequest,
  restoreAccountRequest,
  restoreAccountResponse,
  ack,
  error,
  notSet
}

/// Account management RPC message container.
class AccountManagement extends $pb.GeneratedMessage {
  factory AccountManagement({
    ExportAccountRequest? exportAccountRequest,
    ExportAccountResponse? exportAccountResponse,
    DeleteAccountRequest? deleteAccountRequest,
    RestoreAccountRequest? restoreAccountRequest,
    RestoreAccountResponse? restoreAccountResponse,
    $0.Ack? ack,
    $0.RpcError? error,
  }) {
    final result = create();
    if (exportAccountRequest != null)
      result.exportAccountRequest = exportAccountRequest;
    if (exportAccountResponse != null)
      result.exportAccountResponse = exportAccountResponse;
    if (deleteAccountRequest != null)
      result.deleteAccountRequest = deleteAccountRequest;
    if (restoreAccountRequest != null)
      result.restoreAccountRequest = restoreAccountRequest;
    if (restoreAccountResponse != null)
      result.restoreAccountResponse = restoreAccountResponse;
    if (ack != null) result.ack = ack;
    if (error != null) result.error = error;
    return result;
  }

  AccountManagement._();

  factory AccountManagement.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory AccountManagement.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, AccountManagement_Message>
      _AccountManagement_MessageByTag = {
    1: AccountManagement_Message.exportAccountRequest,
    2: AccountManagement_Message.exportAccountResponse,
    3: AccountManagement_Message.deleteAccountRequest,
    4: AccountManagement_Message.restoreAccountRequest,
    5: AccountManagement_Message.restoreAccountResponse,
    6: AccountManagement_Message.ack,
    7: AccountManagement_Message.error,
    0: AccountManagement_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AccountManagement',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'qaul.rpc.account_management'),
      createEmptyInstance: create)
    ..oo(0, [1, 2, 3, 4, 5, 6, 7])
    ..aOM<ExportAccountRequest>(
        1, _omitFieldNames ? '' : 'exportAccountRequest',
        subBuilder: ExportAccountRequest.create)
    ..aOM<ExportAccountResponse>(
        2, _omitFieldNames ? '' : 'exportAccountResponse',
        subBuilder: ExportAccountResponse.create)
    ..aOM<DeleteAccountRequest>(
        3, _omitFieldNames ? '' : 'deleteAccountRequest',
        subBuilder: DeleteAccountRequest.create)
    ..aOM<RestoreAccountRequest>(
        4, _omitFieldNames ? '' : 'restoreAccountRequest',
        subBuilder: RestoreAccountRequest.create)
    ..aOM<RestoreAccountResponse>(
        5, _omitFieldNames ? '' : 'restoreAccountResponse',
        subBuilder: RestoreAccountResponse.create)
    ..aOM<$0.Ack>(6, _omitFieldNames ? '' : 'ack', subBuilder: $0.Ack.create)
    ..aOM<$0.RpcError>(7, _omitFieldNames ? '' : 'error',
        subBuilder: $0.RpcError.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AccountManagement clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AccountManagement copyWith(void Function(AccountManagement) updates) =>
      super.copyWith((message) => updates(message as AccountManagement))
          as AccountManagement;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static AccountManagement create() => AccountManagement._();
  @$core.override
  AccountManagement createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static AccountManagement getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AccountManagement>(create);
  static AccountManagement? _defaultInstance;

  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  @$pb.TagNumber(6)
  @$pb.TagNumber(7)
  AccountManagement_Message whichMessage() =>
      _AccountManagement_MessageByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  @$pb.TagNumber(6)
  @$pb.TagNumber(7)
  void clearMessage() => $_clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  ExportAccountRequest get exportAccountRequest => $_getN(0);
  @$pb.TagNumber(1)
  set exportAccountRequest(ExportAccountRequest value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasExportAccountRequest() => $_has(0);
  @$pb.TagNumber(1)
  void clearExportAccountRequest() => $_clearField(1);
  @$pb.TagNumber(1)
  ExportAccountRequest ensureExportAccountRequest() => $_ensure(0);

  @$pb.TagNumber(2)
  ExportAccountResponse get exportAccountResponse => $_getN(1);
  @$pb.TagNumber(2)
  set exportAccountResponse(ExportAccountResponse value) =>
      $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasExportAccountResponse() => $_has(1);
  @$pb.TagNumber(2)
  void clearExportAccountResponse() => $_clearField(2);
  @$pb.TagNumber(2)
  ExportAccountResponse ensureExportAccountResponse() => $_ensure(1);

  @$pb.TagNumber(3)
  DeleteAccountRequest get deleteAccountRequest => $_getN(2);
  @$pb.TagNumber(3)
  set deleteAccountRequest(DeleteAccountRequest value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasDeleteAccountRequest() => $_has(2);
  @$pb.TagNumber(3)
  void clearDeleteAccountRequest() => $_clearField(3);
  @$pb.TagNumber(3)
  DeleteAccountRequest ensureDeleteAccountRequest() => $_ensure(2);

  @$pb.TagNumber(4)
  RestoreAccountRequest get restoreAccountRequest => $_getN(3);
  @$pb.TagNumber(4)
  set restoreAccountRequest(RestoreAccountRequest value) =>
      $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasRestoreAccountRequest() => $_has(3);
  @$pb.TagNumber(4)
  void clearRestoreAccountRequest() => $_clearField(4);
  @$pb.TagNumber(4)
  RestoreAccountRequest ensureRestoreAccountRequest() => $_ensure(3);

  @$pb.TagNumber(5)
  RestoreAccountResponse get restoreAccountResponse => $_getN(4);
  @$pb.TagNumber(5)
  set restoreAccountResponse(RestoreAccountResponse value) =>
      $_setField(5, value);
  @$pb.TagNumber(5)
  $core.bool hasRestoreAccountResponse() => $_has(4);
  @$pb.TagNumber(5)
  void clearRestoreAccountResponse() => $_clearField(5);
  @$pb.TagNumber(5)
  RestoreAccountResponse ensureRestoreAccountResponse() => $_ensure(4);

  /// acknowledgement response (delete success)
  @$pb.TagNumber(6)
  $0.Ack get ack => $_getN(5);
  @$pb.TagNumber(6)
  set ack($0.Ack value) => $_setField(6, value);
  @$pb.TagNumber(6)
  $core.bool hasAck() => $_has(5);
  @$pb.TagNumber(6)
  void clearAck() => $_clearField(6);
  @$pb.TagNumber(6)
  $0.Ack ensureAck() => $_ensure(5);

  /// RPC error response
  @$pb.TagNumber(7)
  $0.RpcError get error => $_getN(6);
  @$pb.TagNumber(7)
  set error($0.RpcError value) => $_setField(7, value);
  @$pb.TagNumber(7)
  $core.bool hasError() => $_has(6);
  @$pb.TagNumber(7)
  void clearError() => $_clearField(7);
  @$pb.TagNumber(7)
  $0.RpcError ensureError() => $_ensure(6);
}

/// Export an account to a portable `.qaul_export` archive.
///
/// Acts on the calling account: the target identity comes from the
/// RequestContext (outer QaulRpc envelope), not the request body.
class ExportAccountRequest extends $pb.GeneratedMessage {
  factory ExportAccountRequest({
    $core.String? outputPath,
  }) {
    final result = create();
    if (outputPath != null) result.outputPath = outputPath;
    return result;
  }

  ExportAccountRequest._();

  factory ExportAccountRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ExportAccountRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ExportAccountRequest',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'qaul.rpc.account_management'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'outputPath')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ExportAccountRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ExportAccountRequest copyWith(void Function(ExportAccountRequest) updates) =>
      super.copyWith((message) => updates(message as ExportAccountRequest))
          as ExportAccountRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ExportAccountRequest create() => ExportAccountRequest._();
  @$core.override
  ExportAccountRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ExportAccountRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ExportAccountRequest>(create);
  static ExportAccountRequest? _defaultInstance;

  /// directory to write the archive into; empty = default (storage root)
  @$pb.TagNumber(1)
  $core.String get outputPath => $_getSZ(0);
  @$pb.TagNumber(1)
  set outputPath($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasOutputPath() => $_has(0);
  @$pb.TagNumber(1)
  void clearOutputPath() => $_clearField(1);
}

class ExportAccountResponse extends $pb.GeneratedMessage {
  factory ExportAccountResponse({
    $core.String? path,
  }) {
    final result = create();
    if (path != null) result.path = path;
    return result;
  }

  ExportAccountResponse._();

  factory ExportAccountResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ExportAccountResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ExportAccountResponse',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'qaul.rpc.account_management'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'path')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ExportAccountResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ExportAccountResponse copyWith(
          void Function(ExportAccountResponse) updates) =>
      super.copyWith((message) => updates(message as ExportAccountResponse))
          as ExportAccountResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ExportAccountResponse create() => ExportAccountResponse._();
  @$core.override
  ExportAccountResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ExportAccountResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ExportAccountResponse>(create);
  static ExportAccountResponse? _defaultInstance;

  /// path the archive was written to
  @$pb.TagNumber(1)
  $core.String get path => $_getSZ(0);
  @$pb.TagNumber(1)
  set path($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasPath() => $_has(0);
  @$pb.TagNumber(1)
  void clearPath() => $_clearField(1);
}

/// Delete the calling account and all of its data from this node.
///
/// Acts on the calling account: the target identity comes from the
/// RequestContext (outer QaulRpc envelope), so the message carries no fields.
class DeleteAccountRequest extends $pb.GeneratedMessage {
  factory DeleteAccountRequest() => create();

  DeleteAccountRequest._();

  factory DeleteAccountRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory DeleteAccountRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DeleteAccountRequest',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'qaul.rpc.account_management'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteAccountRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteAccountRequest copyWith(void Function(DeleteAccountRequest) updates) =>
      super.copyWith((message) => updates(message as DeleteAccountRequest))
          as DeleteAccountRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static DeleteAccountRequest create() => DeleteAccountRequest._();
  @$core.override
  DeleteAccountRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static DeleteAccountRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DeleteAccountRequest>(create);
  static DeleteAccountRequest? _defaultInstance;
}

/// Restore an account from a `.qaul_export` archive on disk.
class RestoreAccountRequest extends $pb.GeneratedMessage {
  factory RestoreAccountRequest({
    $core.String? archivePath,
  }) {
    final result = create();
    if (archivePath != null) result.archivePath = archivePath;
    return result;
  }

  RestoreAccountRequest._();

  factory RestoreAccountRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory RestoreAccountRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RestoreAccountRequest',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'qaul.rpc.account_management'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'archivePath')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RestoreAccountRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RestoreAccountRequest copyWith(
          void Function(RestoreAccountRequest) updates) =>
      super.copyWith((message) => updates(message as RestoreAccountRequest))
          as RestoreAccountRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RestoreAccountRequest create() => RestoreAccountRequest._();
  @$core.override
  RestoreAccountRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static RestoreAccountRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RestoreAccountRequest>(create);
  static RestoreAccountRequest? _defaultInstance;

  /// path to the `.qaul_export` archive
  @$pb.TagNumber(1)
  $core.String get archivePath => $_getSZ(0);
  @$pb.TagNumber(1)
  set archivePath($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasArchivePath() => $_has(0);
  @$pb.TagNumber(1)
  void clearArchivePath() => $_clearField(1);
}

class RestoreAccountResponse extends $pb.GeneratedMessage {
  factory RestoreAccountResponse({
    $core.List<$core.int>? userId,
    $core.String? userIdBase58,
  }) {
    final result = create();
    if (userId != null) result.userId = userId;
    if (userIdBase58 != null) result.userIdBase58 = userIdBase58;
    return result;
  }

  RestoreAccountResponse._();

  factory RestoreAccountResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory RestoreAccountResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RestoreAccountResponse',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'qaul.rpc.account_management'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'userId', $pb.PbFieldType.OY)
    ..aOS(2, _omitFieldNames ? '' : 'userIdBase58')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RestoreAccountResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RestoreAccountResponse copyWith(
          void Function(RestoreAccountResponse) updates) =>
      super.copyWith((message) => updates(message as RestoreAccountResponse))
          as RestoreAccountResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RestoreAccountResponse create() => RestoreAccountResponse._();
  @$core.override
  RestoreAccountResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static RestoreAccountResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RestoreAccountResponse>(create);
  static RestoreAccountResponse? _defaultInstance;

  /// binary user id of the restored account
  @$pb.TagNumber(1)
  $core.List<$core.int> get userId => $_getN(0);
  @$pb.TagNumber(1)
  set userId($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasUserId() => $_has(0);
  @$pb.TagNumber(1)
  void clearUserId() => $_clearField(1);

  /// base58 user id of the restored account
  @$pb.TagNumber(2)
  $core.String get userIdBase58 => $_getSZ(1);
  @$pb.TagNumber(2)
  set userIdBase58($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasUserIdBase58() => $_has(1);
  @$pb.TagNumber(2)
  void clearUserIdBase58() => $_clearField(2);
}

class AccountManagementServiceApi {
  final $pb.RpcClient _client;

  AccountManagementServiceApi(this._client);

  $async.Future<ExportAccountResponse> export(
          $pb.ClientContext? ctx, ExportAccountRequest request) =>
      _client.invoke<ExportAccountResponse>(ctx, 'AccountManagementService',
          'Export', request, ExportAccountResponse());
  $async.Future<$0.Ack> delete(
          $pb.ClientContext? ctx, DeleteAccountRequest request) =>
      _client.invoke<$0.Ack>(
          ctx, 'AccountManagementService', 'Delete', request, $0.Ack());
  $async.Future<RestoreAccountResponse> restore(
          $pb.ClientContext? ctx, RestoreAccountRequest request) =>
      _client.invoke<RestoreAccountResponse>(ctx, 'AccountManagementService',
          'Restore', request, RestoreAccountResponse());
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
