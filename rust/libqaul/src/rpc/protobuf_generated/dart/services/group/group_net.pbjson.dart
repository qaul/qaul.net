///
//  Generated code. Do not modify.
//  source: services/group/group_net.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields,deprecated_member_use_from_same_package

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use groupContainerDescriptor instead')
const GroupContainer$json = const {
  '1': 'GroupContainer',
  '2': const [
    const {'1': 'invite_member', '3': 1, '4': 1, '5': 11, '6': '.qaul.net.group.InviteMember', '9': 0, '10': 'inviteMember'},
    const {'1': 'reply_invite', '3': 2, '4': 1, '5': 11, '6': '.qaul.net.group.ReplyInvite', '9': 0, '10': 'replyInvite'},
    const {'1': 'notify', '3': 3, '4': 1, '5': 11, '6': '.qaul.net.group.GroupNotify', '9': 0, '10': 'notify'},
    const {'1': 'removed', '3': 4, '4': 1, '5': 11, '6': '.qaul.net.group.RemovedMember', '9': 0, '10': 'removed'},
    const {'1': 'group_message', '3': 5, '4': 1, '5': 11, '6': '.qaul.net.group.GroupMessage', '9': 0, '10': 'groupMessage'},
  ],
  '8': const [
    const {'1': 'message'},
  ],
};

/// Descriptor for `GroupContainer`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupContainerDescriptor = $convert.base64Decode('Cg5Hcm91cENvbnRhaW5lchJDCg1pbnZpdGVfbWVtYmVyGAEgASgLMhwucWF1bC5uZXQuZ3JvdXAuSW52aXRlTWVtYmVySABSDGludml0ZU1lbWJlchJACgxyZXBseV9pbnZpdGUYAiABKAsyGy5xYXVsLm5ldC5ncm91cC5SZXBseUludml0ZUgAUgtyZXBseUludml0ZRI1CgZub3RpZnkYAyABKAsyGy5xYXVsLm5ldC5ncm91cC5Hcm91cE5vdGlmeUgAUgZub3RpZnkSOQoHcmVtb3ZlZBgEIAEoCzIdLnFhdWwubmV0Lmdyb3VwLlJlbW92ZWRNZW1iZXJIAFIHcmVtb3ZlZBJDCg1ncm91cF9tZXNzYWdlGAUgASgLMhwucWF1bC5uZXQuZ3JvdXAuR3JvdXBNZXNzYWdlSABSDGdyb3VwTWVzc2FnZUIJCgdtZXNzYWdl');
@$core.Deprecated('Use inviteMemberDescriptor instead')
const InviteMember$json = const {
  '1': 'InviteMember',
  '2': const [
    const {'1': 'group_id', '3': 1, '4': 1, '5': 12, '10': 'groupId'},
    const {'1': 'group_name', '3': 2, '4': 1, '5': 9, '10': 'groupName'},
    const {'1': 'admin_id', '3': 3, '4': 1, '5': 12, '10': 'adminId'},
    const {'1': 'created_at', '3': 4, '4': 1, '5': 4, '10': 'createdAt'},
    const {'1': 'members_count', '3': 5, '4': 1, '5': 13, '10': 'membersCount'},
  ],
};

/// Descriptor for `InviteMember`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List inviteMemberDescriptor = $convert.base64Decode('CgxJbnZpdGVNZW1iZXISGQoIZ3JvdXBfaWQYASABKAxSB2dyb3VwSWQSHQoKZ3JvdXBfbmFtZRgCIAEoCVIJZ3JvdXBOYW1lEhkKCGFkbWluX2lkGAMgASgMUgdhZG1pbklkEh0KCmNyZWF0ZWRfYXQYBCABKARSCWNyZWF0ZWRBdBIjCg1tZW1iZXJzX2NvdW50GAUgASgNUgxtZW1iZXJzQ291bnQ=');
@$core.Deprecated('Use memberDescriptor instead')
const Member$json = const {
  '1': 'Member',
  '2': const [
    const {'1': 'user_id', '3': 1, '4': 1, '5': 12, '10': 'userId'},
    const {'1': 'role', '3': 2, '4': 1, '5': 13, '10': 'role'},
    const {'1': 'joined_at', '3': 3, '4': 1, '5': 4, '10': 'joinedAt'},
    const {'1': 'state', '3': 4, '4': 1, '5': 13, '10': 'state'},
  ],
};

/// Descriptor for `Member`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List memberDescriptor = $convert.base64Decode('CgZNZW1iZXISFwoHdXNlcl9pZBgBIAEoDFIGdXNlcklkEhIKBHJvbGUYAiABKA1SBHJvbGUSGwoJam9pbmVkX2F0GAMgASgEUghqb2luZWRBdBIUCgVzdGF0ZRgEIAEoDVIFc3RhdGU=');
@$core.Deprecated('Use groupNotifyDescriptor instead')
const GroupNotify$json = const {
  '1': 'GroupNotify',
  '2': const [
    const {'1': 'group_id', '3': 1, '4': 1, '5': 12, '10': 'groupId'},
    const {'1': 'group_name', '3': 2, '4': 1, '5': 9, '10': 'groupName'},
    const {'1': 'created_at', '3': 3, '4': 1, '5': 4, '10': 'createdAt'},
    const {'1': 'creator_id', '3': 4, '4': 1, '5': 12, '10': 'creatorId'},
    const {'1': 'members', '3': 5, '4': 3, '5': 11, '6': '.qaul.net.group.Member', '10': 'members'},
  ],
};

/// Descriptor for `GroupNotify`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupNotifyDescriptor = $convert.base64Decode('CgtHcm91cE5vdGlmeRIZCghncm91cF9pZBgBIAEoDFIHZ3JvdXBJZBIdCgpncm91cF9uYW1lGAIgASgJUglncm91cE5hbWUSHQoKY3JlYXRlZF9hdBgDIAEoBFIJY3JlYXRlZEF0Eh0KCmNyZWF0b3JfaWQYBCABKAxSCWNyZWF0b3JJZBIwCgdtZW1iZXJzGAUgAygLMhYucWF1bC5uZXQuZ3JvdXAuTWVtYmVyUgdtZW1iZXJz');
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
@$core.Deprecated('Use groupMessageDescriptor instead')
const GroupMessage$json = const {
  '1': 'GroupMessage',
  '2': const [
    const {'1': 'group_id', '3': 1, '4': 1, '5': 12, '10': 'groupId'},
    const {'1': 'content', '3': 2, '4': 1, '5': 9, '10': 'content'},
    const {'1': 'sent_at', '3': 3, '4': 1, '5': 4, '10': 'sentAt'},
  ],
};

/// Descriptor for `GroupMessage`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupMessageDescriptor = $convert.base64Decode('CgxHcm91cE1lc3NhZ2USGQoIZ3JvdXBfaWQYASABKAxSB2dyb3VwSWQSGAoHY29udGVudBgCIAEoCVIHY29udGVudBIXCgdzZW50X2F0GAMgASgEUgZzZW50QXQ=');
