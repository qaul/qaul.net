///
//  Generated code. Do not modify.
//  source: services/chat/chatfile_net.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,deprecated_member_use_from_same_package,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use chatFileContainerDescriptor instead')
const ChatFileContainer$json = const {
  '1': 'ChatFileContainer',
  '2': const [
    const {'1': 'file_info', '3': 1, '4': 1, '5': 11, '6': '.qaul.net.chatfile.ChatFileInfo', '9': 0, '10': 'fileInfo'},
    const {'1': 'file_data', '3': 2, '4': 1, '5': 11, '6': '.qaul.net.chatfile.ChatFileData', '9': 0, '10': 'fileData'},
  ],
  '8': const [
    const {'1': 'message'},
  ],
};

/// Descriptor for `ChatFileContainer`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List chatFileContainerDescriptor = $convert.base64Decode('ChFDaGF0RmlsZUNvbnRhaW5lchI+CglmaWxlX2luZm8YASABKAsyHy5xYXVsLm5ldC5jaGF0ZmlsZS5DaGF0RmlsZUluZm9IAFIIZmlsZUluZm8SPgoJZmlsZV9kYXRhGAIgASgLMh8ucWF1bC5uZXQuY2hhdGZpbGUuQ2hhdEZpbGVEYXRhSABSCGZpbGVEYXRhQgkKB21lc3NhZ2U=');
@$core.Deprecated('Use chatFileInfoDescriptor instead')
const ChatFileInfo$json = const {
  '1': 'ChatFileInfo',
  '2': const [
    const {'1': 'file_id', '3': 1, '4': 1, '5': 4, '10': 'fileId'},
    const {'1': 'file_name', '3': 2, '4': 1, '5': 9, '10': 'fileName'},
    const {'1': 'file_extension', '3': 3, '4': 1, '5': 9, '10': 'fileExtension'},
    const {'1': 'file_size', '3': 4, '4': 1, '5': 13, '10': 'fileSize'},
    const {'1': 'file_description', '3': 5, '4': 1, '5': 9, '10': 'fileDescription'},
    const {'1': 'start_index', '3': 6, '4': 1, '5': 13, '10': 'startIndex'},
    const {'1': 'message_count', '3': 7, '4': 1, '5': 13, '10': 'messageCount'},
    const {'1': 'data_chunk_size', '3': 8, '4': 1, '5': 13, '10': 'dataChunkSize'},
  ],
};

/// Descriptor for `ChatFileInfo`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List chatFileInfoDescriptor = $convert.base64Decode('CgxDaGF0RmlsZUluZm8SFwoHZmlsZV9pZBgBIAEoBFIGZmlsZUlkEhsKCWZpbGVfbmFtZRgCIAEoCVIIZmlsZU5hbWUSJQoOZmlsZV9leHRlbnNpb24YAyABKAlSDWZpbGVFeHRlbnNpb24SGwoJZmlsZV9zaXplGAQgASgNUghmaWxlU2l6ZRIpChBmaWxlX2Rlc2NyaXB0aW9uGAUgASgJUg9maWxlRGVzY3JpcHRpb24SHwoLc3RhcnRfaW5kZXgYBiABKA1SCnN0YXJ0SW5kZXgSIwoNbWVzc2FnZV9jb3VudBgHIAEoDVIMbWVzc2FnZUNvdW50EiYKD2RhdGFfY2h1bmtfc2l6ZRgIIAEoDVINZGF0YUNodW5rU2l6ZQ==');
@$core.Deprecated('Use chatFileDataDescriptor instead')
const ChatFileData$json = const {
  '1': 'ChatFileData',
  '2': const [
    const {'1': 'file_id', '3': 1, '4': 1, '5': 4, '10': 'fileId'},
    const {'1': 'start_index', '3': 2, '4': 1, '5': 13, '10': 'startIndex'},
    const {'1': 'message_count', '3': 3, '4': 1, '5': 13, '10': 'messageCount'},
    const {'1': 'data', '3': 4, '4': 1, '5': 12, '10': 'data'},
  ],
};

/// Descriptor for `ChatFileData`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List chatFileDataDescriptor = $convert.base64Decode('CgxDaGF0RmlsZURhdGESFwoHZmlsZV9pZBgBIAEoBFIGZmlsZUlkEh8KC3N0YXJ0X2luZGV4GAIgASgNUgpzdGFydEluZGV4EiMKDW1lc3NhZ2VfY291bnQYAyABKA1SDG1lc3NhZ2VDb3VudBISCgRkYXRhGAQgASgMUgRkYXRh');
