///
//  Generated code. Do not modify.
//  source: services/group/group_net.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

// ignore_for_file: UNDEFINED_SHOWN_NAME
import 'dart:core' as $core;
import 'package:protobuf/protobuf.dart' as $pb;

class GroupMemberState extends $pb.ProtobufEnum {
  static const GroupMemberState Invited = GroupMemberState._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'Invited');
  static const GroupMemberState Activated = GroupMemberState._(1, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'Activated');

  static const $core.List<GroupMemberState> values = <GroupMemberState> [
    Invited,
    Activated,
  ];

  static final $core.Map<$core.int, GroupMemberState> _byValue = $pb.ProtobufEnum.initByValue(values);
  static GroupMemberState? valueOf($core.int value) => _byValue[value];

  const GroupMemberState._($core.int v, $core.String n) : super(v, n);
}

class GroupMemberRole extends $pb.ProtobufEnum {
  static const GroupMemberRole User = GroupMemberRole._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'User');
  static const GroupMemberRole Admin = GroupMemberRole._(255, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'Admin');

  static const $core.List<GroupMemberRole> values = <GroupMemberRole> [
    User,
    Admin,
  ];

  static final $core.Map<$core.int, GroupMemberRole> _byValue = $pb.ProtobufEnum.initByValue(values);
  static GroupMemberRole? valueOf($core.int value) => _byValue[value];

  const GroupMemberRole._($core.int v, $core.String n) : super(v, n);
}

