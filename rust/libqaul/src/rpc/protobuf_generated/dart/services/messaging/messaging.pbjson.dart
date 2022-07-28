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
    const {'1': 'data', '3': 3, '4': 3, '5': 11, '6': '.qaul.net.messaging.Data', '10': 'data'},
  ],
};

/// Descriptor for `Envelope`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List envelopeDescriptor = $convert.base64Decode('CghFbnZlbG9wZRIbCglzZW5kZXJfaWQYASABKAxSCHNlbmRlcklkEh8KC3JlY2VpdmVyX2lkGAIgASgMUgpyZWNlaXZlcklkEiwKBGRhdGEYAyADKAsyGC5xYXVsLm5ldC5tZXNzYWdpbmcuRGF0YVIEZGF0YQ==');
@$core.Deprecated('Use dataDescriptor instead')
const Data$json = const {
  '1': 'Data',
  '2': const [
    const {'1': 'nonce', '3': 1, '4': 1, '5': 4, '10': 'nonce'},
    const {'1': 'data', '3': 2, '4': 1, '5': 12, '10': 'data'},
  ],
};

/// Descriptor for `Data`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dataDescriptor = $convert.base64Decode('CgREYXRhEhQKBW5vbmNlGAEgASgEUgVub25jZRISCgRkYXRhGAIgASgMUgRkYXRh');
@$core.Deprecated('Use messagingDescriptor instead')
const Messaging$json = const {
  '1': 'Messaging',
  '2': const [
    const {'1': 'confirmation_message', '3': 1, '4': 1, '5': 11, '6': '.qaul.net.messaging.Confirmation', '9': 0, '10': 'confirmationMessage'},
    const {'1': 'crypto_service', '3': 2, '4': 1, '5': 11, '6': '.qaul.net.messaging.CryptoService', '9': 0, '10': 'cryptoService'},
    const {'1': 'chat_message', '3': 3, '4': 1, '5': 11, '6': '.qaul.net.messaging.ChatMessage', '9': 0, '10': 'chatMessage'},
    const {'1': 'file_message', '3': 4, '4': 1, '5': 11, '6': '.qaul.net.messaging.FileMessage', '9': 0, '10': 'fileMessage'},
    const {'1': 'group_chat_message', '3': 5, '4': 1, '5': 11, '6': '.qaul.net.messaging.GroupChatMessage', '9': 0, '10': 'groupChatMessage'},
    const {'1': 'rtc_message', '3': 6, '4': 1, '5': 11, '6': '.qaul.net.messaging.RtcMessage', '9': 0, '10': 'rtcMessage'},
  ],
  '8': const [
    const {'1': 'message'},
  ],
};

/// Descriptor for `Messaging`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List messagingDescriptor = $convert.base64Decode('CglNZXNzYWdpbmcSVQoUY29uZmlybWF0aW9uX21lc3NhZ2UYASABKAsyIC5xYXVsLm5ldC5tZXNzYWdpbmcuQ29uZmlybWF0aW9uSABSE2NvbmZpcm1hdGlvbk1lc3NhZ2USSgoOY3J5cHRvX3NlcnZpY2UYAiABKAsyIS5xYXVsLm5ldC5tZXNzYWdpbmcuQ3J5cHRvU2VydmljZUgAUg1jcnlwdG9TZXJ2aWNlEkQKDGNoYXRfbWVzc2FnZRgDIAEoCzIfLnFhdWwubmV0Lm1lc3NhZ2luZy5DaGF0TWVzc2FnZUgAUgtjaGF0TWVzc2FnZRJECgxmaWxlX21lc3NhZ2UYBCABKAsyHy5xYXVsLm5ldC5tZXNzYWdpbmcuRmlsZU1lc3NhZ2VIAFILZmlsZU1lc3NhZ2USVAoSZ3JvdXBfY2hhdF9tZXNzYWdlGAUgASgLMiQucWF1bC5uZXQubWVzc2FnaW5nLkdyb3VwQ2hhdE1lc3NhZ2VIAFIQZ3JvdXBDaGF0TWVzc2FnZRJBCgtydGNfbWVzc2FnZRgGIAEoCzIeLnFhdWwubmV0Lm1lc3NhZ2luZy5SdGNNZXNzYWdlSABSCnJ0Y01lc3NhZ2VCCQoHbWVzc2FnZQ==');
@$core.Deprecated('Use cryptoServiceDescriptor instead')
const CryptoService$json = const {
  '1': 'CryptoService',
};

/// Descriptor for `CryptoService`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List cryptoServiceDescriptor = $convert.base64Decode('Cg1DcnlwdG9TZXJ2aWNl');
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
@$core.Deprecated('Use fileMessageDescriptor instead')
const FileMessage$json = const {
  '1': 'FileMessage',
  '2': const [
    const {'1': 'content', '3': 1, '4': 1, '5': 12, '10': 'content'},
  ],
};

/// Descriptor for `FileMessage`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fileMessageDescriptor = $convert.base64Decode('CgtGaWxlTWVzc2FnZRIYCgdjb250ZW50GAEgASgMUgdjb250ZW50');
@$core.Deprecated('Use groupChatMessageDescriptor instead')
const GroupChatMessage$json = const {
  '1': 'GroupChatMessage',
  '2': const [
    const {'1': 'content', '3': 1, '4': 1, '5': 12, '10': 'content'},
  ],
};

/// Descriptor for `GroupChatMessage`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupChatMessageDescriptor = $convert.base64Decode('ChBHcm91cENoYXRNZXNzYWdlEhgKB2NvbnRlbnQYASABKAxSB2NvbnRlbnQ=');
@$core.Deprecated('Use rtcMessageDescriptor instead')
const RtcMessage$json = const {
  '1': 'RtcMessage',
  '2': const [
    const {'1': 'content', '3': 1, '4': 1, '5': 12, '10': 'content'},
  ],
};

/// Descriptor for `RtcMessage`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List rtcMessageDescriptor = $convert.base64Decode('CgpSdGNNZXNzYWdlEhgKB2NvbnRlbnQYASABKAxSB2NvbnRlbnQ=');
