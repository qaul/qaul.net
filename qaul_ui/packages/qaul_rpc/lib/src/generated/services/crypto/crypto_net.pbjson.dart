///
//  Generated code. Do not modify.
//  source: services/crypto/crypto_net.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,deprecated_member_use_from_same_package,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use cryptoserviceContainerDescriptor instead')
const CryptoserviceContainer$json = const {
  '1': 'CryptoserviceContainer',
  '2': const [
    const {'1': 'second_handshake', '3': 1, '4': 1, '5': 11, '6': '.qaul.net.crypto.SecondHandshake', '9': 0, '10': 'secondHandshake'},
  ],
  '8': const [
    const {'1': 'message'},
  ],
};

/// Descriptor for `CryptoserviceContainer`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List cryptoserviceContainerDescriptor = $convert.base64Decode('ChZDcnlwdG9zZXJ2aWNlQ29udGFpbmVyEk0KEHNlY29uZF9oYW5kc2hha2UYASABKAsyIC5xYXVsLm5ldC5jcnlwdG8uU2Vjb25kSGFuZHNoYWtlSABSD3NlY29uZEhhbmRzaGFrZUIJCgdtZXNzYWdl');
@$core.Deprecated('Use secondHandshakeDescriptor instead')
const SecondHandshake$json = const {
  '1': 'SecondHandshake',
  '2': const [
    const {'1': 'signature', '3': 1, '4': 1, '5': 12, '10': 'signature'},
    const {'1': 'received_at', '3': 2, '4': 1, '5': 4, '10': 'receivedAt'},
  ],
};

/// Descriptor for `SecondHandshake`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List secondHandshakeDescriptor = $convert.base64Decode('Cg9TZWNvbmRIYW5kc2hha2USHAoJc2lnbmF0dXJlGAEgASgMUglzaWduYXR1cmUSHwoLcmVjZWl2ZWRfYXQYAiABKARSCnJlY2VpdmVkQXQ=');
