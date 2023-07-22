//
//  Generated code. Do not modify.
//  source: router/router_net_info.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use routerInfoModuleDescriptor instead')
const RouterInfoModule$json = {
  '1': 'RouterInfoModule',
  '2': [
    {'1': 'ROUTER_INFO', '2': 0},
    {'1': 'FEED_REQUEST', '2': 1},
    {'1': 'FEED_RESPONSE', '2': 2},
    {'1': 'USER_REQUEST', '2': 3},
    {'1': 'USER_RESPONSE', '2': 4},
  ],
};

/// Descriptor for `RouterInfoModule`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List routerInfoModuleDescriptor = $convert.base64Decode(
    'ChBSb3V0ZXJJbmZvTW9kdWxlEg8KC1JPVVRFUl9JTkZPEAASEAoMRkVFRF9SRVFVRVNUEAESEQ'
    'oNRkVFRF9SRVNQT05TRRACEhAKDFVTRVJfUkVRVUVTVBADEhEKDVVTRVJfUkVTUE9OU0UQBA==');

@$core.Deprecated('Use routerInfoContainerDescriptor instead')
const RouterInfoContainer$json = {
  '1': 'RouterInfoContainer',
  '2': [
    {'1': 'signature', '3': 1, '4': 1, '5': 12, '10': 'signature'},
    {'1': 'message', '3': 2, '4': 1, '5': 12, '10': 'message'},
  ],
};

/// Descriptor for `RouterInfoContainer`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List routerInfoContainerDescriptor = $convert.base64Decode(
    'ChNSb3V0ZXJJbmZvQ29udGFpbmVyEhwKCXNpZ25hdHVyZRgBIAEoDFIJc2lnbmF0dXJlEhgKB2'
    '1lc3NhZ2UYAiABKAxSB21lc3NhZ2U=');

@$core.Deprecated('Use routerInfoContentDescriptor instead')
const RouterInfoContent$json = {
  '1': 'RouterInfoContent',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 12, '10': 'id'},
    {'1': 'routerInfoModule', '3': 2, '4': 1, '5': 14, '6': '.qaul.net.router_net_info.RouterInfoModule', '10': 'routerInfoModule'},
    {'1': 'content', '3': 3, '4': 1, '5': 12, '10': 'content'},
    {'1': 'time', '3': 4, '4': 1, '5': 4, '10': 'time'},
  ],
};

/// Descriptor for `RouterInfoContent`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List routerInfoContentDescriptor = $convert.base64Decode(
    'ChFSb3V0ZXJJbmZvQ29udGVudBIOCgJpZBgBIAEoDFICaWQSVgoQcm91dGVySW5mb01vZHVsZR'
    'gCIAEoDjIqLnFhdWwubmV0LnJvdXRlcl9uZXRfaW5mby5Sb3V0ZXJJbmZvTW9kdWxlUhByb3V0'
    'ZXJJbmZvTW9kdWxlEhgKB2NvbnRlbnQYAyABKAxSB2NvbnRlbnQSEgoEdGltZRgEIAEoBFIEdG'
    'ltZQ==');

@$core.Deprecated('Use routerInfoMessageDescriptor instead')
const RouterInfoMessage$json = {
  '1': 'RouterInfoMessage',
  '2': [
    {'1': 'node', '3': 1, '4': 1, '5': 12, '10': 'node'},
    {'1': 'routes', '3': 2, '4': 1, '5': 11, '6': '.qaul.net.router_net_info.RoutingInfoTable', '10': 'routes'},
    {'1': 'feeds', '3': 4, '4': 1, '5': 11, '6': '.qaul.net.router_net_info.FeedIdsTable', '10': 'feeds'},
    {'1': 'timestamp', '3': 5, '4': 1, '5': 4, '10': 'timestamp'},
  ],
};

/// Descriptor for `RouterInfoMessage`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List routerInfoMessageDescriptor = $convert.base64Decode(
    'ChFSb3V0ZXJJbmZvTWVzc2FnZRISCgRub2RlGAEgASgMUgRub2RlEkIKBnJvdXRlcxgCIAEoCz'
    'IqLnFhdWwubmV0LnJvdXRlcl9uZXRfaW5mby5Sb3V0aW5nSW5mb1RhYmxlUgZyb3V0ZXMSPAoF'
    'ZmVlZHMYBCABKAsyJi5xYXVsLm5ldC5yb3V0ZXJfbmV0X2luZm8uRmVlZElkc1RhYmxlUgVmZW'
    'VkcxIcCgl0aW1lc3RhbXAYBSABKARSCXRpbWVzdGFtcA==');

@$core.Deprecated('Use routingInfoTableDescriptor instead')
const RoutingInfoTable$json = {
  '1': 'RoutingInfoTable',
  '2': [
    {'1': 'entry', '3': 1, '4': 3, '5': 11, '6': '.qaul.net.router_net_info.RoutingInfoEntry', '10': 'entry'},
  ],
};

/// Descriptor for `RoutingInfoTable`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List routingInfoTableDescriptor = $convert.base64Decode(
    'ChBSb3V0aW5nSW5mb1RhYmxlEkAKBWVudHJ5GAEgAygLMioucWF1bC5uZXQucm91dGVyX25ldF'
    '9pbmZvLlJvdXRpbmdJbmZvRW50cnlSBWVudHJ5');

@$core.Deprecated('Use routingInfoEntryDescriptor instead')
const RoutingInfoEntry$json = {
  '1': 'RoutingInfoEntry',
  '2': [
    {'1': 'user', '3': 1, '4': 1, '5': 12, '10': 'user'},
    {'1': 'rtt', '3': 2, '4': 1, '5': 13, '10': 'rtt'},
    {'1': 'hc', '3': 3, '4': 1, '5': 12, '10': 'hc'},
    {'1': 'pgid', '3': 5, '4': 1, '5': 13, '10': 'pgid'},
  ],
};

/// Descriptor for `RoutingInfoEntry`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List routingInfoEntryDescriptor = $convert.base64Decode(
    'ChBSb3V0aW5nSW5mb0VudHJ5EhIKBHVzZXIYASABKAxSBHVzZXISEAoDcnR0GAIgASgNUgNydH'
    'QSDgoCaGMYAyABKAxSAmhjEhIKBHBnaWQYBSABKA1SBHBnaWQ=');

@$core.Deprecated('Use userIdTableDescriptor instead')
const UserIdTable$json = {
  '1': 'UserIdTable',
  '2': [
    {'1': 'ids', '3': 1, '4': 3, '5': 12, '10': 'ids'},
  ],
};

/// Descriptor for `UserIdTable`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List userIdTableDescriptor = $convert.base64Decode(
    'CgtVc2VySWRUYWJsZRIQCgNpZHMYASADKAxSA2lkcw==');

@$core.Deprecated('Use userInfoTableDescriptor instead')
const UserInfoTable$json = {
  '1': 'UserInfoTable',
  '2': [
    {'1': 'info', '3': 1, '4': 3, '5': 11, '6': '.qaul.net.router_net_info.UserInfo', '10': 'info'},
  ],
};

/// Descriptor for `UserInfoTable`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List userInfoTableDescriptor = $convert.base64Decode(
    'Cg1Vc2VySW5mb1RhYmxlEjYKBGluZm8YASADKAsyIi5xYXVsLm5ldC5yb3V0ZXJfbmV0X2luZm'
    '8uVXNlckluZm9SBGluZm8=');

@$core.Deprecated('Use userInfoDescriptor instead')
const UserInfo$json = {
  '1': 'UserInfo',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 12, '10': 'id'},
    {'1': 'key', '3': 2, '4': 1, '5': 12, '10': 'key'},
    {'1': 'name', '3': 3, '4': 1, '5': 9, '10': 'name'},
  ],
};

/// Descriptor for `UserInfo`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List userInfoDescriptor = $convert.base64Decode(
    'CghVc2VySW5mbxIOCgJpZBgBIAEoDFICaWQSEAoDa2V5GAIgASgMUgNrZXkSEgoEbmFtZRgDIA'
    'EoCVIEbmFtZQ==');

@$core.Deprecated('Use feedIdsTableDescriptor instead')
const FeedIdsTable$json = {
  '1': 'FeedIdsTable',
  '2': [
    {'1': 'ids', '3': 1, '4': 3, '5': 12, '10': 'ids'},
  ],
};

/// Descriptor for `FeedIdsTable`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List feedIdsTableDescriptor = $convert.base64Decode(
    'CgxGZWVkSWRzVGFibGUSEAoDaWRzGAEgAygMUgNpZHM=');

@$core.Deprecated('Use feedRequestMessageDescriptor instead')
const FeedRequestMessage$json = {
  '1': 'FeedRequestMessage',
  '2': [
    {'1': 'feeds', '3': 1, '4': 1, '5': 11, '6': '.qaul.net.router_net_info.FeedIdsTable', '10': 'feeds'},
  ],
};

/// Descriptor for `FeedRequestMessage`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List feedRequestMessageDescriptor = $convert.base64Decode(
    'ChJGZWVkUmVxdWVzdE1lc3NhZ2USPAoFZmVlZHMYASABKAsyJi5xYXVsLm5ldC5yb3V0ZXJfbm'
    'V0X2luZm8uRmVlZElkc1RhYmxlUgVmZWVkcw==');

@$core.Deprecated('Use feedResponseMessageDescriptor instead')
const FeedResponseMessage$json = {
  '1': 'FeedResponseMessage',
  '2': [
    {'1': 'feeds', '3': 1, '4': 1, '5': 11, '6': '.qaul.net.router_net_info.FeedResponseTable', '10': 'feeds'},
  ],
};

/// Descriptor for `FeedResponseMessage`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List feedResponseMessageDescriptor = $convert.base64Decode(
    'ChNGZWVkUmVzcG9uc2VNZXNzYWdlEkEKBWZlZWRzGAEgASgLMisucWF1bC5uZXQucm91dGVyX2'
    '5ldF9pbmZvLkZlZWRSZXNwb25zZVRhYmxlUgVmZWVkcw==');

@$core.Deprecated('Use feedResponseTableDescriptor instead')
const FeedResponseTable$json = {
  '1': 'FeedResponseTable',
  '2': [
    {'1': 'messages', '3': 1, '4': 3, '5': 11, '6': '.qaul.net.router_net_info.FeedMessage', '10': 'messages'},
  ],
};

/// Descriptor for `FeedResponseTable`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List feedResponseTableDescriptor = $convert.base64Decode(
    'ChFGZWVkUmVzcG9uc2VUYWJsZRJBCghtZXNzYWdlcxgBIAMoCzIlLnFhdWwubmV0LnJvdXRlcl'
    '9uZXRfaW5mby5GZWVkTWVzc2FnZVIIbWVzc2FnZXM=');

@$core.Deprecated('Use feedMessageDescriptor instead')
const FeedMessage$json = {
  '1': 'FeedMessage',
  '2': [
    {'1': 'message_id', '3': 1, '4': 1, '5': 12, '10': 'messageId'},
    {'1': 'sender_id', '3': 2, '4': 1, '5': 12, '10': 'senderId'},
    {'1': 'content', '3': 3, '4': 1, '5': 9, '10': 'content'},
    {'1': 'time', '3': 4, '4': 1, '5': 4, '10': 'time'},
  ],
};

/// Descriptor for `FeedMessage`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List feedMessageDescriptor = $convert.base64Decode(
    'CgtGZWVkTWVzc2FnZRIdCgptZXNzYWdlX2lkGAEgASgMUgltZXNzYWdlSWQSGwoJc2VuZGVyX2'
    'lkGAIgASgMUghzZW5kZXJJZBIYCgdjb250ZW50GAMgASgJUgdjb250ZW50EhIKBHRpbWUYBCAB'
    'KARSBHRpbWU=');

