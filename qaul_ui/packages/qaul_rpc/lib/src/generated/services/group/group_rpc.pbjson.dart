//
//  Generated code. Do not modify.
//  source: services/group/group_rpc.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

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
final $typed_data.Uint8List groupMemberRoleDescriptor = $convert.base64Decode(
    'Cg9Hcm91cE1lbWJlclJvbGUSCAoEVXNlchAAEgoKBUFkbWluEP8B');

@$core.Deprecated('Use groupStatusDescriptor instead')
const GroupStatus$json = {
  '1': 'GroupStatus',
  '2': [
    {'1': 'ACTIVE', '2': 0},
    {'1': 'INVITE_ACCEPTED', '2': 1},
    {'1': 'DEACTIVATED', '2': 2},
  ],
};

/// Descriptor for `GroupStatus`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List groupStatusDescriptor = $convert.base64Decode(
    'CgtHcm91cFN0YXR1cxIKCgZBQ1RJVkUQABITCg9JTlZJVEVfQUNDRVBURUQQARIPCgtERUFDVE'
    'lWQVRFRBAC');

@$core.Deprecated('Use groupDescriptor instead')
const Group$json = {
  '1': 'Group',
  '2': [
    {'1': 'group_create_request', '3': 1, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupCreateRequest', '9': 0, '10': 'groupCreateRequest'},
    {'1': 'group_create_response', '3': 2, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupCreateResponse', '9': 0, '10': 'groupCreateResponse'},
    {'1': 'group_rename_request', '3': 3, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupRenameRequest', '9': 0, '10': 'groupRenameRequest'},
    {'1': 'group_rename_response', '3': 4, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupRenameResponse', '9': 0, '10': 'groupRenameResponse'},
    {'1': 'group_invite_member_request', '3': 5, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupInviteMemberRequest', '9': 0, '10': 'groupInviteMemberRequest'},
    {'1': 'group_invite_member_response', '3': 6, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupInviteMemberResponse', '9': 0, '10': 'groupInviteMemberResponse'},
    {'1': 'group_remove_member_request', '3': 7, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupRemoveMemberRequest', '9': 0, '10': 'groupRemoveMemberRequest'},
    {'1': 'group_remove_member_response', '3': 8, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupRemoveMemberResponse', '9': 0, '10': 'groupRemoveMemberResponse'},
    {'1': 'group_info_request', '3': 9, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupInfoRequest', '9': 0, '10': 'groupInfoRequest'},
    {'1': 'group_info_response', '3': 10, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupInfo', '9': 0, '10': 'groupInfoResponse'},
    {'1': 'group_reply_invite_request', '3': 11, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupReplyInviteRequest', '9': 0, '10': 'groupReplyInviteRequest'},
    {'1': 'group_reply_invite_response', '3': 12, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupReplyInviteResponse', '9': 0, '10': 'groupReplyInviteResponse'},
    {'1': 'group_list_request', '3': 13, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupListRequest', '9': 0, '10': 'groupListRequest'},
    {'1': 'group_list_response', '3': 14, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupListResponse', '9': 0, '10': 'groupListResponse'},
    {'1': 'group_invited_request', '3': 15, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupInvitedRequest', '9': 0, '10': 'groupInvitedRequest'},
    {'1': 'group_invited_response', '3': 16, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupInvitedResponse', '9': 0, '10': 'groupInvitedResponse'},
  ],
  '8': [
    {'1': 'message'},
  ],
};

/// Descriptor for `Group`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupDescriptor = $convert.base64Decode(
    'CgVHcm91cBJWChRncm91cF9jcmVhdGVfcmVxdWVzdBgBIAEoCzIiLnFhdWwucnBjLmdyb3VwLk'
    'dyb3VwQ3JlYXRlUmVxdWVzdEgAUhJncm91cENyZWF0ZVJlcXVlc3QSWQoVZ3JvdXBfY3JlYXRl'
    'X3Jlc3BvbnNlGAIgASgLMiMucWF1bC5ycGMuZ3JvdXAuR3JvdXBDcmVhdGVSZXNwb25zZUgAUh'
    'Nncm91cENyZWF0ZVJlc3BvbnNlElYKFGdyb3VwX3JlbmFtZV9yZXF1ZXN0GAMgASgLMiIucWF1'
    'bC5ycGMuZ3JvdXAuR3JvdXBSZW5hbWVSZXF1ZXN0SABSEmdyb3VwUmVuYW1lUmVxdWVzdBJZCh'
    'Vncm91cF9yZW5hbWVfcmVzcG9uc2UYBCABKAsyIy5xYXVsLnJwYy5ncm91cC5Hcm91cFJlbmFt'
    'ZVJlc3BvbnNlSABSE2dyb3VwUmVuYW1lUmVzcG9uc2USaQobZ3JvdXBfaW52aXRlX21lbWJlcl'
    '9yZXF1ZXN0GAUgASgLMigucWF1bC5ycGMuZ3JvdXAuR3JvdXBJbnZpdGVNZW1iZXJSZXF1ZXN0'
    'SABSGGdyb3VwSW52aXRlTWVtYmVyUmVxdWVzdBJsChxncm91cF9pbnZpdGVfbWVtYmVyX3Jlc3'
    'BvbnNlGAYgASgLMikucWF1bC5ycGMuZ3JvdXAuR3JvdXBJbnZpdGVNZW1iZXJSZXNwb25zZUgA'
    'Uhlncm91cEludml0ZU1lbWJlclJlc3BvbnNlEmkKG2dyb3VwX3JlbW92ZV9tZW1iZXJfcmVxdW'
    'VzdBgHIAEoCzIoLnFhdWwucnBjLmdyb3VwLkdyb3VwUmVtb3ZlTWVtYmVyUmVxdWVzdEgAUhhn'
    'cm91cFJlbW92ZU1lbWJlclJlcXVlc3QSbAocZ3JvdXBfcmVtb3ZlX21lbWJlcl9yZXNwb25zZR'
    'gIIAEoCzIpLnFhdWwucnBjLmdyb3VwLkdyb3VwUmVtb3ZlTWVtYmVyUmVzcG9uc2VIAFIZZ3Jv'
    'dXBSZW1vdmVNZW1iZXJSZXNwb25zZRJQChJncm91cF9pbmZvX3JlcXVlc3QYCSABKAsyIC5xYX'
    'VsLnJwYy5ncm91cC5Hcm91cEluZm9SZXF1ZXN0SABSEGdyb3VwSW5mb1JlcXVlc3QSSwoTZ3Jv'
    'dXBfaW5mb19yZXNwb25zZRgKIAEoCzIZLnFhdWwucnBjLmdyb3VwLkdyb3VwSW5mb0gAUhFncm'
    '91cEluZm9SZXNwb25zZRJmChpncm91cF9yZXBseV9pbnZpdGVfcmVxdWVzdBgLIAEoCzInLnFh'
    'dWwucnBjLmdyb3VwLkdyb3VwUmVwbHlJbnZpdGVSZXF1ZXN0SABSF2dyb3VwUmVwbHlJbnZpdG'
    'VSZXF1ZXN0EmkKG2dyb3VwX3JlcGx5X2ludml0ZV9yZXNwb25zZRgMIAEoCzIoLnFhdWwucnBj'
    'Lmdyb3VwLkdyb3VwUmVwbHlJbnZpdGVSZXNwb25zZUgAUhhncm91cFJlcGx5SW52aXRlUmVzcG'
    '9uc2USUAoSZ3JvdXBfbGlzdF9yZXF1ZXN0GA0gASgLMiAucWF1bC5ycGMuZ3JvdXAuR3JvdXBM'
    'aXN0UmVxdWVzdEgAUhBncm91cExpc3RSZXF1ZXN0ElMKE2dyb3VwX2xpc3RfcmVzcG9uc2UYDi'
    'ABKAsyIS5xYXVsLnJwYy5ncm91cC5Hcm91cExpc3RSZXNwb25zZUgAUhFncm91cExpc3RSZXNw'
    'b25zZRJZChVncm91cF9pbnZpdGVkX3JlcXVlc3QYDyABKAsyIy5xYXVsLnJwYy5ncm91cC5Hcm'
    '91cEludml0ZWRSZXF1ZXN0SABSE2dyb3VwSW52aXRlZFJlcXVlc3QSXAoWZ3JvdXBfaW52aXRl'
    'ZF9yZXNwb25zZRgQIAEoCzIkLnFhdWwucnBjLmdyb3VwLkdyb3VwSW52aXRlZFJlc3BvbnNlSA'
    'BSFGdyb3VwSW52aXRlZFJlc3BvbnNlQgkKB21lc3NhZ2U=');

@$core.Deprecated('Use groupResultDescriptor instead')
const GroupResult$json = {
  '1': 'GroupResult',
  '2': [
    {'1': 'status', '3': 1, '4': 1, '5': 8, '10': 'status'},
    {'1': 'message', '3': 2, '4': 1, '5': 9, '10': 'message'},
  ],
};

/// Descriptor for `GroupResult`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupResultDescriptor = $convert.base64Decode(
    'CgtHcm91cFJlc3VsdBIWCgZzdGF0dXMYASABKAhSBnN0YXR1cxIYCgdtZXNzYWdlGAIgASgJUg'
    'dtZXNzYWdl');

@$core.Deprecated('Use groupCreateRequestDescriptor instead')
const GroupCreateRequest$json = {
  '1': 'GroupCreateRequest',
  '2': [
    {'1': 'group_name', '3': 1, '4': 1, '5': 9, '10': 'groupName'},
  ],
};

/// Descriptor for `GroupCreateRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupCreateRequestDescriptor = $convert.base64Decode(
    'ChJHcm91cENyZWF0ZVJlcXVlc3QSHQoKZ3JvdXBfbmFtZRgBIAEoCVIJZ3JvdXBOYW1l');

@$core.Deprecated('Use groupCreateResponseDescriptor instead')
const GroupCreateResponse$json = {
  '1': 'GroupCreateResponse',
  '2': [
    {'1': 'group_id', '3': 1, '4': 1, '5': 12, '10': 'groupId'},
    {'1': 'result', '3': 2, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupResult', '10': 'result'},
  ],
};

/// Descriptor for `GroupCreateResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupCreateResponseDescriptor = $convert.base64Decode(
    'ChNHcm91cENyZWF0ZVJlc3BvbnNlEhkKCGdyb3VwX2lkGAEgASgMUgdncm91cElkEjMKBnJlc3'
    'VsdBgCIAEoCzIbLnFhdWwucnBjLmdyb3VwLkdyb3VwUmVzdWx0UgZyZXN1bHQ=');

@$core.Deprecated('Use groupRenameRequestDescriptor instead')
const GroupRenameRequest$json = {
  '1': 'GroupRenameRequest',
  '2': [
    {'1': 'group_id', '3': 1, '4': 1, '5': 12, '10': 'groupId'},
    {'1': 'group_name', '3': 2, '4': 1, '5': 9, '10': 'groupName'},
  ],
};

/// Descriptor for `GroupRenameRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupRenameRequestDescriptor = $convert.base64Decode(
    'ChJHcm91cFJlbmFtZVJlcXVlc3QSGQoIZ3JvdXBfaWQYASABKAxSB2dyb3VwSWQSHQoKZ3JvdX'
    'BfbmFtZRgCIAEoCVIJZ3JvdXBOYW1l');

@$core.Deprecated('Use groupRenameResponseDescriptor instead')
const GroupRenameResponse$json = {
  '1': 'GroupRenameResponse',
  '2': [
    {'1': 'group_id', '3': 1, '4': 1, '5': 12, '10': 'groupId'},
    {'1': 'group_name', '3': 2, '4': 1, '5': 9, '10': 'groupName'},
    {'1': 'result', '3': 3, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupResult', '10': 'result'},
  ],
};

/// Descriptor for `GroupRenameResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupRenameResponseDescriptor = $convert.base64Decode(
    'ChNHcm91cFJlbmFtZVJlc3BvbnNlEhkKCGdyb3VwX2lkGAEgASgMUgdncm91cElkEh0KCmdyb3'
    'VwX25hbWUYAiABKAlSCWdyb3VwTmFtZRIzCgZyZXN1bHQYAyABKAsyGy5xYXVsLnJwYy5ncm91'
    'cC5Hcm91cFJlc3VsdFIGcmVzdWx0');

@$core.Deprecated('Use groupInviteMemberRequestDescriptor instead')
const GroupInviteMemberRequest$json = {
  '1': 'GroupInviteMemberRequest',
  '2': [
    {'1': 'group_id', '3': 1, '4': 1, '5': 12, '10': 'groupId'},
    {'1': 'user_id', '3': 2, '4': 1, '5': 12, '10': 'userId'},
  ],
};

/// Descriptor for `GroupInviteMemberRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupInviteMemberRequestDescriptor = $convert.base64Decode(
    'ChhHcm91cEludml0ZU1lbWJlclJlcXVlc3QSGQoIZ3JvdXBfaWQYASABKAxSB2dyb3VwSWQSFw'
    'oHdXNlcl9pZBgCIAEoDFIGdXNlcklk');

@$core.Deprecated('Use groupInviteMemberResponseDescriptor instead')
const GroupInviteMemberResponse$json = {
  '1': 'GroupInviteMemberResponse',
  '2': [
    {'1': 'group_id', '3': 1, '4': 1, '5': 12, '10': 'groupId'},
    {'1': 'user_id', '3': 2, '4': 1, '5': 12, '10': 'userId'},
    {'1': 'result', '3': 3, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupResult', '10': 'result'},
  ],
};

/// Descriptor for `GroupInviteMemberResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupInviteMemberResponseDescriptor = $convert.base64Decode(
    'ChlHcm91cEludml0ZU1lbWJlclJlc3BvbnNlEhkKCGdyb3VwX2lkGAEgASgMUgdncm91cElkEh'
    'cKB3VzZXJfaWQYAiABKAxSBnVzZXJJZBIzCgZyZXN1bHQYAyABKAsyGy5xYXVsLnJwYy5ncm91'
    'cC5Hcm91cFJlc3VsdFIGcmVzdWx0');

@$core.Deprecated('Use groupReplyInviteRequestDescriptor instead')
const GroupReplyInviteRequest$json = {
  '1': 'GroupReplyInviteRequest',
  '2': [
    {'1': 'group_id', '3': 1, '4': 1, '5': 12, '10': 'groupId'},
    {'1': 'accept', '3': 3, '4': 1, '5': 8, '10': 'accept'},
  ],
};

/// Descriptor for `GroupReplyInviteRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupReplyInviteRequestDescriptor = $convert.base64Decode(
    'ChdHcm91cFJlcGx5SW52aXRlUmVxdWVzdBIZCghncm91cF9pZBgBIAEoDFIHZ3JvdXBJZBIWCg'
    'ZhY2NlcHQYAyABKAhSBmFjY2VwdA==');

@$core.Deprecated('Use groupReplyInviteResponseDescriptor instead')
const GroupReplyInviteResponse$json = {
  '1': 'GroupReplyInviteResponse',
  '2': [
    {'1': 'group_id', '3': 1, '4': 1, '5': 12, '10': 'groupId'},
    {'1': 'result', '3': 3, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupResult', '10': 'result'},
  ],
};

/// Descriptor for `GroupReplyInviteResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupReplyInviteResponseDescriptor = $convert.base64Decode(
    'ChhHcm91cFJlcGx5SW52aXRlUmVzcG9uc2USGQoIZ3JvdXBfaWQYASABKAxSB2dyb3VwSWQSMw'
    'oGcmVzdWx0GAMgASgLMhsucWF1bC5ycGMuZ3JvdXAuR3JvdXBSZXN1bHRSBnJlc3VsdA==');

@$core.Deprecated('Use groupRemoveMemberRequestDescriptor instead')
const GroupRemoveMemberRequest$json = {
  '1': 'GroupRemoveMemberRequest',
  '2': [
    {'1': 'group_id', '3': 1, '4': 1, '5': 12, '10': 'groupId'},
    {'1': 'user_id', '3': 2, '4': 1, '5': 12, '10': 'userId'},
  ],
};

/// Descriptor for `GroupRemoveMemberRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupRemoveMemberRequestDescriptor = $convert.base64Decode(
    'ChhHcm91cFJlbW92ZU1lbWJlclJlcXVlc3QSGQoIZ3JvdXBfaWQYASABKAxSB2dyb3VwSWQSFw'
    'oHdXNlcl9pZBgCIAEoDFIGdXNlcklk');

@$core.Deprecated('Use groupRemoveMemberResponseDescriptor instead')
const GroupRemoveMemberResponse$json = {
  '1': 'GroupRemoveMemberResponse',
  '2': [
    {'1': 'group_id', '3': 1, '4': 1, '5': 12, '10': 'groupId'},
    {'1': 'user_id', '3': 2, '4': 1, '5': 12, '10': 'userId'},
    {'1': 'result', '3': 3, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupResult', '10': 'result'},
  ],
};

/// Descriptor for `GroupRemoveMemberResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupRemoveMemberResponseDescriptor = $convert.base64Decode(
    'ChlHcm91cFJlbW92ZU1lbWJlclJlc3BvbnNlEhkKCGdyb3VwX2lkGAEgASgMUgdncm91cElkEh'
    'cKB3VzZXJfaWQYAiABKAxSBnVzZXJJZBIzCgZyZXN1bHQYAyABKAsyGy5xYXVsLnJwYy5ncm91'
    'cC5Hcm91cFJlc3VsdFIGcmVzdWx0');

@$core.Deprecated('Use groupInfoRequestDescriptor instead')
const GroupInfoRequest$json = {
  '1': 'GroupInfoRequest',
  '2': [
    {'1': 'group_id', '3': 1, '4': 1, '5': 12, '10': 'groupId'},
  ],
};

/// Descriptor for `GroupInfoRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupInfoRequestDescriptor = $convert.base64Decode(
    'ChBHcm91cEluZm9SZXF1ZXN0EhkKCGdyb3VwX2lkGAEgASgMUgdncm91cElk');

@$core.Deprecated('Use groupMemberDescriptor instead')
const GroupMember$json = {
  '1': 'GroupMember',
  '2': [
    {'1': 'user_id', '3': 1, '4': 1, '5': 12, '10': 'userId'},
    {'1': 'role', '3': 2, '4': 1, '5': 14, '6': '.qaul.rpc.group.GroupMemberRole', '10': 'role'},
    {'1': 'joined_at', '3': 3, '4': 1, '5': 4, '10': 'joinedAt'},
    {'1': 'state', '3': 4, '4': 1, '5': 14, '6': '.qaul.rpc.group.GroupMemberState', '10': 'state'},
    {'1': 'last_message_index', '3': 5, '4': 1, '5': 13, '10': 'lastMessageIndex'},
  ],
};

/// Descriptor for `GroupMember`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupMemberDescriptor = $convert.base64Decode(
    'CgtHcm91cE1lbWJlchIXCgd1c2VyX2lkGAEgASgMUgZ1c2VySWQSMwoEcm9sZRgCIAEoDjIfLn'
    'FhdWwucnBjLmdyb3VwLkdyb3VwTWVtYmVyUm9sZVIEcm9sZRIbCglqb2luZWRfYXQYAyABKARS'
    'CGpvaW5lZEF0EjYKBXN0YXRlGAQgASgOMiAucWF1bC5ycGMuZ3JvdXAuR3JvdXBNZW1iZXJTdG'
    'F0ZVIFc3RhdGUSLAoSbGFzdF9tZXNzYWdlX2luZGV4GAUgASgNUhBsYXN0TWVzc2FnZUluZGV4');

@$core.Deprecated('Use groupInfoDescriptor instead')
const GroupInfo$json = {
  '1': 'GroupInfo',
  '2': [
    {'1': 'group_id', '3': 1, '4': 1, '5': 12, '10': 'groupId'},
    {'1': 'group_name', '3': 2, '4': 1, '5': 9, '10': 'groupName'},
    {'1': 'created_at', '3': 3, '4': 1, '5': 4, '10': 'createdAt'},
    {'1': 'status', '3': 4, '4': 1, '5': 14, '6': '.qaul.rpc.group.GroupStatus', '10': 'status'},
    {'1': 'revision', '3': 5, '4': 1, '5': 13, '10': 'revision'},
    {'1': 'is_direct_chat', '3': 6, '4': 1, '5': 8, '10': 'isDirectChat'},
    {'1': 'members', '3': 7, '4': 3, '5': 11, '6': '.qaul.rpc.group.GroupMember', '10': 'members'},
    {'1': 'unread_messages', '3': 8, '4': 1, '5': 13, '10': 'unreadMessages'},
    {'1': 'last_message_at', '3': 9, '4': 1, '5': 4, '10': 'lastMessageAt'},
    {'1': 'last_message', '3': 10, '4': 1, '5': 12, '10': 'lastMessage'},
    {'1': 'last_message_sender_id', '3': 11, '4': 1, '5': 12, '10': 'lastMessageSenderId'},
  ],
};

/// Descriptor for `GroupInfo`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupInfoDescriptor = $convert.base64Decode(
    'CglHcm91cEluZm8SGQoIZ3JvdXBfaWQYASABKAxSB2dyb3VwSWQSHQoKZ3JvdXBfbmFtZRgCIA'
    'EoCVIJZ3JvdXBOYW1lEh0KCmNyZWF0ZWRfYXQYAyABKARSCWNyZWF0ZWRBdBIzCgZzdGF0dXMY'
    'BCABKA4yGy5xYXVsLnJwYy5ncm91cC5Hcm91cFN0YXR1c1IGc3RhdHVzEhoKCHJldmlzaW9uGA'
    'UgASgNUghyZXZpc2lvbhIkCg5pc19kaXJlY3RfY2hhdBgGIAEoCFIMaXNEaXJlY3RDaGF0EjUK'
    'B21lbWJlcnMYByADKAsyGy5xYXVsLnJwYy5ncm91cC5Hcm91cE1lbWJlclIHbWVtYmVycxInCg'
    '91bnJlYWRfbWVzc2FnZXMYCCABKA1SDnVucmVhZE1lc3NhZ2VzEiYKD2xhc3RfbWVzc2FnZV9h'
    'dBgJIAEoBFINbGFzdE1lc3NhZ2VBdBIhCgxsYXN0X21lc3NhZ2UYCiABKAxSC2xhc3RNZXNzYW'
    'dlEjMKFmxhc3RfbWVzc2FnZV9zZW5kZXJfaWQYCyABKAxSE2xhc3RNZXNzYWdlU2VuZGVySWQ=');

@$core.Deprecated('Use groupListRequestDescriptor instead')
const GroupListRequest$json = {
  '1': 'GroupListRequest',
};

/// Descriptor for `GroupListRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupListRequestDescriptor = $convert.base64Decode(
    'ChBHcm91cExpc3RSZXF1ZXN0');

@$core.Deprecated('Use groupListResponseDescriptor instead')
const GroupListResponse$json = {
  '1': 'GroupListResponse',
  '2': [
    {'1': 'groups', '3': 1, '4': 3, '5': 11, '6': '.qaul.rpc.group.GroupInfo', '10': 'groups'},
  ],
};

/// Descriptor for `GroupListResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupListResponseDescriptor = $convert.base64Decode(
    'ChFHcm91cExpc3RSZXNwb25zZRIxCgZncm91cHMYASADKAsyGS5xYXVsLnJwYy5ncm91cC5Hcm'
    '91cEluZm9SBmdyb3Vwcw==');

@$core.Deprecated('Use groupInvitedDescriptor instead')
const GroupInvited$json = {
  '1': 'GroupInvited',
  '2': [
    {'1': 'sender_id', '3': 1, '4': 1, '5': 12, '10': 'senderId'},
    {'1': 'received_at', '3': 2, '4': 1, '5': 4, '10': 'receivedAt'},
    {'1': 'group', '3': 3, '4': 1, '5': 11, '6': '.qaul.rpc.group.GroupInfo', '10': 'group'},
  ],
};

/// Descriptor for `GroupInvited`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupInvitedDescriptor = $convert.base64Decode(
    'CgxHcm91cEludml0ZWQSGwoJc2VuZGVyX2lkGAEgASgMUghzZW5kZXJJZBIfCgtyZWNlaXZlZF'
    '9hdBgCIAEoBFIKcmVjZWl2ZWRBdBIvCgVncm91cBgDIAEoCzIZLnFhdWwucnBjLmdyb3VwLkdy'
    'b3VwSW5mb1IFZ3JvdXA=');

@$core.Deprecated('Use groupInvitedRequestDescriptor instead')
const GroupInvitedRequest$json = {
  '1': 'GroupInvitedRequest',
};

/// Descriptor for `GroupInvitedRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupInvitedRequestDescriptor = $convert.base64Decode(
    'ChNHcm91cEludml0ZWRSZXF1ZXN0');

@$core.Deprecated('Use groupInvitedResponseDescriptor instead')
const GroupInvitedResponse$json = {
  '1': 'GroupInvitedResponse',
  '2': [
    {'1': 'invited', '3': 1, '4': 3, '5': 11, '6': '.qaul.rpc.group.GroupInvited', '10': 'invited'},
  ],
};

/// Descriptor for `GroupInvitedResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupInvitedResponseDescriptor = $convert.base64Decode(
    'ChRHcm91cEludml0ZWRSZXNwb25zZRI2CgdpbnZpdGVkGAEgAygLMhwucWF1bC5ycGMuZ3JvdX'
    'AuR3JvdXBJbnZpdGVkUgdpbnZpdGVk');

