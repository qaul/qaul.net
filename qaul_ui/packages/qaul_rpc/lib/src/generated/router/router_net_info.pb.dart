///
//  Generated code. Do not modify.
//  source: router/router_net_info.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

class RouterInfoContainer extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'RouterInfoContainer', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.net.router_net_info'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'signature', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'message', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  RouterInfoContainer._() : super();
  factory RouterInfoContainer({
    $core.List<$core.int>? signature,
    $core.List<$core.int>? message,
  }) {
    final _result = create();
    if (signature != null) {
      _result.signature = signature;
    }
    if (message != null) {
      _result.message = message;
    }
    return _result;
  }
  factory RouterInfoContainer.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RouterInfoContainer.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RouterInfoContainer clone() => RouterInfoContainer()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RouterInfoContainer copyWith(void Function(RouterInfoContainer) updates) => super.copyWith((message) => updates(message as RouterInfoContainer)) as RouterInfoContainer; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static RouterInfoContainer create() => RouterInfoContainer._();
  RouterInfoContainer createEmptyInstance() => create();
  static $pb.PbList<RouterInfoContainer> createRepeated() => $pb.PbList<RouterInfoContainer>();
  @$core.pragma('dart2js:noInline')
  static RouterInfoContainer getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RouterInfoContainer>(create);
  static RouterInfoContainer? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get signature => $_getN(0);
  @$pb.TagNumber(1)
  set signature($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasSignature() => $_has(0);
  @$pb.TagNumber(1)
  void clearSignature() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get message => $_getN(1);
  @$pb.TagNumber(2)
  set message($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasMessage() => $_has(1);
  @$pb.TagNumber(2)
  void clearMessage() => clearField(2);
}

class RouterInfoContent extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'RouterInfoContent', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.net.router_net_info'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'id', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'content', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'time', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  RouterInfoContent._() : super();
  factory RouterInfoContent({
    $core.List<$core.int>? id,
    $core.List<$core.int>? content,
    $fixnum.Int64? time,
  }) {
    final _result = create();
    if (id != null) {
      _result.id = id;
    }
    if (content != null) {
      _result.content = content;
    }
    if (time != null) {
      _result.time = time;
    }
    return _result;
  }
  factory RouterInfoContent.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RouterInfoContent.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RouterInfoContent clone() => RouterInfoContent()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RouterInfoContent copyWith(void Function(RouterInfoContent) updates) => super.copyWith((message) => updates(message as RouterInfoContent)) as RouterInfoContent; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static RouterInfoContent create() => RouterInfoContent._();
  RouterInfoContent createEmptyInstance() => create();
  static $pb.PbList<RouterInfoContent> createRepeated() => $pb.PbList<RouterInfoContent>();
  @$core.pragma('dart2js:noInline')
  static RouterInfoContent getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RouterInfoContent>(create);
  static RouterInfoContent? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get id => $_getN(0);
  @$pb.TagNumber(1)
  set id($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get content => $_getN(1);
  @$pb.TagNumber(2)
  set content($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasContent() => $_has(1);
  @$pb.TagNumber(2)
  void clearContent() => clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get time => $_getI64(2);
  @$pb.TagNumber(3)
  set time($fixnum.Int64 v) { $_setInt64(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasTime() => $_has(2);
  @$pb.TagNumber(3)
  void clearTime() => clearField(3);
}

class RouterInfoMessage extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'RouterInfoMessage', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.net.router_net_info'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'node', $pb.PbFieldType.OY)
    ..aOM<RoutingInfoTable>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'routes', subBuilder: RoutingInfoTable.create)
    ..aOM<UserInfoTable>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'users', subBuilder: UserInfoTable.create)
    ..a<$fixnum.Int64>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'timestamp', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  RouterInfoMessage._() : super();
  factory RouterInfoMessage({
    $core.List<$core.int>? node,
    RoutingInfoTable? routes,
    UserInfoTable? users,
    $fixnum.Int64? timestamp,
  }) {
    final _result = create();
    if (node != null) {
      _result.node = node;
    }
    if (routes != null) {
      _result.routes = routes;
    }
    if (users != null) {
      _result.users = users;
    }
    if (timestamp != null) {
      _result.timestamp = timestamp;
    }
    return _result;
  }
  factory RouterInfoMessage.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RouterInfoMessage.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RouterInfoMessage clone() => RouterInfoMessage()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RouterInfoMessage copyWith(void Function(RouterInfoMessage) updates) => super.copyWith((message) => updates(message as RouterInfoMessage)) as RouterInfoMessage; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static RouterInfoMessage create() => RouterInfoMessage._();
  RouterInfoMessage createEmptyInstance() => create();
  static $pb.PbList<RouterInfoMessage> createRepeated() => $pb.PbList<RouterInfoMessage>();
  @$core.pragma('dart2js:noInline')
  static RouterInfoMessage getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RouterInfoMessage>(create);
  static RouterInfoMessage? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get node => $_getN(0);
  @$pb.TagNumber(1)
  set node($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasNode() => $_has(0);
  @$pb.TagNumber(1)
  void clearNode() => clearField(1);

  @$pb.TagNumber(2)
  RoutingInfoTable get routes => $_getN(1);
  @$pb.TagNumber(2)
  set routes(RoutingInfoTable v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasRoutes() => $_has(1);
  @$pb.TagNumber(2)
  void clearRoutes() => clearField(2);
  @$pb.TagNumber(2)
  RoutingInfoTable ensureRoutes() => $_ensure(1);

  @$pb.TagNumber(3)
  UserInfoTable get users => $_getN(2);
  @$pb.TagNumber(3)
  set users(UserInfoTable v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasUsers() => $_has(2);
  @$pb.TagNumber(3)
  void clearUsers() => clearField(3);
  @$pb.TagNumber(3)
  UserInfoTable ensureUsers() => $_ensure(2);

  @$pb.TagNumber(4)
  $fixnum.Int64 get timestamp => $_getI64(3);
  @$pb.TagNumber(4)
  set timestamp($fixnum.Int64 v) { $_setInt64(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasTimestamp() => $_has(3);
  @$pb.TagNumber(4)
  void clearTimestamp() => clearField(4);
}

enum Routing_Message {
  userInfoTable, 
  userInfo, 
  routingInfoTable, 
  routingInfoEntry, 
  notSet
}

class Routing extends $pb.GeneratedMessage {
  static const $core.Map<$core.int, Routing_Message> _Routing_MessageByTag = {
    1 : Routing_Message.userInfoTable,
    2 : Routing_Message.userInfo,
    3 : Routing_Message.routingInfoTable,
    4 : Routing_Message.routingInfoEntry,
    0 : Routing_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Routing', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.net.router_net_info'), createEmptyInstance: create)
    ..oo(0, [1, 2, 3, 4])
    ..aOM<UserInfoTable>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'userInfoTable', subBuilder: UserInfoTable.create)
    ..aOM<UserInfo>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'userInfo', subBuilder: UserInfo.create)
    ..aOM<RoutingInfoTable>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'routingInfoTable', subBuilder: RoutingInfoTable.create)
    ..aOM<RoutingInfoEntry>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'routingInfoEntry', subBuilder: RoutingInfoEntry.create)
    ..hasRequiredFields = false
  ;

  Routing._() : super();
  factory Routing({
    UserInfoTable? userInfoTable,
    UserInfo? userInfo,
    RoutingInfoTable? routingInfoTable,
    RoutingInfoEntry? routingInfoEntry,
  }) {
    final _result = create();
    if (userInfoTable != null) {
      _result.userInfoTable = userInfoTable;
    }
    if (userInfo != null) {
      _result.userInfo = userInfo;
    }
    if (routingInfoTable != null) {
      _result.routingInfoTable = routingInfoTable;
    }
    if (routingInfoEntry != null) {
      _result.routingInfoEntry = routingInfoEntry;
    }
    return _result;
  }
  factory Routing.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Routing.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Routing clone() => Routing()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Routing copyWith(void Function(Routing) updates) => super.copyWith((message) => updates(message as Routing)) as Routing; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Routing create() => Routing._();
  Routing createEmptyInstance() => create();
  static $pb.PbList<Routing> createRepeated() => $pb.PbList<Routing>();
  @$core.pragma('dart2js:noInline')
  static Routing getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Routing>(create);
  static Routing? _defaultInstance;

  Routing_Message whichMessage() => _Routing_MessageByTag[$_whichOneof(0)]!;
  void clearMessage() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  UserInfoTable get userInfoTable => $_getN(0);
  @$pb.TagNumber(1)
  set userInfoTable(UserInfoTable v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasUserInfoTable() => $_has(0);
  @$pb.TagNumber(1)
  void clearUserInfoTable() => clearField(1);
  @$pb.TagNumber(1)
  UserInfoTable ensureUserInfoTable() => $_ensure(0);

  @$pb.TagNumber(2)
  UserInfo get userInfo => $_getN(1);
  @$pb.TagNumber(2)
  set userInfo(UserInfo v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasUserInfo() => $_has(1);
  @$pb.TagNumber(2)
  void clearUserInfo() => clearField(2);
  @$pb.TagNumber(2)
  UserInfo ensureUserInfo() => $_ensure(1);

  @$pb.TagNumber(3)
  RoutingInfoTable get routingInfoTable => $_getN(2);
  @$pb.TagNumber(3)
  set routingInfoTable(RoutingInfoTable v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasRoutingInfoTable() => $_has(2);
  @$pb.TagNumber(3)
  void clearRoutingInfoTable() => clearField(3);
  @$pb.TagNumber(3)
  RoutingInfoTable ensureRoutingInfoTable() => $_ensure(2);

  @$pb.TagNumber(4)
  RoutingInfoEntry get routingInfoEntry => $_getN(3);
  @$pb.TagNumber(4)
  set routingInfoEntry(RoutingInfoEntry v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasRoutingInfoEntry() => $_has(3);
  @$pb.TagNumber(4)
  void clearRoutingInfoEntry() => clearField(4);
  @$pb.TagNumber(4)
  RoutingInfoEntry ensureRoutingInfoEntry() => $_ensure(3);
}

class RoutingInfoTable extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'RoutingInfoTable', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.net.router_net_info'), createEmptyInstance: create)
    ..pc<RoutingInfoEntry>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'entry', $pb.PbFieldType.PM, subBuilder: RoutingInfoEntry.create)
    ..hasRequiredFields = false
  ;

  RoutingInfoTable._() : super();
  factory RoutingInfoTable({
    $core.Iterable<RoutingInfoEntry>? entry,
  }) {
    final _result = create();
    if (entry != null) {
      _result.entry.addAll(entry);
    }
    return _result;
  }
  factory RoutingInfoTable.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RoutingInfoTable.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RoutingInfoTable clone() => RoutingInfoTable()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RoutingInfoTable copyWith(void Function(RoutingInfoTable) updates) => super.copyWith((message) => updates(message as RoutingInfoTable)) as RoutingInfoTable; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static RoutingInfoTable create() => RoutingInfoTable._();
  RoutingInfoTable createEmptyInstance() => create();
  static $pb.PbList<RoutingInfoTable> createRepeated() => $pb.PbList<RoutingInfoTable>();
  @$core.pragma('dart2js:noInline')
  static RoutingInfoTable getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RoutingInfoTable>(create);
  static RoutingInfoTable? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<RoutingInfoEntry> get entry => $_getList(0);
}

class RoutingInfoEntry extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'RoutingInfoEntry', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.net.router_net_info'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'user', $pb.PbFieldType.OY)
    ..a<$core.int>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'rtt', $pb.PbFieldType.OU3)
    ..a<$core.List<$core.int>>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'hc', $pb.PbFieldType.OY)
    ..a<$core.double>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'pl', $pb.PbFieldType.OF)
    ..hasRequiredFields = false
  ;

  RoutingInfoEntry._() : super();
  factory RoutingInfoEntry({
    $core.List<$core.int>? user,
    $core.int? rtt,
    $core.List<$core.int>? hc,
    $core.double? pl,
  }) {
    final _result = create();
    if (user != null) {
      _result.user = user;
    }
    if (rtt != null) {
      _result.rtt = rtt;
    }
    if (hc != null) {
      _result.hc = hc;
    }
    if (pl != null) {
      _result.pl = pl;
    }
    return _result;
  }
  factory RoutingInfoEntry.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RoutingInfoEntry.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RoutingInfoEntry clone() => RoutingInfoEntry()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RoutingInfoEntry copyWith(void Function(RoutingInfoEntry) updates) => super.copyWith((message) => updates(message as RoutingInfoEntry)) as RoutingInfoEntry; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static RoutingInfoEntry create() => RoutingInfoEntry._();
  RoutingInfoEntry createEmptyInstance() => create();
  static $pb.PbList<RoutingInfoEntry> createRepeated() => $pb.PbList<RoutingInfoEntry>();
  @$core.pragma('dart2js:noInline')
  static RoutingInfoEntry getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RoutingInfoEntry>(create);
  static RoutingInfoEntry? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get user => $_getN(0);
  @$pb.TagNumber(1)
  set user($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasUser() => $_has(0);
  @$pb.TagNumber(1)
  void clearUser() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get rtt => $_getIZ(1);
  @$pb.TagNumber(2)
  set rtt($core.int v) { $_setUnsignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasRtt() => $_has(1);
  @$pb.TagNumber(2)
  void clearRtt() => clearField(2);

  @$pb.TagNumber(3)
  $core.List<$core.int> get hc => $_getN(2);
  @$pb.TagNumber(3)
  set hc($core.List<$core.int> v) { $_setBytes(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasHc() => $_has(2);
  @$pb.TagNumber(3)
  void clearHc() => clearField(3);

  @$pb.TagNumber(4)
  $core.double get pl => $_getN(3);
  @$pb.TagNumber(4)
  set pl($core.double v) { $_setFloat(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasPl() => $_has(3);
  @$pb.TagNumber(4)
  void clearPl() => clearField(4);
}

class UserInfoTable extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'UserInfoTable', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.net.router_net_info'), createEmptyInstance: create)
    ..pc<UserInfo>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'info', $pb.PbFieldType.PM, subBuilder: UserInfo.create)
    ..hasRequiredFields = false
  ;

  UserInfoTable._() : super();
  factory UserInfoTable({
    $core.Iterable<UserInfo>? info,
  }) {
    final _result = create();
    if (info != null) {
      _result.info.addAll(info);
    }
    return _result;
  }
  factory UserInfoTable.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory UserInfoTable.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  UserInfoTable clone() => UserInfoTable()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  UserInfoTable copyWith(void Function(UserInfoTable) updates) => super.copyWith((message) => updates(message as UserInfoTable)) as UserInfoTable; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static UserInfoTable create() => UserInfoTable._();
  UserInfoTable createEmptyInstance() => create();
  static $pb.PbList<UserInfoTable> createRepeated() => $pb.PbList<UserInfoTable>();
  @$core.pragma('dart2js:noInline')
  static UserInfoTable getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UserInfoTable>(create);
  static UserInfoTable? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<UserInfo> get info => $_getList(0);
}

class UserInfo extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'UserInfo', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.net.router_net_info'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'id', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'key', $pb.PbFieldType.OY)
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'name')
    ..hasRequiredFields = false
  ;

  UserInfo._() : super();
  factory UserInfo({
    $core.List<$core.int>? id,
    $core.List<$core.int>? key,
    $core.String? name,
  }) {
    final _result = create();
    if (id != null) {
      _result.id = id;
    }
    if (key != null) {
      _result.key = key;
    }
    if (name != null) {
      _result.name = name;
    }
    return _result;
  }
  factory UserInfo.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory UserInfo.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  UserInfo clone() => UserInfo()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  UserInfo copyWith(void Function(UserInfo) updates) => super.copyWith((message) => updates(message as UserInfo)) as UserInfo; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static UserInfo create() => UserInfo._();
  UserInfo createEmptyInstance() => create();
  static $pb.PbList<UserInfo> createRepeated() => $pb.PbList<UserInfo>();
  @$core.pragma('dart2js:noInline')
  static UserInfo getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UserInfo>(create);
  static UserInfo? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get id => $_getN(0);
  @$pb.TagNumber(1)
  set id($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get key => $_getN(1);
  @$pb.TagNumber(2)
  set key($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasKey() => $_has(1);
  @$pb.TagNumber(2)
  void clearKey() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get name => $_getSZ(2);
  @$pb.TagNumber(3)
  set name($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasName() => $_has(2);
  @$pb.TagNumber(3)
  void clearName() => clearField(3);
}

