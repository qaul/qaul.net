// This is a generated file - do not edit.
//
// Generated from connections/connections.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'connections.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

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
    final result = create();
    if (internetNodesRequest != null)
      result.internetNodesRequest = internetNodesRequest;
    if (internetNodesList != null) result.internetNodesList = internetNodesList;
    if (internetNodesAdd != null) result.internetNodesAdd = internetNodesAdd;
    if (internetNodesRemove != null)
      result.internetNodesRemove = internetNodesRemove;
    if (internetNodesState != null)
      result.internetNodesState = internetNodesState;
    if (internetNodesRename != null)
      result.internetNodesRename = internetNodesRename;
    return result;
  }

  Connections._();

  factory Connections.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory Connections.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, Connections_Message>
      _Connections_MessageByTag = {
    1: Connections_Message.internetNodesRequest,
    2: Connections_Message.internetNodesList,
    3: Connections_Message.internetNodesAdd,
    4: Connections_Message.internetNodesRemove,
    5: Connections_Message.internetNodesState,
    6: Connections_Message.internetNodesRename,
    0: Connections_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Connections',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'qaul.rpc.connections'),
      createEmptyInstance: create)
    ..oo(0, [1, 2, 3, 4, 5, 6])
    ..aOM<InternetNodesRequest>(
        1, _omitFieldNames ? '' : 'internetNodesRequest',
        subBuilder: InternetNodesRequest.create)
    ..aOM<InternetNodesList>(2, _omitFieldNames ? '' : 'internetNodesList',
        subBuilder: InternetNodesList.create)
    ..aOM<InternetNodesEntry>(3, _omitFieldNames ? '' : 'internetNodesAdd',
        subBuilder: InternetNodesEntry.create)
    ..aOM<InternetNodesEntry>(4, _omitFieldNames ? '' : 'internetNodesRemove',
        subBuilder: InternetNodesEntry.create)
    ..aOM<InternetNodesEntry>(5, _omitFieldNames ? '' : 'internetNodesState',
        subBuilder: InternetNodesEntry.create)
    ..aOM<InternetNodesEntry>(6, _omitFieldNames ? '' : 'internetNodesRename',
        subBuilder: InternetNodesEntry.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Connections clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Connections copyWith(void Function(Connections) updates) =>
      super.copyWith((message) => updates(message as Connections))
          as Connections;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Connections create() => Connections._();
  @$core.override
  Connections createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static Connections getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<Connections>(create);
  static Connections? _defaultInstance;

  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  @$pb.TagNumber(6)
  Connections_Message whichMessage() =>
      _Connections_MessageByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  @$pb.TagNumber(6)
  void clearMessage() => $_clearField($_whichOneof(0));

  /// Request a list of all internet nodes.
  /// libqaul returns an internet_nodes_list message.
  @$pb.TagNumber(1)
  InternetNodesRequest get internetNodesRequest => $_getN(0);
  @$pb.TagNumber(1)
  set internetNodesRequest(InternetNodesRequest value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasInternetNodesRequest() => $_has(0);
  @$pb.TagNumber(1)
  void clearInternetNodesRequest() => $_clearField(1);
  @$pb.TagNumber(1)
  InternetNodesRequest ensureInternetNodesRequest() => $_ensure(0);

  /// returns a list of all internet nodes and
  /// an information about why this message has been sent.
  @$pb.TagNumber(2)
  InternetNodesList get internetNodesList => $_getN(1);
  @$pb.TagNumber(2)
  set internetNodesList(InternetNodesList value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasInternetNodesList() => $_has(1);
  @$pb.TagNumber(2)
  void clearInternetNodesList() => $_clearField(2);
  @$pb.TagNumber(2)
  InternetNodesList ensureInternetNodesList() => $_ensure(1);

  /// Add a new internet node address.
  /// libqaul returns an internet_nodes_list message.
  @$pb.TagNumber(3)
  InternetNodesEntry get internetNodesAdd => $_getN(2);
  @$pb.TagNumber(3)
  set internetNodesAdd(InternetNodesEntry value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasInternetNodesAdd() => $_has(2);
  @$pb.TagNumber(3)
  void clearInternetNodesAdd() => $_clearField(3);
  @$pb.TagNumber(3)
  InternetNodesEntry ensureInternetNodesAdd() => $_ensure(2);

  /// Remove an internet node address.
  /// libqaul returns an internet_nodes_list message.
  @$pb.TagNumber(4)
  InternetNodesEntry get internetNodesRemove => $_getN(3);
  @$pb.TagNumber(4)
  set internetNodesRemove(InternetNodesEntry value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasInternetNodesRemove() => $_has(3);
  @$pb.TagNumber(4)
  void clearInternetNodesRemove() => $_clearField(4);
  @$pb.TagNumber(4)
  InternetNodesEntry ensureInternetNodesRemove() => $_ensure(3);

  /// Update an internet node state.
  /// libqaul returns an internet_nodes_list message.
  @$pb.TagNumber(5)
  InternetNodesEntry get internetNodesState => $_getN(4);
  @$pb.TagNumber(5)
  set internetNodesState(InternetNodesEntry value) => $_setField(5, value);
  @$pb.TagNumber(5)
  $core.bool hasInternetNodesState() => $_has(4);
  @$pb.TagNumber(5)
  void clearInternetNodesState() => $_clearField(5);
  @$pb.TagNumber(5)
  InternetNodesEntry ensureInternetNodesState() => $_ensure(4);

  /// Rename internet node.
  /// libqaul returns an internet_nodes_list message.
  @$pb.TagNumber(6)
  InternetNodesEntry get internetNodesRename => $_getN(5);
  @$pb.TagNumber(6)
  set internetNodesRename(InternetNodesEntry value) => $_setField(6, value);
  @$pb.TagNumber(6)
  $core.bool hasInternetNodesRename() => $_has(5);
  @$pb.TagNumber(6)
  void clearInternetNodesRename() => $_clearField(6);
  @$pb.TagNumber(6)
  InternetNodesEntry ensureInternetNodesRename() => $_ensure(5);
}

/// UI request for Internet nodes list
class InternetNodesRequest extends $pb.GeneratedMessage {
  factory InternetNodesRequest() => create();

  InternetNodesRequest._();

  factory InternetNodesRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory InternetNodesRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'InternetNodesRequest',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'qaul.rpc.connections'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  InternetNodesRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  InternetNodesRequest copyWith(void Function(InternetNodesRequest) updates) =>
      super.copyWith((message) => updates(message as InternetNodesRequest))
          as InternetNodesRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static InternetNodesRequest create() => InternetNodesRequest._();
  @$core.override
  InternetNodesRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static InternetNodesRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<InternetNodesRequest>(create);
  static InternetNodesRequest? _defaultInstance;
}

/// Internet Nodes List
///
/// This is a list of all peer nodes the internet
/// connections module tries to connect to.
///
/// This message is returned after a request, or when
/// adding or removing a node address.
class InternetNodesList extends $pb.GeneratedMessage {
  factory InternetNodesList({
    Info? info,
    $core.Iterable<InternetNodesEntry>? nodes,
  }) {
    final result = create();
    if (info != null) result.info = info;
    if (nodes != null) result.nodes.addAll(nodes);
    return result;
  }

  InternetNodesList._();

  factory InternetNodesList.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory InternetNodesList.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'InternetNodesList',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'qaul.rpc.connections'),
      createEmptyInstance: create)
    ..aE<Info>(1, _omitFieldNames ? '' : 'info', enumValues: Info.values)
    ..pPM<InternetNodesEntry>(2, _omitFieldNames ? '' : 'nodes',
        subBuilder: InternetNodesEntry.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  InternetNodesList clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  InternetNodesList copyWith(void Function(InternetNodesList) updates) =>
      super.copyWith((message) => updates(message as InternetNodesList))
          as InternetNodesList;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static InternetNodesList create() => InternetNodesList._();
  @$core.override
  InternetNodesList createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static InternetNodesList getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<InternetNodesList>(create);
  static InternetNodesList? _defaultInstance;

  /// Information about why this message is sent
  /// and the result of the request, adding or removing
  /// of nodes.
  @$pb.TagNumber(1)
  Info get info => $_getN(0);
  @$pb.TagNumber(1)
  set info(Info value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasInfo() => $_has(0);
  @$pb.TagNumber(1)
  void clearInfo() => $_clearField(1);

  /// list of all node multiaddresses that
  /// the internet module will try to connect to.
  @$pb.TagNumber(2)
  $pb.PbList<InternetNodesEntry> get nodes => $_getList(1);
}

/// Internet Nodes Entry
///
/// Contains a node address as a libp2p multiaddress.
/// e.g. "/ip4/144.91.74.192/udp/9229/quic-v1"
class InternetNodesEntry extends $pb.GeneratedMessage {
  factory InternetNodesEntry({
    $core.String? address,
    $core.bool? enabled,
    $core.String? name,
  }) {
    final result = create();
    if (address != null) result.address = address;
    if (enabled != null) result.enabled = enabled;
    if (name != null) result.name = name;
    return result;
  }

  InternetNodesEntry._();

  factory InternetNodesEntry.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory InternetNodesEntry.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'InternetNodesEntry',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'qaul.rpc.connections'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'address')
    ..aOB(2, _omitFieldNames ? '' : 'enabled')
    ..aOS(3, _omitFieldNames ? '' : 'name')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  InternetNodesEntry clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  InternetNodesEntry copyWith(void Function(InternetNodesEntry) updates) =>
      super.copyWith((message) => updates(message as InternetNodesEntry))
          as InternetNodesEntry;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static InternetNodesEntry create() => InternetNodesEntry._();
  @$core.override
  InternetNodesEntry createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static InternetNodesEntry getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<InternetNodesEntry>(create);
  static InternetNodesEntry? _defaultInstance;

  /// address
  @$pb.TagNumber(1)
  $core.String get address => $_getSZ(0);
  @$pb.TagNumber(1)
  set address($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasAddress() => $_has(0);
  @$pb.TagNumber(1)
  void clearAddress() => $_clearField(1);

  /// enabled
  @$pb.TagNumber(2)
  $core.bool get enabled => $_getBF(1);
  @$pb.TagNumber(2)
  set enabled($core.bool value) => $_setBool(1, value);
  @$pb.TagNumber(2)
  $core.bool hasEnabled() => $_has(1);
  @$pb.TagNumber(2)
  void clearEnabled() => $_clearField(2);

  /// name
  @$pb.TagNumber(3)
  $core.String get name => $_getSZ(2);
  @$pb.TagNumber(3)
  set name($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasName() => $_has(2);
  @$pb.TagNumber(3)
  void clearName() => $_clearField(3);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
