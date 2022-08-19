///
//  Generated code. Do not modify.
//  source: services/filesharing/filesharing_rpc.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

enum FileSharing_Message {
  sendFileRequest, 
  fileHistory, 
  fileHistoryResponse, 
  notSet
}

class FileSharing extends $pb.GeneratedMessage {
  static const $core.Map<$core.int, FileSharing_Message> _FileSharing_MessageByTag = {
    1 : FileSharing_Message.sendFileRequest,
    2 : FileSharing_Message.fileHistory,
    3 : FileSharing_Message.fileHistoryResponse,
    0 : FileSharing_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'FileSharing', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.filesharing'), createEmptyInstance: create)
    ..oo(0, [1, 2, 3])
    ..aOM<SendFileRequest>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'sendFileRequest', subBuilder: SendFileRequest.create)
    ..aOM<FileHistoryRequest>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fileHistory', subBuilder: FileHistoryRequest.create)
    ..aOM<FileHistoryResponse>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fileHistoryResponse', subBuilder: FileHistoryResponse.create)
    ..hasRequiredFields = false
  ;

  FileSharing._() : super();
  factory FileSharing({
    SendFileRequest? sendFileRequest,
    FileHistoryRequest? fileHistory,
    FileHistoryResponse? fileHistoryResponse,
  }) {
    final _result = create();
    if (sendFileRequest != null) {
      _result.sendFileRequest = sendFileRequest;
    }
    if (fileHistory != null) {
      _result.fileHistory = fileHistory;
    }
    if (fileHistoryResponse != null) {
      _result.fileHistoryResponse = fileHistoryResponse;
    }
    return _result;
  }
  factory FileSharing.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FileSharing.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  FileSharing clone() => FileSharing()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  FileSharing copyWith(void Function(FileSharing) updates) => super.copyWith((message) => updates(message as FileSharing)) as FileSharing; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static FileSharing create() => FileSharing._();
  FileSharing createEmptyInstance() => create();
  static $pb.PbList<FileSharing> createRepeated() => $pb.PbList<FileSharing>();
  @$core.pragma('dart2js:noInline')
  static FileSharing getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FileSharing>(create);
  static FileSharing? _defaultInstance;

  FileSharing_Message whichMessage() => _FileSharing_MessageByTag[$_whichOneof(0)]!;
  void clearMessage() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  SendFileRequest get sendFileRequest => $_getN(0);
  @$pb.TagNumber(1)
  set sendFileRequest(SendFileRequest v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasSendFileRequest() => $_has(0);
  @$pb.TagNumber(1)
  void clearSendFileRequest() => clearField(1);
  @$pb.TagNumber(1)
  SendFileRequest ensureSendFileRequest() => $_ensure(0);

  @$pb.TagNumber(2)
  FileHistoryRequest get fileHistory => $_getN(1);
  @$pb.TagNumber(2)
  set fileHistory(FileHistoryRequest v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasFileHistory() => $_has(1);
  @$pb.TagNumber(2)
  void clearFileHistory() => clearField(2);
  @$pb.TagNumber(2)
  FileHistoryRequest ensureFileHistory() => $_ensure(1);

  @$pb.TagNumber(3)
  FileHistoryResponse get fileHistoryResponse => $_getN(2);
  @$pb.TagNumber(3)
  set fileHistoryResponse(FileHistoryResponse v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasFileHistoryResponse() => $_has(2);
  @$pb.TagNumber(3)
  void clearFileHistoryResponse() => clearField(3);
  @$pb.TagNumber(3)
  FileHistoryResponse ensureFileHistoryResponse() => $_ensure(2);
}

class SendFileRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'SendFileRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.filesharing'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'pathName')
    ..a<$core.List<$core.int>>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'conversationId', $pb.PbFieldType.OY)
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'description')
    ..hasRequiredFields = false
  ;

  SendFileRequest._() : super();
  factory SendFileRequest({
    $core.String? pathName,
    $core.List<$core.int>? conversationId,
    $core.String? description,
  }) {
    final _result = create();
    if (pathName != null) {
      _result.pathName = pathName;
    }
    if (conversationId != null) {
      _result.conversationId = conversationId;
    }
    if (description != null) {
      _result.description = description;
    }
    return _result;
  }
  factory SendFileRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory SendFileRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  SendFileRequest clone() => SendFileRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  SendFileRequest copyWith(void Function(SendFileRequest) updates) => super.copyWith((message) => updates(message as SendFileRequest)) as SendFileRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static SendFileRequest create() => SendFileRequest._();
  SendFileRequest createEmptyInstance() => create();
  static $pb.PbList<SendFileRequest> createRepeated() => $pb.PbList<SendFileRequest>();
  @$core.pragma('dart2js:noInline')
  static SendFileRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SendFileRequest>(create);
  static SendFileRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get pathName => $_getSZ(0);
  @$pb.TagNumber(1)
  set pathName($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasPathName() => $_has(0);
  @$pb.TagNumber(1)
  void clearPathName() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get conversationId => $_getN(1);
  @$pb.TagNumber(2)
  set conversationId($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasConversationId() => $_has(1);
  @$pb.TagNumber(2)
  void clearConversationId() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get description => $_getSZ(2);
  @$pb.TagNumber(3)
  set description($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasDescription() => $_has(2);
  @$pb.TagNumber(3)
  void clearDescription() => clearField(3);
}

class FileHistoryRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'FileHistoryRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.filesharing'), createEmptyInstance: create)
    ..a<$core.int>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'offset', $pb.PbFieldType.OU3)
    ..a<$core.int>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'limit', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  FileHistoryRequest._() : super();
  factory FileHistoryRequest({
    $core.int? offset,
    $core.int? limit,
  }) {
    final _result = create();
    if (offset != null) {
      _result.offset = offset;
    }
    if (limit != null) {
      _result.limit = limit;
    }
    return _result;
  }
  factory FileHistoryRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FileHistoryRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  FileHistoryRequest clone() => FileHistoryRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  FileHistoryRequest copyWith(void Function(FileHistoryRequest) updates) => super.copyWith((message) => updates(message as FileHistoryRequest)) as FileHistoryRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static FileHistoryRequest create() => FileHistoryRequest._();
  FileHistoryRequest createEmptyInstance() => create();
  static $pb.PbList<FileHistoryRequest> createRepeated() => $pb.PbList<FileHistoryRequest>();
  @$core.pragma('dart2js:noInline')
  static FileHistoryRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FileHistoryRequest>(create);
  static FileHistoryRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get offset => $_getIZ(0);
  @$pb.TagNumber(1)
  set offset($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasOffset() => $_has(0);
  @$pb.TagNumber(1)
  void clearOffset() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get limit => $_getIZ(1);
  @$pb.TagNumber(2)
  set limit($core.int v) { $_setUnsignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasLimit() => $_has(1);
  @$pb.TagNumber(2)
  void clearLimit() => clearField(2);
}

class FileHistoryEntry extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'FileHistoryEntry', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.filesharing'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fileId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fileName')
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fileExt')
    ..a<$core.int>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fileSize', $pb.PbFieldType.OU3)
    ..aOS(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fileDescr')
    ..a<$fixnum.Int64>(6, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'time', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOS(7, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'senderId')
    ..aOS(8, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupId')
    ..hasRequiredFields = false
  ;

  FileHistoryEntry._() : super();
  factory FileHistoryEntry({
    $fixnum.Int64? fileId,
    $core.String? fileName,
    $core.String? fileExt,
    $core.int? fileSize,
    $core.String? fileDescr,
    $fixnum.Int64? time,
    $core.String? senderId,
    $core.String? groupId,
  }) {
    final _result = create();
    if (fileId != null) {
      _result.fileId = fileId;
    }
    if (fileName != null) {
      _result.fileName = fileName;
    }
    if (fileExt != null) {
      _result.fileExt = fileExt;
    }
    if (fileSize != null) {
      _result.fileSize = fileSize;
    }
    if (fileDescr != null) {
      _result.fileDescr = fileDescr;
    }
    if (time != null) {
      _result.time = time;
    }
    if (senderId != null) {
      _result.senderId = senderId;
    }
    if (groupId != null) {
      _result.groupId = groupId;
    }
    return _result;
  }
  factory FileHistoryEntry.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FileHistoryEntry.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  FileHistoryEntry clone() => FileHistoryEntry()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  FileHistoryEntry copyWith(void Function(FileHistoryEntry) updates) => super.copyWith((message) => updates(message as FileHistoryEntry)) as FileHistoryEntry; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static FileHistoryEntry create() => FileHistoryEntry._();
  FileHistoryEntry createEmptyInstance() => create();
  static $pb.PbList<FileHistoryEntry> createRepeated() => $pb.PbList<FileHistoryEntry>();
  @$core.pragma('dart2js:noInline')
  static FileHistoryEntry getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FileHistoryEntry>(create);
  static FileHistoryEntry? _defaultInstance;

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
  $core.String get fileExt => $_getSZ(2);
  @$pb.TagNumber(3)
  set fileExt($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasFileExt() => $_has(2);
  @$pb.TagNumber(3)
  void clearFileExt() => clearField(3);

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
  $fixnum.Int64 get time => $_getI64(5);
  @$pb.TagNumber(6)
  set time($fixnum.Int64 v) { $_setInt64(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasTime() => $_has(5);
  @$pb.TagNumber(6)
  void clearTime() => clearField(6);

  @$pb.TagNumber(7)
  $core.String get senderId => $_getSZ(6);
  @$pb.TagNumber(7)
  set senderId($core.String v) { $_setString(6, v); }
  @$pb.TagNumber(7)
  $core.bool hasSenderId() => $_has(6);
  @$pb.TagNumber(7)
  void clearSenderId() => clearField(7);

  @$pb.TagNumber(8)
  $core.String get groupId => $_getSZ(7);
  @$pb.TagNumber(8)
  set groupId($core.String v) { $_setString(7, v); }
  @$pb.TagNumber(8)
  $core.bool hasGroupId() => $_has(7);
  @$pb.TagNumber(8)
  void clearGroupId() => clearField(8);
}

class FileHistoryResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'FileHistoryResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.filesharing'), createEmptyInstance: create)
    ..a<$core.int>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'offset', $pb.PbFieldType.OU3)
    ..a<$core.int>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'limit', $pb.PbFieldType.OU3)
    ..a<$fixnum.Int64>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'total', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..pc<FileHistoryEntry>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'histories', $pb.PbFieldType.PM, subBuilder: FileHistoryEntry.create)
    ..hasRequiredFields = false
  ;

  FileHistoryResponse._() : super();
  factory FileHistoryResponse({
    $core.int? offset,
    $core.int? limit,
    $fixnum.Int64? total,
    $core.Iterable<FileHistoryEntry>? histories,
  }) {
    final _result = create();
    if (offset != null) {
      _result.offset = offset;
    }
    if (limit != null) {
      _result.limit = limit;
    }
    if (total != null) {
      _result.total = total;
    }
    if (histories != null) {
      _result.histories.addAll(histories);
    }
    return _result;
  }
  factory FileHistoryResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FileHistoryResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  FileHistoryResponse clone() => FileHistoryResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  FileHistoryResponse copyWith(void Function(FileHistoryResponse) updates) => super.copyWith((message) => updates(message as FileHistoryResponse)) as FileHistoryResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static FileHistoryResponse create() => FileHistoryResponse._();
  FileHistoryResponse createEmptyInstance() => create();
  static $pb.PbList<FileHistoryResponse> createRepeated() => $pb.PbList<FileHistoryResponse>();
  @$core.pragma('dart2js:noInline')
  static FileHistoryResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FileHistoryResponse>(create);
  static FileHistoryResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get offset => $_getIZ(0);
  @$pb.TagNumber(1)
  set offset($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasOffset() => $_has(0);
  @$pb.TagNumber(1)
  void clearOffset() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get limit => $_getIZ(1);
  @$pb.TagNumber(2)
  set limit($core.int v) { $_setUnsignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasLimit() => $_has(1);
  @$pb.TagNumber(2)
  void clearLimit() => clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get total => $_getI64(2);
  @$pb.TagNumber(3)
  set total($fixnum.Int64 v) { $_setInt64(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasTotal() => $_has(2);
  @$pb.TagNumber(3)
  void clearTotal() => clearField(3);

  @$pb.TagNumber(4)
  $core.List<FileHistoryEntry> get histories => $_getList(3);
}

