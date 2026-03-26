// This is a generated file - do not edit.
//
// Generated from connections/ble/ble_net.proto.

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

@$core.Deprecated('Use bleMessageDescriptor instead')
const BleMessage$json = {
  '1': 'BleMessage',
  '2': [
    {'1': 'info', '3': 1, '4': 1, '5': 12, '9': 0, '10': 'info'},
    {'1': 'feed', '3': 2, '4': 1, '5': 12, '9': 0, '10': 'feed'},
    {'1': 'messaging', '3': 3, '4': 1, '5': 12, '9': 0, '10': 'messaging'},
    {
      '1': 'identification',
      '3': 4,
      '4': 1,
      '5': 11,
      '6': '.qaul.net.ble.Identification',
      '9': 0,
      '10': 'identification'
    },
    {
      '1': 'encrypted',
      '3': 5,
      '4': 1,
      '5': 11,
      '6': '.qaul.net.ble.EncryptedBleTransport',
      '9': 0,
      '10': 'encrypted'
    },
    {
      '1': 'handshake',
      '3': 6,
      '4': 1,
      '5': 11,
      '6': '.qaul.net.ble.NoiseHandshake',
      '9': 0,
      '10': 'handshake'
    },
  ],
  '8': [
    {'1': 'message'},
  ],
};

/// Descriptor for `BleMessage`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List bleMessageDescriptor = $convert.base64Decode(
    'CgpCbGVNZXNzYWdlEhQKBGluZm8YASABKAxIAFIEaW5mbxIUCgRmZWVkGAIgASgMSABSBGZlZW'
    'QSHgoJbWVzc2FnaW5nGAMgASgMSABSCW1lc3NhZ2luZxJGCg5pZGVudGlmaWNhdGlvbhgEIAEo'
    'CzIcLnFhdWwubmV0LmJsZS5JZGVudGlmaWNhdGlvbkgAUg5pZGVudGlmaWNhdGlvbhJDCgllbm'
    'NyeXB0ZWQYBSABKAsyIy5xYXVsLm5ldC5ibGUuRW5jcnlwdGVkQmxlVHJhbnNwb3J0SABSCWVu'
    'Y3J5cHRlZBI8CgloYW5kc2hha2UYBiABKAsyHC5xYXVsLm5ldC5ibGUuTm9pc2VIYW5kc2hha2'
    'VIAFIJaGFuZHNoYWtlQgkKB21lc3NhZ2U=');

@$core.Deprecated('Use identificationDescriptor instead')
const Identification$json = {
  '1': 'Identification',
  '2': [
    {'1': 'request', '3': 1, '4': 1, '5': 8, '10': 'request'},
    {
      '1': 'node',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.qaul.net.ble.NodeIdentification',
      '10': 'node'
    },
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
final $typed_data.Uint8List nodeIdentificationDescriptor =
    $convert.base64Decode('ChJOb2RlSWRlbnRpZmljYXRpb24SDgoCaWQYASABKAxSAmlk');

@$core.Deprecated('Use encryptedBleTransportDescriptor instead')
const EncryptedBleTransport$json = {
  '1': 'EncryptedBleTransport',
  '2': [
    {'1': 'session_id', '3': 1, '4': 1, '5': 13, '10': 'sessionId'},
    {'1': 'nonce', '3': 2, '4': 1, '5': 4, '10': 'nonce'},
    {'1': 'ciphertext', '3': 3, '4': 1, '5': 12, '10': 'ciphertext'},
  ],
};

/// Descriptor for `EncryptedBleTransport`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List encryptedBleTransportDescriptor = $convert.base64Decode(
    'ChVFbmNyeXB0ZWRCbGVUcmFuc3BvcnQSHQoKc2Vzc2lvbl9pZBgBIAEoDVIJc2Vzc2lvbklkEh'
    'QKBW5vbmNlGAIgASgEUgVub25jZRIeCgpjaXBoZXJ0ZXh0GAMgASgMUgpjaXBoZXJ0ZXh0');

@$core.Deprecated('Use noiseHandshakeDescriptor instead')
const NoiseHandshake$json = {
  '1': 'NoiseHandshake',
  '2': [
    {'1': 'session_id', '3': 1, '4': 1, '5': 13, '10': 'sessionId'},
    {'1': 'message_number', '3': 2, '4': 1, '5': 13, '10': 'messageNumber'},
    {'1': 'payload', '3': 3, '4': 1, '5': 12, '10': 'payload'},
  ],
};

/// Descriptor for `NoiseHandshake`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List noiseHandshakeDescriptor = $convert.base64Decode(
    'Cg5Ob2lzZUhhbmRzaGFrZRIdCgpzZXNzaW9uX2lkGAEgASgNUglzZXNzaW9uSWQSJQoObWVzc2'
    'FnZV9udW1iZXIYAiABKA1SDW1lc3NhZ2VOdW1iZXISGAoHcGF5bG9hZBgDIAEoDFIHcGF5bG9h'
    'ZA==');
