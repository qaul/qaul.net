///
//  Generated code. Do not modify.
//  source: connections/connections.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

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
  notSet
}

class Connections extends $pb.GeneratedMessage {
  static const $core.Map<$core.int, Connections_Message> _Connections_MessageByTag = {
    1 : Connections_Message.internetNodesRequest,
    2 : Connections_Message.internetNodesList,
    3 : Connections_Message.internetNodesAdd,
    4 : Connections_Message.internetNodesRemove,
    5 : Connections_Message.internetNodesState,
    0 : Connections_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Connections', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.connections'), createEmptyInstance: create)
    ..oo(0, [1, 2, 3, 4, 5])
    ..aOM<InternetNodesRequest>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'internetNodesRequest', subBuilder: InternetNodesRequest.create)
    ..aOM<InternetNodesList>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'internetNodesList', subBuilder: InternetNodesList.create)
    ..aOM<InternetNodesEntry>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'internetNodesAdd', subBuilder: InternetNodesEntry.create)
    ..aOM<InternetNodesEntry>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'internetNodesRemove', subBuilder: InternetNodesEntry.create)
    ..aOM<InternetNodesEntry>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'internetNodesState', subBuilder: InternetNodesEntry.create)
    ..hasRequiredFields = false
  ;

  Connections._() : super();
  factory Connections({
    InternetNodesRequest? internetNodesRequest,
    InternetNodesList? internetNodesList,
    InternetNodesEntry? internetNodesAdd,
    InternetNodesEntry? internetNodesRemove,
    InternetNodesEntry? internetNodesState,
  }) {
    final _result = create();
    if (internetNodesRequest != null) {
      _result.internetNodesRequest = internetNodesRequest;
    }
    if (internetNodesList != null) {
      _result.internetNodesList = internetNodesList;
    }
    if (internetNodesAdd != null) {
      _result.internetNodesAdd = internetNodesAdd;
    }
    if (internetNodesRemove != null) {
      _result.internetNodesRemove = internetNodesRemove;
    }
    if (internetNodesState != null) {
      _result.internetNodesState = internetNodesState;
    }
    return _result;
  }
  factory Connections.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Connections.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Connections clone() => Connections()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Connections copyWith(void Function(Connections) updates) => super.copyWith((message) => updates(message as Connections)) as Connections; // ignore: deprecated_member_use
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
}

class InternetNodesRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'InternetNodesRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.connections'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  InternetNodesRequest._() : super();
  factory InternetNodesRequest() => create();
  factory InternetNodesRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory InternetNodesRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  InternetNodesRequest clone() => InternetNodesRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  InternetNodesRequest copyWith(void Function(InternetNodesRequest) updates) => super.copyWith((message) => updates(message as InternetNodesRequest)) as InternetNodesRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static InternetNodesRequest create() => InternetNodesRequest._();
  InternetNodesRequest createEmptyInstance() => create();
  static $pb.PbList<InternetNodesRequest> createRepeated() => $pb.PbList<InternetNodesRequest>();
  @$core.pragma('dart2js:noInline')
  static InternetNodesRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<InternetNodesRequest>(create);
  static InternetNodesRequest? _defaultInstance;
}

class InternetNodesList extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'InternetNodesList', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.connections'), createEmptyInstance: create)
    ..e<Info>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'info', $pb.PbFieldType.OE, defaultOrMaker: Info.REQUEST, valueOf: Info.valueOf, enumValues: Info.values)
    ..pc<InternetNodesEntry>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'nodes', $pb.PbFieldType.PM, subBuilder: InternetNodesEntry.create)
    ..hasRequiredFields = false
  ;

  InternetNodesList._() : super();
  factory InternetNodesList({
    Info? info,
    $core.Iterable<InternetNodesEntry>? nodes,
  }) {
    final _result = create();
    if (info != null) {
      _result.info = info;
    }
    if (nodes != null) {
      _result.nodes.addAll(nodes);
    }
    return _result;
  }
  factory InternetNodesList.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory InternetNodesList.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  InternetNodesList clone() => InternetNodesList()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  InternetNodesList copyWith(void Function(InternetNodesList) updates) => super.copyWith((message) => updates(message as InternetNodesList)) as InternetNodesList; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static InternetNodesList create() => InternetNodesList._();
  InternetNodesList createEmptyInstance() => create();
  static $pb.PbList<InternetNodesList> createRepeated() => $pb.PbList<InternetNodesList>();
  @$core.pragma('dart2js:noInline')
  static InternetNodesList getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<InternetNodesList>(create);
  static InternetNodesList? _defaultInstance;

  @$pb.TagNumber(1)
  Info get info => $_getN(0);
  @$pb.TagNumber(1)
  set info(Info v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasInfo() => $_has(0);
  @$pb.TagNumber(1)
  void clearInfo() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<InternetNodesEntry> get nodes => $_getList(1);
}

class InternetNodesEntry extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'InternetNodesEntry', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.connections'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'address')
    ..aOB(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'enabled')
    ..hasRequiredFields = false
  ;

  InternetNodesEntry._() : super();
  factory InternetNodesEntry({
    $core.String? address,
    $core.bool? enabled,
  }) {
    final _result = create();
    if (address != null) {
      _result.address = address;
    }
    if (enabled != null) {
      _result.enabled = enabled;
    }
    return _result;
  }
  factory InternetNodesEntry.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory InternetNodesEntry.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  InternetNodesEntry clone() => InternetNodesEntry()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  InternetNodesEntry copyWith(void Function(InternetNodesEntry) updates) => super.copyWith((message) => updates(message as InternetNodesEntry)) as InternetNodesEntry; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static InternetNodesEntry create() => InternetNodesEntry._();
  InternetNodesEntry createEmptyInstance() => create();
  static $pb.PbList<InternetNodesEntry> createRepeated() => $pb.PbList<InternetNodesEntry>();
  @$core.pragma('dart2js:noInline')
  static InternetNodesEntry getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<InternetNodesEntry>(create);
  static InternetNodesEntry? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get address => $_getSZ(0);
  @$pb.TagNumber(1)
  set address($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasAddress() => $_has(0);
  @$pb.TagNumber(1)
  void clearAddress() => clearField(1);

  @$pb.TagNumber(2)
  $core.bool get enabled => $_getBF(1);
  @$pb.TagNumber(2)
  set enabled($core.bool v) { $_setBool(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasEnabled() => $_has(1);
  @$pb.TagNumber(2)
  void clearEnabled() => clearField(2);
}

