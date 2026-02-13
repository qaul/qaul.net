// This is a generated file - do not edit.
//
// Generated from connections/ble/ble_rpc.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports
// ignore_for_file: unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use bleDescriptor instead')
const Ble$json = {
  '1': 'Ble',
  '2': [
    {
      '1': 'info_request',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.qaul.rpc.ble.InfoRequest',
      '9': 0,
      '10': 'infoRequest'
    },
    {
      '1': 'info_response',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.qaul.rpc.ble.InfoResponse',
      '9': 0,
      '10': 'infoResponse'
    },
    {
      '1': 'start_request',
      '3': 3,
      '4': 1,
      '5': 11,
      '6': '.qaul.rpc.ble.StartRequest',
      '9': 0,
      '10': 'startRequest'
    },
    {
      '1': 'stop_request',
      '3': 4,
      '4': 1,
      '5': 11,
      '6': '.qaul.rpc.ble.StopRequest',
      '9': 0,
      '10': 'stopRequest'
    },
    {
      '1': 'discovered_request',
      '3': 5,
      '4': 1,
      '5': 11,
      '6': '.qaul.rpc.ble.DiscoveredRequest',
      '9': 0,
      '10': 'discoveredRequest'
    },
    {
      '1': 'discovered_response',
      '3': 6,
      '4': 1,
      '5': 11,
      '6': '.qaul.rpc.ble.DiscoveredResponse',
      '9': 0,
      '10': 'discoveredResponse'
    },
    {
      '1': 'rights_request',
      '3': 7,
      '4': 1,
      '5': 11,
      '6': '.qaul.rpc.ble.RightsRequest',
      '9': 0,
      '10': 'rightsRequest'
    },
    {
      '1': 'rights_result',
      '3': 8,
      '4': 1,
      '5': 11,
      '6': '.qaul.rpc.ble.RightsResult',
      '9': 0,
      '10': 'rightsResult'
    },
  ],
  '8': [
    {'1': 'message'},
  ],
};

/// Descriptor for `Ble`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List bleDescriptor = $convert.base64Decode(
    'CgNCbGUSPgoMaW5mb19yZXF1ZXN0GAEgASgLMhkucWF1bC5ycGMuYmxlLkluZm9SZXF1ZXN0SA'
    'BSC2luZm9SZXF1ZXN0EkEKDWluZm9fcmVzcG9uc2UYAiABKAsyGi5xYXVsLnJwYy5ibGUuSW5m'
    'b1Jlc3BvbnNlSABSDGluZm9SZXNwb25zZRJBCg1zdGFydF9yZXF1ZXN0GAMgASgLMhoucWF1bC'
    '5ycGMuYmxlLlN0YXJ0UmVxdWVzdEgAUgxzdGFydFJlcXVlc3QSPgoMc3RvcF9yZXF1ZXN0GAQg'
    'ASgLMhkucWF1bC5ycGMuYmxlLlN0b3BSZXF1ZXN0SABSC3N0b3BSZXF1ZXN0ElAKEmRpc2Nvdm'
    'VyZWRfcmVxdWVzdBgFIAEoCzIfLnFhdWwucnBjLmJsZS5EaXNjb3ZlcmVkUmVxdWVzdEgAUhFk'
    'aXNjb3ZlcmVkUmVxdWVzdBJTChNkaXNjb3ZlcmVkX3Jlc3BvbnNlGAYgASgLMiAucWF1bC5ycG'
    'MuYmxlLkRpc2NvdmVyZWRSZXNwb25zZUgAUhJkaXNjb3ZlcmVkUmVzcG9uc2USRAoOcmlnaHRz'
    'X3JlcXVlc3QYByABKAsyGy5xYXVsLnJwYy5ibGUuUmlnaHRzUmVxdWVzdEgAUg1yaWdodHNSZX'
    'F1ZXN0EkEKDXJpZ2h0c19yZXN1bHQYCCABKAsyGi5xYXVsLnJwYy5ibGUuUmlnaHRzUmVzdWx0'
    'SABSDHJpZ2h0c1Jlc3VsdEIJCgdtZXNzYWdl');

@$core.Deprecated('Use infoRequestDescriptor instead')
const InfoRequest$json = {
  '1': 'InfoRequest',
};

/// Descriptor for `InfoRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List infoRequestDescriptor =
    $convert.base64Decode('CgtJbmZvUmVxdWVzdA==');

@$core.Deprecated('Use infoResponseDescriptor instead')
const InfoResponse$json = {
  '1': 'InfoResponse',
  '2': [
    {'1': 'small_id', '3': 1, '4': 1, '5': 12, '10': 'smallId'},
    {'1': 'status', '3': 2, '4': 1, '5': 9, '10': 'status'},
    {'1': 'device_info', '3': 3, '4': 1, '5': 12, '10': 'deviceInfo'},
  ],
};

/// Descriptor for `InfoResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List infoResponseDescriptor = $convert.base64Decode(
    'CgxJbmZvUmVzcG9uc2USGQoIc21hbGxfaWQYASABKAxSB3NtYWxsSWQSFgoGc3RhdHVzGAIgAS'
    'gJUgZzdGF0dXMSHwoLZGV2aWNlX2luZm8YAyABKAxSCmRldmljZUluZm8=');

@$core.Deprecated('Use startRequestDescriptor instead')
const StartRequest$json = {
  '1': 'StartRequest',
};

/// Descriptor for `StartRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List startRequestDescriptor =
    $convert.base64Decode('CgxTdGFydFJlcXVlc3Q=');

@$core.Deprecated('Use stopRequestDescriptor instead')
const StopRequest$json = {
  '1': 'StopRequest',
};

/// Descriptor for `StopRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List stopRequestDescriptor =
    $convert.base64Decode('CgtTdG9wUmVxdWVzdA==');

@$core.Deprecated('Use discoveredRequestDescriptor instead')
const DiscoveredRequest$json = {
  '1': 'DiscoveredRequest',
};

/// Descriptor for `DiscoveredRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List discoveredRequestDescriptor =
    $convert.base64Decode('ChFEaXNjb3ZlcmVkUmVxdWVzdA==');

@$core.Deprecated('Use discoveredResponseDescriptor instead')
const DiscoveredResponse$json = {
  '1': 'DiscoveredResponse',
  '2': [
    {'1': 'nodes_count', '3': 1, '4': 1, '5': 13, '10': 'nodesCount'},
    {'1': 'to_confirm_count', '3': 2, '4': 1, '5': 13, '10': 'toConfirmCount'},
  ],
};

/// Descriptor for `DiscoveredResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List discoveredResponseDescriptor = $convert.base64Decode(
    'ChJEaXNjb3ZlcmVkUmVzcG9uc2USHwoLbm9kZXNfY291bnQYASABKA1SCm5vZGVzQ291bnQSKA'
    'oQdG9fY29uZmlybV9jb3VudBgCIAEoDVIOdG9Db25maXJtQ291bnQ=');

@$core.Deprecated('Use rightsRequestDescriptor instead')
const RightsRequest$json = {
  '1': 'RightsRequest',
};

/// Descriptor for `RightsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List rightsRequestDescriptor =
    $convert.base64Decode('Cg1SaWdodHNSZXF1ZXN0');

@$core.Deprecated('Use rightsResultDescriptor instead')
const RightsResult$json = {
  '1': 'RightsResult',
  '2': [
    {'1': 'rights_granted', '3': 1, '4': 1, '5': 8, '10': 'rightsGranted'},
  ],
};

/// Descriptor for `RightsResult`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List rightsResultDescriptor = $convert.base64Decode(
    'CgxSaWdodHNSZXN1bHQSJQoOcmlnaHRzX2dyYW50ZWQYASABKAhSDXJpZ2h0c0dyYW50ZWQ=');
