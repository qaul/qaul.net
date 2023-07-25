///
//  Generated code. Do not modify.
//  source: services/chat/chat.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

// ignore_for_file: UNDEFINED_SHOWN_NAME
import 'dart:core' as $core;
import 'package:protobuf/protobuf.dart' as $pb;

class MessageStatus extends $pb.ProtobufEnum {
  static const MessageStatus SENDING = MessageStatus._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'SENDING');
  static const MessageStatus SENT = MessageStatus._(1, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'SENT');
  static const MessageStatus CONFIRMED = MessageStatus._(2, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'CONFIRMED');
  static const MessageStatus CONFIRMED_BY_ALL = MessageStatus._(3, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'CONFIRMED_BY_ALL');
  static const MessageStatus RECEIVING = MessageStatus._(4, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'RECEIVING');
  static const MessageStatus RECEIVED = MessageStatus._(5, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'RECEIVED');

  static const $core.List<MessageStatus> values = <MessageStatus> [
    SENDING,
    SENT,
    CONFIRMED,
    CONFIRMED_BY_ALL,
    RECEIVING,
    RECEIVED,
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
  static const GroupEventType REMOVED = GroupEventType._(4, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'REMOVED');
  static const GroupEventType CLOSED = GroupEventType._(5, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'CLOSED');
  static const GroupEventType CREATED = GroupEventType._(6, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'CREATED');
  static const GroupEventType INVITE_ACCEPTED = GroupEventType._(7, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'INVITE_ACCEPTED');

  static const $core.List<GroupEventType> values = <GroupEventType> [
    DEFAULT,
    INVITED,
    JOINED,
    LEFT,
    REMOVED,
    CLOSED,
    CREATED,
    INVITE_ACCEPTED,
  ];

  static final $core.Map<$core.int, GroupEventType> _byValue = $pb.ProtobufEnum.initByValue(values);
  static GroupEventType? valueOf($core.int value) => _byValue[value];

  const GroupEventType._($core.int v, $core.String n) : super(v, n);
}

