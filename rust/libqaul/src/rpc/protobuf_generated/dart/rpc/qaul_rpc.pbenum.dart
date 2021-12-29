///
//  Generated code. Do not modify.
//  source: rpc/qaul_rpc.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

// ignore_for_file: UNDEFINED_SHOWN_NAME
import 'dart:core' as $core;
import 'package:protobuf/protobuf.dart' as $pb;

class Modules extends $pb.ProtobufEnum {
  static const Modules NONE = Modules._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'NONE');
  static const Modules RPC = Modules._(1, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'RPC');
  static const Modules NODE = Modules._(2, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'NODE');
  static const Modules USERACCOUNTS = Modules._(3, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'USERACCOUNTS');
  static const Modules USERS = Modules._(4, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'USERS');
  static const Modules ROUTER = Modules._(5, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'ROUTER');
  static const Modules FEED = Modules._(6, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'FEED');
  static const Modules CONNECTIONS = Modules._(7, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'CONNECTIONS');
  static const Modules DEBUG = Modules._(8, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'DEBUG');
  static const Modules CHAT = Modules._(9, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'CHAT');

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
    CHAT,
  ];

  static final $core.Map<$core.int, Modules> _byValue = $pb.ProtobufEnum.initByValue(values);
  static Modules? valueOf($core.int value) => _byValue[value];

  const Modules._($core.int v, $core.String n) : super(v, n);
}

