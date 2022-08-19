///
//  Generated code. Do not modify.
//  source: node/node.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,deprecated_member_use_from_same_package,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use nodeDescriptor instead')
const Node$json = const {
  '1': 'Node',
  '2': const [
    const {'1': 'get_node_info', '3': 1, '4': 1, '5': 8, '9': 0, '10': 'getNodeInfo'},
    const {'1': 'info', '3': 2, '4': 1, '5': 11, '6': '.qaul.rpc.node.NodeInformation', '9': 0, '10': 'info'},
  ],
  '8': const [
    const {'1': 'message'},
  ],
};

/// Descriptor for `Node`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List nodeDescriptor = $convert.base64Decode('CgROb2RlEiQKDWdldF9ub2RlX2luZm8YASABKAhIAFILZ2V0Tm9kZUluZm8SNAoEaW5mbxgCIAEoCzIeLnFhdWwucnBjLm5vZGUuTm9kZUluZm9ybWF0aW9uSABSBGluZm9CCQoHbWVzc2FnZQ==');
@$core.Deprecated('Use nodeInformationDescriptor instead')
const NodeInformation$json = const {
  '1': 'NodeInformation',
  '2': const [
    const {'1': 'id_base58', '3': 1, '4': 1, '5': 9, '10': 'idBase58'},
    const {'1': 'addresses', '3': 2, '4': 3, '5': 9, '10': 'addresses'},
  ],
};

/// Descriptor for `NodeInformation`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List nodeInformationDescriptor = $convert.base64Decode('Cg9Ob2RlSW5mb3JtYXRpb24SGwoJaWRfYmFzZTU4GAEgASgJUghpZEJhc2U1OBIcCglhZGRyZXNzZXMYAiADKAlSCWFkZHJlc3Nlcw==');
