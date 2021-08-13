///
//  Generated code. Do not modify.
//  source: router/router.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

// ignore_for_file: UNDEFINED_SHOWN_NAME
import 'dart:core' as $core;
import 'package:protobuf/protobuf.dart' as $pb;

class Connectivity extends $pb.ProtobufEnum {
  static const Connectivity Online = Connectivity._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'Online');
  static const Connectivity Reachable = Connectivity._(1, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'Reachable');
  static const Connectivity Offline = Connectivity._(2, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'Offline');

  static const $core.List<Connectivity> values = <Connectivity> [
    Online,
    Reachable,
    Offline,
  ];

  static final $core.Map<$core.int, Connectivity> _byValue = $pb.ProtobufEnum.initByValue(values);
  static Connectivity? valueOf($core.int value) => _byValue[value];

  const Connectivity._($core.int v, $core.String n) : super(v, n);
}

