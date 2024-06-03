//
//  Generated code. Do not modify.
//  source: connections/ble/ble_net.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

enum BleMessage_Message {
  info, 
  feed, 
  messaging, 
  identification, 
  notSet
}

/// BLE network communication message
class BleMessage extends $pb.GeneratedMessage {
  factory BleMessage({
    $core.List<$core.int>? info,
    $core.List<$core.int>? feed,
    $core.List<$core.int>? messaging,
    Identification? identification,
  }) {
    final $result = create();
    if (info != null) {
      $result.info = info;
    }
    if (feed != null) {
      $result.feed = feed;
    }
    if (messaging != null) {
      $result.messaging = messaging;
    }
    if (identification != null) {
      $result.identification = identification;
    }
    return $result;
  }
  BleMessage._() : super();
  factory BleMessage.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory BleMessage.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static const $core.Map<$core.int, BleMessage_Message> _BleMessage_MessageByTag = {
    1 : BleMessage_Message.info,
    2 : BleMessage_Message.feed,
    3 : BleMessage_Message.messaging,
    4 : BleMessage_Message.identification,
    0 : BleMessage_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'BleMessage', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.ble'), createEmptyInstance: create)
    ..oo(0, [1, 2, 3, 4])
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'info', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(2, _omitFieldNames ? '' : 'feed', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(3, _omitFieldNames ? '' : 'messaging', $pb.PbFieldType.OY)
    ..aOM<Identification>(4, _omitFieldNames ? '' : 'identification', subBuilder: Identification.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  BleMessage clone() => BleMessage()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  BleMessage copyWith(void Function(BleMessage) updates) => super.copyWith((message) => updates(message as BleMessage)) as BleMessage;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static BleMessage create() => BleMessage._();
  BleMessage createEmptyInstance() => create();
  static $pb.PbList<BleMessage> createRepeated() => $pb.PbList<BleMessage>();
  @$core.pragma('dart2js:noInline')
  static BleMessage getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<BleMessage>(create);
  static BleMessage? _defaultInstance;

  BleMessage_Message whichMessage() => _BleMessage_MessageByTag[$_whichOneof(0)]!;
  void clearMessage() => clearField($_whichOneof(0));

  /// info message
  @$pb.TagNumber(1)
  $core.List<$core.int> get info => $_getN(0);
  @$pb.TagNumber(1)
  set info($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasInfo() => $_has(0);
  @$pb.TagNumber(1)
  void clearInfo() => clearField(1);

  /// feed message
  @$pb.TagNumber(2)
  $core.List<$core.int> get feed => $_getN(1);
  @$pb.TagNumber(2)
  set feed($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasFeed() => $_has(1);
  @$pb.TagNumber(2)
  void clearFeed() => clearField(2);

  /// messaging message
  @$pb.TagNumber(3)
  $core.List<$core.int> get messaging => $_getN(2);
  @$pb.TagNumber(3)
  set messaging($core.List<$core.int> v) { $_setBytes(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasMessaging() => $_has(2);
  @$pb.TagNumber(3)
  void clearMessaging() => clearField(3);

  /// identification request
  @$pb.TagNumber(4)
  Identification get identification => $_getN(3);
  @$pb.TagNumber(4)
  set identification(Identification v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasIdentification() => $_has(3);
  @$pb.TagNumber(4)
  void clearIdentification() => clearField(4);
  @$pb.TagNumber(4)
  Identification ensureIdentification() => $_ensure(3);
}

/// Identfication Request
class Identification extends $pb.GeneratedMessage {
  factory Identification({
    $core.bool? request,
    NodeIdentification? node,
  }) {
    final $result = create();
    if (request != null) {
      $result.request = request;
    }
    if (node != null) {
      $result.node = node;
    }
    return $result;
  }
  Identification._() : super();
  factory Identification.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Identification.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Identification', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.ble'), createEmptyInstance: create)
    ..aOB(1, _omitFieldNames ? '' : 'request')
    ..aOM<NodeIdentification>(2, _omitFieldNames ? '' : 'node', subBuilder: NodeIdentification.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Identification clone() => Identification()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Identification copyWith(void Function(Identification) updates) => super.copyWith((message) => updates(message as Identification)) as Identification;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Identification create() => Identification._();
  Identification createEmptyInstance() => create();
  static $pb.PbList<Identification> createRepeated() => $pb.PbList<Identification>();
  @$core.pragma('dart2js:noInline')
  static Identification getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Identification>(create);
  static Identification? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get request => $_getBF(0);
  @$pb.TagNumber(1)
  set request($core.bool v) { $_setBool(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasRequest() => $_has(0);
  @$pb.TagNumber(1)
  void clearRequest() => clearField(1);

  @$pb.TagNumber(2)
  NodeIdentification get node => $_getN(1);
  @$pb.TagNumber(2)
  set node(NodeIdentification v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasNode() => $_has(1);
  @$pb.TagNumber(2)
  void clearNode() => clearField(2);
  @$pb.TagNumber(2)
  NodeIdentification ensureNode() => $_ensure(1);
}

/// Identity Information
class NodeIdentification extends $pb.GeneratedMessage {
  factory NodeIdentification({
    $core.List<$core.int>? id,
  }) {
    final $result = create();
    if (id != null) {
      $result.id = id;
    }
    return $result;
  }
  NodeIdentification._() : super();
  factory NodeIdentification.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NodeIdentification.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'NodeIdentification', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.ble'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'id', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NodeIdentification clone() => NodeIdentification()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NodeIdentification copyWith(void Function(NodeIdentification) updates) => super.copyWith((message) => updates(message as NodeIdentification)) as NodeIdentification;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NodeIdentification create() => NodeIdentification._();
  NodeIdentification createEmptyInstance() => create();
  static $pb.PbList<NodeIdentification> createRepeated() => $pb.PbList<NodeIdentification>();
  @$core.pragma('dart2js:noInline')
  static NodeIdentification getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NodeIdentification>(create);
  static NodeIdentification? _defaultInstance;

  /// Node ID
  @$pb.TagNumber(1)
  $core.List<$core.int> get id => $_getN(0);
  @$pb.TagNumber(1)
  set id($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => clearField(1);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
