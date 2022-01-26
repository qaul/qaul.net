///
//  Generated code. Do not modify.
//  source: rpc/qaul_rpc.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields,deprecated_member_use_from_same_package

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use modulesDescriptor instead')
const Modules$json = const {
  '1': 'Modules',
  '2': const [
    const {'1': 'NONE', '2': 0},
    const {'1': 'RPC', '2': 1},
    const {'1': 'NODE', '2': 2},
    const {'1': 'USERACCOUNTS', '2': 3},
    const {'1': 'USERS', '2': 4},
    const {'1': 'ROUTER', '2': 5},
    const {'1': 'FEED', '2': 6},
    const {'1': 'CONNECTIONS', '2': 7},
    const {'1': 'DEBUG', '2': 8},
  ],
};

/// Descriptor for `Modules`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List modulesDescriptor = $convert.base64Decode('CgdNb2R1bGVzEggKBE5PTkUQABIHCgNSUEMQARIICgROT0RFEAISEAoMVVNFUkFDQ09VTlRTEAMSCQoFVVNFUlMQBBIKCgZST1VURVIQBRIICgRGRUVEEAYSDwoLQ09OTkVDVElPTlMQBxIJCgVERUJVRxAI');
@$core.Deprecated('Use qaulRpcDescriptor instead')
const QaulRpc$json = const {
  '1': 'QaulRpc',
  '2': const [
    const {'1': 'module', '3': 1, '4': 1, '5': 14, '6': '.qaul.rpc.Modules', '10': 'module'},
    const {'1': 'request_id', '3': 2, '4': 1, '5': 9, '10': 'requestId'},
    const {'1': 'user_id', '3': 3, '4': 1, '5': 12, '10': 'userId'},
    const {'1': 'data', '3': 4, '4': 1, '5': 12, '10': 'data'},
  ],
};

/// Descriptor for `QaulRpc`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List qaulRpcDescriptor = $convert.base64Decode('CgdRYXVsUnBjEikKBm1vZHVsZRgBIAEoDjIRLnFhdWwucnBjLk1vZHVsZXNSBm1vZHVsZRIdCgpyZXF1ZXN0X2lkGAIgASgJUglyZXF1ZXN0SWQSFwoHdXNlcl9pZBgDIAEoDFIGdXNlcklkEhIKBGRhdGEYBCABKAxSBGRhdGE=');
