//
//  Generated code. Do not modify.
//  source: services/group/group_net.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class GroupMemberState extends $pb.ProtobufEnum {
  static const GroupMemberState Invited = GroupMemberState._(0, _omitEnumNames ? '' : 'Invited');
  static const GroupMemberState Activated = GroupMemberState._(1, _omitEnumNames ? '' : 'Activated');

  static const $core.List<GroupMemberState> values = <GroupMemberState> [
    Invited,
    Activated,
  ];

  static final $core.Map<$core.int, GroupMemberState> _byValue = $pb.ProtobufEnum.initByValue(values);
  static GroupMemberState? valueOf($core.int value) => _byValue[value];

  const GroupMemberState._($core.int v, $core.String n) : super(v, n);
}

class GroupMemberRole extends $pb.ProtobufEnum {
  static const GroupMemberRole User = GroupMemberRole._(0, _omitEnumNames ? '' : 'User');
  static const GroupMemberRole Admin = GroupMemberRole._(255, _omitEnumNames ? '' : 'Admin');

  static const $core.List<GroupMemberRole> values = <GroupMemberRole> [
    User,
    Admin,
  ];

  static final $core.Map<$core.int, GroupMemberRole> _byValue = $pb.ProtobufEnum.initByValue(values);
  static GroupMemberRole? valueOf($core.int value) => _byValue[value];

  const GroupMemberRole._($core.int v, $core.String n) : super(v, n);
}


const _omitEnumNames = $core.bool.fromEnvironment('protobuf.omit_enum_names');
