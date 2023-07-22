//
//  Generated code. Do not modify.
//  source: rpc/qaul_rpc.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class Modules extends $pb.ProtobufEnum {
  static const Modules NONE = Modules._(0, _omitEnumNames ? '' : 'NONE');
  static const Modules RPC = Modules._(1, _omitEnumNames ? '' : 'RPC');
  static const Modules NODE = Modules._(2, _omitEnumNames ? '' : 'NODE');
  static const Modules USERACCOUNTS = Modules._(3, _omitEnumNames ? '' : 'USERACCOUNTS');
  static const Modules USERS = Modules._(4, _omitEnumNames ? '' : 'USERS');
  static const Modules ROUTER = Modules._(5, _omitEnumNames ? '' : 'ROUTER');
  static const Modules FEED = Modules._(6, _omitEnumNames ? '' : 'FEED');
  static const Modules CONNECTIONS = Modules._(7, _omitEnumNames ? '' : 'CONNECTIONS');
  static const Modules DEBUG = Modules._(8, _omitEnumNames ? '' : 'DEBUG');
  static const Modules GROUP = Modules._(9, _omitEnumNames ? '' : 'GROUP');
  static const Modules CHAT = Modules._(10, _omitEnumNames ? '' : 'CHAT');
  static const Modules CHATFILE = Modules._(11, _omitEnumNames ? '' : 'CHATFILE');
  static const Modules BLE = Modules._(12, _omitEnumNames ? '' : 'BLE');
  static const Modules RTC = Modules._(13, _omitEnumNames ? '' : 'RTC');
  static const Modules DTN = Modules._(14, _omitEnumNames ? '' : 'DTN');

  static const $core.List<Modules> values = <Modules> [
    NONE,
    RPC,
    NODE,
    USERACCOUNTS,
    USERS,
    ROUTER,
    FEED,
    CONNECTIONS,
    DEBUG,
    GROUP,
    CHAT,
    CHATFILE,
    BLE,
    RTC,
    DTN,
  ];

  static final $core.Map<$core.int, Modules> _byValue = $pb.ProtobufEnum.initByValue(values);
  static Modules? valueOf($core.int value) => _byValue[value];

  const Modules._($core.int v, $core.String n) : super(v, n);
}


const _omitEnumNames = $core.bool.fromEnvironment('protobuf.omit_enum_names');
