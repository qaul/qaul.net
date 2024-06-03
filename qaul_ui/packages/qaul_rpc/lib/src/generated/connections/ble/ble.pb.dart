//
//  Generated code. Do not modify.
//  source: connections/ble/ble.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'ble.pbenum.dart';

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
    final $result = create();
    if (infoRequest != null) {
      $result.infoRequest = infoRequest;
    }
    if (infoResponse != null) {
      $result.infoResponse = infoResponse;
    }
    if (startRequest != null) {
      $result.startRequest = startRequest;
    }
    if (startResult != null) {
      $result.startResult = startResult;
    }
    if (stopRequest != null) {
      $result.stopRequest = stopRequest;
    }
    if (stopResult != null) {
      $result.stopResult = stopResult;
    }
    if (deviceDiscovered != null) {
      $result.deviceDiscovered = deviceDiscovered;
    }
    if (deviceUnavailable != null) {
      $result.deviceUnavailable = deviceUnavailable;
    }
    if (directSend != null) {
      $result.directSend = directSend;
    }
    if (directSendResult != null) {
      $result.directSendResult = directSendResult;
    }
    if (directReceived != null) {
      $result.directReceived = directReceived;
    }
    return $result;
  }
  Ble._() : super();
  factory Ble.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Ble.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static const $core.Map<$core.int, Ble_Message> _Ble_MessageByTag = {
    1 : Ble_Message.infoRequest,
    2 : Ble_Message.infoResponse,
    3 : Ble_Message.startRequest,
    4 : Ble_Message.startResult,
    5 : Ble_Message.stopRequest,
    6 : Ble_Message.stopResult,
    7 : Ble_Message.deviceDiscovered,
    8 : Ble_Message.deviceUnavailable,
    9 : Ble_Message.directSend,
    10 : Ble_Message.directSendResult,
    11 : Ble_Message.directReceived,
    0 : Ble_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Ble', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.sys.ble'), createEmptyInstance: create)
    ..oo(0, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11])
    ..aOM<BleInfoRequest>(1, _omitFieldNames ? '' : 'infoRequest', subBuilder: BleInfoRequest.create)
    ..aOM<BleInfoResponse>(2, _omitFieldNames ? '' : 'infoResponse', subBuilder: BleInfoResponse.create)
    ..aOM<BleStartRequest>(3, _omitFieldNames ? '' : 'startRequest', subBuilder: BleStartRequest.create)
    ..aOM<BleStartResult>(4, _omitFieldNames ? '' : 'startResult', subBuilder: BleStartResult.create)
    ..aOM<BleStopRequest>(5, _omitFieldNames ? '' : 'stopRequest', subBuilder: BleStopRequest.create)
    ..aOM<BleStopResult>(6, _omitFieldNames ? '' : 'stopResult', subBuilder: BleStopResult.create)
    ..aOM<BleDeviceDiscovered>(7, _omitFieldNames ? '' : 'deviceDiscovered', subBuilder: BleDeviceDiscovered.create)
    ..aOM<BleDeviceUnavailable>(8, _omitFieldNames ? '' : 'deviceUnavailable', subBuilder: BleDeviceUnavailable.create)
    ..aOM<BleDirectSend>(9, _omitFieldNames ? '' : 'directSend', subBuilder: BleDirectSend.create)
    ..aOM<BleDirectSendResult>(10, _omitFieldNames ? '' : 'directSendResult', subBuilder: BleDirectSendResult.create)
    ..aOM<BleDirectReceived>(11, _omitFieldNames ? '' : 'directReceived', subBuilder: BleDirectReceived.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Ble clone() => Ble()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Ble copyWith(void Function(Ble) updates) => super.copyWith((message) => updates(message as Ble)) as Ble;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Ble create() => Ble._();
  Ble createEmptyInstance() => create();
  static $pb.PbList<Ble> createRepeated() => $pb.PbList<Ble>();
  @$core.pragma('dart2js:noInline')
  static Ble getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Ble>(create);
  static Ble? _defaultInstance;

  Ble_Message whichMessage() => _Ble_MessageByTag[$_whichOneof(0)]!;
  void clearMessage() => clearField($_whichOneof(0));

  /// device information request
  @$pb.TagNumber(1)
  BleInfoRequest get infoRequest => $_getN(0);
  @$pb.TagNumber(1)
  set infoRequest(BleInfoRequest v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasInfoRequest() => $_has(0);
  @$pb.TagNumber(1)
  void clearInfoRequest() => clearField(1);
  @$pb.TagNumber(1)
  BleInfoRequest ensureInfoRequest() => $_ensure(0);

  /// device information response
  @$pb.TagNumber(2)
  BleInfoResponse get infoResponse => $_getN(1);
  @$pb.TagNumber(2)
  set infoResponse(BleInfoResponse v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasInfoResponse() => $_has(1);
  @$pb.TagNumber(2)
  void clearInfoResponse() => clearField(2);
  @$pb.TagNumber(2)
  BleInfoResponse ensureInfoResponse() => $_ensure(1);

  /// start device request
  @$pb.TagNumber(3)
  BleStartRequest get startRequest => $_getN(2);
  @$pb.TagNumber(3)
  set startRequest(BleStartRequest v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasStartRequest() => $_has(2);
  @$pb.TagNumber(3)
  void clearStartRequest() => clearField(3);
  @$pb.TagNumber(3)
  BleStartRequest ensureStartRequest() => $_ensure(2);

  /// start device result
  @$pb.TagNumber(4)
  BleStartResult get startResult => $_getN(3);
  @$pb.TagNumber(4)
  set startResult(BleStartResult v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasStartResult() => $_has(3);
  @$pb.TagNumber(4)
  void clearStartResult() => clearField(4);
  @$pb.TagNumber(4)
  BleStartResult ensureStartResult() => $_ensure(3);

  /// stop device request
  @$pb.TagNumber(5)
  BleStopRequest get stopRequest => $_getN(4);
  @$pb.TagNumber(5)
  set stopRequest(BleStopRequest v) { setField(5, v); }
  @$pb.TagNumber(5)
  $core.bool hasStopRequest() => $_has(4);
  @$pb.TagNumber(5)
  void clearStopRequest() => clearField(5);
  @$pb.TagNumber(5)
  BleStopRequest ensureStopRequest() => $_ensure(4);

  /// stop device result
  @$pb.TagNumber(6)
  BleStopResult get stopResult => $_getN(5);
  @$pb.TagNumber(6)
  set stopResult(BleStopResult v) { setField(6, v); }
  @$pb.TagNumber(6)
  $core.bool hasStopResult() => $_has(5);
  @$pb.TagNumber(6)
  void clearStopResult() => clearField(6);
  @$pb.TagNumber(6)
  BleStopResult ensureStopResult() => $_ensure(5);

  /// device discovered
  @$pb.TagNumber(7)
  BleDeviceDiscovered get deviceDiscovered => $_getN(6);
  @$pb.TagNumber(7)
  set deviceDiscovered(BleDeviceDiscovered v) { setField(7, v); }
  @$pb.TagNumber(7)
  $core.bool hasDeviceDiscovered() => $_has(6);
  @$pb.TagNumber(7)
  void clearDeviceDiscovered() => clearField(7);
  @$pb.TagNumber(7)
  BleDeviceDiscovered ensureDeviceDiscovered() => $_ensure(6);

  /// device became unavailable
  @$pb.TagNumber(8)
  BleDeviceUnavailable get deviceUnavailable => $_getN(7);
  @$pb.TagNumber(8)
  set deviceUnavailable(BleDeviceUnavailable v) { setField(8, v); }
  @$pb.TagNumber(8)
  $core.bool hasDeviceUnavailable() => $_has(7);
  @$pb.TagNumber(8)
  void clearDeviceUnavailable() => clearField(8);
  @$pb.TagNumber(8)
  BleDeviceUnavailable ensureDeviceUnavailable() => $_ensure(7);

  /// send a direct message
  @$pb.TagNumber(9)
  BleDirectSend get directSend => $_getN(8);
  @$pb.TagNumber(9)
  set directSend(BleDirectSend v) { setField(9, v); }
  @$pb.TagNumber(9)
  $core.bool hasDirectSend() => $_has(8);
  @$pb.TagNumber(9)
  void clearDirectSend() => clearField(9);
  @$pb.TagNumber(9)
  BleDirectSend ensureDirectSend() => $_ensure(8);

  /// direct message send result
  @$pb.TagNumber(10)
  BleDirectSendResult get directSendResult => $_getN(9);
  @$pb.TagNumber(10)
  set directSendResult(BleDirectSendResult v) { setField(10, v); }
  @$pb.TagNumber(10)
  $core.bool hasDirectSendResult() => $_has(9);
  @$pb.TagNumber(10)
  void clearDirectSendResult() => clearField(10);
  @$pb.TagNumber(10)
  BleDirectSendResult ensureDirectSendResult() => $_ensure(9);

  /// direct message received
  @$pb.TagNumber(11)
  BleDirectReceived get directReceived => $_getN(10);
  @$pb.TagNumber(11)
  set directReceived(BleDirectReceived v) { setField(11, v); }
  @$pb.TagNumber(11)
  $core.bool hasDirectReceived() => $_has(10);
  @$pb.TagNumber(11)
  void clearDirectReceived() => clearField(11);
  @$pb.TagNumber(11)
  BleDirectReceived ensureDirectReceived() => $_ensure(10);
}

/// device information request message
class BleInfoRequest extends $pb.GeneratedMessage {
  factory BleInfoRequest() => create();
  BleInfoRequest._() : super();
  factory BleInfoRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory BleInfoRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'BleInfoRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.sys.ble'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  BleInfoRequest clone() => BleInfoRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  BleInfoRequest copyWith(void Function(BleInfoRequest) updates) => super.copyWith((message) => updates(message as BleInfoRequest)) as BleInfoRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static BleInfoRequest create() => BleInfoRequest._();
  BleInfoRequest createEmptyInstance() => create();
  static $pb.PbList<BleInfoRequest> createRepeated() => $pb.PbList<BleInfoRequest>();
  @$core.pragma('dart2js:noInline')
  static BleInfoRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<BleInfoRequest>(create);
  static BleInfoRequest? _defaultInstance;
}

/// device information response message
class BleInfoResponse extends $pb.GeneratedMessage {
  factory BleInfoResponse({
    BleDeviceInfo? device,
  }) {
    final $result = create();
    if (device != null) {
      $result.device = device;
    }
    return $result;
  }
  BleInfoResponse._() : super();
  factory BleInfoResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory BleInfoResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'BleInfoResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.sys.ble'), createEmptyInstance: create)
    ..aOM<BleDeviceInfo>(1, _omitFieldNames ? '' : 'device', subBuilder: BleDeviceInfo.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  BleInfoResponse clone() => BleInfoResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  BleInfoResponse copyWith(void Function(BleInfoResponse) updates) => super.copyWith((message) => updates(message as BleInfoResponse)) as BleInfoResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static BleInfoResponse create() => BleInfoResponse._();
  BleInfoResponse createEmptyInstance() => create();
  static $pb.PbList<BleInfoResponse> createRepeated() => $pb.PbList<BleInfoResponse>();
  @$core.pragma('dart2js:noInline')
  static BleInfoResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<BleInfoResponse>(create);
  static BleInfoResponse? _defaultInstance;

  /// fill in a device information of the BLE device
  @$pb.TagNumber(1)
  BleDeviceInfo get device => $_getN(0);
  @$pb.TagNumber(1)
  set device(BleDeviceInfo v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasDevice() => $_has(0);
  @$pb.TagNumber(1)
  void clearDevice() => clearField(1);
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
    final $result = create();
    if (bleSupport != null) {
      $result.bleSupport = bleSupport;
    }
    if (id != null) {
      $result.id = id;
    }
    if (name != null) {
      $result.name = name;
    }
    if (bluetoothOn != null) {
      $result.bluetoothOn = bluetoothOn;
    }
    if (advExtended != null) {
      $result.advExtended = advExtended;
    }
    if (advExtendedBytes != null) {
      $result.advExtendedBytes = advExtendedBytes;
    }
    if (le2m != null) {
      $result.le2m = le2m;
    }
    if (leCoded != null) {
      $result.leCoded = leCoded;
    }
    if (leAudio != null) {
      $result.leAudio = leAudio;
    }
    if (lePeriodicAdvSupport != null) {
      $result.lePeriodicAdvSupport = lePeriodicAdvSupport;
    }
    if (leMultipleAdvSupport != null) {
      $result.leMultipleAdvSupport = leMultipleAdvSupport;
    }
    if (offloadFilterSupport != null) {
      $result.offloadFilterSupport = offloadFilterSupport;
    }
    if (offloadScanBatchingSupport != null) {
      $result.offloadScanBatchingSupport = offloadScanBatchingSupport;
    }
    return $result;
  }
  BleDeviceInfo._() : super();
  factory BleDeviceInfo.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory BleDeviceInfo.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'BleDeviceInfo', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.sys.ble'), createEmptyInstance: create)
    ..aOB(1, _omitFieldNames ? '' : 'bleSupport')
    ..aOS(2, _omitFieldNames ? '' : 'id')
    ..aOS(3, _omitFieldNames ? '' : 'name')
    ..aOB(4, _omitFieldNames ? '' : 'bluetoothOn')
    ..aOB(5, _omitFieldNames ? '' : 'advExtended')
    ..a<$core.int>(6, _omitFieldNames ? '' : 'advExtendedBytes', $pb.PbFieldType.OU3)
    ..aOB(7, _omitFieldNames ? '' : 'le2m', protoName: 'le_2m')
    ..aOB(8, _omitFieldNames ? '' : 'leCoded')
    ..aOB(9, _omitFieldNames ? '' : 'leAudio')
    ..aOB(14, _omitFieldNames ? '' : 'lePeriodicAdvSupport')
    ..aOB(15, _omitFieldNames ? '' : 'leMultipleAdvSupport')
    ..aOB(16, _omitFieldNames ? '' : 'offloadFilterSupport')
    ..aOB(17, _omitFieldNames ? '' : 'offloadScanBatchingSupport')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  BleDeviceInfo clone() => BleDeviceInfo()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  BleDeviceInfo copyWith(void Function(BleDeviceInfo) updates) => super.copyWith((message) => updates(message as BleDeviceInfo)) as BleDeviceInfo;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static BleDeviceInfo create() => BleDeviceInfo._();
  BleDeviceInfo createEmptyInstance() => create();
  static $pb.PbList<BleDeviceInfo> createRepeated() => $pb.PbList<BleDeviceInfo>();
  @$core.pragma('dart2js:noInline')
  static BleDeviceInfo getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<BleDeviceInfo>(create);
  static BleDeviceInfo? _defaultInstance;

  ///  Check if Bluetooth / Bluetooth Low Energy is supported
  ///
  ///  Android: check if a bluetooth adapter is found
  @$pb.TagNumber(1)
  $core.bool get bleSupport => $_getBF(0);
  @$pb.TagNumber(1)
  set bleSupport($core.bool v) { $_setBool(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasBleSupport() => $_has(0);
  @$pb.TagNumber(1)
  void clearBleSupport() => clearField(1);

  /// Bluetooth device address
  /// 48 bit unique Bluetooth device addr
  /// e.g. 80:86:F2:08:C7:98
  ///
  /// Android: BluetoothAdapter getAddress()
  /// https://developer.android.com/reference/kotlin/android/bluetooth/BluetoothAdapter#getAddress()
  @$pb.TagNumber(2)
  $core.String get id => $_getSZ(1);
  @$pb.TagNumber(2)
  set id($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasId() => $_has(1);
  @$pb.TagNumber(2)
  void clearId() => clearField(2);

  ///  Get Bluetooth Name
  ///  this is field is purely informative
  ///
  ///  Android: BluetoothAdapter getName()
  ///  https://developer.android.com/reference/kotlin/android/bluetooth/BluetoothAdapter#getName()
  @$pb.TagNumber(3)
  $core.String get name => $_getSZ(2);
  @$pb.TagNumber(3)
  set name($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasName() => $_has(2);
  @$pb.TagNumber(3)
  void clearName() => clearField(3);

  ///  Bluetooth is enable / powered on
  ///
  ///  Android: BluetoothAdapter isEnabled()
  ///  https://developer.android.com/reference/kotlin/android/bluetooth/BluetoothAdapter#isEnabled()
  @$pb.TagNumber(4)
  $core.bool get bluetoothOn => $_getBF(3);
  @$pb.TagNumber(4)
  set bluetoothOn($core.bool v) { $_setBool(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasBluetoothOn() => $_has(3);
  @$pb.TagNumber(4)
  void clearBluetoothOn() => clearField(4);

  ///  Is extended advertisement supported?
  ///
  ///  Android: BluetoothAdapter isLeExtendedAdvertisingSupported ()
  ///  https://developer.android.com/reference/android/bluetooth/BluetoothAdapter#isLeExtendedAdvertisingSupported()
  @$pb.TagNumber(5)
  $core.bool get advExtended => $_getBF(4);
  @$pb.TagNumber(5)
  set advExtended($core.bool v) { $_setBool(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasAdvExtended() => $_has(4);
  @$pb.TagNumber(5)
  void clearAdvExtended() => clearField(5);

  ///  what is the maximal amount of bytes sendable via advertising?
  ///
  ///  Android: BluetoothAdapter getLeMaximumAdvertisingDataLength()
  ///  https://developer.android.com/reference/android/bluetooth/BluetoothAdapter#getLeMaximumAdvertisingDataLength()
  @$pb.TagNumber(6)
  $core.int get advExtendedBytes => $_getIZ(5);
  @$pb.TagNumber(6)
  set advExtendedBytes($core.int v) { $_setUnsignedInt32(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasAdvExtendedBytes() => $_has(5);
  @$pb.TagNumber(6)
  void clearAdvExtendedBytes() => clearField(6);

  ///  Is 2M phy supported?
  ///
  ///  Android: BluetoothAdapter isLe2MPhySupported()
  ///  https://developer.android.com/reference/android/bluetooth/BluetoothAdapter#isLe2MPhySupported()
  @$pb.TagNumber(7)
  $core.bool get le2m => $_getBF(6);
  @$pb.TagNumber(7)
  set le2m($core.bool v) { $_setBool(6, v); }
  @$pb.TagNumber(7)
  $core.bool hasLe2m() => $_has(6);
  @$pb.TagNumber(7)
  void clearLe2m() => clearField(7);

  ///  is extended advertising supported in coded
  ///  mode? (For long distance connections)
  ///
  ///  Android: BluetoothAdapter isLeCodedPhySupported()
  ///  https://developer.android.com/reference/android/bluetooth/BluetoothAdapter#isLeCodedPhySupported()
  @$pb.TagNumber(8)
  $core.bool get leCoded => $_getBF(7);
  @$pb.TagNumber(8)
  set leCoded($core.bool v) { $_setBool(7, v); }
  @$pb.TagNumber(8)
  $core.bool hasLeCoded() => $_has(7);
  @$pb.TagNumber(8)
  void clearLeCoded() => clearField(8);

  ///  is LE audio supported?
  ///
  ///  This is the most recent BLE feature, supported on:
  ///
  ///  * android 12 and above
  ///  * linux ?
  ///  * ios ?
  ///  * macos ?
  ///  * windows ?
  ///
  ///  Android: AndroidAdapter isLeAudioSupported()
  ///  https://developer.android.com/reference/kotlin/android/bluetooth/BluetoothAdapter#isLeAudioSupported()
  @$pb.TagNumber(9)
  $core.bool get leAudio => $_getBF(8);
  @$pb.TagNumber(9)
  set leAudio($core.bool v) { $_setBool(8, v); }
  @$pb.TagNumber(9)
  $core.bool hasLeAudio() => $_has(8);
  @$pb.TagNumber(9)
  void clearLeAudio() => clearField(9);

  ///  is periodic advertisment supported?
  ///
  ///  Android: BluetoothAdapter isLePeriodicAdvertisingSupported()
  ///  https://developer.android.com/reference/android/bluetooth/BluetoothAdapter#isLePeriodicAdvertisingSupported()
  @$pb.TagNumber(14)
  $core.bool get lePeriodicAdvSupport => $_getBF(9);
  @$pb.TagNumber(14)
  set lePeriodicAdvSupport($core.bool v) { $_setBool(9, v); }
  @$pb.TagNumber(14)
  $core.bool hasLePeriodicAdvSupport() => $_has(9);
  @$pb.TagNumber(14)
  void clearLePeriodicAdvSupport() => clearField(14);

  ///  Is multi advertisement supported?
  ///
  ///  When multi advertisement is supported one can have different
  ///  advertisement types parallely. Each advertisement has a
  ///  different device address.
  ///  For scanning devices it looks, as if multiple devices devices
  ///  would advertise themselves.
  ///  This is helpful to support several incompatible advertisement
  ///  modes at the same time.
  ///
  ///  Android: BluetoothAdapter isMultipleAdvertisementSupported()
  ///  https://developer.android.com/reference/android/bluetooth/BluetoothAdapter#isMultipleAdvertisementSupported()
  @$pb.TagNumber(15)
  $core.bool get leMultipleAdvSupport => $_getBF(10);
  @$pb.TagNumber(15)
  set leMultipleAdvSupport($core.bool v) { $_setBool(10, v); }
  @$pb.TagNumber(15)
  $core.bool hasLeMultipleAdvSupport() => $_has(10);
  @$pb.TagNumber(15)
  void clearLeMultipleAdvSupport() => clearField(15);

  ///  Android Specific: is Offloaded Filtering Supported?
  ///
  ///  Android: BluetoothAdapter isOffloadedFilteringSupported()
  @$pb.TagNumber(16)
  $core.bool get offloadFilterSupport => $_getBF(11);
  @$pb.TagNumber(16)
  set offloadFilterSupport($core.bool v) { $_setBool(11, v); }
  @$pb.TagNumber(16)
  $core.bool hasOffloadFilterSupport() => $_has(11);
  @$pb.TagNumber(16)
  void clearOffloadFilterSupport() => clearField(16);

  ///  Android Specific: is Offloaded Scan Batching Supported?
  ///
  ///  Android: BluetoothAdapter isOffloadedScanBatchingSupported()
  ///  https://developer.android.com/reference/android/bluetooth/BluetoothAdapter#isOffloadedScanBatchingSupported()
  @$pb.TagNumber(17)
  $core.bool get offloadScanBatchingSupport => $_getBF(12);
  @$pb.TagNumber(17)
  set offloadScanBatchingSupport($core.bool v) { $_setBool(12, v); }
  @$pb.TagNumber(17)
  $core.bool hasOffloadScanBatchingSupport() => $_has(12);
  @$pb.TagNumber(17)
  void clearOffloadScanBatchingSupport() => clearField(17);
}

///  Start Device
///
///  the module will try to start the device, power it up,
///  get all rights, configure it for qaul, and
///  send & receive advertising messages
class BleStartRequest extends $pb.GeneratedMessage {
  factory BleStartRequest({
    $core.List<$core.int>? qaulId,
    BlePowerSetting? powerSetting,
  }) {
    final $result = create();
    if (qaulId != null) {
      $result.qaulId = qaulId;
    }
    if (powerSetting != null) {
      $result.powerSetting = powerSetting;
    }
    return $result;
  }
  BleStartRequest._() : super();
  factory BleStartRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory BleStartRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'BleStartRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.sys.ble'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'qaulId', $pb.PbFieldType.OY)
    ..e<BlePowerSetting>(2, _omitFieldNames ? '' : 'powerSetting', $pb.PbFieldType.OE, defaultOrMaker: BlePowerSetting.low_power, valueOf: BlePowerSetting.valueOf, enumValues: BlePowerSetting.values)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  BleStartRequest clone() => BleStartRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  BleStartRequest copyWith(void Function(BleStartRequest) updates) => super.copyWith((message) => updates(message as BleStartRequest)) as BleStartRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static BleStartRequest create() => BleStartRequest._();
  BleStartRequest createEmptyInstance() => create();
  static $pb.PbList<BleStartRequest> createRepeated() => $pb.PbList<BleStartRequest>();
  @$core.pragma('dart2js:noInline')
  static BleStartRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<BleStartRequest>(create);
  static BleStartRequest? _defaultInstance;

  ///  qaul ID
  ///
  ///  The small 16 byte qaul id
  ///  to be used to identify this node
  @$pb.TagNumber(1)
  $core.List<$core.int> get qaulId => $_getN(0);
  @$pb.TagNumber(1)
  set qaulId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasQaulId() => $_has(0);
  @$pb.TagNumber(1)
  void clearQaulId() => clearField(1);

  /// power settings
  @$pb.TagNumber(2)
  BlePowerSetting get powerSetting => $_getN(1);
  @$pb.TagNumber(2)
  set powerSetting(BlePowerSetting v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasPowerSetting() => $_has(1);
  @$pb.TagNumber(2)
  void clearPowerSetting() => clearField(2);
}

///  Start device result message
///
///  Feedback from the
class BleStartResult extends $pb.GeneratedMessage {
  factory BleStartResult({
    $core.bool? success,
    BleError? errorReason,
    $core.String? errorMessage,
  }) {
    final $result = create();
    if (success != null) {
      $result.success = success;
    }
    if (errorReason != null) {
      $result.errorReason = errorReason;
    }
    if (errorMessage != null) {
      $result.errorMessage = errorMessage;
    }
    return $result;
  }
  BleStartResult._() : super();
  factory BleStartResult.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory BleStartResult.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'BleStartResult', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.sys.ble'), createEmptyInstance: create)
    ..aOB(1, _omitFieldNames ? '' : 'success')
    ..e<BleError>(2, _omitFieldNames ? '' : 'errorReason', $pb.PbFieldType.OE, defaultOrMaker: BleError.UNKNOWN_ERROR, valueOf: BleError.valueOf, enumValues: BleError.values)
    ..aOS(3, _omitFieldNames ? '' : 'errorMessage')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  BleStartResult clone() => BleStartResult()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  BleStartResult copyWith(void Function(BleStartResult) updates) => super.copyWith((message) => updates(message as BleStartResult)) as BleStartResult;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static BleStartResult create() => BleStartResult._();
  BleStartResult createEmptyInstance() => create();
  static $pb.PbList<BleStartResult> createRepeated() => $pb.PbList<BleStartResult>();
  @$core.pragma('dart2js:noInline')
  static BleStartResult getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<BleStartResult>(create);
  static BleStartResult? _defaultInstance;

  /// whether the device was successfully started
  @$pb.TagNumber(1)
  $core.bool get success => $_getBF(0);
  @$pb.TagNumber(1)
  set success($core.bool v) { $_setBool(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasSuccess() => $_has(0);
  @$pb.TagNumber(1)
  void clearSuccess() => clearField(1);

  /// error reason
  @$pb.TagNumber(2)
  BleError get errorReason => $_getN(1);
  @$pb.TagNumber(2)
  set errorReason(BleError v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasErrorReason() => $_has(1);
  @$pb.TagNumber(2)
  void clearErrorReason() => clearField(2);

  /// error message
  @$pb.TagNumber(3)
  $core.String get errorMessage => $_getSZ(2);
  @$pb.TagNumber(3)
  set errorMessage($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasErrorMessage() => $_has(2);
  @$pb.TagNumber(3)
  void clearErrorMessage() => clearField(3);
}

/// Stop Bluetooth Device
class BleStopRequest extends $pb.GeneratedMessage {
  factory BleStopRequest() => create();
  BleStopRequest._() : super();
  factory BleStopRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory BleStopRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'BleStopRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.sys.ble'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  BleStopRequest clone() => BleStopRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  BleStopRequest copyWith(void Function(BleStopRequest) updates) => super.copyWith((message) => updates(message as BleStopRequest)) as BleStopRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static BleStopRequest create() => BleStopRequest._();
  BleStopRequest createEmptyInstance() => create();
  static $pb.PbList<BleStopRequest> createRepeated() => $pb.PbList<BleStopRequest>();
  @$core.pragma('dart2js:noInline')
  static BleStopRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<BleStopRequest>(create);
  static BleStopRequest? _defaultInstance;
}

///  Stop Result
///
///  Feedback of the stop request
class BleStopResult extends $pb.GeneratedMessage {
  factory BleStopResult({
    $core.bool? success,
    BleError? errorReason,
    $core.String? errorMessage,
  }) {
    final $result = create();
    if (success != null) {
      $result.success = success;
    }
    if (errorReason != null) {
      $result.errorReason = errorReason;
    }
    if (errorMessage != null) {
      $result.errorMessage = errorMessage;
    }
    return $result;
  }
  BleStopResult._() : super();
  factory BleStopResult.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory BleStopResult.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'BleStopResult', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.sys.ble'), createEmptyInstance: create)
    ..aOB(1, _omitFieldNames ? '' : 'success')
    ..e<BleError>(2, _omitFieldNames ? '' : 'errorReason', $pb.PbFieldType.OE, defaultOrMaker: BleError.UNKNOWN_ERROR, valueOf: BleError.valueOf, enumValues: BleError.values)
    ..aOS(3, _omitFieldNames ? '' : 'errorMessage')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  BleStopResult clone() => BleStopResult()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  BleStopResult copyWith(void Function(BleStopResult) updates) => super.copyWith((message) => updates(message as BleStopResult)) as BleStopResult;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static BleStopResult create() => BleStopResult._();
  BleStopResult createEmptyInstance() => create();
  static $pb.PbList<BleStopResult> createRepeated() => $pb.PbList<BleStopResult>();
  @$core.pragma('dart2js:noInline')
  static BleStopResult getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<BleStopResult>(create);
  static BleStopResult? _defaultInstance;

  /// whether the device was successfully stopped
  @$pb.TagNumber(1)
  $core.bool get success => $_getBF(0);
  @$pb.TagNumber(1)
  set success($core.bool v) { $_setBool(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasSuccess() => $_has(0);
  @$pb.TagNumber(1)
  void clearSuccess() => clearField(1);

  /// error reason
  @$pb.TagNumber(2)
  BleError get errorReason => $_getN(1);
  @$pb.TagNumber(2)
  set errorReason(BleError v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasErrorReason() => $_has(1);
  @$pb.TagNumber(2)
  void clearErrorReason() => clearField(2);

  /// error message
  @$pb.TagNumber(3)
  $core.String get errorMessage => $_getSZ(2);
  @$pb.TagNumber(3)
  set errorMessage($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasErrorMessage() => $_has(2);
  @$pb.TagNumber(3)
  void clearErrorMessage() => clearField(3);
}

///  Device Discovered
///
///  A new device has been discovered.
class BleDeviceDiscovered extends $pb.GeneratedMessage {
  factory BleDeviceDiscovered({
    $core.List<$core.int>? qaulId,
    $core.int? rssi,
  }) {
    final $result = create();
    if (qaulId != null) {
      $result.qaulId = qaulId;
    }
    if (rssi != null) {
      $result.rssi = rssi;
    }
    return $result;
  }
  BleDeviceDiscovered._() : super();
  factory BleDeviceDiscovered.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory BleDeviceDiscovered.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'BleDeviceDiscovered', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.sys.ble'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'qaulId', $pb.PbFieldType.OY)
    ..a<$core.int>(2, _omitFieldNames ? '' : 'rssi', $pb.PbFieldType.O3)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  BleDeviceDiscovered clone() => BleDeviceDiscovered()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  BleDeviceDiscovered copyWith(void Function(BleDeviceDiscovered) updates) => super.copyWith((message) => updates(message as BleDeviceDiscovered)) as BleDeviceDiscovered;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static BleDeviceDiscovered create() => BleDeviceDiscovered._();
  BleDeviceDiscovered createEmptyInstance() => create();
  static $pb.PbList<BleDeviceDiscovered> createRepeated() => $pb.PbList<BleDeviceDiscovered>();
  @$core.pragma('dart2js:noInline')
  static BleDeviceDiscovered getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<BleDeviceDiscovered>(create);
  static BleDeviceDiscovered? _defaultInstance;

  /// qaul id of the device
  @$pb.TagNumber(1)
  $core.List<$core.int> get qaulId => $_getN(0);
  @$pb.TagNumber(1)
  set qaulId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasQaulId() => $_has(0);
  @$pb.TagNumber(1)
  void clearQaulId() => clearField(1);

  /// the received signal strength of this device
  @$pb.TagNumber(2)
  $core.int get rssi => $_getIZ(1);
  @$pb.TagNumber(2)
  set rssi($core.int v) { $_setSignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasRssi() => $_has(1);
  @$pb.TagNumber(2)
  void clearRssi() => clearField(2);
}

///  Device Unavailable
///
///  A formerly discovered device has become
///  unavailable. No messages can be sent to it.
class BleDeviceUnavailable extends $pb.GeneratedMessage {
  factory BleDeviceUnavailable({
    $core.List<$core.int>? qaulId,
  }) {
    final $result = create();
    if (qaulId != null) {
      $result.qaulId = qaulId;
    }
    return $result;
  }
  BleDeviceUnavailable._() : super();
  factory BleDeviceUnavailable.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory BleDeviceUnavailable.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'BleDeviceUnavailable', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.sys.ble'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'qaulId', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  BleDeviceUnavailable clone() => BleDeviceUnavailable()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  BleDeviceUnavailable copyWith(void Function(BleDeviceUnavailable) updates) => super.copyWith((message) => updates(message as BleDeviceUnavailable)) as BleDeviceUnavailable;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static BleDeviceUnavailable create() => BleDeviceUnavailable._();
  BleDeviceUnavailable createEmptyInstance() => create();
  static $pb.PbList<BleDeviceUnavailable> createRepeated() => $pb.PbList<BleDeviceUnavailable>();
  @$core.pragma('dart2js:noInline')
  static BleDeviceUnavailable getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<BleDeviceUnavailable>(create);
  static BleDeviceUnavailable? _defaultInstance;

  /// qaul id of the device that
  /// became unavailable
  @$pb.TagNumber(1)
  $core.List<$core.int> get qaulId => $_getN(0);
  @$pb.TagNumber(1)
  set qaulId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasQaulId() => $_has(0);
  @$pb.TagNumber(1)
  void clearQaulId() => clearField(1);
}

/// send a direct message
class BleDirectSend extends $pb.GeneratedMessage {
  factory BleDirectSend({
    $core.List<$core.int>? messageId,
    $core.List<$core.int>? receiverId,
    $core.List<$core.int>? senderId,
    $core.List<$core.int>? data,
  }) {
    final $result = create();
    if (messageId != null) {
      $result.messageId = messageId;
    }
    if (receiverId != null) {
      $result.receiverId = receiverId;
    }
    if (senderId != null) {
      $result.senderId = senderId;
    }
    if (data != null) {
      $result.data = data;
    }
    return $result;
  }
  BleDirectSend._() : super();
  factory BleDirectSend.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory BleDirectSend.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'BleDirectSend', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.sys.ble'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'messageId', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(2, _omitFieldNames ? '' : 'receiverId', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(3, _omitFieldNames ? '' : 'senderId', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(4, _omitFieldNames ? '' : 'data', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  BleDirectSend clone() => BleDirectSend()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  BleDirectSend copyWith(void Function(BleDirectSend) updates) => super.copyWith((message) => updates(message as BleDirectSend)) as BleDirectSend;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static BleDirectSend create() => BleDirectSend._();
  BleDirectSend createEmptyInstance() => create();
  static $pb.PbList<BleDirectSend> createRepeated() => $pb.PbList<BleDirectSend>();
  @$core.pragma('dart2js:noInline')
  static BleDirectSend getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<BleDirectSend>(create);
  static BleDirectSend? _defaultInstance;

  /// message id (as a reference for the result message)
  @$pb.TagNumber(1)
  $core.List<$core.int> get messageId => $_getN(0);
  @$pb.TagNumber(1)
  set messageId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasMessageId() => $_has(0);
  @$pb.TagNumber(1)
  void clearMessageId() => clearField(1);

  /// qaul id of the device to send it to
  @$pb.TagNumber(2)
  $core.List<$core.int> get receiverId => $_getN(1);
  @$pb.TagNumber(2)
  set receiverId($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasReceiverId() => $_has(1);
  @$pb.TagNumber(2)
  void clearReceiverId() => clearField(2);

  /// qaul id of the sending device
  @$pb.TagNumber(3)
  $core.List<$core.int> get senderId => $_getN(2);
  @$pb.TagNumber(3)
  set senderId($core.List<$core.int> v) { $_setBytes(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasSenderId() => $_has(2);
  @$pb.TagNumber(3)
  void clearSenderId() => clearField(3);

  /// data to be sent
  @$pb.TagNumber(4)
  $core.List<$core.int> get data => $_getN(3);
  @$pb.TagNumber(4)
  set data($core.List<$core.int> v) { $_setBytes(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasData() => $_has(3);
  @$pb.TagNumber(4)
  void clearData() => clearField(4);
}

/// result after sending the direct message
class BleDirectSendResult extends $pb.GeneratedMessage {
  factory BleDirectSendResult({
    $core.List<$core.int>? id,
    $core.bool? success,
    $core.String? errorMessage,
  }) {
    final $result = create();
    if (id != null) {
      $result.id = id;
    }
    if (success != null) {
      $result.success = success;
    }
    if (errorMessage != null) {
      $result.errorMessage = errorMessage;
    }
    return $result;
  }
  BleDirectSendResult._() : super();
  factory BleDirectSendResult.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory BleDirectSendResult.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'BleDirectSendResult', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.sys.ble'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'id', $pb.PbFieldType.OY)
    ..aOB(2, _omitFieldNames ? '' : 'success')
    ..aOS(3, _omitFieldNames ? '' : 'errorMessage')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  BleDirectSendResult clone() => BleDirectSendResult()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  BleDirectSendResult copyWith(void Function(BleDirectSendResult) updates) => super.copyWith((message) => updates(message as BleDirectSendResult)) as BleDirectSendResult;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static BleDirectSendResult create() => BleDirectSendResult._();
  BleDirectSendResult createEmptyInstance() => create();
  static $pb.PbList<BleDirectSendResult> createRepeated() => $pb.PbList<BleDirectSendResult>();
  @$core.pragma('dart2js:noInline')
  static BleDirectSendResult getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<BleDirectSendResult>(create);
  static BleDirectSendResult? _defaultInstance;

  /// message id
  @$pb.TagNumber(1)
  $core.List<$core.int> get id => $_getN(0);
  @$pb.TagNumber(1)
  set id($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => clearField(1);

  /// result after sending the message
  @$pb.TagNumber(2)
  $core.bool get success => $_getBF(1);
  @$pb.TagNumber(2)
  set success($core.bool v) { $_setBool(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasSuccess() => $_has(1);
  @$pb.TagNumber(2)
  void clearSuccess() => clearField(2);

  /// error messages
  @$pb.TagNumber(3)
  $core.String get errorMessage => $_getSZ(2);
  @$pb.TagNumber(3)
  set errorMessage($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasErrorMessage() => $_has(2);
  @$pb.TagNumber(3)
  void clearErrorMessage() => clearField(3);
}

/// direct message received message
class BleDirectReceived extends $pb.GeneratedMessage {
  factory BleDirectReceived({
    $core.List<$core.int>? from,
    $core.List<$core.int>? data,
  }) {
    final $result = create();
    if (from != null) {
      $result.from = from;
    }
    if (data != null) {
      $result.data = data;
    }
    return $result;
  }
  BleDirectReceived._() : super();
  factory BleDirectReceived.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory BleDirectReceived.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'BleDirectReceived', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.sys.ble'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'from', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(4, _omitFieldNames ? '' : 'data', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  BleDirectReceived clone() => BleDirectReceived()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  BleDirectReceived copyWith(void Function(BleDirectReceived) updates) => super.copyWith((message) => updates(message as BleDirectReceived)) as BleDirectReceived;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static BleDirectReceived create() => BleDirectReceived._();
  BleDirectReceived createEmptyInstance() => create();
  static $pb.PbList<BleDirectReceived> createRepeated() => $pb.PbList<BleDirectReceived>();
  @$core.pragma('dart2js:noInline')
  static BleDirectReceived getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<BleDirectReceived>(create);
  static BleDirectReceived? _defaultInstance;

  /// qaul id of the sending device
  @$pb.TagNumber(1)
  $core.List<$core.int> get from => $_getN(0);
  @$pb.TagNumber(1)
  set from($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasFrom() => $_has(0);
  @$pb.TagNumber(1)
  void clearFrom() => clearField(1);

  /// the data received
  @$pb.TagNumber(4)
  $core.List<$core.int> get data => $_getN(1);
  @$pb.TagNumber(4)
  set data($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(4)
  $core.bool hasData() => $_has(1);
  @$pb.TagNumber(4)
  void clearData() => clearField(4);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
