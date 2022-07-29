///
//  Generated code. Do not modify.
//  source: services/group/group_rpc.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields,deprecated_member_use_from_same_package

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use groupDescriptor instead')
const Group$json = const {
  '1': 'Group',
  '2': const [
    const {'1': 'group_create_request', '3': 1, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupCreateRequest', '9': 0, '10': 'groupCreateRequest'},
    const {'1': 'group_create_response', '3': 2, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupCreateResponse', '9': 0, '10': 'groupCreateResponse'},
    const {'1': 'group_rename_request', '3': 3, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupRenameRequest', '9': 0, '10': 'groupRenameRequest'},
    const {'1': 'group_invite_member_request', '3': 4, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupInviteMemberRequest', '9': 0, '10': 'groupInviteMemberRequest'},
    const {'1': 'group_remove_member_request', '3': 5, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupRemoveMemberRequest', '9': 0, '10': 'groupRemoveMemberRequest'},
    const {'1': 'group_info_request', '3': 6, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupInfoRequest', '9': 0, '10': 'groupInfoRequest'},
    const {'1': 'group_info_response', '3': 7, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupInfoResponse', '9': 0, '10': 'groupInfoResponse'},
    const {'1': 'group_reply_invite_request', '3': 8, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupReplyInviteRequest', '9': 0, '10': 'groupReplyInviteRequest'},
    const {'1': 'group_list_request', '3': 9, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupListRequest', '9': 0, '10': 'groupListRequest'},
    const {'1': 'group_list_response', '3': 10, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupListResponse', '9': 0, '10': 'groupListResponse'},
    const {'1': 'group_send_request', '3': 11, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupSendRequest', '9': 0, '10': 'groupSendRequest'},
  ],
  '8': const [
    const {'1': 'message'},
  ],
};

/// Descriptor for `Group`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupDescriptor = $convert.base64Decode('CgVHcm91cBJWChRncm91cF9jcmVhdGVfcmVxdWVzdBgBIAEoCzIiLnFhdWwucnBjLmdyb3VwLkdyb3VwQ3JlYXRlUmVxdWVzdEgAUhJncm91cENyZWF0ZVJlcXVlc3QSWQoVZ3JvdXBfY3JlYXRlX3Jlc3BvbnNlGAIgASgLMiMucWF1bC5ycGMuZ3JvdXAuR3JvdXBDcmVhdGVSZXNwb25zZUgAUhNncm91cENyZWF0ZVJlc3BvbnNlElYKFGdyb3VwX3JlbmFtZV9yZXF1ZXN0GAMgASgLMiIucWF1bC5ycGMuZ3JvdXAuR3JvdXBSZW5hbWVSZXF1ZXN0SABSEmdyb3VwUmVuYW1lUmVxdWVzdBJpChtncm91cF9pbnZpdGVfbWVtYmVyX3JlcXVlc3QYBCABKAsyKC5xYXVsLnJwYy5ncm91cC5Hcm91cEludml0ZU1lbWJlclJlcXVlc3RIAFIYZ3JvdXBJbnZpdGVNZW1iZXJSZXF1ZXN0EmkKG2dyb3VwX3JlbW92ZV9tZW1iZXJfcmVxdWVzdBgFIAEoCzIoLnFhdWwucnBjLmdyb3VwLkdyb3VwUmVtb3ZlTWVtYmVyUmVxdWVzdEgAUhhncm91cFJlbW92ZU1lbWJlclJlcXVlc3QSUAoSZ3JvdXBfaW5mb19yZXF1ZXN0GAYgASgLMiAucWF1bC5ycGMuZ3JvdXAuR3JvdXBJbmZvUmVxdWVzdEgAUhBncm91cEluZm9SZXF1ZXN0ElMKE2dyb3VwX2luZm9fcmVzcG9uc2UYByABKAsyIS5xYXVsLnJwYy5ncm91cC5Hcm91cEluZm9SZXNwb25zZUgAUhFncm91cEluZm9SZXNwb25zZRJmChpncm91cF9yZXBseV9pbnZpdGVfcmVxdWVzdBgIIAEoCzInLnFhdWwucnBjLmdyb3VwLkdyb3VwUmVwbHlJbnZpdGVSZXF1ZXN0SABSF2dyb3VwUmVwbHlJbnZpdGVSZXF1ZXN0ElAKEmdyb3VwX2xpc3RfcmVxdWVzdBgJIAEoCzIgLnFhdWwucnBjLmdyb3VwLkdyb3VwTGlzdFJlcXVlc3RIAFIQZ3JvdXBMaXN0UmVxdWVzdBJTChNncm91cF9saXN0X3Jlc3BvbnNlGAogASgLMiEucWF1bC5ycGMuZ3JvdXAuR3JvdXBMaXN0UmVzcG9uc2VIAFIRZ3JvdXBMaXN0UmVzcG9uc2USUAoSZ3JvdXBfc2VuZF9yZXF1ZXN0GAsgASgLMiAucWF1bC5ycGMuZ3JvdXAuR3JvdXBTZW5kUmVxdWVzdEgAUhBncm91cFNlbmRSZXF1ZXN0QgkKB21lc3NhZ2U=');
@$core.Deprecated('Use groupCreateRequestDescriptor instead')
const GroupCreateRequest$json = const {
  '1': 'GroupCreateRequest',
  '2': const [
    const {'1': 'group_name', '3': 1, '4': 1, '5': 9, '10': 'groupName'},
  ],
};

/// Descriptor for `GroupCreateRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupCreateRequestDescriptor = $convert.base64Decode('ChJHcm91cENyZWF0ZVJlcXVlc3QSHQoKZ3JvdXBfbmFtZRgBIAEoCVIJZ3JvdXBOYW1l');
@$core.Deprecated('Use groupCreateResponseDescriptor instead')
const GroupCreateResponse$json = const {
  '1': 'GroupCreateResponse',
  '2': const [
    const {'1': 'group_name', '3': 1, '4': 1, '5': 9, '10': 'groupName'},
    const {'1': 'group_id', '3': 2, '4': 1, '5': 12, '10': 'groupId'},
  ],
};

/// Descriptor for `GroupCreateResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupCreateResponseDescriptor = $convert.base64Decode('ChNHcm91cENyZWF0ZVJlc3BvbnNlEh0KCmdyb3VwX25hbWUYASABKAlSCWdyb3VwTmFtZRIZCghncm91cF9pZBgCIAEoDFIHZ3JvdXBJZA==');
@$core.Deprecated('Use groupRenameRequestDescriptor instead')
const GroupRenameRequest$json = const {
  '1': 'GroupRenameRequest',
  '2': const [
    const {'1': 'group_id', '3': 1, '4': 1, '5': 12, '10': 'groupId'},
    const {'1': 'group_name', '3': 2, '4': 1, '5': 9, '10': 'groupName'},
  ],
};

/// Descriptor for `GroupRenameRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupRenameRequestDescriptor = $convert.base64Decode('ChJHcm91cFJlbmFtZVJlcXVlc3QSGQoIZ3JvdXBfaWQYASABKAxSB2dyb3VwSWQSHQoKZ3JvdXBfbmFtZRgCIAEoCVIJZ3JvdXBOYW1l');
@$core.Deprecated('Use groupInviteMemberRequestDescriptor instead')
const GroupInviteMemberRequest$json = const {
  '1': 'GroupInviteMemberRequest',
  '2': const [
    const {'1': 'group_id', '3': 1, '4': 1, '5': 12, '10': 'groupId'},
    const {'1': 'user_id', '3': 2, '4': 1, '5': 12, '10': 'userId'},
  ],
};

/// Descriptor for `GroupInviteMemberRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupInviteMemberRequestDescriptor = $convert.base64Decode('ChhHcm91cEludml0ZU1lbWJlclJlcXVlc3QSGQoIZ3JvdXBfaWQYASABKAxSB2dyb3VwSWQSFwoHdXNlcl9pZBgCIAEoDFIGdXNlcklk');
@$core.Deprecated('Use groupReplyInviteRequestDescriptor instead')
const GroupReplyInviteRequest$json = const {
  '1': 'GroupReplyInviteRequest',
  '2': const [
    const {'1': 'group_id', '3': 1, '4': 1, '5': 12, '10': 'groupId'},
    const {'1': 'user_id', '3': 2, '4': 1, '5': 12, '10': 'userId'},
    const {'1': 'accept', '3': 3, '4': 1, '5': 8, '10': 'accept'},
  ],
};

/// Descriptor for `GroupReplyInviteRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupReplyInviteRequestDescriptor = $convert.base64Decode('ChdHcm91cFJlcGx5SW52aXRlUmVxdWVzdBIZCghncm91cF9pZBgBIAEoDFIHZ3JvdXBJZBIXCgd1c2VyX2lkGAIgASgMUgZ1c2VySWQSFgoGYWNjZXB0GAMgASgIUgZhY2NlcHQ=');
@$core.Deprecated('Use groupRemoveMemberRequestDescriptor instead')
const GroupRemoveMemberRequest$json = const {
  '1': 'GroupRemoveMemberRequest',
  '2': const [
    const {'1': 'group_id', '3': 1, '4': 1, '5': 12, '10': 'groupId'},
    const {'1': 'user_id', '3': 2, '4': 1, '5': 12, '10': 'userId'},
  ],
};

/// Descriptor for `GroupRemoveMemberRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupRemoveMemberRequestDescriptor = $convert.base64Decode('ChhHcm91cFJlbW92ZU1lbWJlclJlcXVlc3QSGQoIZ3JvdXBfaWQYASABKAxSB2dyb3VwSWQSFwoHdXNlcl9pZBgCIAEoDFIGdXNlcklk');
@$core.Deprecated('Use groupInfoRequestDescriptor instead')
const GroupInfoRequest$json = const {
  '1': 'GroupInfoRequest',
  '2': const [
    const {'1': 'group_id', '3': 1, '4': 1, '5': 12, '10': 'groupId'},
  ],
};

/// Descriptor for `GroupInfoRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupInfoRequestDescriptor = $convert.base64Decode('ChBHcm91cEluZm9SZXF1ZXN0EhkKCGdyb3VwX2lkGAEgASgMUgdncm91cElk');
@$core.Deprecated('Use groupMemberDescriptor instead')
const GroupMember$json = const {
  '1': 'GroupMember',
  '2': const [
    const {'1': 'user_id', '3': 1, '4': 1, '5': 12, '10': 'userId'},
    const {'1': 'role', '3': 2, '4': 1, '5': 13, '10': 'role'},
    const {'1': 'joined_at', '3': 3, '4': 1, '5': 4, '10': 'joinedAt'},
    const {'1': 'state', '3': 4, '4': 1, '5': 13, '10': 'state'},
  ],
};

/// Descriptor for `GroupMember`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupMemberDescriptor = $convert.base64Decode('CgtHcm91cE1lbWJlchIXCgd1c2VyX2lkGAEgASgMUgZ1c2VySWQSEgoEcm9sZRgCIAEoDVIEcm9sZRIbCglqb2luZWRfYXQYAyABKARSCGpvaW5lZEF0EhQKBXN0YXRlGAQgASgNUgVzdGF0ZQ==');
@$core.Deprecated('Use groupInfoResponseDescriptor instead')
const GroupInfoResponse$json = const {
  '1': 'GroupInfoResponse',
  '2': const [
    const {'1': 'group_id', '3': 1, '4': 1, '5': 12, '10': 'groupId'},
    const {'1': 'group_name', '3': 2, '4': 1, '5': 9, '10': 'groupName'},
    const {'1': 'created_at', '3': 3, '4': 1, '5': 4, '10': 'createdAt'},
    const {'1': 'members', '3': 4, '4': 3, '5': 11, '6': '.qaul.rpc.group.GroupMember', '10': 'members'},
  ],
};

/// Descriptor for `GroupInfoResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupInfoResponseDescriptor = $convert.base64Decode('ChFHcm91cEluZm9SZXNwb25zZRIZCghncm91cF9pZBgBIAEoDFIHZ3JvdXBJZBIdCgpncm91cF9uYW1lGAIgASgJUglncm91cE5hbWUSHQoKY3JlYXRlZF9hdBgDIAEoBFIJY3JlYXRlZEF0EjUKB21lbWJlcnMYBCADKAsyGy5xYXVsLnJwYy5ncm91cC5Hcm91cE1lbWJlclIHbWVtYmVycw==');
@$core.Deprecated('Use groupListRequestDescriptor instead')
const GroupListRequest$json = const {
  '1': 'GroupListRequest',
};

/// Descriptor for `GroupListRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupListRequestDescriptor = $convert.base64Decode('ChBHcm91cExpc3RSZXF1ZXN0');
@$core.Deprecated('Use groupListResponseDescriptor instead')
const GroupListResponse$json = const {
  '1': 'GroupListResponse',
  '2': const [
    const {'1': 'groups', '3': 1, '4': 3, '5': 11, '6': '.qaul.rpc.group.GroupInfoResponse', '10': 'groups'},
  ],
};

/// Descriptor for `GroupListResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupListResponseDescriptor = $convert.base64Decode('ChFHcm91cExpc3RSZXNwb25zZRI5CgZncm91cHMYASADKAsyIS5xYXVsLnJwYy5ncm91cC5Hcm91cEluZm9SZXNwb25zZVIGZ3JvdXBz');
@$core.Deprecated('Use groupSendRequestDescriptor instead')
const GroupSendRequest$json = const {
  '1': 'GroupSendRequest',
  '2': const [
    const {'1': 'group_id', '3': 1, '4': 1, '5': 12, '10': 'groupId'},
    const {'1': 'message', '3': 2, '4': 1, '5': 9, '10': 'message'},
  ],
};

/// Descriptor for `GroupSendRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupSendRequestDescriptor = $convert.base64Decode('ChBHcm91cFNlbmRSZXF1ZXN0EhkKCGdyb3VwX2lkGAEgASgMUgdncm91cElkEhgKB21lc3NhZ2UYAiABKAlSB21lc3NhZ2U=');
@$core.Deprecated('Use groupConversationRequestDescriptor instead')
const GroupConversationRequest$json = const {
  '1': 'GroupConversationRequest',
  '2': const [
    const {'1': 'group_id', '3': 1, '4': 1, '5': 12, '10': 'groupId'},
  ],
};

/// Descriptor for `GroupConversationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupConversationRequestDescriptor = $convert.base64Decode('ChhHcm91cENvbnZlcnNhdGlvblJlcXVlc3QSGQoIZ3JvdXBfaWQYASABKAxSB2dyb3VwSWQ=');
