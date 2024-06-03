//
//  Generated code. Do not modify.
//  source: connections/connections.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

/// Information about the system actions that led to
/// the creation of this message.
class Info extends $pb.ProtobufEnum {
  static const Info REQUEST = Info._(0, _omitEnumNames ? '' : 'REQUEST');
  static const Info ADD_SUCCESS = Info._(1, _omitEnumNames ? '' : 'ADD_SUCCESS');
  static const Info ADD_ERROR_INVALID = Info._(2, _omitEnumNames ? '' : 'ADD_ERROR_INVALID');
  static const Info REMOVE_SUCCESS = Info._(5, _omitEnumNames ? '' : 'REMOVE_SUCCESS');
  static const Info STATE_SUCCESS = Info._(6, _omitEnumNames ? '' : 'STATE_SUCCESS');
  static const Info REMOVE_ERROR_NOT_FOUND = Info._(7, _omitEnumNames ? '' : 'REMOVE_ERROR_NOT_FOUND');

  static const $core.List<Info> values = <Info> [
    REQUEST,
    ADD_SUCCESS,
    ADD_ERROR_INVALID,
    REMOVE_SUCCESS,
    STATE_SUCCESS,
    REMOVE_ERROR_NOT_FOUND,
  ];

  static final $core.Map<$core.int, Info> _byValue = $pb.ProtobufEnum.initByValue(values);
  static Info? valueOf($core.int value) => _byValue[value];

  const Info._($core.int v, $core.String n) : super(v, n);
}


const _omitEnumNames = $core.bool.fromEnvironment('protobuf.omit_enum_names');
