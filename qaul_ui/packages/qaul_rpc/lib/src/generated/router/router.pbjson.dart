//
//  Generated code. Do not modify.
//  source: router/router.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use connectionModuleDescriptor instead')
const ConnectionModule$json = {
  '1': 'ConnectionModule',
  '2': [
    {'1': 'NONE', '2': 0},
    {'1': 'LAN', '2': 1},
    {'1': 'INTERNET', '2': 2},
    {'1': 'BLE', '2': 3},
    {'1': 'LOCAL', '2': 4},
  ],
};

/// Descriptor for `ConnectionModule`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List connectionModuleDescriptor = $convert.base64Decode(
    'ChBDb25uZWN0aW9uTW9kdWxlEggKBE5PTkUQABIHCgNMQU4QARIMCghJTlRFUk5FVBACEgcKA0'
    'JMRRADEgkKBUxPQ0FMEAQ=');

@$core.Deprecated('Use routerDescriptor instead')
const Router$json = {
  '1': 'Router',
  '2': [
    {'1': 'routing_table_request', '3': 1, '4': 1, '5': 11, '6': '.qaul.rpc.router.RoutingTableRequest', '9': 0, '10': 'routingTableRequest'},
    {'1': 'routing_table', '3': 2, '4': 1, '5': 11, '6': '.qaul.rpc.router.RoutingTableList', '9': 0, '10': 'routingTable'},
    {'1': 'connections_request', '3': 3, '4': 1, '5': 11, '6': '.qaul.rpc.router.ConnectionsRequest', '9': 0, '10': 'connectionsRequest'},
    {'1': 'connections_list', '3': 4, '4': 1, '5': 11, '6': '.qaul.rpc.router.ConnectionsList', '9': 0, '10': 'connectionsList'},
    {'1': 'neighbours_request', '3': 5, '4': 1, '5': 11, '6': '.qaul.rpc.router.NeighboursRequest', '9': 0, '10': 'neighboursRequest'},
    {'1': 'neighbours_list', '3': 6, '4': 1, '5': 11, '6': '.qaul.rpc.router.NeighboursList', '9': 0, '10': 'neighboursList'},
  ],
  '8': [
    {'1': 'message'},
  ],
};

/// Descriptor for `Router`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List routerDescriptor = $convert.base64Decode(
    'CgZSb3V0ZXISWgoVcm91dGluZ190YWJsZV9yZXF1ZXN0GAEgASgLMiQucWF1bC5ycGMucm91dG'
    'VyLlJvdXRpbmdUYWJsZVJlcXVlc3RIAFITcm91dGluZ1RhYmxlUmVxdWVzdBJICg1yb3V0aW5n'
    'X3RhYmxlGAIgASgLMiEucWF1bC5ycGMucm91dGVyLlJvdXRpbmdUYWJsZUxpc3RIAFIMcm91dG'
    'luZ1RhYmxlElYKE2Nvbm5lY3Rpb25zX3JlcXVlc3QYAyABKAsyIy5xYXVsLnJwYy5yb3V0ZXIu'
    'Q29ubmVjdGlvbnNSZXF1ZXN0SABSEmNvbm5lY3Rpb25zUmVxdWVzdBJNChBjb25uZWN0aW9uc1'
    '9saXN0GAQgASgLMiAucWF1bC5ycGMucm91dGVyLkNvbm5lY3Rpb25zTGlzdEgAUg9jb25uZWN0'
    'aW9uc0xpc3QSUwoSbmVpZ2hib3Vyc19yZXF1ZXN0GAUgASgLMiIucWF1bC5ycGMucm91dGVyLk'
    '5laWdoYm91cnNSZXF1ZXN0SABSEW5laWdoYm91cnNSZXF1ZXN0EkoKD25laWdoYm91cnNfbGlz'
    'dBgGIAEoCzIfLnFhdWwucnBjLnJvdXRlci5OZWlnaGJvdXJzTGlzdEgAUg5uZWlnaGJvdXJzTG'
    'lzdEIJCgdtZXNzYWdl');

@$core.Deprecated('Use routingTableRequestDescriptor instead')
const RoutingTableRequest$json = {
  '1': 'RoutingTableRequest',
};

/// Descriptor for `RoutingTableRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List routingTableRequestDescriptor = $convert.base64Decode(
    'ChNSb3V0aW5nVGFibGVSZXF1ZXN0');

@$core.Deprecated('Use routingTableListDescriptor instead')
const RoutingTableList$json = {
  '1': 'RoutingTableList',
  '2': [
    {'1': 'routing_table', '3': 1, '4': 3, '5': 11, '6': '.qaul.rpc.router.RoutingTableEntry', '10': 'routingTable'},
  ],
};

/// Descriptor for `RoutingTableList`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List routingTableListDescriptor = $convert.base64Decode(
    'ChBSb3V0aW5nVGFibGVMaXN0EkcKDXJvdXRpbmdfdGFibGUYASADKAsyIi5xYXVsLnJwYy5yb3'
    'V0ZXIuUm91dGluZ1RhYmxlRW50cnlSDHJvdXRpbmdUYWJsZQ==');

@$core.Deprecated('Use routingTableEntryDescriptor instead')
const RoutingTableEntry$json = {
  '1': 'RoutingTableEntry',
  '2': [
    {'1': 'user_id', '3': 1, '4': 1, '5': 12, '10': 'userId'},
    {'1': 'connections', '3': 2, '4': 3, '5': 11, '6': '.qaul.rpc.router.RoutingTableConnection', '10': 'connections'},
  ],
};

/// Descriptor for `RoutingTableEntry`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List routingTableEntryDescriptor = $convert.base64Decode(
    'ChFSb3V0aW5nVGFibGVFbnRyeRIXCgd1c2VyX2lkGAEgASgMUgZ1c2VySWQSSQoLY29ubmVjdG'
    'lvbnMYAiADKAsyJy5xYXVsLnJwYy5yb3V0ZXIuUm91dGluZ1RhYmxlQ29ubmVjdGlvblILY29u'
    'bmVjdGlvbnM=');

@$core.Deprecated('Use routingTableConnectionDescriptor instead')
const RoutingTableConnection$json = {
  '1': 'RoutingTableConnection',
  '2': [
    {'1': 'module', '3': 2, '4': 1, '5': 14, '6': '.qaul.rpc.router.ConnectionModule', '10': 'module'},
    {'1': 'rtt', '3': 3, '4': 1, '5': 13, '10': 'rtt'},
    {'1': 'hop_count', '3': 5, '4': 1, '5': 13, '10': 'hopCount'},
    {'1': 'via', '3': 4, '4': 1, '5': 12, '10': 'via'},
  ],
};

/// Descriptor for `RoutingTableConnection`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List routingTableConnectionDescriptor = $convert.base64Decode(
    'ChZSb3V0aW5nVGFibGVDb25uZWN0aW9uEjkKBm1vZHVsZRgCIAEoDjIhLnFhdWwucnBjLnJvdX'
    'Rlci5Db25uZWN0aW9uTW9kdWxlUgZtb2R1bGUSEAoDcnR0GAMgASgNUgNydHQSGwoJaG9wX2Nv'
    'dW50GAUgASgNUghob3BDb3VudBIQCgN2aWEYBCABKAxSA3ZpYQ==');

@$core.Deprecated('Use connectionsRequestDescriptor instead')
const ConnectionsRequest$json = {
  '1': 'ConnectionsRequest',
};

/// Descriptor for `ConnectionsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List connectionsRequestDescriptor = $convert.base64Decode(
    'ChJDb25uZWN0aW9uc1JlcXVlc3Q=');

@$core.Deprecated('Use connectionsListDescriptor instead')
const ConnectionsList$json = {
  '1': 'ConnectionsList',
  '2': [
    {'1': 'lan', '3': 1, '4': 3, '5': 11, '6': '.qaul.rpc.router.ConnectionsUserEntry', '10': 'lan'},
    {'1': 'internet', '3': 2, '4': 3, '5': 11, '6': '.qaul.rpc.router.ConnectionsUserEntry', '10': 'internet'},
    {'1': 'ble', '3': 3, '4': 3, '5': 11, '6': '.qaul.rpc.router.ConnectionsUserEntry', '10': 'ble'},
    {'1': 'local', '3': 4, '4': 3, '5': 11, '6': '.qaul.rpc.router.ConnectionsUserEntry', '10': 'local'},
  ],
};

/// Descriptor for `ConnectionsList`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List connectionsListDescriptor = $convert.base64Decode(
    'Cg9Db25uZWN0aW9uc0xpc3QSNwoDbGFuGAEgAygLMiUucWF1bC5ycGMucm91dGVyLkNvbm5lY3'
    'Rpb25zVXNlckVudHJ5UgNsYW4SQQoIaW50ZXJuZXQYAiADKAsyJS5xYXVsLnJwYy5yb3V0ZXIu'
    'Q29ubmVjdGlvbnNVc2VyRW50cnlSCGludGVybmV0EjcKA2JsZRgDIAMoCzIlLnFhdWwucnBjLn'
    'JvdXRlci5Db25uZWN0aW9uc1VzZXJFbnRyeVIDYmxlEjsKBWxvY2FsGAQgAygLMiUucWF1bC5y'
    'cGMucm91dGVyLkNvbm5lY3Rpb25zVXNlckVudHJ5UgVsb2NhbA==');

@$core.Deprecated('Use connectionsUserEntryDescriptor instead')
const ConnectionsUserEntry$json = {
  '1': 'ConnectionsUserEntry',
  '2': [
    {'1': 'user_id', '3': 1, '4': 1, '5': 12, '10': 'userId'},
    {'1': 'connections', '3': 2, '4': 3, '5': 11, '6': '.qaul.rpc.router.ConnectionEntry', '10': 'connections'},
  ],
};

/// Descriptor for `ConnectionsUserEntry`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List connectionsUserEntryDescriptor = $convert.base64Decode(
    'ChRDb25uZWN0aW9uc1VzZXJFbnRyeRIXCgd1c2VyX2lkGAEgASgMUgZ1c2VySWQSQgoLY29ubm'
    'VjdGlvbnMYAiADKAsyIC5xYXVsLnJwYy5yb3V0ZXIuQ29ubmVjdGlvbkVudHJ5Ugtjb25uZWN0'
    'aW9ucw==');

@$core.Deprecated('Use connectionEntryDescriptor instead')
const ConnectionEntry$json = {
  '1': 'ConnectionEntry',
  '2': [
    {'1': 'rtt', '3': 1, '4': 1, '5': 13, '10': 'rtt'},
    {'1': 'hop_count', '3': 2, '4': 1, '5': 13, '10': 'hopCount'},
    {'1': 'via', '3': 3, '4': 1, '5': 12, '10': 'via'},
  ],
};

/// Descriptor for `ConnectionEntry`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List connectionEntryDescriptor = $convert.base64Decode(
    'Cg9Db25uZWN0aW9uRW50cnkSEAoDcnR0GAEgASgNUgNydHQSGwoJaG9wX2NvdW50GAIgASgNUg'
    'hob3BDb3VudBIQCgN2aWEYAyABKAxSA3ZpYQ==');

@$core.Deprecated('Use neighboursRequestDescriptor instead')
const NeighboursRequest$json = {
  '1': 'NeighboursRequest',
};

/// Descriptor for `NeighboursRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List neighboursRequestDescriptor = $convert.base64Decode(
    'ChFOZWlnaGJvdXJzUmVxdWVzdA==');

@$core.Deprecated('Use neighboursListDescriptor instead')
const NeighboursList$json = {
  '1': 'NeighboursList',
  '2': [
    {'1': 'lan', '3': 1, '4': 3, '5': 11, '6': '.qaul.rpc.router.NeighboursEntry', '10': 'lan'},
    {'1': 'internet', '3': 2, '4': 3, '5': 11, '6': '.qaul.rpc.router.NeighboursEntry', '10': 'internet'},
    {'1': 'ble', '3': 3, '4': 3, '5': 11, '6': '.qaul.rpc.router.NeighboursEntry', '10': 'ble'},
  ],
};

/// Descriptor for `NeighboursList`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List neighboursListDescriptor = $convert.base64Decode(
    'Cg5OZWlnaGJvdXJzTGlzdBIyCgNsYW4YASADKAsyIC5xYXVsLnJwYy5yb3V0ZXIuTmVpZ2hib3'
    'Vyc0VudHJ5UgNsYW4SPAoIaW50ZXJuZXQYAiADKAsyIC5xYXVsLnJwYy5yb3V0ZXIuTmVpZ2hi'
    'b3Vyc0VudHJ5UghpbnRlcm5ldBIyCgNibGUYAyADKAsyIC5xYXVsLnJwYy5yb3V0ZXIuTmVpZ2'
    'hib3Vyc0VudHJ5UgNibGU=');

@$core.Deprecated('Use neighboursEntryDescriptor instead')
const NeighboursEntry$json = {
  '1': 'NeighboursEntry',
  '2': [
    {'1': 'node_id', '3': 1, '4': 1, '5': 12, '10': 'nodeId'},
    {'1': 'rtt', '3': 2, '4': 1, '5': 13, '10': 'rtt'},
  ],
};

/// Descriptor for `NeighboursEntry`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List neighboursEntryDescriptor = $convert.base64Decode(
    'Cg9OZWlnaGJvdXJzRW50cnkSFwoHbm9kZV9pZBgBIAEoDFIGbm9kZUlkEhAKA3J0dBgCIAEoDV'
    'IDcnR0');

