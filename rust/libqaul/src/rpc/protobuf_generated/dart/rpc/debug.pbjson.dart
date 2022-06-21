///
//  Generated code. Do not modify.
//  source: rpc/debug.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,deprecated_member_use_from_same_package,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use debugDescriptor instead')
const Debug$json = const {
  '1': 'Debug',
  '2': const [
    const {'1': 'heartbeat_request', '3': 1, '4': 1, '5': 11, '6': '.qaul.rpc.debug.HeartbeatRequest', '9': 0, '10': 'heartbeatRequest'},
    const {'1': 'heartbeat_response', '3': 2, '4': 1, '5': 11, '6': '.qaul.rpc.debug.HeartbeatResponse', '9': 0, '10': 'heartbeatResponse'},
    const {'1': 'panic', '3': 3, '4': 1, '5': 11, '6': '.qaul.rpc.debug.Panic', '9': 0, '10': 'panic'},
    const {'1': 'log_to_file', '3': 4, '4': 1, '5': 11, '6': '.qaul.rpc.debug.LogToFile', '9': 0, '10': 'logToFile'},
    const {'1': 'storage_path_request', '3': 5, '4': 1, '5': 11, '6': '.qaul.rpc.debug.StoragePathRequest', '9': 0, '10': 'storagePathRequest'},
    const {'1': 'storage_path_response', '3': 6, '4': 1, '5': 11, '6': '.qaul.rpc.debug.StoragePathResponse', '9': 0, '10': 'storagePathResponse'},
    const {'1': 'delete_libqaul_logs_request', '3': 7, '4': 1, '5': 11, '6': '.qaul.rpc.debug.DeleteLibqaulLogsRequest', '9': 0, '10': 'deleteLibqaulLogsRequest'},
  ],
  '8': const [
    const {'1': 'message'},
  ],
};

/// Descriptor for `Debug`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List debugDescriptor = $convert.base64Decode('CgVEZWJ1ZxJPChFoZWFydGJlYXRfcmVxdWVzdBgBIAEoCzIgLnFhdWwucnBjLmRlYnVnLkhlYXJ0YmVhdFJlcXVlc3RIAFIQaGVhcnRiZWF0UmVxdWVzdBJSChJoZWFydGJlYXRfcmVzcG9uc2UYAiABKAsyIS5xYXVsLnJwYy5kZWJ1Zy5IZWFydGJlYXRSZXNwb25zZUgAUhFoZWFydGJlYXRSZXNwb25zZRItCgVwYW5pYxgDIAEoCzIVLnFhdWwucnBjLmRlYnVnLlBhbmljSABSBXBhbmljEjsKC2xvZ190b19maWxlGAQgASgLMhkucWF1bC5ycGMuZGVidWcuTG9nVG9GaWxlSABSCWxvZ1RvRmlsZRJWChRzdG9yYWdlX3BhdGhfcmVxdWVzdBgFIAEoCzIiLnFhdWwucnBjLmRlYnVnLlN0b3JhZ2VQYXRoUmVxdWVzdEgAUhJzdG9yYWdlUGF0aFJlcXVlc3QSWQoVc3RvcmFnZV9wYXRoX3Jlc3BvbnNlGAYgASgLMiMucWF1bC5ycGMuZGVidWcuU3RvcmFnZVBhdGhSZXNwb25zZUgAUhNzdG9yYWdlUGF0aFJlc3BvbnNlEmkKG2RlbGV0ZV9saWJxYXVsX2xvZ3NfcmVxdWVzdBgHIAEoCzIoLnFhdWwucnBjLmRlYnVnLkRlbGV0ZUxpYnFhdWxMb2dzUmVxdWVzdEgAUhhkZWxldGVMaWJxYXVsTG9nc1JlcXVlc3RCCQoHbWVzc2FnZQ==');
@$core.Deprecated('Use heartbeatRequestDescriptor instead')
const HeartbeatRequest$json = const {
  '1': 'HeartbeatRequest',
};

/// Descriptor for `HeartbeatRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List heartbeatRequestDescriptor = $convert.base64Decode('ChBIZWFydGJlYXRSZXF1ZXN0');
@$core.Deprecated('Use heartbeatResponseDescriptor instead')
const HeartbeatResponse$json = const {
  '1': 'HeartbeatResponse',
};

/// Descriptor for `HeartbeatResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List heartbeatResponseDescriptor = $convert.base64Decode('ChFIZWFydGJlYXRSZXNwb25zZQ==');
@$core.Deprecated('Use panicDescriptor instead')
const Panic$json = const {
  '1': 'Panic',
};

/// Descriptor for `Panic`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List panicDescriptor = $convert.base64Decode('CgVQYW5pYw==');
@$core.Deprecated('Use logToFileDescriptor instead')
const LogToFile$json = const {
  '1': 'LogToFile',
  '2': const [
    const {'1': 'enable', '3': 1, '4': 1, '5': 8, '10': 'enable'},
  ],
};

/// Descriptor for `LogToFile`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List logToFileDescriptor = $convert.base64Decode('CglMb2dUb0ZpbGUSFgoGZW5hYmxlGAEgASgIUgZlbmFibGU=');
@$core.Deprecated('Use storagePathRequestDescriptor instead')
const StoragePathRequest$json = const {
  '1': 'StoragePathRequest',
};

/// Descriptor for `StoragePathRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List storagePathRequestDescriptor = $convert.base64Decode('ChJTdG9yYWdlUGF0aFJlcXVlc3Q=');
@$core.Deprecated('Use storagePathResponseDescriptor instead')
const StoragePathResponse$json = const {
  '1': 'StoragePathResponse',
  '2': const [
    const {'1': 'storage_path', '3': 1, '4': 1, '5': 9, '10': 'storagePath'},
  ],
};

/// Descriptor for `StoragePathResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List storagePathResponseDescriptor = $convert.base64Decode('ChNTdG9yYWdlUGF0aFJlc3BvbnNlEiEKDHN0b3JhZ2VfcGF0aBgBIAEoCVILc3RvcmFnZVBhdGg=');
@$core.Deprecated('Use deleteLibqaulLogsRequestDescriptor instead')
const DeleteLibqaulLogsRequest$json = const {
  '1': 'DeleteLibqaulLogsRequest',
};

/// Descriptor for `DeleteLibqaulLogsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deleteLibqaulLogsRequestDescriptor = $convert.base64Decode('ChhEZWxldGVMaWJxYXVsTG9nc1JlcXVlc3Q=');
