///
//  Generated code. Do not modify.
//  source: from_libqaul.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

export 'from_libqaul.pbenum.dart';

enum FromLibqaul_Module {
  node, 
  router, 
  feed, 
  notSet
}

class FromLibqaul extends $pb.GeneratedMessage {
  static const $core.Map<$core.int, FromLibqaul_Module> _FromLibqaul_ModuleByTag = {
    1 : FromLibqaul_Module.node,
    2 : FromLibqaul_Module.router,
    3 : FromLibqaul_Module.feed,
    0 : FromLibqaul_Module.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'FromLibqaul', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'QaulRpc'), createEmptyInstance: create)
    ..oo(0, [1, 2, 3])
    ..aOM<FromNode>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'node', subBuilder: FromNode.create)
    ..aOM<FromRouter>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'router', subBuilder: FromRouter.create)
    ..aOM<FromFeed>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'feed', subBuilder: FromFeed.create)
    ..hasRequiredFields = false
  ;

  FromLibqaul._() : super();
  factory FromLibqaul({
    FromNode? node,
    FromRouter? router,
    FromFeed? feed,
  }) {
    final _result = create();
    if (node != null) {
      _result.node = node;
    }
    if (router != null) {
      _result.router = router;
    }
    if (feed != null) {
      _result.feed = feed;
    }
    return _result;
  }
  factory FromLibqaul.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FromLibqaul.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  FromLibqaul clone() => FromLibqaul()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  FromLibqaul copyWith(void Function(FromLibqaul) updates) => super.copyWith((message) => updates(message as FromLibqaul)) as FromLibqaul; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static FromLibqaul create() => FromLibqaul._();
  FromLibqaul createEmptyInstance() => create();
  static $pb.PbList<FromLibqaul> createRepeated() => $pb.PbList<FromLibqaul>();
  @$core.pragma('dart2js:noInline')
  static FromLibqaul getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FromLibqaul>(create);
  static FromLibqaul? _defaultInstance;

  FromLibqaul_Module whichModule() => _FromLibqaul_ModuleByTag[$_whichOneof(0)]!;
  void clearModule() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  FromNode get node => $_getN(0);
  @$pb.TagNumber(1)
  set node(FromNode v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasNode() => $_has(0);
  @$pb.TagNumber(1)
  void clearNode() => clearField(1);
  @$pb.TagNumber(1)
  FromNode ensureNode() => $_ensure(0);

  @$pb.TagNumber(2)
  FromRouter get router => $_getN(1);
  @$pb.TagNumber(2)
  set router(FromRouter v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasRouter() => $_has(1);
  @$pb.TagNumber(2)
  void clearRouter() => clearField(2);
  @$pb.TagNumber(2)
  FromRouter ensureRouter() => $_ensure(1);

  @$pb.TagNumber(3)
  FromFeed get feed => $_getN(2);
  @$pb.TagNumber(3)
  set feed(FromFeed v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasFeed() => $_has(2);
  @$pb.TagNumber(3)
  void clearFeed() => clearField(3);
  @$pb.TagNumber(3)
  FromFeed ensureFeed() => $_ensure(2);
}

enum FromNode_Type {
  session, 
  myUser, 
  notSet
}

class FromNode extends $pb.GeneratedMessage {
  static const $core.Map<$core.int, FromNode_Type> _FromNode_TypeByTag = {
    1 : FromNode_Type.session,
    2 : FromNode_Type.myUser,
    0 : FromNode_Type.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'FromNode', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'QaulRpc'), createEmptyInstance: create)
    ..oo(0, [1, 2])
    ..aOM<SessionInformation>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'session', subBuilder: SessionInformation.create)
    ..aOM<MyUser>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'myUser', subBuilder: MyUser.create)
    ..hasRequiredFields = false
  ;

  FromNode._() : super();
  factory FromNode({
    SessionInformation? session,
    MyUser? myUser,
  }) {
    final _result = create();
    if (session != null) {
      _result.session = session;
    }
    if (myUser != null) {
      _result.myUser = myUser;
    }
    return _result;
  }
  factory FromNode.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FromNode.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  FromNode clone() => FromNode()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  FromNode copyWith(void Function(FromNode) updates) => super.copyWith((message) => updates(message as FromNode)) as FromNode; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static FromNode create() => FromNode._();
  FromNode createEmptyInstance() => create();
  static $pb.PbList<FromNode> createRepeated() => $pb.PbList<FromNode>();
  @$core.pragma('dart2js:noInline')
  static FromNode getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FromNode>(create);
  static FromNode? _defaultInstance;

  FromNode_Type whichType() => _FromNode_TypeByTag[$_whichOneof(0)]!;
  void clearType() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  SessionInformation get session => $_getN(0);
  @$pb.TagNumber(1)
  set session(SessionInformation v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasSession() => $_has(0);
  @$pb.TagNumber(1)
  void clearSession() => clearField(1);
  @$pb.TagNumber(1)
  SessionInformation ensureSession() => $_ensure(0);

  @$pb.TagNumber(2)
  MyUser get myUser => $_getN(1);
  @$pb.TagNumber(2)
  set myUser(MyUser v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasMyUser() => $_has(1);
  @$pb.TagNumber(2)
  void clearMyUser() => clearField(2);
  @$pb.TagNumber(2)
  MyUser ensureMyUser() => $_ensure(1);
}

class SessionInformation extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'SessionInformation', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'QaulRpc'), createEmptyInstance: create)
    ..aOB(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'userExists')
    ..aOM<MyUser>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'myUser', subBuilder: MyUser.create)
    ..hasRequiredFields = false
  ;

  SessionInformation._() : super();
  factory SessionInformation({
    $core.bool? userExists,
    MyUser? myUser,
  }) {
    final _result = create();
    if (userExists != null) {
      _result.userExists = userExists;
    }
    if (myUser != null) {
      _result.myUser = myUser;
    }
    return _result;
  }
  factory SessionInformation.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory SessionInformation.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  SessionInformation clone() => SessionInformation()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  SessionInformation copyWith(void Function(SessionInformation) updates) => super.copyWith((message) => updates(message as SessionInformation)) as SessionInformation; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static SessionInformation create() => SessionInformation._();
  SessionInformation createEmptyInstance() => create();
  static $pb.PbList<SessionInformation> createRepeated() => $pb.PbList<SessionInformation>();
  @$core.pragma('dart2js:noInline')
  static SessionInformation getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SessionInformation>(create);
  static SessionInformation? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get userExists => $_getBF(0);
  @$pb.TagNumber(1)
  set userExists($core.bool v) { $_setBool(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasUserExists() => $_has(0);
  @$pb.TagNumber(1)
  void clearUserExists() => clearField(1);

  @$pb.TagNumber(2)
  MyUser get myUser => $_getN(1);
  @$pb.TagNumber(2)
  set myUser(MyUser v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasMyUser() => $_has(1);
  @$pb.TagNumber(2)
  void clearMyUser() => clearField(2);
  @$pb.TagNumber(2)
  MyUser ensureMyUser() => $_ensure(1);
}

class MyUser extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'MyUser', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'QaulRpc'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'name')
    ..a<$core.List<$core.int>>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'id', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  MyUser._() : super();
  factory MyUser({
    $core.String? name,
    $core.List<$core.int>? id,
  }) {
    final _result = create();
    if (name != null) {
      _result.name = name;
    }
    if (id != null) {
      _result.id = id;
    }
    return _result;
  }
  factory MyUser.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory MyUser.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  MyUser clone() => MyUser()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  MyUser copyWith(void Function(MyUser) updates) => super.copyWith((message) => updates(message as MyUser)) as MyUser; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static MyUser create() => MyUser._();
  MyUser createEmptyInstance() => create();
  static $pb.PbList<MyUser> createRepeated() => $pb.PbList<MyUser>();
  @$core.pragma('dart2js:noInline')
  static MyUser getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<MyUser>(create);
  static MyUser? _defaultInstance;

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
}

enum FromRouter_Type {
  userList, 
  notSet
}

class FromRouter extends $pb.GeneratedMessage {
  static const $core.Map<$core.int, FromRouter_Type> _FromRouter_TypeByTag = {
    1 : FromRouter_Type.userList,
    0 : FromRouter_Type.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'FromRouter', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'QaulRpc'), createEmptyInstance: create)
    ..oo(0, [1])
    ..aOM<UserList>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'userList', subBuilder: UserList.create)
    ..hasRequiredFields = false
  ;

  FromRouter._() : super();
  factory FromRouter({
    UserList? userList,
  }) {
    final _result = create();
    if (userList != null) {
      _result.userList = userList;
    }
    return _result;
  }
  factory FromRouter.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FromRouter.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  FromRouter clone() => FromRouter()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  FromRouter copyWith(void Function(FromRouter) updates) => super.copyWith((message) => updates(message as FromRouter)) as FromRouter; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static FromRouter create() => FromRouter._();
  FromRouter createEmptyInstance() => create();
  static $pb.PbList<FromRouter> createRepeated() => $pb.PbList<FromRouter>();
  @$core.pragma('dart2js:noInline')
  static FromRouter getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FromRouter>(create);
  static FromRouter? _defaultInstance;

  FromRouter_Type whichType() => _FromRouter_TypeByTag[$_whichOneof(0)]!;
  void clearType() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  UserList get userList => $_getN(0);
  @$pb.TagNumber(1)
  set userList(UserList v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasUserList() => $_has(0);
  @$pb.TagNumber(1)
  void clearUserList() => clearField(1);
  @$pb.TagNumber(1)
  UserList ensureUserList() => $_ensure(0);
}

class UserList extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'UserList', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'QaulRpc'), createEmptyInstance: create)
    ..pc<UserEntry>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'user', $pb.PbFieldType.PM, subBuilder: UserEntry.create)
    ..hasRequiredFields = false
  ;

  UserList._() : super();
  factory UserList({
    $core.Iterable<UserEntry>? user,
  }) {
    final _result = create();
    if (user != null) {
      _result.user.addAll(user);
    }
    return _result;
  }
  factory UserList.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory UserList.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  UserList clone() => UserList()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  UserList copyWith(void Function(UserList) updates) => super.copyWith((message) => updates(message as UserList)) as UserList; // ignore: deprecated_member_use
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
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'UserEntry', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'QaulRpc'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'name')
    ..a<$core.List<$core.int>>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'id', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'key', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  UserEntry._() : super();
  factory UserEntry({
    $core.String? name,
    $core.List<$core.int>? id,
    $core.List<$core.int>? key,
  }) {
    final _result = create();
    if (name != null) {
      _result.name = name;
    }
    if (id != null) {
      _result.id = id;
    }
    if (key != null) {
      _result.key = key;
    }
    return _result;
  }
  factory UserEntry.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory UserEntry.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  UserEntry clone() => UserEntry()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  UserEntry copyWith(void Function(UserEntry) updates) => super.copyWith((message) => updates(message as UserEntry)) as UserEntry; // ignore: deprecated_member_use
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
  $core.List<$core.int> get key => $_getN(2);
  @$pb.TagNumber(3)
  set key($core.List<$core.int> v) { $_setBytes(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasKey() => $_has(2);
  @$pb.TagNumber(3)
  void clearKey() => clearField(3);
}

enum FromFeed_Type {
  message, 
  notSet
}

class FromFeed extends $pb.GeneratedMessage {
  static const $core.Map<$core.int, FromFeed_Type> _FromFeed_TypeByTag = {
    1 : FromFeed_Type.message,
    0 : FromFeed_Type.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'FromFeed', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'QaulRpc'), createEmptyInstance: create)
    ..oo(0, [1])
    ..aOM<FeedMessage>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'message', subBuilder: FeedMessage.create)
    ..hasRequiredFields = false
  ;

  FromFeed._() : super();
  factory FromFeed({
    FeedMessage? message,
  }) {
    final _result = create();
    if (message != null) {
      _result.message = message;
    }
    return _result;
  }
  factory FromFeed.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FromFeed.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  FromFeed clone() => FromFeed()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  FromFeed copyWith(void Function(FromFeed) updates) => super.copyWith((message) => updates(message as FromFeed)) as FromFeed; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static FromFeed create() => FromFeed._();
  FromFeed createEmptyInstance() => create();
  static $pb.PbList<FromFeed> createRepeated() => $pb.PbList<FromFeed>();
  @$core.pragma('dart2js:noInline')
  static FromFeed getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FromFeed>(create);
  static FromFeed? _defaultInstance;

  FromFeed_Type whichType() => _FromFeed_TypeByTag[$_whichOneof(0)]!;
  void clearType() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  FeedMessage get message => $_getN(0);
  @$pb.TagNumber(1)
  set message(FeedMessage v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasMessage() => $_has(0);
  @$pb.TagNumber(1)
  void clearMessage() => clearField(1);
  @$pb.TagNumber(1)
  FeedMessage ensureMessage() => $_ensure(0);
}

class FeedMessage extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'FeedMessage', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'QaulRpc'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'senderId', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'messageId', $pb.PbFieldType.OY)
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'timeSent')
    ..aOS(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'timeReceived')
    ..aOS(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'content')
    ..hasRequiredFields = false
  ;

  FeedMessage._() : super();
  factory FeedMessage({
    $core.List<$core.int>? senderId,
    $core.List<$core.int>? messageId,
    $core.String? timeSent,
    $core.String? timeReceived,
    $core.String? content,
  }) {
    final _result = create();
    if (senderId != null) {
      _result.senderId = senderId;
    }
    if (messageId != null) {
      _result.messageId = messageId;
    }
    if (timeSent != null) {
      _result.timeSent = timeSent;
    }
    if (timeReceived != null) {
      _result.timeReceived = timeReceived;
    }
    if (content != null) {
      _result.content = content;
    }
    return _result;
  }
  factory FeedMessage.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FeedMessage.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  FeedMessage clone() => FeedMessage()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  FeedMessage copyWith(void Function(FeedMessage) updates) => super.copyWith((message) => updates(message as FeedMessage)) as FeedMessage; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static FeedMessage create() => FeedMessage._();
  FeedMessage createEmptyInstance() => create();
  static $pb.PbList<FeedMessage> createRepeated() => $pb.PbList<FeedMessage>();
  @$core.pragma('dart2js:noInline')
  static FeedMessage getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FeedMessage>(create);
  static FeedMessage? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get senderId => $_getN(0);
  @$pb.TagNumber(1)
  set senderId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasSenderId() => $_has(0);
  @$pb.TagNumber(1)
  void clearSenderId() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get messageId => $_getN(1);
  @$pb.TagNumber(2)
  set messageId($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasMessageId() => $_has(1);
  @$pb.TagNumber(2)
  void clearMessageId() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get timeSent => $_getSZ(2);
  @$pb.TagNumber(3)
  set timeSent($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasTimeSent() => $_has(2);
  @$pb.TagNumber(3)
  void clearTimeSent() => clearField(3);

  @$pb.TagNumber(4)
  $core.String get timeReceived => $_getSZ(3);
  @$pb.TagNumber(4)
  set timeReceived($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasTimeReceived() => $_has(3);
  @$pb.TagNumber(4)
  void clearTimeReceived() => clearField(4);

  @$pb.TagNumber(5)
  $core.String get content => $_getSZ(4);
  @$pb.TagNumber(5)
  set content($core.String v) { $_setString(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasContent() => $_has(4);
  @$pb.TagNumber(5)
  void clearContent() => clearField(5);
}

