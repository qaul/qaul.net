///
//  Generated code. Do not modify.
//  source: connections/ble/ble_net.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,deprecated_member_use_from_same_package,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use bleMessageDescriptor instead')
const BleMessage$json = const {
  '1': 'BleMessage',
  '2': const [
    const {'1': 'info', '3': 1, '4': 1, '5': 12, '9': 0, '10': 'info'},
    const {'1': 'feed', '3': 2, '4': 1, '5': 12, '9': 0, '10': 'feed'},
    const {'1': 'messaging', '3': 3, '4': 1, '5': 12, '9': 0, '10': 'messaging'},
    const {'1': 'identification', '3': 4, '4': 1, '5': 11, '6': '.qaul.net.ble.Identification', '9': 0, '10': 'identification'},
  ],
  '8': const [
    const {'1': 'message'},
  ],
};

/// Descriptor for `BleMessage`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List bleMessageDescriptor = $convert.base64Decode('CgpCbGVNZXNzYWdlEhQKBGluZm8YASABKAxIAFIEaW5mbxIUCgRmZWVkGAIgASgMSABSBGZlZWQSHgoJbWVzc2FnaW5nGAMgASgMSABSCW1lc3NhZ2luZxJGCg5pZGVudGlmaWNhdGlvbhgEIAEoCzIcLnFhdWwubmV0LmJsZS5JZGVudGlmaWNhdGlvbkgAUg5pZGVudGlmaWNhdGlvbkIJCgdtZXNzYWdl');
@$core.Deprecated('Use identificationDescriptor instead')
const Identification$json = const {
  '1': 'Identification',
  '2': const [
    const {'1': 'request', '3': 1, '4': 1, '5': 8, '10': 'request'},
    const {'1': 'node', '3': 2, '4': 1, '5': 11, '6': '.qaul.net.ble.NodeIdentification', '10': 'node'},
  ],
};

/// Descriptor for `Identification`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List identificationDescriptor = $convert.base64Decode('Cg5JZGVudGlmaWNhdGlvbhIYCgdyZXF1ZXN0GAEgASgIUgdyZXF1ZXN0EjQKBG5vZGUYAiABKAsyIC5xYXVsLm5ldC5ibGUuTm9kZUlkZW50aWZpY2F0aW9uUgRub2Rl');
@$core.Deprecated('Use nodeIdentificationDescriptor instead')
const NodeIdentification$json = const {
  '1': 'NodeIdentification',
  '2': const [
    const {'1': 'id', '3': 1, '4': 1, '5': 12, '10': 'id'},
  ],
};

/// Descriptor for `NodeIdentification`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List nodeIdentificationDescriptor = $convert.base64Decode('ChJOb2RlSWRlbnRpZmljYXRpb24SDgoCaWQYASABKAxSAmlk');
