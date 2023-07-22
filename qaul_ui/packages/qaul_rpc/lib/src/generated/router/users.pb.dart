//
//  Generated code. Do not modify.
//  source: router/users.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'users.pbenum.dart';

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

class Users extends $pb.GeneratedMessage {
  factory Users() => create();
  Users._() : super();
  factory Users.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Users.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static const $core.Map<$core.int, Users_Message> _Users_MessageByTag = {
    1 : Users_Message.userRequest,
    2 : Users_Message.userOnlineRequest,
    3 : Users_Message.userList,
    4 : Users_Message.userUpdate,
    5 : Users_Message.securityNumberRequest,
    6 : Users_Message.securityNumberResponse,
    0 : Users_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Users', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.users'), createEmptyInstance: create)
    ..oo(0, [1, 2, 3, 4, 5, 6])
    ..aOM<UserRequest>(1, _omitFieldNames ? '' : 'userRequest', subBuilder: UserRequest.create)
    ..aOM<UserOnlineRequest>(2, _omitFieldNames ? '' : 'userOnlineRequest', subBuilder: UserOnlineRequest.create)
    ..aOM<UserList>(3, _omitFieldNames ? '' : 'userList', subBuilder: UserList.create)
    ..aOM<UserEntry>(4, _omitFieldNames ? '' : 'userUpdate', subBuilder: UserEntry.create)
    ..aOM<SecurityNumberRequest>(5, _omitFieldNames ? '' : 'securityNumberRequest', subBuilder: SecurityNumberRequest.create)
    ..aOM<SecurityNumberResponse>(6, _omitFieldNames ? '' : 'securityNumberResponse', subBuilder: SecurityNumberResponse.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Users clone() => Users()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Users copyWith(void Function(Users) updates) => super.copyWith((message) => updates(message as Users)) as Users;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Users create() => Users._();
  Users createEmptyInstance() => create();
  static $pb.PbList<Users> createRepeated() => $pb.PbList<Users>();
  @$core.pragma('dart2js:noInline')
  static Users getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Users>(create);
  static Users? _defaultInstance;

  Users_Message whichMessage() => _Users_MessageByTag[$_whichOneof(0)]!;
  void clearMessage() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  UserRequest get userRequest => $_getN(0);
  @$pb.TagNumber(1)
  set userRequest(UserRequest v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasUserRequest() => $_has(0);
  @$pb.TagNumber(1)
  void clearUserRequest() => clearField(1);
  @$pb.TagNumber(1)
  UserRequest ensureUserRequest() => $_ensure(0);

  @$pb.TagNumber(2)
  UserOnlineRequest get userOnlineRequest => $_getN(1);
  @$pb.TagNumber(2)
  set userOnlineRequest(UserOnlineRequest v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasUserOnlineRequest() => $_has(1);
  @$pb.TagNumber(2)
  void clearUserOnlineRequest() => clearField(2);
  @$pb.TagNumber(2)
  UserOnlineRequest ensureUserOnlineRequest() => $_ensure(1);

  @$pb.TagNumber(3)
  UserList get userList => $_getN(2);
  @$pb.TagNumber(3)
  set userList(UserList v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasUserList() => $_has(2);
  @$pb.TagNumber(3)
  void clearUserList() => clearField(3);
  @$pb.TagNumber(3)
  UserList ensureUserList() => $_ensure(2);

  @$pb.TagNumber(4)
  UserEntry get userUpdate => $_getN(3);
  @$pb.TagNumber(4)
  set userUpdate(UserEntry v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasUserUpdate() => $_has(3);
  @$pb.TagNumber(4)
  void clearUserUpdate() => clearField(4);
  @$pb.TagNumber(4)
  UserEntry ensureUserUpdate() => $_ensure(3);

  @$pb.TagNumber(5)
  SecurityNumberRequest get securityNumberRequest => $_getN(4);
  @$pb.TagNumber(5)
  set securityNumberRequest(SecurityNumberRequest v) { setField(5, v); }
  @$pb.TagNumber(5)
  $core.bool hasSecurityNumberRequest() => $_has(4);
  @$pb.TagNumber(5)
  void clearSecurityNumberRequest() => clearField(5);
  @$pb.TagNumber(5)
  SecurityNumberRequest ensureSecurityNumberRequest() => $_ensure(4);

  @$pb.TagNumber(6)
  SecurityNumberResponse get securityNumberResponse => $_getN(5);
  @$pb.TagNumber(6)
  set securityNumberResponse(SecurityNumberResponse v) { setField(6, v); }
  @$pb.TagNumber(6)
  $core.bool hasSecurityNumberResponse() => $_has(5);
  @$pb.TagNumber(6)
  void clearSecurityNumberResponse() => clearField(6);
  @$pb.TagNumber(6)
  SecurityNumberResponse ensureSecurityNumberResponse() => $_ensure(5);
}

class UserRequest extends $pb.GeneratedMessage {
  factory UserRequest() => create();
  UserRequest._() : super();
  factory UserRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory UserRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'UserRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.users'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  UserRequest clone() => UserRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  UserRequest copyWith(void Function(UserRequest) updates) => super.copyWith((message) => updates(message as UserRequest)) as UserRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static UserRequest create() => UserRequest._();
  UserRequest createEmptyInstance() => create();
  static $pb.PbList<UserRequest> createRepeated() => $pb.PbList<UserRequest>();
  @$core.pragma('dart2js:noInline')
  static UserRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UserRequest>(create);
  static UserRequest? _defaultInstance;
}

class UserOnlineRequest extends $pb.GeneratedMessage {
  factory UserOnlineRequest() => create();
  UserOnlineRequest._() : super();
  factory UserOnlineRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory UserOnlineRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'UserOnlineRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.users'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  UserOnlineRequest clone() => UserOnlineRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  UserOnlineRequest copyWith(void Function(UserOnlineRequest) updates) => super.copyWith((message) => updates(message as UserOnlineRequest)) as UserOnlineRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static UserOnlineRequest create() => UserOnlineRequest._();
  UserOnlineRequest createEmptyInstance() => create();
  static $pb.PbList<UserOnlineRequest> createRepeated() => $pb.PbList<UserOnlineRequest>();
  @$core.pragma('dart2js:noInline')
  static UserOnlineRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UserOnlineRequest>(create);
  static UserOnlineRequest? _defaultInstance;
}

class UserList extends $pb.GeneratedMessage {
  factory UserList() => create();
  UserList._() : super();
  factory UserList.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory UserList.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'UserList', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.users'), createEmptyInstance: create)
    ..pc<UserEntry>(1, _omitFieldNames ? '' : 'user', $pb.PbFieldType.PM, subBuilder: UserEntry.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  UserList clone() => UserList()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  UserList copyWith(void Function(UserList) updates) => super.copyWith((message) => updates(message as UserList)) as UserList;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static UserList create() => UserList._();
  UserList createEmptyInstance() => create();
  static $pb.PbList<UserList> createRepeated() => $pb.PbList<UserList>();
  @$core.pragma('dart2js:noInline')
  static UserList getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UserList>(create);
  static UserList? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<UserEntry> get user => $_getList(0);
}

class UserEntry extends $pb.GeneratedMessage {
  factory UserEntry() => create();
  UserEntry._() : super();
  factory UserEntry.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory UserEntry.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'UserEntry', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.users'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..a<$core.List<$core.int>>(2, _omitFieldNames ? '' : 'id', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(3, _omitFieldNames ? '' : 'groupId', $pb.PbFieldType.OY)
    ..aOS(7, _omitFieldNames ? '' : 'keyBase58')
    ..e<Connectivity>(8, _omitFieldNames ? '' : 'connectivity', $pb.PbFieldType.OE, defaultOrMaker: Connectivity.Online, valueOf: Connectivity.valueOf, enumValues: Connectivity.values)
    ..aOB(9, _omitFieldNames ? '' : 'verified')
    ..aOB(10, _omitFieldNames ? '' : 'blocked')
    ..pc<RoutingTableConnection>(11, _omitFieldNames ? '' : 'connections', $pb.PbFieldType.PM, subBuilder: RoutingTableConnection.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  UserEntry clone() => UserEntry()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  UserEntry copyWith(void Function(UserEntry) updates) => super.copyWith((message) => updates(message as UserEntry)) as UserEntry;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static UserEntry create() => UserEntry._();
  UserEntry createEmptyInstance() => create();
  static $pb.PbList<UserEntry> createRepeated() => $pb.PbList<UserEntry>();
  @$core.pragma('dart2js:noInline')
  static UserEntry getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UserEntry>(create);
  static UserEntry? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get id => $_getN(1);
  @$pb.TagNumber(2)
  set id($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasId() => $_has(1);
  @$pb.TagNumber(2)
  void clearId() => clearField(2);

  @$pb.TagNumber(3)
  $core.List<$core.int> get groupId => $_getN(2);
  @$pb.TagNumber(3)
  set groupId($core.List<$core.int> v) { $_setBytes(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasGroupId() => $_has(2);
  @$pb.TagNumber(3)
  void clearGroupId() => clearField(3);

  @$pb.TagNumber(7)
  $core.String get keyBase58 => $_getSZ(3);
  @$pb.TagNumber(7)
  set keyBase58($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(7)
  $core.bool hasKeyBase58() => $_has(3);
  @$pb.TagNumber(7)
  void clearKeyBase58() => clearField(7);

  @$pb.TagNumber(8)
  Connectivity get connectivity => $_getN(4);
  @$pb.TagNumber(8)
  set connectivity(Connectivity v) { setField(8, v); }
  @$pb.TagNumber(8)
  $core.bool hasConnectivity() => $_has(4);
  @$pb.TagNumber(8)
  void clearConnectivity() => clearField(8);

  @$pb.TagNumber(9)
  $core.bool get verified => $_getBF(5);
  @$pb.TagNumber(9)
  set verified($core.bool v) { $_setBool(5, v); }
  @$pb.TagNumber(9)
  $core.bool hasVerified() => $_has(5);
  @$pb.TagNumber(9)
  void clearVerified() => clearField(9);

  @$pb.TagNumber(10)
  $core.bool get blocked => $_getBF(6);
  @$pb.TagNumber(10)
  set blocked($core.bool v) { $_setBool(6, v); }
  @$pb.TagNumber(10)
  $core.bool hasBlocked() => $_has(6);
  @$pb.TagNumber(10)
  void clearBlocked() => clearField(10);

  @$pb.TagNumber(11)
  $core.List<RoutingTableConnection> get connections => $_getList(7);
}

class RoutingTableConnection extends $pb.GeneratedMessage {
  factory RoutingTableConnection() => create();
  RoutingTableConnection._() : super();
  factory RoutingTableConnection.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RoutingTableConnection.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'RoutingTableConnection', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.users'), createEmptyInstance: create)
    ..e<ConnectionModule>(2, _omitFieldNames ? '' : 'module', $pb.PbFieldType.OE, defaultOrMaker: ConnectionModule.NONE, valueOf: ConnectionModule.valueOf, enumValues: ConnectionModule.values)
    ..a<$core.int>(3, _omitFieldNames ? '' : 'rtt', $pb.PbFieldType.OU3)
    ..a<$core.List<$core.int>>(4, _omitFieldNames ? '' : 'via', $pb.PbFieldType.OY)
    ..a<$core.int>(5, _omitFieldNames ? '' : 'hopCount', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RoutingTableConnection clone() => RoutingTableConnection()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RoutingTableConnection copyWith(void Function(RoutingTableConnection) updates) => super.copyWith((message) => updates(message as RoutingTableConnection)) as RoutingTableConnection;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RoutingTableConnection create() => RoutingTableConnection._();
  RoutingTableConnection createEmptyInstance() => create();
  static $pb.PbList<RoutingTableConnection> createRepeated() => $pb.PbList<RoutingTableConnection>();
  @$core.pragma('dart2js:noInline')
  static RoutingTableConnection getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RoutingTableConnection>(create);
  static RoutingTableConnection? _defaultInstance;

  @$pb.TagNumber(2)
  ConnectionModule get module => $_getN(0);
  @$pb.TagNumber(2)
  set module(ConnectionModule v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasModule() => $_has(0);
  @$pb.TagNumber(2)
  void clearModule() => clearField(2);

  @$pb.TagNumber(3)
  $core.int get rtt => $_getIZ(1);
  @$pb.TagNumber(3)
  set rtt($core.int v) { $_setUnsignedInt32(1, v); }
  @$pb.TagNumber(3)
  $core.bool hasRtt() => $_has(1);
  @$pb.TagNumber(3)
  void clearRtt() => clearField(3);

  @$pb.TagNumber(4)
  $core.List<$core.int> get via => $_getN(2);
  @$pb.TagNumber(4)
  set via($core.List<$core.int> v) { $_setBytes(2, v); }
  @$pb.TagNumber(4)
  $core.bool hasVia() => $_has(2);
  @$pb.TagNumber(4)
  void clearVia() => clearField(4);

  @$pb.TagNumber(5)
  $core.int get hopCount => $_getIZ(3);
  @$pb.TagNumber(5)
  set hopCount($core.int v) { $_setUnsignedInt32(3, v); }
  @$pb.TagNumber(5)
  $core.bool hasHopCount() => $_has(3);
  @$pb.TagNumber(5)
  void clearHopCount() => clearField(5);
}

class SecurityNumberRequest extends $pb.GeneratedMessage {
  factory SecurityNumberRequest() => create();
  SecurityNumberRequest._() : super();
  factory SecurityNumberRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory SecurityNumberRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'SecurityNumberRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.users'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'userId', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  SecurityNumberRequest clone() => SecurityNumberRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  SecurityNumberRequest copyWith(void Function(SecurityNumberRequest) updates) => super.copyWith((message) => updates(message as SecurityNumberRequest)) as SecurityNumberRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SecurityNumberRequest create() => SecurityNumberRequest._();
  SecurityNumberRequest createEmptyInstance() => create();
  static $pb.PbList<SecurityNumberRequest> createRepeated() => $pb.PbList<SecurityNumberRequest>();
  @$core.pragma('dart2js:noInline')
  static SecurityNumberRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SecurityNumberRequest>(create);
  static SecurityNumberRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get userId => $_getN(0);
  @$pb.TagNumber(1)
  set userId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasUserId() => $_has(0);
  @$pb.TagNumber(1)
  void clearUserId() => clearField(1);
}

class SecurityNumberResponse extends $pb.GeneratedMessage {
  factory SecurityNumberResponse() => create();
  SecurityNumberResponse._() : super();
  factory SecurityNumberResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory SecurityNumberResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'SecurityNumberResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.users'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'userId', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(2, _omitFieldNames ? '' : 'securityHash', $pb.PbFieldType.OY)
    ..p<$core.int>(3, _omitFieldNames ? '' : 'securityNumberBlocks', $pb.PbFieldType.KU3)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  SecurityNumberResponse clone() => SecurityNumberResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  SecurityNumberResponse copyWith(void Function(SecurityNumberResponse) updates) => super.copyWith((message) => updates(message as SecurityNumberResponse)) as SecurityNumberResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SecurityNumberResponse create() => SecurityNumberResponse._();
  SecurityNumberResponse createEmptyInstance() => create();
  static $pb.PbList<SecurityNumberResponse> createRepeated() => $pb.PbList<SecurityNumberResponse>();
  @$core.pragma('dart2js:noInline')
  static SecurityNumberResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SecurityNumberResponse>(create);
  static SecurityNumberResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get userId => $_getN(0);
  @$pb.TagNumber(1)
  set userId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasUserId() => $_has(0);
  @$pb.TagNumber(1)
  void clearUserId() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get securityHash => $_getN(1);
  @$pb.TagNumber(2)
  set securityHash($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasSecurityHash() => $_has(1);
  @$pb.TagNumber(2)
  void clearSecurityHash() => clearField(2);

  @$pb.TagNumber(3)
  $core.List<$core.int> get securityNumberBlocks => $_getList(2);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
