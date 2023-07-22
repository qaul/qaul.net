//
//  Generated code. Do not modify.
//  source: services/chat/chatfile_net.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use chatFileContainerDescriptor instead')
const ChatFileContainer$json = {
  '1': 'ChatFileContainer',
  '2': [
    {'1': 'file_info', '3': 1, '4': 1, '5': 11, '6': '.qaul.net.chatfile.ChatFileInfo', '9': 0, '10': 'fileInfo'},
    {'1': 'file_data', '3': 2, '4': 1, '5': 11, '6': '.qaul.net.chatfile.ChatFileData', '9': 0, '10': 'fileData'},
  ],
  '8': [
    {'1': 'message'},
  ],
};

/// Descriptor for `ChatFileContainer`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List chatFileContainerDescriptor = $convert.base64Decode(
    'ChFDaGF0RmlsZUNvbnRhaW5lchI+CglmaWxlX2luZm8YASABKAsyHy5xYXVsLm5ldC5jaGF0Zm'
    'lsZS5DaGF0RmlsZUluZm9IAFIIZmlsZUluZm8SPgoJZmlsZV9kYXRhGAIgASgLMh8ucWF1bC5u'
    'ZXQuY2hhdGZpbGUuQ2hhdEZpbGVEYXRhSABSCGZpbGVEYXRhQgkKB21lc3NhZ2U=');

@$core.Deprecated('Use chatFileInfoDescriptor instead')
const ChatFileInfo$json = {
  '1': 'ChatFileInfo',
  '2': [
    {'1': 'file_id', '3': 1, '4': 1, '5': 4, '10': 'fileId'},
    {'1': 'file_name', '3': 2, '4': 1, '5': 9, '10': 'fileName'},
    {'1': 'file_extension', '3': 3, '4': 1, '5': 9, '10': 'fileExtension'},
    {'1': 'file_size', '3': 4, '4': 1, '5': 13, '10': 'fileSize'},
    {'1': 'file_description', '3': 5, '4': 1, '5': 9, '10': 'fileDescription'},
    {'1': 'start_index', '3': 6, '4': 1, '5': 13, '10': 'startIndex'},
    {'1': 'message_count', '3': 7, '4': 1, '5': 13, '10': 'messageCount'},
    {'1': 'data_chunk_size', '3': 8, '4': 1, '5': 13, '10': 'dataChunkSize'},
  ],
};

/// Descriptor for `ChatFileInfo`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List chatFileInfoDescriptor = $convert.base64Decode(
    'CgxDaGF0RmlsZUluZm8SFwoHZmlsZV9pZBgBIAEoBFIGZmlsZUlkEhsKCWZpbGVfbmFtZRgCIA'
    'EoCVIIZmlsZU5hbWUSJQoOZmlsZV9leHRlbnNpb24YAyABKAlSDWZpbGVFeHRlbnNpb24SGwoJ'
    'ZmlsZV9zaXplGAQgASgNUghmaWxlU2l6ZRIpChBmaWxlX2Rlc2NyaXB0aW9uGAUgASgJUg9maW'
    'xlRGVzY3JpcHRpb24SHwoLc3RhcnRfaW5kZXgYBiABKA1SCnN0YXJ0SW5kZXgSIwoNbWVzc2Fn'
    'ZV9jb3VudBgHIAEoDVIMbWVzc2FnZUNvdW50EiYKD2RhdGFfY2h1bmtfc2l6ZRgIIAEoDVINZG'
    'F0YUNodW5rU2l6ZQ==');

@$core.Deprecated('Use chatFileDataDescriptor instead')
const ChatFileData$json = {
  '1': 'ChatFileData',
  '2': [
    {'1': 'file_id', '3': 1, '4': 1, '5': 4, '10': 'fileId'},
    {'1': 'start_index', '3': 2, '4': 1, '5': 13, '10': 'startIndex'},
    {'1': 'message_count', '3': 3, '4': 1, '5': 13, '10': 'messageCount'},
    {'1': 'data', '3': 4, '4': 1, '5': 12, '10': 'data'},
  ],
};

/// Descriptor for `ChatFileData`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List chatFileDataDescriptor = $convert.base64Decode(
    'CgxDaGF0RmlsZURhdGESFwoHZmlsZV9pZBgBIAEoBFIGZmlsZUlkEh8KC3N0YXJ0X2luZGV4GA'
    'IgASgNUgpzdGFydEluZGV4EiMKDW1lc3NhZ2VfY291bnQYAyABKA1SDG1lc3NhZ2VDb3VudBIS'
    'CgRkYXRhGAQgASgMUgRkYXRh');

