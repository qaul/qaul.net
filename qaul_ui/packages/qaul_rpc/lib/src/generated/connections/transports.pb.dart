// This is a generated file - do not edit.
//
// Generated from connections/transports.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

enum Transports_Message {
  listRequest,
  list,
  setEnabled,
  setEnabledResult,
  notSet
}

/// Transports RPC message container
class Transports extends $pb.GeneratedMessage {
  factory Transports({
    TransportsListRequest? listRequest,
    TransportsList? list,
    TransportSetEnabled? setEnabled,
    TransportSetEnabledResult? setEnabledResult,
  }) {
    final result = Transports._();
    if (listRequest != null) result.listRequest = listRequest;
    if (list != null) result.list = list;
    if (setEnabled != null) result.setEnabled = setEnabled;
    if (setEnabledResult != null) result.setEnabledResult = setEnabledResult;
    return result;
  }

  Transports._();

  factory Transports.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Transports()..mergeFromBuffer(data, registry);
  factory Transports.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Transports()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, Transports_Message>
      _Transports_MessageByTag = {
    1: Transports_Message.listRequest,
    2: Transports_Message.list,
    3: Transports_Message.setEnabled,
    4: Transports_Message.setEnabledResult,
    0: Transports_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Transports',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.transports'),
      createEmptyInstance: Transports.$_createMessage)
    ..oo(0, [1, 2, 3, 4])
    ..aOM<TransportsListRequest>(1, _omitFieldNames ? '' : 'listRequest',
        subBuilder: TransportsListRequest.$_createMessage)
    ..aOM<TransportsList>(2, _omitFieldNames ? '' : 'list',
        subBuilder: TransportsList.$_createMessage)
    ..aOM<TransportSetEnabled>(3, _omitFieldNames ? '' : 'setEnabled',
        subBuilder: TransportSetEnabled.$_createMessage)
    ..aOM<TransportSetEnabledResult>(
        4, _omitFieldNames ? '' : 'setEnabledResult',
        subBuilder: TransportSetEnabledResult.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Transports clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Transports copyWith(void Function(Transports) updates) =>
      super.copyWith((message) => updates(message as Transports)) as Transports;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use Transports() / Transports.new instead')
  static Transports create() => Transports._();
  static Transports $_createMessage() => Transports._();
  @$core.override
  Transports createEmptyInstance() => Transports._();
  @$core.pragma('dart2js:noInline')
  static Transports getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<Transports>(Transports.$_createMessage);
  static Transports? _defaultInstance;

  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  Transports_Message whichMessage() =>
      _Transports_MessageByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  void clearMessage() => $_clearField($_whichOneof(0));

  /// Request a list of all transports and their status.
  @$pb.TagNumber(1)
  TransportsListRequest get listRequest => $_getN(0);
  @$pb.TagNumber(1)
  set listRequest(TransportsListRequest value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasListRequest() => $_has(0);
  @$pb.TagNumber(1)
  void clearListRequest() => $_clearField(1);
  @$pb.TagNumber(1)
  TransportsListRequest ensureListRequest() => $_ensure(0);

  /// Response with the list of transports.
  @$pb.TagNumber(2)
  TransportsList get list => $_getN(1);
  @$pb.TagNumber(2)
  set list(TransportsList value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasList() => $_has(1);
  @$pb.TagNumber(2)
  void clearList() => $_clearField(2);
  @$pb.TagNumber(2)
  TransportsList ensureList() => $_ensure(1);

  /// Enable or disable a transport by id.
  @$pb.TagNumber(3)
  TransportSetEnabled get setEnabled => $_getN(2);
  @$pb.TagNumber(3)
  set setEnabled(TransportSetEnabled value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasSetEnabled() => $_has(2);
  @$pb.TagNumber(3)
  void clearSetEnabled() => $_clearField(3);
  @$pb.TagNumber(3)
  TransportSetEnabled ensureSetEnabled() => $_ensure(2);

  /// Result of a set_enabled call.
  @$pb.TagNumber(4)
  TransportSetEnabledResult get setEnabledResult => $_getN(3);
  @$pb.TagNumber(4)
  set setEnabledResult(TransportSetEnabledResult value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasSetEnabledResult() => $_has(3);
  @$pb.TagNumber(4)
  void clearSetEnabledResult() => $_clearField(4);
  @$pb.TagNumber(4)
  TransportSetEnabledResult ensureSetEnabledResult() => $_ensure(3);
}

/// Request the list of all transports.
class TransportsListRequest extends $pb.GeneratedMessage {
  factory TransportsListRequest() => TransportsListRequest._();

  TransportsListRequest._();

  factory TransportsListRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TransportsListRequest()..mergeFromBuffer(data, registry);
  factory TransportsListRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TransportsListRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TransportsListRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.transports'),
      createEmptyInstance: TransportsListRequest.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TransportsListRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TransportsListRequest copyWith(
          void Function(TransportsListRequest) updates) =>
      super.copyWith((message) => updates(message as TransportsListRequest))
          as TransportsListRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use TransportsListRequest() / TransportsListRequest.new instead')
  static TransportsListRequest create() => TransportsListRequest._();
  static TransportsListRequest $_createMessage() => TransportsListRequest._();
  @$core.override
  TransportsListRequest createEmptyInstance() => TransportsListRequest._();
  @$core.pragma('dart2js:noInline')
  static TransportsListRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TransportsListRequest>(
          TransportsListRequest.$_createMessage);
  static TransportsListRequest? _defaultInstance;
}

/// List of all transports.
class TransportsList extends $pb.GeneratedMessage {
  factory TransportsList({
    $core.Iterable<TransportInfo>? transports,
  }) {
    final result = TransportsList._();
    if (transports != null) result.transports.addAll(transports);
    return result;
  }

  TransportsList._();

  factory TransportsList.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TransportsList()..mergeFromBuffer(data, registry);
  factory TransportsList.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TransportsList()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TransportsList',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.transports'),
      createEmptyInstance: TransportsList.$_createMessage)
    ..pPM<TransportInfo>(1, _omitFieldNames ? '' : 'transports',
        subBuilder: TransportInfo.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TransportsList clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TransportsList copyWith(void Function(TransportsList) updates) =>
      super.copyWith((message) => updates(message as TransportsList))
          as TransportsList;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use TransportsList() / TransportsList.new instead')
  static TransportsList create() => TransportsList._();
  static TransportsList $_createMessage() => TransportsList._();
  @$core.override
  TransportsList createEmptyInstance() => TransportsList._();
  @$core.pragma('dart2js:noInline')
  static TransportsList getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<TransportsList>(
          TransportsList.$_createMessage);
  static TransportsList? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<TransportInfo> get transports => $_getList(0);
}

/// Information about a single transport.
class TransportInfo extends $pb.GeneratedMessage {
  factory TransportInfo({
    $core.String? id,
    $core.String? label,
    $core.String? status,
    $core.bool? enabled,
    $core.bool? supportsRuntimeToggle,
    $core.bool? isLocalOnly,
  }) {
    final result = TransportInfo._();
    if (id != null) result.id = id;
    if (label != null) result.label = label;
    if (status != null) result.status = status;
    if (enabled != null) result.enabled = enabled;
    if (supportsRuntimeToggle != null)
      result.supportsRuntimeToggle = supportsRuntimeToggle;
    if (isLocalOnly != null) result.isLocalOnly = isLocalOnly;
    return result;
  }

  TransportInfo._();

  factory TransportInfo.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TransportInfo()..mergeFromBuffer(data, registry);
  factory TransportInfo.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TransportInfo()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TransportInfo',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.transports'),
      createEmptyInstance: TransportInfo.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'label')
    ..aOS(3, _omitFieldNames ? '' : 'status')
    ..aOB(4, _omitFieldNames ? '' : 'enabled')
    ..aOB(5, _omitFieldNames ? '' : 'supportsRuntimeToggle')
    ..aOB(6, _omitFieldNames ? '' : 'isLocalOnly')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TransportInfo clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TransportInfo copyWith(void Function(TransportInfo) updates) =>
      super.copyWith((message) => updates(message as TransportInfo))
          as TransportInfo;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use TransportInfo() / TransportInfo.new instead')
  static TransportInfo create() => TransportInfo._();
  static TransportInfo $_createMessage() => TransportInfo._();
  @$core.override
  TransportInfo createEmptyInstance() => TransportInfo._();
  @$core.pragma('dart2js:noInline')
  static TransportInfo getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<TransportInfo>(
          TransportInfo.$_createMessage);
  static TransportInfo? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get id => $_getSZ(0);
  @$pb.TagNumber(1)
  set id($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get label => $_getSZ(1);
  @$pb.TagNumber(2)
  set label($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasLabel() => $_has(1);
  @$pb.TagNumber(2)
  void clearLabel() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get status => $_getSZ(2);
  @$pb.TagNumber(3)
  set status($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasStatus() => $_has(2);
  @$pb.TagNumber(3)
  void clearStatus() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.bool get enabled => $_getBF(3);
  @$pb.TagNumber(4)
  set enabled($core.bool value) => $_setBool(3, value);
  @$pb.TagNumber(4)
  $core.bool hasEnabled() => $_has(3);
  @$pb.TagNumber(4)
  void clearEnabled() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.bool get supportsRuntimeToggle => $_getBF(4);
  @$pb.TagNumber(5)
  set supportsRuntimeToggle($core.bool value) => $_setBool(4, value);
  @$pb.TagNumber(5)
  $core.bool hasSupportsRuntimeToggle() => $_has(4);
  @$pb.TagNumber(5)
  void clearSupportsRuntimeToggle() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.bool get isLocalOnly => $_getBF(5);
  @$pb.TagNumber(6)
  set isLocalOnly($core.bool value) => $_setBool(5, value);
  @$pb.TagNumber(6)
  $core.bool hasIsLocalOnly() => $_has(5);
  @$pb.TagNumber(6)
  void clearIsLocalOnly() => $_clearField(6);
}

/// Enable or disable a transport.
class TransportSetEnabled extends $pb.GeneratedMessage {
  factory TransportSetEnabled({
    $core.String? id,
    $core.bool? enabled,
  }) {
    final result = TransportSetEnabled._();
    if (id != null) result.id = id;
    if (enabled != null) result.enabled = enabled;
    return result;
  }

  TransportSetEnabled._();

  factory TransportSetEnabled.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TransportSetEnabled()..mergeFromBuffer(data, registry);
  factory TransportSetEnabled.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TransportSetEnabled()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TransportSetEnabled',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.transports'),
      createEmptyInstance: TransportSetEnabled.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'id')
    ..aOB(2, _omitFieldNames ? '' : 'enabled')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TransportSetEnabled clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TransportSetEnabled copyWith(void Function(TransportSetEnabled) updates) =>
      super.copyWith((message) => updates(message as TransportSetEnabled))
          as TransportSetEnabled;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use TransportSetEnabled() / TransportSetEnabled.new instead')
  static TransportSetEnabled create() => TransportSetEnabled._();
  static TransportSetEnabled $_createMessage() => TransportSetEnabled._();
  @$core.override
  TransportSetEnabled createEmptyInstance() => TransportSetEnabled._();
  @$core.pragma('dart2js:noInline')
  static TransportSetEnabled getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TransportSetEnabled>(
          TransportSetEnabled.$_createMessage);
  static TransportSetEnabled? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get id => $_getSZ(0);
  @$pb.TagNumber(1)
  set id($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.bool get enabled => $_getBF(1);
  @$pb.TagNumber(2)
  set enabled($core.bool value) => $_setBool(1, value);
  @$pb.TagNumber(2)
  $core.bool hasEnabled() => $_has(1);
  @$pb.TagNumber(2)
  void clearEnabled() => $_clearField(2);
}

/// Result of set_enabled.
class TransportSetEnabledResult extends $pb.GeneratedMessage {
  factory TransportSetEnabledResult({
    $core.String? id,
    $core.bool? success,
    $core.String? error,
  }) {
    final result = TransportSetEnabledResult._();
    if (id != null) result.id = id;
    if (success != null) result.success = success;
    if (error != null) result.error = error;
    return result;
  }

  TransportSetEnabledResult._();

  factory TransportSetEnabledResult.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TransportSetEnabledResult()..mergeFromBuffer(data, registry);
  factory TransportSetEnabledResult.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      TransportSetEnabledResult()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TransportSetEnabledResult',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.transports'),
      createEmptyInstance: TransportSetEnabledResult.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'id')
    ..aOB(2, _omitFieldNames ? '' : 'success')
    ..aOS(3, _omitFieldNames ? '' : 'error')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TransportSetEnabledResult clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TransportSetEnabledResult copyWith(
          void Function(TransportSetEnabledResult) updates) =>
      super.copyWith((message) => updates(message as TransportSetEnabledResult))
          as TransportSetEnabledResult;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use TransportSetEnabledResult() / TransportSetEnabledResult.new instead')
  static TransportSetEnabledResult create() => TransportSetEnabledResult._();
  static TransportSetEnabledResult $_createMessage() =>
      TransportSetEnabledResult._();
  @$core.override
  TransportSetEnabledResult createEmptyInstance() =>
      TransportSetEnabledResult._();
  @$core.pragma('dart2js:noInline')
  static TransportSetEnabledResult getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TransportSetEnabledResult>(
          TransportSetEnabledResult.$_createMessage);
  static TransportSetEnabledResult? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get id => $_getSZ(0);
  @$pb.TagNumber(1)
  set id($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.bool get success => $_getBF(1);
  @$pb.TagNumber(2)
  set success($core.bool value) => $_setBool(1, value);
  @$pb.TagNumber(2)
  $core.bool hasSuccess() => $_has(1);
  @$pb.TagNumber(2)
  void clearSuccess() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get error => $_getSZ(2);
  @$pb.TagNumber(3)
  set error($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasError() => $_has(2);
  @$pb.TagNumber(3)
  void clearError() => $_clearField(3);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
