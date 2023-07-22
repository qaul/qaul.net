//
//  Generated code. Do not modify.
//  source: services/chat/chat.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use messageStatusDescriptor instead')
const MessageStatus$json = {
  '1': 'MessageStatus',
  '2': [
    {'1': 'SENDING', '2': 0},
    {'1': 'SENT', '2': 1},
    {'1': 'CONFIRMED', '2': 2},
    {'1': 'CONFIRMED_BY_ALL', '2': 3},
    {'1': 'RECEIVING', '2': 4},
    {'1': 'RECEIVED', '2': 5},
  ],
};

/// Descriptor for `MessageStatus`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List messageStatusDescriptor = $convert.base64Decode(
    'Cg1NZXNzYWdlU3RhdHVzEgsKB1NFTkRJTkcQABIICgRTRU5UEAESDQoJQ09ORklSTUVEEAISFA'
    'oQQ09ORklSTUVEX0JZX0FMTBADEg0KCVJFQ0VJVklORxAEEgwKCFJFQ0VJVkVEEAU=');

@$core.Deprecated('Use groupEventTypeDescriptor instead')
const GroupEventType$json = {
  '1': 'GroupEventType',
  '2': [
    {'1': 'DEFAULT', '2': 0},
    {'1': 'INVITED', '2': 1},
    {'1': 'JOINED', '2': 2},
    {'1': 'LEFT', '2': 3},
    {'1': 'REMOVED', '2': 4},
    {'1': 'CLOSED', '2': 5},
    {'1': 'CREATED', '2': 6},
    {'1': 'INVITE_ACCEPTED', '2': 7},
  ],
};

/// Descriptor for `GroupEventType`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List groupEventTypeDescriptor = $convert.base64Decode(
    'Cg5Hcm91cEV2ZW50VHlwZRILCgdERUZBVUxUEAASCwoHSU5WSVRFRBABEgoKBkpPSU5FRBACEg'
    'gKBExFRlQQAxILCgdSRU1PVkVEEAQSCgoGQ0xPU0VEEAUSCwoHQ1JFQVRFRBAGEhMKD0lOVklU'
    'RV9BQ0NFUFRFRBAH');

@$core.Deprecated('Use chatDescriptor instead')
const Chat$json = {
  '1': 'Chat',
  '2': [
    {'1': 'conversation_request', '3': 3, '4': 1, '5': 11, '6': '.qaul.rpc.chat.ChatConversationRequest', '9': 0, '10': 'conversationRequest'},
    {'1': 'conversation_list', '3': 4, '4': 1, '5': 11, '6': '.qaul.rpc.chat.ChatConversationList', '9': 0, '10': 'conversationList'},
    {'1': 'send', '3': 5, '4': 1, '5': 11, '6': '.qaul.rpc.chat.ChatMessageSend', '9': 0, '10': 'send'},
  ],
  '8': [
    {'1': 'message'},
  ],
};

/// Descriptor for `Chat`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List chatDescriptor = $convert.base64Decode(
    'CgRDaGF0ElsKFGNvbnZlcnNhdGlvbl9yZXF1ZXN0GAMgASgLMiYucWF1bC5ycGMuY2hhdC5DaG'
    'F0Q29udmVyc2F0aW9uUmVxdWVzdEgAUhNjb252ZXJzYXRpb25SZXF1ZXN0ElIKEWNvbnZlcnNh'
    'dGlvbl9saXN0GAQgASgLMiMucWF1bC5ycGMuY2hhdC5DaGF0Q29udmVyc2F0aW9uTGlzdEgAUh'
    'Bjb252ZXJzYXRpb25MaXN0EjQKBHNlbmQYBSABKAsyHi5xYXVsLnJwYy5jaGF0LkNoYXRNZXNz'
    'YWdlU2VuZEgAUgRzZW5kQgkKB21lc3NhZ2U=');

@$core.Deprecated('Use chatConversationRequestDescriptor instead')
const ChatConversationRequest$json = {
  '1': 'ChatConversationRequest',
  '2': [
    {'1': 'group_id', '3': 1, '4': 1, '5': 12, '10': 'groupId'},
    {'1': 'last_index', '3': 2, '4': 1, '5': 4, '10': 'lastIndex'},
  ],
};

/// Descriptor for `ChatConversationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List chatConversationRequestDescriptor = $convert.base64Decode(
    'ChdDaGF0Q29udmVyc2F0aW9uUmVxdWVzdBIZCghncm91cF9pZBgBIAEoDFIHZ3JvdXBJZBIdCg'
    'psYXN0X2luZGV4GAIgASgEUglsYXN0SW5kZXg=');

@$core.Deprecated('Use chatConversationListDescriptor instead')
const ChatConversationList$json = {
  '1': 'ChatConversationList',
  '2': [
    {'1': 'group_id', '3': 1, '4': 1, '5': 12, '10': 'groupId'},
    {'1': 'message_list', '3': 2, '4': 3, '5': 11, '6': '.qaul.rpc.chat.ChatMessage', '10': 'messageList'},
  ],
};

/// Descriptor for `ChatConversationList`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List chatConversationListDescriptor = $convert.base64Decode(
    'ChRDaGF0Q29udmVyc2F0aW9uTGlzdBIZCghncm91cF9pZBgBIAEoDFIHZ3JvdXBJZBI9CgxtZX'
    'NzYWdlX2xpc3QYAiADKAsyGi5xYXVsLnJwYy5jaGF0LkNoYXRNZXNzYWdlUgttZXNzYWdlTGlz'
    'dA==');

@$core.Deprecated('Use chatMessageDescriptor instead')
const ChatMessage$json = {
  '1': 'ChatMessage',
  '2': [
    {'1': 'index', '3': 1, '4': 1, '5': 4, '10': 'index'},
    {'1': 'sender_id', '3': 2, '4': 1, '5': 12, '10': 'senderId'},
    {'1': 'message_id', '3': 3, '4': 1, '5': 12, '10': 'messageId'},
    {'1': 'status', '3': 4, '4': 1, '5': 14, '6': '.qaul.rpc.chat.MessageStatus', '10': 'status'},
    {'1': 'message_reception_confirmed', '3': 10, '4': 3, '5': 11, '6': '.qaul.rpc.chat.MessageReceptionConfirmed', '10': 'messageReceptionConfirmed'},
    {'1': 'group_id', '3': 5, '4': 1, '5': 12, '10': 'groupId'},
    {'1': 'sent_at', '3': 6, '4': 1, '5': 4, '10': 'sentAt'},
    {'1': 'received_at', '3': 7, '4': 1, '5': 4, '10': 'receivedAt'},
    {'1': 'content', '3': 8, '4': 1, '5': 12, '10': 'content'},
  ],
};

/// Descriptor for `ChatMessage`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List chatMessageDescriptor = $convert.base64Decode(
    'CgtDaGF0TWVzc2FnZRIUCgVpbmRleBgBIAEoBFIFaW5kZXgSGwoJc2VuZGVyX2lkGAIgASgMUg'
    'hzZW5kZXJJZBIdCgptZXNzYWdlX2lkGAMgASgMUgltZXNzYWdlSWQSNAoGc3RhdHVzGAQgASgO'
    'MhwucWF1bC5ycGMuY2hhdC5NZXNzYWdlU3RhdHVzUgZzdGF0dXMSaAobbWVzc2FnZV9yZWNlcH'
    'Rpb25fY29uZmlybWVkGAogAygLMigucWF1bC5ycGMuY2hhdC5NZXNzYWdlUmVjZXB0aW9uQ29u'
    'ZmlybWVkUhltZXNzYWdlUmVjZXB0aW9uQ29uZmlybWVkEhkKCGdyb3VwX2lkGAUgASgMUgdncm'
    '91cElkEhcKB3NlbnRfYXQYBiABKARSBnNlbnRBdBIfCgtyZWNlaXZlZF9hdBgHIAEoBFIKcmVj'
    'ZWl2ZWRBdBIYCgdjb250ZW50GAggASgMUgdjb250ZW50');

@$core.Deprecated('Use messageReceptionConfirmedDescriptor instead')
const MessageReceptionConfirmed$json = {
  '1': 'MessageReceptionConfirmed',
  '2': [
    {'1': 'user_id', '3': 1, '4': 1, '5': 12, '10': 'userId'},
    {'1': 'confirmed_at', '3': 2, '4': 1, '5': 4, '10': 'confirmedAt'},
  ],
};

/// Descriptor for `MessageReceptionConfirmed`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List messageReceptionConfirmedDescriptor = $convert.base64Decode(
    'ChlNZXNzYWdlUmVjZXB0aW9uQ29uZmlybWVkEhcKB3VzZXJfaWQYASABKAxSBnVzZXJJZBIhCg'
    'xjb25maXJtZWRfYXQYAiABKARSC2NvbmZpcm1lZEF0');

@$core.Deprecated('Use chatContentMessageDescriptor instead')
const ChatContentMessage$json = {
  '1': 'ChatContentMessage',
  '2': [
    {'1': 'chat_content', '3': 1, '4': 1, '5': 11, '6': '.qaul.rpc.chat.ChatContent', '9': 0, '10': 'chatContent'},
    {'1': 'file_content', '3': 2, '4': 1, '5': 11, '6': '.qaul.rpc.chat.FileContent', '9': 0, '10': 'fileContent'},
    {'1': 'group_event', '3': 3, '4': 1, '5': 11, '6': '.qaul.rpc.chat.GroupEvent', '9': 0, '10': 'groupEvent'},
  ],
  '8': [
    {'1': 'message'},
  ],
};

/// Descriptor for `ChatContentMessage`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List chatContentMessageDescriptor = $convert.base64Decode(
    'ChJDaGF0Q29udGVudE1lc3NhZ2USPwoMY2hhdF9jb250ZW50GAEgASgLMhoucWF1bC5ycGMuY2'
    'hhdC5DaGF0Q29udGVudEgAUgtjaGF0Q29udGVudBI/CgxmaWxlX2NvbnRlbnQYAiABKAsyGi5x'
    'YXVsLnJwYy5jaGF0LkZpbGVDb250ZW50SABSC2ZpbGVDb250ZW50EjwKC2dyb3VwX2V2ZW50GA'
    'MgASgLMhkucWF1bC5ycGMuY2hhdC5Hcm91cEV2ZW50SABSCmdyb3VwRXZlbnRCCQoHbWVzc2Fn'
    'ZQ==');

@$core.Deprecated('Use chatContentDescriptor instead')
const ChatContent$json = {
  '1': 'ChatContent',
  '2': [
    {'1': 'text', '3': 1, '4': 1, '5': 9, '10': 'text'},
  ],
};

/// Descriptor for `ChatContent`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List chatContentDescriptor = $convert.base64Decode(
    'CgtDaGF0Q29udGVudBISCgR0ZXh0GAEgASgJUgR0ZXh0');

@$core.Deprecated('Use fileContentDescriptor instead')
const FileContent$json = {
  '1': 'FileContent',
  '2': [
    {'1': 'file_id', '3': 1, '4': 1, '5': 4, '10': 'fileId'},
    {'1': 'file_name', '3': 2, '4': 1, '5': 9, '10': 'fileName'},
    {'1': 'file_extension', '3': 3, '4': 1, '5': 9, '10': 'fileExtension'},
    {'1': 'file_size', '3': 4, '4': 1, '5': 13, '10': 'fileSize'},
    {'1': 'file_description', '3': 5, '4': 1, '5': 9, '10': 'fileDescription'},
  ],
};

/// Descriptor for `FileContent`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fileContentDescriptor = $convert.base64Decode(
    'CgtGaWxlQ29udGVudBIXCgdmaWxlX2lkGAEgASgEUgZmaWxlSWQSGwoJZmlsZV9uYW1lGAIgAS'
    'gJUghmaWxlTmFtZRIlCg5maWxlX2V4dGVuc2lvbhgDIAEoCVINZmlsZUV4dGVuc2lvbhIbCglm'
    'aWxlX3NpemUYBCABKA1SCGZpbGVTaXplEikKEGZpbGVfZGVzY3JpcHRpb24YBSABKAlSD2ZpbG'
    'VEZXNjcmlwdGlvbg==');

@$core.Deprecated('Use groupEventDescriptor instead')
const GroupEvent$json = {
  '1': 'GroupEvent',
  '2': [
    {'1': 'event_type', '3': 1, '4': 1, '5': 14, '6': '.qaul.rpc.chat.GroupEventType', '10': 'eventType'},
    {'1': 'user_id', '3': 2, '4': 1, '5': 12, '10': 'userId'},
  ],
};

/// Descriptor for `GroupEvent`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupEventDescriptor = $convert.base64Decode(
    'CgpHcm91cEV2ZW50EjwKCmV2ZW50X3R5cGUYASABKA4yHS5xYXVsLnJwYy5jaGF0Lkdyb3VwRX'
    'ZlbnRUeXBlUglldmVudFR5cGUSFwoHdXNlcl9pZBgCIAEoDFIGdXNlcklk');

@$core.Deprecated('Use chatMessageSendDescriptor instead')
const ChatMessageSend$json = {
  '1': 'ChatMessageSend',
  '2': [
    {'1': 'group_id', '3': 1, '4': 1, '5': 12, '10': 'groupId'},
    {'1': 'content', '3': 2, '4': 1, '5': 9, '10': 'content'},
  ],
};

/// Descriptor for `ChatMessageSend`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List chatMessageSendDescriptor = $convert.base64Decode(
    'Cg9DaGF0TWVzc2FnZVNlbmQSGQoIZ3JvdXBfaWQYASABKAxSB2dyb3VwSWQSGAoHY29udGVudB'
    'gCIAEoCVIHY29udGVudA==');

