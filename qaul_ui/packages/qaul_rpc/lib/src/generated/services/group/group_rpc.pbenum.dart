// This is a generated file - do not edit.
//
// Generated from services/group/group_rpc.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

/// Group member state
class GroupMemberState extends $pb.ProtobufEnum {
  /// invited
  static const GroupMemberState Invited =
      GroupMemberState._(0, _omitEnumNames ? '' : 'Invited');

  /// activated
  static const GroupMemberState Activated =
      GroupMemberState._(1, _omitEnumNames ? '' : 'Activated');

  static const $core.List<GroupMemberState> values = <GroupMemberState>[
    Invited,
    Activated,
  ];

  static final $core.List<GroupMemberState?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 1);
  static GroupMemberState? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const GroupMemberState._(super.value, super.name);
}

/// Group member role
class GroupMemberRole extends $pb.ProtobufEnum {
  /// user
  static const GroupMemberRole User =
      GroupMemberRole._(0, _omitEnumNames ? '' : 'User');

  /// admin
  static const GroupMemberRole Admin =
      GroupMemberRole._(255, _omitEnumNames ? '' : 'Admin');

  static const $core.List<GroupMemberRole> values = <GroupMemberRole>[
    User,
    Admin,
  ];

  static final $core.Map<$core.int, GroupMemberRole> _byValue =
      $pb.ProtobufEnum.initByValue(values);
  static GroupMemberRole? valueOf($core.int value) => _byValue[value];

  const GroupMemberRole._(super.value, super.name);
}

/// Group Status
///
/// Indicates the working status of a group.
class GroupStatus extends $pb.ProtobufEnum {
  /// Group is Active
  ///
  /// The group is in active state and we can post
  /// messages to this group.
  static const GroupStatus ACTIVE =
      GroupStatus._(0, _omitEnumNames ? '' : 'ACTIVE');

  /// Invite Accepted
  ///
  /// We accepted the invitation to this group
  /// but we haven't received the updated group
  /// info from the group administrator yet.
  /// We therefore can't yet post messages into
  /// the group.
  static const GroupStatus INVITE_ACCEPTED =
      GroupStatus._(1, _omitEnumNames ? '' : 'INVITE_ACCEPTED');

  /// The group was deactivated
  ///
  /// We either left the group or have been removed from the group
  /// by the group administrator.
  /// We therefore can't post messages into this group anymore.
  static const GroupStatus DEACTIVATED =
      GroupStatus._(2, _omitEnumNames ? '' : 'DEACTIVATED');

  static const $core.List<GroupStatus> values = <GroupStatus>[
    ACTIVE,
    INVITE_ACCEPTED,
    DEACTIVATED,
  ];

  static final $core.List<GroupStatus?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 2);
  static GroupStatus? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const GroupStatus._(super.value, super.name);
}

const $core.bool _omitEnumNames =
    $core.bool.fromEnvironment('protobuf.omit_enum_names');
