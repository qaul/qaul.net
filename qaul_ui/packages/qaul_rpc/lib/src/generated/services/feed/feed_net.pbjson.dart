///
//  Generated code. Do not modify.
//  source: services/feed/feed_net.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,deprecated_member_use_from_same_package,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use feedContainerDescriptor instead')
const FeedContainer$json = const {
  '1': 'FeedContainer',
  '2': const [
    const {'1': 'signature', '3': 1, '4': 1, '5': 12, '10': 'signature'},
    const {'1': 'message', '3': 2, '4': 1, '5': 12, '10': 'message'},
  ],
};

/// Descriptor for `FeedContainer`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List feedContainerDescriptor = $convert.base64Decode('Cg1GZWVkQ29udGFpbmVyEhwKCXNpZ25hdHVyZRgBIAEoDFIJc2lnbmF0dXJlEhgKB21lc3NhZ2UYAiABKAxSB21lc3NhZ2U=');
@$core.Deprecated('Use feedMessageContentDescriptor instead')
const FeedMessageContent$json = const {
  '1': 'FeedMessageContent',
  '2': const [
    const {'1': 'sender', '3': 1, '4': 1, '5': 12, '10': 'sender'},
    const {'1': 'content', '3': 2, '4': 1, '5': 9, '10': 'content'},
    const {'1': 'time', '3': 3, '4': 1, '5': 4, '10': 'time'},
  ],
};

/// Descriptor for `FeedMessageContent`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List feedMessageContentDescriptor = $convert.base64Decode('ChJGZWVkTWVzc2FnZUNvbnRlbnQSFgoGc2VuZGVyGAEgASgMUgZzZW5kZXISGAoHY29udGVudBgCIAEoCVIHY29udGVudBISCgR0aW1lGAMgASgEUgR0aW1l');
