// This is a generated file - do not edit.
//
// Generated from rpc/authentication.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:async' as $async;
import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import '../common/common.pb.dart' as $0;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

enum AuthRpc_Message {
  authRequest,
  authChallenge,
  authResponse,
  authResult,
  usersRequest,
  usersResponse,
  logoutRequest,
  sessionStatusRequest,
  sessionStatusResponse,
  ack,
  error,
  notSet
}

class AuthRpc extends $pb.GeneratedMessage {
  factory AuthRpc({
    AuthRequest? authRequest,
    AuthChallenge? authChallenge,
    AuthResponse? authResponse,
    AuthResult? authResult,
    UsersRequest? usersRequest,
    UsersResponse? usersResponse,
    LogoutRequest? logoutRequest,
    SessionStatusRequest? sessionStatusRequest,
    SessionStatusResponse? sessionStatusResponse,
    $0.Ack? ack,
    $0.RpcError? error,
  }) {
    final result = create();
    if (authRequest != null) result.authRequest = authRequest;
    if (authChallenge != null) result.authChallenge = authChallenge;
    if (authResponse != null) result.authResponse = authResponse;
    if (authResult != null) result.authResult = authResult;
    if (usersRequest != null) result.usersRequest = usersRequest;
    if (usersResponse != null) result.usersResponse = usersResponse;
    if (logoutRequest != null) result.logoutRequest = logoutRequest;
    if (sessionStatusRequest != null)
      result.sessionStatusRequest = sessionStatusRequest;
    if (sessionStatusResponse != null)
      result.sessionStatusResponse = sessionStatusResponse;
    if (ack != null) result.ack = ack;
    if (error != null) result.error = error;
    return result;
  }

  AuthRpc._();

  factory AuthRpc.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory AuthRpc.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, AuthRpc_Message> _AuthRpc_MessageByTag = {
    1: AuthRpc_Message.authRequest,
    2: AuthRpc_Message.authChallenge,
    3: AuthRpc_Message.authResponse,
    4: AuthRpc_Message.authResult,
    5: AuthRpc_Message.usersRequest,
    6: AuthRpc_Message.usersResponse,
    7: AuthRpc_Message.logoutRequest,
    8: AuthRpc_Message.sessionStatusRequest,
    9: AuthRpc_Message.sessionStatusResponse,
    10: AuthRpc_Message.ack,
    11: AuthRpc_Message.error,
    0: AuthRpc_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AuthRpc',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'qaul.rpc.authentication'),
      createEmptyInstance: create)
    ..oo(0, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11])
    ..aOM<AuthRequest>(1, _omitFieldNames ? '' : 'authRequest',
        subBuilder: AuthRequest.create)
    ..aOM<AuthChallenge>(2, _omitFieldNames ? '' : 'authChallenge',
        subBuilder: AuthChallenge.create)
    ..aOM<AuthResponse>(3, _omitFieldNames ? '' : 'authResponse',
        subBuilder: AuthResponse.create)
    ..aOM<AuthResult>(4, _omitFieldNames ? '' : 'authResult',
        subBuilder: AuthResult.create)
    ..aOM<UsersRequest>(5, _omitFieldNames ? '' : 'usersRequest',
        subBuilder: UsersRequest.create)
    ..aOM<UsersResponse>(6, _omitFieldNames ? '' : 'usersResponse',
        subBuilder: UsersResponse.create)
    ..aOM<LogoutRequest>(7, _omitFieldNames ? '' : 'logoutRequest',
        subBuilder: LogoutRequest.create)
    ..aOM<SessionStatusRequest>(
        8, _omitFieldNames ? '' : 'sessionStatusRequest',
        subBuilder: SessionStatusRequest.create)
    ..aOM<SessionStatusResponse>(
        9, _omitFieldNames ? '' : 'sessionStatusResponse',
        subBuilder: SessionStatusResponse.create)
    ..aOM<$0.Ack>(10, _omitFieldNames ? '' : 'ack', subBuilder: $0.Ack.create)
    ..aOM<$0.RpcError>(11, _omitFieldNames ? '' : 'error',
        subBuilder: $0.RpcError.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AuthRpc clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AuthRpc copyWith(void Function(AuthRpc) updates) =>
      super.copyWith((message) => updates(message as AuthRpc)) as AuthRpc;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static AuthRpc create() => AuthRpc._();
  @$core.override
  AuthRpc createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static AuthRpc getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<AuthRpc>(create);
  static AuthRpc? _defaultInstance;

  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  @$pb.TagNumber(6)
  @$pb.TagNumber(7)
  @$pb.TagNumber(8)
  @$pb.TagNumber(9)
  @$pb.TagNumber(10)
  @$pb.TagNumber(11)
  AuthRpc_Message whichMessage() => _AuthRpc_MessageByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  @$pb.TagNumber(6)
  @$pb.TagNumber(7)
  @$pb.TagNumber(8)
  @$pb.TagNumber(9)
  @$pb.TagNumber(10)
  @$pb.TagNumber(11)
  void clearMessage() => $_clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  AuthRequest get authRequest => $_getN(0);
  @$pb.TagNumber(1)
  set authRequest(AuthRequest value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasAuthRequest() => $_has(0);
  @$pb.TagNumber(1)
  void clearAuthRequest() => $_clearField(1);
  @$pb.TagNumber(1)
  AuthRequest ensureAuthRequest() => $_ensure(0);

  @$pb.TagNumber(2)
  AuthChallenge get authChallenge => $_getN(1);
  @$pb.TagNumber(2)
  set authChallenge(AuthChallenge value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasAuthChallenge() => $_has(1);
  @$pb.TagNumber(2)
  void clearAuthChallenge() => $_clearField(2);
  @$pb.TagNumber(2)
  AuthChallenge ensureAuthChallenge() => $_ensure(1);

  @$pb.TagNumber(3)
  AuthResponse get authResponse => $_getN(2);
  @$pb.TagNumber(3)
  set authResponse(AuthResponse value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasAuthResponse() => $_has(2);
  @$pb.TagNumber(3)
  void clearAuthResponse() => $_clearField(3);
  @$pb.TagNumber(3)
  AuthResponse ensureAuthResponse() => $_ensure(2);

  @$pb.TagNumber(4)
  AuthResult get authResult => $_getN(3);
  @$pb.TagNumber(4)
  set authResult(AuthResult value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasAuthResult() => $_has(3);
  @$pb.TagNumber(4)
  void clearAuthResult() => $_clearField(4);
  @$pb.TagNumber(4)
  AuthResult ensureAuthResult() => $_ensure(3);

  @$pb.TagNumber(5)
  UsersRequest get usersRequest => $_getN(4);
  @$pb.TagNumber(5)
  set usersRequest(UsersRequest value) => $_setField(5, value);
  @$pb.TagNumber(5)
  $core.bool hasUsersRequest() => $_has(4);
  @$pb.TagNumber(5)
  void clearUsersRequest() => $_clearField(5);
  @$pb.TagNumber(5)
  UsersRequest ensureUsersRequest() => $_ensure(4);

  @$pb.TagNumber(6)
  UsersResponse get usersResponse => $_getN(5);
  @$pb.TagNumber(6)
  set usersResponse(UsersResponse value) => $_setField(6, value);
  @$pb.TagNumber(6)
  $core.bool hasUsersResponse() => $_has(5);
  @$pb.TagNumber(6)
  void clearUsersResponse() => $_clearField(6);
  @$pb.TagNumber(6)
  UsersResponse ensureUsersResponse() => $_ensure(5);

  /// logout request (drop the daemon-side session)
  @$pb.TagNumber(7)
  LogoutRequest get logoutRequest => $_getN(6);
  @$pb.TagNumber(7)
  set logoutRequest(LogoutRequest value) => $_setField(7, value);
  @$pb.TagNumber(7)
  $core.bool hasLogoutRequest() => $_has(6);
  @$pb.TagNumber(7)
  void clearLogoutRequest() => $_clearField(7);
  @$pb.TagNumber(7)
  LogoutRequest ensureLogoutRequest() => $_ensure(6);

  /// session-status ("active session") request / response
  @$pb.TagNumber(8)
  SessionStatusRequest get sessionStatusRequest => $_getN(7);
  @$pb.TagNumber(8)
  set sessionStatusRequest(SessionStatusRequest value) => $_setField(8, value);
  @$pb.TagNumber(8)
  $core.bool hasSessionStatusRequest() => $_has(7);
  @$pb.TagNumber(8)
  void clearSessionStatusRequest() => $_clearField(8);
  @$pb.TagNumber(8)
  SessionStatusRequest ensureSessionStatusRequest() => $_ensure(7);

  @$pb.TagNumber(9)
  SessionStatusResponse get sessionStatusResponse => $_getN(8);
  @$pb.TagNumber(9)
  set sessionStatusResponse(SessionStatusResponse value) =>
      $_setField(9, value);
  @$pb.TagNumber(9)
  $core.bool hasSessionStatusResponse() => $_has(8);
  @$pb.TagNumber(9)
  void clearSessionStatusResponse() => $_clearField(9);
  @$pb.TagNumber(9)
  SessionStatusResponse ensureSessionStatusResponse() => $_ensure(8);

  /// acknowledgement response
  @$pb.TagNumber(10)
  $0.Ack get ack => $_getN(9);
  @$pb.TagNumber(10)
  set ack($0.Ack value) => $_setField(10, value);
  @$pb.TagNumber(10)
  $core.bool hasAck() => $_has(9);
  @$pb.TagNumber(10)
  void clearAck() => $_clearField(10);
  @$pb.TagNumber(10)
  $0.Ack ensureAck() => $_ensure(9);

  /// RPC error response
  @$pb.TagNumber(11)
  $0.RpcError get error => $_getN(10);
  @$pb.TagNumber(11)
  set error($0.RpcError value) => $_setField(11, value);
  @$pb.TagNumber(11)
  $core.bool hasError() => $_has(10);
  @$pb.TagNumber(11)
  void clearError() => $_clearField(11);
  @$pb.TagNumber(11)
  $0.RpcError ensureError() => $_ensure(10);
}

class UsersRequest extends $pb.GeneratedMessage {
  factory UsersRequest() => create();

  UsersRequest._();

  factory UsersRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory UsersRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UsersRequest',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'qaul.rpc.authentication'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UsersRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UsersRequest copyWith(void Function(UsersRequest) updates) =>
      super.copyWith((message) => updates(message as UsersRequest))
          as UsersRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static UsersRequest create() => UsersRequest._();
  @$core.override
  UsersRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static UsersRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UsersRequest>(create);
  static UsersRequest? _defaultInstance;
}

class UsersResponse extends $pb.GeneratedMessage {
  factory UsersResponse({
    $core.Iterable<UserInfo>? users,
    $core.String? errorMessage,
  }) {
    final result = create();
    if (users != null) result.users.addAll(users);
    if (errorMessage != null) result.errorMessage = errorMessage;
    return result;
  }

  UsersResponse._();

  factory UsersResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory UsersResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UsersResponse',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'qaul.rpc.authentication'),
      createEmptyInstance: create)
    ..pPM<UserInfo>(1, _omitFieldNames ? '' : 'users',
        subBuilder: UserInfo.create)
    ..aOS(2, _omitFieldNames ? '' : 'errorMessage')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UsersResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UsersResponse copyWith(void Function(UsersResponse) updates) =>
      super.copyWith((message) => updates(message as UsersResponse))
          as UsersResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static UsersResponse create() => UsersResponse._();
  @$core.override
  UsersResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static UsersResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UsersResponse>(create);
  static UsersResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<UserInfo> get users => $_getList(0);

  @$pb.TagNumber(2)
  $core.String get errorMessage => $_getSZ(1);
  @$pb.TagNumber(2)
  set errorMessage($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasErrorMessage() => $_has(1);
  @$pb.TagNumber(2)
  void clearErrorMessage() => $_clearField(2);
}

class UserInfo extends $pb.GeneratedMessage {
  factory UserInfo({
    $core.String? username,
    $core.List<$core.int>? userId,
    $core.String? salt,
    $core.bool? hasPassword,
  }) {
    final result = create();
    if (username != null) result.username = username;
    if (userId != null) result.userId = userId;
    if (salt != null) result.salt = salt;
    if (hasPassword != null) result.hasPassword = hasPassword;
    return result;
  }

  UserInfo._();

  factory UserInfo.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory UserInfo.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UserInfo',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'qaul.rpc.authentication'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'username')
    ..a<$core.List<$core.int>>(
        2, _omitFieldNames ? '' : 'userId', $pb.PbFieldType.OY)
    ..aOS(3, _omitFieldNames ? '' : 'salt')
    ..aOB(4, _omitFieldNames ? '' : 'hasPassword')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UserInfo clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UserInfo copyWith(void Function(UserInfo) updates) =>
      super.copyWith((message) => updates(message as UserInfo)) as UserInfo;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static UserInfo create() => UserInfo._();
  @$core.override
  UserInfo createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static UserInfo getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UserInfo>(create);
  static UserInfo? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get username => $_getSZ(0);
  @$pb.TagNumber(1)
  set username($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasUsername() => $_has(0);
  @$pb.TagNumber(1)
  void clearUsername() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get userId => $_getN(1);
  @$pb.TagNumber(2)
  set userId($core.List<$core.int> value) => $_setBytes(1, value);
  @$pb.TagNumber(2)
  $core.bool hasUserId() => $_has(1);
  @$pb.TagNumber(2)
  void clearUserId() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get salt => $_getSZ(2);
  @$pb.TagNumber(3)
  set salt($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasSalt() => $_has(2);
  @$pb.TagNumber(3)
  void clearSalt() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.bool get hasPassword => $_getBF(3);
  @$pb.TagNumber(4)
  set hasPassword($core.bool value) => $_setBool(3, value);
  @$pb.TagNumber(4)
  $core.bool hasHasPassword() => $_has(3);
  @$pb.TagNumber(4)
  void clearHasPassword() => $_clearField(4);
}

class AuthRequest extends $pb.GeneratedMessage {
  factory AuthRequest({
    $core.List<$core.int>? userId,
  }) {
    final result = create();
    if (userId != null) result.userId = userId;
    return result;
  }

  AuthRequest._();

  factory AuthRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory AuthRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AuthRequest',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'qaul.rpc.authentication'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'userId', $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AuthRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AuthRequest copyWith(void Function(AuthRequest) updates) =>
      super.copyWith((message) => updates(message as AuthRequest))
          as AuthRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static AuthRequest create() => AuthRequest._();
  @$core.override
  AuthRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static AuthRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AuthRequest>(create);
  static AuthRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get userId => $_getN(0);
  @$pb.TagNumber(1)
  set userId($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasUserId() => $_has(0);
  @$pb.TagNumber(1)
  void clearUserId() => $_clearField(1);
}

class AuthChallenge extends $pb.GeneratedMessage {
  factory AuthChallenge({
    $fixnum.Int64? nonce,
    $fixnum.Int64? expiresAt,
  }) {
    final result = create();
    if (nonce != null) result.nonce = nonce;
    if (expiresAt != null) result.expiresAt = expiresAt;
    return result;
  }

  AuthChallenge._();

  factory AuthChallenge.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory AuthChallenge.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AuthChallenge',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'qaul.rpc.authentication'),
      createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, _omitFieldNames ? '' : 'nonce', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(
        2, _omitFieldNames ? '' : 'expiresAt', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AuthChallenge clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AuthChallenge copyWith(void Function(AuthChallenge) updates) =>
      super.copyWith((message) => updates(message as AuthChallenge))
          as AuthChallenge;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static AuthChallenge create() => AuthChallenge._();
  @$core.override
  AuthChallenge createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static AuthChallenge getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AuthChallenge>(create);
  static AuthChallenge? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get nonce => $_getI64(0);
  @$pb.TagNumber(1)
  set nonce($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasNonce() => $_has(0);
  @$pb.TagNumber(1)
  void clearNonce() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get expiresAt => $_getI64(1);
  @$pb.TagNumber(2)
  set expiresAt($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasExpiresAt() => $_has(1);
  @$pb.TagNumber(2)
  void clearExpiresAt() => $_clearField(2);
}

class AuthResponse extends $pb.GeneratedMessage {
  factory AuthResponse({
    $core.List<$core.int>? challengeHash,
    $core.List<$core.int>? userId,
  }) {
    final result = create();
    if (challengeHash != null) result.challengeHash = challengeHash;
    if (userId != null) result.userId = userId;
    return result;
  }

  AuthResponse._();

  factory AuthResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory AuthResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AuthResponse',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'qaul.rpc.authentication'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'challengeHash', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(
        2, _omitFieldNames ? '' : 'userId', $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AuthResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AuthResponse copyWith(void Function(AuthResponse) updates) =>
      super.copyWith((message) => updates(message as AuthResponse))
          as AuthResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static AuthResponse create() => AuthResponse._();
  @$core.override
  AuthResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static AuthResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AuthResponse>(create);
  static AuthResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get challengeHash => $_getN(0);
  @$pb.TagNumber(1)
  set challengeHash($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasChallengeHash() => $_has(0);
  @$pb.TagNumber(1)
  void clearChallengeHash() => $_clearField(1);

  /// caller identity. The generated service dispatch only forwards the
  /// decoded request to the handler (not the outer QaulRpc.user_id), so the
  /// challenge response must carry the user id it is answering for.
  @$pb.TagNumber(2)
  $core.List<$core.int> get userId => $_getN(1);
  @$pb.TagNumber(2)
  set userId($core.List<$core.int> value) => $_setBytes(1, value);
  @$pb.TagNumber(2)
  $core.bool hasUserId() => $_has(1);
  @$pb.TagNumber(2)
  void clearUserId() => $_clearField(2);
}

class AuthResult extends $pb.GeneratedMessage {
  factory AuthResult({
    $core.bool? success,
    $core.String? errorMessage,
  }) {
    final result = create();
    if (success != null) result.success = success;
    if (errorMessage != null) result.errorMessage = errorMessage;
    return result;
  }

  AuthResult._();

  factory AuthResult.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory AuthResult.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AuthResult',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'qaul.rpc.authentication'),
      createEmptyInstance: create)
    ..aOB(1, _omitFieldNames ? '' : 'success')
    ..aOS(2, _omitFieldNames ? '' : 'errorMessage')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AuthResult clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AuthResult copyWith(void Function(AuthResult) updates) =>
      super.copyWith((message) => updates(message as AuthResult)) as AuthResult;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static AuthResult create() => AuthResult._();
  @$core.override
  AuthResult createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static AuthResult getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AuthResult>(create);
  static AuthResult? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get success => $_getBF(0);
  @$pb.TagNumber(1)
  set success($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSuccess() => $_has(0);
  @$pb.TagNumber(1)
  void clearSuccess() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get errorMessage => $_getSZ(1);
  @$pb.TagNumber(2)
  set errorMessage($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasErrorMessage() => $_has(1);
  @$pb.TagNumber(2)
  void clearErrorMessage() => $_clearField(2);
}

/// Drop the daemon-side authenticated session for the calling user.
/// The target is the caller's identity, taken from the RequestContext (outer
/// QaulRpc envelope), so the message carries no fields — a caller can only log
/// itself out.
class LogoutRequest extends $pb.GeneratedMessage {
  factory LogoutRequest() => create();

  LogoutRequest._();

  factory LogoutRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory LogoutRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'LogoutRequest',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'qaul.rpc.authentication'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  LogoutRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  LogoutRequest copyWith(void Function(LogoutRequest) updates) =>
      super.copyWith((message) => updates(message as LogoutRequest))
          as LogoutRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static LogoutRequest create() => LogoutRequest._();
  @$core.override
  LogoutRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static LogoutRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<LogoutRequest>(create);
  static LogoutRequest? _defaultInstance;
}

/// Query whether the calling user currently has an active authenticated
/// session. Identity comes from the RequestContext (outer envelope), so the
/// message carries no fields.
class SessionStatusRequest extends $pb.GeneratedMessage {
  factory SessionStatusRequest() => create();

  SessionStatusRequest._();

  factory SessionStatusRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory SessionStatusRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SessionStatusRequest',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'qaul.rpc.authentication'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SessionStatusRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SessionStatusRequest copyWith(void Function(SessionStatusRequest) updates) =>
      super.copyWith((message) => updates(message as SessionStatusRequest))
          as SessionStatusRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SessionStatusRequest create() => SessionStatusRequest._();
  @$core.override
  SessionStatusRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static SessionStatusRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SessionStatusRequest>(create);
  static SessionStatusRequest? _defaultInstance;
}

class SessionStatusResponse extends $pb.GeneratedMessage {
  factory SessionStatusResponse({
    $core.bool? authenticated,
  }) {
    final result = create();
    if (authenticated != null) result.authenticated = authenticated;
    return result;
  }

  SessionStatusResponse._();

  factory SessionStatusResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory SessionStatusResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SessionStatusResponse',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'qaul.rpc.authentication'),
      createEmptyInstance: create)
    ..aOB(1, _omitFieldNames ? '' : 'authenticated')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SessionStatusResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SessionStatusResponse copyWith(
          void Function(SessionStatusResponse) updates) =>
      super.copyWith((message) => updates(message as SessionStatusResponse))
          as SessionStatusResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SessionStatusResponse create() => SessionStatusResponse._();
  @$core.override
  SessionStatusResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static SessionStatusResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SessionStatusResponse>(create);
  static SessionStatusResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get authenticated => $_getBF(0);
  @$pb.TagNumber(1)
  set authenticated($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasAuthenticated() => $_has(0);
  @$pb.TagNumber(1)
  void clearAuthenticated() => $_clearField(1);
}

class AuthRpcServiceApi {
  final $pb.RpcClient _client;

  AuthRpcServiceApi(this._client);

  $async.Future<UsersResponse> users(
          $pb.ClientContext? ctx, UsersRequest request) =>
      _client.invoke<UsersResponse>(
          ctx, 'AuthRpcService', 'Users', request, UsersResponse());
  $async.Future<AuthChallenge> requestChallenge(
          $pb.ClientContext? ctx, AuthRequest request) =>
      _client.invoke<AuthChallenge>(
          ctx, 'AuthRpcService', 'RequestChallenge', request, AuthChallenge());
  $async.Future<AuthResult> respondChallenge(
          $pb.ClientContext? ctx, AuthResponse request) =>
      _client.invoke<AuthResult>(
          ctx, 'AuthRpcService', 'RespondChallenge', request, AuthResult());
  $async.Future<$0.Ack> logoutSession(
          $pb.ClientContext? ctx, LogoutRequest request) =>
      _client.invoke<$0.Ack>(
          ctx, 'AuthRpcService', 'LogoutSession', request, $0.Ack());
  $async.Future<SessionStatusResponse> sessionStatus(
          $pb.ClientContext? ctx, SessionStatusRequest request) =>
      _client.invoke<SessionStatusResponse>(ctx, 'AuthRpcService',
          'SessionStatus', request, SessionStatusResponse());
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
