///
//  Generated code. Do not modify.
//  source: connections/ble/ble_rpc.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields,deprecated_member_use_from_same_package

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use bleDescriptor instead')
const Ble$json = const {
  '1': 'Ble',
  '2': const [
    const {'1': 'info_request', '3': 1, '4': 1, '5': 11, '6': '.qaul.rpc.ble.InfoRequest', '9': 0, '10': 'infoRequest'},
    const {'1': 'info_response', '3': 2, '4': 1, '5': 11, '6': '.qaul.rpc.ble.InfoResponse', '9': 0, '10': 'infoResponse'},
    const {'1': 'start_request', '3': 3, '4': 1, '5': 11, '6': '.qaul.rpc.ble.StartRequest', '9': 0, '10': 'startRequest'},
    const {'1': 'stop_request', '3': 4, '4': 1, '5': 11, '6': '.qaul.rpc.ble.StopRequest', '9': 0, '10': 'stopRequest'},
    const {'1': 'discovered_request', '3': 5, '4': 1, '5': 11, '6': '.qaul.rpc.ble.DiscoveredRequest', '9': 0, '10': 'discoveredRequest'},
    const {'1': 'discovered_response', '3': 6, '4': 1, '5': 11, '6': '.qaul.rpc.ble.DiscoveredResponse', '9': 0, '10': 'discoveredResponse'},
  ],
  '8': const [
    const {'1': 'message'},
  ],
};

/// Descriptor for `Ble`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List bleDescriptor = $convert.base64Decode('CgNCbGUSPgoMaW5mb19yZXF1ZXN0GAEgASgLMhkucWF1bC5ycGMuYmxlLkluZm9SZXF1ZXN0SABSC2luZm9SZXF1ZXN0EkEKDWluZm9fcmVzcG9uc2UYAiABKAsyGi5xYXVsLnJwYy5ibGUuSW5mb1Jlc3BvbnNlSABSDGluZm9SZXNwb25zZRJBCg1zdGFydF9yZXF1ZXN0GAMgASgLMhoucWF1bC5ycGMuYmxlLlN0YXJ0UmVxdWVzdEgAUgxzdGFydFJlcXVlc3QSPgoMc3RvcF9yZXF1ZXN0GAQgASgLMhkucWF1bC5ycGMuYmxlLlN0b3BSZXF1ZXN0SABSC3N0b3BSZXF1ZXN0ElAKEmRpc2NvdmVyZWRfcmVxdWVzdBgFIAEoCzIfLnFhdWwucnBjLmJsZS5EaXNjb3ZlcmVkUmVxdWVzdEgAUhFkaXNjb3ZlcmVkUmVxdWVzdBJTChNkaXNjb3ZlcmVkX3Jlc3BvbnNlGAYgASgLMiAucWF1bC5ycGMuYmxlLkRpc2NvdmVyZWRSZXNwb25zZUgAUhJkaXNjb3ZlcmVkUmVzcG9uc2VCCQoHbWVzc2FnZQ==');
@$core.Deprecated('Use infoRequestDescriptor instead')
const InfoRequest$json = const {
  '1': 'InfoRequest',
};

/// Descriptor for `InfoRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List infoRequestDescriptor = $convert.base64Decode('CgtJbmZvUmVxdWVzdA==');
@$core.Deprecated('Use infoResponseDescriptor instead')
const InfoResponse$json = const {
  '1': 'InfoResponse',
  '2': const [
    const {'1': 'small_id', '3': 1, '4': 1, '5': 12, '10': 'smallId'},
    const {'1': 'status', '3': 2, '4': 1, '5': 9, '10': 'status'},
    const {'1': 'device_info', '3': 3, '4': 1, '5': 12, '10': 'deviceInfo'},
  ],
};

/// Descriptor for `InfoResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List infoResponseDescriptor = $convert.base64Decode('CgxJbmZvUmVzcG9uc2USGQoIc21hbGxfaWQYASABKAxSB3NtYWxsSWQSFgoGc3RhdHVzGAIgASgJUgZzdGF0dXMSHwoLZGV2aWNlX2luZm8YAyABKAxSCmRldmljZUluZm8=');
@$core.Deprecated('Use startRequestDescriptor instead')
const StartRequest$json = const {
  '1': 'StartRequest',
};

/// Descriptor for `StartRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List startRequestDescriptor = $convert.base64Decode('CgxTdGFydFJlcXVlc3Q=');
@$core.Deprecated('Use stopRequestDescriptor instead')
const StopRequest$json = const {
  '1': 'StopRequest',
};

/// Descriptor for `StopRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List stopRequestDescriptor = $convert.base64Decode('CgtTdG9wUmVxdWVzdA==');
@$core.Deprecated('Use discoveredRequestDescriptor instead')
const DiscoveredRequest$json = const {
  '1': 'DiscoveredRequest',
};

/// Descriptor for `DiscoveredRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List discoveredRequestDescriptor = $convert.base64Decode('ChFEaXNjb3ZlcmVkUmVxdWVzdA==');
@$core.Deprecated('Use discoveredResponseDescriptor instead')
const DiscoveredResponse$json = const {
  '1': 'DiscoveredResponse',
  '2': const [
    const {'1': 'nodes_count', '3': 1, '4': 1, '5': 13, '10': 'nodesCount'},
    const {'1': 'to_confirm_count', '3': 2, '4': 1, '5': 13, '10': 'toConfirmCount'},
  ],
};

/// Descriptor for `DiscoveredResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List discoveredResponseDescriptor = $convert.base64Decode('ChJEaXNjb3ZlcmVkUmVzcG9uc2USHwoLbm9kZXNfY291bnQYASABKA1SCm5vZGVzQ291bnQSKAoQdG9fY29uZmlybV9jb3VudBgCIAEoDVIOdG9Db25maXJtQ291bnQ=');
