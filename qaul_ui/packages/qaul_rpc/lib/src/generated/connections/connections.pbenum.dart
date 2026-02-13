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

/// Information about the system actions that led to
/// the creation of this message.
class Info extends $pb.ProtobufEnum {
  /// Internet Nodes Request
  /// By default, this message is sent due to an
  /// internet nodes request message.
  static const Info REQUEST = Info._(0, _omitEnumNames ? '' : 'REQUEST');

  /// Add Internet Node
  /// Successfully added an address
  static const Info ADD_SUCCESS =
      Info._(1, _omitEnumNames ? '' : 'ADD_SUCCESS');

  /// Error: not a valid multiaddress
  static const Info ADD_ERROR_INVALID =
      Info._(2, _omitEnumNames ? '' : 'ADD_ERROR_INVALID');

  /// Remove Internet Node
  /// Successfully removed the address
  static const Info REMOVE_SUCCESS =
      Info._(5, _omitEnumNames ? '' : 'REMOVE_SUCCESS');

  /// Successfully changed state of the address
  static const Info STATE_SUCCESS =
      Info._(6, _omitEnumNames ? '' : 'STATE_SUCCESS');

  /// Error: Address not found
  static const Info REMOVE_ERROR_NOT_FOUND =
      Info._(7, _omitEnumNames ? '' : 'REMOVE_ERROR_NOT_FOUND');

  static const $core.List<Info> values = <Info>[
    REQUEST,
    ADD_SUCCESS,
    ADD_ERROR_INVALID,
    REMOVE_SUCCESS,
    STATE_SUCCESS,
    REMOVE_ERROR_NOT_FOUND,
  ];

  static final $core.List<Info?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 7);
  static Info? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const Info._(super.value, super.name);
}

const $core.bool _omitEnumNames =
    $core.bool.fromEnvironment('protobuf.omit_enum_names');
