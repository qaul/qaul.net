// This is a generated file - do not edit.
//
// Generated from router/router_net_info.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

/// RouterInfoModule
class RouterInfoModule extends $pb.ProtobufEnum {
  /// Message is a common RouterInfoMessage
  static const RouterInfoModule ROUTER_INFO =
      RouterInfoModule._(0, _omitEnumNames ? '' : 'ROUTER_INFO');

  /// Message is a FeedRequestMessage
  static const RouterInfoModule FEED_REQUEST =
      RouterInfoModule._(1, _omitEnumNames ? '' : 'FEED_REQUEST');

  /// Message is a FeedResponseMessage
  static const RouterInfoModule FEED_RESPONSE =
      RouterInfoModule._(2, _omitEnumNames ? '' : 'FEED_RESPONSE');

  /// Message is a UserRequestMessage
  static const RouterInfoModule USER_REQUEST =
      RouterInfoModule._(3, _omitEnumNames ? '' : 'USER_REQUEST');

  /// Message is a UserResponseMessage
  static const RouterInfoModule USER_RESPONSE =
      RouterInfoModule._(4, _omitEnumNames ? '' : 'USER_RESPONSE');

  static const $core.List<RouterInfoModule> values = <RouterInfoModule>[
    ROUTER_INFO,
    FEED_REQUEST,
    FEED_RESPONSE,
    USER_REQUEST,
    USER_RESPONSE,
  ];

  static final $core.List<RouterInfoModule?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 4);
  static RouterInfoModule? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const RouterInfoModule._(super.value, super.name);
}

const $core.bool _omitEnumNames =
    $core.bool.fromEnvironment('protobuf.omit_enum_names');
