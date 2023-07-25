///
//  Generated code. Do not modify.
//  source: router/router_net_info.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,deprecated_member_use_from_same_package,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use routerInfoModuleDescriptor instead')
const RouterInfoModule$json = const {
  '1': 'RouterInfoModule',
  '2': const [
    const {'1': 'ROUTER_INFO', '2': 0},
    const {'1': 'FEED_REQUEST', '2': 1},
    const {'1': 'FEED_RESPONSE', '2': 2},
    const {'1': 'USER_REQUEST', '2': 3},
    const {'1': 'USER_RESPONSE', '2': 4},
  ],
};

/// Descriptor for `RouterInfoModule`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List routerInfoModuleDescriptor = $convert.base64Decode('ChBSb3V0ZXJJbmZvTW9kdWxlEg8KC1JPVVRFUl9JTkZPEAASEAoMRkVFRF9SRVFVRVNUEAESEQoNRkVFRF9SRVNQT05TRRACEhAKDFVTRVJfUkVRVUVTVBADEhEKDVVTRVJfUkVTUE9OU0UQBA==');
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
    const {'1': 'routerInfoModule', '3': 2, '4': 1, '5': 14, '6': '.qaul.net.router_net_info.RouterInfoModule', '10': 'routerInfoModule'},
    const {'1': 'content', '3': 3, '4': 1, '5': 12, '10': 'content'},
    const {'1': 'time', '3': 4, '4': 1, '5': 4, '10': 'time'},
  ],
};

/// Descriptor for `RouterInfoContent`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List routerInfoContentDescriptor = $convert.base64Decode('ChFSb3V0ZXJJbmZvQ29udGVudBIOCgJpZBgBIAEoDFICaWQSVgoQcm91dGVySW5mb01vZHVsZRgCIAEoDjIqLnFhdWwubmV0LnJvdXRlcl9uZXRfaW5mby5Sb3V0ZXJJbmZvTW9kdWxlUhByb3V0ZXJJbmZvTW9kdWxlEhgKB2NvbnRlbnQYAyABKAxSB2NvbnRlbnQSEgoEdGltZRgEIAEoBFIEdGltZQ==');
@$core.Deprecated('Use routerInfoMessageDescriptor instead')
const RouterInfoMessage$json = const {
  '1': 'RouterInfoMessage',
  '2': const [
    const {'1': 'node', '3': 1, '4': 1, '5': 12, '10': 'node'},
    const {'1': 'routes', '3': 2, '4': 1, '5': 11, '6': '.qaul.net.router_net_info.RoutingInfoTable', '10': 'routes'},
    const {'1': 'feeds', '3': 4, '4': 1, '5': 11, '6': '.qaul.net.router_net_info.FeedIdsTable', '10': 'feeds'},
    const {'1': 'timestamp', '3': 5, '4': 1, '5': 4, '10': 'timestamp'},
  ],
};

/// Descriptor for `RouterInfoMessage`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List routerInfoMessageDescriptor = $convert.base64Decode('ChFSb3V0ZXJJbmZvTWVzc2FnZRISCgRub2RlGAEgASgMUgRub2RlEkIKBnJvdXRlcxgCIAEoCzIqLnFhdWwubmV0LnJvdXRlcl9uZXRfaW5mby5Sb3V0aW5nSW5mb1RhYmxlUgZyb3V0ZXMSPAoFZmVlZHMYBCABKAsyJi5xYXVsLm5ldC5yb3V0ZXJfbmV0X2luZm8uRmVlZElkc1RhYmxlUgVmZWVkcxIcCgl0aW1lc3RhbXAYBSABKARSCXRpbWVzdGFtcA==');
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
    const {'1': 'pgid', '3': 5, '4': 1, '5': 13, '10': 'pgid'},
  ],
};

/// Descriptor for `RoutingInfoEntry`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List routingInfoEntryDescriptor = $convert.base64Decode('ChBSb3V0aW5nSW5mb0VudHJ5EhIKBHVzZXIYASABKAxSBHVzZXISEAoDcnR0GAIgASgNUgNydHQSDgoCaGMYAyABKAxSAmhjEhIKBHBnaWQYBSABKA1SBHBnaWQ=');
@$core.Deprecated('Use userIdTableDescriptor instead')
const UserIdTable$json = const {
  '1': 'UserIdTable',
  '2': const [
    const {'1': 'ids', '3': 1, '4': 3, '5': 12, '10': 'ids'},
  ],
};

/// Descriptor for `UserIdTable`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List userIdTableDescriptor = $convert.base64Decode('CgtVc2VySWRUYWJsZRIQCgNpZHMYASADKAxSA2lkcw==');
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
@$core.Deprecated('Use feedIdsTableDescriptor instead')
const FeedIdsTable$json = const {
  '1': 'FeedIdsTable',
  '2': const [
    const {'1': 'ids', '3': 1, '4': 3, '5': 12, '10': 'ids'},
  ],
};

/// Descriptor for `FeedIdsTable`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List feedIdsTableDescriptor = $convert.base64Decode('CgxGZWVkSWRzVGFibGUSEAoDaWRzGAEgAygMUgNpZHM=');
@$core.Deprecated('Use feedRequestMessageDescriptor instead')
const FeedRequestMessage$json = const {
  '1': 'FeedRequestMessage',
  '2': const [
    const {'1': 'feeds', '3': 1, '4': 1, '5': 11, '6': '.qaul.net.router_net_info.FeedIdsTable', '10': 'feeds'},
  ],
};

/// Descriptor for `FeedRequestMessage`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List feedRequestMessageDescriptor = $convert.base64Decode('ChJGZWVkUmVxdWVzdE1lc3NhZ2USPAoFZmVlZHMYASABKAsyJi5xYXVsLm5ldC5yb3V0ZXJfbmV0X2luZm8uRmVlZElkc1RhYmxlUgVmZWVkcw==');
@$core.Deprecated('Use feedResponseMessageDescriptor instead')
const FeedResponseMessage$json = const {
  '1': 'FeedResponseMessage',
  '2': const [
    const {'1': 'feeds', '3': 1, '4': 1, '5': 11, '6': '.qaul.net.router_net_info.FeedResponseTable', '10': 'feeds'},
  ],
};

/// Descriptor for `FeedResponseMessage`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List feedResponseMessageDescriptor = $convert.base64Decode('ChNGZWVkUmVzcG9uc2VNZXNzYWdlEkEKBWZlZWRzGAEgASgLMisucWF1bC5uZXQucm91dGVyX25ldF9pbmZvLkZlZWRSZXNwb25zZVRhYmxlUgVmZWVkcw==');
@$core.Deprecated('Use feedResponseTableDescriptor instead')
const FeedResponseTable$json = const {
  '1': 'FeedResponseTable',
  '2': const [
    const {'1': 'messages', '3': 1, '4': 3, '5': 11, '6': '.qaul.net.router_net_info.FeedMessage', '10': 'messages'},
  ],
};

/// Descriptor for `FeedResponseTable`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List feedResponseTableDescriptor = $convert.base64Decode('ChFGZWVkUmVzcG9uc2VUYWJsZRJBCghtZXNzYWdlcxgBIAMoCzIlLnFhdWwubmV0LnJvdXRlcl9uZXRfaW5mby5GZWVkTWVzc2FnZVIIbWVzc2FnZXM=');
@$core.Deprecated('Use feedMessageDescriptor instead')
const FeedMessage$json = const {
  '1': 'FeedMessage',
  '2': const [
    const {'1': 'message_id', '3': 1, '4': 1, '5': 12, '10': 'messageId'},
    const {'1': 'sender_id', '3': 2, '4': 1, '5': 12, '10': 'senderId'},
    const {'1': 'content', '3': 3, '4': 1, '5': 9, '10': 'content'},
    const {'1': 'time', '3': 4, '4': 1, '5': 4, '10': 'time'},
  ],
};

/// Descriptor for `FeedMessage`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List feedMessageDescriptor = $convert.base64Decode('CgtGZWVkTWVzc2FnZRIdCgptZXNzYWdlX2lkGAEgASgMUgltZXNzYWdlSWQSGwoJc2VuZGVyX2lkGAIgASgMUghzZW5kZXJJZBIYCgdjb250ZW50GAMgASgJUgdjb250ZW50EhIKBHRpbWUYBCABKARSBHRpbWU=');
