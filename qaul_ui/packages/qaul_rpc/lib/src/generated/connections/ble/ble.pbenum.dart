// This is a generated file - do not edit.
//
// Generated from connections/ble/ble.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

/// power settings
///
/// These power settings relate to the android
/// power modes.
class BlePowerSetting extends $pb.ProtobufEnum {
  /// use power saving option
  ///
  /// this option will miss a lot of incoming messages,
  /// as the processor is often sleeping
  static const BlePowerSetting low_power =
      BlePowerSetting._(0, _omitEnumNames ? '' : 'low_power');

  /// use a compromise between power
  /// saving and reactivity
  static const BlePowerSetting balanced =
      BlePowerSetting._(1, _omitEnumNames ? '' : 'balanced');

  /// always listen
  ///
  /// this option uses the most battery power
  static const BlePowerSetting low_latency =
      BlePowerSetting._(2, _omitEnumNames ? '' : 'low_latency');

  static const $core.List<BlePowerSetting> values = <BlePowerSetting>[
    low_power,
    balanced,
    low_latency,
  ];

  static final $core.List<BlePowerSetting?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 2);
  static BlePowerSetting? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const BlePowerSetting._(super.value, super.name);
}

/// BLE Error Reasons
///
/// TODO: this list needs to be completed
///       if none of the reasons apply, use
///       UNKNOWN_ERROR
class BleError extends $pb.ProtobufEnum {
  /// undefined error
  ///
  /// use this when no other reason applies
  static const BleError UNKNOWN_ERROR =
      BleError._(0, _omitEnumNames ? '' : 'UNKNOWN_ERROR');

  /// the rights to use BLE were
  /// not provided by the user
  static const BleError RIGHTS_MISSING =
      BleError._(1, _omitEnumNames ? '' : 'RIGHTS_MISSING');

  /// there was a module timeout
  static const BleError TIMEOUT =
      BleError._(2, _omitEnumNames ? '' : 'TIMEOUT');

  static const $core.List<BleError> values = <BleError>[
    UNKNOWN_ERROR,
    RIGHTS_MISSING,
    TIMEOUT,
  ];

  static final $core.List<BleError?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 2);
  static BleError? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const BleError._(super.value, super.name);
}

const $core.bool _omitEnumNames =
    $core.bool.fromEnvironment('protobuf.omit_enum_names');
