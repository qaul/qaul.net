//
//  Generated code. Do not modify.
//  source: node/user_accounts.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use userAccountsDescriptor instead')
const UserAccounts$json = {
  '1': 'UserAccounts',
  '2': [
    {'1': 'get_default_user_account', '3': 1, '4': 1, '5': 8, '9': 0, '10': 'getDefaultUserAccount'},
    {'1': 'create_user_account', '3': 2, '4': 1, '5': 11, '6': '.qaul.rpc.user_accounts.CreateUserAccount', '9': 0, '10': 'createUserAccount'},
    {'1': 'default_user_account', '3': 3, '4': 1, '5': 11, '6': '.qaul.rpc.user_accounts.DefaultUserAccount', '9': 0, '10': 'defaultUserAccount'},
    {'1': 'my_user_account', '3': 4, '4': 1, '5': 11, '6': '.qaul.rpc.user_accounts.MyUserAccount', '9': 0, '10': 'myUserAccount'},
  ],
  '8': [
    {'1': 'message'},
  ],
};

/// Descriptor for `UserAccounts`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List userAccountsDescriptor = $convert.base64Decode(
    'CgxVc2VyQWNjb3VudHMSOQoYZ2V0X2RlZmF1bHRfdXNlcl9hY2NvdW50GAEgASgISABSFWdldE'
    'RlZmF1bHRVc2VyQWNjb3VudBJbChNjcmVhdGVfdXNlcl9hY2NvdW50GAIgASgLMikucWF1bC5y'
    'cGMudXNlcl9hY2NvdW50cy5DcmVhdGVVc2VyQWNjb3VudEgAUhFjcmVhdGVVc2VyQWNjb3VudB'
    'JeChRkZWZhdWx0X3VzZXJfYWNjb3VudBgDIAEoCzIqLnFhdWwucnBjLnVzZXJfYWNjb3VudHMu'
    'RGVmYXVsdFVzZXJBY2NvdW50SABSEmRlZmF1bHRVc2VyQWNjb3VudBJPCg9teV91c2VyX2FjY2'
    '91bnQYBCABKAsyJS5xYXVsLnJwYy51c2VyX2FjY291bnRzLk15VXNlckFjY291bnRIAFINbXlV'
    'c2VyQWNjb3VudEIJCgdtZXNzYWdl');

@$core.Deprecated('Use createUserAccountDescriptor instead')
const CreateUserAccount$json = {
  '1': 'CreateUserAccount',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 9, '10': 'name'},
  ],
};

/// Descriptor for `CreateUserAccount`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createUserAccountDescriptor = $convert.base64Decode(
    'ChFDcmVhdGVVc2VyQWNjb3VudBISCgRuYW1lGAEgASgJUgRuYW1l');

@$core.Deprecated('Use defaultUserAccountDescriptor instead')
const DefaultUserAccount$json = {
  '1': 'DefaultUserAccount',
  '2': [
    {'1': 'user_account_exists', '3': 1, '4': 1, '5': 8, '10': 'userAccountExists'},
    {'1': 'my_user_account', '3': 2, '4': 1, '5': 11, '6': '.qaul.rpc.user_accounts.MyUserAccount', '10': 'myUserAccount'},
  ],
};

/// Descriptor for `DefaultUserAccount`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List defaultUserAccountDescriptor = $convert.base64Decode(
    'ChJEZWZhdWx0VXNlckFjY291bnQSLgoTdXNlcl9hY2NvdW50X2V4aXN0cxgBIAEoCFIRdXNlck'
    'FjY291bnRFeGlzdHMSTQoPbXlfdXNlcl9hY2NvdW50GAIgASgLMiUucWF1bC5ycGMudXNlcl9h'
    'Y2NvdW50cy5NeVVzZXJBY2NvdW50Ug1teVVzZXJBY2NvdW50');

@$core.Deprecated('Use myUserAccountDescriptor instead')
const MyUserAccount$json = {
  '1': 'MyUserAccount',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 9, '10': 'name'},
    {'1': 'id', '3': 2, '4': 1, '5': 12, '10': 'id'},
    {'1': 'id_base58', '3': 3, '4': 1, '5': 9, '10': 'idBase58'},
    {'1': 'key', '3': 4, '4': 1, '5': 12, '10': 'key'},
    {'1': 'key_type', '3': 5, '4': 1, '5': 9, '10': 'keyType'},
    {'1': 'key_base58', '3': 6, '4': 1, '5': 9, '10': 'keyBase58'},
  ],
};

/// Descriptor for `MyUserAccount`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List myUserAccountDescriptor = $convert.base64Decode(
    'Cg1NeVVzZXJBY2NvdW50EhIKBG5hbWUYASABKAlSBG5hbWUSDgoCaWQYAiABKAxSAmlkEhsKCW'
    'lkX2Jhc2U1OBgDIAEoCVIIaWRCYXNlNTgSEAoDa2V5GAQgASgMUgNrZXkSGQoIa2V5X3R5cGUY'
    'BSABKAlSB2tleVR5cGUSHQoKa2V5X2Jhc2U1OBgGIAEoCVIJa2V5QmFzZTU4');

