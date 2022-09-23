///
//  Generated code. Do not modify.
//  source: connections/connections.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

// ignore_for_file: UNDEFINED_SHOWN_NAME
import 'dart:core' as $core;
import 'package:protobuf/protobuf.dart' as $pb;

class Info extends $pb.ProtobufEnum {
  static const Info REQUEST = Info._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'REQUEST');
  static const Info ADD_SUCCESS = Info._(1, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'ADD_SUCCESS');
  static const Info ADD_ERROR_INVALID = Info._(2, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'ADD_ERROR_INVALID');
  static const Info REMOVE_SUCCESS = Info._(5, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'REMOVE_SUCCESS');
  static const Info STATE_SUCCESS = Info._(6, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'STATE_SUCCESS');
  static const Info REMOVE_ERROR_NOT_FOUND = Info._(7, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'REMOVE_ERROR_NOT_FOUND');

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

