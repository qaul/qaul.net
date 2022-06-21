///
//  Generated code. Do not modify.
//  source: router/router_net_info.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields,deprecated_member_use_from_same_package

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use routerInfoContainerDescriptor instead')
const RouterInfoContainer$json = const {
  '1': 'RouterInfoContainer',
  '2': const [
    const {'1': 'signature', '3': 1, '4': 1, '5': 12, '10': 'signature'},
    const {'1': 'message', '3': 2, '4': 1, '5': 12, '10': 'message'},
  ],
};

/// Descriptor for `RouterInfoContainer`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List routerInfoContainerDescriptor = $convert.base64Decode('ChNSb3V0ZXJJbmZvQ29udGFpbmVyEhwKCXNpZ25hdHVyZRgBIAEoDFIJc2lnbmF0dXJlEhgKB21lc3NhZ2UYAiABKAxSB21lc3NhZ2U=');
@$core.Deprecated('Use routerInfoContentDescriptor instead')
const RouterInfoContent$json = const {
  '1': 'RouterInfoContent',
  '2': const [
    const {'1': 'id', '3': 1, '4': 1, '5': 12, '10': 'id'},
    const {'1': 'content', '3': 2, '4': 1, '5': 12, '10': 'content'},
    const {'1': 'time', '3': 3, '4': 1, '5': 4, '10': 'time'},
  ],
};

/// Descriptor for `RouterInfoContent`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List routerInfoContentDescriptor = $convert.base64Decode('ChFSb3V0ZXJJbmZvQ29udGVudBIOCgJpZBgBIAEoDFICaWQSGAoHY29udGVudBgCIAEoDFIHY29udGVudBISCgR0aW1lGAMgASgEUgR0aW1l');
@$core.Deprecated('Use routerInfoMessageDescriptor instead')
const RouterInfoMessage$json = const {
  '1': 'RouterInfoMessage',
  '2': const [
    const {'1': 'node', '3': 1, '4': 1, '5': 12, '10': 'node'},
    const {'1': 'routes', '3': 2, '4': 1, '5': 11, '6': '.qaul.net.router_net_info.RoutingInfoTable', '10': 'routes'},
    const {'1': 'users', '3': 3, '4': 1, '5': 11, '6': '.qaul.net.router_net_info.UserInfoTable', '10': 'users'},
    const {'1': 'timestamp', '3': 4, '4': 1, '5': 4, '10': 'timestamp'},
  ],
};

/// Descriptor for `RouterInfoMessage`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List routerInfoMessageDescriptor = $convert.base64Decode('ChFSb3V0ZXJJbmZvTWVzc2FnZRISCgRub2RlGAEgASgMUgRub2RlEkIKBnJvdXRlcxgCIAEoCzIqLnFhdWwubmV0LnJvdXRlcl9uZXRfaW5mby5Sb3V0aW5nSW5mb1RhYmxlUgZyb3V0ZXMSPQoFdXNlcnMYAyABKAsyJy5xYXVsLm5ldC5yb3V0ZXJfbmV0X2luZm8uVXNlckluZm9UYWJsZVIFdXNlcnMSHAoJdGltZXN0YW1wGAQgASgEUgl0aW1lc3RhbXA=');
@$core.Deprecated('Use routingDescriptor instead')
const Routing$json = const {
  '1': 'Routing',
  '2': const [
    const {'1': 'user_info_table', '3': 1, '4': 1, '5': 11, '6': '.qaul.net.router_net_info.UserInfoTable', '9': 0, '10': 'userInfoTable'},
    const {'1': 'user_info', '3': 2, '4': 1, '5': 11, '6': '.qaul.net.router_net_info.UserInfo', '9': 0, '10': 'userInfo'},
    const {'1': 'routing_info_table', '3': 3, '4': 1, '5': 11, '6': '.qaul.net.router_net_info.RoutingInfoTable', '9': 0, '10': 'routingInfoTable'},
    const {'1': 'routing_info_entry', '3': 4, '4': 1, '5': 11, '6': '.qaul.net.router_net_info.RoutingInfoEntry', '9': 0, '10': 'routingInfoEntry'},
  ],
  '8': const [
    const {'1': 'message'},
  ],
};

/// Descriptor for `Routing`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List routingDescriptor = $convert.base64Decode('CgdSb3V0aW5nElEKD3VzZXJfaW5mb190YWJsZRgBIAEoCzInLnFhdWwubmV0LnJvdXRlcl9uZXRfaW5mby5Vc2VySW5mb1RhYmxlSABSDXVzZXJJbmZvVGFibGUSQQoJdXNlcl9pbmZvGAIgASgLMiIucWF1bC5uZXQucm91dGVyX25ldF9pbmZvLlVzZXJJbmZvSABSCHVzZXJJbmZvEloKEnJvdXRpbmdfaW5mb190YWJsZRgDIAEoCzIqLnFhdWwubmV0LnJvdXRlcl9uZXRfaW5mby5Sb3V0aW5nSW5mb1RhYmxlSABSEHJvdXRpbmdJbmZvVGFibGUSWgoScm91dGluZ19pbmZvX2VudHJ5GAQgASgLMioucWF1bC5uZXQucm91dGVyX25ldF9pbmZvLlJvdXRpbmdJbmZvRW50cnlIAFIQcm91dGluZ0luZm9FbnRyeUIJCgdtZXNzYWdl');
@$core.Deprecated('Use routingInfoTableDescriptor instead')
const RoutingInfoTable$json = const {
  '1': 'RoutingInfoTable',
  '2': const [
    const {'1': 'entry', '3': 1, '4': 3, '5': 11, '6': '.qaul.net.router_net_info.RoutingInfoEntry', '10': 'entry'},
  ],
};

/// Descriptor for `RoutingInfoTable`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List routingInfoTableDescriptor = $convert.base64Decode('ChBSb3V0aW5nSW5mb1RhYmxlEkAKBWVudHJ5GAEgAygLMioucWF1bC5uZXQucm91dGVyX25ldF9pbmZvLlJvdXRpbmdJbmZvRW50cnlSBWVudHJ5');
@$core.Deprecated('Use routingInfoEntryDescriptor instead')
const RoutingInfoEntry$json = const {
  '1': 'RoutingInfoEntry',
  '2': const [
    const {'1': 'user', '3': 1, '4': 1, '5': 12, '10': 'user'},
    const {'1': 'rtt', '3': 2, '4': 1, '5': 13, '10': 'rtt'},
    const {'1': 'hc', '3': 3, '4': 1, '5': 12, '10': 'hc'},
    const {'1': 'pl', '3': 4, '4': 1, '5': 2, '10': 'pl'},
  ],
};

/// Descriptor for `RoutingInfoEntry`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List routingInfoEntryDescriptor = $convert.base64Decode('ChBSb3V0aW5nSW5mb0VudHJ5EhIKBHVzZXIYASABKAxSBHVzZXISEAoDcnR0GAIgASgNUgNydHQSDgoCaGMYAyABKAxSAmhjEg4KAnBsGAQgASgCUgJwbA==');
@$core.Deprecated('Use userInfoTableDescriptor instead')
const UserInfoTable$json = const {
  '1': 'UserInfoTable',
  '2': const [
    const {'1': 'info', '3': 1, '4': 3, '5': 11, '6': '.qaul.net.router_net_info.UserInfo', '10': 'info'},
  ],
};

/// Descriptor for `UserInfoTable`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List userInfoTableDescriptor = $convert.base64Decode('Cg1Vc2VySW5mb1RhYmxlEjYKBGluZm8YASADKAsyIi5xYXVsLm5ldC5yb3V0ZXJfbmV0X2luZm8uVXNlckluZm9SBGluZm8=');
@$core.Deprecated('Use userInfoDescriptor instead')
const UserInfo$json = const {
  '1': 'UserInfo',
  '2': const [
    const {'1': 'id', '3': 1, '4': 1, '5': 12, '10': 'id'},
    const {'1': 'key', '3': 2, '4': 1, '5': 12, '10': 'key'},
    const {'1': 'name', '3': 3, '4': 1, '5': 9, '10': 'name'},
  ],
};

/// Descriptor for `UserInfo`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List userInfoDescriptor = $convert.base64Decode('CghVc2VySW5mbxIOCgJpZBgBIAEoDFICaWQSEAoDa2V5GAIgASgMUgNrZXkSEgoEbmFtZRgDIAEoCVIEbmFtZQ==');
