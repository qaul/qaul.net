///
//  Generated code. Do not modify.
//  source: services/messaging/messaging.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields,deprecated_member_use_from_same_package

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use containerDescriptor instead')
const Container$json = const {
  '1': 'Container',
  '2': const [
    const {'1': 'signature', '3': 1, '4': 1, '5': 12, '10': 'signature'},
    const {'1': 'envelope', '3': 2, '4': 1, '5': 11, '6': '.qaul.net.messaging.Envelope', '10': 'envelope'},
  ],
};

/// Descriptor for `Container`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List containerDescriptor = $convert.base64Decode('CglDb250YWluZXISHAoJc2lnbmF0dXJlGAEgASgMUglzaWduYXR1cmUSOAoIZW52ZWxvcGUYAiABKAsyHC5xYXVsLm5ldC5tZXNzYWdpbmcuRW52ZWxvcGVSCGVudmVsb3Bl');
@$core.Deprecated('Use envelopeDescriptor instead')
const Envelope$json = const {
  '1': 'Envelope',
  '2': const [
    const {'1': 'sender_id', '3': 1, '4': 1, '5': 12, '10': 'senderId'},
    const {'1': 'receiver_id', '3': 2, '4': 1, '5': 12, '10': 'receiverId'},
    const {'1': 'data', '3': 3, '4': 1, '5': 12, '10': 'data'},
  ],
};

/// Descriptor for `Envelope`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List envelopeDescriptor = $convert.base64Decode('CghFbnZlbG9wZRIbCglzZW5kZXJfaWQYASABKAxSCHNlbmRlcklkEh8KC3JlY2VpdmVyX2lkGAIgASgMUgpyZWNlaXZlcklkEhIKBGRhdGEYAyABKAxSBGRhdGE=');
@$core.Deprecated('Use confirmationDescriptor instead')
const Confirmation$json = const {
  '1': 'Confirmation',
  '2': const [
    const {'1': 'message_id', '3': 1, '4': 1, '5': 12, '10': 'messageId'},
    const {'1': 'received_at', '3': 2, '4': 1, '5': 4, '10': 'receivedAt'},
  ],
};

/// Descriptor for `Confirmation`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List confirmationDescriptor = $convert.base64Decode('CgxDb25maXJtYXRpb24SHQoKbWVzc2FnZV9pZBgBIAEoDFIJbWVzc2FnZUlkEh8KC3JlY2VpdmVkX2F0GAIgASgEUgpyZWNlaXZlZEF0');
@$core.Deprecated('Use chatMessageDescriptor instead')
const ChatMessage$json = const {
  '1': 'ChatMessage',
  '2': const [
    const {'1': 'group', '3': 1, '4': 1, '5': 8, '10': 'group'},
    const {'1': 'conversation_id', '3': 2, '4': 1, '5': 12, '10': 'conversationId'},
    const {'1': 'sent_at', '3': 3, '4': 1, '5': 4, '10': 'sentAt'},
    const {'1': 'content', '3': 4, '4': 1, '5': 9, '10': 'content'},
  ],
};

/// Descriptor for `ChatMessage`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List chatMessageDescriptor = $convert.base64Decode('CgtDaGF0TWVzc2FnZRIUCgVncm91cBgBIAEoCFIFZ3JvdXASJwoPY29udmVyc2F0aW9uX2lkGAIgASgMUg5jb252ZXJzYXRpb25JZBIXCgdzZW50X2F0GAMgASgEUgZzZW50QXQSGAoHY29udGVudBgEIAEoCVIHY29udGVudA==');
