// This is a generated file - do not edit.
//
// Generated from node/user_accounts.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

enum UserAccounts_Message {
  getDefaultUserAccount,
  createUserAccount,
  defaultUserAccount,
  myUserAccount,
  setPasswordRequest,
  setPasswordResponse,
  notSet
}

/// user account rpc message container
class UserAccounts extends $pb.GeneratedMessage {
  factory UserAccounts({
    $core.bool? getDefaultUserAccount,
    CreateUserAccount? createUserAccount,
    DefaultUserAccount? defaultUserAccount,
    MyUserAccount? myUserAccount,
    SetPasswordRequest? setPasswordRequest,
    SetPasswordResponse? setPasswordResponse,
  }) {
    final result = create();
    if (getDefaultUserAccount != null)
      result.getDefaultUserAccount = getDefaultUserAccount;
    if (createUserAccount != null) result.createUserAccount = createUserAccount;
    if (defaultUserAccount != null)
      result.defaultUserAccount = defaultUserAccount;
    if (myUserAccount != null) result.myUserAccount = myUserAccount;
    if (setPasswordRequest != null)
      result.setPasswordRequest = setPasswordRequest;
    if (setPasswordResponse != null)
      result.setPasswordResponse = setPasswordResponse;
    return result;
  }

  UserAccounts._();

  factory UserAccounts.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory UserAccounts.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, UserAccounts_Message>
      _UserAccounts_MessageByTag = {
    1: UserAccounts_Message.getDefaultUserAccount,
    2: UserAccounts_Message.createUserAccount,
    3: UserAccounts_Message.defaultUserAccount,
    4: UserAccounts_Message.myUserAccount,
    5: UserAccounts_Message.setPasswordRequest,
    6: UserAccounts_Message.setPasswordResponse,
    0: UserAccounts_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UserAccounts',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'qaul.rpc.user_accounts'),
      createEmptyInstance: create)
    ..oo(0, [1, 2, 3, 4, 5, 6])
    ..aOB(1, _omitFieldNames ? '' : 'getDefaultUserAccount')
    ..aOM<CreateUserAccount>(2, _omitFieldNames ? '' : 'createUserAccount',
        subBuilder: CreateUserAccount.create)
    ..aOM<DefaultUserAccount>(3, _omitFieldNames ? '' : 'defaultUserAccount',
        subBuilder: DefaultUserAccount.create)
    ..aOM<MyUserAccount>(4, _omitFieldNames ? '' : 'myUserAccount',
        subBuilder: MyUserAccount.create)
    ..aOM<SetPasswordRequest>(5, _omitFieldNames ? '' : 'setPasswordRequest',
        subBuilder: SetPasswordRequest.create)
    ..aOM<SetPasswordResponse>(6, _omitFieldNames ? '' : 'setPasswordResponse',
        subBuilder: SetPasswordResponse.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UserAccounts clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UserAccounts copyWith(void Function(UserAccounts) updates) =>
      super.copyWith((message) => updates(message as UserAccounts))
          as UserAccounts;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static UserAccounts create() => UserAccounts._();
  @$core.override
  UserAccounts createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static UserAccounts getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UserAccounts>(create);
  static UserAccounts? _defaultInstance;

  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  @$pb.TagNumber(6)
  UserAccounts_Message whichMessage() =>
      _UserAccounts_MessageByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  @$pb.TagNumber(6)
  void clearMessage() => $_clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  $core.bool get getDefaultUserAccount => $_getBF(0);
  @$pb.TagNumber(1)
  set getDefaultUserAccount($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasGetDefaultUserAccount() => $_has(0);
  @$pb.TagNumber(1)
  void clearGetDefaultUserAccount() => $_clearField(1);

  @$pb.TagNumber(2)
  CreateUserAccount get createUserAccount => $_getN(1);
  @$pb.TagNumber(2)
  set createUserAccount(CreateUserAccount value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasCreateUserAccount() => $_has(1);
  @$pb.TagNumber(2)
  void clearCreateUserAccount() => $_clearField(2);
  @$pb.TagNumber(2)
  CreateUserAccount ensureCreateUserAccount() => $_ensure(1);

  @$pb.TagNumber(3)
  DefaultUserAccount get defaultUserAccount => $_getN(2);
  @$pb.TagNumber(3)
  set defaultUserAccount(DefaultUserAccount value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasDefaultUserAccount() => $_has(2);
  @$pb.TagNumber(3)
  void clearDefaultUserAccount() => $_clearField(3);
  @$pb.TagNumber(3)
  DefaultUserAccount ensureDefaultUserAccount() => $_ensure(2);

  @$pb.TagNumber(4)
  MyUserAccount get myUserAccount => $_getN(3);
  @$pb.TagNumber(4)
  set myUserAccount(MyUserAccount value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasMyUserAccount() => $_has(3);
  @$pb.TagNumber(4)
  void clearMyUserAccount() => $_clearField(4);
  @$pb.TagNumber(4)
  MyUserAccount ensureMyUserAccount() => $_ensure(3);

  @$pb.TagNumber(5)
  SetPasswordRequest get setPasswordRequest => $_getN(4);
  @$pb.TagNumber(5)
  set setPasswordRequest(SetPasswordRequest value) => $_setField(5, value);
  @$pb.TagNumber(5)
  $core.bool hasSetPasswordRequest() => $_has(4);
  @$pb.TagNumber(5)
  void clearSetPasswordRequest() => $_clearField(5);
  @$pb.TagNumber(5)
  SetPasswordRequest ensureSetPasswordRequest() => $_ensure(4);

  @$pb.TagNumber(6)
  SetPasswordResponse get setPasswordResponse => $_getN(5);
  @$pb.TagNumber(6)
  set setPasswordResponse(SetPasswordResponse value) => $_setField(6, value);
  @$pb.TagNumber(6)
  $core.bool hasSetPasswordResponse() => $_has(5);
  @$pb.TagNumber(6)
  void clearSetPasswordResponse() => $_clearField(6);
  @$pb.TagNumber(6)
  SetPasswordResponse ensureSetPasswordResponse() => $_ensure(5);
}

/// create a new user on this node
class CreateUserAccount extends $pb.GeneratedMessage {
  factory CreateUserAccount({
    $core.String? name,
    $core.String? password,
  }) {
    final result = create();
    if (name != null) result.name = name;
    if (password != null) result.password = password;
    return result;
  }

  CreateUserAccount._();

  factory CreateUserAccount.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory CreateUserAccount.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CreateUserAccount',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'qaul.rpc.user_accounts'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..aOS(2, _omitFieldNames ? '' : 'password')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateUserAccount clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CreateUserAccount copyWith(void Function(CreateUserAccount) updates) =>
      super.copyWith((message) => updates(message as CreateUserAccount))
          as CreateUserAccount;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static CreateUserAccount create() => CreateUserAccount._();
  @$core.override
  CreateUserAccount createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static CreateUserAccount getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CreateUserAccount>(create);
  static CreateUserAccount? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get password => $_getSZ(1);
  @$pb.TagNumber(2)
  set password($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasPassword() => $_has(1);
  @$pb.TagNumber(2)
  void clearPassword() => $_clearField(2);
}

/// set password request for existing user
class SetPasswordRequest extends $pb.GeneratedMessage {
  factory SetPasswordRequest({
    $core.String? password,
  }) {
    final result = create();
    if (password != null) result.password = password;
    return result;
  }

  SetPasswordRequest._();

  factory SetPasswordRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory SetPasswordRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SetPasswordRequest',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'qaul.rpc.user_accounts'),
      createEmptyInstance: create)
    ..aOS(2, _omitFieldNames ? '' : 'password')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SetPasswordRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SetPasswordRequest copyWith(void Function(SetPasswordRequest) updates) =>
      super.copyWith((message) => updates(message as SetPasswordRequest))
          as SetPasswordRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SetPasswordRequest create() => SetPasswordRequest._();
  @$core.override
  SetPasswordRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static SetPasswordRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SetPasswordRequest>(create);
  static SetPasswordRequest? _defaultInstance;

  @$pb.TagNumber(2)
  $core.String get password => $_getSZ(0);
  @$pb.TagNumber(2)
  set password($core.String value) => $_setString(0, value);
  @$pb.TagNumber(2)
  $core.bool hasPassword() => $_has(0);
  @$pb.TagNumber(2)
  void clearPassword() => $_clearField(2);
}

/// set password response
class SetPasswordResponse extends $pb.GeneratedMessage {
  factory SetPasswordResponse({
    $core.bool? success,
    $core.String? errorMessage,
  }) {
    final result = create();
    if (success != null) result.success = success;
    if (errorMessage != null) result.errorMessage = errorMessage;
    return result;
  }

  SetPasswordResponse._();

  factory SetPasswordResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory SetPasswordResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SetPasswordResponse',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'qaul.rpc.user_accounts'),
      createEmptyInstance: create)
    ..aOB(1, _omitFieldNames ? '' : 'success')
    ..aOS(2, _omitFieldNames ? '' : 'errorMessage')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SetPasswordResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SetPasswordResponse copyWith(void Function(SetPasswordResponse) updates) =>
      super.copyWith((message) => updates(message as SetPasswordResponse))
          as SetPasswordResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SetPasswordResponse create() => SetPasswordResponse._();
  @$core.override
  SetPasswordResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static SetPasswordResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SetPasswordResponse>(create);
  static SetPasswordResponse? _defaultInstance;

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

/// Session Information
class DefaultUserAccount extends $pb.GeneratedMessage {
  factory DefaultUserAccount({
    $core.bool? userAccountExists,
    MyUserAccount? myUserAccount,
  }) {
    final result = create();
    if (userAccountExists != null) result.userAccountExists = userAccountExists;
    if (myUserAccount != null) result.myUserAccount = myUserAccount;
    return result;
  }

  DefaultUserAccount._();

  factory DefaultUserAccount.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory DefaultUserAccount.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DefaultUserAccount',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'qaul.rpc.user_accounts'),
      createEmptyInstance: create)
    ..aOB(1, _omitFieldNames ? '' : 'userAccountExists')
    ..aOM<MyUserAccount>(2, _omitFieldNames ? '' : 'myUserAccount',
        subBuilder: MyUserAccount.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DefaultUserAccount clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DefaultUserAccount copyWith(void Function(DefaultUserAccount) updates) =>
      super.copyWith((message) => updates(message as DefaultUserAccount))
          as DefaultUserAccount;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static DefaultUserAccount create() => DefaultUserAccount._();
  @$core.override
  DefaultUserAccount createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static DefaultUserAccount getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DefaultUserAccount>(create);
  static DefaultUserAccount? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get userAccountExists => $_getBF(0);
  @$pb.TagNumber(1)
  set userAccountExists($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasUserAccountExists() => $_has(0);
  @$pb.TagNumber(1)
  void clearUserAccountExists() => $_clearField(1);

  @$pb.TagNumber(2)
  MyUserAccount get myUserAccount => $_getN(1);
  @$pb.TagNumber(2)
  set myUserAccount(MyUserAccount value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasMyUserAccount() => $_has(1);
  @$pb.TagNumber(2)
  void clearMyUserAccount() => $_clearField(2);
  @$pb.TagNumber(2)
  MyUserAccount ensureMyUserAccount() => $_ensure(1);
}

/// Information about my user
class MyUserAccount extends $pb.GeneratedMessage {
  factory MyUserAccount({
    $core.String? name,
    $core.List<$core.int>? id,
    $core.String? idBase58,
    $core.List<$core.int>? key,
    $core.String? keyType,
    $core.String? keyBase58,
    $core.bool? hasPassword,
  }) {
    final result = create();
    if (name != null) result.name = name;
    if (id != null) result.id = id;
    if (idBase58 != null) result.idBase58 = idBase58;
    if (key != null) result.key = key;
    if (keyType != null) result.keyType = keyType;
    if (keyBase58 != null) result.keyBase58 = keyBase58;
    if (hasPassword != null) result.hasPassword = hasPassword;
    return result;
  }

  MyUserAccount._();

  factory MyUserAccount.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory MyUserAccount.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MyUserAccount',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'qaul.rpc.user_accounts'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..a<$core.List<$core.int>>(
        2, _omitFieldNames ? '' : 'id', $pb.PbFieldType.OY)
    ..aOS(3, _omitFieldNames ? '' : 'idBase58')
    ..a<$core.List<$core.int>>(
        4, _omitFieldNames ? '' : 'key', $pb.PbFieldType.OY)
    ..aOS(5, _omitFieldNames ? '' : 'keyType')
    ..aOS(6, _omitFieldNames ? '' : 'keyBase58')
    ..aOB(7, _omitFieldNames ? '' : 'hasPassword')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MyUserAccount clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  MyUserAccount copyWith(void Function(MyUserAccount) updates) =>
      super.copyWith((message) => updates(message as MyUserAccount))
          as MyUserAccount;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static MyUserAccount create() => MyUserAccount._();
  @$core.override
  MyUserAccount createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static MyUserAccount getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<MyUserAccount>(create);
  static MyUserAccount? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get id => $_getN(1);
  @$pb.TagNumber(2)
  set id($core.List<$core.int> value) => $_setBytes(1, value);
  @$pb.TagNumber(2)
  $core.bool hasId() => $_has(1);
  @$pb.TagNumber(2)
  void clearId() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get idBase58 => $_getSZ(2);
  @$pb.TagNumber(3)
  set idBase58($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasIdBase58() => $_has(2);
  @$pb.TagNumber(3)
  void clearIdBase58() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.List<$core.int> get key => $_getN(3);
  @$pb.TagNumber(4)
  set key($core.List<$core.int> value) => $_setBytes(3, value);
  @$pb.TagNumber(4)
  $core.bool hasKey() => $_has(3);
  @$pb.TagNumber(4)
  void clearKey() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get keyType => $_getSZ(4);
  @$pb.TagNumber(5)
  set keyType($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasKeyType() => $_has(4);
  @$pb.TagNumber(5)
  void clearKeyType() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get keyBase58 => $_getSZ(5);
  @$pb.TagNumber(6)
  set keyBase58($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasKeyBase58() => $_has(5);
  @$pb.TagNumber(6)
  void clearKeyBase58() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.bool get hasPassword => $_getBF(6);
  @$pb.TagNumber(7)
  set hasPassword($core.bool value) => $_setBool(6, value);
  @$pb.TagNumber(7)
  $core.bool hasHasPassword() => $_has(6);
  @$pb.TagNumber(7)
  void clearHasPassword() => $_clearField(7);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
