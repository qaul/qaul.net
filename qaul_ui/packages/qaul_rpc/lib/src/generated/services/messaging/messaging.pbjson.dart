//
//  Generated code. Do not modify.
//  source: services/messaging/messaging.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use cryptoStateDescriptor instead')
const CryptoState$json = {
  '1': 'CryptoState',
  '2': [
    {'1': 'NONE', '2': 0},
    {'1': 'HANDSHAKE', '2': 1},
    {'1': 'TRANSPORT', '2': 2},
  ],
};

/// Descriptor for `CryptoState`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List cryptoStateDescriptor = $convert.base64Decode(
    'CgtDcnlwdG9TdGF0ZRIICgROT05FEAASDQoJSEFORFNIQUtFEAESDQoJVFJBTlNQT1JUEAI=');

@$core.Deprecated('Use containerDescriptor instead')
const Container$json = {
  '1': 'Container',
  '2': [
    {'1': 'signature', '3': 1, '4': 1, '5': 12, '10': 'signature'},
    {'1': 'envelope', '3': 2, '4': 1, '5': 11, '6': '.qaul.net.messaging.Envelope', '10': 'envelope'},
  ],
};

/// Descriptor for `Container`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List containerDescriptor = $convert.base64Decode(
    'CglDb250YWluZXISHAoJc2lnbmF0dXJlGAEgASgMUglzaWduYXR1cmUSOAoIZW52ZWxvcGUYAi'
    'ABKAsyHC5xYXVsLm5ldC5tZXNzYWdpbmcuRW52ZWxvcGVSCGVudmVsb3Bl');

@$core.Deprecated('Use envelopeDescriptor instead')
const Envelope$json = {
  '1': 'Envelope',
  '2': [
    {'1': 'sender_id', '3': 1, '4': 1, '5': 12, '10': 'senderId'},
    {'1': 'receiver_id', '3': 2, '4': 1, '5': 12, '10': 'receiverId'},
    {'1': 'payload', '3': 3, '4': 1, '5': 12, '10': 'payload'},
  ],
};

/// Descriptor for `Envelope`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List envelopeDescriptor = $convert.base64Decode(
    'CghFbnZlbG9wZRIbCglzZW5kZXJfaWQYASABKAxSCHNlbmRlcklkEh8KC3JlY2VpdmVyX2lkGA'
    'IgASgMUgpyZWNlaXZlcklkEhgKB3BheWxvYWQYAyABKAxSB3BheWxvYWQ=');

@$core.Deprecated('Use envelopPayloadDescriptor instead')
const EnvelopPayload$json = {
  '1': 'EnvelopPayload',
  '2': [
    {'1': 'encrypted', '3': 1, '4': 1, '5': 11, '6': '.qaul.net.messaging.Encrypted', '9': 0, '10': 'encrypted'},
    {'1': 'dtn', '3': 2, '4': 1, '5': 12, '9': 0, '10': 'dtn'},
  ],
  '8': [
    {'1': 'payload'},
  ],
};

/// Descriptor for `EnvelopPayload`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List envelopPayloadDescriptor = $convert.base64Decode(
    'Cg5FbnZlbG9wUGF5bG9hZBI9CgllbmNyeXB0ZWQYASABKAsyHS5xYXVsLm5ldC5tZXNzYWdpbm'
    'cuRW5jcnlwdGVkSABSCWVuY3J5cHRlZBISCgNkdG4YAiABKAxIAFIDZHRuQgkKB3BheWxvYWQ=');

@$core.Deprecated('Use encryptedDescriptor instead')
const Encrypted$json = {
  '1': 'Encrypted',
  '2': [
    {'1': 'state', '3': 1, '4': 1, '5': 14, '6': '.qaul.net.messaging.CryptoState', '10': 'state'},
    {'1': 'session_id', '3': 2, '4': 1, '5': 13, '10': 'sessionId'},
    {'1': 'data', '3': 3, '4': 3, '5': 11, '6': '.qaul.net.messaging.Data', '10': 'data'},
  ],
};

/// Descriptor for `Encrypted`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List encryptedDescriptor = $convert.base64Decode(
    'CglFbmNyeXB0ZWQSNQoFc3RhdGUYASABKA4yHy5xYXVsLm5ldC5tZXNzYWdpbmcuQ3J5cHRvU3'
    'RhdGVSBXN0YXRlEh0KCnNlc3Npb25faWQYAiABKA1SCXNlc3Npb25JZBIsCgRkYXRhGAMgAygL'
    'MhgucWF1bC5uZXQubWVzc2FnaW5nLkRhdGFSBGRhdGE=');

@$core.Deprecated('Use dataDescriptor instead')
const Data$json = {
  '1': 'Data',
  '2': [
    {'1': 'nonce', '3': 1, '4': 1, '5': 4, '10': 'nonce'},
    {'1': 'data', '3': 2, '4': 1, '5': 12, '10': 'data'},
  ],
};

/// Descriptor for `Data`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dataDescriptor = $convert.base64Decode(
    'CgREYXRhEhQKBW5vbmNlGAEgASgEUgVub25jZRISCgRkYXRhGAIgASgMUgRkYXRh');

@$core.Deprecated('Use messagingDescriptor instead')
const Messaging$json = {
  '1': 'Messaging',
  '2': [
    {'1': 'confirmation_message', '3': 1, '4': 1, '5': 11, '6': '.qaul.net.messaging.Confirmation', '9': 0, '10': 'confirmationMessage'},
    {'1': 'dtn_response', '3': 2, '4': 1, '5': 11, '6': '.qaul.net.messaging.DtnResponse', '9': 0, '10': 'dtnResponse'},
    {'1': 'crypto_service', '3': 3, '4': 1, '5': 11, '6': '.qaul.net.messaging.CryptoService', '9': 0, '10': 'cryptoService'},
    {'1': 'rtc_stream_message', '3': 4, '4': 1, '5': 11, '6': '.qaul.net.messaging.RtcStreamMessage', '9': 0, '10': 'rtcStreamMessage'},
    {'1': 'group_invite_message', '3': 5, '4': 1, '5': 11, '6': '.qaul.net.messaging.GroupInviteMessage', '9': 0, '10': 'groupInviteMessage'},
    {'1': 'common_message', '3': 6, '4': 1, '5': 11, '6': '.qaul.net.messaging.CommonMessage', '9': 0, '10': 'commonMessage'},
  ],
  '8': [
    {'1': 'message'},
  ],
};

/// Descriptor for `Messaging`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List messagingDescriptor = $convert.base64Decode(
    'CglNZXNzYWdpbmcSVQoUY29uZmlybWF0aW9uX21lc3NhZ2UYASABKAsyIC5xYXVsLm5ldC5tZX'
    'NzYWdpbmcuQ29uZmlybWF0aW9uSABSE2NvbmZpcm1hdGlvbk1lc3NhZ2USRAoMZHRuX3Jlc3Bv'
    'bnNlGAIgASgLMh8ucWF1bC5uZXQubWVzc2FnaW5nLkR0blJlc3BvbnNlSABSC2R0blJlc3Bvbn'
    'NlEkoKDmNyeXB0b19zZXJ2aWNlGAMgASgLMiEucWF1bC5uZXQubWVzc2FnaW5nLkNyeXB0b1Nl'
    'cnZpY2VIAFINY3J5cHRvU2VydmljZRJUChJydGNfc3RyZWFtX21lc3NhZ2UYBCABKAsyJC5xYX'
    'VsLm5ldC5tZXNzYWdpbmcuUnRjU3RyZWFtTWVzc2FnZUgAUhBydGNTdHJlYW1NZXNzYWdlEloK'
    'FGdyb3VwX2ludml0ZV9tZXNzYWdlGAUgASgLMiYucWF1bC5uZXQubWVzc2FnaW5nLkdyb3VwSW'
    '52aXRlTWVzc2FnZUgAUhJncm91cEludml0ZU1lc3NhZ2USSgoOY29tbW9uX21lc3NhZ2UYBiAB'
    'KAsyIS5xYXVsLm5ldC5tZXNzYWdpbmcuQ29tbW9uTWVzc2FnZUgAUg1jb21tb25NZXNzYWdlQg'
    'kKB21lc3NhZ2U=');

@$core.Deprecated('Use confirmationDescriptor instead')
const Confirmation$json = {
  '1': 'Confirmation',
  '2': [
    {'1': 'signature', '3': 1, '4': 1, '5': 12, '10': 'signature'},
    {'1': 'received_at', '3': 2, '4': 1, '5': 4, '10': 'receivedAt'},
  ],
};

/// Descriptor for `Confirmation`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List confirmationDescriptor = $convert.base64Decode(
    'CgxDb25maXJtYXRpb24SHAoJc2lnbmF0dXJlGAEgASgMUglzaWduYXR1cmUSHwoLcmVjZWl2ZW'
    'RfYXQYAiABKARSCnJlY2VpdmVkQXQ=');

@$core.Deprecated('Use cryptoServiceDescriptor instead')
const CryptoService$json = {
  '1': 'CryptoService',
  '2': [
    {'1': 'content', '3': 1, '4': 1, '5': 12, '10': 'content'},
  ],
};

/// Descriptor for `CryptoService`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List cryptoServiceDescriptor = $convert.base64Decode(
    'Cg1DcnlwdG9TZXJ2aWNlEhgKB2NvbnRlbnQYASABKAxSB2NvbnRlbnQ=');

@$core.Deprecated('Use rtcStreamMessageDescriptor instead')
const RtcStreamMessage$json = {
  '1': 'RtcStreamMessage',
  '2': [
    {'1': 'content', '3': 1, '4': 1, '5': 12, '10': 'content'},
  ],
};

/// Descriptor for `RtcStreamMessage`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List rtcStreamMessageDescriptor = $convert.base64Decode(
    'ChBSdGNTdHJlYW1NZXNzYWdlEhgKB2NvbnRlbnQYASABKAxSB2NvbnRlbnQ=');

@$core.Deprecated('Use groupInviteMessageDescriptor instead')
const GroupInviteMessage$json = {
  '1': 'GroupInviteMessage',
  '2': [
    {'1': 'content', '3': 1, '4': 1, '5': 12, '10': 'content'},
  ],
};

/// Descriptor for `GroupInviteMessage`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupInviteMessageDescriptor = $convert.base64Decode(
    'ChJHcm91cEludml0ZU1lc3NhZ2USGAoHY29udGVudBgBIAEoDFIHY29udGVudA==');

@$core.Deprecated('Use commonMessageDescriptor instead')
const CommonMessage$json = {
  '1': 'CommonMessage',
  '2': [
    {'1': 'message_id', '3': 1, '4': 1, '5': 12, '10': 'messageId'},
    {'1': 'group_id', '3': 2, '4': 1, '5': 12, '10': 'groupId'},
    {'1': 'sent_at', '3': 3, '4': 1, '5': 4, '10': 'sentAt'},
    {'1': 'chat_message', '3': 4, '4': 1, '5': 11, '6': '.qaul.net.messaging.ChatMessage', '9': 0, '10': 'chatMessage'},
    {'1': 'file_message', '3': 5, '4': 1, '5': 11, '6': '.qaul.net.messaging.FileMessage', '9': 0, '10': 'fileMessage'},
    {'1': 'group_message', '3': 6, '4': 1, '5': 11, '6': '.qaul.net.messaging.GroupMessage', '9': 0, '10': 'groupMessage'},
    {'1': 'rtc_message', '3': 7, '4': 1, '5': 11, '6': '.qaul.net.messaging.RtcMessage', '9': 0, '10': 'rtcMessage'},
  ],
  '8': [
    {'1': 'payload'},
  ],
};

/// Descriptor for `CommonMessage`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List commonMessageDescriptor = $convert.base64Decode(
    'Cg1Db21tb25NZXNzYWdlEh0KCm1lc3NhZ2VfaWQYASABKAxSCW1lc3NhZ2VJZBIZCghncm91cF'
    '9pZBgCIAEoDFIHZ3JvdXBJZBIXCgdzZW50X2F0GAMgASgEUgZzZW50QXQSRAoMY2hhdF9tZXNz'
    'YWdlGAQgASgLMh8ucWF1bC5uZXQubWVzc2FnaW5nLkNoYXRNZXNzYWdlSABSC2NoYXRNZXNzYW'
    'dlEkQKDGZpbGVfbWVzc2FnZRgFIAEoCzIfLnFhdWwubmV0Lm1lc3NhZ2luZy5GaWxlTWVzc2Fn'
    'ZUgAUgtmaWxlTWVzc2FnZRJHCg1ncm91cF9tZXNzYWdlGAYgASgLMiAucWF1bC5uZXQubWVzc2'
    'FnaW5nLkdyb3VwTWVzc2FnZUgAUgxncm91cE1lc3NhZ2USQQoLcnRjX21lc3NhZ2UYByABKAsy'
    'Hi5xYXVsLm5ldC5tZXNzYWdpbmcuUnRjTWVzc2FnZUgAUgpydGNNZXNzYWdlQgkKB3BheWxvYW'
    'Q=');

@$core.Deprecated('Use chatMessageDescriptor instead')
const ChatMessage$json = {
  '1': 'ChatMessage',
  '2': [
    {'1': 'content', '3': 1, '4': 1, '5': 9, '10': 'content'},
  ],
};

/// Descriptor for `ChatMessage`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List chatMessageDescriptor = $convert.base64Decode(
    'CgtDaGF0TWVzc2FnZRIYCgdjb250ZW50GAEgASgJUgdjb250ZW50');

@$core.Deprecated('Use fileMessageDescriptor instead')
const FileMessage$json = {
  '1': 'FileMessage',
  '2': [
    {'1': 'content', '3': 1, '4': 1, '5': 12, '10': 'content'},
  ],
};

/// Descriptor for `FileMessage`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fileMessageDescriptor = $convert.base64Decode(
    'CgtGaWxlTWVzc2FnZRIYCgdjb250ZW50GAEgASgMUgdjb250ZW50');

@$core.Deprecated('Use groupMessageDescriptor instead')
const GroupMessage$json = {
  '1': 'GroupMessage',
  '2': [
    {'1': 'content', '3': 1, '4': 1, '5': 12, '10': 'content'},
  ],
};

/// Descriptor for `GroupMessage`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupMessageDescriptor = $convert.base64Decode(
    'CgxHcm91cE1lc3NhZ2USGAoHY29udGVudBgBIAEoDFIHY29udGVudA==');

@$core.Deprecated('Use rtcMessageDescriptor instead')
const RtcMessage$json = {
  '1': 'RtcMessage',
  '2': [
    {'1': 'content', '3': 1, '4': 1, '5': 12, '10': 'content'},
  ],
};

/// Descriptor for `RtcMessage`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List rtcMessageDescriptor = $convert.base64Decode(
    'CgpSdGNNZXNzYWdlEhgKB2NvbnRlbnQYASABKAxSB2NvbnRlbnQ=');

@$core.Deprecated('Use dtnDescriptor instead')
const Dtn$json = {
  '1': 'Dtn',
  '2': [
    {'1': 'container', '3': 1, '4': 1, '5': 12, '9': 0, '10': 'container'},
    {'1': 'response', '3': 2, '4': 1, '5': 11, '6': '.qaul.net.messaging.DtnResponse', '9': 0, '10': 'response'},
  ],
  '8': [
    {'1': 'message'},
  ],
};

/// Descriptor for `Dtn`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dtnDescriptor = $convert.base64Decode(
    'CgNEdG4SHgoJY29udGFpbmVyGAEgASgMSABSCWNvbnRhaW5lchI9CghyZXNwb25zZRgCIAEoCz'
    'IfLnFhdWwubmV0Lm1lc3NhZ2luZy5EdG5SZXNwb25zZUgAUghyZXNwb25zZUIJCgdtZXNzYWdl');

@$core.Deprecated('Use dtnResponseDescriptor instead')
const DtnResponse$json = {
  '1': 'DtnResponse',
  '2': [
    {'1': 'response_type', '3': 1, '4': 1, '5': 14, '6': '.qaul.net.messaging.DtnResponse.ResponseType', '10': 'responseType'},
    {'1': 'signature', '3': 2, '4': 1, '5': 12, '10': 'signature'},
    {'1': 'reason', '3': 3, '4': 1, '5': 14, '6': '.qaul.net.messaging.DtnResponse.Reason', '10': 'reason'},
  ],
  '4': [DtnResponse_ResponseType$json, DtnResponse_Reason$json],
};

@$core.Deprecated('Use dtnResponseDescriptor instead')
const DtnResponse_ResponseType$json = {
  '1': 'ResponseType',
  '2': [
    {'1': 'ACCEPTED', '2': 0},
    {'1': 'REJECTED', '2': 1},
  ],
};

@$core.Deprecated('Use dtnResponseDescriptor instead')
const DtnResponse_Reason$json = {
  '1': 'Reason',
  '2': [
    {'1': 'NONE', '2': 0},
    {'1': 'USER_NOT_ACCEPTED', '2': 1},
    {'1': 'OVERALL_QUOTA', '2': 2},
    {'1': 'USER_QUOTA', '2': 3},
  ],
};

/// Descriptor for `DtnResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dtnResponseDescriptor = $convert.base64Decode(
    'CgtEdG5SZXNwb25zZRJRCg1yZXNwb25zZV90eXBlGAEgASgOMiwucWF1bC5uZXQubWVzc2FnaW'
    '5nLkR0blJlc3BvbnNlLlJlc3BvbnNlVHlwZVIMcmVzcG9uc2VUeXBlEhwKCXNpZ25hdHVyZRgC'
    'IAEoDFIJc2lnbmF0dXJlEj4KBnJlYXNvbhgDIAEoDjImLnFhdWwubmV0Lm1lc3NhZ2luZy5EdG'
    '5SZXNwb25zZS5SZWFzb25SBnJlYXNvbiIqCgxSZXNwb25zZVR5cGUSDAoIQUNDRVBURUQQABIM'
    'CghSRUpFQ1RFRBABIkwKBlJlYXNvbhIICgROT05FEAASFQoRVVNFUl9OT1RfQUNDRVBURUQQAR'
    'IRCg1PVkVSQUxMX1FVT1RBEAISDgoKVVNFUl9RVU9UQRAD');

