///
//  Generated code. Do not modify.
//  source: services/chat/chat.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

// ignore_for_file: UNDEFINED_SHOWN_NAME
import 'dart:core' as $core;
import 'package:protobuf/protobuf.dart' as $pb;

class ChatContentType extends $pb.ProtobufEnum {
  static const ChatContentType NONE = ChatContentType._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'NONE');
  static const ChatContentType CHAT = ChatContentType._(1, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'CHAT');
  static const ChatContentType FILE = ChatContentType._(2, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'FILE');
  static const ChatContentType GROUP = ChatContentType._(3, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'GROUP');
  static const ChatContentType RTC = ChatContentType._(4, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'RTC');

  static const $core.List<ChatContentType> values = <ChatContentType> [
    NONE,
    CHAT,
    FILE,
    GROUP,
    RTC,
  ];

  static final $core.Map<$core.int, ChatContentType> _byValue = $pb.ProtobufEnum.initByValue(values);
  static ChatContentType? valueOf($core.int value) => _byValue[value];

  const ChatContentType._($core.int v, $core.String n) : super(v, n);
}

class MessageStatus extends $pb.ProtobufEnum {
  static const MessageStatus SENDING = MessageStatus._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'SENDING');
  static const MessageStatus SENT = MessageStatus._(1, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'SENT');
  static const MessageStatus RECEIVED = MessageStatus._(2, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'RECEIVED');
  static const MessageStatus RECEIVED_BY_ALL = MessageStatus._(3, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'RECEIVED_BY_ALL');

  static const $core.List<MessageStatus> values = <MessageStatus> [
    SENDING,
    SENT,
    RECEIVED,
    RECEIVED_BY_ALL,
  ];

  static final $core.Map<$core.int, MessageStatus> _byValue = $pb.ProtobufEnum.initByValue(values);
  static MessageStatus? valueOf($core.int value) => _byValue[value];

  const MessageStatus._($core.int v, $core.String n) : super(v, n);
}

class GroupEventType extends $pb.ProtobufEnum {
  static const GroupEventType DEFAULT = GroupEventType._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'DEFAULT');
  static const GroupEventType INVITED = GroupEventType._(1, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'INVITED');
  static const GroupEventType JOINED = GroupEventType._(2, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'JOINED');
  static const GroupEventType LEFT = GroupEventType._(3, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'LEFT');
  static const GroupEventType CLOSED = GroupEventType._(4, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'CLOSED');

  static const $core.List<GroupEventType> values = <GroupEventType> [
    DEFAULT,
    INVITED,
    JOINED,
    LEFT,
    CLOSED,
  ];

  static final $core.Map<$core.int, GroupEventType> _byValue = $pb.ProtobufEnum.initByValue(values);
  static GroupEventType? valueOf($core.int value) => _byValue[value];

  const GroupEventType._($core.int v, $core.String n) : super(v, n);
}

