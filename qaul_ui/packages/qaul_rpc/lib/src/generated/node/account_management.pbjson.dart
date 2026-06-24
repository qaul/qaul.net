// This is a generated file - do not edit.
//
// Generated from node/account_management.proto.

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

import '../common/common.pbjson.dart' as $0;

@$core.Deprecated('Use accountManagementDescriptor instead')
const AccountManagement$json = {
  '1': 'AccountManagement',
  '2': [
    {
      '1': 'export_account_request',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.qaul.rpc.account_management.ExportAccountRequest',
      '9': 0,
      '10': 'exportAccountRequest'
    },
    {
      '1': 'export_account_response',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.qaul.rpc.account_management.ExportAccountResponse',
      '9': 0,
      '10': 'exportAccountResponse'
    },
    {
      '1': 'delete_account_request',
      '3': 3,
      '4': 1,
      '5': 11,
      '6': '.qaul.rpc.account_management.DeleteAccountRequest',
      '9': 0,
      '10': 'deleteAccountRequest'
    },
    {
      '1': 'restore_account_request',
      '3': 4,
      '4': 1,
      '5': 11,
      '6': '.qaul.rpc.account_management.RestoreAccountRequest',
      '9': 0,
      '10': 'restoreAccountRequest'
    },
    {
      '1': 'restore_account_response',
      '3': 5,
      '4': 1,
      '5': 11,
      '6': '.qaul.rpc.account_management.RestoreAccountResponse',
      '9': 0,
      '10': 'restoreAccountResponse'
    },
    {
      '1': 'ack',
      '3': 6,
      '4': 1,
      '5': 11,
      '6': '.qaul.common.Ack',
      '9': 0,
      '10': 'ack'
    },
    {
      '1': 'error',
      '3': 7,
      '4': 1,
      '5': 11,
      '6': '.qaul.common.RpcError',
      '9': 0,
      '10': 'error'
    },
  ],
  '8': [
    {'1': 'message'},
  ],
};

/// Descriptor for `AccountManagement`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List accountManagementDescriptor = $convert.base64Decode(
    'ChFBY2NvdW50TWFuYWdlbWVudBJpChZleHBvcnRfYWNjb3VudF9yZXF1ZXN0GAEgASgLMjEucW'
    'F1bC5ycGMuYWNjb3VudF9tYW5hZ2VtZW50LkV4cG9ydEFjY291bnRSZXF1ZXN0SABSFGV4cG9y'
    'dEFjY291bnRSZXF1ZXN0EmwKF2V4cG9ydF9hY2NvdW50X3Jlc3BvbnNlGAIgASgLMjIucWF1bC'
    '5ycGMuYWNjb3VudF9tYW5hZ2VtZW50LkV4cG9ydEFjY291bnRSZXNwb25zZUgAUhVleHBvcnRB'
    'Y2NvdW50UmVzcG9uc2USaQoWZGVsZXRlX2FjY291bnRfcmVxdWVzdBgDIAEoCzIxLnFhdWwucn'
    'BjLmFjY291bnRfbWFuYWdlbWVudC5EZWxldGVBY2NvdW50UmVxdWVzdEgAUhRkZWxldGVBY2Nv'
    'dW50UmVxdWVzdBJsChdyZXN0b3JlX2FjY291bnRfcmVxdWVzdBgEIAEoCzIyLnFhdWwucnBjLm'
    'FjY291bnRfbWFuYWdlbWVudC5SZXN0b3JlQWNjb3VudFJlcXVlc3RIAFIVcmVzdG9yZUFjY291'
    'bnRSZXF1ZXN0Em8KGHJlc3RvcmVfYWNjb3VudF9yZXNwb25zZRgFIAEoCzIzLnFhdWwucnBjLm'
    'FjY291bnRfbWFuYWdlbWVudC5SZXN0b3JlQWNjb3VudFJlc3BvbnNlSABSFnJlc3RvcmVBY2Nv'
    'dW50UmVzcG9uc2USJAoDYWNrGAYgASgLMhAucWF1bC5jb21tb24uQWNrSABSA2FjaxItCgVlcn'
    'JvchgHIAEoCzIVLnFhdWwuY29tbW9uLlJwY0Vycm9ySABSBWVycm9yQgkKB21lc3NhZ2U=');

@$core.Deprecated('Use exportAccountRequestDescriptor instead')
const ExportAccountRequest$json = {
  '1': 'ExportAccountRequest',
  '2': [
    {'1': 'output_path', '3': 1, '4': 1, '5': 9, '10': 'outputPath'},
  ],
};

/// Descriptor for `ExportAccountRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List exportAccountRequestDescriptor = $convert.base64Decode(
    'ChRFeHBvcnRBY2NvdW50UmVxdWVzdBIfCgtvdXRwdXRfcGF0aBgBIAEoCVIKb3V0cHV0UGF0aA'
    '==');

@$core.Deprecated('Use exportAccountResponseDescriptor instead')
const ExportAccountResponse$json = {
  '1': 'ExportAccountResponse',
  '2': [
    {'1': 'path', '3': 1, '4': 1, '5': 9, '10': 'path'},
  ],
};

/// Descriptor for `ExportAccountResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List exportAccountResponseDescriptor =
    $convert.base64Decode(
        'ChVFeHBvcnRBY2NvdW50UmVzcG9uc2USEgoEcGF0aBgBIAEoCVIEcGF0aA==');

@$core.Deprecated('Use deleteAccountRequestDescriptor instead')
const DeleteAccountRequest$json = {
  '1': 'DeleteAccountRequest',
};

/// Descriptor for `DeleteAccountRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deleteAccountRequestDescriptor =
    $convert.base64Decode('ChREZWxldGVBY2NvdW50UmVxdWVzdA==');

@$core.Deprecated('Use restoreAccountRequestDescriptor instead')
const RestoreAccountRequest$json = {
  '1': 'RestoreAccountRequest',
  '2': [
    {'1': 'archive_path', '3': 1, '4': 1, '5': 9, '10': 'archivePath'},
  ],
};

/// Descriptor for `RestoreAccountRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List restoreAccountRequestDescriptor = $convert.base64Decode(
    'ChVSZXN0b3JlQWNjb3VudFJlcXVlc3QSIQoMYXJjaGl2ZV9wYXRoGAEgASgJUgthcmNoaXZlUG'
    'F0aA==');

@$core.Deprecated('Use restoreAccountResponseDescriptor instead')
const RestoreAccountResponse$json = {
  '1': 'RestoreAccountResponse',
  '2': [
    {'1': 'user_id', '3': 1, '4': 1, '5': 12, '10': 'userId'},
    {'1': 'user_id_base58', '3': 2, '4': 1, '5': 9, '10': 'userIdBase58'},
  ],
};

/// Descriptor for `RestoreAccountResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List restoreAccountResponseDescriptor =
    $convert.base64Decode(
        'ChZSZXN0b3JlQWNjb3VudFJlc3BvbnNlEhcKB3VzZXJfaWQYASABKAxSBnVzZXJJZBIkCg51c2'
        'VyX2lkX2Jhc2U1OBgCIAEoCVIMdXNlcklkQmFzZTU4');

const $core.Map<$core.String, $core.dynamic> AccountManagementServiceBase$json =
    {
  '1': 'AccountManagementService',
  '2': [
    {
      '1': 'Export',
      '2': '.qaul.rpc.account_management.ExportAccountRequest',
      '3': '.qaul.rpc.account_management.ExportAccountResponse'
    },
    {
      '1': 'Delete',
      '2': '.qaul.rpc.account_management.DeleteAccountRequest',
      '3': '.qaul.common.Ack'
    },
    {
      '1': 'Restore',
      '2': '.qaul.rpc.account_management.RestoreAccountRequest',
      '3': '.qaul.rpc.account_management.RestoreAccountResponse'
    },
  ],
};

@$core.Deprecated('Use accountManagementServiceDescriptor instead')
const $core.Map<$core.String, $core.Map<$core.String, $core.dynamic>>
    AccountManagementServiceBase$messageJson = {
  '.qaul.rpc.account_management.ExportAccountRequest':
      ExportAccountRequest$json,
  '.qaul.rpc.account_management.ExportAccountResponse':
      ExportAccountResponse$json,
  '.qaul.rpc.account_management.DeleteAccountRequest':
      DeleteAccountRequest$json,
  '.qaul.common.Ack': $0.Ack$json,
  '.qaul.rpc.account_management.RestoreAccountRequest':
      RestoreAccountRequest$json,
  '.qaul.rpc.account_management.RestoreAccountResponse':
      RestoreAccountResponse$json,
};

/// Descriptor for `AccountManagementService`. Decode as a `google.protobuf.ServiceDescriptorProto`.
final $typed_data.Uint8List accountManagementServiceDescriptor = $convert.base64Decode(
    'ChhBY2NvdW50TWFuYWdlbWVudFNlcnZpY2USbwoGRXhwb3J0EjEucWF1bC5ycGMuYWNjb3VudF'
    '9tYW5hZ2VtZW50LkV4cG9ydEFjY291bnRSZXF1ZXN0GjIucWF1bC5ycGMuYWNjb3VudF9tYW5h'
    'Z2VtZW50LkV4cG9ydEFjY291bnRSZXNwb25zZRJNCgZEZWxldGUSMS5xYXVsLnJwYy5hY2NvdW'
    '50X21hbmFnZW1lbnQuRGVsZXRlQWNjb3VudFJlcXVlc3QaEC5xYXVsLmNvbW1vbi5BY2sScgoH'
    'UmVzdG9yZRIyLnFhdWwucnBjLmFjY291bnRfbWFuYWdlbWVudC5SZXN0b3JlQWNjb3VudFJlcX'
    'Vlc3QaMy5xYXVsLnJwYy5hY2NvdW50X21hbmFnZW1lbnQuUmVzdG9yZUFjY291bnRSZXNwb25z'
    'ZQ==');
