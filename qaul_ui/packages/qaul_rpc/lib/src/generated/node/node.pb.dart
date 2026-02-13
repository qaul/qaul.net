// This is a generated file - do not edit.
//
// Generated from node/node.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

enum Node_Message { getNodeInfo, info, notSet }

/// node rpc message container
class Node extends $pb.GeneratedMessage {
  factory Node({
    $core.bool? getNodeInfo,
    NodeInformation? info,
  }) {
    final result = create();
    if (getNodeInfo != null) result.getNodeInfo = getNodeInfo;
    if (info != null) result.info = info;
    return result;
  }

  Node._();

  factory Node.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory Node.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, Node_Message> _Node_MessageByTag = {
    1: Node_Message.getNodeInfo,
    2: Node_Message.info,
    0: Node_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Node',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.node'),
      createEmptyInstance: create)
    ..oo(0, [1, 2])
    ..aOB(1, _omitFieldNames ? '' : 'getNodeInfo')
    ..aOM<NodeInformation>(2, _omitFieldNames ? '' : 'info',
        subBuilder: NodeInformation.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Node clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Node copyWith(void Function(Node) updates) =>
      super.copyWith((message) => updates(message as Node)) as Node;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Node create() => Node._();
  @$core.override
  Node createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static Node getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Node>(create);
  static Node? _defaultInstance;

  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  Node_Message whichMessage() => _Node_MessageByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  void clearMessage() => $_clearField($_whichOneof(0));

  /// request node info message from libqaul
  @$pb.TagNumber(1)
  $core.bool get getNodeInfo => $_getBF(0);
  @$pb.TagNumber(1)
  set getNodeInfo($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasGetNodeInfo() => $_has(0);
  @$pb.TagNumber(1)
  void clearGetNodeInfo() => $_clearField(1);

  /// libqaul sends node info
  @$pb.TagNumber(2)
  NodeInformation get info => $_getN(1);
  @$pb.TagNumber(2)
  set info(NodeInformation value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasInfo() => $_has(1);
  @$pb.TagNumber(2)
  void clearInfo() => $_clearField(2);
  @$pb.TagNumber(2)
  NodeInformation ensureInfo() => $_ensure(1);
}

/// node information
class NodeInformation extends $pb.GeneratedMessage {
  factory NodeInformation({
    $core.String? idBase58,
    $core.Iterable<$core.String>? addresses,
  }) {
    final result = create();
    if (idBase58 != null) result.idBase58 = idBase58;
    if (addresses != null) result.addresses.addAll(addresses);
    return result;
  }

  NodeInformation._();

  factory NodeInformation.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory NodeInformation.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'NodeInformation',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.node'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'idBase58')
    ..pPS(2, _omitFieldNames ? '' : 'addresses')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NodeInformation clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NodeInformation copyWith(void Function(NodeInformation) updates) =>
      super.copyWith((message) => updates(message as NodeInformation))
          as NodeInformation;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NodeInformation create() => NodeInformation._();
  @$core.override
  NodeInformation createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static NodeInformation getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<NodeInformation>(create);
  static NodeInformation? _defaultInstance;

  /// the node ID in base 58 encoding
  @$pb.TagNumber(1)
  $core.String get idBase58 => $_getSZ(0);
  @$pb.TagNumber(1)
  set idBase58($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasIdBase58() => $_has(0);
  @$pb.TagNumber(1)
  void clearIdBase58() => $_clearField(1);

  /// all known multi addresses under which
  /// this node can be connected.
  @$pb.TagNumber(2)
  $pb.PbList<$core.String> get addresses => $_getList(1);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
