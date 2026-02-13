// This is a generated file - do not edit.
//
// Generated from rpc/qaul_rpc.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'qaul_rpc.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'qaul_rpc.pbenum.dart';

/// The main libqaul RPC message container.
/// All RPC messages from and to libqaul are packed
/// into this container.
class QaulRpc extends $pb.GeneratedMessage {
  factory QaulRpc({
    Modules? module,
    $core.String? requestId,
    $core.List<$core.int>? userId,
    $core.List<$core.int>? data,
  }) {
    final result = create();
    if (module != null) result.module = module;
    if (requestId != null) result.requestId = requestId;
    if (userId != null) result.userId = userId;
    if (data != null) result.data = data;
    return result;
  }

  QaulRpc._();

  factory QaulRpc.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory QaulRpc.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'QaulRpc',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc'),
      createEmptyInstance: create)
    ..aE<Modules>(1, _omitFieldNames ? '' : 'module',
        enumValues: Modules.values)
    ..aOS(2, _omitFieldNames ? '' : 'requestId')
    ..a<$core.List<$core.int>>(
        3, _omitFieldNames ? '' : 'userId', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(
        4, _omitFieldNames ? '' : 'data', $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  QaulRpc clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  QaulRpc copyWith(void Function(QaulRpc) updates) =>
      super.copyWith((message) => updates(message as QaulRpc)) as QaulRpc;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static QaulRpc create() => QaulRpc._();
  @$core.override
  QaulRpc createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static QaulRpc getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<QaulRpc>(create);
  static QaulRpc? _defaultInstance;

  /// which module to approach
  @$pb.TagNumber(1)
  Modules get module => $_getN(0);
  @$pb.TagNumber(1)
  set module(Modules value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasModule() => $_has(0);
  @$pb.TagNumber(1)
  void clearModule() => $_clearField(1);

  /// can be used to identify responses
  @$pb.TagNumber(2)
  $core.String get requestId => $_getSZ(1);
  @$pb.TagNumber(2)
  set requestId($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasRequestId() => $_has(1);
  @$pb.TagNumber(2)
  void clearRequestId() => $_clearField(2);

  /// authorisation
  /// binary user id
  @$pb.TagNumber(3)
  $core.List<$core.int> get userId => $_getN(2);
  @$pb.TagNumber(3)
  set userId($core.List<$core.int> value) => $_setBytes(2, value);
  @$pb.TagNumber(3)
  $core.bool hasUserId() => $_has(2);
  @$pb.TagNumber(3)
  void clearUserId() => $_clearField(3);

  /// the protobuf encoded binary message data
  /// which is passed to the module.
  @$pb.TagNumber(4)
  $core.List<$core.int> get data => $_getN(3);
  @$pb.TagNumber(4)
  set data($core.List<$core.int> value) => $_setBytes(3, value);
  @$pb.TagNumber(4)
  $core.bool hasData() => $_has(3);
  @$pb.TagNumber(4)
  void clearData() => $_clearField(4);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
