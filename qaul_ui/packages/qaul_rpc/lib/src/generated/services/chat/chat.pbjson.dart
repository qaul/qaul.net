///
//  Generated code. Do not modify.
//  source: services/chat/chat.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,deprecated_member_use_from_same_package,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use chatContentTypeDescriptor instead')
const ChatContentType$json = const {
  '1': 'ChatContentType',
  '2': const [
    const {'1': 'NONE', '2': 0},
    const {'1': 'CHAT', '2': 1},
    const {'1': 'FILE', '2': 2},
    const {'1': 'GROUP', '2': 3},
    const {'1': 'RTC', '2': 4},
  ],
};

/// Descriptor for `ChatContentType`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List chatContentTypeDescriptor = $convert.base64Decode('Cg9DaGF0Q29udGVudFR5cGUSCAoETk9ORRAAEggKBENIQVQQARIICgRGSUxFEAISCQoFR1JPVVAQAxIHCgNSVEMQBA==');
@$core.Deprecated('Use messageStatusDescriptor instead')
const MessageStatus$json = const {
  '1': 'MessageStatus',
  '2': const [
    const {'1': 'SENDING', '2': 0},
    const {'1': 'SENT', '2': 1},
    const {'1': 'RECEIVED', '2': 2},
    const {'1': 'RECEIVED_BY_ALL', '2': 3},
  ],
};

/// Descriptor for `MessageStatus`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List messageStatusDescriptor = $convert.base64Decode('Cg1NZXNzYWdlU3RhdHVzEgsKB1NFTkRJTkcQABIICgRTRU5UEAESDAoIUkVDRUlWRUQQAhITCg9SRUNFSVZFRF9CWV9BTEwQAw==');
@$core.Deprecated('Use groupEventTypeDescriptor instead')
const GroupEventType$json = const {
  '1': 'GroupEventType',
  '2': const [
    const {'1': 'DEFAULT', '2': 0},
    const {'1': 'INVITED', '2': 1},
    const {'1': 'JOINED', '2': 2},
    const {'1': 'LEFT', '2': 3},
    const {'1': 'CLOSED', '2': 4},
  ],
};

/// Descriptor for `GroupEventType`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List groupEventTypeDescriptor = $convert.base64Decode('Cg5Hcm91cEV2ZW50VHlwZRILCgdERUZBVUxUEAASCwoHSU5WSVRFRBABEgoKBkpPSU5FRBACEggKBExFRlQQAxIKCgZDTE9TRUQQBA==');
@$core.Deprecated('Use chatDescriptor instead')
const Chat$json = const {
  '1': 'Chat',
  '2': const [
    const {'1': 'overview_request', '3': 1, '4': 1, '5': 11, '6': '.qaul.rpc.chat.ChatOverviewRequest', '9': 0, '10': 'overviewRequest'},
    const {'1': 'overview_list', '3': 2, '4': 1, '5': 11, '6': '.qaul.rpc.chat.ChatOverviewList', '9': 0, '10': 'overviewList'},
    const {'1': 'conversation_request', '3': 3, '4': 1, '5': 11, '6': '.qaul.rpc.chat.ChatConversationRequest', '9': 0, '10': 'conversationRequest'},
    const {'1': 'conversation_list', '3': 4, '4': 1, '5': 11, '6': '.qaul.rpc.chat.ChatConversationList', '9': 0, '10': 'conversationList'},
    const {'1': 'send', '3': 5, '4': 1, '5': 11, '6': '.qaul.rpc.chat.ChatMessageSend', '9': 0, '10': 'send'},
  ],
  '8': const [
    const {'1': 'message'},
  ],
};

/// Descriptor for `Chat`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List chatDescriptor = $convert.base64Decode('CgRDaGF0Ek8KEG92ZXJ2aWV3X3JlcXVlc3QYASABKAsyIi5xYXVsLnJwYy5jaGF0LkNoYXRPdmVydmlld1JlcXVlc3RIAFIPb3ZlcnZpZXdSZXF1ZXN0EkYKDW92ZXJ2aWV3X2xpc3QYAiABKAsyHy5xYXVsLnJwYy5jaGF0LkNoYXRPdmVydmlld0xpc3RIAFIMb3ZlcnZpZXdMaXN0ElsKFGNvbnZlcnNhdGlvbl9yZXF1ZXN0GAMgASgLMiYucWF1bC5ycGMuY2hhdC5DaGF0Q29udmVyc2F0aW9uUmVxdWVzdEgAUhNjb252ZXJzYXRpb25SZXF1ZXN0ElIKEWNvbnZlcnNhdGlvbl9saXN0GAQgASgLMiMucWF1bC5ycGMuY2hhdC5DaGF0Q29udmVyc2F0aW9uTGlzdEgAUhBjb252ZXJzYXRpb25MaXN0EjQKBHNlbmQYBSABKAsyHi5xYXVsLnJwYy5jaGF0LkNoYXRNZXNzYWdlU2VuZEgAUgRzZW5kQgkKB21lc3NhZ2U=');
@$core.Deprecated('Use chatOverviewRequestDescriptor instead')
const ChatOverviewRequest$json = const {
  '1': 'ChatOverviewRequest',
};

/// Descriptor for `ChatOverviewRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List chatOverviewRequestDescriptor = $convert.base64Decode('ChNDaGF0T3ZlcnZpZXdSZXF1ZXN0');
@$core.Deprecated('Use chatOverviewListDescriptor instead')
const ChatOverviewList$json = const {
  '1': 'ChatOverviewList',
  '2': const [
    const {'1': 'overview_list', '3': 1, '4': 3, '5': 11, '6': '.qaul.rpc.chat.ChatOverview', '10': 'overviewList'},
  ],
};

/// Descriptor for `ChatOverviewList`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List chatOverviewListDescriptor = $convert.base64Decode('ChBDaGF0T3ZlcnZpZXdMaXN0EkAKDW92ZXJ2aWV3X2xpc3QYASADKAsyGy5xYXVsLnJwYy5jaGF0LkNoYXRPdmVydmlld1IMb3ZlcnZpZXdMaXN0');
@$core.Deprecated('Use chatOverviewDescriptor instead')
const ChatOverview$json = const {
  '1': 'ChatOverview',
  '2': const [
    const {'1': 'conversation_id', '3': 1, '4': 1, '5': 12, '10': 'conversationId'},
    const {'1': 'last_message_index', '3': 2, '4': 1, '5': 4, '10': 'lastMessageIndex'},
    const {'1': 'name', '3': 3, '4': 1, '5': 9, '10': 'name'},
    const {'1': 'last_message_at', '3': 4, '4': 1, '5': 4, '10': 'lastMessageAt'},
    const {'1': 'unread', '3': 5, '4': 1, '5': 5, '10': 'unread'},
    const {'1': 'content_type', '3': 6, '4': 1, '5': 14, '6': '.qaul.rpc.chat.ChatContentType', '10': 'contentType'},
    const {'1': 'content', '3': 7, '4': 1, '5': 12, '10': 'content'},
    const {'1': 'last_message_sender_id', '3': 8, '4': 1, '5': 12, '10': 'lastMessageSenderId'},
  ],
};

/// Descriptor for `ChatOverview`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List chatOverviewDescriptor = $convert.base64Decode('CgxDaGF0T3ZlcnZpZXcSJwoPY29udmVyc2F0aW9uX2lkGAEgASgMUg5jb252ZXJzYXRpb25JZBIsChJsYXN0X21lc3NhZ2VfaW5kZXgYAiABKARSEGxhc3RNZXNzYWdlSW5kZXgSEgoEbmFtZRgDIAEoCVIEbmFtZRImCg9sYXN0X21lc3NhZ2VfYXQYBCABKARSDWxhc3RNZXNzYWdlQXQSFgoGdW5yZWFkGAUgASgFUgZ1bnJlYWQSQQoMY29udGVudF90eXBlGAYgASgOMh4ucWF1bC5ycGMuY2hhdC5DaGF0Q29udGVudFR5cGVSC2NvbnRlbnRUeXBlEhgKB2NvbnRlbnQYByABKAxSB2NvbnRlbnQSMwoWbGFzdF9tZXNzYWdlX3NlbmRlcl9pZBgIIAEoDFITbGFzdE1lc3NhZ2VTZW5kZXJJZA==');
@$core.Deprecated('Use chatConversationRequestDescriptor instead')
const ChatConversationRequest$json = const {
  '1': 'ChatConversationRequest',
  '2': const [
    const {'1': 'conversation_id', '3': 1, '4': 1, '5': 12, '10': 'conversationId'},
    const {'1': 'last_index', '3': 2, '4': 1, '5': 4, '10': 'lastIndex'},
  ],
};

/// Descriptor for `ChatConversationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List chatConversationRequestDescriptor = $convert.base64Decode('ChdDaGF0Q29udmVyc2F0aW9uUmVxdWVzdBInCg9jb252ZXJzYXRpb25faWQYASABKAxSDmNvbnZlcnNhdGlvbklkEh0KCmxhc3RfaW5kZXgYAiABKARSCWxhc3RJbmRleA==');
@$core.Deprecated('Use chatConversationListDescriptor instead')
const ChatConversationList$json = const {
  '1': 'ChatConversationList',
  '2': const [
    const {'1': 'conversation_id', '3': 1, '4': 1, '5': 12, '10': 'conversationId'},
    const {'1': 'message_list', '3': 2, '4': 3, '5': 11, '6': '.qaul.rpc.chat.ChatMessage', '10': 'messageList'},
  ],
};

/// Descriptor for `ChatConversationList`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List chatConversationListDescriptor = $convert.base64Decode('ChRDaGF0Q29udmVyc2F0aW9uTGlzdBInCg9jb252ZXJzYXRpb25faWQYASABKAxSDmNvbnZlcnNhdGlvbklkEj0KDG1lc3NhZ2VfbGlzdBgCIAMoCzIaLnFhdWwucnBjLmNoYXQuQ2hhdE1lc3NhZ2VSC21lc3NhZ2VMaXN0');
@$core.Deprecated('Use chatMessageDescriptor instead')
const ChatMessage$json = const {
  '1': 'ChatMessage',
  '2': const [
    const {'1': 'index', '3': 1, '4': 1, '5': 4, '10': 'index'},
    const {'1': 'sender_id', '3': 2, '4': 1, '5': 12, '10': 'senderId'},
    const {'1': 'message_id', '3': 3, '4': 1, '5': 12, '10': 'messageId'},
    const {'1': 'status', '3': 4, '4': 1, '5': 14, '6': '.qaul.rpc.chat.MessageStatus', '10': 'status'},
    const {'1': 'message_reception_confirmed', '3': 10, '4': 3, '5': 11, '6': '.qaul.rpc.chat.MessageReceptionConfirmed', '10': 'messageReceptionConfirmed'},
    const {'1': 'conversation_id', '3': 5, '4': 1, '5': 12, '10': 'conversationId'},
    const {'1': 'sent_at', '3': 6, '4': 1, '5': 4, '10': 'sentAt'},
    const {'1': 'received_at', '3': 7, '4': 1, '5': 4, '10': 'receivedAt'},
    const {'1': 'content_type', '3': 8, '4': 1, '5': 14, '6': '.qaul.rpc.chat.ChatContentType', '10': 'contentType'},
    const {'1': 'content', '3': 9, '4': 1, '5': 12, '10': 'content'},
  ],
};

/// Descriptor for `ChatMessage`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List chatMessageDescriptor = $convert.base64Decode('CgtDaGF0TWVzc2FnZRIUCgVpbmRleBgBIAEoBFIFaW5kZXgSGwoJc2VuZGVyX2lkGAIgASgMUghzZW5kZXJJZBIdCgptZXNzYWdlX2lkGAMgASgMUgltZXNzYWdlSWQSNAoGc3RhdHVzGAQgASgOMhwucWF1bC5ycGMuY2hhdC5NZXNzYWdlU3RhdHVzUgZzdGF0dXMSaAobbWVzc2FnZV9yZWNlcHRpb25fY29uZmlybWVkGAogAygLMigucWF1bC5ycGMuY2hhdC5NZXNzYWdlUmVjZXB0aW9uQ29uZmlybWVkUhltZXNzYWdlUmVjZXB0aW9uQ29uZmlybWVkEicKD2NvbnZlcnNhdGlvbl9pZBgFIAEoDFIOY29udmVyc2F0aW9uSWQSFwoHc2VudF9hdBgGIAEoBFIGc2VudEF0Eh8KC3JlY2VpdmVkX2F0GAcgASgEUgpyZWNlaXZlZEF0EkEKDGNvbnRlbnRfdHlwZRgIIAEoDjIeLnFhdWwucnBjLmNoYXQuQ2hhdENvbnRlbnRUeXBlUgtjb250ZW50VHlwZRIYCgdjb250ZW50GAkgASgMUgdjb250ZW50');
@$core.Deprecated('Use messageReceptionConfirmedDescriptor instead')
const MessageReceptionConfirmed$json = const {
  '1': 'MessageReceptionConfirmed',
  '2': const [
    const {'1': 'user_id', '3': 1, '4': 1, '5': 12, '10': 'userId'},
    const {'1': 'confirmed_at', '3': 2, '4': 1, '5': 4, '10': 'confirmedAt'},
  ],
};

/// Descriptor for `MessageReceptionConfirmed`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List messageReceptionConfirmedDescriptor = $convert.base64Decode('ChlNZXNzYWdlUmVjZXB0aW9uQ29uZmlybWVkEhcKB3VzZXJfaWQYASABKAxSBnVzZXJJZBIhCgxjb25maXJtZWRfYXQYAiABKARSC2NvbmZpcm1lZEF0');
@$core.Deprecated('Use chatContentDescriptor instead')
const ChatContent$json = const {
  '1': 'ChatContent',
  '2': const [
    const {'1': 'text', '3': 1, '4': 1, '5': 9, '10': 'text'},
  ],
};

/// Descriptor for `ChatContent`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List chatContentDescriptor = $convert.base64Decode('CgtDaGF0Q29udGVudBISCgR0ZXh0GAEgASgJUgR0ZXh0');
@$core.Deprecated('Use fileContentDescriptor instead')
const FileContent$json = const {
  '1': 'FileContent',
  '2': const [
    const {'1': 'file_id', '3': 1, '4': 1, '5': 4, '10': 'fileId'},
    const {'1': 'file_name', '3': 2, '4': 1, '5': 9, '10': 'fileName'},
    const {'1': 'file_extension', '3': 3, '4': 1, '5': 9, '10': 'fileExtension'},
    const {'1': 'file_size', '3': 4, '4': 1, '5': 13, '10': 'fileSize'},
    const {'1': 'file_description', '3': 5, '4': 1, '5': 9, '10': 'fileDescription'},
  ],
};

/// Descriptor for `FileContent`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fileContentDescriptor = $convert.base64Decode('CgtGaWxlQ29udGVudBIXCgdmaWxlX2lkGAEgASgEUgZmaWxlSWQSGwoJZmlsZV9uYW1lGAIgASgJUghmaWxlTmFtZRIlCg5maWxlX2V4dGVuc2lvbhgDIAEoCVINZmlsZUV4dGVuc2lvbhIbCglmaWxlX3NpemUYBCABKA1SCGZpbGVTaXplEikKEGZpbGVfZGVzY3JpcHRpb24YBSABKAlSD2ZpbGVEZXNjcmlwdGlvbg==');
@$core.Deprecated('Use groupEventDescriptor instead')
const GroupEvent$json = const {
  '1': 'GroupEvent',
  '2': const [
    const {'1': 'event_type', '3': 1, '4': 1, '5': 14, '6': '.qaul.rpc.chat.GroupEventType', '10': 'eventType'},
    const {'1': 'user_id', '3': 2, '4': 1, '5': 12, '10': 'userId'},
  ],
};

/// Descriptor for `GroupEvent`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupEventDescriptor = $convert.base64Decode('CgpHcm91cEV2ZW50EjwKCmV2ZW50X3R5cGUYASABKA4yHS5xYXVsLnJwYy5jaGF0Lkdyb3VwRXZlbnRUeXBlUglldmVudFR5cGUSFwoHdXNlcl9pZBgCIAEoDFIGdXNlcklk');
@$core.Deprecated('Use chatMessageSendDescriptor instead')
const ChatMessageSend$json = const {
  '1': 'ChatMessageSend',
  '2': const [
    const {'1': 'conversation_id', '3': 1, '4': 1, '5': 12, '10': 'conversationId'},
    const {'1': 'content', '3': 2, '4': 1, '5': 9, '10': 'content'},
  ],
};

/// Descriptor for `ChatMessageSend`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List chatMessageSendDescriptor = $convert.base64Decode('Cg9DaGF0TWVzc2FnZVNlbmQSJwoPY29udmVyc2F0aW9uX2lkGAEgASgMUg5jb252ZXJzYXRpb25JZBIYCgdjb250ZW50GAIgASgJUgdjb250ZW50');
