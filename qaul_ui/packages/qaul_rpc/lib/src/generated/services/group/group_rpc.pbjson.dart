///
//  Generated code. Do not modify.
//  source: services/group/group_rpc.proto
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
@$core.Deprecated('Use groupStatusDescriptor instead')
const GroupStatus$json = const {
  '1': 'GroupStatus',
  '2': const [
    const {'1': 'ACTIVE', '2': 0},
    const {'1': 'INVITE_ACCEPTED', '2': 1},
    const {'1': 'DEACTIVATED', '2': 2},
  ],
};

/// Descriptor for `GroupStatus`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List groupStatusDescriptor = $convert.base64Decode('CgtHcm91cFN0YXR1cxIKCgZBQ1RJVkUQABITCg9JTlZJVEVfQUNDRVBURUQQARIPCgtERUFDVElWQVRFRBAC');
@$core.Deprecated('Use groupDescriptor instead')
const Group$json = const {
  '1': 'Group',
  '2': const [
    const {'1': 'group_create_request', '3': 1, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupCreateRequest', '9': 0, '10': 'groupCreateRequest'},
    const {'1': 'group_create_response', '3': 2, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupCreateResponse', '9': 0, '10': 'groupCreateResponse'},
    const {'1': 'group_rename_request', '3': 3, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupRenameRequest', '9': 0, '10': 'groupRenameRequest'},
    const {'1': 'group_rename_response', '3': 4, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupRenameResponse', '9': 0, '10': 'groupRenameResponse'},
    const {'1': 'group_invite_member_request', '3': 5, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupInviteMemberRequest', '9': 0, '10': 'groupInviteMemberRequest'},
    const {'1': 'group_invite_member_response', '3': 6, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupInviteMemberResponse', '9': 0, '10': 'groupInviteMemberResponse'},
    const {'1': 'group_remove_member_request', '3': 7, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupRemoveMemberRequest', '9': 0, '10': 'groupRemoveMemberRequest'},
    const {'1': 'group_remove_member_response', '3': 8, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupRemoveMemberResponse', '9': 0, '10': 'groupRemoveMemberResponse'},
    const {'1': 'group_info_request', '3': 9, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupInfoRequest', '9': 0, '10': 'groupInfoRequest'},
    const {'1': 'group_info_response', '3': 10, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupInfo', '9': 0, '10': 'groupInfoResponse'},
    const {'1': 'group_reply_invite_request', '3': 11, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupReplyInviteRequest', '9': 0, '10': 'groupReplyInviteRequest'},
    const {'1': 'group_reply_invite_response', '3': 12, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupReplyInviteResponse', '9': 0, '10': 'groupReplyInviteResponse'},
    const {'1': 'group_list_request', '3': 13, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupListRequest', '9': 0, '10': 'groupListRequest'},
    const {'1': 'group_list_response', '3': 14, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupListResponse', '9': 0, '10': 'groupListResponse'},
    const {'1': 'group_invited_request', '3': 15, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupInvitedRequest', '9': 0, '10': 'groupInvitedRequest'},
    const {'1': 'group_invited_response', '3': 16, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupInvitedResponse', '9': 0, '10': 'groupInvitedResponse'},
  ],
  '8': const [
    const {'1': 'message'},
  ],
};

/// Descriptor for `Group`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupDescriptor = $convert.base64Decode('CgVHcm91cBJWChRncm91cF9jcmVhdGVfcmVxdWVzdBgBIAEoCzIiLnFhdWwucnBjLmdyb3VwLkdyb3VwQ3JlYXRlUmVxdWVzdEgAUhJncm91cENyZWF0ZVJlcXVlc3QSWQoVZ3JvdXBfY3JlYXRlX3Jlc3BvbnNlGAIgASgLMiMucWF1bC5ycGMuZ3JvdXAuR3JvdXBDcmVhdGVSZXNwb25zZUgAUhNncm91cENyZWF0ZVJlc3BvbnNlElYKFGdyb3VwX3JlbmFtZV9yZXF1ZXN0GAMgASgLMiIucWF1bC5ycGMuZ3JvdXAuR3JvdXBSZW5hbWVSZXF1ZXN0SABSEmdyb3VwUmVuYW1lUmVxdWVzdBJZChVncm91cF9yZW5hbWVfcmVzcG9uc2UYBCABKAsyIy5xYXVsLnJwYy5ncm91cC5Hcm91cFJlbmFtZVJlc3BvbnNlSABSE2dyb3VwUmVuYW1lUmVzcG9uc2USaQobZ3JvdXBfaW52aXRlX21lbWJlcl9yZXF1ZXN0GAUgASgLMigucWF1bC5ycGMuZ3JvdXAuR3JvdXBJbnZpdGVNZW1iZXJSZXF1ZXN0SABSGGdyb3VwSW52aXRlTWVtYmVyUmVxdWVzdBJsChxncm91cF9pbnZpdGVfbWVtYmVyX3Jlc3BvbnNlGAYgASgLMikucWF1bC5ycGMuZ3JvdXAuR3JvdXBJbnZpdGVNZW1iZXJSZXNwb25zZUgAUhlncm91cEludml0ZU1lbWJlclJlc3BvbnNlEmkKG2dyb3VwX3JlbW92ZV9tZW1iZXJfcmVxdWVzdBgHIAEoCzIoLnFhdWwucnBjLmdyb3VwLkdyb3VwUmVtb3ZlTWVtYmVyUmVxdWVzdEgAUhhncm91cFJlbW92ZU1lbWJlclJlcXVlc3QSbAocZ3JvdXBfcmVtb3ZlX21lbWJlcl9yZXNwb25zZRgIIAEoCzIpLnFhdWwucnBjLmdyb3VwLkdyb3VwUmVtb3ZlTWVtYmVyUmVzcG9uc2VIAFIZZ3JvdXBSZW1vdmVNZW1iZXJSZXNwb25zZRJQChJncm91cF9pbmZvX3JlcXVlc3QYCSABKAsyIC5xYXVsLnJwYy5ncm91cC5Hcm91cEluZm9SZXF1ZXN0SABSEGdyb3VwSW5mb1JlcXVlc3QSSwoTZ3JvdXBfaW5mb19yZXNwb25zZRgKIAEoCzIZLnFhdWwucnBjLmdyb3VwLkdyb3VwSW5mb0gAUhFncm91cEluZm9SZXNwb25zZRJmChpncm91cF9yZXBseV9pbnZpdGVfcmVxdWVzdBgLIAEoCzInLnFhdWwucnBjLmdyb3VwLkdyb3VwUmVwbHlJbnZpdGVSZXF1ZXN0SABSF2dyb3VwUmVwbHlJbnZpdGVSZXF1ZXN0EmkKG2dyb3VwX3JlcGx5X2ludml0ZV9yZXNwb25zZRgMIAEoCzIoLnFhdWwucnBjLmdyb3VwLkdyb3VwUmVwbHlJbnZpdGVSZXNwb25zZUgAUhhncm91cFJlcGx5SW52aXRlUmVzcG9uc2USUAoSZ3JvdXBfbGlzdF9yZXF1ZXN0GA0gASgLMiAucWF1bC5ycGMuZ3JvdXAuR3JvdXBMaXN0UmVxdWVzdEgAUhBncm91cExpc3RSZXF1ZXN0ElMKE2dyb3VwX2xpc3RfcmVzcG9uc2UYDiABKAsyIS5xYXVsLnJwYy5ncm91cC5Hcm91cExpc3RSZXNwb25zZUgAUhFncm91cExpc3RSZXNwb25zZRJZChVncm91cF9pbnZpdGVkX3JlcXVlc3QYDyABKAsyIy5xYXVsLnJwYy5ncm91cC5Hcm91cEludml0ZWRSZXF1ZXN0SABSE2dyb3VwSW52aXRlZFJlcXVlc3QSXAoWZ3JvdXBfaW52aXRlZF9yZXNwb25zZRgQIAEoCzIkLnFhdWwucnBjLmdyb3VwLkdyb3VwSW52aXRlZFJlc3BvbnNlSABSFGdyb3VwSW52aXRlZFJlc3BvbnNlQgkKB21lc3NhZ2U=');
@$core.Deprecated('Use groupResultDescriptor instead')
const GroupResult$json = const {
  '1': 'GroupResult',
  '2': const [
    const {'1': 'status', '3': 1, '4': 1, '5': 8, '10': 'status'},
    const {'1': 'message', '3': 2, '4': 1, '5': 9, '10': 'message'},
  ],
};

/// Descriptor for `GroupResult`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupResultDescriptor = $convert.base64Decode('CgtHcm91cFJlc3VsdBIWCgZzdGF0dXMYASABKAhSBnN0YXR1cxIYCgdtZXNzYWdlGAIgASgJUgdtZXNzYWdl');
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
    const {'1': 'group_id', '3': 1, '4': 1, '5': 12, '10': 'groupId'},
    const {'1': 'result', '3': 2, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupResult', '10': 'result'},
  ],
};

/// Descriptor for `GroupCreateResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupCreateResponseDescriptor = $convert.base64Decode('ChNHcm91cENyZWF0ZVJlc3BvbnNlEhkKCGdyb3VwX2lkGAEgASgMUgdncm91cElkEjMKBnJlc3VsdBgCIAEoCzIbLnFhdWwucnBjLmdyb3VwLkdyb3VwUmVzdWx0UgZyZXN1bHQ=');
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
@$core.Deprecated('Use groupRenameResponseDescriptor instead')
const GroupRenameResponse$json = const {
  '1': 'GroupRenameResponse',
  '2': const [
    const {'1': 'group_id', '3': 1, '4': 1, '5': 12, '10': 'groupId'},
    const {'1': 'group_name', '3': 2, '4': 1, '5': 9, '10': 'groupName'},
    const {'1': 'result', '3': 3, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupResult', '10': 'result'},
  ],
};

/// Descriptor for `GroupRenameResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupRenameResponseDescriptor = $convert.base64Decode('ChNHcm91cFJlbmFtZVJlc3BvbnNlEhkKCGdyb3VwX2lkGAEgASgMUgdncm91cElkEh0KCmdyb3VwX25hbWUYAiABKAlSCWdyb3VwTmFtZRIzCgZyZXN1bHQYAyABKAsyGy5xYXVsLnJwYy5ncm91cC5Hcm91cFJlc3VsdFIGcmVzdWx0');
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
@$core.Deprecated('Use groupInviteMemberResponseDescriptor instead')
const GroupInviteMemberResponse$json = const {
  '1': 'GroupInviteMemberResponse',
  '2': const [
    const {'1': 'group_id', '3': 1, '4': 1, '5': 12, '10': 'groupId'},
    const {'1': 'user_id', '3': 2, '4': 1, '5': 12, '10': 'userId'},
    const {'1': 'result', '3': 3, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupResult', '10': 'result'},
  ],
};

/// Descriptor for `GroupInviteMemberResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupInviteMemberResponseDescriptor = $convert.base64Decode('ChlHcm91cEludml0ZU1lbWJlclJlc3BvbnNlEhkKCGdyb3VwX2lkGAEgASgMUgdncm91cElkEhcKB3VzZXJfaWQYAiABKAxSBnVzZXJJZBIzCgZyZXN1bHQYAyABKAsyGy5xYXVsLnJwYy5ncm91cC5Hcm91cFJlc3VsdFIGcmVzdWx0');
@$core.Deprecated('Use groupReplyInviteRequestDescriptor instead')
const GroupReplyInviteRequest$json = const {
  '1': 'GroupReplyInviteRequest',
  '2': const [
    const {'1': 'group_id', '3': 1, '4': 1, '5': 12, '10': 'groupId'},
    const {'1': 'accept', '3': 3, '4': 1, '5': 8, '10': 'accept'},
  ],
};

/// Descriptor for `GroupReplyInviteRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupReplyInviteRequestDescriptor = $convert.base64Decode('ChdHcm91cFJlcGx5SW52aXRlUmVxdWVzdBIZCghncm91cF9pZBgBIAEoDFIHZ3JvdXBJZBIWCgZhY2NlcHQYAyABKAhSBmFjY2VwdA==');
@$core.Deprecated('Use groupReplyInviteResponseDescriptor instead')
const GroupReplyInviteResponse$json = const {
  '1': 'GroupReplyInviteResponse',
  '2': const [
    const {'1': 'group_id', '3': 1, '4': 1, '5': 12, '10': 'groupId'},
    const {'1': 'result', '3': 3, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupResult', '10': 'result'},
  ],
};

/// Descriptor for `GroupReplyInviteResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupReplyInviteResponseDescriptor = $convert.base64Decode('ChhHcm91cFJlcGx5SW52aXRlUmVzcG9uc2USGQoIZ3JvdXBfaWQYASABKAxSB2dyb3VwSWQSMwoGcmVzdWx0GAMgASgLMhsucWF1bC5ycGMuZ3JvdXAuR3JvdXBSZXN1bHRSBnJlc3VsdA==');
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
@$core.Deprecated('Use groupRemoveMemberResponseDescriptor instead')
const GroupRemoveMemberResponse$json = const {
  '1': 'GroupRemoveMemberResponse',
  '2': const [
    const {'1': 'group_id', '3': 1, '4': 1, '5': 12, '10': 'groupId'},
    const {'1': 'user_id', '3': 2, '4': 1, '5': 12, '10': 'userId'},
    const {'1': 'result', '3': 3, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupResult', '10': 'result'},
  ],
};

/// Descriptor for `GroupRemoveMemberResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupRemoveMemberResponseDescriptor = $convert.base64Decode('ChlHcm91cFJlbW92ZU1lbWJlclJlc3BvbnNlEhkKCGdyb3VwX2lkGAEgASgMUgdncm91cElkEhcKB3VzZXJfaWQYAiABKAxSBnVzZXJJZBIzCgZyZXN1bHQYAyABKAsyGy5xYXVsLnJwYy5ncm91cC5Hcm91cFJlc3VsdFIGcmVzdWx0');
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
    const {'1': 'role', '3': 2, '4': 1, '5': 14, '6': '.qaul.rpc.group.GroupMemberRole', '10': 'role'},
    const {'1': 'joined_at', '3': 3, '4': 1, '5': 4, '10': 'joinedAt'},
    const {'1': 'state', '3': 4, '4': 1, '5': 14, '6': '.qaul.rpc.group.GroupMemberState', '10': 'state'},
    const {'1': 'last_message_index', '3': 5, '4': 1, '5': 13, '10': 'lastMessageIndex'},
  ],
};

/// Descriptor for `GroupMember`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupMemberDescriptor = $convert.base64Decode('CgtHcm91cE1lbWJlchIXCgd1c2VyX2lkGAEgASgMUgZ1c2VySWQSMwoEcm9sZRgCIAEoDjIfLnFhdWwucnBjLmdyb3VwLkdyb3VwTWVtYmVyUm9sZVIEcm9sZRIbCglqb2luZWRfYXQYAyABKARSCGpvaW5lZEF0EjYKBXN0YXRlGAQgASgOMiAucWF1bC5ycGMuZ3JvdXAuR3JvdXBNZW1iZXJTdGF0ZVIFc3RhdGUSLAoSbGFzdF9tZXNzYWdlX2luZGV4GAUgASgNUhBsYXN0TWVzc2FnZUluZGV4');
@$core.Deprecated('Use groupInfoDescriptor instead')
const GroupInfo$json = const {
  '1': 'GroupInfo',
  '2': const [
    const {'1': 'group_id', '3': 1, '4': 1, '5': 12, '10': 'groupId'},
    const {'1': 'group_name', '3': 2, '4': 1, '5': 9, '10': 'groupName'},
    const {'1': 'created_at', '3': 3, '4': 1, '5': 4, '10': 'createdAt'},
    const {'1': 'status', '3': 4, '4': 1, '5': 14, '6': '.qaul.rpc.group.GroupStatus', '10': 'status'},
    const {'1': 'revision', '3': 5, '4': 1, '5': 13, '10': 'revision'},
    const {'1': 'is_direct_chat', '3': 6, '4': 1, '5': 8, '10': 'isDirectChat'},
    const {'1': 'members', '3': 7, '4': 3, '5': 11, '6': '.qaul.rpc.group.GroupMember', '10': 'members'},
    const {'1': 'unread_messages', '3': 8, '4': 1, '5': 13, '10': 'unreadMessages'},
    const {'1': 'last_message_at', '3': 9, '4': 1, '5': 4, '10': 'lastMessageAt'},
    const {'1': 'last_message', '3': 10, '4': 1, '5': 12, '10': 'lastMessage'},
    const {'1': 'last_message_sender_id', '3': 11, '4': 1, '5': 12, '10': 'lastMessageSenderId'},
  ],
};

/// Descriptor for `GroupInfo`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupInfoDescriptor = $convert.base64Decode('CglHcm91cEluZm8SGQoIZ3JvdXBfaWQYASABKAxSB2dyb3VwSWQSHQoKZ3JvdXBfbmFtZRgCIAEoCVIJZ3JvdXBOYW1lEh0KCmNyZWF0ZWRfYXQYAyABKARSCWNyZWF0ZWRBdBIzCgZzdGF0dXMYBCABKA4yGy5xYXVsLnJwYy5ncm91cC5Hcm91cFN0YXR1c1IGc3RhdHVzEhoKCHJldmlzaW9uGAUgASgNUghyZXZpc2lvbhIkCg5pc19kaXJlY3RfY2hhdBgGIAEoCFIMaXNEaXJlY3RDaGF0EjUKB21lbWJlcnMYByADKAsyGy5xYXVsLnJwYy5ncm91cC5Hcm91cE1lbWJlclIHbWVtYmVycxInCg91bnJlYWRfbWVzc2FnZXMYCCABKA1SDnVucmVhZE1lc3NhZ2VzEiYKD2xhc3RfbWVzc2FnZV9hdBgJIAEoBFINbGFzdE1lc3NhZ2VBdBIhCgxsYXN0X21lc3NhZ2UYCiABKAxSC2xhc3RNZXNzYWdlEjMKFmxhc3RfbWVzc2FnZV9zZW5kZXJfaWQYCyABKAxSE2xhc3RNZXNzYWdlU2VuZGVySWQ=');
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
    const {'1': 'groups', '3': 1, '4': 3, '5': 11, '6': '.qaul.rpc.group.GroupInfo', '10': 'groups'},
  ],
};

/// Descriptor for `GroupListResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupListResponseDescriptor = $convert.base64Decode('ChFHcm91cExpc3RSZXNwb25zZRIxCgZncm91cHMYASADKAsyGS5xYXVsLnJwYy5ncm91cC5Hcm91cEluZm9SBmdyb3Vwcw==');
@$core.Deprecated('Use groupInvitedDescriptor instead')
const GroupInvited$json = const {
  '1': 'GroupInvited',
  '2': const [
    const {'1': 'sender_id', '3': 1, '4': 1, '5': 12, '10': 'senderId'},
    const {'1': 'received_at', '3': 2, '4': 1, '5': 4, '10': 'receivedAt'},
    const {'1': 'group', '3': 3, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupInfo', '10': 'group'},
  ],
};

/// Descriptor for `GroupInvited`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupInvitedDescriptor = $convert.base64Decode('CgxHcm91cEludml0ZWQSGwoJc2VuZGVyX2lkGAEgASgMUghzZW5kZXJJZBIfCgtyZWNlaXZlZF9hdBgCIAEoBFIKcmVjZWl2ZWRBdBIvCgVncm91cBgDIAEoCzIZLnFhdWwucnBjLmdyb3VwLkdyb3VwSW5mb1IFZ3JvdXA=');
@$core.Deprecated('Use groupInvitedRequestDescriptor instead')
const GroupInvitedRequest$json = const {
  '1': 'GroupInvitedRequest',
};

/// Descriptor for `GroupInvitedRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupInvitedRequestDescriptor = $convert.base64Decode('ChNHcm91cEludml0ZWRSZXF1ZXN0');
@$core.Deprecated('Use groupInvitedResponseDescriptor instead')
const GroupInvitedResponse$json = const {
  '1': 'GroupInvitedResponse',
  '2': const [
    const {'1': 'invited', '3': 1, '4': 3, '5': 11, '6': '.qaul.rpc.group.GroupInvited', '10': 'invited'},
  ],
};

/// Descriptor for `GroupInvitedResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupInvitedResponseDescriptor = $convert.base64Decode('ChRHcm91cEludml0ZWRSZXNwb25zZRI2CgdpbnZpdGVkGAEgAygLMhwucWF1bC5ycGMuZ3JvdXAuR3JvdXBJbnZpdGVkUgdpbnZpdGVk');
