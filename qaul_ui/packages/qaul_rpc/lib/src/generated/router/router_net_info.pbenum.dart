//
//  Generated code. Do not modify.
//  source: router/router_net_info.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

/// RouterInfoModule
class RouterInfoModule extends $pb.ProtobufEnum {
  static const RouterInfoModule ROUTER_INFO = RouterInfoModule._(0, _omitEnumNames ? '' : 'ROUTER_INFO');
  static const RouterInfoModule FEED_REQUEST = RouterInfoModule._(1, _omitEnumNames ? '' : 'FEED_REQUEST');
  static const RouterInfoModule FEED_RESPONSE = RouterInfoModule._(2, _omitEnumNames ? '' : 'FEED_RESPONSE');
  static const RouterInfoModule USER_REQUEST = RouterInfoModule._(3, _omitEnumNames ? '' : 'USER_REQUEST');
  static const RouterInfoModule USER_RESPONSE = RouterInfoModule._(4, _omitEnumNames ? '' : 'USER_RESPONSE');

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


const _omitEnumNames = $core.bool.fromEnvironment('protobuf.omit_enum_names');
