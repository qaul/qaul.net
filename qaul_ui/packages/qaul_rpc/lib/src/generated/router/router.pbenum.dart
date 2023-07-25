///
//  Generated code. Do not modify.
//  source: router/router.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

// ignore_for_file: UNDEFINED_SHOWN_NAME
import 'dart:core' as $core;
import 'package:protobuf/protobuf.dart' as $pb;

class ConnectionModule extends $pb.ProtobufEnum {
  static const ConnectionModule NONE = ConnectionModule._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'NONE');
  static const ConnectionModule LAN = ConnectionModule._(1, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'LAN');
  static const ConnectionModule INTERNET = ConnectionModule._(2, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'INTERNET');
  static const ConnectionModule BLE = ConnectionModule._(3, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'BLE');
  static const ConnectionModule LOCAL = ConnectionModule._(4, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'LOCAL');

  static const $core.List<ConnectionModule> values = <ConnectionModule> [
    NONE,
    LAN,
    INTERNET,
    BLE,
    LOCAL,
  ];

  static final $core.Map<$core.int, ConnectionModule> _byValue = $pb.ProtobufEnum.initByValue(values);
  static ConnectionModule? valueOf($core.int value) => _byValue[value];

  const ConnectionModule._($core.int v, $core.String n) : super(v, n);
}

