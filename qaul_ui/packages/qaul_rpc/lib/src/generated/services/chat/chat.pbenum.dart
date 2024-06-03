//
//  Generated code. Do not modify.
//  source: services/chat/chat.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

/// Sending status of sent messages
class MessageStatus extends $pb.ProtobufEnum {
  static const MessageStatus SENDING = MessageStatus._(0, _omitEnumNames ? '' : 'SENDING');
  static const MessageStatus SENT = MessageStatus._(1, _omitEnumNames ? '' : 'SENT');
  static const MessageStatus CONFIRMED = MessageStatus._(2, _omitEnumNames ? '' : 'CONFIRMED');
  static const MessageStatus CONFIRMED_BY_ALL = MessageStatus._(3, _omitEnumNames ? '' : 'CONFIRMED_BY_ALL');
  static const MessageStatus RECEIVING = MessageStatus._(4, _omitEnumNames ? '' : 'RECEIVING');
  static const MessageStatus RECEIVED = MessageStatus._(5, _omitEnumNames ? '' : 'RECEIVED');

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

/// Group info type definition
class GroupEventType extends $pb.ProtobufEnum {
  static const GroupEventType DEFAULT = GroupEventType._(0, _omitEnumNames ? '' : 'DEFAULT');
  static const GroupEventType INVITED = GroupEventType._(1, _omitEnumNames ? '' : 'INVITED');
  static const GroupEventType JOINED = GroupEventType._(2, _omitEnumNames ? '' : 'JOINED');
  static const GroupEventType LEFT = GroupEventType._(3, _omitEnumNames ? '' : 'LEFT');
  static const GroupEventType REMOVED = GroupEventType._(4, _omitEnumNames ? '' : 'REMOVED');
  static const GroupEventType CLOSED = GroupEventType._(5, _omitEnumNames ? '' : 'CLOSED');
  static const GroupEventType CREATED = GroupEventType._(6, _omitEnumNames ? '' : 'CREATED');
  static const GroupEventType INVITE_ACCEPTED = GroupEventType._(7, _omitEnumNames ? '' : 'INVITE_ACCEPTED');

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


const _omitEnumNames = $core.bool.fromEnvironment('protobuf.omit_enum_names');
