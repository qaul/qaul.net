///
//  Generated code. Do not modify.
//  source: router/users.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,deprecated_member_use_from_same_package,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use connectionModuleDescriptor instead')
const ConnectionModule$json = const {
  '1': 'ConnectionModule',
  '2': const [
    const {'1': 'NONE', '2': 0},
    const {'1': 'LAN', '2': 1},
    const {'1': 'INTERNET', '2': 2},
    const {'1': 'BLE', '2': 3},
    const {'1': 'LOCAL', '2': 4},
  ],
};

/// Descriptor for `ConnectionModule`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List connectionModuleDescriptor = $convert.base64Decode('ChBDb25uZWN0aW9uTW9kdWxlEggKBE5PTkUQABIHCgNMQU4QARIMCghJTlRFUk5FVBACEgcKA0JMRRADEgkKBUxPQ0FMEAQ=');
@$core.Deprecated('Use connectivityDescriptor instead')
const Connectivity$json = const {
  '1': 'Connectivity',
  '2': const [
    const {'1': 'Online', '2': 0},
    const {'1': 'Reachable', '2': 1},
    const {'1': 'Offline', '2': 2},
  ],
};

/// Descriptor for `Connectivity`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List connectivityDescriptor = $convert.base64Decode('CgxDb25uZWN0aXZpdHkSCgoGT25saW5lEAASDQoJUmVhY2hhYmxlEAESCwoHT2ZmbGluZRAC');
@$core.Deprecated('Use usersDescriptor instead')
const Users$json = const {
  '1': 'Users',
  '2': const [
    const {'1': 'user_request', '3': 1, '4': 1, '5': 11, '6': '.qaul.rpc.users.UserRequest', '9': 0, '10': 'userRequest'},
    const {'1': 'user_online_request', '3': 2, '4': 1, '5': 11, '6': '.qaul.rpc.users.UserOnlineRequest', '9': 0, '10': 'userOnlineRequest'},
    const {'1': 'user_list', '3': 3, '4': 1, '5': 11, '6': '.qaul.rpc.users.UserList', '9': 0, '10': 'userList'},
    const {'1': 'user_update', '3': 4, '4': 1, '5': 11, '6': '.qaul.rpc.users.UserEntry', '9': 0, '10': 'userUpdate'},
    const {'1': 'security_number_request', '3': 5, '4': 1, '5': 11, '6': '.qaul.rpc.users.SecurityNumberRequest', '9': 0, '10': 'securityNumberRequest'},
    const {'1': 'security_number_response', '3': 6, '4': 1, '5': 11, '6': '.qaul.rpc.users.SecurityNumberResponse', '9': 0, '10': 'securityNumberResponse'},
  ],
  '8': const [
    const {'1': 'message'},
  ],
};

/// Descriptor for `Users`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List usersDescriptor = $convert.base64Decode('CgVVc2VycxJACgx1c2VyX3JlcXVlc3QYASABKAsyGy5xYXVsLnJwYy51c2Vycy5Vc2VyUmVxdWVzdEgAUgt1c2VyUmVxdWVzdBJTChN1c2VyX29ubGluZV9yZXF1ZXN0GAIgASgLMiEucWF1bC5ycGMudXNlcnMuVXNlck9ubGluZVJlcXVlc3RIAFIRdXNlck9ubGluZVJlcXVlc3QSNwoJdXNlcl9saXN0GAMgASgLMhgucWF1bC5ycGMudXNlcnMuVXNlckxpc3RIAFIIdXNlckxpc3QSPAoLdXNlcl91cGRhdGUYBCABKAsyGS5xYXVsLnJwYy51c2Vycy5Vc2VyRW50cnlIAFIKdXNlclVwZGF0ZRJfChdzZWN1cml0eV9udW1iZXJfcmVxdWVzdBgFIAEoCzIlLnFhdWwucnBjLnVzZXJzLlNlY3VyaXR5TnVtYmVyUmVxdWVzdEgAUhVzZWN1cml0eU51bWJlclJlcXVlc3QSYgoYc2VjdXJpdHlfbnVtYmVyX3Jlc3BvbnNlGAYgASgLMiYucWF1bC5ycGMudXNlcnMuU2VjdXJpdHlOdW1iZXJSZXNwb25zZUgAUhZzZWN1cml0eU51bWJlclJlc3BvbnNlQgkKB21lc3NhZ2U=');
@$core.Deprecated('Use userRequestDescriptor instead')
const UserRequest$json = const {
  '1': 'UserRequest',
};

/// Descriptor for `UserRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List userRequestDescriptor = $convert.base64Decode('CgtVc2VyUmVxdWVzdA==');
@$core.Deprecated('Use userOnlineRequestDescriptor instead')
const UserOnlineRequest$json = const {
  '1': 'UserOnlineRequest',
};

/// Descriptor for `UserOnlineRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List userOnlineRequestDescriptor = $convert.base64Decode('ChFVc2VyT25saW5lUmVxdWVzdA==');
@$core.Deprecated('Use userListDescriptor instead')
const UserList$json = const {
  '1': 'UserList',
  '2': const [
    const {'1': 'user', '3': 1, '4': 3, '5': 11, '6': '.qaul.rpc.users.UserEntry', '10': 'user'},
  ],
};

/// Descriptor for `UserList`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List userListDescriptor = $convert.base64Decode('CghVc2VyTGlzdBItCgR1c2VyGAEgAygLMhkucWF1bC5ycGMudXNlcnMuVXNlckVudHJ5UgR1c2Vy');
@$core.Deprecated('Use userEntryDescriptor instead')
const UserEntry$json = const {
  '1': 'UserEntry',
  '2': const [
    const {'1': 'name', '3': 1, '4': 1, '5': 9, '10': 'name'},
    const {'1': 'id', '3': 2, '4': 1, '5': 12, '10': 'id'},
    const {'1': 'group_id', '3': 3, '4': 1, '5': 12, '10': 'groupId'},
    const {'1': 'key_base58', '3': 7, '4': 1, '5': 9, '10': 'keyBase58'},
    const {'1': 'connectivity', '3': 8, '4': 1, '5': 14, '6': '.qaul.rpc.users.Connectivity', '10': 'connectivity'},
    const {'1': 'verified', '3': 9, '4': 1, '5': 8, '10': 'verified'},
    const {'1': 'blocked', '3': 10, '4': 1, '5': 8, '10': 'blocked'},
    const {'1': 'connections', '3': 11, '4': 3, '5': 11, '6': '.qaul.rpc.users.RoutingTableConnection', '10': 'connections'},
  ],
};

/// Descriptor for `UserEntry`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List userEntryDescriptor = $convert.base64Decode('CglVc2VyRW50cnkSEgoEbmFtZRgBIAEoCVIEbmFtZRIOCgJpZBgCIAEoDFICaWQSGQoIZ3JvdXBfaWQYAyABKAxSB2dyb3VwSWQSHQoKa2V5X2Jhc2U1OBgHIAEoCVIJa2V5QmFzZTU4EkAKDGNvbm5lY3Rpdml0eRgIIAEoDjIcLnFhdWwucnBjLnVzZXJzLkNvbm5lY3Rpdml0eVIMY29ubmVjdGl2aXR5EhoKCHZlcmlmaWVkGAkgASgIUgh2ZXJpZmllZBIYCgdibG9ja2VkGAogASgIUgdibG9ja2VkEkgKC2Nvbm5lY3Rpb25zGAsgAygLMiYucWF1bC5ycGMudXNlcnMuUm91dGluZ1RhYmxlQ29ubmVjdGlvblILY29ubmVjdGlvbnM=');
@$core.Deprecated('Use routingTableConnectionDescriptor instead')
const RoutingTableConnection$json = const {
  '1': 'RoutingTableConnection',
  '2': const [
    const {'1': 'module', '3': 2, '4': 1, '5': 14, '6': '.qaul.rpc.users.ConnectionModule', '10': 'module'},
    const {'1': 'rtt', '3': 3, '4': 1, '5': 13, '10': 'rtt'},
    const {'1': 'hop_count', '3': 5, '4': 1, '5': 13, '10': 'hopCount'},
    const {'1': 'via', '3': 4, '4': 1, '5': 12, '10': 'via'},
  ],
};

/// Descriptor for `RoutingTableConnection`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List routingTableConnectionDescriptor = $convert.base64Decode('ChZSb3V0aW5nVGFibGVDb25uZWN0aW9uEjgKBm1vZHVsZRgCIAEoDjIgLnFhdWwucnBjLnVzZXJzLkNvbm5lY3Rpb25Nb2R1bGVSBm1vZHVsZRIQCgNydHQYAyABKA1SA3J0dBIbCglob3BfY291bnQYBSABKA1SCGhvcENvdW50EhAKA3ZpYRgEIAEoDFIDdmlh');
@$core.Deprecated('Use securityNumberRequestDescriptor instead')
const SecurityNumberRequest$json = const {
  '1': 'SecurityNumberRequest',
  '2': const [
    const {'1': 'user_id', '3': 1, '4': 1, '5': 12, '10': 'userId'},
  ],
};

/// Descriptor for `SecurityNumberRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List securityNumberRequestDescriptor = $convert.base64Decode('ChVTZWN1cml0eU51bWJlclJlcXVlc3QSFwoHdXNlcl9pZBgBIAEoDFIGdXNlcklk');
@$core.Deprecated('Use securityNumberResponseDescriptor instead')
const SecurityNumberResponse$json = const {
  '1': 'SecurityNumberResponse',
  '2': const [
    const {'1': 'user_id', '3': 1, '4': 1, '5': 12, '10': 'userId'},
    const {'1': 'security_hash', '3': 2, '4': 1, '5': 12, '10': 'securityHash'},
    const {'1': 'security_number_blocks', '3': 3, '4': 3, '5': 13, '10': 'securityNumberBlocks'},
  ],
};

/// Descriptor for `SecurityNumberResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List securityNumberResponseDescriptor = $convert.base64Decode('ChZTZWN1cml0eU51bWJlclJlc3BvbnNlEhcKB3VzZXJfaWQYASABKAxSBnVzZXJJZBIjCg1zZWN1cml0eV9oYXNoGAIgASgMUgxzZWN1cml0eUhhc2gSNAoWc2VjdXJpdHlfbnVtYmVyX2Jsb2NrcxgDIAMoDVIUc2VjdXJpdHlOdW1iZXJCbG9ja3M=');
