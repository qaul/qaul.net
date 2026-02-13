// This is a generated file - do not edit.
//
// Generated from connections/ble/ble.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'ble.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'ble.pbenum.dart';

enum Ble_Message {
  infoRequest,
  infoResponse,
  startRequest,
  startResult,
  stopRequest,
  stopResult,
  deviceDiscovered,
  deviceUnavailable,
  directSend,
  directSendResult,
  directReceived,
  notSet
}

/// BLE system communication message
class Ble extends $pb.GeneratedMessage {
  factory Ble({
    BleInfoRequest? infoRequest,
    BleInfoResponse? infoResponse,
    BleStartRequest? startRequest,
    BleStartResult? startResult,
    BleStopRequest? stopRequest,
    BleStopResult? stopResult,
    BleDeviceDiscovered? deviceDiscovered,
    BleDeviceUnavailable? deviceUnavailable,
    BleDirectSend? directSend,
    BleDirectSendResult? directSendResult,
    BleDirectReceived? directReceived,
  }) {
    final result = create();
    if (infoRequest != null) result.infoRequest = infoRequest;
    if (infoResponse != null) result.infoResponse = infoResponse;
    if (startRequest != null) result.startRequest = startRequest;
    if (startResult != null) result.startResult = startResult;
    if (stopRequest != null) result.stopRequest = stopRequest;
    if (stopResult != null) result.stopResult = stopResult;
    if (deviceDiscovered != null) result.deviceDiscovered = deviceDiscovered;
    if (deviceUnavailable != null) result.deviceUnavailable = deviceUnavailable;
    if (directSend != null) result.directSend = directSend;
    if (directSendResult != null) result.directSendResult = directSendResult;
    if (directReceived != null) result.directReceived = directReceived;
    return result;
  }

  Ble._();

  factory Ble.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory Ble.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, Ble_Message> _Ble_MessageByTag = {
    1: Ble_Message.infoRequest,
    2: Ble_Message.infoResponse,
    3: Ble_Message.startRequest,
    4: Ble_Message.startResult,
    5: Ble_Message.stopRequest,
    6: Ble_Message.stopResult,
    7: Ble_Message.deviceDiscovered,
    8: Ble_Message.deviceUnavailable,
    9: Ble_Message.directSend,
    10: Ble_Message.directSendResult,
    11: Ble_Message.directReceived,
    0: Ble_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Ble',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.sys.ble'),
      createEmptyInstance: create)
    ..oo(0, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11])
    ..aOM<BleInfoRequest>(1, _omitFieldNames ? '' : 'infoRequest',
        subBuilder: BleInfoRequest.create)
    ..aOM<BleInfoResponse>(2, _omitFieldNames ? '' : 'infoResponse',
        subBuilder: BleInfoResponse.create)
    ..aOM<BleStartRequest>(3, _omitFieldNames ? '' : 'startRequest',
        subBuilder: BleStartRequest.create)
    ..aOM<BleStartResult>(4, _omitFieldNames ? '' : 'startResult',
        subBuilder: BleStartResult.create)
    ..aOM<BleStopRequest>(5, _omitFieldNames ? '' : 'stopRequest',
        subBuilder: BleStopRequest.create)
    ..aOM<BleStopResult>(6, _omitFieldNames ? '' : 'stopResult',
        subBuilder: BleStopResult.create)
    ..aOM<BleDeviceDiscovered>(7, _omitFieldNames ? '' : 'deviceDiscovered',
        subBuilder: BleDeviceDiscovered.create)
    ..aOM<BleDeviceUnavailable>(8, _omitFieldNames ? '' : 'deviceUnavailable',
        subBuilder: BleDeviceUnavailable.create)
    ..aOM<BleDirectSend>(9, _omitFieldNames ? '' : 'directSend',
        subBuilder: BleDirectSend.create)
    ..aOM<BleDirectSendResult>(10, _omitFieldNames ? '' : 'directSendResult',
        subBuilder: BleDirectSendResult.create)
    ..aOM<BleDirectReceived>(11, _omitFieldNames ? '' : 'directReceived',
        subBuilder: BleDirectReceived.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Ble clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Ble copyWith(void Function(Ble) updates) =>
      super.copyWith((message) => updates(message as Ble)) as Ble;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Ble create() => Ble._();
  @$core.override
  Ble createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static Ble getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Ble>(create);
  static Ble? _defaultInstance;

  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  @$pb.TagNumber(6)
  @$pb.TagNumber(7)
  @$pb.TagNumber(8)
  @$pb.TagNumber(9)
  @$pb.TagNumber(10)
  @$pb.TagNumber(11)
  Ble_Message whichMessage() => _Ble_MessageByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  @$pb.TagNumber(6)
  @$pb.TagNumber(7)
  @$pb.TagNumber(8)
  @$pb.TagNumber(9)
  @$pb.TagNumber(10)
  @$pb.TagNumber(11)
  void clearMessage() => $_clearField($_whichOneof(0));

  /// device information request
  @$pb.TagNumber(1)
  BleInfoRequest get infoRequest => $_getN(0);
  @$pb.TagNumber(1)
  set infoRequest(BleInfoRequest value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasInfoRequest() => $_has(0);
  @$pb.TagNumber(1)
  void clearInfoRequest() => $_clearField(1);
  @$pb.TagNumber(1)
  BleInfoRequest ensureInfoRequest() => $_ensure(0);

  /// device information response
  @$pb.TagNumber(2)
  BleInfoResponse get infoResponse => $_getN(1);
  @$pb.TagNumber(2)
  set infoResponse(BleInfoResponse value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasInfoResponse() => $_has(1);
  @$pb.TagNumber(2)
  void clearInfoResponse() => $_clearField(2);
  @$pb.TagNumber(2)
  BleInfoResponse ensureInfoResponse() => $_ensure(1);

  /// start device request
  @$pb.TagNumber(3)
  BleStartRequest get startRequest => $_getN(2);
  @$pb.TagNumber(3)
  set startRequest(BleStartRequest value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasStartRequest() => $_has(2);
  @$pb.TagNumber(3)
  void clearStartRequest() => $_clearField(3);
  @$pb.TagNumber(3)
  BleStartRequest ensureStartRequest() => $_ensure(2);

  /// start device result
  @$pb.TagNumber(4)
  BleStartResult get startResult => $_getN(3);
  @$pb.TagNumber(4)
  set startResult(BleStartResult value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasStartResult() => $_has(3);
  @$pb.TagNumber(4)
  void clearStartResult() => $_clearField(4);
  @$pb.TagNumber(4)
  BleStartResult ensureStartResult() => $_ensure(3);

  /// stop device request
  @$pb.TagNumber(5)
  BleStopRequest get stopRequest => $_getN(4);
  @$pb.TagNumber(5)
  set stopRequest(BleStopRequest value) => $_setField(5, value);
  @$pb.TagNumber(5)
  $core.bool hasStopRequest() => $_has(4);
  @$pb.TagNumber(5)
  void clearStopRequest() => $_clearField(5);
  @$pb.TagNumber(5)
  BleStopRequest ensureStopRequest() => $_ensure(4);

  /// stop device result
  @$pb.TagNumber(6)
  BleStopResult get stopResult => $_getN(5);
  @$pb.TagNumber(6)
  set stopResult(BleStopResult value) => $_setField(6, value);
  @$pb.TagNumber(6)
  $core.bool hasStopResult() => $_has(5);
  @$pb.TagNumber(6)
  void clearStopResult() => $_clearField(6);
  @$pb.TagNumber(6)
  BleStopResult ensureStopResult() => $_ensure(5);

  /// device discovered
  @$pb.TagNumber(7)
  BleDeviceDiscovered get deviceDiscovered => $_getN(6);
  @$pb.TagNumber(7)
  set deviceDiscovered(BleDeviceDiscovered value) => $_setField(7, value);
  @$pb.TagNumber(7)
  $core.bool hasDeviceDiscovered() => $_has(6);
  @$pb.TagNumber(7)
  void clearDeviceDiscovered() => $_clearField(7);
  @$pb.TagNumber(7)
  BleDeviceDiscovered ensureDeviceDiscovered() => $_ensure(6);

  /// device became unavailable
  @$pb.TagNumber(8)
  BleDeviceUnavailable get deviceUnavailable => $_getN(7);
  @$pb.TagNumber(8)
  set deviceUnavailable(BleDeviceUnavailable value) => $_setField(8, value);
  @$pb.TagNumber(8)
  $core.bool hasDeviceUnavailable() => $_has(7);
  @$pb.TagNumber(8)
  void clearDeviceUnavailable() => $_clearField(8);
  @$pb.TagNumber(8)
  BleDeviceUnavailable ensureDeviceUnavailable() => $_ensure(7);

  /// send a direct message
  @$pb.TagNumber(9)
  BleDirectSend get directSend => $_getN(8);
  @$pb.TagNumber(9)
  set directSend(BleDirectSend value) => $_setField(9, value);
  @$pb.TagNumber(9)
  $core.bool hasDirectSend() => $_has(8);
  @$pb.TagNumber(9)
  void clearDirectSend() => $_clearField(9);
  @$pb.TagNumber(9)
  BleDirectSend ensureDirectSend() => $_ensure(8);

  /// direct message send result
  @$pb.TagNumber(10)
  BleDirectSendResult get directSendResult => $_getN(9);
  @$pb.TagNumber(10)
  set directSendResult(BleDirectSendResult value) => $_setField(10, value);
  @$pb.TagNumber(10)
  $core.bool hasDirectSendResult() => $_has(9);
  @$pb.TagNumber(10)
  void clearDirectSendResult() => $_clearField(10);
  @$pb.TagNumber(10)
  BleDirectSendResult ensureDirectSendResult() => $_ensure(9);

  /// direct message received
  @$pb.TagNumber(11)
  BleDirectReceived get directReceived => $_getN(10);
  @$pb.TagNumber(11)
  set directReceived(BleDirectReceived value) => $_setField(11, value);
  @$pb.TagNumber(11)
  $core.bool hasDirectReceived() => $_has(10);
  @$pb.TagNumber(11)
  void clearDirectReceived() => $_clearField(11);
  @$pb.TagNumber(11)
  BleDirectReceived ensureDirectReceived() => $_ensure(10);
}

/// device information request message
class BleInfoRequest extends $pb.GeneratedMessage {
  factory BleInfoRequest() => create();

  BleInfoRequest._();

  factory BleInfoRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory BleInfoRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BleInfoRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.sys.ble'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BleInfoRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BleInfoRequest copyWith(void Function(BleInfoRequest) updates) =>
      super.copyWith((message) => updates(message as BleInfoRequest))
          as BleInfoRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static BleInfoRequest create() => BleInfoRequest._();
  @$core.override
  BleInfoRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static BleInfoRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BleInfoRequest>(create);
  static BleInfoRequest? _defaultInstance;
}

/// device information response message
class BleInfoResponse extends $pb.GeneratedMessage {
  factory BleInfoResponse({
    BleDeviceInfo? device,
  }) {
    final result = create();
    if (device != null) result.device = device;
    return result;
  }

  BleInfoResponse._();

  factory BleInfoResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory BleInfoResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BleInfoResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.sys.ble'),
      createEmptyInstance: create)
    ..aOM<BleDeviceInfo>(1, _omitFieldNames ? '' : 'device',
        subBuilder: BleDeviceInfo.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BleInfoResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BleInfoResponse copyWith(void Function(BleInfoResponse) updates) =>
      super.copyWith((message) => updates(message as BleInfoResponse))
          as BleInfoResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static BleInfoResponse create() => BleInfoResponse._();
  @$core.override
  BleInfoResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static BleInfoResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BleInfoResponse>(create);
  static BleInfoResponse? _defaultInstance;

  /// fill in a device information of the BLE device
  @$pb.TagNumber(1)
  BleDeviceInfo get device => $_getN(0);
  @$pb.TagNumber(1)
  set device(BleDeviceInfo value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasDevice() => $_has(0);
  @$pb.TagNumber(1)
  void clearDevice() => $_clearField(1);
  @$pb.TagNumber(1)
  BleDeviceInfo ensureDevice() => $_ensure(0);
}

/// BLE device information
class BleDeviceInfo extends $pb.GeneratedMessage {
  factory BleDeviceInfo({
    $core.bool? bleSupport,
    $core.String? id,
    $core.String? name,
    $core.bool? bluetoothOn,
    $core.bool? advExtended,
    $core.int? advExtendedBytes,
    $core.bool? le2m,
    $core.bool? leCoded,
    $core.bool? leAudio,
    $core.bool? lePeriodicAdvSupport,
    $core.bool? leMultipleAdvSupport,
    $core.bool? offloadFilterSupport,
    $core.bool? offloadScanBatchingSupport,
  }) {
    final result = create();
    if (bleSupport != null) result.bleSupport = bleSupport;
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (bluetoothOn != null) result.bluetoothOn = bluetoothOn;
    if (advExtended != null) result.advExtended = advExtended;
    if (advExtendedBytes != null) result.advExtendedBytes = advExtendedBytes;
    if (le2m != null) result.le2m = le2m;
    if (leCoded != null) result.leCoded = leCoded;
    if (leAudio != null) result.leAudio = leAudio;
    if (lePeriodicAdvSupport != null)
      result.lePeriodicAdvSupport = lePeriodicAdvSupport;
    if (leMultipleAdvSupport != null)
      result.leMultipleAdvSupport = leMultipleAdvSupport;
    if (offloadFilterSupport != null)
      result.offloadFilterSupport = offloadFilterSupport;
    if (offloadScanBatchingSupport != null)
      result.offloadScanBatchingSupport = offloadScanBatchingSupport;
    return result;
  }

  BleDeviceInfo._();

  factory BleDeviceInfo.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory BleDeviceInfo.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BleDeviceInfo',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.sys.ble'),
      createEmptyInstance: create)
    ..aOB(1, _omitFieldNames ? '' : 'bleSupport')
    ..aOS(2, _omitFieldNames ? '' : 'id')
    ..aOS(3, _omitFieldNames ? '' : 'name')
    ..aOB(4, _omitFieldNames ? '' : 'bluetoothOn')
    ..aOB(5, _omitFieldNames ? '' : 'advExtended')
    ..aI(6, _omitFieldNames ? '' : 'advExtendedBytes',
        fieldType: $pb.PbFieldType.OU3)
    ..aOB(7, _omitFieldNames ? '' : 'le2m', protoName: 'le_2m')
    ..aOB(8, _omitFieldNames ? '' : 'leCoded')
    ..aOB(9, _omitFieldNames ? '' : 'leAudio')
    ..aOB(14, _omitFieldNames ? '' : 'lePeriodicAdvSupport')
    ..aOB(15, _omitFieldNames ? '' : 'leMultipleAdvSupport')
    ..aOB(16, _omitFieldNames ? '' : 'offloadFilterSupport')
    ..aOB(17, _omitFieldNames ? '' : 'offloadScanBatchingSupport')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BleDeviceInfo clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BleDeviceInfo copyWith(void Function(BleDeviceInfo) updates) =>
      super.copyWith((message) => updates(message as BleDeviceInfo))
          as BleDeviceInfo;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static BleDeviceInfo create() => BleDeviceInfo._();
  @$core.override
  BleDeviceInfo createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static BleDeviceInfo getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BleDeviceInfo>(create);
  static BleDeviceInfo? _defaultInstance;

  /// Check if Bluetooth / Bluetooth Low Energy is supported
  ///
  /// Android: check if a bluetooth adapter is found
  @$pb.TagNumber(1)
  $core.bool get bleSupport => $_getBF(0);
  @$pb.TagNumber(1)
  set bleSupport($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasBleSupport() => $_has(0);
  @$pb.TagNumber(1)
  void clearBleSupport() => $_clearField(1);

  /// Bluetooth device address
  /// 48 bit unique Bluetooth device addr
  /// e.g. 80:86:F2:08:C7:98
  ///
  /// Android: BluetoothAdapter getAddress()
  /// https://developer.android.com/reference/kotlin/android/bluetooth/BluetoothAdapter#getAddress()
  @$pb.TagNumber(2)
  $core.String get id => $_getSZ(1);
  @$pb.TagNumber(2)
  set id($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasId() => $_has(1);
  @$pb.TagNumber(2)
  void clearId() => $_clearField(2);

  /// Get Bluetooth Name
  /// this is field is purely informative
  ///
  /// Android: BluetoothAdapter getName()
  /// https://developer.android.com/reference/kotlin/android/bluetooth/BluetoothAdapter#getName()
  @$pb.TagNumber(3)
  $core.String get name => $_getSZ(2);
  @$pb.TagNumber(3)
  set name($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasName() => $_has(2);
  @$pb.TagNumber(3)
  void clearName() => $_clearField(3);

  /// Bluetooth is enable / powered on
  ///
  /// Android: BluetoothAdapter isEnabled()
  /// https://developer.android.com/reference/kotlin/android/bluetooth/BluetoothAdapter#isEnabled()
  @$pb.TagNumber(4)
  $core.bool get bluetoothOn => $_getBF(3);
  @$pb.TagNumber(4)
  set bluetoothOn($core.bool value) => $_setBool(3, value);
  @$pb.TagNumber(4)
  $core.bool hasBluetoothOn() => $_has(3);
  @$pb.TagNumber(4)
  void clearBluetoothOn() => $_clearField(4);

  /// Is extended advertisement supported?
  ///
  /// Android: BluetoothAdapter isLeExtendedAdvertisingSupported ()
  /// https://developer.android.com/reference/android/bluetooth/BluetoothAdapter#isLeExtendedAdvertisingSupported()
  @$pb.TagNumber(5)
  $core.bool get advExtended => $_getBF(4);
  @$pb.TagNumber(5)
  set advExtended($core.bool value) => $_setBool(4, value);
  @$pb.TagNumber(5)
  $core.bool hasAdvExtended() => $_has(4);
  @$pb.TagNumber(5)
  void clearAdvExtended() => $_clearField(5);

  /// what is the maximal amount of bytes sendable via advertising?
  ///
  /// Android: BluetoothAdapter getLeMaximumAdvertisingDataLength()
  /// https://developer.android.com/reference/android/bluetooth/BluetoothAdapter#getLeMaximumAdvertisingDataLength()
  @$pb.TagNumber(6)
  $core.int get advExtendedBytes => $_getIZ(5);
  @$pb.TagNumber(6)
  set advExtendedBytes($core.int value) => $_setUnsignedInt32(5, value);
  @$pb.TagNumber(6)
  $core.bool hasAdvExtendedBytes() => $_has(5);
  @$pb.TagNumber(6)
  void clearAdvExtendedBytes() => $_clearField(6);

  /// Is 2M phy supported?
  ///
  /// Android: BluetoothAdapter isLe2MPhySupported()
  /// https://developer.android.com/reference/android/bluetooth/BluetoothAdapter#isLe2MPhySupported()
  @$pb.TagNumber(7)
  $core.bool get le2m => $_getBF(6);
  @$pb.TagNumber(7)
  set le2m($core.bool value) => $_setBool(6, value);
  @$pb.TagNumber(7)
  $core.bool hasLe2m() => $_has(6);
  @$pb.TagNumber(7)
  void clearLe2m() => $_clearField(7);

  /// is extended advertising supported in coded
  /// mode? (For long distance connections)
  ///
  /// Android: BluetoothAdapter isLeCodedPhySupported()
  /// https://developer.android.com/reference/android/bluetooth/BluetoothAdapter#isLeCodedPhySupported()
  @$pb.TagNumber(8)
  $core.bool get leCoded => $_getBF(7);
  @$pb.TagNumber(8)
  set leCoded($core.bool value) => $_setBool(7, value);
  @$pb.TagNumber(8)
  $core.bool hasLeCoded() => $_has(7);
  @$pb.TagNumber(8)
  void clearLeCoded() => $_clearField(8);

  /// is LE audio supported?
  ///
  /// This is the most recent BLE feature, supported on:
  ///
  /// * android 12 and above
  /// * linux ?
  /// * ios ?
  /// * macos ?
  /// * windows ?
  ///
  /// Android: AndroidAdapter isLeAudioSupported()
  /// https://developer.android.com/reference/kotlin/android/bluetooth/BluetoothAdapter#isLeAudioSupported()
  @$pb.TagNumber(9)
  $core.bool get leAudio => $_getBF(8);
  @$pb.TagNumber(9)
  set leAudio($core.bool value) => $_setBool(8, value);
  @$pb.TagNumber(9)
  $core.bool hasLeAudio() => $_has(8);
  @$pb.TagNumber(9)
  void clearLeAudio() => $_clearField(9);

  /// is periodic advertisment supported?
  ///
  /// Android: BluetoothAdapter isLePeriodicAdvertisingSupported()
  /// https://developer.android.com/reference/android/bluetooth/BluetoothAdapter#isLePeriodicAdvertisingSupported()
  @$pb.TagNumber(14)
  $core.bool get lePeriodicAdvSupport => $_getBF(9);
  @$pb.TagNumber(14)
  set lePeriodicAdvSupport($core.bool value) => $_setBool(9, value);
  @$pb.TagNumber(14)
  $core.bool hasLePeriodicAdvSupport() => $_has(9);
  @$pb.TagNumber(14)
  void clearLePeriodicAdvSupport() => $_clearField(14);

  /// Is multi advertisement supported?
  ///
  /// When multi advertisement is supported one can have different
  /// advertisement types parallely. Each advertisement has a
  /// different device address.
  /// For scanning devices it looks, as if multiple devices devices
  /// would advertise themselves.
  /// This is helpful to support several incompatible advertisement
  /// modes at the same time.
  ///
  /// Android: BluetoothAdapter isMultipleAdvertisementSupported()
  /// https://developer.android.com/reference/android/bluetooth/BluetoothAdapter#isMultipleAdvertisementSupported()
  @$pb.TagNumber(15)
  $core.bool get leMultipleAdvSupport => $_getBF(10);
  @$pb.TagNumber(15)
  set leMultipleAdvSupport($core.bool value) => $_setBool(10, value);
  @$pb.TagNumber(15)
  $core.bool hasLeMultipleAdvSupport() => $_has(10);
  @$pb.TagNumber(15)
  void clearLeMultipleAdvSupport() => $_clearField(15);

  /// Android Specific: is Offloaded Filtering Supported?
  ///
  /// Android: BluetoothAdapter isOffloadedFilteringSupported()
  @$pb.TagNumber(16)
  $core.bool get offloadFilterSupport => $_getBF(11);
  @$pb.TagNumber(16)
  set offloadFilterSupport($core.bool value) => $_setBool(11, value);
  @$pb.TagNumber(16)
  $core.bool hasOffloadFilterSupport() => $_has(11);
  @$pb.TagNumber(16)
  void clearOffloadFilterSupport() => $_clearField(16);

  /// Android Specific: is Offloaded Scan Batching Supported?
  ///
  /// Android: BluetoothAdapter isOffloadedScanBatchingSupported()
  /// https://developer.android.com/reference/android/bluetooth/BluetoothAdapter#isOffloadedScanBatchingSupported()
  @$pb.TagNumber(17)
  $core.bool get offloadScanBatchingSupport => $_getBF(12);
  @$pb.TagNumber(17)
  set offloadScanBatchingSupport($core.bool value) => $_setBool(12, value);
  @$pb.TagNumber(17)
  $core.bool hasOffloadScanBatchingSupport() => $_has(12);
  @$pb.TagNumber(17)
  void clearOffloadScanBatchingSupport() => $_clearField(17);
}

/// Start Device
///
/// the module will try to start the device, power it up,
/// get all rights, configure it for qaul, and
/// send & receive advertising messages
class BleStartRequest extends $pb.GeneratedMessage {
  factory BleStartRequest({
    $core.List<$core.int>? qaulId,
    BlePowerSetting? powerSetting,
  }) {
    final result = create();
    if (qaulId != null) result.qaulId = qaulId;
    if (powerSetting != null) result.powerSetting = powerSetting;
    return result;
  }

  BleStartRequest._();

  factory BleStartRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory BleStartRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BleStartRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.sys.ble'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'qaulId', $pb.PbFieldType.OY)
    ..aE<BlePowerSetting>(2, _omitFieldNames ? '' : 'powerSetting',
        enumValues: BlePowerSetting.values)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BleStartRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BleStartRequest copyWith(void Function(BleStartRequest) updates) =>
      super.copyWith((message) => updates(message as BleStartRequest))
          as BleStartRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static BleStartRequest create() => BleStartRequest._();
  @$core.override
  BleStartRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static BleStartRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BleStartRequest>(create);
  static BleStartRequest? _defaultInstance;

  /// qaul ID
  ///
  /// The small 16 byte qaul id
  /// to be used to identify this node
  @$pb.TagNumber(1)
  $core.List<$core.int> get qaulId => $_getN(0);
  @$pb.TagNumber(1)
  set qaulId($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasQaulId() => $_has(0);
  @$pb.TagNumber(1)
  void clearQaulId() => $_clearField(1);

  /// power settings
  @$pb.TagNumber(2)
  BlePowerSetting get powerSetting => $_getN(1);
  @$pb.TagNumber(2)
  set powerSetting(BlePowerSetting value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasPowerSetting() => $_has(1);
  @$pb.TagNumber(2)
  void clearPowerSetting() => $_clearField(2);
}

/// Start device result message
///
/// Feedback from the
class BleStartResult extends $pb.GeneratedMessage {
  factory BleStartResult({
    $core.bool? success,
    BleError? errorReason,
    $core.String? errorMessage,
  }) {
    final result = create();
    if (success != null) result.success = success;
    if (errorReason != null) result.errorReason = errorReason;
    if (errorMessage != null) result.errorMessage = errorMessage;
    return result;
  }

  BleStartResult._();

  factory BleStartResult.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory BleStartResult.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BleStartResult',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.sys.ble'),
      createEmptyInstance: create)
    ..aOB(1, _omitFieldNames ? '' : 'success')
    ..aE<BleError>(2, _omitFieldNames ? '' : 'errorReason',
        enumValues: BleError.values)
    ..aOS(3, _omitFieldNames ? '' : 'errorMessage')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BleStartResult clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BleStartResult copyWith(void Function(BleStartResult) updates) =>
      super.copyWith((message) => updates(message as BleStartResult))
          as BleStartResult;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static BleStartResult create() => BleStartResult._();
  @$core.override
  BleStartResult createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static BleStartResult getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BleStartResult>(create);
  static BleStartResult? _defaultInstance;

  /// whether the device was successfully started
  @$pb.TagNumber(1)
  $core.bool get success => $_getBF(0);
  @$pb.TagNumber(1)
  set success($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSuccess() => $_has(0);
  @$pb.TagNumber(1)
  void clearSuccess() => $_clearField(1);

  /// error reason
  @$pb.TagNumber(2)
  BleError get errorReason => $_getN(1);
  @$pb.TagNumber(2)
  set errorReason(BleError value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasErrorReason() => $_has(1);
  @$pb.TagNumber(2)
  void clearErrorReason() => $_clearField(2);

  /// error message
  @$pb.TagNumber(3)
  $core.String get errorMessage => $_getSZ(2);
  @$pb.TagNumber(3)
  set errorMessage($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasErrorMessage() => $_has(2);
  @$pb.TagNumber(3)
  void clearErrorMessage() => $_clearField(3);
}

/// Stop Bluetooth Device
class BleStopRequest extends $pb.GeneratedMessage {
  factory BleStopRequest() => create();

  BleStopRequest._();

  factory BleStopRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory BleStopRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BleStopRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.sys.ble'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BleStopRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BleStopRequest copyWith(void Function(BleStopRequest) updates) =>
      super.copyWith((message) => updates(message as BleStopRequest))
          as BleStopRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static BleStopRequest create() => BleStopRequest._();
  @$core.override
  BleStopRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static BleStopRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BleStopRequest>(create);
  static BleStopRequest? _defaultInstance;
}

/// Stop Result
///
/// Feedback of the stop request
class BleStopResult extends $pb.GeneratedMessage {
  factory BleStopResult({
    $core.bool? success,
    BleError? errorReason,
    $core.String? errorMessage,
  }) {
    final result = create();
    if (success != null) result.success = success;
    if (errorReason != null) result.errorReason = errorReason;
    if (errorMessage != null) result.errorMessage = errorMessage;
    return result;
  }

  BleStopResult._();

  factory BleStopResult.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory BleStopResult.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BleStopResult',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.sys.ble'),
      createEmptyInstance: create)
    ..aOB(1, _omitFieldNames ? '' : 'success')
    ..aE<BleError>(2, _omitFieldNames ? '' : 'errorReason',
        enumValues: BleError.values)
    ..aOS(3, _omitFieldNames ? '' : 'errorMessage')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BleStopResult clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BleStopResult copyWith(void Function(BleStopResult) updates) =>
      super.copyWith((message) => updates(message as BleStopResult))
          as BleStopResult;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static BleStopResult create() => BleStopResult._();
  @$core.override
  BleStopResult createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static BleStopResult getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BleStopResult>(create);
  static BleStopResult? _defaultInstance;

  /// whether the device was successfully stopped
  @$pb.TagNumber(1)
  $core.bool get success => $_getBF(0);
  @$pb.TagNumber(1)
  set success($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSuccess() => $_has(0);
  @$pb.TagNumber(1)
  void clearSuccess() => $_clearField(1);

  /// error reason
  @$pb.TagNumber(2)
  BleError get errorReason => $_getN(1);
  @$pb.TagNumber(2)
  set errorReason(BleError value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasErrorReason() => $_has(1);
  @$pb.TagNumber(2)
  void clearErrorReason() => $_clearField(2);

  /// error message
  @$pb.TagNumber(3)
  $core.String get errorMessage => $_getSZ(2);
  @$pb.TagNumber(3)
  set errorMessage($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasErrorMessage() => $_has(2);
  @$pb.TagNumber(3)
  void clearErrorMessage() => $_clearField(3);
}

/// Device Discovered
///
/// A new device has been discovered.
class BleDeviceDiscovered extends $pb.GeneratedMessage {
  factory BleDeviceDiscovered({
    $core.List<$core.int>? qaulId,
    $core.int? rssi,
  }) {
    final result = create();
    if (qaulId != null) result.qaulId = qaulId;
    if (rssi != null) result.rssi = rssi;
    return result;
  }

  BleDeviceDiscovered._();

  factory BleDeviceDiscovered.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory BleDeviceDiscovered.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BleDeviceDiscovered',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.sys.ble'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'qaulId', $pb.PbFieldType.OY)
    ..aI(2, _omitFieldNames ? '' : 'rssi')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BleDeviceDiscovered clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BleDeviceDiscovered copyWith(void Function(BleDeviceDiscovered) updates) =>
      super.copyWith((message) => updates(message as BleDeviceDiscovered))
          as BleDeviceDiscovered;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static BleDeviceDiscovered create() => BleDeviceDiscovered._();
  @$core.override
  BleDeviceDiscovered createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static BleDeviceDiscovered getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BleDeviceDiscovered>(create);
  static BleDeviceDiscovered? _defaultInstance;

  /// qaul id of the device
  @$pb.TagNumber(1)
  $core.List<$core.int> get qaulId => $_getN(0);
  @$pb.TagNumber(1)
  set qaulId($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasQaulId() => $_has(0);
  @$pb.TagNumber(1)
  void clearQaulId() => $_clearField(1);

  /// the received signal strength of this device
  @$pb.TagNumber(2)
  $core.int get rssi => $_getIZ(1);
  @$pb.TagNumber(2)
  set rssi($core.int value) => $_setSignedInt32(1, value);
  @$pb.TagNumber(2)
  $core.bool hasRssi() => $_has(1);
  @$pb.TagNumber(2)
  void clearRssi() => $_clearField(2);
}

/// Device Unavailable
///
/// A formerly discovered device has become
/// unavailable. No messages can be sent to it.
class BleDeviceUnavailable extends $pb.GeneratedMessage {
  factory BleDeviceUnavailable({
    $core.List<$core.int>? qaulId,
  }) {
    final result = create();
    if (qaulId != null) result.qaulId = qaulId;
    return result;
  }

  BleDeviceUnavailable._();

  factory BleDeviceUnavailable.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory BleDeviceUnavailable.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BleDeviceUnavailable',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.sys.ble'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'qaulId', $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BleDeviceUnavailable clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BleDeviceUnavailable copyWith(void Function(BleDeviceUnavailable) updates) =>
      super.copyWith((message) => updates(message as BleDeviceUnavailable))
          as BleDeviceUnavailable;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static BleDeviceUnavailable create() => BleDeviceUnavailable._();
  @$core.override
  BleDeviceUnavailable createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static BleDeviceUnavailable getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BleDeviceUnavailable>(create);
  static BleDeviceUnavailable? _defaultInstance;

  /// qaul id of the device that
  /// became unavailable
  @$pb.TagNumber(1)
  $core.List<$core.int> get qaulId => $_getN(0);
  @$pb.TagNumber(1)
  set qaulId($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasQaulId() => $_has(0);
  @$pb.TagNumber(1)
  void clearQaulId() => $_clearField(1);
}

/// send a direct message
class BleDirectSend extends $pb.GeneratedMessage {
  factory BleDirectSend({
    $core.List<$core.int>? messageId,
    $core.List<$core.int>? receiverId,
    $core.List<$core.int>? senderId,
    $core.List<$core.int>? data,
  }) {
    final result = create();
    if (messageId != null) result.messageId = messageId;
    if (receiverId != null) result.receiverId = receiverId;
    if (senderId != null) result.senderId = senderId;
    if (data != null) result.data = data;
    return result;
  }

  BleDirectSend._();

  factory BleDirectSend.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory BleDirectSend.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BleDirectSend',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.sys.ble'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'messageId', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(
        2, _omitFieldNames ? '' : 'receiverId', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(
        3, _omitFieldNames ? '' : 'senderId', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(
        4, _omitFieldNames ? '' : 'data', $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BleDirectSend clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BleDirectSend copyWith(void Function(BleDirectSend) updates) =>
      super.copyWith((message) => updates(message as BleDirectSend))
          as BleDirectSend;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static BleDirectSend create() => BleDirectSend._();
  @$core.override
  BleDirectSend createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static BleDirectSend getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BleDirectSend>(create);
  static BleDirectSend? _defaultInstance;

  /// message id (as a reference for the result message)
  @$pb.TagNumber(1)
  $core.List<$core.int> get messageId => $_getN(0);
  @$pb.TagNumber(1)
  set messageId($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasMessageId() => $_has(0);
  @$pb.TagNumber(1)
  void clearMessageId() => $_clearField(1);

  /// qaul id of the device to send it to
  @$pb.TagNumber(2)
  $core.List<$core.int> get receiverId => $_getN(1);
  @$pb.TagNumber(2)
  set receiverId($core.List<$core.int> value) => $_setBytes(1, value);
  @$pb.TagNumber(2)
  $core.bool hasReceiverId() => $_has(1);
  @$pb.TagNumber(2)
  void clearReceiverId() => $_clearField(2);

  /// qaul id of the sending device
  @$pb.TagNumber(3)
  $core.List<$core.int> get senderId => $_getN(2);
  @$pb.TagNumber(3)
  set senderId($core.List<$core.int> value) => $_setBytes(2, value);
  @$pb.TagNumber(3)
  $core.bool hasSenderId() => $_has(2);
  @$pb.TagNumber(3)
  void clearSenderId() => $_clearField(3);

  /// data to be sent
  @$pb.TagNumber(4)
  $core.List<$core.int> get data => $_getN(3);
  @$pb.TagNumber(4)
  set data($core.List<$core.int> value) => $_setBytes(3, value);
  @$pb.TagNumber(4)
  $core.bool hasData() => $_has(3);
  @$pb.TagNumber(4)
  void clearData() => $_clearField(4);
}

/// result after sending the direct message
class BleDirectSendResult extends $pb.GeneratedMessage {
  factory BleDirectSendResult({
    $core.List<$core.int>? id,
    $core.bool? success,
    $core.String? errorMessage,
  }) {
    final result = create();
    if (id != null) result.id = id;
    if (success != null) result.success = success;
    if (errorMessage != null) result.errorMessage = errorMessage;
    return result;
  }

  BleDirectSendResult._();

  factory BleDirectSendResult.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory BleDirectSendResult.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BleDirectSendResult',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.sys.ble'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'id', $pb.PbFieldType.OY)
    ..aOB(2, _omitFieldNames ? '' : 'success')
    ..aOS(3, _omitFieldNames ? '' : 'errorMessage')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BleDirectSendResult clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BleDirectSendResult copyWith(void Function(BleDirectSendResult) updates) =>
      super.copyWith((message) => updates(message as BleDirectSendResult))
          as BleDirectSendResult;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static BleDirectSendResult create() => BleDirectSendResult._();
  @$core.override
  BleDirectSendResult createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static BleDirectSendResult getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BleDirectSendResult>(create);
  static BleDirectSendResult? _defaultInstance;

  /// message id
  @$pb.TagNumber(1)
  $core.List<$core.int> get id => $_getN(0);
  @$pb.TagNumber(1)
  set id($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  /// result after sending the message
  @$pb.TagNumber(2)
  $core.bool get success => $_getBF(1);
  @$pb.TagNumber(2)
  set success($core.bool value) => $_setBool(1, value);
  @$pb.TagNumber(2)
  $core.bool hasSuccess() => $_has(1);
  @$pb.TagNumber(2)
  void clearSuccess() => $_clearField(2);

  /// error messages
  @$pb.TagNumber(3)
  $core.String get errorMessage => $_getSZ(2);
  @$pb.TagNumber(3)
  set errorMessage($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasErrorMessage() => $_has(2);
  @$pb.TagNumber(3)
  void clearErrorMessage() => $_clearField(3);
}

/// direct message received message
class BleDirectReceived extends $pb.GeneratedMessage {
  factory BleDirectReceived({
    $core.List<$core.int>? from,
    $core.List<$core.int>? data,
  }) {
    final result = create();
    if (from != null) result.from = from;
    if (data != null) result.data = data;
    return result;
  }

  BleDirectReceived._();

  factory BleDirectReceived.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory BleDirectReceived.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BleDirectReceived',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.sys.ble'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'from', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(
        4, _omitFieldNames ? '' : 'data', $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BleDirectReceived clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BleDirectReceived copyWith(void Function(BleDirectReceived) updates) =>
      super.copyWith((message) => updates(message as BleDirectReceived))
          as BleDirectReceived;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static BleDirectReceived create() => BleDirectReceived._();
  @$core.override
  BleDirectReceived createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static BleDirectReceived getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BleDirectReceived>(create);
  static BleDirectReceived? _defaultInstance;

  /// qaul id of the sending device
  @$pb.TagNumber(1)
  $core.List<$core.int> get from => $_getN(0);
  @$pb.TagNumber(1)
  set from($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasFrom() => $_has(0);
  @$pb.TagNumber(1)
  void clearFrom() => $_clearField(1);

  /// the data received
  @$pb.TagNumber(4)
  $core.List<$core.int> get data => $_getN(1);
  @$pb.TagNumber(4)
  set data($core.List<$core.int> value) => $_setBytes(1, value);
  @$pb.TagNumber(4)
  $core.bool hasData() => $_has(1);
  @$pb.TagNumber(4)
  void clearData() => $_clearField(4);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
