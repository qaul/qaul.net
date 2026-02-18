// This is a generated file - do not edit.
//
// Generated from services/feed/feed_net.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports
// ignore_for_file: unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use feedContainerDescriptor instead')
const FeedContainer$json = {
  '1': 'FeedContainer',
  '2': [
    {'1': 'signature', '3': 1, '4': 1, '5': 12, '10': 'signature'},
    {'1': 'message', '3': 2, '4': 1, '5': 12, '10': 'message'},
  ],
};

/// Descriptor for `FeedContainer`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List feedContainerDescriptor = $convert.base64Decode(
    'Cg1GZWVkQ29udGFpbmVyEhwKCXNpZ25hdHVyZRgBIAEoDFIJc2lnbmF0dXJlEhgKB21lc3NhZ2'
    'UYAiABKAxSB21lc3NhZ2U=');

@$core.Deprecated('Use feedMessageContentDescriptor instead')
const FeedMessageContent$json = {
  '1': 'FeedMessageContent',
  '2': [
    {'1': 'sender', '3': 1, '4': 1, '5': 12, '10': 'sender'},
    {'1': 'content', '3': 2, '4': 1, '5': 9, '10': 'content'},
    {'1': 'time', '3': 3, '4': 1, '5': 4, '10': 'time'},
  ],
};

/// Descriptor for `FeedMessageContent`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List feedMessageContentDescriptor = $convert.base64Decode(
    'ChJGZWVkTWVzc2FnZUNvbnRlbnQSFgoGc2VuZGVyGAEgASgMUgZzZW5kZXISGAoHY29udGVudB'
    'gCIAEoCVIHY29udGVudBISCgR0aW1lGAMgASgEUgR0aW1l');
