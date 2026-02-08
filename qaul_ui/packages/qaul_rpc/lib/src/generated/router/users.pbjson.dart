// This is a generated file - do not edit.
//
// Generated from router/users.proto.

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

@$core.Deprecated('Use connectivityDescriptor instead')
const Connectivity$json = {
  '1': 'Connectivity',
  '2': [
    {'1': 'Online', '2': 0},
    {'1': 'Reachable', '2': 1},
    {'1': 'Offline', '2': 2},
  ],
};

/// Descriptor for `Connectivity`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List connectivityDescriptor = $convert.base64Decode(
    'CgxDb25uZWN0aXZpdHkSCgoGT25saW5lEAASDQoJUmVhY2hhYmxlEAESCwoHT2ZmbGluZRAC');

@$core.Deprecated('Use usersDescriptor instead')
const Users$json = {
  '1': 'Users',
  '2': [
    {
      '1': 'user_request',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.qaul.rpc.users.UserRequest',
      '9': 0,
      '10': 'userRequest'
    },
    {
      '1': 'user_online_request',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.qaul.rpc.users.UserOnlineRequest',
      '9': 0,
      '10': 'userOnlineRequest'
    },
    {
      '1': 'user_list',
      '3': 3,
      '4': 1,
      '5': 11,
      '6': '.qaul.rpc.users.UserList',
      '9': 0,
      '10': 'userList'
    },
    {
      '1': 'user_update',
      '3': 4,
      '4': 1,
      '5': 11,
      '6': '.qaul.rpc.users.UserEntry',
      '9': 0,
      '10': 'userUpdate'
    },
    {
      '1': 'security_number_request',
      '3': 5,
      '4': 1,
      '5': 11,
      '6': '.qaul.rpc.users.SecurityNumberRequest',
      '9': 0,
      '10': 'securityNumberRequest'
    },
    {
      '1': 'security_number_response',
      '3': 6,
      '4': 1,
      '5': 11,
      '6': '.qaul.rpc.users.SecurityNumberResponse',
      '9': 0,
      '10': 'securityNumberResponse'
    },
  ],
  '8': [
    {'1': 'message'},
  ],
};

/// Descriptor for `Users`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List usersDescriptor = $convert.base64Decode(
    'CgVVc2VycxJACgx1c2VyX3JlcXVlc3QYASABKAsyGy5xYXVsLnJwYy51c2Vycy5Vc2VyUmVxdW'
    'VzdEgAUgt1c2VyUmVxdWVzdBJTChN1c2VyX29ubGluZV9yZXF1ZXN0GAIgASgLMiEucWF1bC5y'
    'cGMudXNlcnMuVXNlck9ubGluZVJlcXVlc3RIAFIRdXNlck9ubGluZVJlcXVlc3QSNwoJdXNlcl'
    '9saXN0GAMgASgLMhgucWF1bC5ycGMudXNlcnMuVXNlckxpc3RIAFIIdXNlckxpc3QSPAoLdXNl'
    'cl91cGRhdGUYBCABKAsyGS5xYXVsLnJwYy51c2Vycy5Vc2VyRW50cnlIAFIKdXNlclVwZGF0ZR'
    'JfChdzZWN1cml0eV9udW1iZXJfcmVxdWVzdBgFIAEoCzIlLnFhdWwucnBjLnVzZXJzLlNlY3Vy'
    'aXR5TnVtYmVyUmVxdWVzdEgAUhVzZWN1cml0eU51bWJlclJlcXVlc3QSYgoYc2VjdXJpdHlfbn'
    'VtYmVyX3Jlc3BvbnNlGAYgASgLMiYucWF1bC5ycGMudXNlcnMuU2VjdXJpdHlOdW1iZXJSZXNw'
    'b25zZUgAUhZzZWN1cml0eU51bWJlclJlc3BvbnNlQgkKB21lc3NhZ2U=');

@$core.Deprecated('Use userRequestDescriptor instead')
const UserRequest$json = {
  '1': 'UserRequest',
  '2': [
    {'1': 'offset', '3': 10, '4': 1, '5': 13, '10': 'offset'},
    {'1': 'limit', '3': 20, '4': 1, '5': 13, '10': 'limit'},
  ],
};

/// Descriptor for `UserRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List userRequestDescriptor = $convert.base64Decode(
    'CgtVc2VyUmVxdWVzdBIWCgZvZmZzZXQYCiABKA1SBm9mZnNldBIUCgVsaW1pdBgUIAEoDVIFbG'
    'ltaXQ=');

@$core.Deprecated('Use userOnlineRequestDescriptor instead')
const UserOnlineRequest$json = {
  '1': 'UserOnlineRequest',
  '2': [
    {'1': 'offset', '3': 10, '4': 1, '5': 13, '10': 'offset'},
    {'1': 'limit', '3': 20, '4': 1, '5': 13, '10': 'limit'},
  ],
};

/// Descriptor for `UserOnlineRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List userOnlineRequestDescriptor = $convert.base64Decode(
    'ChFVc2VyT25saW5lUmVxdWVzdBIWCgZvZmZzZXQYCiABKA1SBm9mZnNldBIUCgVsaW1pdBgUIA'
    'EoDVIFbGltaXQ=');

@$core.Deprecated('Use paginationMetadataDescriptor instead')
const PaginationMetadata$json = {
  '1': 'PaginationMetadata',
  '2': [
    {'1': 'has_more', '3': 10, '4': 1, '5': 8, '10': 'hasMore'},
    {'1': 'total', '3': 20, '4': 1, '5': 13, '10': 'total'},
    {'1': 'offset', '3': 30, '4': 1, '5': 13, '10': 'offset'},
    {'1': 'limit', '3': 40, '4': 1, '5': 13, '10': 'limit'},
  ],
};

/// Descriptor for `PaginationMetadata`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List paginationMetadataDescriptor = $convert.base64Decode(
    'ChJQYWdpbmF0aW9uTWV0YWRhdGESGQoIaGFzX21vcmUYCiABKAhSB2hhc01vcmUSFAoFdG90YW'
    'wYFCABKA1SBXRvdGFsEhYKBm9mZnNldBgeIAEoDVIGb2Zmc2V0EhQKBWxpbWl0GCggASgNUgVs'
    'aW1pdA==');

@$core.Deprecated('Use userListDescriptor instead')
const UserList$json = {
  '1': 'UserList',
  '2': [
    {
      '1': 'user',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.qaul.rpc.users.UserEntry',
      '10': 'user'
    },
    {
      '1': 'pagination',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.qaul.rpc.users.PaginationMetadata',
      '10': 'pagination'
    },
  ],
};

/// Descriptor for `UserList`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List userListDescriptor = $convert.base64Decode(
    'CghVc2VyTGlzdBItCgR1c2VyGAEgAygLMhkucWF1bC5ycGMudXNlcnMuVXNlckVudHJ5UgR1c2'
    'VyEkIKCnBhZ2luYXRpb24YAiABKAsyIi5xYXVsLnJwYy51c2Vycy5QYWdpbmF0aW9uTWV0YWRh'
    'dGFSCnBhZ2luYXRpb24=');

@$core.Deprecated('Use userEntryDescriptor instead')
const UserEntry$json = {
  '1': 'UserEntry',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 9, '10': 'name'},
    {'1': 'id', '3': 2, '4': 1, '5': 12, '10': 'id'},
    {'1': 'group_id', '3': 3, '4': 1, '5': 12, '10': 'groupId'},
    {'1': 'key_base58', '3': 7, '4': 1, '5': 9, '10': 'keyBase58'},
    {
      '1': 'connectivity',
      '3': 8,
      '4': 1,
      '5': 14,
      '6': '.qaul.rpc.users.Connectivity',
      '10': 'connectivity'
    },
    {'1': 'verified', '3': 9, '4': 1, '5': 8, '10': 'verified'},
    {'1': 'blocked', '3': 10, '4': 1, '5': 8, '10': 'blocked'},
    {
      '1': 'connections',
      '3': 11,
      '4': 3,
      '5': 11,
      '6': '.qaul.rpc.users.RoutingTableConnection',
      '10': 'connections'
    },
  ],
};

/// Descriptor for `UserEntry`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List userEntryDescriptor = $convert.base64Decode(
    'CglVc2VyRW50cnkSEgoEbmFtZRgBIAEoCVIEbmFtZRIOCgJpZBgCIAEoDFICaWQSGQoIZ3JvdX'
    'BfaWQYAyABKAxSB2dyb3VwSWQSHQoKa2V5X2Jhc2U1OBgHIAEoCVIJa2V5QmFzZTU4EkAKDGNv'
    'bm5lY3Rpdml0eRgIIAEoDjIcLnFhdWwucnBjLnVzZXJzLkNvbm5lY3Rpdml0eVIMY29ubmVjdG'
    'l2aXR5EhoKCHZlcmlmaWVkGAkgASgIUgh2ZXJpZmllZBIYCgdibG9ja2VkGAogASgIUgdibG9j'
    'a2VkEkgKC2Nvbm5lY3Rpb25zGAsgAygLMiYucWF1bC5ycGMudXNlcnMuUm91dGluZ1RhYmxlQ2'
    '9ubmVjdGlvblILY29ubmVjdGlvbnM=');

@$core.Deprecated('Use routingTableConnectionDescriptor instead')
const RoutingTableConnection$json = {
  '1': 'RoutingTableConnection',
  '2': [
    {
      '1': 'module',
      '3': 2,
      '4': 1,
      '5': 14,
      '6': '.qaul.rpc.users.ConnectionModule',
      '10': 'module'
    },
    {'1': 'rtt', '3': 3, '4': 1, '5': 13, '10': 'rtt'},
    {'1': 'hop_count', '3': 5, '4': 1, '5': 13, '10': 'hopCount'},
    {'1': 'via', '3': 4, '4': 1, '5': 12, '10': 'via'},
  ],
};

/// Descriptor for `RoutingTableConnection`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List routingTableConnectionDescriptor = $convert.base64Decode(
    'ChZSb3V0aW5nVGFibGVDb25uZWN0aW9uEjgKBm1vZHVsZRgCIAEoDjIgLnFhdWwucnBjLnVzZX'
    'JzLkNvbm5lY3Rpb25Nb2R1bGVSBm1vZHVsZRIQCgNydHQYAyABKA1SA3J0dBIbCglob3BfY291'
    'bnQYBSABKA1SCGhvcENvdW50EhAKA3ZpYRgEIAEoDFIDdmlh');

@$core.Deprecated('Use securityNumberRequestDescriptor instead')
const SecurityNumberRequest$json = {
  '1': 'SecurityNumberRequest',
  '2': [
    {'1': 'user_id', '3': 1, '4': 1, '5': 12, '10': 'userId'},
  ],
};

/// Descriptor for `SecurityNumberRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List securityNumberRequestDescriptor =
    $convert.base64Decode(
        'ChVTZWN1cml0eU51bWJlclJlcXVlc3QSFwoHdXNlcl9pZBgBIAEoDFIGdXNlcklk');

@$core.Deprecated('Use securityNumberResponseDescriptor instead')
const SecurityNumberResponse$json = {
  '1': 'SecurityNumberResponse',
  '2': [
    {'1': 'user_id', '3': 1, '4': 1, '5': 12, '10': 'userId'},
    {'1': 'security_hash', '3': 2, '4': 1, '5': 12, '10': 'securityHash'},
    {
      '1': 'security_number_blocks',
      '3': 3,
      '4': 3,
      '5': 13,
      '10': 'securityNumberBlocks'
    },
  ],
};

/// Descriptor for `SecurityNumberResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List securityNumberResponseDescriptor = $convert.base64Decode(
    'ChZTZWN1cml0eU51bWJlclJlc3BvbnNlEhcKB3VzZXJfaWQYASABKAxSBnVzZXJJZBIjCg1zZW'
    'N1cml0eV9oYXNoGAIgASgMUgxzZWN1cml0eUhhc2gSNAoWc2VjdXJpdHlfbnVtYmVyX2Jsb2Nr'
    'cxgDIAMoDVIUc2VjdXJpdHlOdW1iZXJCbG9ja3M=');
