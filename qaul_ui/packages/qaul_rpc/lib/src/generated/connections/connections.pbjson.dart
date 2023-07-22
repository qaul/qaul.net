//
//  Generated code. Do not modify.
//  source: connections/connections.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use infoDescriptor instead')
const Info$json = {
  '1': 'Info',
  '2': [
    {'1': 'REQUEST', '2': 0},
    {'1': 'ADD_SUCCESS', '2': 1},
    {'1': 'ADD_ERROR_INVALID', '2': 2},
    {'1': 'REMOVE_SUCCESS', '2': 5},
    {'1': 'STATE_SUCCESS', '2': 6},
    {'1': 'REMOVE_ERROR_NOT_FOUND', '2': 7},
  ],
};

/// Descriptor for `Info`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List infoDescriptor = $convert.base64Decode(
    'CgRJbmZvEgsKB1JFUVVFU1QQABIPCgtBRERfU1VDQ0VTUxABEhUKEUFERF9FUlJPUl9JTlZBTE'
    'lEEAISEgoOUkVNT1ZFX1NVQ0NFU1MQBRIRCg1TVEFURV9TVUNDRVNTEAYSGgoWUkVNT1ZFX0VS'
    'Uk9SX05PVF9GT1VORBAH');

@$core.Deprecated('Use connectionsDescriptor instead')
const Connections$json = {
  '1': 'Connections',
  '2': [
    {'1': 'internet_nodes_request', '3': 1, '4': 1, '5': 11, '6': '.qaul.rpc.connections.InternetNodesRequest', '9': 0, '10': 'internetNodesRequest'},
    {'1': 'internet_nodes_list', '3': 2, '4': 1, '5': 11, '6': '.qaul.rpc.connections.InternetNodesList', '9': 0, '10': 'internetNodesList'},
    {'1': 'internet_nodes_add', '3': 3, '4': 1, '5': 11, '6': '.qaul.rpc.connections.InternetNodesEntry', '9': 0, '10': 'internetNodesAdd'},
    {'1': 'internet_nodes_remove', '3': 4, '4': 1, '5': 11, '6': '.qaul.rpc.connections.InternetNodesEntry', '9': 0, '10': 'internetNodesRemove'},
    {'1': 'internet_nodes_state', '3': 5, '4': 1, '5': 11, '6': '.qaul.rpc.connections.InternetNodesEntry', '9': 0, '10': 'internetNodesState'},
    {'1': 'internet_nodes_rename', '3': 6, '4': 1, '5': 11, '6': '.qaul.rpc.connections.InternetNodesEntry', '9': 0, '10': 'internetNodesRename'},
  ],
  '8': [
    {'1': 'message'},
  ],
};

/// Descriptor for `Connections`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List connectionsDescriptor = $convert.base64Decode(
    'CgtDb25uZWN0aW9ucxJiChZpbnRlcm5ldF9ub2Rlc19yZXF1ZXN0GAEgASgLMioucWF1bC5ycG'
    'MuY29ubmVjdGlvbnMuSW50ZXJuZXROb2Rlc1JlcXVlc3RIAFIUaW50ZXJuZXROb2Rlc1JlcXVl'
    'c3QSWQoTaW50ZXJuZXRfbm9kZXNfbGlzdBgCIAEoCzInLnFhdWwucnBjLmNvbm5lY3Rpb25zLk'
    'ludGVybmV0Tm9kZXNMaXN0SABSEWludGVybmV0Tm9kZXNMaXN0ElgKEmludGVybmV0X25vZGVz'
    'X2FkZBgDIAEoCzIoLnFhdWwucnBjLmNvbm5lY3Rpb25zLkludGVybmV0Tm9kZXNFbnRyeUgAUh'
    'BpbnRlcm5ldE5vZGVzQWRkEl4KFWludGVybmV0X25vZGVzX3JlbW92ZRgEIAEoCzIoLnFhdWwu'
    'cnBjLmNvbm5lY3Rpb25zLkludGVybmV0Tm9kZXNFbnRyeUgAUhNpbnRlcm5ldE5vZGVzUmVtb3'
    'ZlElwKFGludGVybmV0X25vZGVzX3N0YXRlGAUgASgLMigucWF1bC5ycGMuY29ubmVjdGlvbnMu'
    'SW50ZXJuZXROb2Rlc0VudHJ5SABSEmludGVybmV0Tm9kZXNTdGF0ZRJeChVpbnRlcm5ldF9ub2'
    'Rlc19yZW5hbWUYBiABKAsyKC5xYXVsLnJwYy5jb25uZWN0aW9ucy5JbnRlcm5ldE5vZGVzRW50'
    'cnlIAFITaW50ZXJuZXROb2Rlc1JlbmFtZUIJCgdtZXNzYWdl');

@$core.Deprecated('Use internetNodesRequestDescriptor instead')
const InternetNodesRequest$json = {
  '1': 'InternetNodesRequest',
};

/// Descriptor for `InternetNodesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List internetNodesRequestDescriptor = $convert.base64Decode(
    'ChRJbnRlcm5ldE5vZGVzUmVxdWVzdA==');

@$core.Deprecated('Use internetNodesListDescriptor instead')
const InternetNodesList$json = {
  '1': 'InternetNodesList',
  '2': [
    {'1': 'info', '3': 1, '4': 1, '5': 14, '6': '.qaul.rpc.connections.Info', '10': 'info'},
    {'1': 'nodes', '3': 2, '4': 3, '5': 11, '6': '.qaul.rpc.connections.InternetNodesEntry', '10': 'nodes'},
  ],
};

/// Descriptor for `InternetNodesList`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List internetNodesListDescriptor = $convert.base64Decode(
    'ChFJbnRlcm5ldE5vZGVzTGlzdBIuCgRpbmZvGAEgASgOMhoucWF1bC5ycGMuY29ubmVjdGlvbn'
    'MuSW5mb1IEaW5mbxI+CgVub2RlcxgCIAMoCzIoLnFhdWwucnBjLmNvbm5lY3Rpb25zLkludGVy'
    'bmV0Tm9kZXNFbnRyeVIFbm9kZXM=');

@$core.Deprecated('Use internetNodesEntryDescriptor instead')
const InternetNodesEntry$json = {
  '1': 'InternetNodesEntry',
  '2': [
    {'1': 'address', '3': 1, '4': 1, '5': 9, '10': 'address'},
    {'1': 'enabled', '3': 2, '4': 1, '5': 8, '10': 'enabled'},
    {'1': 'name', '3': 3, '4': 1, '5': 9, '10': 'name'},
  ],
};

/// Descriptor for `InternetNodesEntry`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List internetNodesEntryDescriptor = $convert.base64Decode(
    'ChJJbnRlcm5ldE5vZGVzRW50cnkSGAoHYWRkcmVzcxgBIAEoCVIHYWRkcmVzcxIYCgdlbmFibG'
    'VkGAIgASgIUgdlbmFibGVkEhIKBG5hbWUYAyABKAlSBG5hbWU=');

