///
//  Generated code. Do not modify.
//  source: connections/ble/manager/ble.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields,deprecated_member_use_from_same_package

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use bleModeDescriptor instead')
const BleMode$json = const {
  '1': 'BleMode',
  '2': const [
    const {'1': 'legacy', '2': 0},
    const {'1': 'le_1m', '2': 1},
    const {'1': 'le_2m', '2': 2},
    const {'1': 'coded_2', '2': 3},
    const {'1': 'coded_8', '2': 4},
  ],
};

/// Descriptor for `BleMode`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List bleModeDescriptor = $convert.base64Decode('CgdCbGVNb2RlEgoKBmxlZ2FjeRAAEgkKBWxlXzFtEAESCQoFbGVfMm0QAhILCgdjb2RlZF8yEAMSCwoHY29kZWRfOBAE');
@$core.Deprecated('Use bleDescriptor instead')
const Ble$json = const {
  '1': 'Ble',
  '2': const [
    const {'1': 'info_request', '3': 1, '4': 1, '5': 11, '6': '.qaul.sys.ble.BleInfoRequest', '9': 0, '10': 'infoRequest'},
    const {'1': 'info_response', '3': 2, '4': 1, '5': 11, '6': '.qaul.sys.ble.BleInfoResponse', '9': 0, '10': 'infoResponse'},
    const {'1': 'start_request', '3': 3, '4': 1, '5': 11, '6': '.qaul.sys.ble.BleStartRequest', '9': 0, '10': 'startRequest'},
    const {'1': 'start_result', '3': 4, '4': 1, '5': 11, '6': '.qaul.sys.ble.BleStartResult', '9': 0, '10': 'startResult'},
    const {'1': 'advertising_set', '3': 5, '4': 1, '5': 11, '6': '.qaul.sys.ble.BleAdvertisingSet', '9': 0, '10': 'advertisingSet'},
    const {'1': 'advertising_send', '3': 6, '4': 1, '5': 11, '6': '.qaul.sys.ble.BleAdvertisingSend', '9': 0, '10': 'advertisingSend'},
    const {'1': 'advertising_received', '3': 7, '4': 1, '5': 11, '6': '.qaul.sys.ble.BleAdvertisingReceived', '9': 0, '10': 'advertisingReceived'},
    const {'1': 'direct_send', '3': 8, '4': 1, '5': 11, '6': '.qaul.sys.ble.BleDirectSend', '9': 0, '10': 'directSend'},
    const {'1': 'direct_received', '3': 9, '4': 1, '5': 11, '6': '.qaul.sys.ble.BleDirectReceived', '9': 0, '10': 'directReceived'},
  ],
  '8': const [
    const {'1': 'message'},
  ],
};

/// Descriptor for `Ble`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List bleDescriptor = $convert.base64Decode('CgNCbGUSQQoMaW5mb19yZXF1ZXN0GAEgASgLMhwucWF1bC5zeXMuYmxlLkJsZUluZm9SZXF1ZXN0SABSC2luZm9SZXF1ZXN0EkQKDWluZm9fcmVzcG9uc2UYAiABKAsyHS5xYXVsLnN5cy5ibGUuQmxlSW5mb1Jlc3BvbnNlSABSDGluZm9SZXNwb25zZRJECg1zdGFydF9yZXF1ZXN0GAMgASgLMh0ucWF1bC5zeXMuYmxlLkJsZVN0YXJ0UmVxdWVzdEgAUgxzdGFydFJlcXVlc3QSQQoMc3RhcnRfcmVzdWx0GAQgASgLMhwucWF1bC5zeXMuYmxlLkJsZVN0YXJ0UmVzdWx0SABSC3N0YXJ0UmVzdWx0EkoKD2FkdmVydGlzaW5nX3NldBgFIAEoCzIfLnFhdWwuc3lzLmJsZS5CbGVBZHZlcnRpc2luZ1NldEgAUg5hZHZlcnRpc2luZ1NldBJNChBhZHZlcnRpc2luZ19zZW5kGAYgASgLMiAucWF1bC5zeXMuYmxlLkJsZUFkdmVydGlzaW5nU2VuZEgAUg9hZHZlcnRpc2luZ1NlbmQSWQoUYWR2ZXJ0aXNpbmdfcmVjZWl2ZWQYByABKAsyJC5xYXVsLnN5cy5ibGUuQmxlQWR2ZXJ0aXNpbmdSZWNlaXZlZEgAUhNhZHZlcnRpc2luZ1JlY2VpdmVkEj4KC2RpcmVjdF9zZW5kGAggASgLMhsucWF1bC5zeXMuYmxlLkJsZURpcmVjdFNlbmRIAFIKZGlyZWN0U2VuZBJKCg9kaXJlY3RfcmVjZWl2ZWQYCSABKAsyHy5xYXVsLnN5cy5ibGUuQmxlRGlyZWN0UmVjZWl2ZWRIAFIOZGlyZWN0UmVjZWl2ZWRCCQoHbWVzc2FnZQ==');
@$core.Deprecated('Use bleInfoRequestDescriptor instead')
const BleInfoRequest$json = const {
  '1': 'BleInfoRequest',
};

/// Descriptor for `BleInfoRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List bleInfoRequestDescriptor = $convert.base64Decode('Cg5CbGVJbmZvUmVxdWVzdA==');
@$core.Deprecated('Use bleInfoResponseDescriptor instead')
const BleInfoResponse$json = const {
  '1': 'BleInfoResponse',
  '2': const [
    const {'1': 'device', '3': 1, '4': 3, '5': 11, '6': '.qaul.sys.ble.BleDeviceInfo', '10': 'device'},
  ],
};

/// Descriptor for `BleInfoResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List bleInfoResponseDescriptor = $convert.base64Decode('Cg9CbGVJbmZvUmVzcG9uc2USMwoGZGV2aWNlGAEgAygLMhsucWF1bC5zeXMuYmxlLkJsZURldmljZUluZm9SBmRldmljZQ==');
@$core.Deprecated('Use bleDeviceInfoDescriptor instead')
const BleDeviceInfo$json = const {
  '1': 'BleDeviceInfo',
  '2': const [
    const {'1': 'id', '3': 1, '4': 1, '5': 9, '10': 'id'},
    const {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
    const {'1': 'powered', '3': 3, '4': 1, '5': 8, '10': 'powered'},
    const {'1': 'ble_support', '3': 4, '4': 1, '5': 8, '10': 'bleSupport'},
    const {'1': 'adv_251', '3': 7, '4': 1, '5': 8, '10': 'adv251'},
    const {'1': 'adv_extended', '3': 8, '4': 1, '5': 8, '10': 'advExtended'},
    const {'1': 'adv_extended_bytes', '3': 9, '4': 1, '5': 13, '10': 'advExtendedBytes'},
    const {'1': 'adv_1m', '3': 10, '4': 1, '5': 8, '10': 'adv1m'},
    const {'1': 'adv_2m', '3': 11, '4': 1, '5': 8, '10': 'adv2m'},
    const {'1': 'adv_coded', '3': 12, '4': 1, '5': 8, '10': 'advCoded'},
    const {'1': 'le_audio', '3': 13, '4': 1, '5': 8, '10': 'leAudio'},
  ],
};

/// Descriptor for `BleDeviceInfo`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List bleDeviceInfoDescriptor = $convert.base64Decode('Cg1CbGVEZXZpY2VJbmZvEg4KAmlkGAEgASgJUgJpZBISCgRuYW1lGAIgASgJUgRuYW1lEhgKB3Bvd2VyZWQYAyABKAhSB3Bvd2VyZWQSHwoLYmxlX3N1cHBvcnQYBCABKAhSCmJsZVN1cHBvcnQSFwoHYWR2XzI1MRgHIAEoCFIGYWR2MjUxEiEKDGFkdl9leHRlbmRlZBgIIAEoCFILYWR2RXh0ZW5kZWQSLAoSYWR2X2V4dGVuZGVkX2J5dGVzGAkgASgNUhBhZHZFeHRlbmRlZEJ5dGVzEhUKBmFkdl8xbRgKIAEoCFIFYWR2MW0SFQoGYWR2XzJtGAsgASgIUgVhZHYybRIbCglhZHZfY29kZWQYDCABKAhSCGFkdkNvZGVkEhkKCGxlX2F1ZGlvGA0gASgIUgdsZUF1ZGlv');
@$core.Deprecated('Use bleStartRequestDescriptor instead')
const BleStartRequest$json = const {
  '1': 'BleStartRequest',
};

/// Descriptor for `BleStartRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List bleStartRequestDescriptor = $convert.base64Decode('Cg9CbGVTdGFydFJlcXVlc3Q=');
@$core.Deprecated('Use bleStartResultDescriptor instead')
const BleStartResult$json = const {
  '1': 'BleStartResult',
  '2': const [
    const {'1': 'success', '3': 1, '4': 1, '5': 8, '10': 'success'},
    const {'1': 'error_message', '3': 2, '4': 1, '5': 9, '10': 'errorMessage'},
    const {'1': 'unknonw_error', '3': 3, '4': 1, '5': 8, '10': 'unknonwError'},
    const {'1': 'no_rights', '3': 4, '4': 1, '5': 8, '10': 'noRights'},
  ],
};

/// Descriptor for `BleStartResult`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List bleStartResultDescriptor = $convert.base64Decode('Cg5CbGVTdGFydFJlc3VsdBIYCgdzdWNjZXNzGAEgASgIUgdzdWNjZXNzEiMKDWVycm9yX21lc3NhZ2UYAiABKAlSDGVycm9yTWVzc2FnZRIjCg11bmtub253X2Vycm9yGAMgASgIUgx1bmtub253RXJyb3ISGwoJbm9fcmlnaHRzGAQgASgIUghub1JpZ2h0cw==');
@$core.Deprecated('Use bleAdvertisingSetDescriptor instead')
const BleAdvertisingSet$json = const {
  '1': 'BleAdvertisingSet',
  '2': const [
    const {'1': 'data', '3': 1, '4': 1, '5': 12, '10': 'data'},
  ],
};

/// Descriptor for `BleAdvertisingSet`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List bleAdvertisingSetDescriptor = $convert.base64Decode('ChFCbGVBZHZlcnRpc2luZ1NldBISCgRkYXRhGAEgASgMUgRkYXRh');
@$core.Deprecated('Use bleAdvertisingSendDescriptor instead')
const BleAdvertisingSend$json = const {
  '1': 'BleAdvertisingSend',
  '2': const [
    const {'1': 'mode', '3': 1, '4': 1, '5': 14, '6': '.qaul.sys.ble.BleMode', '10': 'mode'},
    const {'1': 'data', '3': 2, '4': 1, '5': 12, '10': 'data'},
  ],
};

/// Descriptor for `BleAdvertisingSend`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List bleAdvertisingSendDescriptor = $convert.base64Decode('ChJCbGVBZHZlcnRpc2luZ1NlbmQSKQoEbW9kZRgBIAEoDjIVLnFhdWwuc3lzLmJsZS5CbGVNb2RlUgRtb2RlEhIKBGRhdGEYAiABKAxSBGRhdGE=');
@$core.Deprecated('Use bleAdvertisingReceivedDescriptor instead')
const BleAdvertisingReceived$json = const {
  '1': 'BleAdvertisingReceived',
  '2': const [
    const {'1': 'id', '3': 1, '4': 1, '5': 12, '10': 'id'},
    const {'1': 'rssi', '3': 2, '4': 1, '5': 5, '10': 'rssi'},
    const {'1': 'mode', '3': 3, '4': 1, '5': 14, '6': '.qaul.sys.ble.BleMode', '10': 'mode'},
    const {'1': 'data', '3': 4, '4': 1, '5': 12, '10': 'data'},
  ],
};

/// Descriptor for `BleAdvertisingReceived`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List bleAdvertisingReceivedDescriptor = $convert.base64Decode('ChZCbGVBZHZlcnRpc2luZ1JlY2VpdmVkEg4KAmlkGAEgASgMUgJpZBISCgRyc3NpGAIgASgFUgRyc3NpEikKBG1vZGUYAyABKA4yFS5xYXVsLnN5cy5ibGUuQmxlTW9kZVIEbW9kZRISCgRkYXRhGAQgASgMUgRkYXRh');
@$core.Deprecated('Use bleDirectSendDescriptor instead')
const BleDirectSend$json = const {
  '1': 'BleDirectSend',
  '2': const [
    const {'1': 'id', '3': 1, '4': 1, '5': 12, '10': 'id'},
    const {'1': 'to', '3': 2, '4': 1, '5': 12, '10': 'to'},
    const {'1': 'mode', '3': 3, '4': 1, '5': 14, '6': '.qaul.sys.ble.BleMode', '10': 'mode'},
    const {'1': 'data', '3': 4, '4': 1, '5': 12, '10': 'data'},
  ],
};

/// Descriptor for `BleDirectSend`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List bleDirectSendDescriptor = $convert.base64Decode('Cg1CbGVEaXJlY3RTZW5kEg4KAmlkGAEgASgMUgJpZBIOCgJ0bxgCIAEoDFICdG8SKQoEbW9kZRgDIAEoDjIVLnFhdWwuc3lzLmJsZS5CbGVNb2RlUgRtb2RlEhIKBGRhdGEYBCABKAxSBGRhdGE=');
@$core.Deprecated('Use bleDirectSendResultDescriptor instead')
const BleDirectSendResult$json = const {
  '1': 'BleDirectSendResult',
  '2': const [
    const {'1': 'id', '3': 1, '4': 1, '5': 12, '10': 'id'},
    const {'1': 'success', '3': 2, '4': 1, '5': 8, '10': 'success'},
    const {'1': 'error_message', '3': 3, '4': 1, '5': 9, '10': 'errorMessage'},
  ],
};

/// Descriptor for `BleDirectSendResult`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List bleDirectSendResultDescriptor = $convert.base64Decode('ChNCbGVEaXJlY3RTZW5kUmVzdWx0Eg4KAmlkGAEgASgMUgJpZBIYCgdzdWNjZXNzGAIgASgIUgdzdWNjZXNzEiMKDWVycm9yX21lc3NhZ2UYAyABKAlSDGVycm9yTWVzc2FnZQ==');
@$core.Deprecated('Use bleDirectReceivedDescriptor instead')
const BleDirectReceived$json = const {
  '1': 'BleDirectReceived',
  '2': const [
    const {'1': 'from', '3': 1, '4': 1, '5': 12, '10': 'from'},
    const {'1': 'rssi', '3': 2, '4': 1, '5': 5, '10': 'rssi'},
    const {'1': 'mode', '3': 3, '4': 1, '5': 14, '6': '.qaul.sys.ble.BleMode', '10': 'mode'},
    const {'1': 'data', '3': 4, '4': 1, '5': 12, '10': 'data'},
  ],
};

/// Descriptor for `BleDirectReceived`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List bleDirectReceivedDescriptor = $convert.base64Decode('ChFCbGVEaXJlY3RSZWNlaXZlZBISCgRmcm9tGAEgASgMUgRmcm9tEhIKBHJzc2kYAiABKAVSBHJzc2kSKQoEbW9kZRgDIAEoDjIVLnFhdWwuc3lzLmJsZS5CbGVNb2RlUgRtb2RlEhIKBGRhdGEYBCABKAxSBGRhdGE=');
