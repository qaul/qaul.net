///
//  Generated code. Do not modify.
//  source: connections/ble/ble_net.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

enum BleMessage_Message {
  info, 
  feed, 
  messaging, 
  identification, 
  notSet
}

class BleMessage extends $pb.GeneratedMessage {
  static const $core.Map<$core.int, BleMessage_Message> _BleMessage_MessageByTag = {
    1 : BleMessage_Message.info,
    2 : BleMessage_Message.feed,
    3 : BleMessage_Message.messaging,
    4 : BleMessage_Message.identification,
    0 : BleMessage_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'BleMessage', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.net.ble'), createEmptyInstance: create)
    ..oo(0, [1, 2, 3, 4])
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'info', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'feed', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'messaging', $pb.PbFieldType.OY)
    ..aOM<Identification>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'identification', subBuilder: Identification.create)
    ..hasRequiredFields = false
  ;

  BleMessage._() : super();
  factory BleMessage({
    $core.List<$core.int>? info,
    $core.List<$core.int>? feed,
    $core.List<$core.int>? messaging,
    Identification? identification,
  }) {
    final _result = create();
    if (info != null) {
      _result.info = info;
    }
    if (feed != null) {
      _result.feed = feed;
    }
    if (messaging != null) {
      _result.messaging = messaging;
    }
    if (identification != null) {
      _result.identification = identification;
    }
    return _result;
  }
  factory BleMessage.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory BleMessage.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  BleMessage clone() => BleMessage()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  BleMessage copyWith(void Function(BleMessage) updates) => super.copyWith((message) => updates(message as BleMessage)) as BleMessage; // ignore: deprecated_member_use
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

  @$pb.TagNumber(1)
  $core.List<$core.int> get info => $_getN(0);
  @$pb.TagNumber(1)
  set info($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasInfo() => $_has(0);
  @$pb.TagNumber(1)
  void clearInfo() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get feed => $_getN(1);
  @$pb.TagNumber(2)
  set feed($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasFeed() => $_has(1);
  @$pb.TagNumber(2)
  void clearFeed() => clearField(2);

  @$pb.TagNumber(3)
  $core.List<$core.int> get messaging => $_getN(2);
  @$pb.TagNumber(3)
  set messaging($core.List<$core.int> v) { $_setBytes(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasMessaging() => $_has(2);
  @$pb.TagNumber(3)
  void clearMessaging() => clearField(3);

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

class Identification extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Identification', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.net.ble'), createEmptyInstance: create)
    ..aOB(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'request')
    ..aOM<NodeIdentification>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'node', subBuilder: NodeIdentification.create)
    ..hasRequiredFields = false
  ;

  Identification._() : super();
  factory Identification({
    $core.bool? request,
    NodeIdentification? node,
  }) {
    final _result = create();
    if (request != null) {
      _result.request = request;
    }
    if (node != null) {
      _result.node = node;
    }
    return _result;
  }
  factory Identification.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Identification.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Identification clone() => Identification()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Identification copyWith(void Function(Identification) updates) => super.copyWith((message) => updates(message as Identification)) as Identification; // ignore: deprecated_member_use
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

class NodeIdentification extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'NodeIdentification', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.net.ble'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'id', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  NodeIdentification._() : super();
  factory NodeIdentification({
    $core.List<$core.int>? id,
  }) {
    final _result = create();
    if (id != null) {
      _result.id = id;
    }
    return _result;
  }
  factory NodeIdentification.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NodeIdentification.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NodeIdentification clone() => NodeIdentification()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NodeIdentification copyWith(void Function(NodeIdentification) updates) => super.copyWith((message) => updates(message as NodeIdentification)) as NodeIdentification; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static NodeIdentification create() => NodeIdentification._();
  NodeIdentification createEmptyInstance() => create();
  static $pb.PbList<NodeIdentification> createRepeated() => $pb.PbList<NodeIdentification>();
  @$core.pragma('dart2js:noInline')
  static NodeIdentification getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NodeIdentification>(create);
  static NodeIdentification? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get id => $_getN(0);
  @$pb.TagNumber(1)
  set id($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => clearField(1);
}

