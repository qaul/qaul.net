///
//  Generated code. Do not modify.
//  source: rpc/debug.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields,deprecated_member_use_from_same_package

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
  ],
  '8': const [
    const {'1': 'message'},
  ],
};

/// Descriptor for `Debug`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List debugDescriptor = $convert.base64Decode('CgVEZWJ1ZxJPChFoZWFydGJlYXRfcmVxdWVzdBgBIAEoCzIgLnFhdWwucnBjLmRlYnVnLkhlYXJ0YmVhdFJlcXVlc3RIAFIQaGVhcnRiZWF0UmVxdWVzdBJSChJoZWFydGJlYXRfcmVzcG9uc2UYAiABKAsyIS5xYXVsLnJwYy5kZWJ1Zy5IZWFydGJlYXRSZXNwb25zZUgAUhFoZWFydGJlYXRSZXNwb25zZRItCgVwYW5pYxgDIAEoCzIVLnFhdWwucnBjLmRlYnVnLlBhbmljSABSBXBhbmljQgkKB21lc3NhZ2U=');
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
