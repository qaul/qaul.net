//
//  Generated code. Do not modify.
//  source: services/crypto/crypto_net.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use cryptoserviceContainerDescriptor instead')
const CryptoserviceContainer$json = {
  '1': 'CryptoserviceContainer',
  '2': [
    {'1': 'second_handshake', '3': 1, '4': 1, '5': 11, '6': '.qaul.net.crypto.SecondHandshake', '9': 0, '10': 'secondHandshake'},
  ],
  '8': [
    {'1': 'message'},
  ],
};

/// Descriptor for `CryptoserviceContainer`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List cryptoserviceContainerDescriptor = $convert.base64Decode(
    'ChZDcnlwdG9zZXJ2aWNlQ29udGFpbmVyEk0KEHNlY29uZF9oYW5kc2hha2UYASABKAsyIC5xYX'
    'VsLm5ldC5jcnlwdG8uU2Vjb25kSGFuZHNoYWtlSABSD3NlY29uZEhhbmRzaGFrZUIJCgdtZXNz'
    'YWdl');

@$core.Deprecated('Use secondHandshakeDescriptor instead')
const SecondHandshake$json = {
  '1': 'SecondHandshake',
  '2': [
    {'1': 'signature', '3': 1, '4': 1, '5': 12, '10': 'signature'},
    {'1': 'received_at', '3': 2, '4': 1, '5': 4, '10': 'receivedAt'},
  ],
};

/// Descriptor for `SecondHandshake`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List secondHandshakeDescriptor = $convert.base64Decode(
    'Cg9TZWNvbmRIYW5kc2hha2USHAoJc2lnbmF0dXJlGAEgASgMUglzaWduYXR1cmUSHwoLcmVjZW'
    'l2ZWRfYXQYAiABKARSCnJlY2VpdmVkQXQ=');

