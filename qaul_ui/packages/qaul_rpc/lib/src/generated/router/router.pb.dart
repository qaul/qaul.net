// This is a generated file - do not edit.
//
// Generated from router/router.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'router.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'router.pbenum.dart';

enum Router_Message {
  routingTableRequest,
  routingTable,
  connectionsRequest,
  connectionsList,
  neighboursRequest,
  neighboursList,
  notSet
}

/// router rpc message container
class Router extends $pb.GeneratedMessage {
  factory Router({
    RoutingTableRequest? routingTableRequest,
    RoutingTableList? routingTable,
    ConnectionsRequest? connectionsRequest,
    ConnectionsList? connectionsList,
    NeighboursRequest? neighboursRequest,
    NeighboursList? neighboursList,
  }) {
    final result = create();
    if (routingTableRequest != null)
      result.routingTableRequest = routingTableRequest;
    if (routingTable != null) result.routingTable = routingTable;
    if (connectionsRequest != null)
      result.connectionsRequest = connectionsRequest;
    if (connectionsList != null) result.connectionsList = connectionsList;
    if (neighboursRequest != null) result.neighboursRequest = neighboursRequest;
    if (neighboursList != null) result.neighboursList = neighboursList;
    return result;
  }

  Router._();

  factory Router.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory Router.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, Router_Message> _Router_MessageByTag = {
    1: Router_Message.routingTableRequest,
    2: Router_Message.routingTable,
    3: Router_Message.connectionsRequest,
    4: Router_Message.connectionsList,
    5: Router_Message.neighboursRequest,
    6: Router_Message.neighboursList,
    0: Router_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Router',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.router'),
      createEmptyInstance: create)
    ..oo(0, [1, 2, 3, 4, 5, 6])
    ..aOM<RoutingTableRequest>(1, _omitFieldNames ? '' : 'routingTableRequest',
        subBuilder: RoutingTableRequest.create)
    ..aOM<RoutingTableList>(2, _omitFieldNames ? '' : 'routingTable',
        subBuilder: RoutingTableList.create)
    ..aOM<ConnectionsRequest>(3, _omitFieldNames ? '' : 'connectionsRequest',
        subBuilder: ConnectionsRequest.create)
    ..aOM<ConnectionsList>(4, _omitFieldNames ? '' : 'connectionsList',
        subBuilder: ConnectionsList.create)
    ..aOM<NeighboursRequest>(5, _omitFieldNames ? '' : 'neighboursRequest',
        subBuilder: NeighboursRequest.create)
    ..aOM<NeighboursList>(6, _omitFieldNames ? '' : 'neighboursList',
        subBuilder: NeighboursList.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Router clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Router copyWith(void Function(Router) updates) =>
      super.copyWith((message) => updates(message as Router)) as Router;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Router create() => Router._();
  @$core.override
  Router createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static Router getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Router>(create);
  static Router? _defaultInstance;

  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  @$pb.TagNumber(6)
  Router_Message whichMessage() => _Router_MessageByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  @$pb.TagNumber(6)
  void clearMessage() => $_clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  RoutingTableRequest get routingTableRequest => $_getN(0);
  @$pb.TagNumber(1)
  set routingTableRequest(RoutingTableRequest value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasRoutingTableRequest() => $_has(0);
  @$pb.TagNumber(1)
  void clearRoutingTableRequest() => $_clearField(1);
  @$pb.TagNumber(1)
  RoutingTableRequest ensureRoutingTableRequest() => $_ensure(0);

  @$pb.TagNumber(2)
  RoutingTableList get routingTable => $_getN(1);
  @$pb.TagNumber(2)
  set routingTable(RoutingTableList value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasRoutingTable() => $_has(1);
  @$pb.TagNumber(2)
  void clearRoutingTable() => $_clearField(2);
  @$pb.TagNumber(2)
  RoutingTableList ensureRoutingTable() => $_ensure(1);

  @$pb.TagNumber(3)
  ConnectionsRequest get connectionsRequest => $_getN(2);
  @$pb.TagNumber(3)
  set connectionsRequest(ConnectionsRequest value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasConnectionsRequest() => $_has(2);
  @$pb.TagNumber(3)
  void clearConnectionsRequest() => $_clearField(3);
  @$pb.TagNumber(3)
  ConnectionsRequest ensureConnectionsRequest() => $_ensure(2);

  @$pb.TagNumber(4)
  ConnectionsList get connectionsList => $_getN(3);
  @$pb.TagNumber(4)
  set connectionsList(ConnectionsList value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasConnectionsList() => $_has(3);
  @$pb.TagNumber(4)
  void clearConnectionsList() => $_clearField(4);
  @$pb.TagNumber(4)
  ConnectionsList ensureConnectionsList() => $_ensure(3);

  @$pb.TagNumber(5)
  NeighboursRequest get neighboursRequest => $_getN(4);
  @$pb.TagNumber(5)
  set neighboursRequest(NeighboursRequest value) => $_setField(5, value);
  @$pb.TagNumber(5)
  $core.bool hasNeighboursRequest() => $_has(4);
  @$pb.TagNumber(5)
  void clearNeighboursRequest() => $_clearField(5);
  @$pb.TagNumber(5)
  NeighboursRequest ensureNeighboursRequest() => $_ensure(4);

  @$pb.TagNumber(6)
  NeighboursList get neighboursList => $_getN(5);
  @$pb.TagNumber(6)
  set neighboursList(NeighboursList value) => $_setField(6, value);
  @$pb.TagNumber(6)
  $core.bool hasNeighboursList() => $_has(5);
  @$pb.TagNumber(6)
  void clearNeighboursList() => $_clearField(6);
  @$pb.TagNumber(6)
  NeighboursList ensureNeighboursList() => $_ensure(5);
}

/// UI request for routing table list
class RoutingTableRequest extends $pb.GeneratedMessage {
  factory RoutingTableRequest() => create();

  RoutingTableRequest._();

  factory RoutingTableRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory RoutingTableRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RoutingTableRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.router'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RoutingTableRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RoutingTableRequest copyWith(void Function(RoutingTableRequest) updates) =>
      super.copyWith((message) => updates(message as RoutingTableRequest))
          as RoutingTableRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RoutingTableRequest create() => RoutingTableRequest._();
  @$core.override
  RoutingTableRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static RoutingTableRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RoutingTableRequest>(create);
  static RoutingTableRequest? _defaultInstance;
}

/// Routing table list
/// This table presents the best view for each user.
/// It represents the decision the router takes
/// when sending and routing packages
class RoutingTableList extends $pb.GeneratedMessage {
  factory RoutingTableList({
    $core.Iterable<RoutingTableEntry>? routingTable,
  }) {
    final result = create();
    if (routingTable != null) result.routingTable.addAll(routingTable);
    return result;
  }

  RoutingTableList._();

  factory RoutingTableList.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory RoutingTableList.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RoutingTableList',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.router'),
      createEmptyInstance: create)
    ..pPM<RoutingTableEntry>(1, _omitFieldNames ? '' : 'routingTable',
        subBuilder: RoutingTableEntry.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RoutingTableList clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RoutingTableList copyWith(void Function(RoutingTableList) updates) =>
      super.copyWith((message) => updates(message as RoutingTableList))
          as RoutingTableList;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RoutingTableList create() => RoutingTableList._();
  @$core.override
  RoutingTableList createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static RoutingTableList getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RoutingTableList>(create);
  static RoutingTableList? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<RoutingTableEntry> get routingTable => $_getList(0);
}

/// Routing table user entry
/// This message contains the best connection to this
/// user per module
class RoutingTableEntry extends $pb.GeneratedMessage {
  factory RoutingTableEntry({
    $core.List<$core.int>? userId,
    $core.Iterable<RoutingTableConnection>? connections,
  }) {
    final result = create();
    if (userId != null) result.userId = userId;
    if (connections != null) result.connections.addAll(connections);
    return result;
  }

  RoutingTableEntry._();

  factory RoutingTableEntry.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory RoutingTableEntry.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RoutingTableEntry',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.router'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'userId', $pb.PbFieldType.OY)
    ..pPM<RoutingTableConnection>(2, _omitFieldNames ? '' : 'connections',
        subBuilder: RoutingTableConnection.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RoutingTableEntry clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RoutingTableEntry copyWith(void Function(RoutingTableEntry) updates) =>
      super.copyWith((message) => updates(message as RoutingTableEntry))
          as RoutingTableEntry;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RoutingTableEntry create() => RoutingTableEntry._();
  @$core.override
  RoutingTableEntry createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static RoutingTableEntry getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RoutingTableEntry>(create);
  static RoutingTableEntry? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get userId => $_getN(0);
  @$pb.TagNumber(1)
  set userId($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasUserId() => $_has(0);
  @$pb.TagNumber(1)
  void clearUserId() => $_clearField(1);

  @$pb.TagNumber(2)
  $pb.PbList<RoutingTableConnection> get connections => $_getList(1);
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
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.router'),
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

/// UI request for connections list
class ConnectionsRequest extends $pb.GeneratedMessage {
  factory ConnectionsRequest() => create();

  ConnectionsRequest._();

  factory ConnectionsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ConnectionsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ConnectionsRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.router'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ConnectionsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ConnectionsRequest copyWith(void Function(ConnectionsRequest) updates) =>
      super.copyWith((message) => updates(message as ConnectionsRequest))
          as ConnectionsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ConnectionsRequest create() => ConnectionsRequest._();
  @$core.override
  ConnectionsRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ConnectionsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ConnectionsRequest>(create);
  static ConnectionsRequest? _defaultInstance;
}

/// Connections list per module.
/// All connections per user per module.
class ConnectionsList extends $pb.GeneratedMessage {
  factory ConnectionsList({
    $core.Iterable<ConnectionsUserEntry>? lan,
    $core.Iterable<ConnectionsUserEntry>? internet,
    $core.Iterable<ConnectionsUserEntry>? ble,
    $core.Iterable<ConnectionsUserEntry>? local,
  }) {
    final result = create();
    if (lan != null) result.lan.addAll(lan);
    if (internet != null) result.internet.addAll(internet);
    if (ble != null) result.ble.addAll(ble);
    if (local != null) result.local.addAll(local);
    return result;
  }

  ConnectionsList._();

  factory ConnectionsList.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ConnectionsList.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ConnectionsList',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.router'),
      createEmptyInstance: create)
    ..pPM<ConnectionsUserEntry>(1, _omitFieldNames ? '' : 'lan',
        subBuilder: ConnectionsUserEntry.create)
    ..pPM<ConnectionsUserEntry>(2, _omitFieldNames ? '' : 'internet',
        subBuilder: ConnectionsUserEntry.create)
    ..pPM<ConnectionsUserEntry>(3, _omitFieldNames ? '' : 'ble',
        subBuilder: ConnectionsUserEntry.create)
    ..pPM<ConnectionsUserEntry>(4, _omitFieldNames ? '' : 'local',
        subBuilder: ConnectionsUserEntry.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ConnectionsList clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ConnectionsList copyWith(void Function(ConnectionsList) updates) =>
      super.copyWith((message) => updates(message as ConnectionsList))
          as ConnectionsList;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ConnectionsList create() => ConnectionsList._();
  @$core.override
  ConnectionsList createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ConnectionsList getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ConnectionsList>(create);
  static ConnectionsList? _defaultInstance;

  /// users connected via the LAN module
  @$pb.TagNumber(1)
  $pb.PbList<ConnectionsUserEntry> get lan => $_getList(0);

  /// users connected via the Internet module
  @$pb.TagNumber(2)
  $pb.PbList<ConnectionsUserEntry> get internet => $_getList(1);

  /// users connected via the BLE module
  @$pb.TagNumber(3)
  $pb.PbList<ConnectionsUserEntry> get ble => $_getList(2);

  /// users connected locally (on the same node)
  @$pb.TagNumber(4)
  $pb.PbList<ConnectionsUserEntry> get local => $_getList(3);
}

/// connections entry for a user
class ConnectionsUserEntry extends $pb.GeneratedMessage {
  factory ConnectionsUserEntry({
    $core.List<$core.int>? userId,
    $core.Iterable<ConnectionEntry>? connections,
  }) {
    final result = create();
    if (userId != null) result.userId = userId;
    if (connections != null) result.connections.addAll(connections);
    return result;
  }

  ConnectionsUserEntry._();

  factory ConnectionsUserEntry.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ConnectionsUserEntry.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ConnectionsUserEntry',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.router'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'userId', $pb.PbFieldType.OY)
    ..pPM<ConnectionEntry>(2, _omitFieldNames ? '' : 'connections',
        subBuilder: ConnectionEntry.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ConnectionsUserEntry clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ConnectionsUserEntry copyWith(void Function(ConnectionsUserEntry) updates) =>
      super.copyWith((message) => updates(message as ConnectionsUserEntry))
          as ConnectionsUserEntry;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ConnectionsUserEntry create() => ConnectionsUserEntry._();
  @$core.override
  ConnectionsUserEntry createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ConnectionsUserEntry getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ConnectionsUserEntry>(create);
  static ConnectionsUserEntry? _defaultInstance;

  /// the id of the user
  @$pb.TagNumber(1)
  $core.List<$core.int> get userId => $_getN(0);
  @$pb.TagNumber(1)
  set userId($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasUserId() => $_has(0);
  @$pb.TagNumber(1)
  void clearUserId() => $_clearField(1);

  /// all connections to this user via this module
  @$pb.TagNumber(2)
  $pb.PbList<ConnectionEntry> get connections => $_getList(1);
}

/// all connections of this user
class ConnectionEntry extends $pb.GeneratedMessage {
  factory ConnectionEntry({
    $core.int? rtt,
    $core.int? hopCount,
    $core.List<$core.int>? via,
  }) {
    final result = create();
    if (rtt != null) result.rtt = rtt;
    if (hopCount != null) result.hopCount = hopCount;
    if (via != null) result.via = via;
    return result;
  }

  ConnectionEntry._();

  factory ConnectionEntry.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ConnectionEntry.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ConnectionEntry',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.router'),
      createEmptyInstance: create)
    ..aI(1, _omitFieldNames ? '' : 'rtt', fieldType: $pb.PbFieldType.OU3)
    ..aI(2, _omitFieldNames ? '' : 'hopCount', fieldType: $pb.PbFieldType.OU3)
    ..a<$core.List<$core.int>>(
        3, _omitFieldNames ? '' : 'via', $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ConnectionEntry clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ConnectionEntry copyWith(void Function(ConnectionEntry) updates) =>
      super.copyWith((message) => updates(message as ConnectionEntry))
          as ConnectionEntry;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ConnectionEntry create() => ConnectionEntry._();
  @$core.override
  ConnectionEntry createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ConnectionEntry getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ConnectionEntry>(create);
  static ConnectionEntry? _defaultInstance;

  /// round trip time in milli seconds
  @$pb.TagNumber(1)
  $core.int get rtt => $_getIZ(0);
  @$pb.TagNumber(1)
  set rtt($core.int value) => $_setUnsignedInt32(0, value);
  @$pb.TagNumber(1)
  $core.bool hasRtt() => $_has(0);
  @$pb.TagNumber(1)
  void clearRtt() => $_clearField(1);

  /// hop count to the user.
  /// This represents the number of nodes between this node and the user.
  @$pb.TagNumber(2)
  $core.int get hopCount => $_getIZ(1);
  @$pb.TagNumber(2)
  set hopCount($core.int value) => $_setUnsignedInt32(1, value);
  @$pb.TagNumber(2)
  $core.bool hasHopCount() => $_has(1);
  @$pb.TagNumber(2)
  void clearHopCount() => $_clearField(2);

  /// connection can be established via the node with the following id
  @$pb.TagNumber(3)
  $core.List<$core.int> get via => $_getN(2);
  @$pb.TagNumber(3)
  set via($core.List<$core.int> value) => $_setBytes(2, value);
  @$pb.TagNumber(3)
  $core.bool hasVia() => $_has(2);
  @$pb.TagNumber(3)
  void clearVia() => $_clearField(3);
}

/// UI request for neighbours list
class NeighboursRequest extends $pb.GeneratedMessage {
  factory NeighboursRequest() => create();

  NeighboursRequest._();

  factory NeighboursRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory NeighboursRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'NeighboursRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.router'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NeighboursRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NeighboursRequest copyWith(void Function(NeighboursRequest) updates) =>
      super.copyWith((message) => updates(message as NeighboursRequest))
          as NeighboursRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NeighboursRequest create() => NeighboursRequest._();
  @$core.override
  NeighboursRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static NeighboursRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<NeighboursRequest>(create);
  static NeighboursRequest? _defaultInstance;
}

/// neighbours list per module
class NeighboursList extends $pb.GeneratedMessage {
  factory NeighboursList({
    $core.Iterable<NeighboursEntry>? lan,
    $core.Iterable<NeighboursEntry>? internet,
    $core.Iterable<NeighboursEntry>? ble,
  }) {
    final result = create();
    if (lan != null) result.lan.addAll(lan);
    if (internet != null) result.internet.addAll(internet);
    if (ble != null) result.ble.addAll(ble);
    return result;
  }

  NeighboursList._();

  factory NeighboursList.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory NeighboursList.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'NeighboursList',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.router'),
      createEmptyInstance: create)
    ..pPM<NeighboursEntry>(1, _omitFieldNames ? '' : 'lan',
        subBuilder: NeighboursEntry.create)
    ..pPM<NeighboursEntry>(2, _omitFieldNames ? '' : 'internet',
        subBuilder: NeighboursEntry.create)
    ..pPM<NeighboursEntry>(3, _omitFieldNames ? '' : 'ble',
        subBuilder: NeighboursEntry.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NeighboursList clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NeighboursList copyWith(void Function(NeighboursList) updates) =>
      super.copyWith((message) => updates(message as NeighboursList))
          as NeighboursList;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NeighboursList create() => NeighboursList._();
  @$core.override
  NeighboursList createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static NeighboursList getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<NeighboursList>(create);
  static NeighboursList? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<NeighboursEntry> get lan => $_getList(0);

  @$pb.TagNumber(2)
  $pb.PbList<NeighboursEntry> get internet => $_getList(1);

  @$pb.TagNumber(3)
  $pb.PbList<NeighboursEntry> get ble => $_getList(2);
}

/// neighbours entry
class NeighboursEntry extends $pb.GeneratedMessage {
  factory NeighboursEntry({
    $core.List<$core.int>? nodeId,
    $core.int? rtt,
  }) {
    final result = create();
    if (nodeId != null) result.nodeId = nodeId;
    if (rtt != null) result.rtt = rtt;
    return result;
  }

  NeighboursEntry._();

  factory NeighboursEntry.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory NeighboursEntry.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'NeighboursEntry',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.router'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'nodeId', $pb.PbFieldType.OY)
    ..aI(2, _omitFieldNames ? '' : 'rtt', fieldType: $pb.PbFieldType.OU3)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NeighboursEntry clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NeighboursEntry copyWith(void Function(NeighboursEntry) updates) =>
      super.copyWith((message) => updates(message as NeighboursEntry))
          as NeighboursEntry;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NeighboursEntry create() => NeighboursEntry._();
  @$core.override
  NeighboursEntry createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static NeighboursEntry getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<NeighboursEntry>(create);
  static NeighboursEntry? _defaultInstance;

  /// the ID of the neighbour node
  @$pb.TagNumber(1)
  $core.List<$core.int> get nodeId => $_getN(0);
  @$pb.TagNumber(1)
  set nodeId($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasNodeId() => $_has(0);
  @$pb.TagNumber(1)
  void clearNodeId() => $_clearField(1);

  /// rtt to this neighbour
  @$pb.TagNumber(2)
  $core.int get rtt => $_getIZ(1);
  @$pb.TagNumber(2)
  set rtt($core.int value) => $_setUnsignedInt32(1, value);
  @$pb.TagNumber(2)
  $core.bool hasRtt() => $_has(1);
  @$pb.TagNumber(2)
  void clearRtt() => $_clearField(2);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
