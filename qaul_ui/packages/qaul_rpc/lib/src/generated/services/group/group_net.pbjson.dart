// This is a generated file - do not edit.
//
// Generated from services/group/group_net.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports
// ignore_for_file: unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use groupMemberStateDescriptor instead')
const GroupMemberState$json = {
  '1': 'GroupMemberState',
  '2': [
    {'1': 'Invited', '2': 0},
    {'1': 'Activated', '2': 1},
  ],
};

/// Descriptor for `GroupMemberState`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List groupMemberStateDescriptor = $convert.base64Decode(
    'ChBHcm91cE1lbWJlclN0YXRlEgsKB0ludml0ZWQQABINCglBY3RpdmF0ZWQQAQ==');

@$core.Deprecated('Use groupMemberRoleDescriptor instead')
const GroupMemberRole$json = {
  '1': 'GroupMemberRole',
  '2': [
    {'1': 'User', '2': 0},
    {'1': 'Admin', '2': 255},
  ],
};

/// Descriptor for `GroupMemberRole`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List groupMemberRoleDescriptor = $convert
    .base64Decode('Cg9Hcm91cE1lbWJlclJvbGUSCAoEVXNlchAAEgoKBUFkbWluEP8B');

@$core.Deprecated('Use groupContainerDescriptor instead')
const GroupContainer$json = {
  '1': 'GroupContainer',
  '2': [
    {
      '1': 'invite_member',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.qaul.net.group.InviteMember',
      '9': 0,
      '10': 'inviteMember'
    },
    {
      '1': 'reply_invite',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.qaul.net.group.ReplyInvite',
      '9': 0,
      '10': 'replyInvite'
    },
    {
      '1': 'group_info',
      '3': 3,
      '4': 1,
      '5': 11,
      '6': '.qaul.net.group.GroupInfo',
      '9': 0,
      '10': 'groupInfo'
    },
    {
      '1': 'removed',
      '3': 4,
      '4': 1,
      '5': 11,
      '6': '.qaul.net.group.RemovedMember',
      '9': 0,
      '10': 'removed'
    },
  ],
  '8': [
    {'1': 'message'},
  ],
};

/// Descriptor for `GroupContainer`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupContainerDescriptor = $convert.base64Decode(
    'Cg5Hcm91cENvbnRhaW5lchJDCg1pbnZpdGVfbWVtYmVyGAEgASgLMhwucWF1bC5uZXQuZ3JvdX'
    'AuSW52aXRlTWVtYmVySABSDGludml0ZU1lbWJlchJACgxyZXBseV9pbnZpdGUYAiABKAsyGy5x'
    'YXVsLm5ldC5ncm91cC5SZXBseUludml0ZUgAUgtyZXBseUludml0ZRI6Cgpncm91cF9pbmZvGA'
    'MgASgLMhkucWF1bC5uZXQuZ3JvdXAuR3JvdXBJbmZvSABSCWdyb3VwSW5mbxI5CgdyZW1vdmVk'
    'GAQgASgLMh0ucWF1bC5uZXQuZ3JvdXAuUmVtb3ZlZE1lbWJlckgAUgdyZW1vdmVkQgkKB21lc3'
    'NhZ2U=');

@$core.Deprecated('Use inviteMemberDescriptor instead')
const InviteMember$json = {
  '1': 'InviteMember',
  '2': [
    {
      '1': 'group',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.qaul.net.group.GroupInfo',
      '10': 'group'
    },
  ],
};

/// Descriptor for `InviteMember`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List inviteMemberDescriptor = $convert.base64Decode(
    'CgxJbnZpdGVNZW1iZXISLwoFZ3JvdXAYASABKAsyGS5xYXVsLm5ldC5ncm91cC5Hcm91cEluZm'
    '9SBWdyb3Vw');

@$core.Deprecated('Use groupMemberDescriptor instead')
const GroupMember$json = {
  '1': 'GroupMember',
  '2': [
    {'1': 'user_id', '3': 1, '4': 1, '5': 12, '10': 'userId'},
    {
      '1': 'role',
      '3': 2,
      '4': 1,
      '5': 14,
      '6': '.qaul.net.group.GroupMemberRole',
      '10': 'role'
    },
    {'1': 'joined_at', '3': 3, '4': 1, '5': 4, '10': 'joinedAt'},
    {
      '1': 'state',
      '3': 4,
      '4': 1,
      '5': 14,
      '6': '.qaul.net.group.GroupMemberState',
      '10': 'state'
    },
    {
      '1': 'last_message_index',
      '3': 5,
      '4': 1,
      '5': 13,
      '10': 'lastMessageIndex'
    },
  ],
};

/// Descriptor for `GroupMember`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupMemberDescriptor = $convert.base64Decode(
    'CgtHcm91cE1lbWJlchIXCgd1c2VyX2lkGAEgASgMUgZ1c2VySWQSMwoEcm9sZRgCIAEoDjIfLn'
    'FhdWwubmV0Lmdyb3VwLkdyb3VwTWVtYmVyUm9sZVIEcm9sZRIbCglqb2luZWRfYXQYAyABKARS'
    'CGpvaW5lZEF0EjYKBXN0YXRlGAQgASgOMiAucWF1bC5uZXQuZ3JvdXAuR3JvdXBNZW1iZXJTdG'
    'F0ZVIFc3RhdGUSLAoSbGFzdF9tZXNzYWdlX2luZGV4GAUgASgNUhBsYXN0TWVzc2FnZUluZGV4');

@$core.Deprecated('Use groupInfoDescriptor instead')
const GroupInfo$json = {
  '1': 'GroupInfo',
  '2': [
    {'1': 'group_id', '3': 1, '4': 1, '5': 12, '10': 'groupId'},
    {'1': 'group_name', '3': 2, '4': 1, '5': 9, '10': 'groupName'},
    {'1': 'created_at', '3': 3, '4': 1, '5': 4, '10': 'createdAt'},
    {'1': 'revision', '3': 4, '4': 1, '5': 13, '10': 'revision'},
    {
      '1': 'members',
      '3': 5,
      '4': 3,
      '5': 11,
      '6': '.qaul.net.group.GroupMember',
      '10': 'members'
    },
  ],
};

/// Descriptor for `GroupInfo`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupInfoDescriptor = $convert.base64Decode(
    'CglHcm91cEluZm8SGQoIZ3JvdXBfaWQYASABKAxSB2dyb3VwSWQSHQoKZ3JvdXBfbmFtZRgCIA'
    'EoCVIJZ3JvdXBOYW1lEh0KCmNyZWF0ZWRfYXQYAyABKARSCWNyZWF0ZWRBdBIaCghyZXZpc2lv'
    'bhgEIAEoDVIIcmV2aXNpb24SNQoHbWVtYmVycxgFIAMoCzIbLnFhdWwubmV0Lmdyb3VwLkdyb3'
    'VwTWVtYmVyUgdtZW1iZXJz');

@$core.Deprecated('Use replyInviteDescriptor instead')
const ReplyInvite$json = {
  '1': 'ReplyInvite',
  '2': [
    {'1': 'group_id', '3': 1, '4': 1, '5': 12, '10': 'groupId'},
    {'1': 'accept', '3': 2, '4': 1, '5': 8, '10': 'accept'},
  ],
};

/// Descriptor for `ReplyInvite`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List replyInviteDescriptor = $convert.base64Decode(
    'CgtSZXBseUludml0ZRIZCghncm91cF9pZBgBIAEoDFIHZ3JvdXBJZBIWCgZhY2NlcHQYAiABKA'
    'hSBmFjY2VwdA==');

@$core.Deprecated('Use removedMemberDescriptor instead')
const RemovedMember$json = {
  '1': 'RemovedMember',
  '2': [
    {'1': 'group_id', '3': 1, '4': 1, '5': 12, '10': 'groupId'},
  ],
};

/// Descriptor for `RemovedMember`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List removedMemberDescriptor = $convert
    .base64Decode('Cg1SZW1vdmVkTWVtYmVyEhkKCGdyb3VwX2lkGAEgASgMUgdncm91cElk');
