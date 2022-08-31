///
//  Generated code. Do not modify.
//  source: services/group/group_net.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,deprecated_member_use_from_same_package,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use groupMemberStateDescriptor instead')
const GroupMemberState$json = const {
  '1': 'GroupMemberState',
  '2': const [
    const {'1': 'Invited', '2': 0},
    const {'1': 'Activated', '2': 1},
  ],
};

/// Descriptor for `GroupMemberState`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List groupMemberStateDescriptor = $convert.base64Decode('ChBHcm91cE1lbWJlclN0YXRlEgsKB0ludml0ZWQQABINCglBY3RpdmF0ZWQQAQ==');
@$core.Deprecated('Use groupMemberRoleDescriptor instead')
const GroupMemberRole$json = const {
  '1': 'GroupMemberRole',
  '2': const [
    const {'1': 'User', '2': 0},
    const {'1': 'Admin', '2': 255},
  ],
};

/// Descriptor for `GroupMemberRole`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List groupMemberRoleDescriptor = $convert.base64Decode('Cg9Hcm91cE1lbWJlclJvbGUSCAoEVXNlchAAEgoKBUFkbWluEP8B');
@$core.Deprecated('Use groupContainerDescriptor instead')
const GroupContainer$json = const {
  '1': 'GroupContainer',
  '2': const [
    const {'1': 'invite_member', '3': 1, '4': 1, '5': 11, '6': '.qaul.net.group.InviteMember', '9': 0, '10': 'inviteMember'},
    const {'1': 'reply_invite', '3': 2, '4': 1, '5': 11, '6': '.qaul.net.group.ReplyInvite', '9': 0, '10': 'replyInvite'},
    const {'1': 'group_info', '3': 3, '4': 1, '5': 11, '6': '.qaul.net.group.GroupInfo', '9': 0, '10': 'groupInfo'},
    const {'1': 'removed', '3': 4, '4': 1, '5': 11, '6': '.qaul.net.group.RemovedMember', '9': 0, '10': 'removed'},
  ],
  '8': const [
    const {'1': 'message'},
  ],
};

/// Descriptor for `GroupContainer`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupContainerDescriptor = $convert.base64Decode('Cg5Hcm91cENvbnRhaW5lchJDCg1pbnZpdGVfbWVtYmVyGAEgASgLMhwucWF1bC5uZXQuZ3JvdXAuSW52aXRlTWVtYmVySABSDGludml0ZU1lbWJlchJACgxyZXBseV9pbnZpdGUYAiABKAsyGy5xYXVsLm5ldC5ncm91cC5SZXBseUludml0ZUgAUgtyZXBseUludml0ZRI6Cgpncm91cF9pbmZvGAMgASgLMhkucWF1bC5uZXQuZ3JvdXAuR3JvdXBJbmZvSABSCWdyb3VwSW5mbxI5CgdyZW1vdmVkGAQgASgLMh0ucWF1bC5uZXQuZ3JvdXAuUmVtb3ZlZE1lbWJlckgAUgdyZW1vdmVkQgkKB21lc3NhZ2U=');
@$core.Deprecated('Use inviteMemberDescriptor instead')
const InviteMember$json = const {
  '1': 'InviteMember',
  '2': const [
    const {'1': 'group', '3': 1, '4': 1, '5': 11, '6': '.qaul.net.group.GroupInfo', '10': 'group'},
  ],
};

/// Descriptor for `InviteMember`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List inviteMemberDescriptor = $convert.base64Decode('CgxJbnZpdGVNZW1iZXISLwoFZ3JvdXAYASABKAsyGS5xYXVsLm5ldC5ncm91cC5Hcm91cEluZm9SBWdyb3Vw');
@$core.Deprecated('Use groupMemberDescriptor instead')
const GroupMember$json = const {
  '1': 'GroupMember',
  '2': const [
    const {'1': 'user_id', '3': 1, '4': 1, '5': 12, '10': 'userId'},
    const {'1': 'role', '3': 2, '4': 1, '5': 14, '6': '.qaul.net.group.GroupMemberRole', '10': 'role'},
    const {'1': 'joined_at', '3': 3, '4': 1, '5': 4, '10': 'joinedAt'},
    const {'1': 'state', '3': 4, '4': 1, '5': 14, '6': '.qaul.net.group.GroupMemberState', '10': 'state'},
    const {'1': 'last_message_index', '3': 5, '4': 1, '5': 13, '10': 'lastMessageIndex'},
  ],
};

/// Descriptor for `GroupMember`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupMemberDescriptor = $convert.base64Decode('CgtHcm91cE1lbWJlchIXCgd1c2VyX2lkGAEgASgMUgZ1c2VySWQSMwoEcm9sZRgCIAEoDjIfLnFhdWwubmV0Lmdyb3VwLkdyb3VwTWVtYmVyUm9sZVIEcm9sZRIbCglqb2luZWRfYXQYAyABKARSCGpvaW5lZEF0EjYKBXN0YXRlGAQgASgOMiAucWF1bC5uZXQuZ3JvdXAuR3JvdXBNZW1iZXJTdGF0ZVIFc3RhdGUSLAoSbGFzdF9tZXNzYWdlX2luZGV4GAUgASgNUhBsYXN0TWVzc2FnZUluZGV4');
@$core.Deprecated('Use groupInfoDescriptor instead')
const GroupInfo$json = const {
  '1': 'GroupInfo',
  '2': const [
    const {'1': 'group_id', '3': 1, '4': 1, '5': 12, '10': 'groupId'},
    const {'1': 'group_name', '3': 2, '4': 1, '5': 9, '10': 'groupName'},
    const {'1': 'created_at', '3': 3, '4': 1, '5': 4, '10': 'createdAt'},
    const {'1': 'revision', '3': 4, '4': 1, '5': 13, '10': 'revision'},
    const {'1': 'members', '3': 5, '4': 3, '5': 11, '6': '.qaul.net.group.GroupMember', '10': 'members'},
  ],
};

/// Descriptor for `GroupInfo`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupInfoDescriptor = $convert.base64Decode('CglHcm91cEluZm8SGQoIZ3JvdXBfaWQYASABKAxSB2dyb3VwSWQSHQoKZ3JvdXBfbmFtZRgCIAEoCVIJZ3JvdXBOYW1lEh0KCmNyZWF0ZWRfYXQYAyABKARSCWNyZWF0ZWRBdBIaCghyZXZpc2lvbhgEIAEoDVIIcmV2aXNpb24SNQoHbWVtYmVycxgFIAMoCzIbLnFhdWwubmV0Lmdyb3VwLkdyb3VwTWVtYmVyUgdtZW1iZXJz');
@$core.Deprecated('Use replyInviteDescriptor instead')
const ReplyInvite$json = const {
  '1': 'ReplyInvite',
  '2': const [
    const {'1': 'group_id', '3': 1, '4': 1, '5': 12, '10': 'groupId'},
    const {'1': 'accept', '3': 2, '4': 1, '5': 8, '10': 'accept'},
  ],
};

/// Descriptor for `ReplyInvite`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List replyInviteDescriptor = $convert.base64Decode('CgtSZXBseUludml0ZRIZCghncm91cF9pZBgBIAEoDFIHZ3JvdXBJZBIWCgZhY2NlcHQYAiABKAhSBmFjY2VwdA==');
@$core.Deprecated('Use removedMemberDescriptor instead')
const RemovedMember$json = const {
  '1': 'RemovedMember',
  '2': const [
    const {'1': 'group_id', '3': 1, '4': 1, '5': 12, '10': 'groupId'},
  ],
};

/// Descriptor for `RemovedMember`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List removedMemberDescriptor = $convert.base64Decode('Cg1SZW1vdmVkTWVtYmVyEhkKCGdyb3VwX2lkGAEgASgMUgdncm91cElk');
