///
//  Generated code. Do not modify.
//  source: connections/ble/manager/ble.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

// ignore_for_file: UNDEFINED_SHOWN_NAME
import 'dart:core' as $core;
import 'package:protobuf/protobuf.dart' as $pb;

class BleMode extends $pb.ProtobufEnum {
  static const BleMode legacy = BleMode._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'legacy');
  static const BleMode le_1m = BleMode._(1, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'le_1m');
  static const BleMode le_2m = BleMode._(2, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'le_2m');
  static const BleMode coded_2 = BleMode._(3, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'coded_2');
  static const BleMode coded_8 = BleMode._(4, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'coded_8');

  static const $core.List<BleMode> values = <BleMode> [
    legacy,
    le_1m,
    le_2m,
    coded_2,
    coded_8,
  ];

  static final $core.Map<$core.int, BleMode> _byValue = $pb.ProtobufEnum.initByValue(values);
  static BleMode? valueOf($core.int value) => _byValue[value];

  const BleMode._($core.int v, $core.String n) : super(v, n);
}

