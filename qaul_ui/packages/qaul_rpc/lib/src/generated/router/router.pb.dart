//
//  Generated code. Do not modify.
//  source: router/router.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'router.pbenum.dart';

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

class Router extends $pb.GeneratedMessage {
  factory Router() => create();
  Router._() : super();
  factory Router.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Router.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static const $core.Map<$core.int, Router_Message> _Router_MessageByTag = {
    1 : Router_Message.routingTableRequest,
    2 : Router_Message.routingTable,
    3 : Router_Message.connectionsRequest,
    4 : Router_Message.connectionsList,
    5 : Router_Message.neighboursRequest,
    6 : Router_Message.neighboursList,
    0 : Router_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Router', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.router'), createEmptyInstance: create)
    ..oo(0, [1, 2, 3, 4, 5, 6])
    ..aOM<RoutingTableRequest>(1, _omitFieldNames ? '' : 'routingTableRequest', subBuilder: RoutingTableRequest.create)
    ..aOM<RoutingTableList>(2, _omitFieldNames ? '' : 'routingTable', subBuilder: RoutingTableList.create)
    ..aOM<ConnectionsRequest>(3, _omitFieldNames ? '' : 'connectionsRequest', subBuilder: ConnectionsRequest.create)
    ..aOM<ConnectionsList>(4, _omitFieldNames ? '' : 'connectionsList', subBuilder: ConnectionsList.create)
    ..aOM<NeighboursRequest>(5, _omitFieldNames ? '' : 'neighboursRequest', subBuilder: NeighboursRequest.create)
    ..aOM<NeighboursList>(6, _omitFieldNames ? '' : 'neighboursList', subBuilder: NeighboursList.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Router clone() => Router()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Router copyWith(void Function(Router) updates) => super.copyWith((message) => updates(message as Router)) as Router;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Router create() => Router._();
  Router createEmptyInstance() => create();
  static $pb.PbList<Router> createRepeated() => $pb.PbList<Router>();
  @$core.pragma('dart2js:noInline')
  static Router getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Router>(create);
  static Router? _defaultInstance;

  Router_Message whichMessage() => _Router_MessageByTag[$_whichOneof(0)]!;
  void clearMessage() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  RoutingTableRequest get routingTableRequest => $_getN(0);
  @$pb.TagNumber(1)
  set routingTableRequest(RoutingTableRequest v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasRoutingTableRequest() => $_has(0);
  @$pb.TagNumber(1)
  void clearRoutingTableRequest() => clearField(1);
  @$pb.TagNumber(1)
  RoutingTableRequest ensureRoutingTableRequest() => $_ensure(0);

  @$pb.TagNumber(2)
  RoutingTableList get routingTable => $_getN(1);
  @$pb.TagNumber(2)
  set routingTable(RoutingTableList v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasRoutingTable() => $_has(1);
  @$pb.TagNumber(2)
  void clearRoutingTable() => clearField(2);
  @$pb.TagNumber(2)
  RoutingTableList ensureRoutingTable() => $_ensure(1);

  @$pb.TagNumber(3)
  ConnectionsRequest get connectionsRequest => $_getN(2);
  @$pb.TagNumber(3)
  set connectionsRequest(ConnectionsRequest v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasConnectionsRequest() => $_has(2);
  @$pb.TagNumber(3)
  void clearConnectionsRequest() => clearField(3);
  @$pb.TagNumber(3)
  ConnectionsRequest ensureConnectionsRequest() => $_ensure(2);

  @$pb.TagNumber(4)
  ConnectionsList get connectionsList => $_getN(3);
  @$pb.TagNumber(4)
  set connectionsList(ConnectionsList v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasConnectionsList() => $_has(3);
  @$pb.TagNumber(4)
  void clearConnectionsList() => clearField(4);
  @$pb.TagNumber(4)
  ConnectionsList ensureConnectionsList() => $_ensure(3);

  @$pb.TagNumber(5)
  NeighboursRequest get neighboursRequest => $_getN(4);
  @$pb.TagNumber(5)
  set neighboursRequest(NeighboursRequest v) { setField(5, v); }
  @$pb.TagNumber(5)
  $core.bool hasNeighboursRequest() => $_has(4);
  @$pb.TagNumber(5)
  void clearNeighboursRequest() => clearField(5);
  @$pb.TagNumber(5)
  NeighboursRequest ensureNeighboursRequest() => $_ensure(4);

  @$pb.TagNumber(6)
  NeighboursList get neighboursList => $_getN(5);
  @$pb.TagNumber(6)
  set neighboursList(NeighboursList v) { setField(6, v); }
  @$pb.TagNumber(6)
  $core.bool hasNeighboursList() => $_has(5);
  @$pb.TagNumber(6)
  void clearNeighboursList() => clearField(6);
  @$pb.TagNumber(6)
  NeighboursList ensureNeighboursList() => $_ensure(5);
}

class RoutingTableRequest extends $pb.GeneratedMessage {
  factory RoutingTableRequest() => create();
  RoutingTableRequest._() : super();
  factory RoutingTableRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RoutingTableRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'RoutingTableRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.router'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RoutingTableRequest clone() => RoutingTableRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RoutingTableRequest copyWith(void Function(RoutingTableRequest) updates) => super.copyWith((message) => updates(message as RoutingTableRequest)) as RoutingTableRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RoutingTableRequest create() => RoutingTableRequest._();
  RoutingTableRequest createEmptyInstance() => create();
  static $pb.PbList<RoutingTableRequest> createRepeated() => $pb.PbList<RoutingTableRequest>();
  @$core.pragma('dart2js:noInline')
  static RoutingTableRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RoutingTableRequest>(create);
  static RoutingTableRequest? _defaultInstance;
}

class RoutingTableList extends $pb.GeneratedMessage {
  factory RoutingTableList() => create();
  RoutingTableList._() : super();
  factory RoutingTableList.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RoutingTableList.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'RoutingTableList', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.router'), createEmptyInstance: create)
    ..pc<RoutingTableEntry>(1, _omitFieldNames ? '' : 'routingTable', $pb.PbFieldType.PM, subBuilder: RoutingTableEntry.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RoutingTableList clone() => RoutingTableList()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RoutingTableList copyWith(void Function(RoutingTableList) updates) => super.copyWith((message) => updates(message as RoutingTableList)) as RoutingTableList;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RoutingTableList create() => RoutingTableList._();
  RoutingTableList createEmptyInstance() => create();
  static $pb.PbList<RoutingTableList> createRepeated() => $pb.PbList<RoutingTableList>();
  @$core.pragma('dart2js:noInline')
  static RoutingTableList getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RoutingTableList>(create);
  static RoutingTableList? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<RoutingTableEntry> get routingTable => $_getList(0);
}

class RoutingTableEntry extends $pb.GeneratedMessage {
  factory RoutingTableEntry() => create();
  RoutingTableEntry._() : super();
  factory RoutingTableEntry.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RoutingTableEntry.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'RoutingTableEntry', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.router'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'userId', $pb.PbFieldType.OY)
    ..pc<RoutingTableConnection>(2, _omitFieldNames ? '' : 'connections', $pb.PbFieldType.PM, subBuilder: RoutingTableConnection.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RoutingTableEntry clone() => RoutingTableEntry()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RoutingTableEntry copyWith(void Function(RoutingTableEntry) updates) => super.copyWith((message) => updates(message as RoutingTableEntry)) as RoutingTableEntry;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RoutingTableEntry create() => RoutingTableEntry._();
  RoutingTableEntry createEmptyInstance() => create();
  static $pb.PbList<RoutingTableEntry> createRepeated() => $pb.PbList<RoutingTableEntry>();
  @$core.pragma('dart2js:noInline')
  static RoutingTableEntry getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RoutingTableEntry>(create);
  static RoutingTableEntry? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get userId => $_getN(0);
  @$pb.TagNumber(1)
  set userId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasUserId() => $_has(0);
  @$pb.TagNumber(1)
  void clearUserId() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<RoutingTableConnection> get connections => $_getList(1);
}

class RoutingTableConnection extends $pb.GeneratedMessage {
  factory RoutingTableConnection() => create();
  RoutingTableConnection._() : super();
  factory RoutingTableConnection.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RoutingTableConnection.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'RoutingTableConnection', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.router'), createEmptyInstance: create)
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

class ConnectionsRequest extends $pb.GeneratedMessage {
  factory ConnectionsRequest() => create();
  ConnectionsRequest._() : super();
  factory ConnectionsRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ConnectionsRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ConnectionsRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.router'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ConnectionsRequest clone() => ConnectionsRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ConnectionsRequest copyWith(void Function(ConnectionsRequest) updates) => super.copyWith((message) => updates(message as ConnectionsRequest)) as ConnectionsRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ConnectionsRequest create() => ConnectionsRequest._();
  ConnectionsRequest createEmptyInstance() => create();
  static $pb.PbList<ConnectionsRequest> createRepeated() => $pb.PbList<ConnectionsRequest>();
  @$core.pragma('dart2js:noInline')
  static ConnectionsRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ConnectionsRequest>(create);
  static ConnectionsRequest? _defaultInstance;
}

class ConnectionsList extends $pb.GeneratedMessage {
  factory ConnectionsList() => create();
  ConnectionsList._() : super();
  factory ConnectionsList.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ConnectionsList.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ConnectionsList', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.router'), createEmptyInstance: create)
    ..pc<ConnectionsUserEntry>(1, _omitFieldNames ? '' : 'lan', $pb.PbFieldType.PM, subBuilder: ConnectionsUserEntry.create)
    ..pc<ConnectionsUserEntry>(2, _omitFieldNames ? '' : 'internet', $pb.PbFieldType.PM, subBuilder: ConnectionsUserEntry.create)
    ..pc<ConnectionsUserEntry>(3, _omitFieldNames ? '' : 'ble', $pb.PbFieldType.PM, subBuilder: ConnectionsUserEntry.create)
    ..pc<ConnectionsUserEntry>(4, _omitFieldNames ? '' : 'local', $pb.PbFieldType.PM, subBuilder: ConnectionsUserEntry.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ConnectionsList clone() => ConnectionsList()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ConnectionsList copyWith(void Function(ConnectionsList) updates) => super.copyWith((message) => updates(message as ConnectionsList)) as ConnectionsList;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ConnectionsList create() => ConnectionsList._();
  ConnectionsList createEmptyInstance() => create();
  static $pb.PbList<ConnectionsList> createRepeated() => $pb.PbList<ConnectionsList>();
  @$core.pragma('dart2js:noInline')
  static ConnectionsList getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ConnectionsList>(create);
  static ConnectionsList? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<ConnectionsUserEntry> get lan => $_getList(0);

  @$pb.TagNumber(2)
  $core.List<ConnectionsUserEntry> get internet => $_getList(1);

  @$pb.TagNumber(3)
  $core.List<ConnectionsUserEntry> get ble => $_getList(2);

  @$pb.TagNumber(4)
  $core.List<ConnectionsUserEntry> get local => $_getList(3);
}

class ConnectionsUserEntry extends $pb.GeneratedMessage {
  factory ConnectionsUserEntry() => create();
  ConnectionsUserEntry._() : super();
  factory ConnectionsUserEntry.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ConnectionsUserEntry.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ConnectionsUserEntry', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.router'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'userId', $pb.PbFieldType.OY)
    ..pc<ConnectionEntry>(2, _omitFieldNames ? '' : 'connections', $pb.PbFieldType.PM, subBuilder: ConnectionEntry.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ConnectionsUserEntry clone() => ConnectionsUserEntry()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ConnectionsUserEntry copyWith(void Function(ConnectionsUserEntry) updates) => super.copyWith((message) => updates(message as ConnectionsUserEntry)) as ConnectionsUserEntry;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ConnectionsUserEntry create() => ConnectionsUserEntry._();
  ConnectionsUserEntry createEmptyInstance() => create();
  static $pb.PbList<ConnectionsUserEntry> createRepeated() => $pb.PbList<ConnectionsUserEntry>();
  @$core.pragma('dart2js:noInline')
  static ConnectionsUserEntry getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ConnectionsUserEntry>(create);
  static ConnectionsUserEntry? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get userId => $_getN(0);
  @$pb.TagNumber(1)
  set userId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasUserId() => $_has(0);
  @$pb.TagNumber(1)
  void clearUserId() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<ConnectionEntry> get connections => $_getList(1);
}

class ConnectionEntry extends $pb.GeneratedMessage {
  factory ConnectionEntry() => create();
  ConnectionEntry._() : super();
  factory ConnectionEntry.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ConnectionEntry.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ConnectionEntry', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.router'), createEmptyInstance: create)
    ..a<$core.int>(1, _omitFieldNames ? '' : 'rtt', $pb.PbFieldType.OU3)
    ..a<$core.int>(2, _omitFieldNames ? '' : 'hopCount', $pb.PbFieldType.OU3)
    ..a<$core.List<$core.int>>(3, _omitFieldNames ? '' : 'via', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ConnectionEntry clone() => ConnectionEntry()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ConnectionEntry copyWith(void Function(ConnectionEntry) updates) => super.copyWith((message) => updates(message as ConnectionEntry)) as ConnectionEntry;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ConnectionEntry create() => ConnectionEntry._();
  ConnectionEntry createEmptyInstance() => create();
  static $pb.PbList<ConnectionEntry> createRepeated() => $pb.PbList<ConnectionEntry>();
  @$core.pragma('dart2js:noInline')
  static ConnectionEntry getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ConnectionEntry>(create);
  static ConnectionEntry? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get rtt => $_getIZ(0);
  @$pb.TagNumber(1)
  set rtt($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasRtt() => $_has(0);
  @$pb.TagNumber(1)
  void clearRtt() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get hopCount => $_getIZ(1);
  @$pb.TagNumber(2)
  set hopCount($core.int v) { $_setUnsignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasHopCount() => $_has(1);
  @$pb.TagNumber(2)
  void clearHopCount() => clearField(2);

  @$pb.TagNumber(3)
  $core.List<$core.int> get via => $_getN(2);
  @$pb.TagNumber(3)
  set via($core.List<$core.int> v) { $_setBytes(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasVia() => $_has(2);
  @$pb.TagNumber(3)
  void clearVia() => clearField(3);
}

class NeighboursRequest extends $pb.GeneratedMessage {
  factory NeighboursRequest() => create();
  NeighboursRequest._() : super();
  factory NeighboursRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NeighboursRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'NeighboursRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.router'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NeighboursRequest clone() => NeighboursRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NeighboursRequest copyWith(void Function(NeighboursRequest) updates) => super.copyWith((message) => updates(message as NeighboursRequest)) as NeighboursRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NeighboursRequest create() => NeighboursRequest._();
  NeighboursRequest createEmptyInstance() => create();
  static $pb.PbList<NeighboursRequest> createRepeated() => $pb.PbList<NeighboursRequest>();
  @$core.pragma('dart2js:noInline')
  static NeighboursRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NeighboursRequest>(create);
  static NeighboursRequest? _defaultInstance;
}

class NeighboursList extends $pb.GeneratedMessage {
  factory NeighboursList() => create();
  NeighboursList._() : super();
  factory NeighboursList.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NeighboursList.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'NeighboursList', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.router'), createEmptyInstance: create)
    ..pc<NeighboursEntry>(1, _omitFieldNames ? '' : 'lan', $pb.PbFieldType.PM, subBuilder: NeighboursEntry.create)
    ..pc<NeighboursEntry>(2, _omitFieldNames ? '' : 'internet', $pb.PbFieldType.PM, subBuilder: NeighboursEntry.create)
    ..pc<NeighboursEntry>(3, _omitFieldNames ? '' : 'ble', $pb.PbFieldType.PM, subBuilder: NeighboursEntry.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NeighboursList clone() => NeighboursList()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NeighboursList copyWith(void Function(NeighboursList) updates) => super.copyWith((message) => updates(message as NeighboursList)) as NeighboursList;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NeighboursList create() => NeighboursList._();
  NeighboursList createEmptyInstance() => create();
  static $pb.PbList<NeighboursList> createRepeated() => $pb.PbList<NeighboursList>();
  @$core.pragma('dart2js:noInline')
  static NeighboursList getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NeighboursList>(create);
  static NeighboursList? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<NeighboursEntry> get lan => $_getList(0);

  @$pb.TagNumber(2)
  $core.List<NeighboursEntry> get internet => $_getList(1);

  @$pb.TagNumber(3)
  $core.List<NeighboursEntry> get ble => $_getList(2);
}

class NeighboursEntry extends $pb.GeneratedMessage {
  factory NeighboursEntry() => create();
  NeighboursEntry._() : super();
  factory NeighboursEntry.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NeighboursEntry.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'NeighboursEntry', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.router'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'nodeId', $pb.PbFieldType.OY)
    ..a<$core.int>(2, _omitFieldNames ? '' : 'rtt', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NeighboursEntry clone() => NeighboursEntry()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NeighboursEntry copyWith(void Function(NeighboursEntry) updates) => super.copyWith((message) => updates(message as NeighboursEntry)) as NeighboursEntry;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NeighboursEntry create() => NeighboursEntry._();
  NeighboursEntry createEmptyInstance() => create();
  static $pb.PbList<NeighboursEntry> createRepeated() => $pb.PbList<NeighboursEntry>();
  @$core.pragma('dart2js:noInline')
  static NeighboursEntry getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NeighboursEntry>(create);
  static NeighboursEntry? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get nodeId => $_getN(0);
  @$pb.TagNumber(1)
  set nodeId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasNodeId() => $_has(0);
  @$pb.TagNumber(1)
  void clearNodeId() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get rtt => $_getIZ(1);
  @$pb.TagNumber(2)
  set rtt($core.int v) { $_setUnsignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasRtt() => $_has(1);
  @$pb.TagNumber(2)
  void clearRtt() => clearField(2);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
