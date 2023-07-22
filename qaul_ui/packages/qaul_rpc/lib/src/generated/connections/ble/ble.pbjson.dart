//
//  Generated code. Do not modify.
//  source: connections/ble/ble.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use blePowerSettingDescriptor instead')
const BlePowerSetting$json = {
  '1': 'BlePowerSetting',
  '2': [
    {'1': 'low_power', '2': 0},
    {'1': 'balanced', '2': 1},
    {'1': 'low_latency', '2': 2},
  ],
};

/// Descriptor for `BlePowerSetting`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List blePowerSettingDescriptor = $convert.base64Decode(
    'Cg9CbGVQb3dlclNldHRpbmcSDQoJbG93X3Bvd2VyEAASDAoIYmFsYW5jZWQQARIPCgtsb3dfbG'
    'F0ZW5jeRAC');

@$core.Deprecated('Use bleErrorDescriptor instead')
const BleError$json = {
  '1': 'BleError',
  '2': [
    {'1': 'UNKNOWN_ERROR', '2': 0},
    {'1': 'RIGHTS_MISSING', '2': 1},
    {'1': 'TIMEOUT', '2': 2},
  ],
};

/// Descriptor for `BleError`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List bleErrorDescriptor = $convert.base64Decode(
    'CghCbGVFcnJvchIRCg1VTktOT1dOX0VSUk9SEAASEgoOUklHSFRTX01JU1NJTkcQARILCgdUSU'
    '1FT1VUEAI=');

@$core.Deprecated('Use bleDescriptor instead')
const Ble$json = {
  '1': 'Ble',
  '2': [
    {'1': 'info_request', '3': 1, '4': 1, '5': 11, '6': '.qaul.sys.ble.BleInfoRequest', '9': 0, '10': 'infoRequest'},
    {'1': 'info_response', '3': 2, '4': 1, '5': 11, '6': '.qaul.sys.ble.BleInfoResponse', '9': 0, '10': 'infoResponse'},
    {'1': 'start_request', '3': 3, '4': 1, '5': 11, '6': '.qaul.sys.ble.BleStartRequest', '9': 0, '10': 'startRequest'},
    {'1': 'start_result', '3': 4, '4': 1, '5': 11, '6': '.qaul.sys.ble.BleStartResult', '9': 0, '10': 'startResult'},
    {'1': 'stop_request', '3': 5, '4': 1, '5': 11, '6': '.qaul.sys.ble.BleStopRequest', '9': 0, '10': 'stopRequest'},
    {'1': 'stop_result', '3': 6, '4': 1, '5': 11, '6': '.qaul.sys.ble.BleStopResult', '9': 0, '10': 'stopResult'},
    {'1': 'device_discovered', '3': 7, '4': 1, '5': 11, '6': '.qaul.sys.ble.BleDeviceDiscovered', '9': 0, '10': 'deviceDiscovered'},
    {'1': 'device_unavailable', '3': 8, '4': 1, '5': 11, '6': '.qaul.sys.ble.BleDeviceUnavailable', '9': 0, '10': 'deviceUnavailable'},
    {'1': 'direct_send', '3': 9, '4': 1, '5': 11, '6': '.qaul.sys.ble.BleDirectSend', '9': 0, '10': 'directSend'},
    {'1': 'direct_send_result', '3': 10, '4': 1, '5': 11, '6': '.qaul.sys.ble.BleDirectSendResult', '9': 0, '10': 'directSendResult'},
    {'1': 'direct_received', '3': 11, '4': 1, '5': 11, '6': '.qaul.sys.ble.BleDirectReceived', '9': 0, '10': 'directReceived'},
  ],
  '8': [
    {'1': 'message'},
  ],
};

/// Descriptor for `Ble`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List bleDescriptor = $convert.base64Decode(
    'CgNCbGUSQQoMaW5mb19yZXF1ZXN0GAEgASgLMhwucWF1bC5zeXMuYmxlLkJsZUluZm9SZXF1ZX'
    'N0SABSC2luZm9SZXF1ZXN0EkQKDWluZm9fcmVzcG9uc2UYAiABKAsyHS5xYXVsLnN5cy5ibGUu'
    'QmxlSW5mb1Jlc3BvbnNlSABSDGluZm9SZXNwb25zZRJECg1zdGFydF9yZXF1ZXN0GAMgASgLMh'
    '0ucWF1bC5zeXMuYmxlLkJsZVN0YXJ0UmVxdWVzdEgAUgxzdGFydFJlcXVlc3QSQQoMc3RhcnRf'
    'cmVzdWx0GAQgASgLMhwucWF1bC5zeXMuYmxlLkJsZVN0YXJ0UmVzdWx0SABSC3N0YXJ0UmVzdW'
    'x0EkEKDHN0b3BfcmVxdWVzdBgFIAEoCzIcLnFhdWwuc3lzLmJsZS5CbGVTdG9wUmVxdWVzdEgA'
    'UgtzdG9wUmVxdWVzdBI+CgtzdG9wX3Jlc3VsdBgGIAEoCzIbLnFhdWwuc3lzLmJsZS5CbGVTdG'
    '9wUmVzdWx0SABSCnN0b3BSZXN1bHQSUAoRZGV2aWNlX2Rpc2NvdmVyZWQYByABKAsyIS5xYXVs'
    'LnN5cy5ibGUuQmxlRGV2aWNlRGlzY292ZXJlZEgAUhBkZXZpY2VEaXNjb3ZlcmVkElMKEmRldm'
    'ljZV91bmF2YWlsYWJsZRgIIAEoCzIiLnFhdWwuc3lzLmJsZS5CbGVEZXZpY2VVbmF2YWlsYWJs'
    'ZUgAUhFkZXZpY2VVbmF2YWlsYWJsZRI+CgtkaXJlY3Rfc2VuZBgJIAEoCzIbLnFhdWwuc3lzLm'
    'JsZS5CbGVEaXJlY3RTZW5kSABSCmRpcmVjdFNlbmQSUQoSZGlyZWN0X3NlbmRfcmVzdWx0GAog'
    'ASgLMiEucWF1bC5zeXMuYmxlLkJsZURpcmVjdFNlbmRSZXN1bHRIAFIQZGlyZWN0U2VuZFJlc3'
    'VsdBJKCg9kaXJlY3RfcmVjZWl2ZWQYCyABKAsyHy5xYXVsLnN5cy5ibGUuQmxlRGlyZWN0UmVj'
    'ZWl2ZWRIAFIOZGlyZWN0UmVjZWl2ZWRCCQoHbWVzc2FnZQ==');

@$core.Deprecated('Use bleInfoRequestDescriptor instead')
const BleInfoRequest$json = {
  '1': 'BleInfoRequest',
};

/// Descriptor for `BleInfoRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List bleInfoRequestDescriptor = $convert.base64Decode(
    'Cg5CbGVJbmZvUmVxdWVzdA==');

@$core.Deprecated('Use bleInfoResponseDescriptor instead')
const BleInfoResponse$json = {
  '1': 'BleInfoResponse',
  '2': [
    {'1': 'device', '3': 1, '4': 1, '5': 11, '6': '.qaul.sys.ble.BleDeviceInfo', '10': 'device'},
  ],
};

/// Descriptor for `BleInfoResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List bleInfoResponseDescriptor = $convert.base64Decode(
    'Cg9CbGVJbmZvUmVzcG9uc2USMwoGZGV2aWNlGAEgASgLMhsucWF1bC5zeXMuYmxlLkJsZURldm'
    'ljZUluZm9SBmRldmljZQ==');

@$core.Deprecated('Use bleDeviceInfoDescriptor instead')
const BleDeviceInfo$json = {
  '1': 'BleDeviceInfo',
  '2': [
    {'1': 'ble_support', '3': 1, '4': 1, '5': 8, '10': 'bleSupport'},
    {'1': 'id', '3': 2, '4': 1, '5': 9, '10': 'id'},
    {'1': 'name', '3': 3, '4': 1, '5': 9, '10': 'name'},
    {'1': 'bluetooth_on', '3': 4, '4': 1, '5': 8, '10': 'bluetoothOn'},
    {'1': 'adv_extended', '3': 5, '4': 1, '5': 8, '10': 'advExtended'},
    {'1': 'adv_extended_bytes', '3': 6, '4': 1, '5': 13, '10': 'advExtendedBytes'},
    {'1': 'le_2m', '3': 7, '4': 1, '5': 8, '10': 'le2m'},
    {'1': 'le_coded', '3': 8, '4': 1, '5': 8, '10': 'leCoded'},
    {'1': 'le_audio', '3': 9, '4': 1, '5': 8, '10': 'leAudio'},
    {'1': 'le_periodic_adv_support', '3': 14, '4': 1, '5': 8, '10': 'lePeriodicAdvSupport'},
    {'1': 'le_multiple_adv_support', '3': 15, '4': 1, '5': 8, '10': 'leMultipleAdvSupport'},
    {'1': 'offload_filter_support', '3': 16, '4': 1, '5': 8, '10': 'offloadFilterSupport'},
    {'1': 'offload_scan_batching_support', '3': 17, '4': 1, '5': 8, '10': 'offloadScanBatchingSupport'},
  ],
};

/// Descriptor for `BleDeviceInfo`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List bleDeviceInfoDescriptor = $convert.base64Decode(
    'Cg1CbGVEZXZpY2VJbmZvEh8KC2JsZV9zdXBwb3J0GAEgASgIUgpibGVTdXBwb3J0Eg4KAmlkGA'
    'IgASgJUgJpZBISCgRuYW1lGAMgASgJUgRuYW1lEiEKDGJsdWV0b290aF9vbhgEIAEoCFILYmx1'
    'ZXRvb3RoT24SIQoMYWR2X2V4dGVuZGVkGAUgASgIUgthZHZFeHRlbmRlZBIsChJhZHZfZXh0ZW'
    '5kZWRfYnl0ZXMYBiABKA1SEGFkdkV4dGVuZGVkQnl0ZXMSEwoFbGVfMm0YByABKAhSBGxlMm0S'
    'GQoIbGVfY29kZWQYCCABKAhSB2xlQ29kZWQSGQoIbGVfYXVkaW8YCSABKAhSB2xlQXVkaW8SNQ'
    'oXbGVfcGVyaW9kaWNfYWR2X3N1cHBvcnQYDiABKAhSFGxlUGVyaW9kaWNBZHZTdXBwb3J0EjUK'
    'F2xlX211bHRpcGxlX2Fkdl9zdXBwb3J0GA8gASgIUhRsZU11bHRpcGxlQWR2U3VwcG9ydBI0Ch'
    'ZvZmZsb2FkX2ZpbHRlcl9zdXBwb3J0GBAgASgIUhRvZmZsb2FkRmlsdGVyU3VwcG9ydBJBCh1v'
    'ZmZsb2FkX3NjYW5fYmF0Y2hpbmdfc3VwcG9ydBgRIAEoCFIab2ZmbG9hZFNjYW5CYXRjaGluZ1'
    'N1cHBvcnQ=');

@$core.Deprecated('Use bleStartRequestDescriptor instead')
const BleStartRequest$json = {
  '1': 'BleStartRequest',
  '2': [
    {'1': 'qaul_id', '3': 1, '4': 1, '5': 12, '10': 'qaulId'},
    {'1': 'power_setting', '3': 2, '4': 1, '5': 14, '6': '.qaul.sys.ble.BlePowerSetting', '10': 'powerSetting'},
  ],
};

/// Descriptor for `BleStartRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List bleStartRequestDescriptor = $convert.base64Decode(
    'Cg9CbGVTdGFydFJlcXVlc3QSFwoHcWF1bF9pZBgBIAEoDFIGcWF1bElkEkIKDXBvd2VyX3NldH'
    'RpbmcYAiABKA4yHS5xYXVsLnN5cy5ibGUuQmxlUG93ZXJTZXR0aW5nUgxwb3dlclNldHRpbmc=');

@$core.Deprecated('Use bleStartResultDescriptor instead')
const BleStartResult$json = {
  '1': 'BleStartResult',
  '2': [
    {'1': 'success', '3': 1, '4': 1, '5': 8, '10': 'success'},
    {'1': 'error_reason', '3': 2, '4': 1, '5': 14, '6': '.qaul.sys.ble.BleError', '10': 'errorReason'},
    {'1': 'error_message', '3': 3, '4': 1, '5': 9, '10': 'errorMessage'},
  ],
};

/// Descriptor for `BleStartResult`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List bleStartResultDescriptor = $convert.base64Decode(
    'Cg5CbGVTdGFydFJlc3VsdBIYCgdzdWNjZXNzGAEgASgIUgdzdWNjZXNzEjkKDGVycm9yX3JlYX'
    'NvbhgCIAEoDjIWLnFhdWwuc3lzLmJsZS5CbGVFcnJvclILZXJyb3JSZWFzb24SIwoNZXJyb3Jf'
    'bWVzc2FnZRgDIAEoCVIMZXJyb3JNZXNzYWdl');

@$core.Deprecated('Use bleStopRequestDescriptor instead')
const BleStopRequest$json = {
  '1': 'BleStopRequest',
};

/// Descriptor for `BleStopRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List bleStopRequestDescriptor = $convert.base64Decode(
    'Cg5CbGVTdG9wUmVxdWVzdA==');

@$core.Deprecated('Use bleStopResultDescriptor instead')
const BleStopResult$json = {
  '1': 'BleStopResult',
  '2': [
    {'1': 'success', '3': 1, '4': 1, '5': 8, '10': 'success'},
    {'1': 'error_reason', '3': 2, '4': 1, '5': 14, '6': '.qaul.sys.ble.BleError', '10': 'errorReason'},
    {'1': 'error_message', '3': 3, '4': 1, '5': 9, '10': 'errorMessage'},
  ],
};

/// Descriptor for `BleStopResult`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List bleStopResultDescriptor = $convert.base64Decode(
    'Cg1CbGVTdG9wUmVzdWx0EhgKB3N1Y2Nlc3MYASABKAhSB3N1Y2Nlc3MSOQoMZXJyb3JfcmVhc2'
    '9uGAIgASgOMhYucWF1bC5zeXMuYmxlLkJsZUVycm9yUgtlcnJvclJlYXNvbhIjCg1lcnJvcl9t'
    'ZXNzYWdlGAMgASgJUgxlcnJvck1lc3NhZ2U=');

@$core.Deprecated('Use bleDeviceDiscoveredDescriptor instead')
const BleDeviceDiscovered$json = {
  '1': 'BleDeviceDiscovered',
  '2': [
    {'1': 'qaul_id', '3': 1, '4': 1, '5': 12, '10': 'qaulId'},
    {'1': 'rssi', '3': 2, '4': 1, '5': 5, '10': 'rssi'},
  ],
};

/// Descriptor for `BleDeviceDiscovered`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List bleDeviceDiscoveredDescriptor = $convert.base64Decode(
    'ChNCbGVEZXZpY2VEaXNjb3ZlcmVkEhcKB3FhdWxfaWQYASABKAxSBnFhdWxJZBISCgRyc3NpGA'
    'IgASgFUgRyc3Np');

@$core.Deprecated('Use bleDeviceUnavailableDescriptor instead')
const BleDeviceUnavailable$json = {
  '1': 'BleDeviceUnavailable',
  '2': [
    {'1': 'qaul_id', '3': 1, '4': 1, '5': 12, '10': 'qaulId'},
  ],
};

/// Descriptor for `BleDeviceUnavailable`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List bleDeviceUnavailableDescriptor = $convert.base64Decode(
    'ChRCbGVEZXZpY2VVbmF2YWlsYWJsZRIXCgdxYXVsX2lkGAEgASgMUgZxYXVsSWQ=');

@$core.Deprecated('Use bleDirectSendDescriptor instead')
const BleDirectSend$json = {
  '1': 'BleDirectSend',
  '2': [
    {'1': 'message_id', '3': 1, '4': 1, '5': 12, '10': 'messageId'},
    {'1': 'receiver_id', '3': 2, '4': 1, '5': 12, '10': 'receiverId'},
    {'1': 'sender_id', '3': 3, '4': 1, '5': 12, '10': 'senderId'},
    {'1': 'data', '3': 4, '4': 1, '5': 12, '10': 'data'},
  ],
};

/// Descriptor for `BleDirectSend`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List bleDirectSendDescriptor = $convert.base64Decode(
    'Cg1CbGVEaXJlY3RTZW5kEh0KCm1lc3NhZ2VfaWQYASABKAxSCW1lc3NhZ2VJZBIfCgtyZWNlaX'
    'Zlcl9pZBgCIAEoDFIKcmVjZWl2ZXJJZBIbCglzZW5kZXJfaWQYAyABKAxSCHNlbmRlcklkEhIK'
    'BGRhdGEYBCABKAxSBGRhdGE=');

@$core.Deprecated('Use bleDirectSendResultDescriptor instead')
const BleDirectSendResult$json = {
  '1': 'BleDirectSendResult',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 12, '10': 'id'},
    {'1': 'success', '3': 2, '4': 1, '5': 8, '10': 'success'},
    {'1': 'error_message', '3': 3, '4': 1, '5': 9, '10': 'errorMessage'},
  ],
};

/// Descriptor for `BleDirectSendResult`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List bleDirectSendResultDescriptor = $convert.base64Decode(
    'ChNCbGVEaXJlY3RTZW5kUmVzdWx0Eg4KAmlkGAEgASgMUgJpZBIYCgdzdWNjZXNzGAIgASgIUg'
    'dzdWNjZXNzEiMKDWVycm9yX21lc3NhZ2UYAyABKAlSDGVycm9yTWVzc2FnZQ==');

@$core.Deprecated('Use bleDirectReceivedDescriptor instead')
const BleDirectReceived$json = {
  '1': 'BleDirectReceived',
  '2': [
    {'1': 'from', '3': 1, '4': 1, '5': 12, '10': 'from'},
    {'1': 'data', '3': 4, '4': 1, '5': 12, '10': 'data'},
  ],
};

/// Descriptor for `BleDirectReceived`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List bleDirectReceivedDescriptor = $convert.base64Decode(
    'ChFCbGVEaXJlY3RSZWNlaXZlZBISCgRmcm9tGAEgASgMUgRmcm9tEhIKBGRhdGEYBCABKAxSBG'
    'RhdGE=');

