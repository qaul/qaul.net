// This is a generated file - do not edit.
//
// Generated from connections/transports.proto.

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

@$core.Deprecated('Use transportsDescriptor instead')
const Transports$json = {
  '1': 'Transports',
  '2': [
    {
      '1': 'list_request',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.qaul.rpc.transports.TransportsListRequest',
      '9': 0,
      '10': 'listRequest'
    },
    {
      '1': 'list',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.qaul.rpc.transports.TransportsList',
      '9': 0,
      '10': 'list'
    },
    {
      '1': 'set_enabled',
      '3': 3,
      '4': 1,
      '5': 11,
      '6': '.qaul.rpc.transports.TransportSetEnabled',
      '9': 0,
      '10': 'setEnabled'
    },
    {
      '1': 'set_enabled_result',
      '3': 4,
      '4': 1,
      '5': 11,
      '6': '.qaul.rpc.transports.TransportSetEnabledResult',
      '9': 0,
      '10': 'setEnabledResult'
    },
  ],
  '8': [
    {'1': 'message'},
  ],
};

/// Descriptor for `Transports`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List transportsDescriptor = $convert.base64Decode(
    'CgpUcmFuc3BvcnRzEk8KDGxpc3RfcmVxdWVzdBgBIAEoCzIqLnFhdWwucnBjLnRyYW5zcG9ydH'
    'MuVHJhbnNwb3J0c0xpc3RSZXF1ZXN0SABSC2xpc3RSZXF1ZXN0EjkKBGxpc3QYAiABKAsyIy5x'
    'YXVsLnJwYy50cmFuc3BvcnRzLlRyYW5zcG9ydHNMaXN0SABSBGxpc3QSSwoLc2V0X2VuYWJsZW'
    'QYAyABKAsyKC5xYXVsLnJwYy50cmFuc3BvcnRzLlRyYW5zcG9ydFNldEVuYWJsZWRIAFIKc2V0'
    'RW5hYmxlZBJeChJzZXRfZW5hYmxlZF9yZXN1bHQYBCABKAsyLi5xYXVsLnJwYy50cmFuc3Bvcn'
    'RzLlRyYW5zcG9ydFNldEVuYWJsZWRSZXN1bHRIAFIQc2V0RW5hYmxlZFJlc3VsdEIJCgdtZXNz'
    'YWdl');

@$core.Deprecated('Use transportsListRequestDescriptor instead')
const TransportsListRequest$json = {
  '1': 'TransportsListRequest',
};

/// Descriptor for `TransportsListRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List transportsListRequestDescriptor =
    $convert.base64Decode('ChVUcmFuc3BvcnRzTGlzdFJlcXVlc3Q=');

@$core.Deprecated('Use transportsListDescriptor instead')
const TransportsList$json = {
  '1': 'TransportsList',
  '2': [
    {
      '1': 'transports',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.qaul.rpc.transports.TransportInfo',
      '10': 'transports'
    },
  ],
};

/// Descriptor for `TransportsList`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List transportsListDescriptor = $convert.base64Decode(
    'Cg5UcmFuc3BvcnRzTGlzdBJCCgp0cmFuc3BvcnRzGAEgAygLMiIucWF1bC5ycGMudHJhbnNwb3'
    'J0cy5UcmFuc3BvcnRJbmZvUgp0cmFuc3BvcnRz');

@$core.Deprecated('Use transportInfoDescriptor instead')
const TransportInfo$json = {
  '1': 'TransportInfo',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 9, '10': 'id'},
    {'1': 'label', '3': 2, '4': 1, '5': 9, '10': 'label'},
    {'1': 'status', '3': 3, '4': 1, '5': 9, '10': 'status'},
    {'1': 'enabled', '3': 4, '4': 1, '5': 8, '10': 'enabled'},
    {
      '1': 'supports_runtime_toggle',
      '3': 5,
      '4': 1,
      '5': 8,
      '10': 'supportsRuntimeToggle'
    },
    {'1': 'is_local_only', '3': 6, '4': 1, '5': 8, '10': 'isLocalOnly'},
  ],
};

/// Descriptor for `TransportInfo`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List transportInfoDescriptor = $convert.base64Decode(
    'Cg1UcmFuc3BvcnRJbmZvEg4KAmlkGAEgASgJUgJpZBIUCgVsYWJlbBgCIAEoCVIFbGFiZWwSFg'
    'oGc3RhdHVzGAMgASgJUgZzdGF0dXMSGAoHZW5hYmxlZBgEIAEoCFIHZW5hYmxlZBI2ChdzdXBw'
    'b3J0c19ydW50aW1lX3RvZ2dsZRgFIAEoCFIVc3VwcG9ydHNSdW50aW1lVG9nZ2xlEiIKDWlzX2'
    'xvY2FsX29ubHkYBiABKAhSC2lzTG9jYWxPbmx5');

@$core.Deprecated('Use transportSetEnabledDescriptor instead')
const TransportSetEnabled$json = {
  '1': 'TransportSetEnabled',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 9, '10': 'id'},
    {'1': 'enabled', '3': 2, '4': 1, '5': 8, '10': 'enabled'},
  ],
};

/// Descriptor for `TransportSetEnabled`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List transportSetEnabledDescriptor = $convert.base64Decode(
    'ChNUcmFuc3BvcnRTZXRFbmFibGVkEg4KAmlkGAEgASgJUgJpZBIYCgdlbmFibGVkGAIgASgIUg'
    'dlbmFibGVk');

@$core.Deprecated('Use transportSetEnabledResultDescriptor instead')
const TransportSetEnabledResult$json = {
  '1': 'TransportSetEnabledResult',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 9, '10': 'id'},
    {'1': 'success', '3': 2, '4': 1, '5': 8, '10': 'success'},
    {'1': 'error', '3': 3, '4': 1, '5': 9, '10': 'error'},
  ],
};

/// Descriptor for `TransportSetEnabledResult`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List transportSetEnabledResultDescriptor =
    $convert.base64Decode(
        'ChlUcmFuc3BvcnRTZXRFbmFibGVkUmVzdWx0Eg4KAmlkGAEgASgJUgJpZBIYCgdzdWNjZXNzGA'
        'IgASgIUgdzdWNjZXNzEhQKBWVycm9yGAMgASgJUgVlcnJvcg==');
