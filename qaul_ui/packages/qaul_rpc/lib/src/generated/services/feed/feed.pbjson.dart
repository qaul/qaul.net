//
//  Generated code. Do not modify.
//  source: services/feed/feed.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use feedDescriptor instead')
const Feed$json = {
  '1': 'Feed',
  '2': [
    {'1': 'received', '3': 1, '4': 1, '5': 11, '6': '.qaul.rpc.feed.FeedMessageList', '9': 0, '10': 'received'},
    {'1': 'send', '3': 2, '4': 1, '5': 11, '6': '.qaul.rpc.feed.SendMessage', '9': 0, '10': 'send'},
    {'1': 'request', '3': 3, '4': 1, '5': 11, '6': '.qaul.rpc.feed.FeedMessageRequest', '9': 0, '10': 'request'},
  ],
  '8': [
    {'1': 'message'},
  ],
};

/// Descriptor for `Feed`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List feedDescriptor = $convert.base64Decode(
    'CgRGZWVkEjwKCHJlY2VpdmVkGAEgASgLMh4ucWF1bC5ycGMuZmVlZC5GZWVkTWVzc2FnZUxpc3'
    'RIAFIIcmVjZWl2ZWQSMAoEc2VuZBgCIAEoCzIaLnFhdWwucnBjLmZlZWQuU2VuZE1lc3NhZ2VI'
    'AFIEc2VuZBI9CgdyZXF1ZXN0GAMgASgLMiEucWF1bC5ycGMuZmVlZC5GZWVkTWVzc2FnZVJlcX'
    'Vlc3RIAFIHcmVxdWVzdEIJCgdtZXNzYWdl');

@$core.Deprecated('Use feedMessageRequestDescriptor instead')
const FeedMessageRequest$json = {
  '1': 'FeedMessageRequest',
  '2': [
    {'1': 'last_received', '3': 1, '4': 1, '5': 12, '10': 'lastReceived'},
    {'1': 'last_index', '3': 2, '4': 1, '5': 4, '10': 'lastIndex'},
  ],
};

/// Descriptor for `FeedMessageRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List feedMessageRequestDescriptor = $convert.base64Decode(
    'ChJGZWVkTWVzc2FnZVJlcXVlc3QSIwoNbGFzdF9yZWNlaXZlZBgBIAEoDFIMbGFzdFJlY2Vpdm'
    'VkEh0KCmxhc3RfaW5kZXgYAiABKARSCWxhc3RJbmRleA==');

@$core.Deprecated('Use feedMessageListDescriptor instead')
const FeedMessageList$json = {
  '1': 'FeedMessageList',
  '2': [
    {'1': 'feed_message', '3': 1, '4': 3, '5': 11, '6': '.qaul.rpc.feed.FeedMessage', '10': 'feedMessage'},
  ],
};

/// Descriptor for `FeedMessageList`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List feedMessageListDescriptor = $convert.base64Decode(
    'Cg9GZWVkTWVzc2FnZUxpc3QSPQoMZmVlZF9tZXNzYWdlGAEgAygLMhoucWF1bC5ycGMuZmVlZC'
    '5GZWVkTWVzc2FnZVILZmVlZE1lc3NhZ2U=');

@$core.Deprecated('Use feedMessageDescriptor instead')
const FeedMessage$json = {
  '1': 'FeedMessage',
  '2': [
    {'1': 'sender_id', '3': 1, '4': 1, '5': 12, '10': 'senderId'},
    {'1': 'sender_id_base58', '3': 2, '4': 1, '5': 9, '10': 'senderIdBase58'},
    {'1': 'message_id', '3': 3, '4': 1, '5': 12, '10': 'messageId'},
    {'1': 'message_id_base58', '3': 4, '4': 1, '5': 9, '10': 'messageIdBase58'},
    {'1': 'time_sent', '3': 5, '4': 1, '5': 9, '10': 'timeSent'},
    {'1': 'timestamp_sent', '3': 9, '4': 1, '5': 4, '10': 'timestampSent'},
    {'1': 'time_received', '3': 6, '4': 1, '5': 9, '10': 'timeReceived'},
    {'1': 'timestamp_received', '3': 10, '4': 1, '5': 4, '10': 'timestampReceived'},
    {'1': 'content', '3': 7, '4': 1, '5': 9, '10': 'content'},
    {'1': 'index', '3': 8, '4': 1, '5': 4, '10': 'index'},
  ],
};

/// Descriptor for `FeedMessage`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List feedMessageDescriptor = $convert.base64Decode(
    'CgtGZWVkTWVzc2FnZRIbCglzZW5kZXJfaWQYASABKAxSCHNlbmRlcklkEigKEHNlbmRlcl9pZF'
    '9iYXNlNTgYAiABKAlSDnNlbmRlcklkQmFzZTU4Eh0KCm1lc3NhZ2VfaWQYAyABKAxSCW1lc3Nh'
    'Z2VJZBIqChFtZXNzYWdlX2lkX2Jhc2U1OBgEIAEoCVIPbWVzc2FnZUlkQmFzZTU4EhsKCXRpbW'
    'Vfc2VudBgFIAEoCVIIdGltZVNlbnQSJQoOdGltZXN0YW1wX3NlbnQYCSABKARSDXRpbWVzdGFt'
    'cFNlbnQSIwoNdGltZV9yZWNlaXZlZBgGIAEoCVIMdGltZVJlY2VpdmVkEi0KEnRpbWVzdGFtcF'
    '9yZWNlaXZlZBgKIAEoBFIRdGltZXN0YW1wUmVjZWl2ZWQSGAoHY29udGVudBgHIAEoCVIHY29u'
    'dGVudBIUCgVpbmRleBgIIAEoBFIFaW5kZXg=');

@$core.Deprecated('Use sendMessageDescriptor instead')
const SendMessage$json = {
  '1': 'SendMessage',
  '2': [
    {'1': 'content', '3': 1, '4': 1, '5': 9, '10': 'content'},
  ],
};

/// Descriptor for `SendMessage`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sendMessageDescriptor = $convert.base64Decode(
    'CgtTZW5kTWVzc2FnZRIYCgdjb250ZW50GAEgASgJUgdjb250ZW50');

