///
//  Generated code. Do not modify.
//  source: services/filesharing/filesharing_rpc.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields,deprecated_member_use_from_same_package

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use fileSharingDescriptor instead')
const FileSharing$json = const {
  '1': 'FileSharing',
  '2': const [
    const {'1': 'send_file_request', '3': 1, '4': 1, '5': 11, '6': '.qaul.rpc.filesharing.SendFileRequest', '9': 0, '10': 'sendFileRequest'},
    const {'1': 'file_history', '3': 2, '4': 1, '5': 11, '6': '.qaul.rpc.filesharing.FileHistoryRequest', '9': 0, '10': 'fileHistory'},
    const {'1': 'file_history_response', '3': 3, '4': 1, '5': 11, '6': '.qaul.rpc.filesharing.FileHistoryResponse', '9': 0, '10': 'fileHistoryResponse'},
  ],
  '8': const [
    const {'1': 'message'},
  ],
};

/// Descriptor for `FileSharing`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fileSharingDescriptor = $convert.base64Decode('CgtGaWxlU2hhcmluZxJTChFzZW5kX2ZpbGVfcmVxdWVzdBgBIAEoCzIlLnFhdWwucnBjLmZpbGVzaGFyaW5nLlNlbmRGaWxlUmVxdWVzdEgAUg9zZW5kRmlsZVJlcXVlc3QSTQoMZmlsZV9oaXN0b3J5GAIgASgLMigucWF1bC5ycGMuZmlsZXNoYXJpbmcuRmlsZUhpc3RvcnlSZXF1ZXN0SABSC2ZpbGVIaXN0b3J5El8KFWZpbGVfaGlzdG9yeV9yZXNwb25zZRgDIAEoCzIpLnFhdWwucnBjLmZpbGVzaGFyaW5nLkZpbGVIaXN0b3J5UmVzcG9uc2VIAFITZmlsZUhpc3RvcnlSZXNwb25zZUIJCgdtZXNzYWdl');
@$core.Deprecated('Use sendFileRequestDescriptor instead')
const SendFileRequest$json = const {
  '1': 'SendFileRequest',
  '2': const [
    const {'1': 'path_name', '3': 1, '4': 1, '5': 9, '10': 'pathName'},
    const {'1': 'conversation_id', '3': 2, '4': 1, '5': 12, '10': 'conversationId'},
    const {'1': 'description', '3': 3, '4': 1, '5': 9, '10': 'description'},
  ],
};

/// Descriptor for `SendFileRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sendFileRequestDescriptor = $convert.base64Decode('Cg9TZW5kRmlsZVJlcXVlc3QSGwoJcGF0aF9uYW1lGAEgASgJUghwYXRoTmFtZRInCg9jb252ZXJzYXRpb25faWQYAiABKAxSDmNvbnZlcnNhdGlvbklkEiAKC2Rlc2NyaXB0aW9uGAMgASgJUgtkZXNjcmlwdGlvbg==');
@$core.Deprecated('Use fileHistoryRequestDescriptor instead')
const FileHistoryRequest$json = const {
  '1': 'FileHistoryRequest',
  '2': const [
    const {'1': 'offset', '3': 1, '4': 1, '5': 13, '10': 'offset'},
    const {'1': 'limit', '3': 2, '4': 1, '5': 13, '10': 'limit'},
  ],
};

/// Descriptor for `FileHistoryRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fileHistoryRequestDescriptor = $convert.base64Decode('ChJGaWxlSGlzdG9yeVJlcXVlc3QSFgoGb2Zmc2V0GAEgASgNUgZvZmZzZXQSFAoFbGltaXQYAiABKA1SBWxpbWl0');
@$core.Deprecated('Use fileHistoryEntryDescriptor instead')
const FileHistoryEntry$json = const {
  '1': 'FileHistoryEntry',
  '2': const [
    const {'1': 'file_id', '3': 1, '4': 1, '5': 4, '10': 'fileId'},
    const {'1': 'file_name', '3': 2, '4': 1, '5': 9, '10': 'fileName'},
    const {'1': 'file_ext', '3': 3, '4': 1, '5': 9, '10': 'fileExt'},
    const {'1': 'file_size', '3': 4, '4': 1, '5': 13, '10': 'fileSize'},
    const {'1': 'file_descr', '3': 5, '4': 1, '5': 9, '10': 'fileDescr'},
    const {'1': 'time', '3': 6, '4': 1, '5': 4, '10': 'time'},
    const {'1': 'sent', '3': 7, '4': 1, '5': 8, '10': 'sent'},
    const {'1': 'peer_id', '3': 8, '4': 1, '5': 9, '10': 'peerId'},
  ],
};

/// Descriptor for `FileHistoryEntry`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fileHistoryEntryDescriptor = $convert.base64Decode('ChBGaWxlSGlzdG9yeUVudHJ5EhcKB2ZpbGVfaWQYASABKARSBmZpbGVJZBIbCglmaWxlX25hbWUYAiABKAlSCGZpbGVOYW1lEhkKCGZpbGVfZXh0GAMgASgJUgdmaWxlRXh0EhsKCWZpbGVfc2l6ZRgEIAEoDVIIZmlsZVNpemUSHQoKZmlsZV9kZXNjchgFIAEoCVIJZmlsZURlc2NyEhIKBHRpbWUYBiABKARSBHRpbWUSEgoEc2VudBgHIAEoCFIEc2VudBIXCgdwZWVyX2lkGAggASgJUgZwZWVySWQ=');
@$core.Deprecated('Use fileHistoryResponseDescriptor instead')
const FileHistoryResponse$json = const {
  '1': 'FileHistoryResponse',
  '2': const [
    const {'1': 'offset', '3': 1, '4': 1, '5': 13, '10': 'offset'},
    const {'1': 'limit', '3': 2, '4': 1, '5': 13, '10': 'limit'},
    const {'1': 'total', '3': 3, '4': 1, '5': 4, '10': 'total'},
    const {'1': 'histories', '3': 4, '4': 3, '5': 11, '6': '.qaul.rpc.filesharing.FileHistoryEntry', '10': 'histories'},
  ],
};

/// Descriptor for `FileHistoryResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fileHistoryResponseDescriptor = $convert.base64Decode('ChNGaWxlSGlzdG9yeVJlc3BvbnNlEhYKBm9mZnNldBgBIAEoDVIGb2Zmc2V0EhQKBWxpbWl0GAIgASgNUgVsaW1pdBIUCgV0b3RhbBgDIAEoBFIFdG90YWwSRAoJaGlzdG9yaWVzGAQgAygLMiYucWF1bC5ycGMuZmlsZXNoYXJpbmcuRmlsZUhpc3RvcnlFbnRyeVIJaGlzdG9yaWVz');
