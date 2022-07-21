///
//  Generated code. Do not modify.
//  source: router/router_net_info.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

// ignore_for_file: UNDEFINED_SHOWN_NAME
import 'dart:core' as $core;
import 'package:protobuf/protobuf.dart' as $pb;

class RouterInfoModule extends $pb.ProtobufEnum {
  static const RouterInfoModule ROUTER_INFO = RouterInfoModule._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'ROUTER_INFO');
  static const RouterInfoModule FEED_REQUEST = RouterInfoModule._(1, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'FEED_REQUEST');
  static const RouterInfoModule FEED_RESPONSE = RouterInfoModule._(2, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'FEED_RESPONSE');

  static const $core.List<RouterInfoModule> values = <RouterInfoModule> [
    ROUTER_INFO,
    FEED_REQUEST,
    FEED_RESPONSE,
  ];

  static final $core.Map<$core.int, RouterInfoModule> _byValue = $pb.ProtobufEnum.initByValue(values);
  static RouterInfoModule? valueOf($core.int value) => _byValue[value];

  const RouterInfoModule._($core.int v, $core.String n) : super(v, n);
}

