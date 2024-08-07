//
//  Generated code. Do not modify.
//  source: services/chat/chatfile_net.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

enum ChatFileContainer_Message {
  fileInfo, 
  fileData, 
  notSet
}

/// Chat file sending container
class ChatFileContainer extends $pb.GeneratedMessage {
  factory ChatFileContainer({
    ChatFileInfo? fileInfo,
    ChatFileData? fileData,
  }) {
    final $result = create();
    if (fileInfo != null) {
      $result.fileInfo = fileInfo;
    }
    if (fileData != null) {
      $result.fileData = fileData;
    }
    return $result;
  }
  ChatFileContainer._() : super();
  factory ChatFileContainer.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChatFileContainer.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static const $core.Map<$core.int, ChatFileContainer_Message> _ChatFileContainer_MessageByTag = {
    1 : ChatFileContainer_Message.fileInfo,
    2 : ChatFileContainer_Message.fileData,
    0 : ChatFileContainer_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ChatFileContainer', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.chatfile'), createEmptyInstance: create)
    ..oo(0, [1, 2])
    ..aOM<ChatFileInfo>(1, _omitFieldNames ? '' : 'fileInfo', subBuilder: ChatFileInfo.create)
    ..aOM<ChatFileData>(2, _omitFieldNames ? '' : 'fileData', subBuilder: ChatFileData.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChatFileContainer clone() => ChatFileContainer()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChatFileContainer copyWith(void Function(ChatFileContainer) updates) => super.copyWith((message) => updates(message as ChatFileContainer)) as ChatFileContainer;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ChatFileContainer create() => ChatFileContainer._();
  ChatFileContainer createEmptyInstance() => create();
  static $pb.PbList<ChatFileContainer> createRepeated() => $pb.PbList<ChatFileContainer>();
  @$core.pragma('dart2js:noInline')
  static ChatFileContainer getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChatFileContainer>(create);
  static ChatFileContainer? _defaultInstance;

  ChatFileContainer_Message whichMessage() => _ChatFileContainer_MessageByTag[$_whichOneof(0)]!;
  void clearMessage() => clearField($_whichOneof(0));

  /// Chat File Info Message
  @$pb.TagNumber(1)
  ChatFileInfo get fileInfo => $_getN(0);
  @$pb.TagNumber(1)
  set fileInfo(ChatFileInfo v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasFileInfo() => $_has(0);
  @$pb.TagNumber(1)
  void clearFileInfo() => clearField(1);
  @$pb.TagNumber(1)
  ChatFileInfo ensureFileInfo() => $_ensure(0);

  /// Chat File Data Message
  @$pb.TagNumber(2)
  ChatFileData get fileData => $_getN(1);
  @$pb.TagNumber(2)
  set fileData(ChatFileData v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasFileData() => $_has(1);
  @$pb.TagNumber(2)
  void clearFileData() => clearField(2);
  @$pb.TagNumber(2)
  ChatFileData ensureFileData() => $_ensure(1);
}

/// Chat File Info Message
class ChatFileInfo extends $pb.GeneratedMessage {
  factory ChatFileInfo({
    $fixnum.Int64? fileId,
    $core.String? fileName,
    $core.String? fileExtension,
    $core.int? fileSize,
    $core.String? fileDescription,
    $core.int? startIndex,
    $core.int? messageCount,
    $core.int? dataChunkSize,
  }) {
    final $result = create();
    if (fileId != null) {
      $result.fileId = fileId;
    }
    if (fileName != null) {
      $result.fileName = fileName;
    }
    if (fileExtension != null) {
      $result.fileExtension = fileExtension;
    }
    if (fileSize != null) {
      $result.fileSize = fileSize;
    }
    if (fileDescription != null) {
      $result.fileDescription = fileDescription;
    }
    if (startIndex != null) {
      $result.startIndex = startIndex;
    }
    if (messageCount != null) {
      $result.messageCount = messageCount;
    }
    if (dataChunkSize != null) {
      $result.dataChunkSize = dataChunkSize;
    }
    return $result;
  }
  ChatFileInfo._() : super();
  factory ChatFileInfo.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChatFileInfo.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ChatFileInfo', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.chatfile'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, _omitFieldNames ? '' : 'fileId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOS(2, _omitFieldNames ? '' : 'fileName')
    ..aOS(3, _omitFieldNames ? '' : 'fileExtension')
    ..a<$core.int>(4, _omitFieldNames ? '' : 'fileSize', $pb.PbFieldType.OU3)
    ..aOS(5, _omitFieldNames ? '' : 'fileDescription')
    ..a<$core.int>(6, _omitFieldNames ? '' : 'startIndex', $pb.PbFieldType.OU3)
    ..a<$core.int>(7, _omitFieldNames ? '' : 'messageCount', $pb.PbFieldType.OU3)
    ..a<$core.int>(8, _omitFieldNames ? '' : 'dataChunkSize', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChatFileInfo clone() => ChatFileInfo()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChatFileInfo copyWith(void Function(ChatFileInfo) updates) => super.copyWith((message) => updates(message as ChatFileInfo)) as ChatFileInfo;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ChatFileInfo create() => ChatFileInfo._();
  ChatFileInfo createEmptyInstance() => create();
  static $pb.PbList<ChatFileInfo> createRepeated() => $pb.PbList<ChatFileInfo>();
  @$core.pragma('dart2js:noInline')
  static ChatFileInfo getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChatFileInfo>(create);
  static ChatFileInfo? _defaultInstance;

  /// file id
  @$pb.TagNumber(1)
  $fixnum.Int64 get fileId => $_getI64(0);
  @$pb.TagNumber(1)
  set fileId($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasFileId() => $_has(0);
  @$pb.TagNumber(1)
  void clearFileId() => clearField(1);

  /// file name
  @$pb.TagNumber(2)
  $core.String get fileName => $_getSZ(1);
  @$pb.TagNumber(2)
  set fileName($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasFileName() => $_has(1);
  @$pb.TagNumber(2)
  void clearFileName() => clearField(2);

  /// file extension
  @$pb.TagNumber(3)
  $core.String get fileExtension => $_getSZ(2);
  @$pb.TagNumber(3)
  set fileExtension($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasFileExtension() => $_has(2);
  @$pb.TagNumber(3)
  void clearFileExtension() => clearField(3);

  /// file size
  @$pb.TagNumber(4)
  $core.int get fileSize => $_getIZ(3);
  @$pb.TagNumber(4)
  set fileSize($core.int v) { $_setUnsignedInt32(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasFileSize() => $_has(3);
  @$pb.TagNumber(4)
  void clearFileSize() => clearField(4);

  /// file description
  @$pb.TagNumber(5)
  $core.String get fileDescription => $_getSZ(4);
  @$pb.TagNumber(5)
  set fileDescription($core.String v) { $_setString(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasFileDescription() => $_has(4);
  @$pb.TagNumber(5)
  void clearFileDescription() => clearField(5);

  /// DEPRECATED: What is this used for?
  /// start index
  @$pb.TagNumber(6)
  $core.int get startIndex => $_getIZ(5);
  @$pb.TagNumber(6)
  set startIndex($core.int v) { $_setUnsignedInt32(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasStartIndex() => $_has(5);
  @$pb.TagNumber(6)
  void clearStartIndex() => clearField(6);

  /// message count
  @$pb.TagNumber(7)
  $core.int get messageCount => $_getIZ(6);
  @$pb.TagNumber(7)
  set messageCount($core.int v) { $_setUnsignedInt32(6, v); }
  @$pb.TagNumber(7)
  $core.bool hasMessageCount() => $_has(6);
  @$pb.TagNumber(7)
  void clearMessageCount() => clearField(7);

  /// file data chunk size
  @$pb.TagNumber(8)
  $core.int get dataChunkSize => $_getIZ(7);
  @$pb.TagNumber(8)
  set dataChunkSize($core.int v) { $_setUnsignedInt32(7, v); }
  @$pb.TagNumber(8)
  $core.bool hasDataChunkSize() => $_has(7);
  @$pb.TagNumber(8)
  void clearDataChunkSize() => clearField(8);
}

/// Chat File Data Message
class ChatFileData extends $pb.GeneratedMessage {
  factory ChatFileData({
    $fixnum.Int64? fileId,
    $core.int? startIndex,
    $core.int? messageCount,
    $core.List<$core.int>? data,
  }) {
    final $result = create();
    if (fileId != null) {
      $result.fileId = fileId;
    }
    if (startIndex != null) {
      $result.startIndex = startIndex;
    }
    if (messageCount != null) {
      $result.messageCount = messageCount;
    }
    if (data != null) {
      $result.data = data;
    }
    return $result;
  }
  ChatFileData._() : super();
  factory ChatFileData.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChatFileData.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ChatFileData', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.chatfile'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, _omitFieldNames ? '' : 'fileId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.int>(2, _omitFieldNames ? '' : 'startIndex', $pb.PbFieldType.OU3)
    ..a<$core.int>(3, _omitFieldNames ? '' : 'messageCount', $pb.PbFieldType.OU3)
    ..a<$core.List<$core.int>>(4, _omitFieldNames ? '' : 'data', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChatFileData clone() => ChatFileData()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChatFileData copyWith(void Function(ChatFileData) updates) => super.copyWith((message) => updates(message as ChatFileData)) as ChatFileData;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ChatFileData create() => ChatFileData._();
  ChatFileData createEmptyInstance() => create();
  static $pb.PbList<ChatFileData> createRepeated() => $pb.PbList<ChatFileData>();
  @$core.pragma('dart2js:noInline')
  static ChatFileData getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChatFileData>(create);
  static ChatFileData? _defaultInstance;

  /// file id
  @$pb.TagNumber(1)
  $fixnum.Int64 get fileId => $_getI64(0);
  @$pb.TagNumber(1)
  set fileId($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasFileId() => $_has(0);
  @$pb.TagNumber(1)
  void clearFileId() => clearField(1);

  /// start index
  @$pb.TagNumber(2)
  $core.int get startIndex => $_getIZ(1);
  @$pb.TagNumber(2)
  set startIndex($core.int v) { $_setUnsignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasStartIndex() => $_has(1);
  @$pb.TagNumber(2)
  void clearStartIndex() => clearField(2);

  /// message count
  @$pb.TagNumber(3)
  $core.int get messageCount => $_getIZ(2);
  @$pb.TagNumber(3)
  set messageCount($core.int v) { $_setUnsignedInt32(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasMessageCount() => $_has(2);
  @$pb.TagNumber(3)
  void clearMessageCount() => clearField(3);

  /// package data
  @$pb.TagNumber(4)
  $core.List<$core.int> get data => $_getN(3);
  @$pb.TagNumber(4)
  set data($core.List<$core.int> v) { $_setBytes(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasData() => $_has(3);
  @$pb.TagNumber(4)
  void clearData() => clearField(4);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
