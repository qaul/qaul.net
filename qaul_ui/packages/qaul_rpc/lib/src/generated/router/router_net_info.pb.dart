// This is a generated file - do not edit.
//
// Generated from router/router_net_info.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'router_net_info.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'router_net_info.pbenum.dart';

/// Router information Container
class RouterInfoContainer extends $pb.GeneratedMessage {
  factory RouterInfoContainer({
    $core.List<$core.int>? signature,
    $core.List<$core.int>? message,
  }) {
    final result = create();
    if (signature != null) result.signature = signature;
    if (message != null) result.message = message;
    return result;
  }

  RouterInfoContainer._();

  factory RouterInfoContainer.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory RouterInfoContainer.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RouterInfoContainer',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'qaul.net.router_net_info'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'signature', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(
        2, _omitFieldNames ? '' : 'message', $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RouterInfoContainer clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RouterInfoContainer copyWith(void Function(RouterInfoContainer) updates) =>
      super.copyWith((message) => updates(message as RouterInfoContainer))
          as RouterInfoContainer;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RouterInfoContainer create() => RouterInfoContainer._();
  @$core.override
  RouterInfoContainer createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static RouterInfoContainer getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RouterInfoContainer>(create);
  static RouterInfoContainer? _defaultInstance;

  /// signature
  @$pb.TagNumber(1)
  $core.List<$core.int> get signature => $_getN(0);
  @$pb.TagNumber(1)
  set signature($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSignature() => $_has(0);
  @$pb.TagNumber(1)
  void clearSignature() => $_clearField(1);

  /// message content
  @$pb.TagNumber(2)
  $core.List<$core.int> get message => $_getN(1);
  @$pb.TagNumber(2)
  set message($core.List<$core.int> value) => $_setBytes(1, value);
  @$pb.TagNumber(2)
  $core.bool hasMessage() => $_has(1);
  @$pb.TagNumber(2)
  void clearMessage() => $_clearField(2);
}

/// Router information content
class RouterInfoContent extends $pb.GeneratedMessage {
  factory RouterInfoContent({
    $core.List<$core.int>? id,
    RouterInfoModule? routerInfoModule,
    $core.List<$core.int>? content,
    $fixnum.Int64? time,
  }) {
    final result = create();
    if (id != null) result.id = id;
    if (routerInfoModule != null) result.routerInfoModule = routerInfoModule;
    if (content != null) result.content = content;
    if (time != null) result.time = time;
    return result;
  }

  RouterInfoContent._();

  factory RouterInfoContent.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory RouterInfoContent.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RouterInfoContent',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'qaul.net.router_net_info'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'id', $pb.PbFieldType.OY)
    ..aE<RouterInfoModule>(2, _omitFieldNames ? '' : 'routerInfoModule',
        protoName: 'routerInfoModule', enumValues: RouterInfoModule.values)
    ..a<$core.List<$core.int>>(
        3, _omitFieldNames ? '' : 'content', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(4, _omitFieldNames ? '' : 'time', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RouterInfoContent clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RouterInfoContent copyWith(void Function(RouterInfoContent) updates) =>
      super.copyWith((message) => updates(message as RouterInfoContent))
          as RouterInfoContent;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RouterInfoContent create() => RouterInfoContent._();
  @$core.override
  RouterInfoContent createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static RouterInfoContent getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RouterInfoContent>(create);
  static RouterInfoContent? _defaultInstance;

  /// node id
  @$pb.TagNumber(1)
  $core.List<$core.int> get id => $_getN(0);
  @$pb.TagNumber(1)
  set id($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  /// RouterInfo Module
  @$pb.TagNumber(2)
  RouterInfoModule get routerInfoModule => $_getN(1);
  @$pb.TagNumber(2)
  set routerInfoModule(RouterInfoModule value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasRouterInfoModule() => $_has(1);
  @$pb.TagNumber(2)
  void clearRouterInfoModule() => $_clearField(2);

  /// message content
  @$pb.TagNumber(3)
  $core.List<$core.int> get content => $_getN(2);
  @$pb.TagNumber(3)
  set content($core.List<$core.int> value) => $_setBytes(2, value);
  @$pb.TagNumber(3)
  $core.bool hasContent() => $_has(2);
  @$pb.TagNumber(3)
  void clearContent() => $_clearField(3);

  /// timestamp in milli seconds
  @$pb.TagNumber(4)
  $fixnum.Int64 get time => $_getI64(3);
  @$pb.TagNumber(4)
  set time($fixnum.Int64 value) => $_setInt64(3, value);
  @$pb.TagNumber(4)
  $core.bool hasTime() => $_has(3);
  @$pb.TagNumber(4)
  void clearTime() => $_clearField(4);
}

/// Router information message
class RouterInfoMessage extends $pb.GeneratedMessage {
  factory RouterInfoMessage({
    $core.List<$core.int>? node,
    RoutingInfoTable? routes,
    FeedIdsTable? feeds,
    $fixnum.Int64? timestamp,
  }) {
    final result = create();
    if (node != null) result.node = node;
    if (routes != null) result.routes = routes;
    if (feeds != null) result.feeds = feeds;
    if (timestamp != null) result.timestamp = timestamp;
    return result;
  }

  RouterInfoMessage._();

  factory RouterInfoMessage.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory RouterInfoMessage.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RouterInfoMessage',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'qaul.net.router_net_info'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'node', $pb.PbFieldType.OY)
    ..aOM<RoutingInfoTable>(2, _omitFieldNames ? '' : 'routes',
        subBuilder: RoutingInfoTable.create)
    ..aOM<FeedIdsTable>(4, _omitFieldNames ? '' : 'feeds',
        subBuilder: FeedIdsTable.create)
    ..a<$fixnum.Int64>(
        5, _omitFieldNames ? '' : 'timestamp', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RouterInfoMessage clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RouterInfoMessage copyWith(void Function(RouterInfoMessage) updates) =>
      super.copyWith((message) => updates(message as RouterInfoMessage))
          as RouterInfoMessage;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RouterInfoMessage create() => RouterInfoMessage._();
  @$core.override
  RouterInfoMessage createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static RouterInfoMessage getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RouterInfoMessage>(create);
  static RouterInfoMessage? _defaultInstance;

  /// node id
  @$pb.TagNumber(1)
  $core.List<$core.int> get node => $_getN(0);
  @$pb.TagNumber(1)
  set node($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasNode() => $_has(0);
  @$pb.TagNumber(1)
  void clearNode() => $_clearField(1);

  /// Routing information table
  @$pb.TagNumber(2)
  RoutingInfoTable get routes => $_getN(1);
  @$pb.TagNumber(2)
  set routes(RoutingInfoTable value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasRoutes() => $_has(1);
  @$pb.TagNumber(2)
  void clearRoutes() => $_clearField(2);
  @$pb.TagNumber(2)
  RoutingInfoTable ensureRoutes() => $_ensure(1);

  /// Latest Feed ids table
  @$pb.TagNumber(4)
  FeedIdsTable get feeds => $_getN(2);
  @$pb.TagNumber(4)
  set feeds(FeedIdsTable value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasFeeds() => $_has(2);
  @$pb.TagNumber(4)
  void clearFeeds() => $_clearField(4);
  @$pb.TagNumber(4)
  FeedIdsTable ensureFeeds() => $_ensure(2);

  /// timestamp
  @$pb.TagNumber(5)
  $fixnum.Int64 get timestamp => $_getI64(3);
  @$pb.TagNumber(5)
  set timestamp($fixnum.Int64 value) => $_setInt64(3, value);
  @$pb.TagNumber(5)
  $core.bool hasTimestamp() => $_has(3);
  @$pb.TagNumber(5)
  void clearTimestamp() => $_clearField(5);
}

/// Routing information to send to neighbours
class RoutingInfoTable extends $pb.GeneratedMessage {
  factory RoutingInfoTable({
    $core.Iterable<RoutingInfoEntry>? entry,
  }) {
    final result = create();
    if (entry != null) result.entry.addAll(entry);
    return result;
  }

  RoutingInfoTable._();

  factory RoutingInfoTable.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory RoutingInfoTable.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RoutingInfoTable',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'qaul.net.router_net_info'),
      createEmptyInstance: create)
    ..pPM<RoutingInfoEntry>(1, _omitFieldNames ? '' : 'entry',
        subBuilder: RoutingInfoEntry.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RoutingInfoTable clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RoutingInfoTable copyWith(void Function(RoutingInfoTable) updates) =>
      super.copyWith((message) => updates(message as RoutingInfoTable))
          as RoutingInfoTable;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RoutingInfoTable create() => RoutingInfoTable._();
  @$core.override
  RoutingInfoTable createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static RoutingInfoTable getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RoutingInfoTable>(create);
  static RoutingInfoTable? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<RoutingInfoEntry> get entry => $_getList(0);
}

/// Routing structures to send over the network
class RoutingInfoEntry extends $pb.GeneratedMessage {
  factory RoutingInfoEntry({
    $core.List<$core.int>? user,
    $core.int? rtt,
    $core.List<$core.int>? hc,
    $core.int? pgid,
  }) {
    final result = create();
    if (user != null) result.user = user;
    if (rtt != null) result.rtt = rtt;
    if (hc != null) result.hc = hc;
    if (pgid != null) result.pgid = pgid;
    return result;
  }

  RoutingInfoEntry._();

  factory RoutingInfoEntry.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory RoutingInfoEntry.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RoutingInfoEntry',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'qaul.net.router_net_info'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'user', $pb.PbFieldType.OY)
    ..aI(2, _omitFieldNames ? '' : 'rtt', fieldType: $pb.PbFieldType.OU3)
    ..a<$core.List<$core.int>>(
        3, _omitFieldNames ? '' : 'hc', $pb.PbFieldType.OY)
    ..aI(5, _omitFieldNames ? '' : 'pgid', fieldType: $pb.PbFieldType.OU3)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RoutingInfoEntry clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RoutingInfoEntry copyWith(void Function(RoutingInfoEntry) updates) =>
      super.copyWith((message) => updates(message as RoutingInfoEntry))
          as RoutingInfoEntry;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RoutingInfoEntry create() => RoutingInfoEntry._();
  @$core.override
  RoutingInfoEntry createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static RoutingInfoEntry getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RoutingInfoEntry>(create);
  static RoutingInfoEntry? _defaultInstance;

  /// user id
  @$pb.TagNumber(1)
  $core.List<$core.int> get user => $_getN(0);
  @$pb.TagNumber(1)
  set user($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasUser() => $_has(0);
  @$pb.TagNumber(1)
  void clearUser() => $_clearField(1);

  /// round trip time
  @$pb.TagNumber(2)
  $core.int get rtt => $_getIZ(1);
  @$pb.TagNumber(2)
  set rtt($core.int value) => $_setUnsignedInt32(1, value);
  @$pb.TagNumber(2)
  $core.bool hasRtt() => $_has(1);
  @$pb.TagNumber(2)
  void clearRtt() => $_clearField(2);

  /// hop count
  @$pb.TagNumber(3)
  $core.List<$core.int> get hc => $_getN(2);
  @$pb.TagNumber(3)
  set hc($core.List<$core.int> value) => $_setBytes(2, value);
  @$pb.TagNumber(3)
  $core.bool hasHc() => $_has(2);
  @$pb.TagNumber(3)
  void clearHc() => $_clearField(3);

  /// propagation id
  @$pb.TagNumber(5)
  $core.int get pgid => $_getIZ(3);
  @$pb.TagNumber(5)
  set pgid($core.int value) => $_setUnsignedInt32(3, value);
  @$pb.TagNumber(5)
  $core.bool hasPgid() => $_has(3);
  @$pb.TagNumber(5)
  void clearPgid() => $_clearField(5);
}

/// User information table
class UserIdTable extends $pb.GeneratedMessage {
  factory UserIdTable({
    $core.Iterable<$core.List<$core.int>>? ids,
  }) {
    final result = create();
    if (ids != null) result.ids.addAll(ids);
    return result;
  }

  UserIdTable._();

  factory UserIdTable.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory UserIdTable.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UserIdTable',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'qaul.net.router_net_info'),
      createEmptyInstance: create)
    ..p<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'ids', $pb.PbFieldType.PY)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UserIdTable clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UserIdTable copyWith(void Function(UserIdTable) updates) =>
      super.copyWith((message) => updates(message as UserIdTable))
          as UserIdTable;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static UserIdTable create() => UserIdTable._();
  @$core.override
  UserIdTable createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static UserIdTable getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UserIdTable>(create);
  static UserIdTable? _defaultInstance;

  /// user ids
  @$pb.TagNumber(1)
  $pb.PbList<$core.List<$core.int>> get ids => $_getList(0);
}

/// User information table
class UserInfoTable extends $pb.GeneratedMessage {
  factory UserInfoTable({
    $core.Iterable<UserInfo>? info,
  }) {
    final result = create();
    if (info != null) result.info.addAll(info);
    return result;
  }

  UserInfoTable._();

  factory UserInfoTable.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory UserInfoTable.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UserInfoTable',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'qaul.net.router_net_info'),
      createEmptyInstance: create)
    ..pPM<UserInfo>(1, _omitFieldNames ? '' : 'info',
        subBuilder: UserInfo.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UserInfoTable clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  UserInfoTable copyWith(void Function(UserInfoTable) updates) =>
      super.copyWith((message) => updates(message as UserInfoTable))
          as UserInfoTable;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static UserInfoTable create() => UserInfoTable._();
  @$core.override
  UserInfoTable createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static UserInfoTable getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<UserInfoTable>(create);
  static UserInfoTable? _defaultInstance;

  /// user info
  @$pb.TagNumber(1)
  $pb.PbList<UserInfo> get info => $_getList(0);
}

/// User info structure for sending to the neighbours
class UserInfo extends $pb.GeneratedMessage {
  factory UserInfo({
    $core.List<$core.int>? id,
    $core.List<$core.int>? key,
    $core.String? name,
  }) {
    final result = create();
    if (id != null) result.id = id;
    if (key != null) result.key = key;
    if (name != null) result.name = name;
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
          _omitMessageNames ? '' : 'qaul.net.router_net_info'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'id', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(
        2, _omitFieldNames ? '' : 'key', $pb.PbFieldType.OY)
    ..aOS(3, _omitFieldNames ? '' : 'name')
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

  /// user id
  @$pb.TagNumber(1)
  $core.List<$core.int> get id => $_getN(0);
  @$pb.TagNumber(1)
  set id($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  /// public key of the user
  @$pb.TagNumber(2)
  $core.List<$core.int> get key => $_getN(1);
  @$pb.TagNumber(2)
  set key($core.List<$core.int> value) => $_setBytes(1, value);
  @$pb.TagNumber(2)
  $core.bool hasKey() => $_has(1);
  @$pb.TagNumber(2)
  void clearKey() => $_clearField(2);

  /// user name
  @$pb.TagNumber(3)
  $core.String get name => $_getSZ(2);
  @$pb.TagNumber(3)
  set name($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasName() => $_has(2);
  @$pb.TagNumber(3)
  void clearName() => $_clearField(3);
}

/// List of feed ID's
class FeedIdsTable extends $pb.GeneratedMessage {
  factory FeedIdsTable({
    $core.Iterable<$core.List<$core.int>>? ids,
  }) {
    final result = create();
    if (ids != null) result.ids.addAll(ids);
    return result;
  }

  FeedIdsTable._();

  factory FeedIdsTable.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory FeedIdsTable.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'FeedIdsTable',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'qaul.net.router_net_info'),
      createEmptyInstance: create)
    ..p<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'ids', $pb.PbFieldType.PY)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  FeedIdsTable clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  FeedIdsTable copyWith(void Function(FeedIdsTable) updates) =>
      super.copyWith((message) => updates(message as FeedIdsTable))
          as FeedIdsTable;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FeedIdsTable create() => FeedIdsTable._();
  @$core.override
  FeedIdsTable createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static FeedIdsTable getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<FeedIdsTable>(create);
  static FeedIdsTable? _defaultInstance;

  /// feed id
  @$pb.TagNumber(1)
  $pb.PbList<$core.List<$core.int>> get ids => $_getList(0);
}

/// Feed request message
class FeedRequestMessage extends $pb.GeneratedMessage {
  factory FeedRequestMessage({
    FeedIdsTable? feeds,
  }) {
    final result = create();
    if (feeds != null) result.feeds = feeds;
    return result;
  }

  FeedRequestMessage._();

  factory FeedRequestMessage.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory FeedRequestMessage.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'FeedRequestMessage',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'qaul.net.router_net_info'),
      createEmptyInstance: create)
    ..aOM<FeedIdsTable>(1, _omitFieldNames ? '' : 'feeds',
        subBuilder: FeedIdsTable.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  FeedRequestMessage clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  FeedRequestMessage copyWith(void Function(FeedRequestMessage) updates) =>
      super.copyWith((message) => updates(message as FeedRequestMessage))
          as FeedRequestMessage;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FeedRequestMessage create() => FeedRequestMessage._();
  @$core.override
  FeedRequestMessage createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static FeedRequestMessage getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<FeedRequestMessage>(create);
  static FeedRequestMessage? _defaultInstance;

  /// Feed ids table
  @$pb.TagNumber(1)
  FeedIdsTable get feeds => $_getN(0);
  @$pb.TagNumber(1)
  set feeds(FeedIdsTable value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasFeeds() => $_has(0);
  @$pb.TagNumber(1)
  void clearFeeds() => $_clearField(1);
  @$pb.TagNumber(1)
  FeedIdsTable ensureFeeds() => $_ensure(0);
}

/// Feed response message
class FeedResponseMessage extends $pb.GeneratedMessage {
  factory FeedResponseMessage({
    FeedResponseTable? feeds,
  }) {
    final result = create();
    if (feeds != null) result.feeds = feeds;
    return result;
  }

  FeedResponseMessage._();

  factory FeedResponseMessage.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory FeedResponseMessage.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'FeedResponseMessage',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'qaul.net.router_net_info'),
      createEmptyInstance: create)
    ..aOM<FeedResponseTable>(1, _omitFieldNames ? '' : 'feeds',
        subBuilder: FeedResponseTable.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  FeedResponseMessage clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  FeedResponseMessage copyWith(void Function(FeedResponseMessage) updates) =>
      super.copyWith((message) => updates(message as FeedResponseMessage))
          as FeedResponseMessage;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FeedResponseMessage create() => FeedResponseMessage._();
  @$core.override
  FeedResponseMessage createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static FeedResponseMessage getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<FeedResponseMessage>(create);
  static FeedResponseMessage? _defaultInstance;

  /// Feed table
  @$pb.TagNumber(1)
  FeedResponseTable get feeds => $_getN(0);
  @$pb.TagNumber(1)
  set feeds(FeedResponseTable value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasFeeds() => $_has(0);
  @$pb.TagNumber(1)
  void clearFeeds() => $_clearField(1);
  @$pb.TagNumber(1)
  FeedResponseTable ensureFeeds() => $_ensure(0);
}

/// Feed response table
/// containing the feed messages for response
class FeedResponseTable extends $pb.GeneratedMessage {
  factory FeedResponseTable({
    $core.Iterable<FeedMessage>? messages,
  }) {
    final result = create();
    if (messages != null) result.messages.addAll(messages);
    return result;
  }

  FeedResponseTable._();

  factory FeedResponseTable.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory FeedResponseTable.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'FeedResponseTable',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'qaul.net.router_net_info'),
      createEmptyInstance: create)
    ..pPM<FeedMessage>(1, _omitFieldNames ? '' : 'messages',
        subBuilder: FeedMessage.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  FeedResponseTable clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  FeedResponseTable copyWith(void Function(FeedResponseTable) updates) =>
      super.copyWith((message) => updates(message as FeedResponseTable))
          as FeedResponseTable;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FeedResponseTable create() => FeedResponseTable._();
  @$core.override
  FeedResponseTable createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static FeedResponseTable getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<FeedResponseTable>(create);
  static FeedResponseTable? _defaultInstance;

  /// feed messages
  @$pb.TagNumber(1)
  $pb.PbList<FeedMessage> get messages => $_getList(0);
}

/// Feed Message
class FeedMessage extends $pb.GeneratedMessage {
  factory FeedMessage({
    $core.List<$core.int>? messageId,
    $core.List<$core.int>? senderId,
    $core.String? content,
    $fixnum.Int64? time,
  }) {
    final result = create();
    if (messageId != null) result.messageId = messageId;
    if (senderId != null) result.senderId = senderId;
    if (content != null) result.content = content;
    if (time != null) result.time = time;
    return result;
  }

  FeedMessage._();

  factory FeedMessage.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory FeedMessage.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'FeedMessage',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'qaul.net.router_net_info'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'messageId', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(
        2, _omitFieldNames ? '' : 'senderId', $pb.PbFieldType.OY)
    ..aOS(3, _omitFieldNames ? '' : 'content')
    ..a<$fixnum.Int64>(4, _omitFieldNames ? '' : 'time', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  FeedMessage clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  FeedMessage copyWith(void Function(FeedMessage) updates) =>
      super.copyWith((message) => updates(message as FeedMessage))
          as FeedMessage;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FeedMessage create() => FeedMessage._();
  @$core.override
  FeedMessage createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static FeedMessage getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<FeedMessage>(create);
  static FeedMessage? _defaultInstance;

  /// message id
  @$pb.TagNumber(1)
  $core.List<$core.int> get messageId => $_getN(0);
  @$pb.TagNumber(1)
  set messageId($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasMessageId() => $_has(0);
  @$pb.TagNumber(1)
  void clearMessageId() => $_clearField(1);

  /// sender id
  @$pb.TagNumber(2)
  $core.List<$core.int> get senderId => $_getN(1);
  @$pb.TagNumber(2)
  set senderId($core.List<$core.int> value) => $_setBytes(1, value);
  @$pb.TagNumber(2)
  $core.bool hasSenderId() => $_has(1);
  @$pb.TagNumber(2)
  void clearSenderId() => $_clearField(2);

  /// message content
  @$pb.TagNumber(3)
  $core.String get content => $_getSZ(2);
  @$pb.TagNumber(3)
  set content($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasContent() => $_has(2);
  @$pb.TagNumber(3)
  void clearContent() => $_clearField(3);

  /// timestamp in milli seconds
  @$pb.TagNumber(4)
  $fixnum.Int64 get time => $_getI64(3);
  @$pb.TagNumber(4)
  set time($fixnum.Int64 value) => $_setInt64(3, value);
  @$pb.TagNumber(4)
  $core.bool hasTime() => $_has(3);
  @$pb.TagNumber(4)
  void clearTime() => $_clearField(4);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
