//
//  Generated code. Do not modify.
//  source: router/router_net_info.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'router_net_info.pbenum.dart';

export 'router_net_info.pbenum.dart';

/// Router information Container
class RouterInfoContainer extends $pb.GeneratedMessage {
  factory RouterInfoContainer({
    $core.List<$core.int>? signature,
    $core.List<$core.int>? message,
  }) {
    final $result = create();
    if (signature != null) {
      $result.signature = signature;
    }
    if (message != null) {
      $result.message = message;
    }
    return $result;
  }
  RouterInfoContainer._() : super();
  factory RouterInfoContainer.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RouterInfoContainer.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'RouterInfoContainer', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.router_net_info'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'signature', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(2, _omitFieldNames ? '' : 'message', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RouterInfoContainer clone() => RouterInfoContainer()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RouterInfoContainer copyWith(void Function(RouterInfoContainer) updates) => super.copyWith((message) => updates(message as RouterInfoContainer)) as RouterInfoContainer;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RouterInfoContainer create() => RouterInfoContainer._();
  RouterInfoContainer createEmptyInstance() => create();
  static $pb.PbList<RouterInfoContainer> createRepeated() => $pb.PbList<RouterInfoContainer>();
  @$core.pragma('dart2js:noInline')
  static RouterInfoContainer getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RouterInfoContainer>(create);
  static RouterInfoContainer? _defaultInstance;

  /// signature
  @$pb.TagNumber(1)
  $core.List<$core.int> get signature => $_getN(0);
  @$pb.TagNumber(1)
  set signature($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasSignature() => $_has(0);
  @$pb.TagNumber(1)
  void clearSignature() => clearField(1);

  /// message content
  @$pb.TagNumber(2)
  $core.List<$core.int> get message => $_getN(1);
  @$pb.TagNumber(2)
  set message($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasMessage() => $_has(1);
  @$pb.TagNumber(2)
  void clearMessage() => clearField(2);
}

/// Router information content
class RouterInfoContent extends $pb.GeneratedMessage {
  factory RouterInfoContent({
    $core.List<$core.int>? id,
    RouterInfoModule? routerInfoModule,
    $core.List<$core.int>? content,
    $fixnum.Int64? time,
  }) {
    final $result = create();
    if (id != null) {
      $result.id = id;
    }
    if (routerInfoModule != null) {
      $result.routerInfoModule = routerInfoModule;
    }
    if (content != null) {
      $result.content = content;
    }
    if (time != null) {
      $result.time = time;
    }
    return $result;
  }
  RouterInfoContent._() : super();
  factory RouterInfoContent.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RouterInfoContent.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'RouterInfoContent', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.router_net_info'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'id', $pb.PbFieldType.OY)
    ..e<RouterInfoModule>(2, _omitFieldNames ? '' : 'routerInfoModule', $pb.PbFieldType.OE, protoName: 'routerInfoModule', defaultOrMaker: RouterInfoModule.ROUTER_INFO, valueOf: RouterInfoModule.valueOf, enumValues: RouterInfoModule.values)
    ..a<$core.List<$core.int>>(3, _omitFieldNames ? '' : 'content', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(4, _omitFieldNames ? '' : 'time', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RouterInfoContent clone() => RouterInfoContent()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RouterInfoContent copyWith(void Function(RouterInfoContent) updates) => super.copyWith((message) => updates(message as RouterInfoContent)) as RouterInfoContent;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RouterInfoContent create() => RouterInfoContent._();
  RouterInfoContent createEmptyInstance() => create();
  static $pb.PbList<RouterInfoContent> createRepeated() => $pb.PbList<RouterInfoContent>();
  @$core.pragma('dart2js:noInline')
  static RouterInfoContent getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RouterInfoContent>(create);
  static RouterInfoContent? _defaultInstance;

  /// node id
  @$pb.TagNumber(1)
  $core.List<$core.int> get id => $_getN(0);
  @$pb.TagNumber(1)
  set id($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => clearField(1);

  /// RouterInfo Module
  @$pb.TagNumber(2)
  RouterInfoModule get routerInfoModule => $_getN(1);
  @$pb.TagNumber(2)
  set routerInfoModule(RouterInfoModule v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasRouterInfoModule() => $_has(1);
  @$pb.TagNumber(2)
  void clearRouterInfoModule() => clearField(2);

  /// message content
  @$pb.TagNumber(3)
  $core.List<$core.int> get content => $_getN(2);
  @$pb.TagNumber(3)
  set content($core.List<$core.int> v) { $_setBytes(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasContent() => $_has(2);
  @$pb.TagNumber(3)
  void clearContent() => clearField(3);

  /// timestamp in milli seconds
  @$pb.TagNumber(4)
  $fixnum.Int64 get time => $_getI64(3);
  @$pb.TagNumber(4)
  set time($fixnum.Int64 v) { $_setInt64(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasTime() => $_has(3);
  @$pb.TagNumber(4)
  void clearTime() => clearField(4);
}

/// Router information message
class RouterInfoMessage extends $pb.GeneratedMessage {
  factory RouterInfoMessage({
    $core.List<$core.int>? node,
    RoutingInfoTable? routes,
    FeedIdsTable? feeds,
    $fixnum.Int64? timestamp,
  }) {
    final $result = create();
    if (node != null) {
      $result.node = node;
    }
    if (routes != null) {
      $result.routes = routes;
    }
    if (feeds != null) {
      $result.feeds = feeds;
    }
    if (timestamp != null) {
      $result.timestamp = timestamp;
    }
    return $result;
  }
  RouterInfoMessage._() : super();
  factory RouterInfoMessage.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RouterInfoMessage.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'RouterInfoMessage', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.router_net_info'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'node', $pb.PbFieldType.OY)
    ..aOM<RoutingInfoTable>(2, _omitFieldNames ? '' : 'routes', subBuilder: RoutingInfoTable.create)
    ..aOM<FeedIdsTable>(4, _omitFieldNames ? '' : 'feeds', subBuilder: FeedIdsTable.create)
    ..a<$fixnum.Int64>(5, _omitFieldNames ? '' : 'timestamp', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RouterInfoMessage clone() => RouterInfoMessage()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RouterInfoMessage copyWith(void Function(RouterInfoMessage) updates) => super.copyWith((message) => updates(message as RouterInfoMessage)) as RouterInfoMessage;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RouterInfoMessage create() => RouterInfoMessage._();
  RouterInfoMessage createEmptyInstance() => create();
  static $pb.PbList<RouterInfoMessage> createRepeated() => $pb.PbList<RouterInfoMessage>();
  @$core.pragma('dart2js:noInline')
  static RouterInfoMessage getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RouterInfoMessage>(create);
  static RouterInfoMessage? _defaultInstance;

  /// node id
  @$pb.TagNumber(1)
  $core.List<$core.int> get node => $_getN(0);
  @$pb.TagNumber(1)
  set node($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasNode() => $_has(0);
  @$pb.TagNumber(1)
  void clearNode() => clearField(1);

  /// Routing information table
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

  /// Latest Feed ids table
  @$pb.TagNumber(4)
  FeedIdsTable get feeds => $_getN(2);
  @$pb.TagNumber(4)
  set feeds(FeedIdsTable v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasFeeds() => $_has(2);
  @$pb.TagNumber(4)
  void clearFeeds() => clearField(4);
  @$pb.TagNumber(4)
  FeedIdsTable ensureFeeds() => $_ensure(2);

  /// timestamp
  @$pb.TagNumber(5)
  $fixnum.Int64 get timestamp => $_getI64(3);
  @$pb.TagNumber(5)
  set timestamp($fixnum.Int64 v) { $_setInt64(3, v); }
  @$pb.TagNumber(5)
  $core.bool hasTimestamp() => $_has(3);
  @$pb.TagNumber(5)
  void clearTimestamp() => clearField(5);
}

/// Routing information to send to neighbours
class RoutingInfoTable extends $pb.GeneratedMessage {
  factory RoutingInfoTable({
    $core.Iterable<RoutingInfoEntry>? entry,
  }) {
    final $result = create();
    if (entry != null) {
      $result.entry.addAll(entry);
    }
    return $result;
  }
  RoutingInfoTable._() : super();
  factory RoutingInfoTable.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RoutingInfoTable.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'RoutingInfoTable', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.router_net_info'), createEmptyInstance: create)
    ..pc<RoutingInfoEntry>(1, _omitFieldNames ? '' : 'entry', $pb.PbFieldType.PM, subBuilder: RoutingInfoEntry.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RoutingInfoTable clone() => RoutingInfoTable()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RoutingInfoTable copyWith(void Function(RoutingInfoTable) updates) => super.copyWith((message) => updates(message as RoutingInfoTable)) as RoutingInfoTable;

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

/// Routing structures to send over the network
class RoutingInfoEntry extends $pb.GeneratedMessage {
  factory RoutingInfoEntry({
    $core.List<$core.int>? user,
    $core.int? rtt,
    $core.List<$core.int>? hc,
    $core.int? pgid,
  }) {
    final $result = create();
    if (user != null) {
      $result.user = user;
    }
    if (rtt != null) {
      $result.rtt = rtt;
    }
    if (hc != null) {
      $result.hc = hc;
    }
    if (pgid != null) {
      $result.pgid = pgid;
    }
    return $result;
  }
  RoutingInfoEntry._() : super();
  factory RoutingInfoEntry.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RoutingInfoEntry.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'RoutingInfoEntry', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.router_net_info'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'user', $pb.PbFieldType.OY)
    ..a<$core.int>(2, _omitFieldNames ? '' : 'rtt', $pb.PbFieldType.OU3)
    ..a<$core.List<$core.int>>(3, _omitFieldNames ? '' : 'hc', $pb.PbFieldType.OY)
    ..a<$core.int>(5, _omitFieldNames ? '' : 'pgid', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RoutingInfoEntry clone() => RoutingInfoEntry()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RoutingInfoEntry copyWith(void Function(RoutingInfoEntry) updates) => super.copyWith((message) => updates(message as RoutingInfoEntry)) as RoutingInfoEntry;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RoutingInfoEntry create() => RoutingInfoEntry._();
  RoutingInfoEntry createEmptyInstance() => create();
  static $pb.PbList<RoutingInfoEntry> createRepeated() => $pb.PbList<RoutingInfoEntry>();
  @$core.pragma('dart2js:noInline')
  static RoutingInfoEntry getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RoutingInfoEntry>(create);
  static RoutingInfoEntry? _defaultInstance;

  /// user id
  @$pb.TagNumber(1)
  $core.List<$core.int> get user => $_getN(0);
  @$pb.TagNumber(1)
  set user($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasUser() => $_has(0);
  @$pb.TagNumber(1)
  void clearUser() => clearField(1);

  /// round trip time
  @$pb.TagNumber(2)
  $core.int get rtt => $_getIZ(1);
  @$pb.TagNumber(2)
  set rtt($core.int v) { $_setUnsignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasRtt() => $_has(1);
  @$pb.TagNumber(2)
  void clearRtt() => clearField(2);

  /// hop count
  @$pb.TagNumber(3)
  $core.List<$core.int> get hc => $_getN(2);
  @$pb.TagNumber(3)
  set hc($core.List<$core.int> v) { $_setBytes(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasHc() => $_has(2);
  @$pb.TagNumber(3)
  void clearHc() => clearField(3);

  /// propagation id
  @$pb.TagNumber(5)
  $core.int get pgid => $_getIZ(3);
  @$pb.TagNumber(5)
  set pgid($core.int v) { $_setUnsignedInt32(3, v); }
  @$pb.TagNumber(5)
  $core.bool hasPgid() => $_has(3);
  @$pb.TagNumber(5)
  void clearPgid() => clearField(5);
}

/// User information table
class UserIdTable extends $pb.GeneratedMessage {
  factory UserIdTable({
    $core.Iterable<$core.List<$core.int>>? ids,
  }) {
    final $result = create();
    if (ids != null) {
      $result.ids.addAll(ids);
    }
    return $result;
  }
  UserIdTable._() : super();
  factory UserIdTable.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory UserIdTable.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'UserIdTable', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.router_net_info'), createEmptyInstance: create)
    ..p<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'ids', $pb.PbFieldType.PY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  UserIdTable clone() => UserIdTable()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  UserIdTable copyWith(void Function(UserIdTable) updates) => super.copyWith((message) => updates(message as UserIdTable)) as UserIdTable;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static UserIdTable create() => UserIdTable._();
  UserIdTable createEmptyInstance() => create();
  static $pb.PbList<UserIdTable> createRepeated() => $pb.PbList<UserIdTable>();
  @$core.pragma('dart2js:noInline')
  static UserIdTable getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UserIdTable>(create);
  static UserIdTable? _defaultInstance;

  /// user ids
  @$pb.TagNumber(1)
  $core.List<$core.List<$core.int>> get ids => $_getList(0);
}

/// User information table
class UserInfoTable extends $pb.GeneratedMessage {
  factory UserInfoTable({
    $core.Iterable<UserInfo>? info,
  }) {
    final $result = create();
    if (info != null) {
      $result.info.addAll(info);
    }
    return $result;
  }
  UserInfoTable._() : super();
  factory UserInfoTable.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory UserInfoTable.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'UserInfoTable', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.router_net_info'), createEmptyInstance: create)
    ..pc<UserInfo>(1, _omitFieldNames ? '' : 'info', $pb.PbFieldType.PM, subBuilder: UserInfo.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  UserInfoTable clone() => UserInfoTable()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  UserInfoTable copyWith(void Function(UserInfoTable) updates) => super.copyWith((message) => updates(message as UserInfoTable)) as UserInfoTable;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static UserInfoTable create() => UserInfoTable._();
  UserInfoTable createEmptyInstance() => create();
  static $pb.PbList<UserInfoTable> createRepeated() => $pb.PbList<UserInfoTable>();
  @$core.pragma('dart2js:noInline')
  static UserInfoTable getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UserInfoTable>(create);
  static UserInfoTable? _defaultInstance;

  /// user info
  @$pb.TagNumber(1)
  $core.List<UserInfo> get info => $_getList(0);
}

/// User info structure for sending to the neighbours
class UserInfo extends $pb.GeneratedMessage {
  factory UserInfo({
    $core.List<$core.int>? id,
    $core.List<$core.int>? key,
    $core.String? name,
  }) {
    final $result = create();
    if (id != null) {
      $result.id = id;
    }
    if (key != null) {
      $result.key = key;
    }
    if (name != null) {
      $result.name = name;
    }
    return $result;
  }
  UserInfo._() : super();
  factory UserInfo.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory UserInfo.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'UserInfo', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.router_net_info'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'id', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(2, _omitFieldNames ? '' : 'key', $pb.PbFieldType.OY)
    ..aOS(3, _omitFieldNames ? '' : 'name')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  UserInfo clone() => UserInfo()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  UserInfo copyWith(void Function(UserInfo) updates) => super.copyWith((message) => updates(message as UserInfo)) as UserInfo;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static UserInfo create() => UserInfo._();
  UserInfo createEmptyInstance() => create();
  static $pb.PbList<UserInfo> createRepeated() => $pb.PbList<UserInfo>();
  @$core.pragma('dart2js:noInline')
  static UserInfo getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UserInfo>(create);
  static UserInfo? _defaultInstance;

  /// user id
  @$pb.TagNumber(1)
  $core.List<$core.int> get id => $_getN(0);
  @$pb.TagNumber(1)
  set id($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => clearField(1);

  /// public key of the user
  @$pb.TagNumber(2)
  $core.List<$core.int> get key => $_getN(1);
  @$pb.TagNumber(2)
  set key($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasKey() => $_has(1);
  @$pb.TagNumber(2)
  void clearKey() => clearField(2);

  /// user name
  @$pb.TagNumber(3)
  $core.String get name => $_getSZ(2);
  @$pb.TagNumber(3)
  set name($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasName() => $_has(2);
  @$pb.TagNumber(3)
  void clearName() => clearField(3);
}

/// List of feed ID's
class FeedIdsTable extends $pb.GeneratedMessage {
  factory FeedIdsTable({
    $core.Iterable<$core.List<$core.int>>? ids,
  }) {
    final $result = create();
    if (ids != null) {
      $result.ids.addAll(ids);
    }
    return $result;
  }
  FeedIdsTable._() : super();
  factory FeedIdsTable.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FeedIdsTable.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'FeedIdsTable', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.router_net_info'), createEmptyInstance: create)
    ..p<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'ids', $pb.PbFieldType.PY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  FeedIdsTable clone() => FeedIdsTable()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  FeedIdsTable copyWith(void Function(FeedIdsTable) updates) => super.copyWith((message) => updates(message as FeedIdsTable)) as FeedIdsTable;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FeedIdsTable create() => FeedIdsTable._();
  FeedIdsTable createEmptyInstance() => create();
  static $pb.PbList<FeedIdsTable> createRepeated() => $pb.PbList<FeedIdsTable>();
  @$core.pragma('dart2js:noInline')
  static FeedIdsTable getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FeedIdsTable>(create);
  static FeedIdsTable? _defaultInstance;

  /// feed id
  @$pb.TagNumber(1)
  $core.List<$core.List<$core.int>> get ids => $_getList(0);
}

/// Feed request message
class FeedRequestMessage extends $pb.GeneratedMessage {
  factory FeedRequestMessage({
    FeedIdsTable? feeds,
  }) {
    final $result = create();
    if (feeds != null) {
      $result.feeds = feeds;
    }
    return $result;
  }
  FeedRequestMessage._() : super();
  factory FeedRequestMessage.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FeedRequestMessage.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'FeedRequestMessage', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.router_net_info'), createEmptyInstance: create)
    ..aOM<FeedIdsTable>(1, _omitFieldNames ? '' : 'feeds', subBuilder: FeedIdsTable.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  FeedRequestMessage clone() => FeedRequestMessage()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  FeedRequestMessage copyWith(void Function(FeedRequestMessage) updates) => super.copyWith((message) => updates(message as FeedRequestMessage)) as FeedRequestMessage;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FeedRequestMessage create() => FeedRequestMessage._();
  FeedRequestMessage createEmptyInstance() => create();
  static $pb.PbList<FeedRequestMessage> createRepeated() => $pb.PbList<FeedRequestMessage>();
  @$core.pragma('dart2js:noInline')
  static FeedRequestMessage getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FeedRequestMessage>(create);
  static FeedRequestMessage? _defaultInstance;

  /// Feed ids table
  @$pb.TagNumber(1)
  FeedIdsTable get feeds => $_getN(0);
  @$pb.TagNumber(1)
  set feeds(FeedIdsTable v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasFeeds() => $_has(0);
  @$pb.TagNumber(1)
  void clearFeeds() => clearField(1);
  @$pb.TagNumber(1)
  FeedIdsTable ensureFeeds() => $_ensure(0);
}

/// Feed response message
class FeedResponseMessage extends $pb.GeneratedMessage {
  factory FeedResponseMessage({
    FeedResponseTable? feeds,
  }) {
    final $result = create();
    if (feeds != null) {
      $result.feeds = feeds;
    }
    return $result;
  }
  FeedResponseMessage._() : super();
  factory FeedResponseMessage.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FeedResponseMessage.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'FeedResponseMessage', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.router_net_info'), createEmptyInstance: create)
    ..aOM<FeedResponseTable>(1, _omitFieldNames ? '' : 'feeds', subBuilder: FeedResponseTable.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  FeedResponseMessage clone() => FeedResponseMessage()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  FeedResponseMessage copyWith(void Function(FeedResponseMessage) updates) => super.copyWith((message) => updates(message as FeedResponseMessage)) as FeedResponseMessage;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FeedResponseMessage create() => FeedResponseMessage._();
  FeedResponseMessage createEmptyInstance() => create();
  static $pb.PbList<FeedResponseMessage> createRepeated() => $pb.PbList<FeedResponseMessage>();
  @$core.pragma('dart2js:noInline')
  static FeedResponseMessage getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FeedResponseMessage>(create);
  static FeedResponseMessage? _defaultInstance;

  /// Feed table
  @$pb.TagNumber(1)
  FeedResponseTable get feeds => $_getN(0);
  @$pb.TagNumber(1)
  set feeds(FeedResponseTable v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasFeeds() => $_has(0);
  @$pb.TagNumber(1)
  void clearFeeds() => clearField(1);
  @$pb.TagNumber(1)
  FeedResponseTable ensureFeeds() => $_ensure(0);
}

/// Feed response table
/// containing the feed messages for response
class FeedResponseTable extends $pb.GeneratedMessage {
  factory FeedResponseTable({
    $core.Iterable<FeedMessage>? messages,
  }) {
    final $result = create();
    if (messages != null) {
      $result.messages.addAll(messages);
    }
    return $result;
  }
  FeedResponseTable._() : super();
  factory FeedResponseTable.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FeedResponseTable.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'FeedResponseTable', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.router_net_info'), createEmptyInstance: create)
    ..pc<FeedMessage>(1, _omitFieldNames ? '' : 'messages', $pb.PbFieldType.PM, subBuilder: FeedMessage.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  FeedResponseTable clone() => FeedResponseTable()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  FeedResponseTable copyWith(void Function(FeedResponseTable) updates) => super.copyWith((message) => updates(message as FeedResponseTable)) as FeedResponseTable;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FeedResponseTable create() => FeedResponseTable._();
  FeedResponseTable createEmptyInstance() => create();
  static $pb.PbList<FeedResponseTable> createRepeated() => $pb.PbList<FeedResponseTable>();
  @$core.pragma('dart2js:noInline')
  static FeedResponseTable getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FeedResponseTable>(create);
  static FeedResponseTable? _defaultInstance;

  /// feed messages
  @$pb.TagNumber(1)
  $core.List<FeedMessage> get messages => $_getList(0);
}

/// Feed Message
class FeedMessage extends $pb.GeneratedMessage {
  factory FeedMessage({
    $core.List<$core.int>? messageId,
    $core.List<$core.int>? senderId,
    $core.String? content,
    $fixnum.Int64? time,
  }) {
    final $result = create();
    if (messageId != null) {
      $result.messageId = messageId;
    }
    if (senderId != null) {
      $result.senderId = senderId;
    }
    if (content != null) {
      $result.content = content;
    }
    if (time != null) {
      $result.time = time;
    }
    return $result;
  }
  FeedMessage._() : super();
  factory FeedMessage.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FeedMessage.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'FeedMessage', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.router_net_info'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'messageId', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(2, _omitFieldNames ? '' : 'senderId', $pb.PbFieldType.OY)
    ..aOS(3, _omitFieldNames ? '' : 'content')
    ..a<$fixnum.Int64>(4, _omitFieldNames ? '' : 'time', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  FeedMessage clone() => FeedMessage()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  FeedMessage copyWith(void Function(FeedMessage) updates) => super.copyWith((message) => updates(message as FeedMessage)) as FeedMessage;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FeedMessage create() => FeedMessage._();
  FeedMessage createEmptyInstance() => create();
  static $pb.PbList<FeedMessage> createRepeated() => $pb.PbList<FeedMessage>();
  @$core.pragma('dart2js:noInline')
  static FeedMessage getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FeedMessage>(create);
  static FeedMessage? _defaultInstance;

  /// message id
  @$pb.TagNumber(1)
  $core.List<$core.int> get messageId => $_getN(0);
  @$pb.TagNumber(1)
  set messageId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasMessageId() => $_has(0);
  @$pb.TagNumber(1)
  void clearMessageId() => clearField(1);

  /// sender id
  @$pb.TagNumber(2)
  $core.List<$core.int> get senderId => $_getN(1);
  @$pb.TagNumber(2)
  set senderId($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasSenderId() => $_has(1);
  @$pb.TagNumber(2)
  void clearSenderId() => clearField(2);

  /// message content
  @$pb.TagNumber(3)
  $core.String get content => $_getSZ(2);
  @$pb.TagNumber(3)
  set content($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasContent() => $_has(2);
  @$pb.TagNumber(3)
  void clearContent() => clearField(3);

  /// timestamp in milli seconds
  @$pb.TagNumber(4)
  $fixnum.Int64 get time => $_getI64(3);
  @$pb.TagNumber(4)
  set time($fixnum.Int64 v) { $_setInt64(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasTime() => $_has(3);
  @$pb.TagNumber(4)
  void clearTime() => clearField(4);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
