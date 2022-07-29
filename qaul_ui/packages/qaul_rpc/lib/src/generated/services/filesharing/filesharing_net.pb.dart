///
//  Generated code. Do not modify.
//  source: services/filesharing/filesharing_net.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

enum FileSharingContainer_Message {
  fileInfo, 
  fileData, 
  confirmation, 
  confirmationInfo, 
  completed, 
  canceled, 
  notSet
}

class FileSharingContainer extends $pb.GeneratedMessage {
  static const $core.Map<$core.int, FileSharingContainer_Message> _FileSharingContainer_MessageByTag = {
    1 : FileSharingContainer_Message.fileInfo,
    2 : FileSharingContainer_Message.fileData,
    3 : FileSharingContainer_Message.confirmation,
    4 : FileSharingContainer_Message.confirmationInfo,
    5 : FileSharingContainer_Message.completed,
    6 : FileSharingContainer_Message.canceled,
    0 : FileSharingContainer_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'FileSharingContainer', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.net.filesharing'), createEmptyInstance: create)
    ..oo(0, [1, 2, 3, 4, 5, 6])
    ..aOM<FileSharingInfo>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fileInfo', subBuilder: FileSharingInfo.create)
    ..aOM<FileSharingData>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fileData', subBuilder: FileSharingData.create)
    ..aOM<FileSharingConfirmation>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'confirmation', subBuilder: FileSharingConfirmation.create)
    ..aOM<FileSharingConfirmationFileInfo>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'confirmationInfo', subBuilder: FileSharingConfirmationFileInfo.create)
    ..aOM<FileSharingCompleted>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'completed', subBuilder: FileSharingCompleted.create)
    ..aOM<FileSharingCanceled>(6, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'canceled', subBuilder: FileSharingCanceled.create)
    ..hasRequiredFields = false
  ;

  FileSharingContainer._() : super();
  factory FileSharingContainer({
    FileSharingInfo? fileInfo,
    FileSharingData? fileData,
    FileSharingConfirmation? confirmation,
    FileSharingConfirmationFileInfo? confirmationInfo,
    FileSharingCompleted? completed,
    FileSharingCanceled? canceled,
  }) {
    final _result = create();
    if (fileInfo != null) {
      _result.fileInfo = fileInfo;
    }
    if (fileData != null) {
      _result.fileData = fileData;
    }
    if (confirmation != null) {
      _result.confirmation = confirmation;
    }
    if (confirmationInfo != null) {
      _result.confirmationInfo = confirmationInfo;
    }
    if (completed != null) {
      _result.completed = completed;
    }
    if (canceled != null) {
      _result.canceled = canceled;
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

  @$pb.TagNumber(3)
  FileSharingConfirmation get confirmation => $_getN(2);
  @$pb.TagNumber(3)
  set confirmation(FileSharingConfirmation v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasConfirmation() => $_has(2);
  @$pb.TagNumber(3)
  void clearConfirmation() => clearField(3);
  @$pb.TagNumber(3)
  FileSharingConfirmation ensureConfirmation() => $_ensure(2);

  @$pb.TagNumber(4)
  FileSharingConfirmationFileInfo get confirmationInfo => $_getN(3);
  @$pb.TagNumber(4)
  set confirmationInfo(FileSharingConfirmationFileInfo v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasConfirmationInfo() => $_has(3);
  @$pb.TagNumber(4)
  void clearConfirmationInfo() => clearField(4);
  @$pb.TagNumber(4)
  FileSharingConfirmationFileInfo ensureConfirmationInfo() => $_ensure(3);

  @$pb.TagNumber(5)
  FileSharingCompleted get completed => $_getN(4);
  @$pb.TagNumber(5)
  set completed(FileSharingCompleted v) { setField(5, v); }
  @$pb.TagNumber(5)
  $core.bool hasCompleted() => $_has(4);
  @$pb.TagNumber(5)
  void clearCompleted() => clearField(5);
  @$pb.TagNumber(5)
  FileSharingCompleted ensureCompleted() => $_ensure(4);

  @$pb.TagNumber(6)
  FileSharingCanceled get canceled => $_getN(5);
  @$pb.TagNumber(6)
  set canceled(FileSharingCanceled v) { setField(6, v); }
  @$pb.TagNumber(6)
  $core.bool hasCanceled() => $_has(5);
  @$pb.TagNumber(6)
  void clearCanceled() => clearField(6);
  @$pb.TagNumber(6)
  FileSharingCanceled ensureCanceled() => $_ensure(5);
}

class FileSharingInfo extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'FileSharingInfo', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.net.filesharing'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fileName')
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fileExtension')
    ..a<$core.int>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fileSize', $pb.PbFieldType.OU3)
    ..aOS(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fileDescr')
    ..a<$core.int>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'sizePerPackage', $pb.PbFieldType.OU3)
    ..a<$fixnum.Int64>(6, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fileId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  FileSharingInfo._() : super();
  factory FileSharingInfo({
    $core.String? fileName,
    $core.String? fileExtension,
    $core.int? fileSize,
    $core.String? fileDescr,
    $core.int? sizePerPackage,
    $fixnum.Int64? fileId,
  }) {
    final _result = create();
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
    if (sizePerPackage != null) {
      _result.sizePerPackage = sizePerPackage;
    }
    if (fileId != null) {
      _result.fileId = fileId;
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
  $core.String get fileName => $_getSZ(0);
  @$pb.TagNumber(1)
  set fileName($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasFileName() => $_has(0);
  @$pb.TagNumber(1)
  void clearFileName() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get fileExtension => $_getSZ(1);
  @$pb.TagNumber(2)
  set fileExtension($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasFileExtension() => $_has(1);
  @$pb.TagNumber(2)
  void clearFileExtension() => clearField(2);

  @$pb.TagNumber(3)
  $core.int get fileSize => $_getIZ(2);
  @$pb.TagNumber(3)
  set fileSize($core.int v) { $_setUnsignedInt32(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasFileSize() => $_has(2);
  @$pb.TagNumber(3)
  void clearFileSize() => clearField(3);

  @$pb.TagNumber(4)
  $core.String get fileDescr => $_getSZ(3);
  @$pb.TagNumber(4)
  set fileDescr($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasFileDescr() => $_has(3);
  @$pb.TagNumber(4)
  void clearFileDescr() => clearField(4);

  @$pb.TagNumber(5)
  $core.int get sizePerPackage => $_getIZ(4);
  @$pb.TagNumber(5)
  set sizePerPackage($core.int v) { $_setUnsignedInt32(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasSizePerPackage() => $_has(4);
  @$pb.TagNumber(5)
  void clearSizePerPackage() => clearField(5);

  @$pb.TagNumber(6)
  $fixnum.Int64 get fileId => $_getI64(5);
  @$pb.TagNumber(6)
  set fileId($fixnum.Int64 v) { $_setInt64(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasFileId() => $_has(5);
  @$pb.TagNumber(6)
  void clearFileId() => clearField(6);
}

class FileSharingData extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'FileSharingData', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.net.filesharing'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fileId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.int>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'sequence', $pb.PbFieldType.OU3)
    ..a<$core.int>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fileSize', $pb.PbFieldType.OU3)
    ..a<$core.int>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'sizePerPackage', $pb.PbFieldType.OU3)
    ..a<$core.List<$core.int>>(6, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'data', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  FileSharingData._() : super();
  factory FileSharingData({
    $fixnum.Int64? fileId,
    $core.int? sequence,
    $core.int? fileSize,
    $core.int? sizePerPackage,
    $core.List<$core.int>? data,
  }) {
    final _result = create();
    if (fileId != null) {
      _result.fileId = fileId;
    }
    if (sequence != null) {
      _result.sequence = sequence;
    }
    if (fileSize != null) {
      _result.fileSize = fileSize;
    }
    if (sizePerPackage != null) {
      _result.sizePerPackage = sizePerPackage;
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
  $fixnum.Int64 get fileId => $_getI64(0);
  @$pb.TagNumber(1)
  set fileId($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasFileId() => $_has(0);
  @$pb.TagNumber(1)
  void clearFileId() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get sequence => $_getIZ(1);
  @$pb.TagNumber(2)
  set sequence($core.int v) { $_setUnsignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasSequence() => $_has(1);
  @$pb.TagNumber(2)
  void clearSequence() => clearField(2);

  @$pb.TagNumber(3)
  $core.int get fileSize => $_getIZ(2);
  @$pb.TagNumber(3)
  set fileSize($core.int v) { $_setUnsignedInt32(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasFileSize() => $_has(2);
  @$pb.TagNumber(3)
  void clearFileSize() => clearField(3);

  @$pb.TagNumber(4)
  $core.int get sizePerPackage => $_getIZ(3);
  @$pb.TagNumber(4)
  set sizePerPackage($core.int v) { $_setUnsignedInt32(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasSizePerPackage() => $_has(3);
  @$pb.TagNumber(4)
  void clearSizePerPackage() => clearField(4);

  @$pb.TagNumber(6)
  $core.List<$core.int> get data => $_getN(4);
  @$pb.TagNumber(6)
  set data($core.List<$core.int> v) { $_setBytes(4, v); }
  @$pb.TagNumber(6)
  $core.bool hasData() => $_has(4);
  @$pb.TagNumber(6)
  void clearData() => clearField(6);
}

class FileSharingConfirmationFileInfo extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'FileSharingConfirmationFileInfo', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.net.filesharing'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fileId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  FileSharingConfirmationFileInfo._() : super();
  factory FileSharingConfirmationFileInfo({
    $fixnum.Int64? fileId,
  }) {
    final _result = create();
    if (fileId != null) {
      _result.fileId = fileId;
    }
    return _result;
  }
  factory FileSharingConfirmationFileInfo.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FileSharingConfirmationFileInfo.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  FileSharingConfirmationFileInfo clone() => FileSharingConfirmationFileInfo()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  FileSharingConfirmationFileInfo copyWith(void Function(FileSharingConfirmationFileInfo) updates) => super.copyWith((message) => updates(message as FileSharingConfirmationFileInfo)) as FileSharingConfirmationFileInfo; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static FileSharingConfirmationFileInfo create() => FileSharingConfirmationFileInfo._();
  FileSharingConfirmationFileInfo createEmptyInstance() => create();
  static $pb.PbList<FileSharingConfirmationFileInfo> createRepeated() => $pb.PbList<FileSharingConfirmationFileInfo>();
  @$core.pragma('dart2js:noInline')
  static FileSharingConfirmationFileInfo getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FileSharingConfirmationFileInfo>(create);
  static FileSharingConfirmationFileInfo? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get fileId => $_getI64(0);
  @$pb.TagNumber(1)
  set fileId($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasFileId() => $_has(0);
  @$pb.TagNumber(1)
  void clearFileId() => clearField(1);
}

class FileSharingConfirmation extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'FileSharingConfirmation', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.net.filesharing'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fileId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.int>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'sequence', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  FileSharingConfirmation._() : super();
  factory FileSharingConfirmation({
    $fixnum.Int64? fileId,
    $core.int? sequence,
  }) {
    final _result = create();
    if (fileId != null) {
      _result.fileId = fileId;
    }
    if (sequence != null) {
      _result.sequence = sequence;
    }
    return _result;
  }
  factory FileSharingConfirmation.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FileSharingConfirmation.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  FileSharingConfirmation clone() => FileSharingConfirmation()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  FileSharingConfirmation copyWith(void Function(FileSharingConfirmation) updates) => super.copyWith((message) => updates(message as FileSharingConfirmation)) as FileSharingConfirmation; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static FileSharingConfirmation create() => FileSharingConfirmation._();
  FileSharingConfirmation createEmptyInstance() => create();
  static $pb.PbList<FileSharingConfirmation> createRepeated() => $pb.PbList<FileSharingConfirmation>();
  @$core.pragma('dart2js:noInline')
  static FileSharingConfirmation getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FileSharingConfirmation>(create);
  static FileSharingConfirmation? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get fileId => $_getI64(0);
  @$pb.TagNumber(1)
  set fileId($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasFileId() => $_has(0);
  @$pb.TagNumber(1)
  void clearFileId() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get sequence => $_getIZ(1);
  @$pb.TagNumber(2)
  set sequence($core.int v) { $_setUnsignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasSequence() => $_has(1);
  @$pb.TagNumber(2)
  void clearSequence() => clearField(2);
}

class FileSharingCompleted extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'FileSharingCompleted', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.net.filesharing'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fileId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  FileSharingCompleted._() : super();
  factory FileSharingCompleted({
    $fixnum.Int64? fileId,
  }) {
    final _result = create();
    if (fileId != null) {
      _result.fileId = fileId;
    }
    return _result;
  }
  factory FileSharingCompleted.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FileSharingCompleted.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  FileSharingCompleted clone() => FileSharingCompleted()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  FileSharingCompleted copyWith(void Function(FileSharingCompleted) updates) => super.copyWith((message) => updates(message as FileSharingCompleted)) as FileSharingCompleted; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static FileSharingCompleted create() => FileSharingCompleted._();
  FileSharingCompleted createEmptyInstance() => create();
  static $pb.PbList<FileSharingCompleted> createRepeated() => $pb.PbList<FileSharingCompleted>();
  @$core.pragma('dart2js:noInline')
  static FileSharingCompleted getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FileSharingCompleted>(create);
  static FileSharingCompleted? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get fileId => $_getI64(0);
  @$pb.TagNumber(1)
  set fileId($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasFileId() => $_has(0);
  @$pb.TagNumber(1)
  void clearFileId() => clearField(1);
}

class FileSharingCanceled extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'FileSharingCanceled', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.net.filesharing'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fileId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  FileSharingCanceled._() : super();
  factory FileSharingCanceled({
    $fixnum.Int64? fileId,
  }) {
    final _result = create();
    if (fileId != null) {
      _result.fileId = fileId;
    }
    return _result;
  }
  factory FileSharingCanceled.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FileSharingCanceled.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  FileSharingCanceled clone() => FileSharingCanceled()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  FileSharingCanceled copyWith(void Function(FileSharingCanceled) updates) => super.copyWith((message) => updates(message as FileSharingCanceled)) as FileSharingCanceled; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static FileSharingCanceled create() => FileSharingCanceled._();
  FileSharingCanceled createEmptyInstance() => create();
  static $pb.PbList<FileSharingCanceled> createRepeated() => $pb.PbList<FileSharingCanceled>();
  @$core.pragma('dart2js:noInline')
  static FileSharingCanceled getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FileSharingCanceled>(create);
  static FileSharingCanceled? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get fileId => $_getI64(0);
  @$pb.TagNumber(1)
  set fileId($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasFileId() => $_has(0);
  @$pb.TagNumber(1)
  void clearFileId() => clearField(1);
}

