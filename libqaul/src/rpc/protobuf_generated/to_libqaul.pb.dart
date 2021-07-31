///
//  Generated code. Do not modify.
//  source: to_libqaul.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

enum ToLibqaul_Module {
  node, 
  router, 
  feed, 
  notSet
}

class ToLibqaul extends $pb.GeneratedMessage {
  static const $core.Map<$core.int, ToLibqaul_Module> _ToLibqaul_ModuleByTag = {
    1 : ToLibqaul_Module.node,
    2 : ToLibqaul_Module.router,
    3 : ToLibqaul_Module.feed,
    0 : ToLibqaul_Module.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ToLibqaul', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'QaulRpc'), createEmptyInstance: create)
    ..oo(0, [1, 2, 3])
    ..aOM<ToNode>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'node', subBuilder: ToNode.create)
    ..aOM<ToRouter>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'router', subBuilder: ToRouter.create)
    ..aOM<ToFeed>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'feed', subBuilder: ToFeed.create)
    ..hasRequiredFields = false
  ;

  ToLibqaul._() : super();
  factory ToLibqaul({
    ToNode? node,
    ToRouter? router,
    ToFeed? feed,
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
  factory ToLibqaul.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ToLibqaul.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ToLibqaul clone() => ToLibqaul()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ToLibqaul copyWith(void Function(ToLibqaul) updates) => super.copyWith((message) => updates(message as ToLibqaul)) as ToLibqaul; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ToLibqaul create() => ToLibqaul._();
  ToLibqaul createEmptyInstance() => create();
  static $pb.PbList<ToLibqaul> createRepeated() => $pb.PbList<ToLibqaul>();
  @$core.pragma('dart2js:noInline')
  static ToLibqaul getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ToLibqaul>(create);
  static ToLibqaul? _defaultInstance;

  ToLibqaul_Module whichModule() => _ToLibqaul_ModuleByTag[$_whichOneof(0)]!;
  void clearModule() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  ToNode get node => $_getN(0);
  @$pb.TagNumber(1)
  set node(ToNode v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasNode() => $_has(0);
  @$pb.TagNumber(1)
  void clearNode() => clearField(1);
  @$pb.TagNumber(1)
  ToNode ensureNode() => $_ensure(0);

  @$pb.TagNumber(2)
  ToRouter get router => $_getN(1);
  @$pb.TagNumber(2)
  set router(ToRouter v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasRouter() => $_has(1);
  @$pb.TagNumber(2)
  void clearRouter() => clearField(2);
  @$pb.TagNumber(2)
  ToRouter ensureRouter() => $_ensure(1);

  @$pb.TagNumber(3)
  ToFeed get feed => $_getN(2);
  @$pb.TagNumber(3)
  set feed(ToFeed v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasFeed() => $_has(2);
  @$pb.TagNumber(3)
  void clearFeed() => clearField(3);
  @$pb.TagNumber(3)
  ToFeed ensureFeed() => $_ensure(2);
}

enum ToNode_Type {
  startSession, 
  createUser, 
  notSet
}

class ToNode extends $pb.GeneratedMessage {
  static const $core.Map<$core.int, ToNode_Type> _ToNode_TypeByTag = {
    1 : ToNode_Type.startSession,
    2 : ToNode_Type.createUser,
    0 : ToNode_Type.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ToNode', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'QaulRpc'), createEmptyInstance: create)
    ..oo(0, [1, 2])
    ..aOB(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'startSession')
    ..aOM<CreateUser>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'createUser', subBuilder: CreateUser.create)
    ..hasRequiredFields = false
  ;

  ToNode._() : super();
  factory ToNode({
    $core.bool? startSession,
    CreateUser? createUser,
  }) {
    final _result = create();
    if (startSession != null) {
      _result.startSession = startSession;
    }
    if (createUser != null) {
      _result.createUser = createUser;
    }
    return _result;
  }
  factory ToNode.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ToNode.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ToNode clone() => ToNode()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ToNode copyWith(void Function(ToNode) updates) => super.copyWith((message) => updates(message as ToNode)) as ToNode; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ToNode create() => ToNode._();
  ToNode createEmptyInstance() => create();
  static $pb.PbList<ToNode> createRepeated() => $pb.PbList<ToNode>();
  @$core.pragma('dart2js:noInline')
  static ToNode getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ToNode>(create);
  static ToNode? _defaultInstance;

  ToNode_Type whichType() => _ToNode_TypeByTag[$_whichOneof(0)]!;
  void clearType() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  $core.bool get startSession => $_getBF(0);
  @$pb.TagNumber(1)
  set startSession($core.bool v) { $_setBool(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasStartSession() => $_has(0);
  @$pb.TagNumber(1)
  void clearStartSession() => clearField(1);

  @$pb.TagNumber(2)
  CreateUser get createUser => $_getN(1);
  @$pb.TagNumber(2)
  set createUser(CreateUser v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasCreateUser() => $_has(1);
  @$pb.TagNumber(2)
  void clearCreateUser() => clearField(2);
  @$pb.TagNumber(2)
  CreateUser ensureCreateUser() => $_ensure(1);
}

class CreateUser extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'CreateUser', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'QaulRpc'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'name')
    ..hasRequiredFields = false
  ;

  CreateUser._() : super();
  factory CreateUser({
    $core.String? name,
  }) {
    final _result = create();
    if (name != null) {
      _result.name = name;
    }
    return _result;
  }
  factory CreateUser.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory CreateUser.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  CreateUser clone() => CreateUser()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  CreateUser copyWith(void Function(CreateUser) updates) => super.copyWith((message) => updates(message as CreateUser)) as CreateUser; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static CreateUser create() => CreateUser._();
  CreateUser createEmptyInstance() => create();
  static $pb.PbList<CreateUser> createRepeated() => $pb.PbList<CreateUser>();
  @$core.pragma('dart2js:noInline')
  static CreateUser getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<CreateUser>(create);
  static CreateUser? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => clearField(1);
}

enum ToRouter_Type {
  requestUsers, 
  notSet
}

class ToRouter extends $pb.GeneratedMessage {
  static const $core.Map<$core.int, ToRouter_Type> _ToRouter_TypeByTag = {
    1 : ToRouter_Type.requestUsers,
    0 : ToRouter_Type.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ToRouter', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'QaulRpc'), createEmptyInstance: create)
    ..oo(0, [1])
    ..aOM<RequestUsers>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'requestUsers', subBuilder: RequestUsers.create)
    ..hasRequiredFields = false
  ;

  ToRouter._() : super();
  factory ToRouter({
    RequestUsers? requestUsers,
  }) {
    final _result = create();
    if (requestUsers != null) {
      _result.requestUsers = requestUsers;
    }
    return _result;
  }
  factory ToRouter.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ToRouter.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ToRouter clone() => ToRouter()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ToRouter copyWith(void Function(ToRouter) updates) => super.copyWith((message) => updates(message as ToRouter)) as ToRouter; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ToRouter create() => ToRouter._();
  ToRouter createEmptyInstance() => create();
  static $pb.PbList<ToRouter> createRepeated() => $pb.PbList<ToRouter>();
  @$core.pragma('dart2js:noInline')
  static ToRouter getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ToRouter>(create);
  static ToRouter? _defaultInstance;

  ToRouter_Type whichType() => _ToRouter_TypeByTag[$_whichOneof(0)]!;
  void clearType() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  RequestUsers get requestUsers => $_getN(0);
  @$pb.TagNumber(1)
  set requestUsers(RequestUsers v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasRequestUsers() => $_has(0);
  @$pb.TagNumber(1)
  void clearRequestUsers() => clearField(1);
  @$pb.TagNumber(1)
  RequestUsers ensureRequestUsers() => $_ensure(0);
}

class RequestUsers extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'RequestUsers', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'QaulRpc'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  RequestUsers._() : super();
  factory RequestUsers() => create();
  factory RequestUsers.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RequestUsers.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RequestUsers clone() => RequestUsers()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RequestUsers copyWith(void Function(RequestUsers) updates) => super.copyWith((message) => updates(message as RequestUsers)) as RequestUsers; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static RequestUsers create() => RequestUsers._();
  RequestUsers createEmptyInstance() => create();
  static $pb.PbList<RequestUsers> createRepeated() => $pb.PbList<RequestUsers>();
  @$core.pragma('dart2js:noInline')
  static RequestUsers getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RequestUsers>(create);
  static RequestUsers? _defaultInstance;
}

enum ToFeed_Type {
  sendFeed, 
  notSet
}

class ToFeed extends $pb.GeneratedMessage {
  static const $core.Map<$core.int, ToFeed_Type> _ToFeed_TypeByTag = {
    1 : ToFeed_Type.sendFeed,
    0 : ToFeed_Type.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ToFeed', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'QaulRpc'), createEmptyInstance: create)
    ..oo(0, [1])
    ..aOM<SendFeed>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'sendFeed', subBuilder: SendFeed.create)
    ..hasRequiredFields = false
  ;

  ToFeed._() : super();
  factory ToFeed({
    SendFeed? sendFeed,
  }) {
    final _result = create();
    if (sendFeed != null) {
      _result.sendFeed = sendFeed;
    }
    return _result;
  }
  factory ToFeed.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ToFeed.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ToFeed clone() => ToFeed()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ToFeed copyWith(void Function(ToFeed) updates) => super.copyWith((message) => updates(message as ToFeed)) as ToFeed; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ToFeed create() => ToFeed._();
  ToFeed createEmptyInstance() => create();
  static $pb.PbList<ToFeed> createRepeated() => $pb.PbList<ToFeed>();
  @$core.pragma('dart2js:noInline')
  static ToFeed getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ToFeed>(create);
  static ToFeed? _defaultInstance;

  ToFeed_Type whichType() => _ToFeed_TypeByTag[$_whichOneof(0)]!;
  void clearType() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  SendFeed get sendFeed => $_getN(0);
  @$pb.TagNumber(1)
  set sendFeed(SendFeed v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasSendFeed() => $_has(0);
  @$pb.TagNumber(1)
  void clearSendFeed() => clearField(1);
  @$pb.TagNumber(1)
  SendFeed ensureSendFeed() => $_ensure(0);
}

class SendFeed extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'SendFeed', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'QaulRpc'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'content')
    ..hasRequiredFields = false
  ;

  SendFeed._() : super();
  factory SendFeed({
    $core.String? content,
  }) {
    final _result = create();
    if (content != null) {
      _result.content = content;
    }
    return _result;
  }
  factory SendFeed.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory SendFeed.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  SendFeed clone() => SendFeed()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  SendFeed copyWith(void Function(SendFeed) updates) => super.copyWith((message) => updates(message as SendFeed)) as SendFeed; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static SendFeed create() => SendFeed._();
  SendFeed createEmptyInstance() => create();
  static $pb.PbList<SendFeed> createRepeated() => $pb.PbList<SendFeed>();
  @$core.pragma('dart2js:noInline')
  static SendFeed getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SendFeed>(create);
  static SendFeed? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get content => $_getSZ(0);
  @$pb.TagNumber(1)
  set content($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasContent() => $_has(0);
  @$pb.TagNumber(1)
  void clearContent() => clearField(1);
}

