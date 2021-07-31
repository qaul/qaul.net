///
//  Generated code. Do not modify.
//  source: to_libqaul.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields,deprecated_member_use_from_same_package

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use toLibqaulDescriptor instead')
const ToLibqaul$json = const {
  '1': 'ToLibqaul',
  '2': const [
    const {'1': 'node', '3': 1, '4': 1, '5': 11, '6': '.QaulRpc.ToNode', '9': 0, '10': 'node'},
    const {'1': 'router', '3': 2, '4': 1, '5': 11, '6': '.QaulRpc.ToRouter', '9': 0, '10': 'router'},
    const {'1': 'feed', '3': 3, '4': 1, '5': 11, '6': '.QaulRpc.ToFeed', '9': 0, '10': 'feed'},
  ],
  '8': const [
    const {'1': 'module'},
  ],
};

/// Descriptor for `ToLibqaul`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List toLibqaulDescriptor = $convert.base64Decode('CglUb0xpYnFhdWwSJQoEbm9kZRgBIAEoCzIPLlFhdWxScGMuVG9Ob2RlSABSBG5vZGUSKwoGcm91dGVyGAIgASgLMhEuUWF1bFJwYy5Ub1JvdXRlckgAUgZyb3V0ZXISJQoEZmVlZBgDIAEoCzIPLlFhdWxScGMuVG9GZWVkSABSBGZlZWRCCAoGbW9kdWxl');
@$core.Deprecated('Use toNodeDescriptor instead')
const ToNode$json = const {
  '1': 'ToNode',
  '2': const [
    const {'1': 'start_session', '3': 1, '4': 1, '5': 8, '9': 0, '10': 'startSession'},
    const {'1': 'create_user', '3': 2, '4': 1, '5': 11, '6': '.QaulRpc.CreateUser', '9': 0, '10': 'createUser'},
  ],
  '8': const [
    const {'1': 'type'},
  ],
};

/// Descriptor for `ToNode`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List toNodeDescriptor = $convert.base64Decode('CgZUb05vZGUSJQoNc3RhcnRfc2Vzc2lvbhgBIAEoCEgAUgxzdGFydFNlc3Npb24SNgoLY3JlYXRlX3VzZXIYAiABKAsyEy5RYXVsUnBjLkNyZWF0ZVVzZXJIAFIKY3JlYXRlVXNlckIGCgR0eXBl');
@$core.Deprecated('Use createUserDescriptor instead')
const CreateUser$json = const {
  '1': 'CreateUser',
  '2': const [
    const {'1': 'name', '3': 1, '4': 1, '5': 9, '10': 'name'},
  ],
};

/// Descriptor for `CreateUser`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createUserDescriptor = $convert.base64Decode('CgpDcmVhdGVVc2VyEhIKBG5hbWUYASABKAlSBG5hbWU=');
@$core.Deprecated('Use toRouterDescriptor instead')
const ToRouter$json = const {
  '1': 'ToRouter',
  '2': const [
    const {'1': 'request_users', '3': 1, '4': 1, '5': 11, '6': '.QaulRpc.RequestUsers', '9': 0, '10': 'requestUsers'},
  ],
  '8': const [
    const {'1': 'type'},
  ],
};

/// Descriptor for `ToRouter`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List toRouterDescriptor = $convert.base64Decode('CghUb1JvdXRlchI8Cg1yZXF1ZXN0X3VzZXJzGAEgASgLMhUuUWF1bFJwYy5SZXF1ZXN0VXNlcnNIAFIMcmVxdWVzdFVzZXJzQgYKBHR5cGU=');
@$core.Deprecated('Use requestUsersDescriptor instead')
const RequestUsers$json = const {
  '1': 'RequestUsers',
};

/// Descriptor for `RequestUsers`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List requestUsersDescriptor = $convert.base64Decode('CgxSZXF1ZXN0VXNlcnM=');
@$core.Deprecated('Use toFeedDescriptor instead')
const ToFeed$json = const {
  '1': 'ToFeed',
  '2': const [
    const {'1': 'send_feed', '3': 1, '4': 1, '5': 11, '6': '.QaulRpc.SendFeed', '9': 0, '10': 'sendFeed'},
  ],
  '8': const [
    const {'1': 'type'},
  ],
};

/// Descriptor for `ToFeed`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List toFeedDescriptor = $convert.base64Decode('CgZUb0ZlZWQSMAoJc2VuZF9mZWVkGAEgASgLMhEuUWF1bFJwYy5TZW5kRmVlZEgAUghzZW5kRmVlZEIGCgR0eXBl');
@$core.Deprecated('Use sendFeedDescriptor instead')
const SendFeed$json = const {
  '1': 'SendFeed',
  '2': const [
    const {'1': 'content', '3': 1, '4': 1, '5': 9, '10': 'content'},
  ],
};

/// Descriptor for `SendFeed`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sendFeedDescriptor = $convert.base64Decode('CghTZW5kRmVlZBIYCgdjb250ZW50GAEgASgJUgdjb250ZW50');
