///
//  Generated code. Do not modify.
//  source: connections/ble/ble.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

// ignore_for_file: UNDEFINED_SHOWN_NAME
import 'dart:core' as $core;
import 'package:protobuf/protobuf.dart' as $pb;

class BlePowerSetting extends $pb.ProtobufEnum {
  static const BlePowerSetting low_power = BlePowerSetting._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'low_power');
  static const BlePowerSetting balanced = BlePowerSetting._(1, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'balanced');
  static const BlePowerSetting low_latency = BlePowerSetting._(2, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'low_latency');

  static const $core.List<BlePowerSetting> values = <BlePowerSetting> [
    low_power,
    balanced,
    low_latency,
  ];

  static final $core.Map<$core.int, BlePowerSetting> _byValue = $pb.ProtobufEnum.initByValue(values);
  static BlePowerSetting? valueOf($core.int value) => _byValue[value];

  const BlePowerSetting._($core.int v, $core.String n) : super(v, n);
}

class BleError extends $pb.ProtobufEnum {
  static const BleError UNKNOWN_ERROR = BleError._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'UNKNOWN_ERROR');
  static const BleError RIGHTS_MISSING = BleError._(1, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'RIGHTS_MISSING');
  static const BleError TIMEOUT = BleError._(2, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'TIMEOUT');

  static const $core.List<BleError> values = <BleError> [
    UNKNOWN_ERROR,
    RIGHTS_MISSING,
    TIMEOUT,
  ];

  static final $core.Map<$core.int, BleError> _byValue = $pb.ProtobufEnum.initByValue(values);
  static BleError? valueOf($core.int value) => _byValue[value];

  const BleError._($core.int v, $core.String n) : super(v, n);
}

