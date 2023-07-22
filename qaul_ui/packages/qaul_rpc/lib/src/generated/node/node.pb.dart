//
//  Generated code. Do not modify.
//  source: node/node.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

enum Node_Message {
  getNodeInfo, 
  info, 
  notSet
}

class Node extends $pb.GeneratedMessage {
  factory Node() => create();
  Node._() : super();
  factory Node.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Node.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static const $core.Map<$core.int, Node_Message> _Node_MessageByTag = {
    1 : Node_Message.getNodeInfo,
    2 : Node_Message.info,
    0 : Node_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Node', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.node'), createEmptyInstance: create)
    ..oo(0, [1, 2])
    ..aOB(1, _omitFieldNames ? '' : 'getNodeInfo')
    ..aOM<NodeInformation>(2, _omitFieldNames ? '' : 'info', subBuilder: NodeInformation.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Node clone() => Node()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Node copyWith(void Function(Node) updates) => super.copyWith((message) => updates(message as Node)) as Node;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Node create() => Node._();
  Node createEmptyInstance() => create();
  static $pb.PbList<Node> createRepeated() => $pb.PbList<Node>();
  @$core.pragma('dart2js:noInline')
  static Node getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Node>(create);
  static Node? _defaultInstance;

  Node_Message whichMessage() => _Node_MessageByTag[$_whichOneof(0)]!;
  void clearMessage() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  $core.bool get getNodeInfo => $_getBF(0);
  @$pb.TagNumber(1)
  set getNodeInfo($core.bool v) { $_setBool(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGetNodeInfo() => $_has(0);
  @$pb.TagNumber(1)
  void clearGetNodeInfo() => clearField(1);

  @$pb.TagNumber(2)
  NodeInformation get info => $_getN(1);
  @$pb.TagNumber(2)
  set info(NodeInformation v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasInfo() => $_has(1);
  @$pb.TagNumber(2)
  void clearInfo() => clearField(2);
  @$pb.TagNumber(2)
  NodeInformation ensureInfo() => $_ensure(1);
}

class NodeInformation extends $pb.GeneratedMessage {
  factory NodeInformation() => create();
  NodeInformation._() : super();
  factory NodeInformation.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NodeInformation.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'NodeInformation', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.node'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'idBase58')
    ..pPS(2, _omitFieldNames ? '' : 'addresses')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NodeInformation clone() => NodeInformation()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NodeInformation copyWith(void Function(NodeInformation) updates) => super.copyWith((message) => updates(message as NodeInformation)) as NodeInformation;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NodeInformation create() => NodeInformation._();
  NodeInformation createEmptyInstance() => create();
  static $pb.PbList<NodeInformation> createRepeated() => $pb.PbList<NodeInformation>();
  @$core.pragma('dart2js:noInline')
  static NodeInformation getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NodeInformation>(create);
  static NodeInformation? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get idBase58 => $_getSZ(0);
  @$pb.TagNumber(1)
  set idBase58($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasIdBase58() => $_has(0);
  @$pb.TagNumber(1)
  void clearIdBase58() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.String> get addresses => $_getList(1);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
