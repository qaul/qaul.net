//
//  Generated code. Do not modify.
//  source: connections/ble/ble.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

/// power settings
///
/// These power settings relate to the android
/// power modes.
class BlePowerSetting extends $pb.ProtobufEnum {
  static const BlePowerSetting low_power = BlePowerSetting._(0, _omitEnumNames ? '' : 'low_power');
  static const BlePowerSetting balanced = BlePowerSetting._(1, _omitEnumNames ? '' : 'balanced');
  static const BlePowerSetting low_latency = BlePowerSetting._(2, _omitEnumNames ? '' : 'low_latency');

  static const $core.List<BlePowerSetting> values = <BlePowerSetting> [
    low_power,
    balanced,
    low_latency,
  ];

  static final $core.Map<$core.int, BlePowerSetting> _byValue = $pb.ProtobufEnum.initByValue(values);
  static BlePowerSetting? valueOf($core.int value) => _byValue[value];

  const BlePowerSetting._($core.int v, $core.String n) : super(v, n);
}

///  BLE Error Reasons
///
///  TODO: this list needs to be completed
///        if none of the reasons apply, use
///        UNKNOWN_ERROR
class BleError extends $pb.ProtobufEnum {
  static const BleError UNKNOWN_ERROR = BleError._(0, _omitEnumNames ? '' : 'UNKNOWN_ERROR');
  static const BleError RIGHTS_MISSING = BleError._(1, _omitEnumNames ? '' : 'RIGHTS_MISSING');
  static const BleError TIMEOUT = BleError._(2, _omitEnumNames ? '' : 'TIMEOUT');

  static const $core.List<BleError> values = <BleError> [
    UNKNOWN_ERROR,
    RIGHTS_MISSING,
    TIMEOUT,
  ];

  static final $core.Map<$core.int, BleError> _byValue = $pb.ProtobufEnum.initByValue(values);
  static BleError? valueOf($core.int value) => _byValue[value];

  const BleError._($core.int v, $core.String n) : super(v, n);
}


const _omitEnumNames = $core.bool.fromEnvironment('protobuf.omit_enum_names');
