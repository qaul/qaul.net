///
//  Generated code. Do not modify.
//  source: from_libqaul.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields,deprecated_member_use_from_same_package

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use fromLibqaulDescriptor instead')
const FromLibqaul$json = const {
  '1': 'FromLibqaul',
  '2': const [
    const {'1': 'node', '3': 1, '4': 1, '5': 11, '6': '.QaulRpc.FromNode', '9': 0, '10': 'node'},
    const {'1': 'router', '3': 2, '4': 1, '5': 11, '6': '.QaulRpc.FromRouter', '9': 0, '10': 'router'},
    const {'1': 'feed', '3': 3, '4': 1, '5': 11, '6': '.QaulRpc.FromFeed', '9': 0, '10': 'feed'},
  ],
  '8': const [
    const {'1': 'module'},
  ],
};

/// Descriptor for `FromLibqaul`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fromLibqaulDescriptor = $convert.base64Decode('CgtGcm9tTGlicWF1bBInCgRub2RlGAEgASgLMhEuUWF1bFJwYy5Gcm9tTm9kZUgAUgRub2RlEi0KBnJvdXRlchgCIAEoCzITLlFhdWxScGMuRnJvbVJvdXRlckgAUgZyb3V0ZXISJwoEZmVlZBgDIAEoCzIRLlFhdWxScGMuRnJvbUZlZWRIAFIEZmVlZEIICgZtb2R1bGU=');
@$core.Deprecated('Use fromNodeDescriptor instead')
const FromNode$json = const {
  '1': 'FromNode',
  '2': const [
    const {'1': 'session', '3': 1, '4': 1, '5': 11, '6': '.QaulRpc.SessionInformation', '9': 0, '10': 'session'},
    const {'1': 'my_user', '3': 2, '4': 1, '5': 11, '6': '.QaulRpc.MyUser', '9': 0, '10': 'myUser'},
  ],
  '8': const [
    const {'1': 'type'},
  ],
};

/// Descriptor for `FromNode`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fromNodeDescriptor = $convert.base64Decode('CghGcm9tTm9kZRI3CgdzZXNzaW9uGAEgASgLMhsuUWF1bFJwYy5TZXNzaW9uSW5mb3JtYXRpb25IAFIHc2Vzc2lvbhIqCgdteV91c2VyGAIgASgLMg8uUWF1bFJwYy5NeVVzZXJIAFIGbXlVc2VyQgYKBHR5cGU=');
@$core.Deprecated('Use sessionInformationDescriptor instead')
const SessionInformation$json = const {
  '1': 'SessionInformation',
  '2': const [
    const {'1': 'user_exists', '3': 1, '4': 1, '5': 8, '10': 'userExists'},
    const {'1': 'my_user', '3': 2, '4': 1, '5': 11, '6': '.QaulRpc.MyUser', '10': 'myUser'},
  ],
};

/// Descriptor for `SessionInformation`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sessionInformationDescriptor = $convert.base64Decode('ChJTZXNzaW9uSW5mb3JtYXRpb24SHwoLdXNlcl9leGlzdHMYASABKAhSCnVzZXJFeGlzdHMSKAoHbXlfdXNlchgCIAEoCzIPLlFhdWxScGMuTXlVc2VyUgZteVVzZXI=');
@$core.Deprecated('Use myUserDescriptor instead')
const MyUser$json = const {
  '1': 'MyUser',
  '2': const [
    const {'1': 'name', '3': 1, '4': 1, '5': 9, '10': 'name'},
    const {'1': 'id', '3': 2, '4': 1, '5': 12, '10': 'id'},
  ],
};

/// Descriptor for `MyUser`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List myUserDescriptor = $convert.base64Decode('CgZNeVVzZXISEgoEbmFtZRgBIAEoCVIEbmFtZRIOCgJpZBgCIAEoDFICaWQ=');
@$core.Deprecated('Use fromRouterDescriptor instead')
const FromRouter$json = const {
  '1': 'FromRouter',
  '2': const [
    const {'1': 'user_list', '3': 1, '4': 1, '5': 11, '6': '.QaulRpc.UserList', '9': 0, '10': 'userList'},
  ],
  '8': const [
    const {'1': 'type'},
  ],
};

/// Descriptor for `FromRouter`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fromRouterDescriptor = $convert.base64Decode('CgpGcm9tUm91dGVyEjAKCXVzZXJfbGlzdBgBIAEoCzIRLlFhdWxScGMuVXNlckxpc3RIAFIIdXNlckxpc3RCBgoEdHlwZQ==');
@$core.Deprecated('Use userListDescriptor instead')
const UserList$json = const {
  '1': 'UserList',
  '2': const [
    const {'1': 'user', '3': 1, '4': 3, '5': 11, '6': '.QaulRpc.UserEntry', '10': 'user'},
  ],
};

/// Descriptor for `UserList`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List userListDescriptor = $convert.base64Decode('CghVc2VyTGlzdBImCgR1c2VyGAEgAygLMhIuUWF1bFJwYy5Vc2VyRW50cnlSBHVzZXI=');
@$core.Deprecated('Use userEntryDescriptor instead')
const UserEntry$json = const {
  '1': 'UserEntry',
  '2': const [
    const {'1': 'name', '3': 1, '4': 1, '5': 9, '10': 'name'},
    const {'1': 'id', '3': 2, '4': 1, '5': 12, '10': 'id'},
    const {'1': 'key', '3': 3, '4': 1, '5': 12, '10': 'key'},
  ],
  '4': const [UserEntry_Connectivity$json],
};

@$core.Deprecated('Use userEntryDescriptor instead')
const UserEntry_Connectivity$json = const {
  '1': 'Connectivity',
  '2': const [
    const {'1': 'Online', '2': 0},
    const {'1': 'Reachable', '2': 1},
    const {'1': 'Offline', '2': 2},
  ],
};

/// Descriptor for `UserEntry`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List userEntryDescriptor = $convert.base64Decode('CglVc2VyRW50cnkSEgoEbmFtZRgBIAEoCVIEbmFtZRIOCgJpZBgCIAEoDFICaWQSEAoDa2V5GAMgASgMUgNrZXkiNgoMQ29ubmVjdGl2aXR5EgoKBk9ubGluZRAAEg0KCVJlYWNoYWJsZRABEgsKB09mZmxpbmUQAg==');
@$core.Deprecated('Use fromFeedDescriptor instead')
const FromFeed$json = const {
  '1': 'FromFeed',
  '2': const [
    const {'1': 'message', '3': 1, '4': 1, '5': 11, '6': '.QaulRpc.FeedMessage', '9': 0, '10': 'message'},
  ],
  '8': const [
    const {'1': 'type'},
  ],
};

/// Descriptor for `FromFeed`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fromFeedDescriptor = $convert.base64Decode('CghGcm9tRmVlZBIwCgdtZXNzYWdlGAEgASgLMhQuUWF1bFJwYy5GZWVkTWVzc2FnZUgAUgdtZXNzYWdlQgYKBHR5cGU=');
@$core.Deprecated('Use feedMessageDescriptor instead')
const FeedMessage$json = const {
  '1': 'FeedMessage',
  '2': const [
    const {'1': 'sender_id', '3': 1, '4': 1, '5': 12, '10': 'senderId'},
    const {'1': 'message_id', '3': 2, '4': 1, '5': 12, '10': 'messageId'},
    const {'1': 'time_sent', '3': 3, '4': 1, '5': 9, '10': 'timeSent'},
    const {'1': 'time_received', '3': 4, '4': 1, '5': 9, '10': 'timeReceived'},
    const {'1': 'content', '3': 5, '4': 1, '5': 9, '10': 'content'},
  ],
};

/// Descriptor for `FeedMessage`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List feedMessageDescriptor = $convert.base64Decode('CgtGZWVkTWVzc2FnZRIbCglzZW5kZXJfaWQYASABKAxSCHNlbmRlcklkEh0KCm1lc3NhZ2VfaWQYAiABKAxSCW1lc3NhZ2VJZBIbCgl0aW1lX3NlbnQYAyABKAlSCHRpbWVTZW50EiMKDXRpbWVfcmVjZWl2ZWQYBCABKAlSDHRpbWVSZWNlaXZlZBIYCgdjb250ZW50GAUgASgJUgdjb250ZW50');
