///
//  Generated code. Do not modify.
//  source: services/filesharing/filesharing_net.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

enum FileSharingContainer_Message {
  fileInfo, 
  fileData, 
  notSet
}

class FileSharingContainer extends $pb.GeneratedMessage {
  static const $core.Map<$core.int, FileSharingContainer_Message> _FileSharingContainer_MessageByTag = {
    1 : FileSharingContainer_Message.fileInfo,
    2 : FileSharingContainer_Message.fileData,
    0 : FileSharingContainer_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'FileSharingContainer', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.net.filesharing'), createEmptyInstance: create)
    ..oo(0, [1, 2])
    ..aOM<FileSharingInfo>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fileInfo', subBuilder: FileSharingInfo.create)
    ..aOM<FileSharingData>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fileData', subBuilder: FileSharingData.create)
    ..hasRequiredFields = false
  ;

  FileSharingContainer._() : super();
  factory FileSharingContainer({
    FileSharingInfo? fileInfo,
    FileSharingData? fileData,
  }) {
    final _result = create();
    if (fileInfo != null) {
      _result.fileInfo = fileInfo;
    }
    if (fileData != null) {
      _result.fileData = fileData;
    }
    return _result;
  }
  factory FileSharingContainer.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FileSharingContainer.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  FileSharingContainer clone() => FileSharingContainer()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  FileSharingContainer copyWith(void Function(FileSharingContainer) updates) => super.copyWith((message) => updates(message as FileSharingContainer)) as FileSharingContainer; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static FileSharingContainer create() => FileSharingContainer._();
  FileSharingContainer createEmptyInstance() => create();
  static $pb.PbList<FileSharingContainer> createRepeated() => $pb.PbList<FileSharingContainer>();
  @$core.pragma('dart2js:noInline')
  static FileSharingContainer getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FileSharingContainer>(create);
  static FileSharingContainer? _defaultInstance;

  FileSharingContainer_Message whichMessage() => _FileSharingContainer_MessageByTag[$_whichOneof(0)]!;
  void clearMessage() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  FileSharingInfo get fileInfo => $_getN(0);
  @$pb.TagNumber(1)
  set fileInfo(FileSharingInfo v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasFileInfo() => $_has(0);
  @$pb.TagNumber(1)
  void clearFileInfo() => clearField(1);
  @$pb.TagNumber(1)
  FileSharingInfo ensureFileInfo() => $_ensure(0);

  @$pb.TagNumber(2)
  FileSharingData get fileData => $_getN(1);
  @$pb.TagNumber(2)
  set fileData(FileSharingData v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasFileData() => $_has(1);
  @$pb.TagNumber(2)
  void clearFileData() => clearField(2);
  @$pb.TagNumber(2)
  FileSharingData ensureFileData() => $_ensure(1);
}

class FileSharingInfo extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'FileSharingInfo', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.net.filesharing'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fileId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fileName')
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fileExtension')
    ..a<$core.int>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fileSize', $pb.PbFieldType.OU3)
    ..aOS(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fileDescr')
    ..a<$core.int>(6, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'startIndex', $pb.PbFieldType.OU3)
    ..a<$core.int>(7, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'messageCount', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  FileSharingInfo._() : super();
  factory FileSharingInfo({
    $fixnum.Int64? fileId,
    $core.String? fileName,
    $core.String? fileExtension,
    $core.int? fileSize,
    $core.String? fileDescr,
    $core.int? startIndex,
    $core.int? messageCount,
  }) {
    final _result = create();
    if (fileId != null) {
      _result.fileId = fileId;
    }
    if (fileName != null) {
      _result.fileName = fileName;
    }
    if (fileExtension != null) {
      _result.fileExtension = fileExtension;
    }
    if (fileSize != null) {
      _result.fileSize = fileSize;
    }
    if (fileDescr != null) {
      _result.fileDescr = fileDescr;
    }
    if (startIndex != null) {
      _result.startIndex = startIndex;
    }
    if (messageCount != null) {
      _result.messageCount = messageCount;
    }
    return _result;
  }
  factory FileSharingInfo.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FileSharingInfo.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  FileSharingInfo clone() => FileSharingInfo()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  FileSharingInfo copyWith(void Function(FileSharingInfo) updates) => super.copyWith((message) => updates(message as FileSharingInfo)) as FileSharingInfo; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static FileSharingInfo create() => FileSharingInfo._();
  FileSharingInfo createEmptyInstance() => create();
  static $pb.PbList<FileSharingInfo> createRepeated() => $pb.PbList<FileSharingInfo>();
  @$core.pragma('dart2js:noInline')
  static FileSharingInfo getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FileSharingInfo>(create);
  static FileSharingInfo? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get fileId => $_getI64(0);
  @$pb.TagNumber(1)
  set fileId($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasFileId() => $_has(0);
  @$pb.TagNumber(1)
  void clearFileId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get fileName => $_getSZ(1);
  @$pb.TagNumber(2)
  set fileName($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasFileName() => $_has(1);
  @$pb.TagNumber(2)
  void clearFileName() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get fileExtension => $_getSZ(2);
  @$pb.TagNumber(3)
  set fileExtension($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasFileExtension() => $_has(2);
  @$pb.TagNumber(3)
  void clearFileExtension() => clearField(3);

  @$pb.TagNumber(4)
  $core.int get fileSize => $_getIZ(3);
  @$pb.TagNumber(4)
  set fileSize($core.int v) { $_setUnsignedInt32(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasFileSize() => $_has(3);
  @$pb.TagNumber(4)
  void clearFileSize() => clearField(4);

  @$pb.TagNumber(5)
  $core.String get fileDescr => $_getSZ(4);
  @$pb.TagNumber(5)
  set fileDescr($core.String v) { $_setString(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasFileDescr() => $_has(4);
  @$pb.TagNumber(5)
  void clearFileDescr() => clearField(5);

  @$pb.TagNumber(6)
  $core.int get startIndex => $_getIZ(5);
  @$pb.TagNumber(6)
  set startIndex($core.int v) { $_setUnsignedInt32(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasStartIndex() => $_has(5);
  @$pb.TagNumber(6)
  void clearStartIndex() => clearField(6);

  @$pb.TagNumber(7)
  $core.int get messageCount => $_getIZ(6);
  @$pb.TagNumber(7)
  set messageCount($core.int v) { $_setUnsignedInt32(6, v); }
  @$pb.TagNumber(7)
  $core.bool hasMessageCount() => $_has(6);
  @$pb.TagNumber(7)
  void clearMessageCount() => clearField(7);
}

class FileSharingData extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'FileSharingData', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.net.filesharing'), createEmptyInstance: create)
    ..a<$core.int>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'startIndex', $pb.PbFieldType.OU3)
    ..a<$core.int>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'messageCount', $pb.PbFieldType.OU3)
    ..a<$core.List<$core.int>>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'data', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  FileSharingData._() : super();
  factory FileSharingData({
    $core.int? startIndex,
    $core.int? messageCount,
    $core.List<$core.int>? data,
  }) {
    final _result = create();
    if (startIndex != null) {
      _result.startIndex = startIndex;
    }
    if (messageCount != null) {
      _result.messageCount = messageCount;
    }
    if (data != null) {
      _result.data = data;
    }
    return _result;
  }
  factory FileSharingData.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FileSharingData.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  FileSharingData clone() => FileSharingData()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  FileSharingData copyWith(void Function(FileSharingData) updates) => super.copyWith((message) => updates(message as FileSharingData)) as FileSharingData; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static FileSharingData create() => FileSharingData._();
  FileSharingData createEmptyInstance() => create();
  static $pb.PbList<FileSharingData> createRepeated() => $pb.PbList<FileSharingData>();
  @$core.pragma('dart2js:noInline')
  static FileSharingData getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FileSharingData>(create);
  static FileSharingData? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get startIndex => $_getIZ(0);
  @$pb.TagNumber(1)
  set startIndex($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasStartIndex() => $_has(0);
  @$pb.TagNumber(1)
  void clearStartIndex() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get messageCount => $_getIZ(1);
  @$pb.TagNumber(2)
  set messageCount($core.int v) { $_setUnsignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasMessageCount() => $_has(1);
  @$pb.TagNumber(2)
  void clearMessageCount() => clearField(2);

  @$pb.TagNumber(3)
  $core.List<$core.int> get data => $_getN(2);
  @$pb.TagNumber(3)
  set data($core.List<$core.int> v) { $_setBytes(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasData() => $_has(2);
  @$pb.TagNumber(3)
  void clearData() => clearField(3);
}

