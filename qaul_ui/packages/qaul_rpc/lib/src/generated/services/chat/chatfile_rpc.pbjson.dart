///
//  Generated code. Do not modify.
//  source: services/chat/chatfile_rpc.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,deprecated_member_use_from_same_package,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use chatFileDescriptor instead')
const ChatFile$json = const {
  '1': 'ChatFile',
  '2': const [
    const {'1': 'send_file_request', '3': 1, '4': 1, '5': 11, '6': '.qaul.rpc.chatfile.SendFileRequest', '9': 0, '10': 'sendFileRequest'},
    const {'1': 'send_file_response', '3': 2, '4': 1, '5': 11, '6': '.qaul.rpc.chatfile.SendFileResponse', '9': 0, '10': 'sendFileResponse'},
    const {'1': 'file_history', '3': 3, '4': 1, '5': 11, '6': '.qaul.rpc.chatfile.FileHistoryRequest', '9': 0, '10': 'fileHistory'},
    const {'1': 'file_history_response', '3': 4, '4': 1, '5': 11, '6': '.qaul.rpc.chatfile.FileHistoryResponse', '9': 0, '10': 'fileHistoryResponse'},
  ],
  '8': const [
    const {'1': 'message'},
  ],
};

/// Descriptor for `ChatFile`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List chatFileDescriptor = $convert.base64Decode('CghDaGF0RmlsZRJQChFzZW5kX2ZpbGVfcmVxdWVzdBgBIAEoCzIiLnFhdWwucnBjLmNoYXRmaWxlLlNlbmRGaWxlUmVxdWVzdEgAUg9zZW5kRmlsZVJlcXVlc3QSUwoSc2VuZF9maWxlX3Jlc3BvbnNlGAIgASgLMiMucWF1bC5ycGMuY2hhdGZpbGUuU2VuZEZpbGVSZXNwb25zZUgAUhBzZW5kRmlsZVJlc3BvbnNlEkoKDGZpbGVfaGlzdG9yeRgDIAEoCzIlLnFhdWwucnBjLmNoYXRmaWxlLkZpbGVIaXN0b3J5UmVxdWVzdEgAUgtmaWxlSGlzdG9yeRJcChVmaWxlX2hpc3RvcnlfcmVzcG9uc2UYBCABKAsyJi5xYXVsLnJwYy5jaGF0ZmlsZS5GaWxlSGlzdG9yeVJlc3BvbnNlSABSE2ZpbGVIaXN0b3J5UmVzcG9uc2VCCQoHbWVzc2FnZQ==');
@$core.Deprecated('Use sendFileRequestDescriptor instead')
const SendFileRequest$json = const {
  '1': 'SendFileRequest',
  '2': const [
    const {'1': 'path_name', '3': 1, '4': 1, '5': 9, '10': 'pathName'},
    const {'1': 'group_id', '3': 2, '4': 1, '5': 12, '10': 'groupId'},
    const {'1': 'description', '3': 3, '4': 1, '5': 9, '10': 'description'},
  ],
};

/// Descriptor for `SendFileRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sendFileRequestDescriptor = $convert.base64Decode('Cg9TZW5kRmlsZVJlcXVlc3QSGwoJcGF0aF9uYW1lGAEgASgJUghwYXRoTmFtZRIZCghncm91cF9pZBgCIAEoDFIHZ3JvdXBJZBIgCgtkZXNjcmlwdGlvbhgDIAEoCVILZGVzY3JpcHRpb24=');
@$core.Deprecated('Use sendFileResponseDescriptor instead')
const SendFileResponse$json = const {
  '1': 'SendFileResponse',
  '2': const [
    const {'1': 'success', '3': 1, '4': 1, '5': 8, '10': 'success'},
    const {'1': 'error', '3': 2, '4': 1, '5': 9, '10': 'error'},
    const {'1': 'file_id', '3': 3, '4': 1, '5': 4, '10': 'fileId'},
  ],
};

/// Descriptor for `SendFileResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sendFileResponseDescriptor = $convert.base64Decode('ChBTZW5kRmlsZVJlc3BvbnNlEhgKB3N1Y2Nlc3MYASABKAhSB3N1Y2Nlc3MSFAoFZXJyb3IYAiABKAlSBWVycm9yEhcKB2ZpbGVfaWQYAyABKARSBmZpbGVJZA==');
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
    const {'1': 'file_extension', '3': 3, '4': 1, '5': 9, '10': 'fileExtension'},
    const {'1': 'file_size', '3': 4, '4': 1, '5': 13, '10': 'fileSize'},
    const {'1': 'file_description', '3': 5, '4': 1, '5': 9, '10': 'fileDescription'},
    const {'1': 'time', '3': 6, '4': 1, '5': 4, '10': 'time'},
    const {'1': 'sender_id', '3': 7, '4': 1, '5': 9, '10': 'senderId'},
    const {'1': 'group_id', '3': 8, '4': 1, '5': 9, '10': 'groupId'},
  ],
};

/// Descriptor for `FileHistoryEntry`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fileHistoryEntryDescriptor = $convert.base64Decode('ChBGaWxlSGlzdG9yeUVudHJ5EhcKB2ZpbGVfaWQYASABKARSBmZpbGVJZBIbCglmaWxlX25hbWUYAiABKAlSCGZpbGVOYW1lEiUKDmZpbGVfZXh0ZW5zaW9uGAMgASgJUg1maWxlRXh0ZW5zaW9uEhsKCWZpbGVfc2l6ZRgEIAEoDVIIZmlsZVNpemUSKQoQZmlsZV9kZXNjcmlwdGlvbhgFIAEoCVIPZmlsZURlc2NyaXB0aW9uEhIKBHRpbWUYBiABKARSBHRpbWUSGwoJc2VuZGVyX2lkGAcgASgJUghzZW5kZXJJZBIZCghncm91cF9pZBgIIAEoCVIHZ3JvdXBJZA==');
@$core.Deprecated('Use fileHistoryResponseDescriptor instead')
const FileHistoryResponse$json = const {
  '1': 'FileHistoryResponse',
  '2': const [
    const {'1': 'offset', '3': 1, '4': 1, '5': 13, '10': 'offset'},
    const {'1': 'limit', '3': 2, '4': 1, '5': 13, '10': 'limit'},
    const {'1': 'total', '3': 3, '4': 1, '5': 4, '10': 'total'},
    const {'1': 'histories', '3': 4, '4': 3, '5': 11, '6': '.qaul.rpc.chatfile.FileHistoryEntry', '10': 'histories'},
  ],
};

/// Descriptor for `FileHistoryResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fileHistoryResponseDescriptor = $convert.base64Decode('ChNGaWxlSGlzdG9yeVJlc3BvbnNlEhYKBm9mZnNldBgBIAEoDVIGb2Zmc2V0EhQKBWxpbWl0GAIgASgNUgVsaW1pdBIUCgV0b3RhbBgDIAEoBFIFdG90YWwSQQoJaGlzdG9yaWVzGAQgAygLMiMucWF1bC5ycGMuY2hhdGZpbGUuRmlsZUhpc3RvcnlFbnRyeVIJaGlzdG9yaWVz');
