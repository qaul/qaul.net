// This is a generated file - do not edit.
//
// Generated from router/users.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'users.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'users.pbenum.dart';

enum Users_Message {
  userRequest,
  userOnlineRequest,
  userList,
  userUpdate,
  securityNumberRequest,
  securityNumberResponse,
  notSet
}

/// users rpc message container
class Users extends $pb.GeneratedMessage {
  factory Users({
    UserRequest? userRequest,
    UserOnlineRequest? userOnlineRequest,
    UserList? userList,
    UserEntry? userUpdate,
    SecurityNumberRequest? securityNumberRequest,
    SecurityNumberResponse? securityNumberResponse,
  }) {
    final result = create();
    if (userRequest != null) result.userRequest = userRequest;
    if (userOnlineRequest != null) result.userOnlineRequest = userOnlineRequest;
    if (userList != null) result.userList = userList;
    if (userUpdate != null) result.userUpdate = userUpdate;
    if (securityNumberRequest != null)
      result.securityNumberRequest = securityNumberRequest;
    if (securityNumberResponse != null)
      result.securityNumberResponse = securityNumberResponse;
    return result;
  }

  Users._();

  factory Users.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory Users.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, Users_Message> _Users_MessageByTag = {
    1: Users_Message.userRequest,
    2: Users_Message.userOnlineRequest,
    3: Users_Message.userList,
    4: Users_Message.userUpdate,
    5: Users_Message.securityNumberRequest,
    6: Users_Message.securityNumberResponse,
    0: Users_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Users',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.users'),
      createEmptyInstance: create)
    ..oo(0, [1, 2, 3, 4, 5, 6])
    ..aOM<UserRequest>(1, _omitFieldNames ? '' : 'userRequest',
        subBuilder: UserRequest.create)
    ..aOM<UserOnlineRequest>(2, _omitFieldNames ? '' : 'userOnlineRequest',
        subBuilder: UserOnlineRequest.create)
    ..aOM<UserList>(3, _omitFieldNames ? '' : 'userList',
        subBuilder: UserList.create)
    ..aOM<UserEntry>(4, _omitFieldNames ? '' : 'userUpdate',
        subBuilder: UserEntry.create)
    ..aOM<SecurityNumberRequest>(
        5, _omitFieldNames ? '' : 'securityNumberRequest',
        subBuilder: SecurityNumberRequest.create)
    ..aOM<SecurityNumberResponse>(
        6, _omitFieldNames ? '' : 'securityNumberResponse',
        subBuilder: SecurityNumberResponse.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Users clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Users copyWith(void Function(Users) updates) =>
      super.copyWith((message) => updates(message as Users)) as Users;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Users create() => Users._();
  @$core.override
  Users createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static Users getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Users>(create);
  static Users? _defaultInstance;

  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  @$pb.TagNumber(6)
  Users_Message whichMessage() => _Users_MessageByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  @$pb.TagNumber(6)
  void clearMessage() => $_clearField($_whichOneof(0));

  /// User Request returns a user list
  /// containing all users with their connectivity
  /// field set to either online or offline.
  /// The connections are not set.
  @$pb.TagNumber(1)
  UserRequest get userRequest => $_getN(0);
  @$pb.TagNumber(1)
  set userRequest(UserRequest value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasUserRequest() => $_has(0);
  @$pb.TagNumber(1)
  void clearUserRequest() => $_clearField(1);
  @$pb.TagNumber(1)
  UserRequest ensureUserRequest() => $_ensure(0);

  /// User Online Request returns a user list
  /// of all users currently online in the network.
  /// Each user has
  @$pb.TagNumber(2)
  UserOnlineRequest get userOnlineRequest => $_getN(1);
  @$pb.TagNumber(2)
  set userOnlineRequest(UserOnlineRequest value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasUserOnlineRequest() => $_has(1);
  @$pb.TagNumber(2)
  void clearUserOnlineRequest() => $_clearField(2);
  @$pb.TagNumber(2)
  UserOnlineRequest ensureUserOnlineRequest() => $_ensure(1);

  /// User List
  ///
  /// Libqaul's return message for  'UserRequest' and
  /// 'UserOnlineRequest', containing a list of UserEntry's
  @$pb.TagNumber(3)
  UserList get userList => $_getN(2);
  @$pb.TagNumber(3)
  set userList(UserList value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasUserList() => $_has(2);
  @$pb.TagNumber(3)
  void clearUserList() => $_clearField(3);
  @$pb.TagNumber(3)
  UserList ensureUserList() => $_ensure(2);

  /// User Update
  ///
  /// Sent to libqaul to update the verification & blocked fields
  /// of a user.
  /// All other fields will be ignored.
  @$pb.TagNumber(4)
  UserEntry get userUpdate => $_getN(3);
  @$pb.TagNumber(4)
  set userUpdate(UserEntry value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasUserUpdate() => $_has(3);
  @$pb.TagNumber(4)
  void clearUserUpdate() => $_clearField(4);
  @$pb.TagNumber(4)
  UserEntry ensureUserUpdate() => $_ensure(3);

  /// Security Number Request
  ///
  /// Requests the specific security number for
  /// for the connection with this user.
  @$pb.TagNumber(5)
  SecurityNumberRequest get securityNumberRequest => $_getN(4);
  @$pb.TagNumber(5)
  set securityNumberRequest(SecurityNumberRequest value) =>
      $_setField(5, value);
  @$pb.TagNumber(5)
  $core.bool hasSecurityNumberRequest() => $_has(4);
  @$pb.TagNumber(5)
  void clearSecurityNumberRequest() => $_clearField(5);
  @$pb.TagNumber(5)
  SecurityNumberRequest ensureSecurityNumberRequest() => $_ensure(4);

  /// Security Number Response
  ///
  /// Libqaul's response containing the security number.
  ///
  /// The security number contains 8 blocks of 5 digit numbers.
  /// They shall be rendered in two rows. If a number is
  /// smaller then five-digits, the missing digits shall be filled
  /// with leading zeros.
  ///
  /// example rendering of security number:
  /// 13246 42369 46193 12484
  /// 12142 31101 09874 34545
  @$pb.TagNumber(6)
  SecurityNumberResponse get securityNumberResponse => $_getN(5);
  @$pb.TagNumber(6)
  set securityNumberResponse(SecurityNumberResponse value) =>
      $_setField(6, value);
  @$pb.TagNumber(6)
  $core.bool hasSecurityNumberResponse() => $_has(5);
  @$pb.TagNumber(6)
  void clearSecurityNumberResponse() => $_clearField(6);
  @$pb.TagNumber(6)
  SecurityNumberResponse ensureSecurityNumberResponse() => $_ensure(5);
}

/// UI request for some users
class UserRequest extends $pb.GeneratedMessage {
  factory UserRequest({
    $core.int? offset,
    $core.int? limit,
  }) {
    final result = create();
    if (offset != null) result.offset = offset;
    if (limit != null) result.limit = limit;
    return result;
  }

  UserRequest._();

  factory UserRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory UserRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UserRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.users'),
      createEmptyInstance: create)
    ..aI(10, _omitFieldNames ? '' : 'offset', fieldType: $pb.PbFieldType.OU3)
    ..aI(20, _omitFieldNames ? '' : 'limit', fieldType: $pb.PbFieldType.OU3)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UserRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UserRequest copyWith(void Function(UserRequest) updates) =>
      super.copyWith((message) => updates(message as UserRequest))
          as UserRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static UserRequest create() => UserRequest._();
  @$core.override
  UserRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static UserRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UserRequest>(create);
  static UserRequest? _defaultInstance;

  @$pb.TagNumber(10)
  $core.int get offset => $_getIZ(0);
  @$pb.TagNumber(10)
  set offset($core.int value) => $_setUnsignedInt32(0, value);
  @$pb.TagNumber(10)
  $core.bool hasOffset() => $_has(0);
  @$pb.TagNumber(10)
  void clearOffset() => $_clearField(10);

  @$pb.TagNumber(20)
  $core.int get limit => $_getIZ(1);
  @$pb.TagNumber(20)
  set limit($core.int value) => $_setUnsignedInt32(1, value);
  @$pb.TagNumber(20)
  $core.bool hasLimit() => $_has(1);
  @$pb.TagNumber(20)
  void clearLimit() => $_clearField(20);
}

/// UI request for some online users
class UserOnlineRequest extends $pb.GeneratedMessage {
  factory UserOnlineRequest({
    $core.int? offset,
    $core.int? limit,
  }) {
    final result = create();
    if (offset != null) result.offset = offset;
    if (limit != null) result.limit = limit;
    return result;
  }

  UserOnlineRequest._();

  factory UserOnlineRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory UserOnlineRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UserOnlineRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.users'),
      createEmptyInstance: create)
    ..aI(10, _omitFieldNames ? '' : 'offset', fieldType: $pb.PbFieldType.OU3)
    ..aI(20, _omitFieldNames ? '' : 'limit', fieldType: $pb.PbFieldType.OU3)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UserOnlineRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UserOnlineRequest copyWith(void Function(UserOnlineRequest) updates) =>
      super.copyWith((message) => updates(message as UserOnlineRequest))
          as UserOnlineRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static UserOnlineRequest create() => UserOnlineRequest._();
  @$core.override
  UserOnlineRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static UserOnlineRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UserOnlineRequest>(create);
  static UserOnlineRequest? _defaultInstance;

  @$pb.TagNumber(10)
  $core.int get offset => $_getIZ(0);
  @$pb.TagNumber(10)
  set offset($core.int value) => $_setUnsignedInt32(0, value);
  @$pb.TagNumber(10)
  $core.bool hasOffset() => $_has(0);
  @$pb.TagNumber(10)
  void clearOffset() => $_clearField(10);

  @$pb.TagNumber(20)
  $core.int get limit => $_getIZ(1);
  @$pb.TagNumber(20)
  set limit($core.int value) => $_setUnsignedInt32(1, value);
  @$pb.TagNumber(20)
  $core.bool hasLimit() => $_has(1);
  @$pb.TagNumber(20)
  void clearLimit() => $_clearField(20);
}

class PaginationMetadata extends $pb.GeneratedMessage {
  factory PaginationMetadata({
    $core.bool? hasMore,
    $core.int? total,
    $core.int? offset,
    $core.int? limit,
  }) {
    final result = create();
    if (hasMore != null) result.hasMore = hasMore;
    if (total != null) result.total = total;
    if (offset != null) result.offset = offset;
    if (limit != null) result.limit = limit;
    return result;
  }

  PaginationMetadata._();

  factory PaginationMetadata.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory PaginationMetadata.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PaginationMetadata',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.users'),
      createEmptyInstance: create)
    ..aOB(10, _omitFieldNames ? '' : 'hasMore')
    ..aI(20, _omitFieldNames ? '' : 'total', fieldType: $pb.PbFieldType.OU3)
    ..aI(30, _omitFieldNames ? '' : 'offset', fieldType: $pb.PbFieldType.OU3)
    ..aI(40, _omitFieldNames ? '' : 'limit', fieldType: $pb.PbFieldType.OU3)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PaginationMetadata clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PaginationMetadata copyWith(void Function(PaginationMetadata) updates) =>
      super.copyWith((message) => updates(message as PaginationMetadata))
          as PaginationMetadata;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static PaginationMetadata create() => PaginationMetadata._();
  @$core.override
  PaginationMetadata createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static PaginationMetadata getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<PaginationMetadata>(create);
  static PaginationMetadata? _defaultInstance;

  @$pb.TagNumber(10)
  $core.bool get hasMore => $_getBF(0);
  @$pb.TagNumber(10)
  set hasMore($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(10)
  $core.bool hasHasMore() => $_has(0);
  @$pb.TagNumber(10)
  void clearHasMore() => $_clearField(10);

  @$pb.TagNumber(20)
  $core.int get total => $_getIZ(1);
  @$pb.TagNumber(20)
  set total($core.int value) => $_setUnsignedInt32(1, value);
  @$pb.TagNumber(20)
  $core.bool hasTotal() => $_has(1);
  @$pb.TagNumber(20)
  void clearTotal() => $_clearField(20);

  @$pb.TagNumber(30)
  $core.int get offset => $_getIZ(2);
  @$pb.TagNumber(30)
  set offset($core.int value) => $_setUnsignedInt32(2, value);
  @$pb.TagNumber(30)
  $core.bool hasOffset() => $_has(2);
  @$pb.TagNumber(30)
  void clearOffset() => $_clearField(30);

  @$pb.TagNumber(40)
  $core.int get limit => $_getIZ(3);
  @$pb.TagNumber(40)
  set limit($core.int value) => $_setUnsignedInt32(3, value);
  @$pb.TagNumber(40)
  $core.bool hasLimit() => $_has(3);
  @$pb.TagNumber(40)
  void clearLimit() => $_clearField(40);
}

/// user list
class UserList extends $pb.GeneratedMessage {
  factory UserList({
    $core.Iterable<UserEntry>? user,
    PaginationMetadata? pagination,
  }) {
    final result = create();
    if (user != null) result.user.addAll(user);
    if (pagination != null) result.pagination = pagination;
    return result;
  }

  UserList._();

  factory UserList.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory UserList.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UserList',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.users'),
      createEmptyInstance: create)
    ..pPM<UserEntry>(1, _omitFieldNames ? '' : 'user',
        subBuilder: UserEntry.create)
    ..aOM<PaginationMetadata>(2, _omitFieldNames ? '' : 'pagination',
        subBuilder: PaginationMetadata.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UserList clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UserList copyWith(void Function(UserList) updates) =>
      super.copyWith((message) => updates(message as UserList)) as UserList;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static UserList create() => UserList._();
  @$core.override
  UserList createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static UserList getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UserList>(create);
  static UserList? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<UserEntry> get user => $_getList(0);

  @$pb.TagNumber(2)
  PaginationMetadata get pagination => $_getN(1);
  @$pb.TagNumber(2)
  set pagination(PaginationMetadata value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasPagination() => $_has(1);
  @$pb.TagNumber(2)
  void clearPagination() => $_clearField(2);
  @$pb.TagNumber(2)
  PaginationMetadata ensurePagination() => $_ensure(1);
}

/// user entry
class UserEntry extends $pb.GeneratedMessage {
  factory UserEntry({
    $core.String? name,
    $core.List<$core.int>? id,
    $core.List<$core.int>? groupId,
    $core.String? keyBase58,
    Connectivity? connectivity,
    $core.bool? verified,
    $core.bool? blocked,
    $core.Iterable<RoutingTableConnection>? connections,
  }) {
    final result = create();
    if (name != null) result.name = name;
    if (id != null) result.id = id;
    if (groupId != null) result.groupId = groupId;
    if (keyBase58 != null) result.keyBase58 = keyBase58;
    if (connectivity != null) result.connectivity = connectivity;
    if (verified != null) result.verified = verified;
    if (blocked != null) result.blocked = blocked;
    if (connections != null) result.connections.addAll(connections);
    return result;
  }

  UserEntry._();

  factory UserEntry.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory UserEntry.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UserEntry',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.users'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..a<$core.List<$core.int>>(
        2, _omitFieldNames ? '' : 'id', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(
        3, _omitFieldNames ? '' : 'groupId', $pb.PbFieldType.OY)
    ..aOS(7, _omitFieldNames ? '' : 'keyBase58')
    ..aE<Connectivity>(8, _omitFieldNames ? '' : 'connectivity',
        enumValues: Connectivity.values)
    ..aOB(9, _omitFieldNames ? '' : 'verified')
    ..aOB(10, _omitFieldNames ? '' : 'blocked')
    ..pPM<RoutingTableConnection>(11, _omitFieldNames ? '' : 'connections',
        subBuilder: RoutingTableConnection.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UserEntry clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UserEntry copyWith(void Function(UserEntry) updates) =>
      super.copyWith((message) => updates(message as UserEntry)) as UserEntry;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static UserEntry create() => UserEntry._();
  @$core.override
  UserEntry createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static UserEntry getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UserEntry>(create);
  static UserEntry? _defaultInstance;

  /// user name
  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => $_clearField(1);

  /// user ID (38 Byte PeerID)
  @$pb.TagNumber(2)
  $core.List<$core.int> get id => $_getN(1);
  @$pb.TagNumber(2)
  set id($core.List<$core.int> value) => $_setBytes(1, value);
  @$pb.TagNumber(2)
  $core.bool hasId() => $_has(1);
  @$pb.TagNumber(2)
  void clearId() => $_clearField(2);

  /// direct chat group id
  ///
  /// this is a predictable 16 bytes UUID
  @$pb.TagNumber(3)
  $core.List<$core.int> get groupId => $_getN(2);
  @$pb.TagNumber(3)
  set groupId($core.List<$core.int> value) => $_setBytes(2, value);
  @$pb.TagNumber(3)
  $core.bool hasGroupId() => $_has(2);
  @$pb.TagNumber(3)
  void clearGroupId() => $_clearField(3);

  /// base58 string of public key
  @$pb.TagNumber(7)
  $core.String get keyBase58 => $_getSZ(3);
  @$pb.TagNumber(7)
  set keyBase58($core.String value) => $_setString(3, value);
  @$pb.TagNumber(7)
  $core.bool hasKeyBase58() => $_has(3);
  @$pb.TagNumber(7)
  void clearKeyBase58() => $_clearField(7);

  /// reachability of the user: online | reachable | offline
  @$pb.TagNumber(8)
  Connectivity get connectivity => $_getN(4);
  @$pb.TagNumber(8)
  set connectivity(Connectivity value) => $_setField(8, value);
  @$pb.TagNumber(8)
  $core.bool hasConnectivity() => $_has(4);
  @$pb.TagNumber(8)
  void clearConnectivity() => $_clearField(8);

  /// user has been verified
  @$pb.TagNumber(9)
  $core.bool get verified => $_getBF(5);
  @$pb.TagNumber(9)
  set verified($core.bool value) => $_setBool(5, value);
  @$pb.TagNumber(9)
  $core.bool hasVerified() => $_has(5);
  @$pb.TagNumber(9)
  void clearVerified() => $_clearField(9);

  /// user is blocked
  @$pb.TagNumber(10)
  $core.bool get blocked => $_getBF(6);
  @$pb.TagNumber(10)
  set blocked($core.bool value) => $_setBool(6, value);
  @$pb.TagNumber(10)
  $core.bool hasBlocked() => $_has(6);
  @$pb.TagNumber(10)
  void clearBlocked() => $_clearField(10);

  /// routing connection entries
  /// RoutingTableConnection connections = 11;
  @$pb.TagNumber(11)
  $pb.PbList<RoutingTableConnection> get connections => $_getList(7);
}

/// Routing table connection entry.
/// This message contains a connection to a specific user.
class RoutingTableConnection extends $pb.GeneratedMessage {
  factory RoutingTableConnection({
    ConnectionModule? module,
    $core.int? rtt,
    $core.List<$core.int>? via,
    $core.int? hopCount,
  }) {
    final result = create();
    if (module != null) result.module = module;
    if (rtt != null) result.rtt = rtt;
    if (via != null) result.via = via;
    if (hopCount != null) result.hopCount = hopCount;
    return result;
  }

  RoutingTableConnection._();

  factory RoutingTableConnection.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory RoutingTableConnection.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RoutingTableConnection',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.users'),
      createEmptyInstance: create)
    ..aE<ConnectionModule>(2, _omitFieldNames ? '' : 'module',
        enumValues: ConnectionModule.values)
    ..aI(3, _omitFieldNames ? '' : 'rtt', fieldType: $pb.PbFieldType.OU3)
    ..a<$core.List<$core.int>>(
        4, _omitFieldNames ? '' : 'via', $pb.PbFieldType.OY)
    ..aI(5, _omitFieldNames ? '' : 'hopCount', fieldType: $pb.PbFieldType.OU3)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RoutingTableConnection clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RoutingTableConnection copyWith(
          void Function(RoutingTableConnection) updates) =>
      super.copyWith((message) => updates(message as RoutingTableConnection))
          as RoutingTableConnection;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RoutingTableConnection create() => RoutingTableConnection._();
  @$core.override
  RoutingTableConnection createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static RoutingTableConnection getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RoutingTableConnection>(create);
  static RoutingTableConnection? _defaultInstance;

  /// the connection module (LAN, Internet, BLE, etc.)
  @$pb.TagNumber(2)
  ConnectionModule get module => $_getN(0);
  @$pb.TagNumber(2)
  set module(ConnectionModule value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasModule() => $_has(0);
  @$pb.TagNumber(2)
  void clearModule() => $_clearField(2);

  /// the round trip time for this connection
  @$pb.TagNumber(3)
  $core.int get rtt => $_getIZ(1);
  @$pb.TagNumber(3)
  set rtt($core.int value) => $_setUnsignedInt32(1, value);
  @$pb.TagNumber(3)
  $core.bool hasRtt() => $_has(1);
  @$pb.TagNumber(3)
  void clearRtt() => $_clearField(3);

  /// node id via which this connection is routed
  @$pb.TagNumber(4)
  $core.List<$core.int> get via => $_getN(2);
  @$pb.TagNumber(4)
  set via($core.List<$core.int> value) => $_setBytes(2, value);
  @$pb.TagNumber(4)
  $core.bool hasVia() => $_has(2);
  @$pb.TagNumber(4)
  void clearVia() => $_clearField(4);

  /// hop count
  @$pb.TagNumber(5)
  $core.int get hopCount => $_getIZ(3);
  @$pb.TagNumber(5)
  set hopCount($core.int value) => $_setUnsignedInt32(3, value);
  @$pb.TagNumber(5)
  $core.bool hasHopCount() => $_has(3);
  @$pb.TagNumber(5)
  void clearHopCount() => $_clearField(5);
}

/// security number request
class SecurityNumberRequest extends $pb.GeneratedMessage {
  factory SecurityNumberRequest({
    $core.List<$core.int>? userId,
  }) {
    final result = create();
    if (userId != null) result.userId = userId;
    return result;
  }

  SecurityNumberRequest._();

  factory SecurityNumberRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory SecurityNumberRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SecurityNumberRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.users'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'userId', $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SecurityNumberRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SecurityNumberRequest copyWith(
          void Function(SecurityNumberRequest) updates) =>
      super.copyWith((message) => updates(message as SecurityNumberRequest))
          as SecurityNumberRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SecurityNumberRequest create() => SecurityNumberRequest._();
  @$core.override
  SecurityNumberRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static SecurityNumberRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SecurityNumberRequest>(create);
  static SecurityNumberRequest? _defaultInstance;

  /// user id
  @$pb.TagNumber(1)
  $core.List<$core.int> get userId => $_getN(0);
  @$pb.TagNumber(1)
  set userId($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasUserId() => $_has(0);
  @$pb.TagNumber(1)
  void clearUserId() => $_clearField(1);
}

/// security number response
class SecurityNumberResponse extends $pb.GeneratedMessage {
  factory SecurityNumberResponse({
    $core.List<$core.int>? userId,
    $core.List<$core.int>? securityHash,
    $core.Iterable<$core.int>? securityNumberBlocks,
  }) {
    final result = create();
    if (userId != null) result.userId = userId;
    if (securityHash != null) result.securityHash = securityHash;
    if (securityNumberBlocks != null)
      result.securityNumberBlocks.addAll(securityNumberBlocks);
    return result;
  }

  SecurityNumberResponse._();

  factory SecurityNumberResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory SecurityNumberResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SecurityNumberResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.users'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'userId', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(
        2, _omitFieldNames ? '' : 'securityHash', $pb.PbFieldType.OY)
    ..p<$core.int>(
        3, _omitFieldNames ? '' : 'securityNumberBlocks', $pb.PbFieldType.KU3)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SecurityNumberResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SecurityNumberResponse copyWith(
          void Function(SecurityNumberResponse) updates) =>
      super.copyWith((message) => updates(message as SecurityNumberResponse))
          as SecurityNumberResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SecurityNumberResponse create() => SecurityNumberResponse._();
  @$core.override
  SecurityNumberResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static SecurityNumberResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SecurityNumberResponse>(create);
  static SecurityNumberResponse? _defaultInstance;

  /// the user id of the remote user
  @$pb.TagNumber(1)
  $core.List<$core.int> get userId => $_getN(0);
  @$pb.TagNumber(1)
  set userId($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasUserId() => $_has(0);
  @$pb.TagNumber(1)
  void clearUserId() => $_clearField(1);

  /// deliver the full bytes of the hash
  @$pb.TagNumber(2)
  $core.List<$core.int> get securityHash => $_getN(1);
  @$pb.TagNumber(2)
  set securityHash($core.List<$core.int> value) => $_setBytes(1, value);
  @$pb.TagNumber(2)
  $core.bool hasSecurityHash() => $_has(1);
  @$pb.TagNumber(2)
  void clearSecurityHash() => $_clearField(2);

  /// fill in 8 numbers of 16bits
  /// uint16 data type does not exist in protobuf, just fill them in the u16 as
  /// u32.
  @$pb.TagNumber(3)
  $pb.PbList<$core.int> get securityNumberBlocks => $_getList(2);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
