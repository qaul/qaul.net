///
//  Generated code. Do not modify.
//  source: services/feed/feed.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields,deprecated_member_use_from_same_package

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use feedDescriptor instead')
const Feed$json = const {
  '1': 'Feed',
  '2': const [
    const {'1': 'received', '3': 1, '4': 1, '5': 11, '6': '.qaul.rpc.feed.FeedMessageList', '9': 0, '10': 'received'},
    const {'1': 'send', '3': 2, '4': 1, '5': 11, '6': '.qaul.rpc.feed.SendMessage', '9': 0, '10': 'send'},
    const {'1': 'request', '3': 3, '4': 1, '5': 11, '6': '.qaul.rpc.feed.FeedMessageRequest', '9': 0, '10': 'request'},
  ],
  '8': const [
    const {'1': 'message'},
  ],
};

/// Descriptor for `Feed`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List feedDescriptor = $convert.base64Decode('CgRGZWVkEjwKCHJlY2VpdmVkGAEgASgLMh4ucWF1bC5ycGMuZmVlZC5GZWVkTWVzc2FnZUxpc3RIAFIIcmVjZWl2ZWQSMAoEc2VuZBgCIAEoCzIaLnFhdWwucnBjLmZlZWQuU2VuZE1lc3NhZ2VIAFIEc2VuZBI9CgdyZXF1ZXN0GAMgASgLMiEucWF1bC5ycGMuZmVlZC5GZWVkTWVzc2FnZVJlcXVlc3RIAFIHcmVxdWVzdEIJCgdtZXNzYWdl');
@$core.Deprecated('Use feedMessageRequestDescriptor instead')
const FeedMessageRequest$json = const {
  '1': 'FeedMessageRequest',
  '2': const [
    const {'1': 'last_received', '3': 1, '4': 1, '5': 12, '10': 'lastReceived'},
  ],
};

/// Descriptor for `FeedMessageRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List feedMessageRequestDescriptor = $convert.base64Decode('ChJGZWVkTWVzc2FnZVJlcXVlc3QSIwoNbGFzdF9yZWNlaXZlZBgBIAEoDFIMbGFzdFJlY2VpdmVk');
@$core.Deprecated('Use feedMessageListDescriptor instead')
const FeedMessageList$json = const {
  '1': 'FeedMessageList',
  '2': const [
    const {'1': 'feed_message', '3': 1, '4': 3, '5': 11, '6': '.qaul.rpc.feed.FeedMessage', '10': 'feedMessage'},
  ],
};

/// Descriptor for `FeedMessageList`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List feedMessageListDescriptor = $convert.base64Decode('Cg9GZWVkTWVzc2FnZUxpc3QSPQoMZmVlZF9tZXNzYWdlGAEgAygLMhoucWF1bC5ycGMuZmVlZC5GZWVkTWVzc2FnZVILZmVlZE1lc3NhZ2U=');
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
@$core.Deprecated('Use sendMessageDescriptor instead')
const SendMessage$json = const {
  '1': 'SendMessage',
  '2': const [
    const {'1': 'content', '3': 1, '4': 1, '5': 9, '10': 'content'},
  ],
};

/// Descriptor for `SendMessage`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sendMessageDescriptor = $convert.base64Decode('CgtTZW5kTWVzc2FnZRIYCgdjb250ZW50GAEgASgJUgdjb250ZW50');
