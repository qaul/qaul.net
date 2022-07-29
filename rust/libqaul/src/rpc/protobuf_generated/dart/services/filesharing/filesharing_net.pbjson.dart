///
//  Generated code. Do not modify.
//  source: services/filesharing/filesharing_net.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields,deprecated_member_use_from_same_package

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use fileSharingContainerDescriptor instead')
const FileSharingContainer$json = const {
  '1': 'FileSharingContainer',
  '2': const [
    const {'1': 'file_info', '3': 1, '4': 1, '5': 11, '6': '.qaul.net.filesharing.FileSharingInfo', '9': 0, '10': 'fileInfo'},
    const {'1': 'file_data', '3': 2, '4': 1, '5': 11, '6': '.qaul.net.filesharing.FileSharingData', '9': 0, '10': 'fileData'},
    const {'1': 'confirmation', '3': 3, '4': 1, '5': 11, '6': '.qaul.net.filesharing.FileSharingConfirmation', '9': 0, '10': 'confirmation'},
    const {'1': 'confirmation_info', '3': 4, '4': 1, '5': 11, '6': '.qaul.net.filesharing.FileSharingConfirmationFileInfo', '9': 0, '10': 'confirmationInfo'},
    const {'1': 'completed', '3': 5, '4': 1, '5': 11, '6': '.qaul.net.filesharing.FileSharingCompleted', '9': 0, '10': 'completed'},
    const {'1': 'canceled', '3': 6, '4': 1, '5': 11, '6': '.qaul.net.filesharing.FileSharingCanceled', '9': 0, '10': 'canceled'},
  ],
  '8': const [
    const {'1': 'message'},
  ],
};

/// Descriptor for `FileSharingContainer`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fileSharingContainerDescriptor = $convert.base64Decode('ChRGaWxlU2hhcmluZ0NvbnRhaW5lchJECglmaWxlX2luZm8YASABKAsyJS5xYXVsLm5ldC5maWxlc2hhcmluZy5GaWxlU2hhcmluZ0luZm9IAFIIZmlsZUluZm8SRAoJZmlsZV9kYXRhGAIgASgLMiUucWF1bC5uZXQuZmlsZXNoYXJpbmcuRmlsZVNoYXJpbmdEYXRhSABSCGZpbGVEYXRhElMKDGNvbmZpcm1hdGlvbhgDIAEoCzItLnFhdWwubmV0LmZpbGVzaGFyaW5nLkZpbGVTaGFyaW5nQ29uZmlybWF0aW9uSABSDGNvbmZpcm1hdGlvbhJkChFjb25maXJtYXRpb25faW5mbxgEIAEoCzI1LnFhdWwubmV0LmZpbGVzaGFyaW5nLkZpbGVTaGFyaW5nQ29uZmlybWF0aW9uRmlsZUluZm9IAFIQY29uZmlybWF0aW9uSW5mbxJKCgljb21wbGV0ZWQYBSABKAsyKi5xYXVsLm5ldC5maWxlc2hhcmluZy5GaWxlU2hhcmluZ0NvbXBsZXRlZEgAUgljb21wbGV0ZWQSRwoIY2FuY2VsZWQYBiABKAsyKS5xYXVsLm5ldC5maWxlc2hhcmluZy5GaWxlU2hhcmluZ0NhbmNlbGVkSABSCGNhbmNlbGVkQgkKB21lc3NhZ2U=');
@$core.Deprecated('Use fileSharingInfoDescriptor instead')
const FileSharingInfo$json = const {
  '1': 'FileSharingInfo',
  '2': const [
    const {'1': 'file_name', '3': 1, '4': 1, '5': 9, '10': 'fileName'},
    const {'1': 'file_extension', '3': 2, '4': 1, '5': 9, '10': 'fileExtension'},
    const {'1': 'file_size', '3': 3, '4': 1, '5': 13, '10': 'fileSize'},
    const {'1': 'file_descr', '3': 4, '4': 1, '5': 9, '10': 'fileDescr'},
    const {'1': 'size_per_package', '3': 5, '4': 1, '5': 13, '10': 'sizePerPackage'},
    const {'1': 'file_id', '3': 6, '4': 1, '5': 4, '10': 'fileId'},
  ],
};

/// Descriptor for `FileSharingInfo`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fileSharingInfoDescriptor = $convert.base64Decode('Cg9GaWxlU2hhcmluZ0luZm8SGwoJZmlsZV9uYW1lGAEgASgJUghmaWxlTmFtZRIlCg5maWxlX2V4dGVuc2lvbhgCIAEoCVINZmlsZUV4dGVuc2lvbhIbCglmaWxlX3NpemUYAyABKA1SCGZpbGVTaXplEh0KCmZpbGVfZGVzY3IYBCABKAlSCWZpbGVEZXNjchIoChBzaXplX3Blcl9wYWNrYWdlGAUgASgNUg5zaXplUGVyUGFja2FnZRIXCgdmaWxlX2lkGAYgASgEUgZmaWxlSWQ=');
@$core.Deprecated('Use fileSharingDataDescriptor instead')
const FileSharingData$json = const {
  '1': 'FileSharingData',
  '2': const [
    const {'1': 'file_id', '3': 1, '4': 1, '5': 4, '10': 'fileId'},
    const {'1': 'sequence', '3': 2, '4': 1, '5': 13, '10': 'sequence'},
    const {'1': 'file_size', '3': 3, '4': 1, '5': 13, '10': 'fileSize'},
    const {'1': 'size_per_package', '3': 4, '4': 1, '5': 13, '10': 'sizePerPackage'},
    const {'1': 'data', '3': 6, '4': 1, '5': 12, '10': 'data'},
  ],
};

/// Descriptor for `FileSharingData`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fileSharingDataDescriptor = $convert.base64Decode('Cg9GaWxlU2hhcmluZ0RhdGESFwoHZmlsZV9pZBgBIAEoBFIGZmlsZUlkEhoKCHNlcXVlbmNlGAIgASgNUghzZXF1ZW5jZRIbCglmaWxlX3NpemUYAyABKA1SCGZpbGVTaXplEigKEHNpemVfcGVyX3BhY2thZ2UYBCABKA1SDnNpemVQZXJQYWNrYWdlEhIKBGRhdGEYBiABKAxSBGRhdGE=');
@$core.Deprecated('Use fileSharingConfirmationFileInfoDescriptor instead')
const FileSharingConfirmationFileInfo$json = const {
  '1': 'FileSharingConfirmationFileInfo',
  '2': const [
    const {'1': 'file_id', '3': 1, '4': 1, '5': 4, '10': 'fileId'},
  ],
};

/// Descriptor for `FileSharingConfirmationFileInfo`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fileSharingConfirmationFileInfoDescriptor = $convert.base64Decode('Ch9GaWxlU2hhcmluZ0NvbmZpcm1hdGlvbkZpbGVJbmZvEhcKB2ZpbGVfaWQYASABKARSBmZpbGVJZA==');
@$core.Deprecated('Use fileSharingConfirmationDescriptor instead')
const FileSharingConfirmation$json = const {
  '1': 'FileSharingConfirmation',
  '2': const [
    const {'1': 'file_id', '3': 1, '4': 1, '5': 4, '10': 'fileId'},
    const {'1': 'sequence', '3': 2, '4': 1, '5': 13, '10': 'sequence'},
  ],
};

/// Descriptor for `FileSharingConfirmation`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fileSharingConfirmationDescriptor = $convert.base64Decode('ChdGaWxlU2hhcmluZ0NvbmZpcm1hdGlvbhIXCgdmaWxlX2lkGAEgASgEUgZmaWxlSWQSGgoIc2VxdWVuY2UYAiABKA1SCHNlcXVlbmNl');
@$core.Deprecated('Use fileSharingCompletedDescriptor instead')
const FileSharingCompleted$json = const {
  '1': 'FileSharingCompleted',
  '2': const [
    const {'1': 'file_id', '3': 1, '4': 1, '5': 4, '10': 'fileId'},
  ],
};

/// Descriptor for `FileSharingCompleted`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fileSharingCompletedDescriptor = $convert.base64Decode('ChRGaWxlU2hhcmluZ0NvbXBsZXRlZBIXCgdmaWxlX2lkGAEgASgEUgZmaWxlSWQ=');
@$core.Deprecated('Use fileSharingCanceledDescriptor instead')
const FileSharingCanceled$json = const {
  '1': 'FileSharingCanceled',
  '2': const [
    const {'1': 'file_id', '3': 1, '4': 1, '5': 4, '10': 'fileId'},
  ],
};

/// Descriptor for `FileSharingCanceled`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fileSharingCanceledDescriptor = $convert.base64Decode('ChNGaWxlU2hhcmluZ0NhbmNlbGVkEhcKB2ZpbGVfaWQYASABKARSBmZpbGVJZA==');
