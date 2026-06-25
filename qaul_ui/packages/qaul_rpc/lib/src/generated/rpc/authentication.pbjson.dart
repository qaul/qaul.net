// This is a generated file - do not edit.
//
// Generated from rpc/authentication.proto.

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

@$core.Deprecated('Use authRpcDescriptor instead')
const AuthRpc$json = {
  '1': 'AuthRpc',
  '2': [
    {
      '1': 'auth_request',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.qaul.rpc.authentication.AuthRequest',
      '9': 0,
      '10': 'authRequest'
    },
    {
      '1': 'auth_challenge',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.qaul.rpc.authentication.AuthChallenge',
      '9': 0,
      '10': 'authChallenge'
    },
    {
      '1': 'auth_response',
      '3': 3,
      '4': 1,
      '5': 11,
      '6': '.qaul.rpc.authentication.AuthResponse',
      '9': 0,
      '10': 'authResponse'
    },
    {
      '1': 'auth_result',
      '3': 4,
      '4': 1,
      '5': 11,
      '6': '.qaul.rpc.authentication.AuthResult',
      '9': 0,
      '10': 'authResult'
    },
    {
      '1': 'users_request',
      '3': 5,
      '4': 1,
      '5': 11,
      '6': '.qaul.rpc.authentication.UsersRequest',
      '9': 0,
      '10': 'usersRequest'
    },
    {
      '1': 'users_response',
      '3': 6,
      '4': 1,
      '5': 11,
      '6': '.qaul.rpc.authentication.UsersResponse',
      '9': 0,
      '10': 'usersResponse'
    },
    {
      '1': 'logout_request',
      '3': 7,
      '4': 1,
      '5': 11,
      '6': '.qaul.rpc.authentication.LogoutRequest',
      '9': 0,
      '10': 'logoutRequest'
    },
    {
      '1': 'session_status_request',
      '3': 8,
      '4': 1,
      '5': 11,
      '6': '.qaul.rpc.authentication.SessionStatusRequest',
      '9': 0,
      '10': 'sessionStatusRequest'
    },
    {
      '1': 'session_status_response',
      '3': 9,
      '4': 1,
      '5': 11,
      '6': '.qaul.rpc.authentication.SessionStatusResponse',
      '9': 0,
      '10': 'sessionStatusResponse'
    },
    {
      '1': 'ack',
      '3': 10,
      '4': 1,
      '5': 11,
      '6': '.qaul.common.Ack',
      '9': 0,
      '10': 'ack'
    },
    {
      '1': 'error',
      '3': 11,
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

/// Descriptor for `AuthRpc`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List authRpcDescriptor = $convert.base64Decode(
    'CgdBdXRoUnBjEkkKDGF1dGhfcmVxdWVzdBgBIAEoCzIkLnFhdWwucnBjLmF1dGhlbnRpY2F0aW'
    '9uLkF1dGhSZXF1ZXN0SABSC2F1dGhSZXF1ZXN0Ek8KDmF1dGhfY2hhbGxlbmdlGAIgASgLMiYu'
    'cWF1bC5ycGMuYXV0aGVudGljYXRpb24uQXV0aENoYWxsZW5nZUgAUg1hdXRoQ2hhbGxlbmdlEk'
    'wKDWF1dGhfcmVzcG9uc2UYAyABKAsyJS5xYXVsLnJwYy5hdXRoZW50aWNhdGlvbi5BdXRoUmVz'
    'cG9uc2VIAFIMYXV0aFJlc3BvbnNlEkYKC2F1dGhfcmVzdWx0GAQgASgLMiMucWF1bC5ycGMuYX'
    'V0aGVudGljYXRpb24uQXV0aFJlc3VsdEgAUgphdXRoUmVzdWx0EkwKDXVzZXJzX3JlcXVlc3QY'
    'BSABKAsyJS5xYXVsLnJwYy5hdXRoZW50aWNhdGlvbi5Vc2Vyc1JlcXVlc3RIAFIMdXNlcnNSZX'
    'F1ZXN0Ek8KDnVzZXJzX3Jlc3BvbnNlGAYgASgLMiYucWF1bC5ycGMuYXV0aGVudGljYXRpb24u'
    'VXNlcnNSZXNwb25zZUgAUg11c2Vyc1Jlc3BvbnNlEk8KDmxvZ291dF9yZXF1ZXN0GAcgASgLMi'
    'YucWF1bC5ycGMuYXV0aGVudGljYXRpb24uTG9nb3V0UmVxdWVzdEgAUg1sb2dvdXRSZXF1ZXN0'
    'EmUKFnNlc3Npb25fc3RhdHVzX3JlcXVlc3QYCCABKAsyLS5xYXVsLnJwYy5hdXRoZW50aWNhdG'
    'lvbi5TZXNzaW9uU3RhdHVzUmVxdWVzdEgAUhRzZXNzaW9uU3RhdHVzUmVxdWVzdBJoChdzZXNz'
    'aW9uX3N0YXR1c19yZXNwb25zZRgJIAEoCzIuLnFhdWwucnBjLmF1dGhlbnRpY2F0aW9uLlNlc3'
    'Npb25TdGF0dXNSZXNwb25zZUgAUhVzZXNzaW9uU3RhdHVzUmVzcG9uc2USJAoDYWNrGAogASgL'
    'MhAucWF1bC5jb21tb24uQWNrSABSA2FjaxItCgVlcnJvchgLIAEoCzIVLnFhdWwuY29tbW9uLl'
    'JwY0Vycm9ySABSBWVycm9yQgkKB21lc3NhZ2U=');

@$core.Deprecated('Use usersRequestDescriptor instead')
const UsersRequest$json = {
  '1': 'UsersRequest',
};

/// Descriptor for `UsersRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List usersRequestDescriptor =
    $convert.base64Decode('CgxVc2Vyc1JlcXVlc3Q=');

@$core.Deprecated('Use usersResponseDescriptor instead')
const UsersResponse$json = {
  '1': 'UsersResponse',
  '2': [
    {
      '1': 'users',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.qaul.rpc.authentication.UserInfo',
      '10': 'users'
    },
    {'1': 'error_message', '3': 2, '4': 1, '5': 9, '10': 'errorMessage'},
  ],
};

/// Descriptor for `UsersResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List usersResponseDescriptor = $convert.base64Decode(
    'Cg1Vc2Vyc1Jlc3BvbnNlEjcKBXVzZXJzGAEgAygLMiEucWF1bC5ycGMuYXV0aGVudGljYXRpb2'
    '4uVXNlckluZm9SBXVzZXJzEiMKDWVycm9yX21lc3NhZ2UYAiABKAlSDGVycm9yTWVzc2FnZQ==');

@$core.Deprecated('Use userInfoDescriptor instead')
const UserInfo$json = {
  '1': 'UserInfo',
  '2': [
    {'1': 'username', '3': 1, '4': 1, '5': 9, '10': 'username'},
    {'1': 'user_id', '3': 2, '4': 1, '5': 12, '10': 'userId'},
    {'1': 'salt', '3': 3, '4': 1, '5': 9, '9': 0, '10': 'salt', '17': true},
    {'1': 'has_password', '3': 4, '4': 1, '5': 8, '10': 'hasPassword'},
  ],
  '8': [
    {'1': '_salt'},
  ],
};

/// Descriptor for `UserInfo`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List userInfoDescriptor = $convert.base64Decode(
    'CghVc2VySW5mbxIaCgh1c2VybmFtZRgBIAEoCVIIdXNlcm5hbWUSFwoHdXNlcl9pZBgCIAEoDF'
    'IGdXNlcklkEhcKBHNhbHQYAyABKAlIAFIEc2FsdIgBARIhCgxoYXNfcGFzc3dvcmQYBCABKAhS'
    'C2hhc1Bhc3N3b3JkQgcKBV9zYWx0');

@$core.Deprecated('Use authRequestDescriptor instead')
const AuthRequest$json = {
  '1': 'AuthRequest',
  '2': [
    {'1': 'user_id', '3': 1, '4': 1, '5': 12, '10': 'userId'},
  ],
};

/// Descriptor for `AuthRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List authRequestDescriptor = $convert
    .base64Decode('CgtBdXRoUmVxdWVzdBIXCgd1c2VyX2lkGAEgASgMUgZ1c2VySWQ=');

@$core.Deprecated('Use authChallengeDescriptor instead')
const AuthChallenge$json = {
  '1': 'AuthChallenge',
  '2': [
    {'1': 'nonce', '3': 1, '4': 1, '5': 4, '10': 'nonce'},
    {'1': 'expires_at', '3': 2, '4': 1, '5': 4, '10': 'expiresAt'},
  ],
};

/// Descriptor for `AuthChallenge`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List authChallengeDescriptor = $convert.base64Decode(
    'Cg1BdXRoQ2hhbGxlbmdlEhQKBW5vbmNlGAEgASgEUgVub25jZRIdCgpleHBpcmVzX2F0GAIgAS'
    'gEUglleHBpcmVzQXQ=');

@$core.Deprecated('Use authResponseDescriptor instead')
const AuthResponse$json = {
  '1': 'AuthResponse',
  '2': [
    {'1': 'challenge_hash', '3': 1, '4': 1, '5': 12, '10': 'challengeHash'},
    {'1': 'user_id', '3': 2, '4': 1, '5': 12, '10': 'userId'},
  ],
};

/// Descriptor for `AuthResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List authResponseDescriptor = $convert.base64Decode(
    'CgxBdXRoUmVzcG9uc2USJQoOY2hhbGxlbmdlX2hhc2gYASABKAxSDWNoYWxsZW5nZUhhc2gSFw'
    'oHdXNlcl9pZBgCIAEoDFIGdXNlcklk');

@$core.Deprecated('Use authResultDescriptor instead')
const AuthResult$json = {
  '1': 'AuthResult',
  '2': [
    {'1': 'success', '3': 1, '4': 1, '5': 8, '10': 'success'},
    {'1': 'error_message', '3': 2, '4': 1, '5': 9, '10': 'errorMessage'},
  ],
};

/// Descriptor for `AuthResult`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List authResultDescriptor = $convert.base64Decode(
    'CgpBdXRoUmVzdWx0EhgKB3N1Y2Nlc3MYASABKAhSB3N1Y2Nlc3MSIwoNZXJyb3JfbWVzc2FnZR'
    'gCIAEoCVIMZXJyb3JNZXNzYWdl');

@$core.Deprecated('Use logoutRequestDescriptor instead')
const LogoutRequest$json = {
  '1': 'LogoutRequest',
};

/// Descriptor for `LogoutRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List logoutRequestDescriptor =
    $convert.base64Decode('Cg1Mb2dvdXRSZXF1ZXN0');

@$core.Deprecated('Use sessionStatusRequestDescriptor instead')
const SessionStatusRequest$json = {
  '1': 'SessionStatusRequest',
};

/// Descriptor for `SessionStatusRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sessionStatusRequestDescriptor =
    $convert.base64Decode('ChRTZXNzaW9uU3RhdHVzUmVxdWVzdA==');

@$core.Deprecated('Use sessionStatusResponseDescriptor instead')
const SessionStatusResponse$json = {
  '1': 'SessionStatusResponse',
  '2': [
    {'1': 'authenticated', '3': 1, '4': 1, '5': 8, '10': 'authenticated'},
  ],
};

/// Descriptor for `SessionStatusResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sessionStatusResponseDescriptor = $convert.base64Decode(
    'ChVTZXNzaW9uU3RhdHVzUmVzcG9uc2USJAoNYXV0aGVudGljYXRlZBgBIAEoCFINYXV0aGVudG'
    'ljYXRlZA==');

const $core.Map<$core.String, $core.dynamic> AuthRpcServiceBase$json = {
  '1': 'AuthRpcService',
  '2': [
    {
      '1': 'Users',
      '2': '.qaul.rpc.authentication.UsersRequest',
      '3': '.qaul.rpc.authentication.UsersResponse'
    },
    {
      '1': 'RequestChallenge',
      '2': '.qaul.rpc.authentication.AuthRequest',
      '3': '.qaul.rpc.authentication.AuthChallenge'
    },
    {
      '1': 'RespondChallenge',
      '2': '.qaul.rpc.authentication.AuthResponse',
      '3': '.qaul.rpc.authentication.AuthResult'
    },
    {
      '1': 'LogoutSession',
      '2': '.qaul.rpc.authentication.LogoutRequest',
      '3': '.qaul.common.Ack'
    },
    {
      '1': 'SessionStatus',
      '2': '.qaul.rpc.authentication.SessionStatusRequest',
      '3': '.qaul.rpc.authentication.SessionStatusResponse'
    },
  ],
};

@$core.Deprecated('Use authRpcServiceDescriptor instead')
const $core.Map<$core.String, $core.Map<$core.String, $core.dynamic>>
    AuthRpcServiceBase$messageJson = {
  '.qaul.rpc.authentication.UsersRequest': UsersRequest$json,
  '.qaul.rpc.authentication.UsersResponse': UsersResponse$json,
  '.qaul.rpc.authentication.UserInfo': UserInfo$json,
  '.qaul.rpc.authentication.AuthRequest': AuthRequest$json,
  '.qaul.rpc.authentication.AuthChallenge': AuthChallenge$json,
  '.qaul.rpc.authentication.AuthResponse': AuthResponse$json,
  '.qaul.rpc.authentication.AuthResult': AuthResult$json,
  '.qaul.rpc.authentication.LogoutRequest': LogoutRequest$json,
  '.qaul.common.Ack': $0.Ack$json,
  '.qaul.rpc.authentication.SessionStatusRequest': SessionStatusRequest$json,
  '.qaul.rpc.authentication.SessionStatusResponse': SessionStatusResponse$json,
};

/// Descriptor for `AuthRpcService`. Decode as a `google.protobuf.ServiceDescriptorProto`.
final $typed_data.Uint8List authRpcServiceDescriptor = $convert.base64Decode(
    'Cg5BdXRoUnBjU2VydmljZRJWCgVVc2VycxIlLnFhdWwucnBjLmF1dGhlbnRpY2F0aW9uLlVzZX'
    'JzUmVxdWVzdBomLnFhdWwucnBjLmF1dGhlbnRpY2F0aW9uLlVzZXJzUmVzcG9uc2USYAoQUmVx'
    'dWVzdENoYWxsZW5nZRIkLnFhdWwucnBjLmF1dGhlbnRpY2F0aW9uLkF1dGhSZXF1ZXN0GiYucW'
    'F1bC5ycGMuYXV0aGVudGljYXRpb24uQXV0aENoYWxsZW5nZRJeChBSZXNwb25kQ2hhbGxlbmdl'
    'EiUucWF1bC5ycGMuYXV0aGVudGljYXRpb24uQXV0aFJlc3BvbnNlGiMucWF1bC5ycGMuYXV0aG'
    'VudGljYXRpb24uQXV0aFJlc3VsdBJJCg1Mb2dvdXRTZXNzaW9uEiYucWF1bC5ycGMuYXV0aGVu'
    'dGljYXRpb24uTG9nb3V0UmVxdWVzdBoQLnFhdWwuY29tbW9uLkFjaxJuCg1TZXNzaW9uU3RhdH'
    'VzEi0ucWF1bC5ycGMuYXV0aGVudGljYXRpb24uU2Vzc2lvblN0YXR1c1JlcXVlc3QaLi5xYXVs'
    'LnJwYy5hdXRoZW50aWNhdGlvbi5TZXNzaW9uU3RhdHVzUmVzcG9uc2U=');
