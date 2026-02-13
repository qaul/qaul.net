// This is a generated file - do not edit.
//
// Generated from rpc/qaul_rpc.proto.

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

@$core.Deprecated('Use modulesDescriptor instead')
const Modules$json = {
  '1': 'Modules',
  '2': [
    {'1': 'NONE', '2': 0},
    {'1': 'RPC', '2': 1},
    {'1': 'NODE', '2': 2},
    {'1': 'USERACCOUNTS', '2': 3},
    {'1': 'USERS', '2': 4},
    {'1': 'ROUTER', '2': 5},
    {'1': 'FEED', '2': 6},
    {'1': 'CONNECTIONS', '2': 7},
    {'1': 'DEBUG', '2': 8},
    {'1': 'GROUP', '2': 9},
    {'1': 'CHAT', '2': 10},
    {'1': 'CHATFILE', '2': 11},
    {'1': 'BLE', '2': 12},
    {'1': 'RTC', '2': 13},
    {'1': 'DTN', '2': 14},
    {'1': 'AUTH', '2': 15},
  ],
};

/// Descriptor for `Modules`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List modulesDescriptor = $convert.base64Decode(
    'CgdNb2R1bGVzEggKBE5PTkUQABIHCgNSUEMQARIICgROT0RFEAISEAoMVVNFUkFDQ09VTlRTEA'
    'MSCQoFVVNFUlMQBBIKCgZST1VURVIQBRIICgRGRUVEEAYSDwoLQ09OTkVDVElPTlMQBxIJCgVE'
    'RUJVRxAIEgkKBUdST1VQEAkSCAoEQ0hBVBAKEgwKCENIQVRGSUxFEAsSBwoDQkxFEAwSBwoDUl'
    'RDEA0SBwoDRFROEA4SCAoEQVVUSBAP');

@$core.Deprecated('Use qaulRpcDescriptor instead')
const QaulRpc$json = {
  '1': 'QaulRpc',
  '2': [
    {
      '1': 'module',
      '3': 1,
      '4': 1,
      '5': 14,
      '6': '.qaul.rpc.Modules',
      '10': 'module'
    },
    {'1': 'request_id', '3': 2, '4': 1, '5': 9, '10': 'requestId'},
    {'1': 'user_id', '3': 3, '4': 1, '5': 12, '10': 'userId'},
    {'1': 'data', '3': 4, '4': 1, '5': 12, '10': 'data'},
  ],
};

/// Descriptor for `QaulRpc`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List qaulRpcDescriptor = $convert.base64Decode(
    'CgdRYXVsUnBjEikKBm1vZHVsZRgBIAEoDjIRLnFhdWwucnBjLk1vZHVsZXNSBm1vZHVsZRIdCg'
    'pyZXF1ZXN0X2lkGAIgASgJUglyZXF1ZXN0SWQSFwoHdXNlcl9pZBgDIAEoDFIGdXNlcklkEhIK'
    'BGRhdGEYBCABKAxSBGRhdGE=');
