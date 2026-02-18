// This is a generated file - do not edit.
//
// Generated from router/router.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

/// Connection modules
class ConnectionModule extends $pb.ProtobufEnum {
  static const ConnectionModule NONE =
      ConnectionModule._(0, _omitEnumNames ? '' : 'NONE');
  static const ConnectionModule LAN =
      ConnectionModule._(1, _omitEnumNames ? '' : 'LAN');
  static const ConnectionModule INTERNET =
      ConnectionModule._(2, _omitEnumNames ? '' : 'INTERNET');
  static const ConnectionModule BLE =
      ConnectionModule._(3, _omitEnumNames ? '' : 'BLE');
  static const ConnectionModule LOCAL =
      ConnectionModule._(4, _omitEnumNames ? '' : 'LOCAL');

  static const $core.List<ConnectionModule> values = <ConnectionModule>[
    NONE,
    LAN,
    INTERNET,
    BLE,
    LOCAL,
  ];

  static final $core.List<ConnectionModule?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 4);
  static ConnectionModule? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const ConnectionModule._(super.value, super.name);
}

const $core.bool _omitEnumNames =
    $core.bool.fromEnvironment('protobuf.omit_enum_names');
