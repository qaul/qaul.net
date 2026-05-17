// This is a generated file - do not edit.
//
// Generated from common/common.proto.

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

@$core.Deprecated('Use ackDescriptor instead')
const Ack$json = {
  '1': 'Ack',
};

/// Descriptor for `Ack`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List ackDescriptor = $convert.base64Decode('CgNBY2s=');

@$core.Deprecated('Use rpcErrorDescriptor instead')
const RpcError$json = {
  '1': 'RpcError',
  '2': [
    {'1': 'code', '3': 1, '4': 1, '5': 13, '10': 'code'},
    {'1': 'message', '3': 2, '4': 1, '5': 9, '10': 'message'},
    {'1': 'details', '3': 3, '4': 1, '5': 9, '10': 'details'},
  ],
};

/// Descriptor for `RpcError`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List rpcErrorDescriptor = $convert.base64Decode(
    'CghScGNFcnJvchISCgRjb2RlGAEgASgNUgRjb2RlEhgKB21lc3NhZ2UYAiABKAlSB21lc3NhZ2'
    'USGAoHZGV0YWlscxgDIAEoCVIHZGV0YWlscw==');
