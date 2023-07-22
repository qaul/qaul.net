//
//  Generated code. Do not modify.
//  source: connections/ble/ble_net.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use bleMessageDescriptor instead')
const BleMessage$json = {
  '1': 'BleMessage',
  '2': [
    {'1': 'info', '3': 1, '4': 1, '5': 12, '9': 0, '10': 'info'},
    {'1': 'feed', '3': 2, '4': 1, '5': 12, '9': 0, '10': 'feed'},
    {'1': 'messaging', '3': 3, '4': 1, '5': 12, '9': 0, '10': 'messaging'},
    {'1': 'identification', '3': 4, '4': 1, '5': 11, '6': '.qaul.net.ble.Identification', '9': 0, '10': 'identification'},
  ],
  '8': [
    {'1': 'message'},
  ],
};

/// Descriptor for `BleMessage`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List bleMessageDescriptor = $convert.base64Decode(
    'CgpCbGVNZXNzYWdlEhQKBGluZm8YASABKAxIAFIEaW5mbxIUCgRmZWVkGAIgASgMSABSBGZlZW'
    'QSHgoJbWVzc2FnaW5nGAMgASgMSABSCW1lc3NhZ2luZxJGCg5pZGVudGlmaWNhdGlvbhgEIAEo'
    'CzIcLnFhdWwubmV0LmJsZS5JZGVudGlmaWNhdGlvbkgAUg5pZGVudGlmaWNhdGlvbkIJCgdtZX'
    'NzYWdl');

@$core.Deprecated('Use identificationDescriptor instead')
const Identification$json = {
  '1': 'Identification',
  '2': [
    {'1': 'request', '3': 1, '4': 1, '5': 8, '10': 'request'},
    {'1': 'node', '3': 2, '4': 1, '5': 11, '6': '.qaul.net.ble.NodeIdentification', '10': 'node'},
  ],
};

/// Descriptor for `Identification`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List identificationDescriptor = $convert.base64Decode(
    'Cg5JZGVudGlmaWNhdGlvbhIYCgdyZXF1ZXN0GAEgASgIUgdyZXF1ZXN0EjQKBG5vZGUYAiABKA'
    'syIC5xYXVsLm5ldC5ibGUuTm9kZUlkZW50aWZpY2F0aW9uUgRub2Rl');

@$core.Deprecated('Use nodeIdentificationDescriptor instead')
const NodeIdentification$json = {
  '1': 'NodeIdentification',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 12, '10': 'id'},
  ],
};

/// Descriptor for `NodeIdentification`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List nodeIdentificationDescriptor = $convert.base64Decode(
    'ChJOb2RlSWRlbnRpZmljYXRpb24SDgoCaWQYASABKAxSAmlk');

