//
//  Generated code. Do not modify.
//  source: router/users.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

/// Connection modules
class ConnectionModule extends $pb.ProtobufEnum {
  static const ConnectionModule NONE = ConnectionModule._(0, _omitEnumNames ? '' : 'NONE');
  static const ConnectionModule LAN = ConnectionModule._(1, _omitEnumNames ? '' : 'LAN');
  static const ConnectionModule INTERNET = ConnectionModule._(2, _omitEnumNames ? '' : 'INTERNET');
  static const ConnectionModule BLE = ConnectionModule._(3, _omitEnumNames ? '' : 'BLE');
  static const ConnectionModule LOCAL = ConnectionModule._(4, _omitEnumNames ? '' : 'LOCAL');

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

/// how is the user connected
class Connectivity extends $pb.ProtobufEnum {
  static const Connectivity Online = Connectivity._(0, _omitEnumNames ? '' : 'Online');
  static const Connectivity Reachable = Connectivity._(1, _omitEnumNames ? '' : 'Reachable');
  static const Connectivity Offline = Connectivity._(2, _omitEnumNames ? '' : 'Offline');

  static const $core.List<Connectivity> values = <Connectivity> [
    Online,
    Reachable,
    Offline,
  ];

  static final $core.Map<$core.int, Connectivity> _byValue = $pb.ProtobufEnum.initByValue(values);
  static Connectivity? valueOf($core.int value) => _byValue[value];

  const Connectivity._($core.int v, $core.String n) : super(v, n);
}


const _omitEnumNames = $core.bool.fromEnvironment('protobuf.omit_enum_names');
