//
//  Generated code. Do not modify.
//  source: services/dtn/dtn_rpc.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use dTNDescriptor instead')
const DTN$json = {
  '1': 'DTN',
  '2': [
    {'1': 'dtn_state_request', '3': 1, '4': 1, '5': 11, '6': '.qaul.rpc.dtn.DtnStateRequest', '9': 0, '10': 'dtnStateRequest'},
    {'1': 'dtn_state_response', '3': 2, '4': 1, '5': 11, '6': '.qaul.rpc.dtn.DtnStateResponse', '9': 0, '10': 'dtnStateResponse'},
    {'1': 'dtn_config_request', '3': 3, '4': 1, '5': 11, '6': '.qaul.rpc.dtn.DtnConfigRequest', '9': 0, '10': 'dtnConfigRequest'},
    {'1': 'dtn_config_response', '3': 4, '4': 1, '5': 11, '6': '.qaul.rpc.dtn.DtnConfigResponse', '9': 0, '10': 'dtnConfigResponse'},
    {'1': 'dtn_add_user_request', '3': 5, '4': 1, '5': 11, '6': '.qaul.rpc.dtn.DtnAddUserRequest', '9': 0, '10': 'dtnAddUserRequest'},
    {'1': 'dtn_add_user_response', '3': 6, '4': 1, '5': 11, '6': '.qaul.rpc.dtn.DtnAddUserResponse', '9': 0, '10': 'dtnAddUserResponse'},
    {'1': 'dtn_remove_user_request', '3': 7, '4': 1, '5': 11, '6': '.qaul.rpc.dtn.DtnRemoveUserRequest', '9': 0, '10': 'dtnRemoveUserRequest'},
    {'1': 'dtn_remove_user_response', '3': 8, '4': 1, '5': 11, '6': '.qaul.rpc.dtn.DtnRemoveUserResponse', '9': 0, '10': 'dtnRemoveUserResponse'},
    {'1': 'dtn_set_total_size_request', '3': 9, '4': 1, '5': 11, '6': '.qaul.rpc.dtn.DtnSetTotalSizeRequest', '9': 0, '10': 'dtnSetTotalSizeRequest'},
    {'1': 'dtn_set_total_size_response', '3': 10, '4': 1, '5': 11, '6': '.qaul.rpc.dtn.DtnSetTotalSizeResponse', '9': 0, '10': 'dtnSetTotalSizeResponse'},
  ],
  '8': [
    {'1': 'message'},
  ],
};

/// Descriptor for `DTN`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dTNDescriptor = $convert.base64Decode(
    'CgNEVE4SSwoRZHRuX3N0YXRlX3JlcXVlc3QYASABKAsyHS5xYXVsLnJwYy5kdG4uRHRuU3RhdG'
    'VSZXF1ZXN0SABSD2R0blN0YXRlUmVxdWVzdBJOChJkdG5fc3RhdGVfcmVzcG9uc2UYAiABKAsy'
    'Hi5xYXVsLnJwYy5kdG4uRHRuU3RhdGVSZXNwb25zZUgAUhBkdG5TdGF0ZVJlc3BvbnNlEk4KEm'
    'R0bl9jb25maWdfcmVxdWVzdBgDIAEoCzIeLnFhdWwucnBjLmR0bi5EdG5Db25maWdSZXF1ZXN0'
    'SABSEGR0bkNvbmZpZ1JlcXVlc3QSUQoTZHRuX2NvbmZpZ19yZXNwb25zZRgEIAEoCzIfLnFhdW'
    'wucnBjLmR0bi5EdG5Db25maWdSZXNwb25zZUgAUhFkdG5Db25maWdSZXNwb25zZRJSChRkdG5f'
    'YWRkX3VzZXJfcmVxdWVzdBgFIAEoCzIfLnFhdWwucnBjLmR0bi5EdG5BZGRVc2VyUmVxdWVzdE'
    'gAUhFkdG5BZGRVc2VyUmVxdWVzdBJVChVkdG5fYWRkX3VzZXJfcmVzcG9uc2UYBiABKAsyIC5x'
    'YXVsLnJwYy5kdG4uRHRuQWRkVXNlclJlc3BvbnNlSABSEmR0bkFkZFVzZXJSZXNwb25zZRJbCh'
    'dkdG5fcmVtb3ZlX3VzZXJfcmVxdWVzdBgHIAEoCzIiLnFhdWwucnBjLmR0bi5EdG5SZW1vdmVV'
    'c2VyUmVxdWVzdEgAUhRkdG5SZW1vdmVVc2VyUmVxdWVzdBJeChhkdG5fcmVtb3ZlX3VzZXJfcm'
    'VzcG9uc2UYCCABKAsyIy5xYXVsLnJwYy5kdG4uRHRuUmVtb3ZlVXNlclJlc3BvbnNlSABSFWR0'
    'blJlbW92ZVVzZXJSZXNwb25zZRJiChpkdG5fc2V0X3RvdGFsX3NpemVfcmVxdWVzdBgJIAEoCz'
    'IkLnFhdWwucnBjLmR0bi5EdG5TZXRUb3RhbFNpemVSZXF1ZXN0SABSFmR0blNldFRvdGFsU2l6'
    'ZVJlcXVlc3QSZQobZHRuX3NldF90b3RhbF9zaXplX3Jlc3BvbnNlGAogASgLMiUucWF1bC5ycG'
    'MuZHRuLkR0blNldFRvdGFsU2l6ZVJlc3BvbnNlSABSF2R0blNldFRvdGFsU2l6ZVJlc3BvbnNl'
    'QgkKB21lc3NhZ2U=');

@$core.Deprecated('Use dtnStateRequestDescriptor instead')
const DtnStateRequest$json = {
  '1': 'DtnStateRequest',
};

/// Descriptor for `DtnStateRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dtnStateRequestDescriptor = $convert.base64Decode(
    'Cg9EdG5TdGF0ZVJlcXVlc3Q=');

@$core.Deprecated('Use dtnStateResponseDescriptor instead')
const DtnStateResponse$json = {
  '1': 'DtnStateResponse',
  '2': [
    {'1': 'used_size', '3': 1, '4': 1, '5': 4, '10': 'usedSize'},
    {'1': 'dtn_message_count', '3': 2, '4': 1, '5': 13, '10': 'dtnMessageCount'},
    {'1': 'unconfirmed_count', '3': 3, '4': 1, '5': 13, '10': 'unconfirmedCount'},
  ],
};

/// Descriptor for `DtnStateResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dtnStateResponseDescriptor = $convert.base64Decode(
    'ChBEdG5TdGF0ZVJlc3BvbnNlEhsKCXVzZWRfc2l6ZRgBIAEoBFIIdXNlZFNpemUSKgoRZHRuX2'
    '1lc3NhZ2VfY291bnQYAiABKA1SD2R0bk1lc3NhZ2VDb3VudBIrChF1bmNvbmZpcm1lZF9jb3Vu'
    'dBgDIAEoDVIQdW5jb25maXJtZWRDb3VudA==');

@$core.Deprecated('Use dtnConfigRequestDescriptor instead')
const DtnConfigRequest$json = {
  '1': 'DtnConfigRequest',
};

/// Descriptor for `DtnConfigRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dtnConfigRequestDescriptor = $convert.base64Decode(
    'ChBEdG5Db25maWdSZXF1ZXN0');

@$core.Deprecated('Use dtnConfigResponseDescriptor instead')
const DtnConfigResponse$json = {
  '1': 'DtnConfigResponse',
  '2': [
    {'1': 'total_size', '3': 1, '4': 1, '5': 13, '10': 'totalSize'},
    {'1': 'users', '3': 2, '4': 3, '5': 12, '10': 'users'},
  ],
};

/// Descriptor for `DtnConfigResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dtnConfigResponseDescriptor = $convert.base64Decode(
    'ChFEdG5Db25maWdSZXNwb25zZRIdCgp0b3RhbF9zaXplGAEgASgNUgl0b3RhbFNpemUSFAoFdX'
    'NlcnMYAiADKAxSBXVzZXJz');

@$core.Deprecated('Use dtnAddUserRequestDescriptor instead')
const DtnAddUserRequest$json = {
  '1': 'DtnAddUserRequest',
  '2': [
    {'1': 'user_id', '3': 1, '4': 1, '5': 12, '10': 'userId'},
  ],
};

/// Descriptor for `DtnAddUserRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dtnAddUserRequestDescriptor = $convert.base64Decode(
    'ChFEdG5BZGRVc2VyUmVxdWVzdBIXCgd1c2VyX2lkGAEgASgMUgZ1c2VySWQ=');

@$core.Deprecated('Use dtnAddUserResponseDescriptor instead')
const DtnAddUserResponse$json = {
  '1': 'DtnAddUserResponse',
  '2': [
    {'1': 'status', '3': 1, '4': 1, '5': 8, '10': 'status'},
    {'1': 'message', '3': 2, '4': 1, '5': 9, '10': 'message'},
  ],
};

/// Descriptor for `DtnAddUserResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dtnAddUserResponseDescriptor = $convert.base64Decode(
    'ChJEdG5BZGRVc2VyUmVzcG9uc2USFgoGc3RhdHVzGAEgASgIUgZzdGF0dXMSGAoHbWVzc2FnZR'
    'gCIAEoCVIHbWVzc2FnZQ==');

@$core.Deprecated('Use dtnRemoveUserRequestDescriptor instead')
const DtnRemoveUserRequest$json = {
  '1': 'DtnRemoveUserRequest',
  '2': [
    {'1': 'user_id', '3': 1, '4': 1, '5': 12, '10': 'userId'},
  ],
};

/// Descriptor for `DtnRemoveUserRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dtnRemoveUserRequestDescriptor = $convert.base64Decode(
    'ChREdG5SZW1vdmVVc2VyUmVxdWVzdBIXCgd1c2VyX2lkGAEgASgMUgZ1c2VySWQ=');

@$core.Deprecated('Use dtnRemoveUserResponseDescriptor instead')
const DtnRemoveUserResponse$json = {
  '1': 'DtnRemoveUserResponse',
  '2': [
    {'1': 'status', '3': 1, '4': 1, '5': 8, '10': 'status'},
    {'1': 'message', '3': 2, '4': 1, '5': 9, '10': 'message'},
  ],
};

/// Descriptor for `DtnRemoveUserResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dtnRemoveUserResponseDescriptor = $convert.base64Decode(
    'ChVEdG5SZW1vdmVVc2VyUmVzcG9uc2USFgoGc3RhdHVzGAEgASgIUgZzdGF0dXMSGAoHbWVzc2'
    'FnZRgCIAEoCVIHbWVzc2FnZQ==');

@$core.Deprecated('Use dtnSetTotalSizeRequestDescriptor instead')
const DtnSetTotalSizeRequest$json = {
  '1': 'DtnSetTotalSizeRequest',
  '2': [
    {'1': 'total_size', '3': 1, '4': 1, '5': 13, '10': 'totalSize'},
  ],
};

/// Descriptor for `DtnSetTotalSizeRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dtnSetTotalSizeRequestDescriptor = $convert.base64Decode(
    'ChZEdG5TZXRUb3RhbFNpemVSZXF1ZXN0Eh0KCnRvdGFsX3NpemUYASABKA1SCXRvdGFsU2l6ZQ'
    '==');

@$core.Deprecated('Use dtnSetTotalSizeResponseDescriptor instead')
const DtnSetTotalSizeResponse$json = {
  '1': 'DtnSetTotalSizeResponse',
  '2': [
    {'1': 'status', '3': 1, '4': 1, '5': 8, '10': 'status'},
    {'1': 'message', '3': 2, '4': 1, '5': 9, '10': 'message'},
  ],
};

/// Descriptor for `DtnSetTotalSizeResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dtnSetTotalSizeResponseDescriptor = $convert.base64Decode(
    'ChdEdG5TZXRUb3RhbFNpemVSZXNwb25zZRIWCgZzdGF0dXMYASABKAhSBnN0YXR1cxIYCgdtZX'
    'NzYWdlGAIgASgJUgdtZXNzYWdl');

