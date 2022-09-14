///
//  Generated code. Do not modify.
//  source: services/dtn/dtn_rpc.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,deprecated_member_use_from_same_package,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use dTNDescriptor instead')
const DTN$json = const {
  '1': 'DTN',
  '2': const [
    const {'1': 'dtn_state_request', '3': 1, '4': 1, '5': 11, '6': '.qaul.rpc.dtn.DtnStateRequest', '9': 0, '10': 'dtnStateRequest'},
    const {'1': 'dtn_state_response', '3': 2, '4': 1, '5': 11, '6': '.qaul.rpc.dtn.DtnStateResponse', '9': 0, '10': 'dtnStateResponse'},
    const {'1': 'dtn_config_request', '3': 3, '4': 1, '5': 11, '6': '.qaul.rpc.dtn.DtnConfigRequest', '9': 0, '10': 'dtnConfigRequest'},
    const {'1': 'dtn_config_response', '3': 4, '4': 1, '5': 11, '6': '.qaul.rpc.dtn.DtnConfigResponse', '9': 0, '10': 'dtnConfigResponse'},
    const {'1': 'dtn_add_user_request', '3': 5, '4': 1, '5': 11, '6': '.qaul.rpc.dtn.DtnAddUserRequest', '9': 0, '10': 'dtnAddUserRequest'},
    const {'1': 'dtn_add_user_response', '3': 6, '4': 1, '5': 11, '6': '.qaul.rpc.dtn.DtnAddUserResponse', '9': 0, '10': 'dtnAddUserResponse'},
    const {'1': 'dtn_remove_user_request', '3': 7, '4': 1, '5': 11, '6': '.qaul.rpc.dtn.DtnRemoveUserRequest', '9': 0, '10': 'dtnRemoveUserRequest'},
    const {'1': 'dtn_remove_user_response', '3': 8, '4': 1, '5': 11, '6': '.qaul.rpc.dtn.DtnRemoveUserResponse', '9': 0, '10': 'dtnRemoveUserResponse'},
    const {'1': 'dtn_set_total_size_request', '3': 9, '4': 1, '5': 11, '6': '.qaul.rpc.dtn.DtnSetTotalSizeRequest', '9': 0, '10': 'dtnSetTotalSizeRequest'},
    const {'1': 'dtn_set_total_size_response', '3': 10, '4': 1, '5': 11, '6': '.qaul.rpc.dtn.DtnSetTotalSizeResponse', '9': 0, '10': 'dtnSetTotalSizeResponse'},
  ],
  '8': const [
    const {'1': 'message'},
  ],
};

/// Descriptor for `DTN`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dTNDescriptor = $convert.base64Decode('CgNEVE4SSwoRZHRuX3N0YXRlX3JlcXVlc3QYASABKAsyHS5xYXVsLnJwYy5kdG4uRHRuU3RhdGVSZXF1ZXN0SABSD2R0blN0YXRlUmVxdWVzdBJOChJkdG5fc3RhdGVfcmVzcG9uc2UYAiABKAsyHi5xYXVsLnJwYy5kdG4uRHRuU3RhdGVSZXNwb25zZUgAUhBkdG5TdGF0ZVJlc3BvbnNlEk4KEmR0bl9jb25maWdfcmVxdWVzdBgDIAEoCzIeLnFhdWwucnBjLmR0bi5EdG5Db25maWdSZXF1ZXN0SABSEGR0bkNvbmZpZ1JlcXVlc3QSUQoTZHRuX2NvbmZpZ19yZXNwb25zZRgEIAEoCzIfLnFhdWwucnBjLmR0bi5EdG5Db25maWdSZXNwb25zZUgAUhFkdG5Db25maWdSZXNwb25zZRJSChRkdG5fYWRkX3VzZXJfcmVxdWVzdBgFIAEoCzIfLnFhdWwucnBjLmR0bi5EdG5BZGRVc2VyUmVxdWVzdEgAUhFkdG5BZGRVc2VyUmVxdWVzdBJVChVkdG5fYWRkX3VzZXJfcmVzcG9uc2UYBiABKAsyIC5xYXVsLnJwYy5kdG4uRHRuQWRkVXNlclJlc3BvbnNlSABSEmR0bkFkZFVzZXJSZXNwb25zZRJbChdkdG5fcmVtb3ZlX3VzZXJfcmVxdWVzdBgHIAEoCzIiLnFhdWwucnBjLmR0bi5EdG5SZW1vdmVVc2VyUmVxdWVzdEgAUhRkdG5SZW1vdmVVc2VyUmVxdWVzdBJeChhkdG5fcmVtb3ZlX3VzZXJfcmVzcG9uc2UYCCABKAsyIy5xYXVsLnJwYy5kdG4uRHRuUmVtb3ZlVXNlclJlc3BvbnNlSABSFWR0blJlbW92ZVVzZXJSZXNwb25zZRJiChpkdG5fc2V0X3RvdGFsX3NpemVfcmVxdWVzdBgJIAEoCzIkLnFhdWwucnBjLmR0bi5EdG5TZXRUb3RhbFNpemVSZXF1ZXN0SABSFmR0blNldFRvdGFsU2l6ZVJlcXVlc3QSZQobZHRuX3NldF90b3RhbF9zaXplX3Jlc3BvbnNlGAogASgLMiUucWF1bC5ycGMuZHRuLkR0blNldFRvdGFsU2l6ZVJlc3BvbnNlSABSF2R0blNldFRvdGFsU2l6ZVJlc3BvbnNlQgkKB21lc3NhZ2U=');
@$core.Deprecated('Use dtnStateRequestDescriptor instead')
const DtnStateRequest$json = const {
  '1': 'DtnStateRequest',
};

/// Descriptor for `DtnStateRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dtnStateRequestDescriptor = $convert.base64Decode('Cg9EdG5TdGF0ZVJlcXVlc3Q=');
@$core.Deprecated('Use dtnStateResponseDescriptor instead')
const DtnStateResponse$json = const {
  '1': 'DtnStateResponse',
  '2': const [
    const {'1': 'used_size', '3': 1, '4': 1, '5': 4, '10': 'usedSize'},
    const {'1': 'dtn_message_count', '3': 2, '4': 1, '5': 13, '10': 'dtnMessageCount'},
    const {'1': 'unconfirmed_count', '3': 3, '4': 1, '5': 13, '10': 'unconfirmedCount'},
  ],
};

/// Descriptor for `DtnStateResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dtnStateResponseDescriptor = $convert.base64Decode('ChBEdG5TdGF0ZVJlc3BvbnNlEhsKCXVzZWRfc2l6ZRgBIAEoBFIIdXNlZFNpemUSKgoRZHRuX21lc3NhZ2VfY291bnQYAiABKA1SD2R0bk1lc3NhZ2VDb3VudBIrChF1bmNvbmZpcm1lZF9jb3VudBgDIAEoDVIQdW5jb25maXJtZWRDb3VudA==');
@$core.Deprecated('Use dtnConfigRequestDescriptor instead')
const DtnConfigRequest$json = const {
  '1': 'DtnConfigRequest',
};

/// Descriptor for `DtnConfigRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dtnConfigRequestDescriptor = $convert.base64Decode('ChBEdG5Db25maWdSZXF1ZXN0');
@$core.Deprecated('Use dtnConfigResponseDescriptor instead')
const DtnConfigResponse$json = const {
  '1': 'DtnConfigResponse',
  '2': const [
    const {'1': 'total_size', '3': 1, '4': 1, '5': 13, '10': 'totalSize'},
    const {'1': 'users', '3': 2, '4': 3, '5': 12, '10': 'users'},
  ],
};

/// Descriptor for `DtnConfigResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dtnConfigResponseDescriptor = $convert.base64Decode('ChFEdG5Db25maWdSZXNwb25zZRIdCgp0b3RhbF9zaXplGAEgASgNUgl0b3RhbFNpemUSFAoFdXNlcnMYAiADKAxSBXVzZXJz');
@$core.Deprecated('Use dtnAddUserRequestDescriptor instead')
const DtnAddUserRequest$json = const {
  '1': 'DtnAddUserRequest',
  '2': const [
    const {'1': 'user_id', '3': 1, '4': 1, '5': 12, '10': 'userId'},
  ],
};

/// Descriptor for `DtnAddUserRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dtnAddUserRequestDescriptor = $convert.base64Decode('ChFEdG5BZGRVc2VyUmVxdWVzdBIXCgd1c2VyX2lkGAEgASgMUgZ1c2VySWQ=');
@$core.Deprecated('Use dtnAddUserResponseDescriptor instead')
const DtnAddUserResponse$json = const {
  '1': 'DtnAddUserResponse',
  '2': const [
    const {'1': 'status', '3': 1, '4': 1, '5': 8, '10': 'status'},
    const {'1': 'message', '3': 2, '4': 1, '5': 9, '10': 'message'},
  ],
};

/// Descriptor for `DtnAddUserResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dtnAddUserResponseDescriptor = $convert.base64Decode('ChJEdG5BZGRVc2VyUmVzcG9uc2USFgoGc3RhdHVzGAEgASgIUgZzdGF0dXMSGAoHbWVzc2FnZRgCIAEoCVIHbWVzc2FnZQ==');
@$core.Deprecated('Use dtnRemoveUserRequestDescriptor instead')
const DtnRemoveUserRequest$json = const {
  '1': 'DtnRemoveUserRequest',
  '2': const [
    const {'1': 'user_id', '3': 1, '4': 1, '5': 12, '10': 'userId'},
  ],
};

/// Descriptor for `DtnRemoveUserRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dtnRemoveUserRequestDescriptor = $convert.base64Decode('ChREdG5SZW1vdmVVc2VyUmVxdWVzdBIXCgd1c2VyX2lkGAEgASgMUgZ1c2VySWQ=');
@$core.Deprecated('Use dtnRemoveUserResponseDescriptor instead')
const DtnRemoveUserResponse$json = const {
  '1': 'DtnRemoveUserResponse',
  '2': const [
    const {'1': 'status', '3': 1, '4': 1, '5': 8, '10': 'status'},
    const {'1': 'message', '3': 2, '4': 1, '5': 9, '10': 'message'},
  ],
};

/// Descriptor for `DtnRemoveUserResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dtnRemoveUserResponseDescriptor = $convert.base64Decode('ChVEdG5SZW1vdmVVc2VyUmVzcG9uc2USFgoGc3RhdHVzGAEgASgIUgZzdGF0dXMSGAoHbWVzc2FnZRgCIAEoCVIHbWVzc2FnZQ==');
@$core.Deprecated('Use dtnSetTotalSizeRequestDescriptor instead')
const DtnSetTotalSizeRequest$json = const {
  '1': 'DtnSetTotalSizeRequest',
  '2': const [
    const {'1': 'total_size', '3': 1, '4': 1, '5': 13, '10': 'totalSize'},
  ],
};

/// Descriptor for `DtnSetTotalSizeRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dtnSetTotalSizeRequestDescriptor = $convert.base64Decode('ChZEdG5TZXRUb3RhbFNpemVSZXF1ZXN0Eh0KCnRvdGFsX3NpemUYASABKA1SCXRvdGFsU2l6ZQ==');
@$core.Deprecated('Use dtnSetTotalSizeResponseDescriptor instead')
const DtnSetTotalSizeResponse$json = const {
  '1': 'DtnSetTotalSizeResponse',
  '2': const [
    const {'1': 'status', '3': 1, '4': 1, '5': 8, '10': 'status'},
    const {'1': 'message', '3': 2, '4': 1, '5': 9, '10': 'message'},
  ],
};

/// Descriptor for `DtnSetTotalSizeResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dtnSetTotalSizeResponseDescriptor = $convert.base64Decode('ChdEdG5TZXRUb3RhbFNpemVSZXNwb25zZRIWCgZzdGF0dXMYASABKAhSBnN0YXR1cxIYCgdtZXNzYWdlGAIgASgJUgdtZXNzYWdl');
