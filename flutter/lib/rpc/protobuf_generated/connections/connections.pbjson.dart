///
//  Generated code. Do not modify.
//  source: connections/connections.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields,deprecated_member_use_from_same_package

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use connectionsDescriptor instead')
const Connections$json = const {
  '1': 'Connections',
  '2': const [
    const {'1': 'internet_nodes_request', '3': 1, '4': 1, '5': 11, '6': '.qaul.rpc.connections.InternetNodesRequest', '9': 0, '10': 'internetNodesRequest'},
    const {'1': 'internet_nodes_list', '3': 2, '4': 1, '5': 11, '6': '.qaul.rpc.connections.InternetNodesList', '9': 0, '10': 'internetNodesList'},
    const {'1': 'internet_nodes_add', '3': 3, '4': 1, '5': 11, '6': '.qaul.rpc.connections.InternetNodesEntry', '9': 0, '10': 'internetNodesAdd'},
    const {'1': 'internet_nodes_remove', '3': 4, '4': 1, '5': 11, '6': '.qaul.rpc.connections.InternetNodesEntry', '9': 0, '10': 'internetNodesRemove'},
  ],
  '8': const [
    const {'1': 'message'},
  ],
};

/// Descriptor for `Connections`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List connectionsDescriptor = $convert.base64Decode('CgtDb25uZWN0aW9ucxJiChZpbnRlcm5ldF9ub2Rlc19yZXF1ZXN0GAEgASgLMioucWF1bC5ycGMuY29ubmVjdGlvbnMuSW50ZXJuZXROb2Rlc1JlcXVlc3RIAFIUaW50ZXJuZXROb2Rlc1JlcXVlc3QSWQoTaW50ZXJuZXRfbm9kZXNfbGlzdBgCIAEoCzInLnFhdWwucnBjLmNvbm5lY3Rpb25zLkludGVybmV0Tm9kZXNMaXN0SABSEWludGVybmV0Tm9kZXNMaXN0ElgKEmludGVybmV0X25vZGVzX2FkZBgDIAEoCzIoLnFhdWwucnBjLmNvbm5lY3Rpb25zLkludGVybmV0Tm9kZXNFbnRyeUgAUhBpbnRlcm5ldE5vZGVzQWRkEl4KFWludGVybmV0X25vZGVzX3JlbW92ZRgEIAEoCzIoLnFhdWwucnBjLmNvbm5lY3Rpb25zLkludGVybmV0Tm9kZXNFbnRyeUgAUhNpbnRlcm5ldE5vZGVzUmVtb3ZlQgkKB21lc3NhZ2U=');
@$core.Deprecated('Use internetNodesRequestDescriptor instead')
const InternetNodesRequest$json = const {
  '1': 'InternetNodesRequest',
};

/// Descriptor for `InternetNodesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List internetNodesRequestDescriptor = $convert.base64Decode('ChRJbnRlcm5ldE5vZGVzUmVxdWVzdA==');
@$core.Deprecated('Use internetNodesListDescriptor instead')
const InternetNodesList$json = const {
  '1': 'InternetNodesList',
  '2': const [
    const {'1': 'nodes', '3': 1, '4': 3, '5': 11, '6': '.qaul.rpc.connections.InternetNodesEntry', '10': 'nodes'},
  ],
};

/// Descriptor for `InternetNodesList`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List internetNodesListDescriptor = $convert.base64Decode('ChFJbnRlcm5ldE5vZGVzTGlzdBI+CgVub2RlcxgBIAMoCzIoLnFhdWwucnBjLmNvbm5lY3Rpb25zLkludGVybmV0Tm9kZXNFbnRyeVIFbm9kZXM=');
@$core.Deprecated('Use internetNodesEntryDescriptor instead')
const InternetNodesEntry$json = const {
  '1': 'InternetNodesEntry',
  '2': const [
    const {'1': 'address', '3': 1, '4': 1, '5': 9, '10': 'address'},
  ],
};

/// Descriptor for `InternetNodesEntry`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List internetNodesEntryDescriptor = $convert.base64Decode('ChJJbnRlcm5ldE5vZGVzRW50cnkSGAoHYWRkcmVzcxgBIAEoCVIHYWRkcmVzcw==');
