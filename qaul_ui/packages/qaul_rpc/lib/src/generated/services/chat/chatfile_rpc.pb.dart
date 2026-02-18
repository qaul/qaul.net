// This is a generated file - do not edit.
//
// Generated from services/chat/chatfile_rpc.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

enum ChatFile_Message {
  sendFileRequest,
  sendFileResponse,
  fileHistory,
  fileHistoryResponse,
  notSet
}

/// Chat file RPC message container
class ChatFile extends $pb.GeneratedMessage {
  factory ChatFile({
    SendFileRequest? sendFileRequest,
    SendFileResponse? sendFileResponse,
    FileHistoryRequest? fileHistory,
    FileHistoryResponse? fileHistoryResponse,
  }) {
    final result = create();
    if (sendFileRequest != null) result.sendFileRequest = sendFileRequest;
    if (sendFileResponse != null) result.sendFileResponse = sendFileResponse;
    if (fileHistory != null) result.fileHistory = fileHistory;
    if (fileHistoryResponse != null)
      result.fileHistoryResponse = fileHistoryResponse;
    return result;
  }

  ChatFile._();

  factory ChatFile.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ChatFile.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, ChatFile_Message> _ChatFile_MessageByTag = {
    1: ChatFile_Message.sendFileRequest,
    2: ChatFile_Message.sendFileResponse,
    3: ChatFile_Message.fileHistory,
    4: ChatFile_Message.fileHistoryResponse,
    0: ChatFile_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ChatFile',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.chatfile'),
      createEmptyInstance: create)
    ..oo(0, [1, 2, 3, 4])
    ..aOM<SendFileRequest>(1, _omitFieldNames ? '' : 'sendFileRequest',
        subBuilder: SendFileRequest.create)
    ..aOM<SendFileResponse>(2, _omitFieldNames ? '' : 'sendFileResponse',
        subBuilder: SendFileResponse.create)
    ..aOM<FileHistoryRequest>(3, _omitFieldNames ? '' : 'fileHistory',
        subBuilder: FileHistoryRequest.create)
    ..aOM<FileHistoryResponse>(4, _omitFieldNames ? '' : 'fileHistoryResponse',
        subBuilder: FileHistoryResponse.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ChatFile clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ChatFile copyWith(void Function(ChatFile) updates) =>
      super.copyWith((message) => updates(message as ChatFile)) as ChatFile;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ChatFile create() => ChatFile._();
  @$core.override
  ChatFile createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ChatFile getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChatFile>(create);
  static ChatFile? _defaultInstance;

  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  ChatFile_Message whichMessage() => _ChatFile_MessageByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  void clearMessage() => $_clearField($_whichOneof(0));

  /// send file request
  ///
  /// this messages sends a file from UI to libqaul
  @$pb.TagNumber(1)
  SendFileRequest get sendFileRequest => $_getN(0);
  @$pb.TagNumber(1)
  set sendFileRequest(SendFileRequest value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasSendFileRequest() => $_has(0);
  @$pb.TagNumber(1)
  void clearSendFileRequest() => $_clearField(1);
  @$pb.TagNumber(1)
  SendFileRequest ensureSendFileRequest() => $_ensure(0);

  /// send file response
  ///
  /// response message from libqaul to the UI about
  /// the result of the send file request
  @$pb.TagNumber(2)
  SendFileResponse get sendFileResponse => $_getN(1);
  @$pb.TagNumber(2)
  set sendFileResponse(SendFileResponse value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasSendFileResponse() => $_has(1);
  @$pb.TagNumber(2)
  void clearSendFileResponse() => $_clearField(2);
  @$pb.TagNumber(2)
  SendFileResponse ensureSendFileResponse() => $_ensure(1);

  /// file history request
  ///
  /// request a paginated list of
  @$pb.TagNumber(3)
  FileHistoryRequest get fileHistory => $_getN(2);
  @$pb.TagNumber(3)
  set fileHistory(FileHistoryRequest value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasFileHistory() => $_has(2);
  @$pb.TagNumber(3)
  void clearFileHistory() => $_clearField(3);
  @$pb.TagNumber(3)
  FileHistoryRequest ensureFileHistory() => $_ensure(2);

  /// file history response
  ///
  /// delivers the requested list of
  @$pb.TagNumber(4)
  FileHistoryResponse get fileHistoryResponse => $_getN(3);
  @$pb.TagNumber(4)
  set fileHistoryResponse(FileHistoryResponse value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasFileHistoryResponse() => $_has(3);
  @$pb.TagNumber(4)
  void clearFileHistoryResponse() => $_clearField(4);
  @$pb.TagNumber(4)
  FileHistoryResponse ensureFileHistoryResponse() => $_ensure(3);
}

/// Send File Request
///
/// UI requests libqaul to send a file
class SendFileRequest extends $pb.GeneratedMessage {
  factory SendFileRequest({
    $core.String? pathName,
    $core.List<$core.int>? groupId,
    $core.String? description,
  }) {
    final result = create();
    if (pathName != null) result.pathName = pathName;
    if (groupId != null) result.groupId = groupId;
    if (description != null) result.description = description;
    return result;
  }

  SendFileRequest._();

  factory SendFileRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory SendFileRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SendFileRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.chatfile'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'pathName')
    ..a<$core.List<$core.int>>(
        2, _omitFieldNames ? '' : 'groupId', $pb.PbFieldType.OY)
    ..aOS(3, _omitFieldNames ? '' : 'description')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SendFileRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SendFileRequest copyWith(void Function(SendFileRequest) updates) =>
      super.copyWith((message) => updates(message as SendFileRequest))
          as SendFileRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SendFileRequest create() => SendFileRequest._();
  @$core.override
  SendFileRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static SendFileRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SendFileRequest>(create);
  static SendFileRequest? _defaultInstance;

  /// file path with file name to send
  @$pb.TagNumber(1)
  $core.String get pathName => $_getSZ(0);
  @$pb.TagNumber(1)
  set pathName($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasPathName() => $_has(0);
  @$pb.TagNumber(1)
  void clearPathName() => $_clearField(1);

  /// group id to receive file
  @$pb.TagNumber(2)
  $core.List<$core.int> get groupId => $_getN(1);
  @$pb.TagNumber(2)
  set groupId($core.List<$core.int> value) => $_setBytes(1, value);
  @$pb.TagNumber(2)
  $core.bool hasGroupId() => $_has(1);
  @$pb.TagNumber(2)
  void clearGroupId() => $_clearField(2);

  /// file description text to be sent in the message
  @$pb.TagNumber(3)
  $core.String get description => $_getSZ(2);
  @$pb.TagNumber(3)
  set description($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasDescription() => $_has(2);
  @$pb.TagNumber(3)
  void clearDescription() => $_clearField(3);
}

/// Send File Response
///
/// sends the result of the file send request to the UI
class SendFileResponse extends $pb.GeneratedMessage {
  factory SendFileResponse({
    $core.bool? success,
    $core.String? error,
    $fixnum.Int64? fileId,
  }) {
    final result = create();
    if (success != null) result.success = success;
    if (error != null) result.error = error;
    if (fileId != null) result.fileId = fileId;
    return result;
  }

  SendFileResponse._();

  factory SendFileResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory SendFileResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SendFileResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.chatfile'),
      createEmptyInstance: create)
    ..aOB(1, _omitFieldNames ? '' : 'success')
    ..aOS(2, _omitFieldNames ? '' : 'error')
    ..a<$fixnum.Int64>(3, _omitFieldNames ? '' : 'fileId', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SendFileResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SendFileResponse copyWith(void Function(SendFileResponse) updates) =>
      super.copyWith((message) => updates(message as SendFileResponse))
          as SendFileResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SendFileResponse create() => SendFileResponse._();
  @$core.override
  SendFileResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static SendFileResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SendFileResponse>(create);
  static SendFileResponse? _defaultInstance;

  /// was the file processing successful
  ///
  /// a success does not mean the file has been sent,
  /// but that it was successfully scheduled for sending.
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
  $core.String get error => $_getSZ(1);
  @$pb.TagNumber(2)
  set error($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasError() => $_has(1);
  @$pb.TagNumber(2)
  void clearError() => $_clearField(2);

  /// file ID (only present if the sending was a success)
  @$pb.TagNumber(3)
  $fixnum.Int64 get fileId => $_getI64(2);
  @$pb.TagNumber(3)
  set fileId($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasFileId() => $_has(2);
  @$pb.TagNumber(3)
  void clearFileId() => $_clearField(3);
}

/// File History Request
class FileHistoryRequest extends $pb.GeneratedMessage {
  factory FileHistoryRequest({
    $core.int? offset,
    $core.int? limit,
  }) {
    final result = create();
    if (offset != null) result.offset = offset;
    if (limit != null) result.limit = limit;
    return result;
  }

  FileHistoryRequest._();

  factory FileHistoryRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory FileHistoryRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'FileHistoryRequest',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.chatfile'),
      createEmptyInstance: create)
    ..aI(1, _omitFieldNames ? '' : 'offset', fieldType: $pb.PbFieldType.OU3)
    ..aI(2, _omitFieldNames ? '' : 'limit', fieldType: $pb.PbFieldType.OU3)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  FileHistoryRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  FileHistoryRequest copyWith(void Function(FileHistoryRequest) updates) =>
      super.copyWith((message) => updates(message as FileHistoryRequest))
          as FileHistoryRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FileHistoryRequest create() => FileHistoryRequest._();
  @$core.override
  FileHistoryRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static FileHistoryRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<FileHistoryRequest>(create);
  static FileHistoryRequest? _defaultInstance;

  /// offset
  @$pb.TagNumber(1)
  $core.int get offset => $_getIZ(0);
  @$pb.TagNumber(1)
  set offset($core.int value) => $_setUnsignedInt32(0, value);
  @$pb.TagNumber(1)
  $core.bool hasOffset() => $_has(0);
  @$pb.TagNumber(1)
  void clearOffset() => $_clearField(1);

  /// limit
  @$pb.TagNumber(2)
  $core.int get limit => $_getIZ(1);
  @$pb.TagNumber(2)
  set limit($core.int value) => $_setUnsignedInt32(1, value);
  @$pb.TagNumber(2)
  $core.bool hasLimit() => $_has(1);
  @$pb.TagNumber(2)
  void clearLimit() => $_clearField(2);
}

/// File History Entry
class FileHistoryEntry extends $pb.GeneratedMessage {
  factory FileHistoryEntry({
    $fixnum.Int64? fileId,
    $core.String? fileName,
    $core.String? fileExtension,
    $core.int? fileSize,
    $core.String? fileDescription,
    $fixnum.Int64? time,
    $core.String? senderId,
    $core.String? groupId,
  }) {
    final result = create();
    if (fileId != null) result.fileId = fileId;
    if (fileName != null) result.fileName = fileName;
    if (fileExtension != null) result.fileExtension = fileExtension;
    if (fileSize != null) result.fileSize = fileSize;
    if (fileDescription != null) result.fileDescription = fileDescription;
    if (time != null) result.time = time;
    if (senderId != null) result.senderId = senderId;
    if (groupId != null) result.groupId = groupId;
    return result;
  }

  FileHistoryEntry._();

  factory FileHistoryEntry.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory FileHistoryEntry.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'FileHistoryEntry',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.chatfile'),
      createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, _omitFieldNames ? '' : 'fileId', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOS(2, _omitFieldNames ? '' : 'fileName')
    ..aOS(3, _omitFieldNames ? '' : 'fileExtension')
    ..aI(4, _omitFieldNames ? '' : 'fileSize', fieldType: $pb.PbFieldType.OU3)
    ..aOS(5, _omitFieldNames ? '' : 'fileDescription')
    ..a<$fixnum.Int64>(6, _omitFieldNames ? '' : 'time', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOS(7, _omitFieldNames ? '' : 'senderId')
    ..aOS(8, _omitFieldNames ? '' : 'groupId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  FileHistoryEntry clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  FileHistoryEntry copyWith(void Function(FileHistoryEntry) updates) =>
      super.copyWith((message) => updates(message as FileHistoryEntry))
          as FileHistoryEntry;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FileHistoryEntry create() => FileHistoryEntry._();
  @$core.override
  FileHistoryEntry createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static FileHistoryEntry getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<FileHistoryEntry>(create);
  static FileHistoryEntry? _defaultInstance;

  /// file id
  @$pb.TagNumber(1)
  $fixnum.Int64 get fileId => $_getI64(0);
  @$pb.TagNumber(1)
  set fileId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasFileId() => $_has(0);
  @$pb.TagNumber(1)
  void clearFileId() => $_clearField(1);

  /// file name (without extension)
  @$pb.TagNumber(2)
  $core.String get fileName => $_getSZ(1);
  @$pb.TagNumber(2)
  set fileName($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasFileName() => $_has(1);
  @$pb.TagNumber(2)
  void clearFileName() => $_clearField(2);

  /// file extension
  @$pb.TagNumber(3)
  $core.String get fileExtension => $_getSZ(2);
  @$pb.TagNumber(3)
  set fileExtension($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasFileExtension() => $_has(2);
  @$pb.TagNumber(3)
  void clearFileExtension() => $_clearField(3);

  /// file size
  @$pb.TagNumber(4)
  $core.int get fileSize => $_getIZ(3);
  @$pb.TagNumber(4)
  set fileSize($core.int value) => $_setUnsignedInt32(3, value);
  @$pb.TagNumber(4)
  $core.bool hasFileSize() => $_has(3);
  @$pb.TagNumber(4)
  void clearFileSize() => $_clearField(4);

  /// file description
  @$pb.TagNumber(5)
  $core.String get fileDescription => $_getSZ(4);
  @$pb.TagNumber(5)
  set fileDescription($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasFileDescription() => $_has(4);
  @$pb.TagNumber(5)
  void clearFileDescription() => $_clearField(5);

  /// time
  @$pb.TagNumber(6)
  $fixnum.Int64 get time => $_getI64(5);
  @$pb.TagNumber(6)
  set time($fixnum.Int64 value) => $_setInt64(5, value);
  @$pb.TagNumber(6)
  $core.bool hasTime() => $_has(5);
  @$pb.TagNumber(6)
  void clearTime() => $_clearField(6);

  /// sender id
  @$pb.TagNumber(7)
  $core.String get senderId => $_getSZ(6);
  @$pb.TagNumber(7)
  set senderId($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasSenderId() => $_has(6);
  @$pb.TagNumber(7)
  void clearSenderId() => $_clearField(7);

  /// group id
  @$pb.TagNumber(8)
  $core.String get groupId => $_getSZ(7);
  @$pb.TagNumber(8)
  set groupId($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasGroupId() => $_has(7);
  @$pb.TagNumber(8)
  void clearGroupId() => $_clearField(8);
}

/// File History Response
class FileHistoryResponse extends $pb.GeneratedMessage {
  factory FileHistoryResponse({
    $core.int? offset,
    $core.int? limit,
    $fixnum.Int64? total,
    $core.Iterable<FileHistoryEntry>? histories,
  }) {
    final result = create();
    if (offset != null) result.offset = offset;
    if (limit != null) result.limit = limit;
    if (total != null) result.total = total;
    if (histories != null) result.histories.addAll(histories);
    return result;
  }

  FileHistoryResponse._();

  factory FileHistoryResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory FileHistoryResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'FileHistoryResponse',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.chatfile'),
      createEmptyInstance: create)
    ..aI(1, _omitFieldNames ? '' : 'offset', fieldType: $pb.PbFieldType.OU3)
    ..aI(2, _omitFieldNames ? '' : 'limit', fieldType: $pb.PbFieldType.OU3)
    ..a<$fixnum.Int64>(3, _omitFieldNames ? '' : 'total', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..pPM<FileHistoryEntry>(4, _omitFieldNames ? '' : 'histories',
        subBuilder: FileHistoryEntry.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  FileHistoryResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  FileHistoryResponse copyWith(void Function(FileHistoryResponse) updates) =>
      super.copyWith((message) => updates(message as FileHistoryResponse))
          as FileHistoryResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FileHistoryResponse create() => FileHistoryResponse._();
  @$core.override
  FileHistoryResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static FileHistoryResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<FileHistoryResponse>(create);
  static FileHistoryResponse? _defaultInstance;

  /// offset
  @$pb.TagNumber(1)
  $core.int get offset => $_getIZ(0);
  @$pb.TagNumber(1)
  set offset($core.int value) => $_setUnsignedInt32(0, value);
  @$pb.TagNumber(1)
  $core.bool hasOffset() => $_has(0);
  @$pb.TagNumber(1)
  void clearOffset() => $_clearField(1);

  /// limit
  @$pb.TagNumber(2)
  $core.int get limit => $_getIZ(1);
  @$pb.TagNumber(2)
  set limit($core.int value) => $_setUnsignedInt32(1, value);
  @$pb.TagNumber(2)
  $core.bool hasLimit() => $_has(1);
  @$pb.TagNumber(2)
  void clearLimit() => $_clearField(2);

  /// limit
  @$pb.TagNumber(3)
  $fixnum.Int64 get total => $_getI64(2);
  @$pb.TagNumber(3)
  set total($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasTotal() => $_has(2);
  @$pb.TagNumber(3)
  void clearTotal() => $_clearField(3);

  /// histories
  @$pb.TagNumber(4)
  $pb.PbList<FileHistoryEntry> get histories => $_getList(3);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
