//
//  Generated code. Do not modify.
//  source: services/chat/chatfile_rpc.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use chatFileDescriptor instead')
const ChatFile$json = {
  '1': 'ChatFile',
  '2': [
    {'1': 'send_file_request', '3': 1, '4': 1, '5': 11, '6': '.qaul.rpc.chatfile.SendFileRequest', '9': 0, '10': 'sendFileRequest'},
    {'1': 'send_file_response', '3': 2, '4': 1, '5': 11, '6': '.qaul.rpc.chatfile.SendFileResponse', '9': 0, '10': 'sendFileResponse'},
    {'1': 'file_history', '3': 3, '4': 1, '5': 11, '6': '.qaul.rpc.chatfile.FileHistoryRequest', '9': 0, '10': 'fileHistory'},
    {'1': 'file_history_response', '3': 4, '4': 1, '5': 11, '6': '.qaul.rpc.chatfile.FileHistoryResponse', '9': 0, '10': 'fileHistoryResponse'},
  ],
  '8': [
    {'1': 'message'},
  ],
};

/// Descriptor for `ChatFile`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List chatFileDescriptor = $convert.base64Decode(
    'CghDaGF0RmlsZRJQChFzZW5kX2ZpbGVfcmVxdWVzdBgBIAEoCzIiLnFhdWwucnBjLmNoYXRmaW'
    'xlLlNlbmRGaWxlUmVxdWVzdEgAUg9zZW5kRmlsZVJlcXVlc3QSUwoSc2VuZF9maWxlX3Jlc3Bv'
    'bnNlGAIgASgLMiMucWF1bC5ycGMuY2hhdGZpbGUuU2VuZEZpbGVSZXNwb25zZUgAUhBzZW5kRm'
    'lsZVJlc3BvbnNlEkoKDGZpbGVfaGlzdG9yeRgDIAEoCzIlLnFhdWwucnBjLmNoYXRmaWxlLkZp'
    'bGVIaXN0b3J5UmVxdWVzdEgAUgtmaWxlSGlzdG9yeRJcChVmaWxlX2hpc3RvcnlfcmVzcG9uc2'
    'UYBCABKAsyJi5xYXVsLnJwYy5jaGF0ZmlsZS5GaWxlSGlzdG9yeVJlc3BvbnNlSABSE2ZpbGVI'
    'aXN0b3J5UmVzcG9uc2VCCQoHbWVzc2FnZQ==');

@$core.Deprecated('Use sendFileRequestDescriptor instead')
const SendFileRequest$json = {
  '1': 'SendFileRequest',
  '2': [
    {'1': 'path_name', '3': 1, '4': 1, '5': 9, '10': 'pathName'},
    {'1': 'group_id', '3': 2, '4': 1, '5': 12, '10': 'groupId'},
    {'1': 'description', '3': 3, '4': 1, '5': 9, '10': 'description'},
  ],
};

/// Descriptor for `SendFileRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sendFileRequestDescriptor = $convert.base64Decode(
    'Cg9TZW5kRmlsZVJlcXVlc3QSGwoJcGF0aF9uYW1lGAEgASgJUghwYXRoTmFtZRIZCghncm91cF'
    '9pZBgCIAEoDFIHZ3JvdXBJZBIgCgtkZXNjcmlwdGlvbhgDIAEoCVILZGVzY3JpcHRpb24=');

@$core.Deprecated('Use sendFileResponseDescriptor instead')
const SendFileResponse$json = {
  '1': 'SendFileResponse',
  '2': [
    {'1': 'success', '3': 1, '4': 1, '5': 8, '10': 'success'},
    {'1': 'error', '3': 2, '4': 1, '5': 9, '10': 'error'},
    {'1': 'file_id', '3': 3, '4': 1, '5': 4, '10': 'fileId'},
  ],
};

/// Descriptor for `SendFileResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sendFileResponseDescriptor = $convert.base64Decode(
    'ChBTZW5kRmlsZVJlc3BvbnNlEhgKB3N1Y2Nlc3MYASABKAhSB3N1Y2Nlc3MSFAoFZXJyb3IYAi'
    'ABKAlSBWVycm9yEhcKB2ZpbGVfaWQYAyABKARSBmZpbGVJZA==');

@$core.Deprecated('Use fileHistoryRequestDescriptor instead')
const FileHistoryRequest$json = {
  '1': 'FileHistoryRequest',
  '2': [
    {'1': 'offset', '3': 1, '4': 1, '5': 13, '10': 'offset'},
    {'1': 'limit', '3': 2, '4': 1, '5': 13, '10': 'limit'},
  ],
};

/// Descriptor for `FileHistoryRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fileHistoryRequestDescriptor = $convert.base64Decode(
    'ChJGaWxlSGlzdG9yeVJlcXVlc3QSFgoGb2Zmc2V0GAEgASgNUgZvZmZzZXQSFAoFbGltaXQYAi'
    'ABKA1SBWxpbWl0');

@$core.Deprecated('Use fileHistoryEntryDescriptor instead')
const FileHistoryEntry$json = {
  '1': 'FileHistoryEntry',
  '2': [
    {'1': 'file_id', '3': 1, '4': 1, '5': 4, '10': 'fileId'},
    {'1': 'file_name', '3': 2, '4': 1, '5': 9, '10': 'fileName'},
    {'1': 'file_extension', '3': 3, '4': 1, '5': 9, '10': 'fileExtension'},
    {'1': 'file_size', '3': 4, '4': 1, '5': 13, '10': 'fileSize'},
    {'1': 'file_description', '3': 5, '4': 1, '5': 9, '10': 'fileDescription'},
    {'1': 'time', '3': 6, '4': 1, '5': 4, '10': 'time'},
    {'1': 'sender_id', '3': 7, '4': 1, '5': 9, '10': 'senderId'},
    {'1': 'group_id', '3': 8, '4': 1, '5': 9, '10': 'groupId'},
  ],
};

/// Descriptor for `FileHistoryEntry`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fileHistoryEntryDescriptor = $convert.base64Decode(
    'ChBGaWxlSGlzdG9yeUVudHJ5EhcKB2ZpbGVfaWQYASABKARSBmZpbGVJZBIbCglmaWxlX25hbW'
    'UYAiABKAlSCGZpbGVOYW1lEiUKDmZpbGVfZXh0ZW5zaW9uGAMgASgJUg1maWxlRXh0ZW5zaW9u'
    'EhsKCWZpbGVfc2l6ZRgEIAEoDVIIZmlsZVNpemUSKQoQZmlsZV9kZXNjcmlwdGlvbhgFIAEoCV'
    'IPZmlsZURlc2NyaXB0aW9uEhIKBHRpbWUYBiABKARSBHRpbWUSGwoJc2VuZGVyX2lkGAcgASgJ'
    'UghzZW5kZXJJZBIZCghncm91cF9pZBgIIAEoCVIHZ3JvdXBJZA==');

@$core.Deprecated('Use fileHistoryResponseDescriptor instead')
const FileHistoryResponse$json = {
  '1': 'FileHistoryResponse',
  '2': [
    {'1': 'offset', '3': 1, '4': 1, '5': 13, '10': 'offset'},
    {'1': 'limit', '3': 2, '4': 1, '5': 13, '10': 'limit'},
    {'1': 'total', '3': 3, '4': 1, '5': 4, '10': 'total'},
    {'1': 'histories', '3': 4, '4': 3, '5': 11, '6': '.qaul.rpc.chatfile.FileHistoryEntry', '10': 'histories'},
  ],
};

/// Descriptor for `FileHistoryResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fileHistoryResponseDescriptor = $convert.base64Decode(
    'ChNGaWxlSGlzdG9yeVJlc3BvbnNlEhYKBm9mZnNldBgBIAEoDVIGb2Zmc2V0EhQKBWxpbWl0GA'
    'IgASgNUgVsaW1pdBIUCgV0b3RhbBgDIAEoBFIFdG90YWwSQQoJaGlzdG9yaWVzGAQgAygLMiMu'
    'cWF1bC5ycGMuY2hhdGZpbGUuRmlsZUhpc3RvcnlFbnRyeVIJaGlzdG9yaWVz');

