///
//  Generated code. Do not modify.
//  source: router/router_net_info.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

// ignore_for_file: UNDEFINED_SHOWN_NAME
import 'dart:core' as $core;
import 'package:protobuf/protobuf.dart' as $pb;

class RouterInfoModule extends $pb.ProtobufEnum {
  static const RouterInfoModule ROUTER_INFO = RouterInfoModule._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'ROUTER_INFO');
  static const RouterInfoModule FEED_REQUEST = RouterInfoModule._(1, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'FEED_REQUEST');
  static const RouterInfoModule FEED_RESPONSE = RouterInfoModule._(2, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'FEED_RESPONSE');
  static const RouterInfoModule USER_REQUEST = RouterInfoModule._(3, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'USER_REQUEST');
  static const RouterInfoModule USER_RESPONSE = RouterInfoModule._(4, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'USER_RESPONSE');

  static const $core.List<RouterInfoModule> values = <RouterInfoModule> [
    ROUTER_INFO,
    FEED_REQUEST,
    FEED_RESPONSE,
    USER_REQUEST,
    USER_RESPONSE,
  ];

  static final $core.Map<$core.int, RouterInfoModule> _byValue = $pb.ProtobufEnum.initByValue(values);
  static RouterInfoModule? valueOf($core.int value) => _byValue[value];

  const RouterInfoModule._($core.int v, $core.String n) : super(v, n);
}

