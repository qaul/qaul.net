///
//  Generated code. Do not modify.
//  source: from_libqaul.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

// ignore_for_file: UNDEFINED_SHOWN_NAME
import 'dart:core' as $core;
import 'package:protobuf/protobuf.dart' as $pb;

class UserEntry_Connectivity extends $pb.ProtobufEnum {
  static const UserEntry_Connectivity Online = UserEntry_Connectivity._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'Online');
  static const UserEntry_Connectivity Reachable = UserEntry_Connectivity._(1, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'Reachable');
  static const UserEntry_Connectivity Offline = UserEntry_Connectivity._(2, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'Offline');

  static const $core.List<UserEntry_Connectivity> values = <UserEntry_Connectivity> [
    Online,
    Reachable,
    Offline,
  ];

  static final $core.Map<$core.int, UserEntry_Connectivity> _byValue = $pb.ProtobufEnum.initByValue(values);
  static UserEntry_Connectivity? valueOf($core.int value) => _byValue[value];

  const UserEntry_Connectivity._($core.int v, $core.String n) : super(v, n);
}

