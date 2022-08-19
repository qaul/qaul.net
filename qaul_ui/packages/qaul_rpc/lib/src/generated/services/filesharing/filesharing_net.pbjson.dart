///
//  Generated code. Do not modify.
//  source: services/filesharing/filesharing_net.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,deprecated_member_use_from_same_package,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use fileSharingContainerDescriptor instead')
const FileSharingContainer$json = const {
  '1': 'FileSharingContainer',
  '2': const [
    const {'1': 'file_info', '3': 1, '4': 1, '5': 11, '6': '.qaul.net.filesharing.FileSharingInfo', '9': 0, '10': 'fileInfo'},
    const {'1': 'file_data', '3': 2, '4': 1, '5': 11, '6': '.qaul.net.filesharing.FileSharingData', '9': 0, '10': 'fileData'},
  ],
  '8': const [
    const {'1': 'message'},
  ],
};

/// Descriptor for `FileSharingContainer`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fileSharingContainerDescriptor = $convert.base64Decode('ChRGaWxlU2hhcmluZ0NvbnRhaW5lchJECglmaWxlX2luZm8YASABKAsyJS5xYXVsLm5ldC5maWxlc2hhcmluZy5GaWxlU2hhcmluZ0luZm9IAFIIZmlsZUluZm8SRAoJZmlsZV9kYXRhGAIgASgLMiUucWF1bC5uZXQuZmlsZXNoYXJpbmcuRmlsZVNoYXJpbmdEYXRhSABSCGZpbGVEYXRhQgkKB21lc3NhZ2U=');
@$core.Deprecated('Use fileSharingInfoDescriptor instead')
const FileSharingInfo$json = const {
  '1': 'FileSharingInfo',
  '2': const [
    const {'1': 'file_id', '3': 1, '4': 1, '5': 4, '10': 'fileId'},
    const {'1': 'file_name', '3': 2, '4': 1, '5': 9, '10': 'fileName'},
    const {'1': 'file_extension', '3': 3, '4': 1, '5': 9, '10': 'fileExtension'},
    const {'1': 'file_size', '3': 4, '4': 1, '5': 13, '10': 'fileSize'},
    const {'1': 'file_descr', '3': 5, '4': 1, '5': 9, '10': 'fileDescr'},
    const {'1': 'start_index', '3': 6, '4': 1, '5': 13, '10': 'startIndex'},
    const {'1': 'message_count', '3': 7, '4': 1, '5': 13, '10': 'messageCount'},
  ],
};

/// Descriptor for `FileSharingInfo`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fileSharingInfoDescriptor = $convert.base64Decode('Cg9GaWxlU2hhcmluZ0luZm8SFwoHZmlsZV9pZBgBIAEoBFIGZmlsZUlkEhsKCWZpbGVfbmFtZRgCIAEoCVIIZmlsZU5hbWUSJQoOZmlsZV9leHRlbnNpb24YAyABKAlSDWZpbGVFeHRlbnNpb24SGwoJZmlsZV9zaXplGAQgASgNUghmaWxlU2l6ZRIdCgpmaWxlX2Rlc2NyGAUgASgJUglmaWxlRGVzY3ISHwoLc3RhcnRfaW5kZXgYBiABKA1SCnN0YXJ0SW5kZXgSIwoNbWVzc2FnZV9jb3VudBgHIAEoDVIMbWVzc2FnZUNvdW50');
@$core.Deprecated('Use fileSharingDataDescriptor instead')
const FileSharingData$json = const {
  '1': 'FileSharingData',
  '2': const [
    const {'1': 'start_index', '3': 1, '4': 1, '5': 13, '10': 'startIndex'},
    const {'1': 'message_count', '3': 2, '4': 1, '5': 13, '10': 'messageCount'},
    const {'1': 'data', '3': 3, '4': 1, '5': 12, '10': 'data'},
  ],
};

/// Descriptor for `FileSharingData`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fileSharingDataDescriptor = $convert.base64Decode('Cg9GaWxlU2hhcmluZ0RhdGESHwoLc3RhcnRfaW5kZXgYASABKA1SCnN0YXJ0SW5kZXgSIwoNbWVzc2FnZV9jb3VudBgCIAEoDVIMbWVzc2FnZUNvdW50EhIKBGRhdGEYAyABKAxSBGRhdGE=');
