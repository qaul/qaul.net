// This is a generated file - do not edit.
//
// Generated from services/chat/chat.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

/// Sending status of sent messages
class MessageStatus extends $pb.ProtobufEnum {
  /// message not sent yet
  ///
  /// this state is used for receiving files too
  static const MessageStatus SENDING =
      MessageStatus._(0, _omitEnumNames ? '' : 'SENDING');

  /// message successfully sent to another node
  static const MessageStatus SENT =
      MessageStatus._(1, _omitEnumNames ? '' : 'SENT');

  /// reciption has been confirmed
  static const MessageStatus CONFIRMED =
      MessageStatus._(2, _omitEnumNames ? '' : 'CONFIRMED');

  /// all group members confirmed that they received
  /// the message
  static const MessageStatus CONFIRMED_BY_ALL =
      MessageStatus._(3, _omitEnumNames ? '' : 'CONFIRMED_BY_ALL');

  /// message receiving
  static const MessageStatus RECEIVING =
      MessageStatus._(4, _omitEnumNames ? '' : 'RECEIVING');

  /// message received
  static const MessageStatus RECEIVED =
      MessageStatus._(5, _omitEnumNames ? '' : 'RECEIVED');

  static const $core.List<MessageStatus> values = <MessageStatus>[
    SENDING,
    SENT,
    CONFIRMED,
    CONFIRMED_BY_ALL,
    RECEIVING,
    RECEIVED,
  ];

  static final $core.List<MessageStatus?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 5);
  static MessageStatus? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const MessageStatus._(super.value, super.name);
}

/// Group info type definition
class GroupEventType extends $pb.ProtobufEnum {
  /// default value, undefined message
  /// ignore this message
  static const GroupEventType DEFAULT =
      GroupEventType._(0, _omitEnumNames ? '' : 'DEFAULT');

  /// user invited to group
  static const GroupEventType INVITED =
      GroupEventType._(1, _omitEnumNames ? '' : 'INVITED');

  /// user joined group
  static const GroupEventType JOINED =
      GroupEventType._(2, _omitEnumNames ? '' : 'JOINED');

  /// user left group
  static const GroupEventType LEFT =
      GroupEventType._(3, _omitEnumNames ? '' : 'LEFT');

  /// your user was removed
  static const GroupEventType REMOVED =
      GroupEventType._(4, _omitEnumNames ? '' : 'REMOVED');

  /// group was closed
  static const GroupEventType CLOSED =
      GroupEventType._(5, _omitEnumNames ? '' : 'CLOSED');

  /// group was created
  static const GroupEventType CREATED =
      GroupEventType._(6, _omitEnumNames ? '' : 'CREATED');

  /// group invite was accepted
  ///
  /// this state indicates, that we accepted
  /// an invite, but that we haven't received
  /// the group update from the administrator yet,
  /// and are therefore not yet an official member of the group.
  static const GroupEventType INVITE_ACCEPTED =
      GroupEventType._(7, _omitEnumNames ? '' : 'INVITE_ACCEPTED');

  static const $core.List<GroupEventType> values = <GroupEventType>[
    DEFAULT,
    INVITED,
    JOINED,
    LEFT,
    REMOVED,
    CLOSED,
    CREATED,
    INVITE_ACCEPTED,
  ];

  static final $core.List<GroupEventType?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 7);
  static GroupEventType? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const GroupEventType._(super.value, super.name);
}

const $core.bool _omitEnumNames =
    $core.bool.fromEnvironment('protobuf.omit_enum_names');
