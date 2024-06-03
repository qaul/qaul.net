//
//  Generated code. Do not modify.
//  source: connections/connections.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'connections.pbenum.dart';

export 'connections.pbenum.dart';

enum Connections_Message {
  internetNodesRequest, 
  internetNodesList, 
  internetNodesAdd, 
  internetNodesRemove, 
  internetNodesState, 
  internetNodesRename, 
  notSet
}

/// Connections rpc message container
class Connections extends $pb.GeneratedMessage {
  factory Connections({
    InternetNodesRequest? internetNodesRequest,
    InternetNodesList? internetNodesList,
    InternetNodesEntry? internetNodesAdd,
    InternetNodesEntry? internetNodesRemove,
    InternetNodesEntry? internetNodesState,
    InternetNodesEntry? internetNodesRename,
  }) {
    final $result = create();
    if (internetNodesRequest != null) {
      $result.internetNodesRequest = internetNodesRequest;
    }
    if (internetNodesList != null) {
      $result.internetNodesList = internetNodesList;
    }
    if (internetNodesAdd != null) {
      $result.internetNodesAdd = internetNodesAdd;
    }
    if (internetNodesRemove != null) {
      $result.internetNodesRemove = internetNodesRemove;
    }
    if (internetNodesState != null) {
      $result.internetNodesState = internetNodesState;
    }
    if (internetNodesRename != null) {
      $result.internetNodesRename = internetNodesRename;
    }
    return $result;
  }
  Connections._() : super();
  factory Connections.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Connections.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static const $core.Map<$core.int, Connections_Message> _Connections_MessageByTag = {
    1 : Connections_Message.internetNodesRequest,
    2 : Connections_Message.internetNodesList,
    3 : Connections_Message.internetNodesAdd,
    4 : Connections_Message.internetNodesRemove,
    5 : Connections_Message.internetNodesState,
    6 : Connections_Message.internetNodesRename,
    0 : Connections_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Connections', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.connections'), createEmptyInstance: create)
    ..oo(0, [1, 2, 3, 4, 5, 6])
    ..aOM<InternetNodesRequest>(1, _omitFieldNames ? '' : 'internetNodesRequest', subBuilder: InternetNodesRequest.create)
    ..aOM<InternetNodesList>(2, _omitFieldNames ? '' : 'internetNodesList', subBuilder: InternetNodesList.create)
    ..aOM<InternetNodesEntry>(3, _omitFieldNames ? '' : 'internetNodesAdd', subBuilder: InternetNodesEntry.create)
    ..aOM<InternetNodesEntry>(4, _omitFieldNames ? '' : 'internetNodesRemove', subBuilder: InternetNodesEntry.create)
    ..aOM<InternetNodesEntry>(5, _omitFieldNames ? '' : 'internetNodesState', subBuilder: InternetNodesEntry.create)
    ..aOM<InternetNodesEntry>(6, _omitFieldNames ? '' : 'internetNodesRename', subBuilder: InternetNodesEntry.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Connections clone() => Connections()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Connections copyWith(void Function(Connections) updates) => super.copyWith((message) => updates(message as Connections)) as Connections;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Connections create() => Connections._();
  Connections createEmptyInstance() => create();
  static $pb.PbList<Connections> createRepeated() => $pb.PbList<Connections>();
  @$core.pragma('dart2js:noInline')
  static Connections getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Connections>(create);
  static Connections? _defaultInstance;

  Connections_Message whichMessage() => _Connections_MessageByTag[$_whichOneof(0)]!;
  void clearMessage() => clearField($_whichOneof(0));

  /// Request a list of all internet nodes.
  /// libqaul returns an internet_nodes_list message.
  @$pb.TagNumber(1)
  InternetNodesRequest get internetNodesRequest => $_getN(0);
  @$pb.TagNumber(1)
  set internetNodesRequest(InternetNodesRequest v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasInternetNodesRequest() => $_has(0);
  @$pb.TagNumber(1)
  void clearInternetNodesRequest() => clearField(1);
  @$pb.TagNumber(1)
  InternetNodesRequest ensureInternetNodesRequest() => $_ensure(0);

  /// returns a list of all internet nodes and
  /// an information about why this message has been sent.
  @$pb.TagNumber(2)
  InternetNodesList get internetNodesList => $_getN(1);
  @$pb.TagNumber(2)
  set internetNodesList(InternetNodesList v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasInternetNodesList() => $_has(1);
  @$pb.TagNumber(2)
  void clearInternetNodesList() => clearField(2);
  @$pb.TagNumber(2)
  InternetNodesList ensureInternetNodesList() => $_ensure(1);

  /// Add a new internet node address.
  /// libqaul returns an internet_nodes_list message.
  @$pb.TagNumber(3)
  InternetNodesEntry get internetNodesAdd => $_getN(2);
  @$pb.TagNumber(3)
  set internetNodesAdd(InternetNodesEntry v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasInternetNodesAdd() => $_has(2);
  @$pb.TagNumber(3)
  void clearInternetNodesAdd() => clearField(3);
  @$pb.TagNumber(3)
  InternetNodesEntry ensureInternetNodesAdd() => $_ensure(2);

  /// Remove an internet node address.
  /// libqaul returns an internet_nodes_list message.
  @$pb.TagNumber(4)
  InternetNodesEntry get internetNodesRemove => $_getN(3);
  @$pb.TagNumber(4)
  set internetNodesRemove(InternetNodesEntry v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasInternetNodesRemove() => $_has(3);
  @$pb.TagNumber(4)
  void clearInternetNodesRemove() => clearField(4);
  @$pb.TagNumber(4)
  InternetNodesEntry ensureInternetNodesRemove() => $_ensure(3);

  /// Update an internet node state.
  /// libqaul returns an internet_nodes_list message.
  @$pb.TagNumber(5)
  InternetNodesEntry get internetNodesState => $_getN(4);
  @$pb.TagNumber(5)
  set internetNodesState(InternetNodesEntry v) { setField(5, v); }
  @$pb.TagNumber(5)
  $core.bool hasInternetNodesState() => $_has(4);
  @$pb.TagNumber(5)
  void clearInternetNodesState() => clearField(5);
  @$pb.TagNumber(5)
  InternetNodesEntry ensureInternetNodesState() => $_ensure(4);

  /// Rename internet node.
  /// libqaul returns an internet_nodes_list message.
  @$pb.TagNumber(6)
  InternetNodesEntry get internetNodesRename => $_getN(5);
  @$pb.TagNumber(6)
  set internetNodesRename(InternetNodesEntry v) { setField(6, v); }
  @$pb.TagNumber(6)
  $core.bool hasInternetNodesRename() => $_has(5);
  @$pb.TagNumber(6)
  void clearInternetNodesRename() => clearField(6);
  @$pb.TagNumber(6)
  InternetNodesEntry ensureInternetNodesRename() => $_ensure(5);
}

/// UI request for Internet nodes list
class InternetNodesRequest extends $pb.GeneratedMessage {
  factory InternetNodesRequest() => create();
  InternetNodesRequest._() : super();
  factory InternetNodesRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory InternetNodesRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'InternetNodesRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.connections'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  InternetNodesRequest clone() => InternetNodesRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  InternetNodesRequest copyWith(void Function(InternetNodesRequest) updates) => super.copyWith((message) => updates(message as InternetNodesRequest)) as InternetNodesRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static InternetNodesRequest create() => InternetNodesRequest._();
  InternetNodesRequest createEmptyInstance() => create();
  static $pb.PbList<InternetNodesRequest> createRepeated() => $pb.PbList<InternetNodesRequest>();
  @$core.pragma('dart2js:noInline')
  static InternetNodesRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<InternetNodesRequest>(create);
  static InternetNodesRequest? _defaultInstance;
}

///  Internet Nodes List
///
///  This is a list of all peer nodes the internet
///  connections module tries to connect to.
///
///  This message is returned after a request, or when
///  adding or removing a node address.
class InternetNodesList extends $pb.GeneratedMessage {
  factory InternetNodesList({
    Info? info,
    $core.Iterable<InternetNodesEntry>? nodes,
  }) {
    final $result = create();
    if (info != null) {
      $result.info = info;
    }
    if (nodes != null) {
      $result.nodes.addAll(nodes);
    }
    return $result;
  }
  InternetNodesList._() : super();
  factory InternetNodesList.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory InternetNodesList.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'InternetNodesList', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.connections'), createEmptyInstance: create)
    ..e<Info>(1, _omitFieldNames ? '' : 'info', $pb.PbFieldType.OE, defaultOrMaker: Info.REQUEST, valueOf: Info.valueOf, enumValues: Info.values)
    ..pc<InternetNodesEntry>(2, _omitFieldNames ? '' : 'nodes', $pb.PbFieldType.PM, subBuilder: InternetNodesEntry.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  InternetNodesList clone() => InternetNodesList()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  InternetNodesList copyWith(void Function(InternetNodesList) updates) => super.copyWith((message) => updates(message as InternetNodesList)) as InternetNodesList;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static InternetNodesList create() => InternetNodesList._();
  InternetNodesList createEmptyInstance() => create();
  static $pb.PbList<InternetNodesList> createRepeated() => $pb.PbList<InternetNodesList>();
  @$core.pragma('dart2js:noInline')
  static InternetNodesList getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<InternetNodesList>(create);
  static InternetNodesList? _defaultInstance;

  /// Information about why this message is sent
  /// and the result of the request, adding or removing
  /// of nodes.
  @$pb.TagNumber(1)
  Info get info => $_getN(0);
  @$pb.TagNumber(1)
  set info(Info v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasInfo() => $_has(0);
  @$pb.TagNumber(1)
  void clearInfo() => clearField(1);

  /// list of all node multiaddresses that
  /// the internet module will try to connect to.
  @$pb.TagNumber(2)
  $core.List<InternetNodesEntry> get nodes => $_getList(1);
}

///  Internet Nodes Entry
///
///  Contains a node address as a libp2p multiaddress.
///  e.g. "/ip4/144.91.74.192/udp/9229/quic-v1"
class InternetNodesEntry extends $pb.GeneratedMessage {
  factory InternetNodesEntry({
    $core.String? address,
    $core.bool? enabled,
    $core.String? name,
  }) {
    final $result = create();
    if (address != null) {
      $result.address = address;
    }
    if (enabled != null) {
      $result.enabled = enabled;
    }
    if (name != null) {
      $result.name = name;
    }
    return $result;
  }
  InternetNodesEntry._() : super();
  factory InternetNodesEntry.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory InternetNodesEntry.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'InternetNodesEntry', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.connections'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'address')
    ..aOB(2, _omitFieldNames ? '' : 'enabled')
    ..aOS(3, _omitFieldNames ? '' : 'name')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  InternetNodesEntry clone() => InternetNodesEntry()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  InternetNodesEntry copyWith(void Function(InternetNodesEntry) updates) => super.copyWith((message) => updates(message as InternetNodesEntry)) as InternetNodesEntry;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static InternetNodesEntry create() => InternetNodesEntry._();
  InternetNodesEntry createEmptyInstance() => create();
  static $pb.PbList<InternetNodesEntry> createRepeated() => $pb.PbList<InternetNodesEntry>();
  @$core.pragma('dart2js:noInline')
  static InternetNodesEntry getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<InternetNodesEntry>(create);
  static InternetNodesEntry? _defaultInstance;

  /// address
  @$pb.TagNumber(1)
  $core.String get address => $_getSZ(0);
  @$pb.TagNumber(1)
  set address($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasAddress() => $_has(0);
  @$pb.TagNumber(1)
  void clearAddress() => clearField(1);

  /// enabled
  @$pb.TagNumber(2)
  $core.bool get enabled => $_getBF(1);
  @$pb.TagNumber(2)
  set enabled($core.bool v) { $_setBool(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasEnabled() => $_has(1);
  @$pb.TagNumber(2)
  void clearEnabled() => clearField(2);

  /// name
  @$pb.TagNumber(3)
  $core.String get name => $_getSZ(2);
  @$pb.TagNumber(3)
  set name($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasName() => $_has(2);
  @$pb.TagNumber(3)
  void clearName() => clearField(3);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
