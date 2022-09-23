///
//  Generated code. Do not modify.
//  source: connections/connections.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,deprecated_member_use_from_same_package,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use infoDescriptor instead')
const Info$json = const {
  '1': 'Info',
  '2': const [
    const {'1': 'REQUEST', '2': 0},
    const {'1': 'ADD_SUCCESS', '2': 1},
    const {'1': 'ADD_ERROR_INVALID', '2': 2},
    const {'1': 'REMOVE_SUCCESS', '2': 5},
    const {'1': 'STATE_SUCCESS', '2': 6},
    const {'1': 'REMOVE_ERROR_NOT_FOUND', '2': 7},
  ],
};

/// Descriptor for `Info`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List infoDescriptor = $convert.base64Decode('CgRJbmZvEgsKB1JFUVVFU1QQABIPCgtBRERfU1VDQ0VTUxABEhUKEUFERF9FUlJPUl9JTlZBTElEEAISEgoOUkVNT1ZFX1NVQ0NFU1MQBRIRCg1TVEFURV9TVUNDRVNTEAYSGgoWUkVNT1ZFX0VSUk9SX05PVF9GT1VORBAH');
@$core.Deprecated('Use connectionsDescriptor instead')
const Connections$json = const {
  '1': 'Connections',
  '2': const [
    const {'1': 'internet_nodes_request', '3': 1, '4': 1, '5': 11, '6': '.qaul.rpc.connections.InternetNodesRequest', '9': 0, '10': 'internetNodesRequest'},
    const {'1': 'internet_nodes_list', '3': 2, '4': 1, '5': 11, '6': '.qaul.rpc.connections.InternetNodesList', '9': 0, '10': 'internetNodesList'},
    const {'1': 'internet_nodes_add', '3': 3, '4': 1, '5': 11, '6': '.qaul.rpc.connections.InternetNodesEntry', '9': 0, '10': 'internetNodesAdd'},
    const {'1': 'internet_nodes_remove', '3': 4, '4': 1, '5': 11, '6': '.qaul.rpc.connections.InternetNodesEntry', '9': 0, '10': 'internetNodesRemove'},
    const {'1': 'internet_nodes_state', '3': 5, '4': 1, '5': 11, '6': '.qaul.rpc.connections.InternetNodesEntry', '9': 0, '10': 'internetNodesState'},
  ],
  '8': const [
    const {'1': 'message'},
  ],
};

/// Descriptor for `Connections`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List connectionsDescriptor = $convert.base64Decode('CgtDb25uZWN0aW9ucxJiChZpbnRlcm5ldF9ub2Rlc19yZXF1ZXN0GAEgASgLMioucWF1bC5ycGMuY29ubmVjdGlvbnMuSW50ZXJuZXROb2Rlc1JlcXVlc3RIAFIUaW50ZXJuZXROb2Rlc1JlcXVlc3QSWQoTaW50ZXJuZXRfbm9kZXNfbGlzdBgCIAEoCzInLnFhdWwucnBjLmNvbm5lY3Rpb25zLkludGVybmV0Tm9kZXNMaXN0SABSEWludGVybmV0Tm9kZXNMaXN0ElgKEmludGVybmV0X25vZGVzX2FkZBgDIAEoCzIoLnFhdWwucnBjLmNvbm5lY3Rpb25zLkludGVybmV0Tm9kZXNFbnRyeUgAUhBpbnRlcm5ldE5vZGVzQWRkEl4KFWludGVybmV0X25vZGVzX3JlbW92ZRgEIAEoCzIoLnFhdWwucnBjLmNvbm5lY3Rpb25zLkludGVybmV0Tm9kZXNFbnRyeUgAUhNpbnRlcm5ldE5vZGVzUmVtb3ZlElwKFGludGVybmV0X25vZGVzX3N0YXRlGAUgASgLMigucWF1bC5ycGMuY29ubmVjdGlvbnMuSW50ZXJuZXROb2Rlc0VudHJ5SABSEmludGVybmV0Tm9kZXNTdGF0ZUIJCgdtZXNzYWdl');
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
    const {'1': 'info', '3': 1, '4': 1, '5': 14, '6': '.qaul.rpc.connections.Info', '10': 'info'},
    const {'1': 'nodes', '3': 2, '4': 3, '5': 11, '6': '.qaul.rpc.connections.InternetNodesEntry', '10': 'nodes'},
  ],
};

/// Descriptor for `InternetNodesList`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List internetNodesListDescriptor = $convert.base64Decode('ChFJbnRlcm5ldE5vZGVzTGlzdBIuCgRpbmZvGAEgASgOMhoucWF1bC5ycGMuY29ubmVjdGlvbnMuSW5mb1IEaW5mbxI+CgVub2RlcxgCIAMoCzIoLnFhdWwucnBjLmNvbm5lY3Rpb25zLkludGVybmV0Tm9kZXNFbnRyeVIFbm9kZXM=');
@$core.Deprecated('Use internetNodesEntryDescriptor instead')
const InternetNodesEntry$json = const {
  '1': 'InternetNodesEntry',
  '2': const [
    const {'1': 'address', '3': 1, '4': 1, '5': 9, '10': 'address'},
    const {'1': 'enabled', '3': 2, '4': 1, '5': 8, '10': 'enabled'},
  ],
};

/// Descriptor for `InternetNodesEntry`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List internetNodesEntryDescriptor = $convert.base64Decode('ChJJbnRlcm5ldE5vZGVzRW50cnkSGAoHYWRkcmVzcxgBIAEoCVIHYWRkcmVzcxIYCgdlbmFibGVkGAIgASgIUgdlbmFibGVk');
