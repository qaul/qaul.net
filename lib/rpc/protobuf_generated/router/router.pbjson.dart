///
//  Generated code. Do not modify.
//  source: router/router.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields,deprecated_member_use_from_same_package

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
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
@$core.Deprecated('Use routerDescriptor instead')
const Router$json = const {
  '1': 'Router',
  '2': const [
    const {'1': 'user_request', '3': 1, '4': 1, '5': 11, '6': '.qaul.rpc.router.UserRequest', '9': 0, '10': 'userRequest'},
    const {'1': 'user_list', '3': 2, '4': 1, '5': 11, '6': '.qaul.rpc.router.UserList', '9': 0, '10': 'userList'},
  ],
  '8': const [
    const {'1': 'message'},
  ],
};

/// Descriptor for `Router`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List routerDescriptor = $convert.base64Decode('CgZSb3V0ZXISQQoMdXNlcl9yZXF1ZXN0GAEgASgLMhwucWF1bC5ycGMucm91dGVyLlVzZXJSZXF1ZXN0SABSC3VzZXJSZXF1ZXN0EjgKCXVzZXJfbGlzdBgCIAEoCzIZLnFhdWwucnBjLnJvdXRlci5Vc2VyTGlzdEgAUgh1c2VyTGlzdEIJCgdtZXNzYWdl');
@$core.Deprecated('Use userRequestDescriptor instead')
const UserRequest$json = const {
  '1': 'UserRequest',
};

/// Descriptor for `UserRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List userRequestDescriptor = $convert.base64Decode('CgtVc2VyUmVxdWVzdA==');
@$core.Deprecated('Use userListDescriptor instead')
const UserList$json = const {
  '1': 'UserList',
  '2': const [
    const {'1': 'user', '3': 1, '4': 3, '5': 11, '6': '.qaul.rpc.router.UserEntry', '10': 'user'},
  ],
};

/// Descriptor for `UserList`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List userListDescriptor = $convert.base64Decode('CghVc2VyTGlzdBIuCgR1c2VyGAEgAygLMhoucWF1bC5ycGMucm91dGVyLlVzZXJFbnRyeVIEdXNlcg==');
@$core.Deprecated('Use userEntryDescriptor instead')
const UserEntry$json = const {
  '1': 'UserEntry',
  '2': const [
    const {'1': 'name', '3': 1, '4': 1, '5': 9, '10': 'name'},
    const {'1': 'id', '3': 2, '4': 1, '5': 12, '10': 'id'},
    const {'1': 'id_base58', '3': 4, '4': 1, '5': 9, '10': 'idBase58'},
    const {'1': 'key', '3': 5, '4': 1, '5': 12, '10': 'key'},
    const {'1': 'key_type', '3': 6, '4': 1, '5': 9, '10': 'keyType'},
    const {'1': 'key_base58', '3': 7, '4': 1, '5': 9, '10': 'keyBase58'},
    const {'1': 'connectivity', '3': 8, '4': 1, '5': 14, '6': '.qaul.rpc.router.Connectivity', '10': 'connectivity'},
  ],
};

/// Descriptor for `UserEntry`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List userEntryDescriptor = $convert.base64Decode('CglVc2VyRW50cnkSEgoEbmFtZRgBIAEoCVIEbmFtZRIOCgJpZBgCIAEoDFICaWQSGwoJaWRfYmFzZTU4GAQgASgJUghpZEJhc2U1OBIQCgNrZXkYBSABKAxSA2tleRIZCghrZXlfdHlwZRgGIAEoCVIHa2V5VHlwZRIdCgprZXlfYmFzZTU4GAcgASgJUglrZXlCYXNlNTgSQQoMY29ubmVjdGl2aXR5GAggASgOMh0ucWF1bC5ycGMucm91dGVyLkNvbm5lY3Rpdml0eVIMY29ubmVjdGl2aXR5');
