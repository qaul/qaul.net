// This is a generated file - do not edit.
//
// Generated from services/group/group_rpc.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'group_rpc.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'group_rpc.pbenum.dart';

enum Group_Message {
  groupCreateRequest,
  groupCreateResponse,
  groupRenameRequest,
  groupRenameResponse,
  groupInviteMemberRequest,
  groupInviteMemberResponse,
  groupRemoveMemberRequest,
  groupRemoveMemberResponse,
  groupInfoRequest,
  groupInfoResponse,
  groupReplyInviteRequest,
  groupReplyInviteResponse,
  groupListRequest,
  groupListResponse,
  groupInvitedRequest,
  groupInvitedResponse,
  notSet
}

/// Group service RPC message container
class Group extends $pb.GeneratedMessage {
  factory Group({
    GroupCreateRequest? groupCreateRequest,
    GroupCreateResponse? groupCreateResponse,
    GroupRenameRequest? groupRenameRequest,
    GroupRenameResponse? groupRenameResponse,
    GroupInviteMemberRequest? groupInviteMemberRequest,
    GroupInviteMemberResponse? groupInviteMemberResponse,
    GroupRemoveMemberRequest? groupRemoveMemberRequest,
    GroupRemoveMemberResponse? groupRemoveMemberResponse,
    GroupInfoRequest? groupInfoRequest,
    GroupInfo? groupInfoResponse,
    GroupReplyInviteRequest? groupReplyInviteRequest,
    GroupReplyInviteResponse? groupReplyInviteResponse,
    GroupListRequest? groupListRequest,
    GroupListResponse? groupListResponse,
    GroupInvitedRequest? groupInvitedRequest,
    GroupInvitedResponse? groupInvitedResponse,
  }) {
    final result = create();
    if (groupCreateRequest != null)
      result.groupCreateRequest = groupCreateRequest;
    if (groupCreateResponse != null)
      result.groupCreateResponse = groupCreateResponse;
    if (groupRenameRequest != null)
      result.groupRenameRequest = groupRenameRequest;
    if (groupRenameResponse != null)
      result.groupRenameResponse = groupRenameResponse;
    if (groupInviteMemberRequest != null)
      result.groupInviteMemberRequest = groupInviteMemberRequest;
    if (groupInviteMemberResponse != null)
      result.groupInviteMemberResponse = groupInviteMemberResponse;
    if (groupRemoveMemberRequest != null)
      result.groupRemoveMemberRequest = groupRemoveMemberRequest;
    if (groupRemoveMemberResponse != null)
      result.groupRemoveMemberResponse = groupRemoveMemberResponse;
    if (groupInfoRequest != null) result.groupInfoRequest = groupInfoRequest;
    if (groupInfoResponse != null) result.groupInfoResponse = groupInfoResponse;
    if (groupReplyInviteRequest != null)
      result.groupReplyInviteRequest = groupReplyInviteRequest;
    if (groupReplyInviteResponse != null)
      result.groupReplyInviteResponse = groupReplyInviteResponse;
    if (groupListRequest != null) result.groupListRequest = groupListRequest;
    if (groupListResponse != null) result.groupListResponse = groupListResponse;
    if (groupInvitedRequest != null)
      result.groupInvitedRequest = groupInvitedRequest;
    if (groupInvitedResponse != null)
      result.groupInvitedResponse = groupInvitedResponse;
    return result;
  }

  Group._();

  factory Group.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory Group.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, Group_Message> _Group_MessageByTag = {
    1: Group_Message.groupCreateRequest,
    2: Group_Message.groupCreateResponse,
    3: Group_Message.groupRenameRequest,
    4: Group_Message.groupRenameResponse,
    5: Group_Message.groupInviteMemberRequest,
    6: Group_Message.groupInviteMemberResponse,
    7: Group_Message.groupRemoveMemberRequest,
    8: Group_Message.groupRemoveMemberResponse,
    9: Group_Message.groupInfoRequest,
    10: Group_Message.groupInfoResponse,
    11: Group_Message.groupReplyInviteRequest,
    12: Group_Message.groupReplyInviteResponse,
    13: Group_Message.groupListRequest,
    14: Group_Message.groupListResponse,
    15: Group_Message.groupInvitedRequest,
    16: Group_Message.groupInvitedResponse,
    0: Group_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Group',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.group'),
      createEmptyInstance: create)
    ..oo(0, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16])
    ..aOM<GroupCreateRequest>(1, _omitFieldNames ? '' : 'groupCreateRequest',
        subBuilder: GroupCreateRequest.create)
    ..aOM<GroupCreateResponse>(2, _omitFieldNames ? '' : 'groupCreateResponse',
        subBuilder: GroupCreateResponse.create)
    ..aOM<GroupRenameRequest>(3, _omitFieldNames ? '' : 'groupRenameRequest',
        subBuilder: GroupRenameRequest.create)
    ..aOM<GroupRenameResponse>(4, _omitFieldNames ? '' : 'groupRenameResponse',
        subBuilder: GroupRenameResponse.create)
    ..aOM<GroupInviteMemberRequest>(
        5, _omitFieldNames ? '' : 'groupInviteMemberRequest',
        subBuilder: GroupInviteMemberRequest.create)
    ..aOM<GroupInviteMemberResponse>(
        6, _omitFieldNames ? '' : 'groupInviteMemberResponse',
        subBuilder: GroupInviteMemberResponse.create)
    ..aOM<GroupRemoveMemberRequest>(
        7, _omitFieldNames ? '' : 'groupRemoveMemberRequest',
        subBuilder: GroupRemoveMemberRequest.create)
    ..aOM<GroupRemoveMemberResponse>(
        8, _omitFieldNames ? '' : 'groupRemoveMemberResponse',
        subBuilder: GroupRemoveMemberResponse.create)
    ..aOM<GroupInfoRequest>(9, _omitFieldNames ? '' : 'groupInfoRequest',
        subBuilder: GroupInfoRequest.create)
    ..aOM<GroupInfo>(10, _omitFieldNames ? '' : 'groupInfoResponse',
        subBuilder: GroupInfo.create)
    ..aOM<GroupReplyInviteRequest>(
        11, _omitFieldNames ? '' : 'groupReplyInviteRequest',
        subBuilder: GroupReplyInviteRequest.create)
    ..aOM<GroupReplyInviteResponse>(
        12, _omitFieldNames ? '' : 'groupReplyInviteResponse',
        subBuilder: GroupReplyInviteResponse.create)
    ..aOM<GroupListRequest>(13, _omitFieldNames ? '' : 'groupListRequest',
        subBuilder: GroupListRequest.create)
    ..aOM<GroupListResponse>(14, _omitFieldNames ? '' : 'groupListResponse',
        subBuilder: GroupListResponse.create)
    ..aOM<GroupInvitedRequest>(15, _omitFieldNames ? '' : 'groupInvitedRequest',
        subBuilder: GroupInvitedRequest.create)
    ..aOM<GroupInvitedResponse>(
        16, _omitFieldNames ? '' : 'groupInvitedResponse',
        subBuilder: GroupInvitedResponse.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Group clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Group copyWith(void Function(Group) updates) =>
      super.copyWith((message) => updates(message as Group)) as Group;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Group create() => Group._();
  @$core.override
  Group createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static Group getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Group>(create);
  static Group? _defaultInstance;

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
  @$pb.TagNumber(12)
  @$pb.TagNumber(13)
  @$pb.TagNumber(14)
  @$pb.TagNumber(15)
  @$pb.TagNumber(16)
  Group_Message whichMessage() => _Group_MessageByTag[$_whichOneof(0)]!;
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
  @$pb.TagNumber(12)
  @$pb.TagNumber(13)
  @$pb.TagNumber(14)
  @$pb.TagNumber(15)
  @$pb.TagNumber(16)
  void clearMessage() => $_clearField($_whichOneof(0));

  /// group create request
  @$pb.TagNumber(1)
  GroupCreateRequest get groupCreateRequest => $_getN(0);
  @$pb.TagNumber(1)
  set groupCreateRequest(GroupCreateRequest value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasGroupCreateRequest() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupCreateRequest() => $_clearField(1);
  @$pb.TagNumber(1)
  GroupCreateRequest ensureGroupCreateRequest() => $_ensure(0);

  /// group create response
  @$pb.TagNumber(2)
  GroupCreateResponse get groupCreateResponse => $_getN(1);
  @$pb.TagNumber(2)
  set groupCreateResponse(GroupCreateResponse value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasGroupCreateResponse() => $_has(1);
  @$pb.TagNumber(2)
  void clearGroupCreateResponse() => $_clearField(2);
  @$pb.TagNumber(2)
  GroupCreateResponse ensureGroupCreateResponse() => $_ensure(1);

  /// group rename request
  @$pb.TagNumber(3)
  GroupRenameRequest get groupRenameRequest => $_getN(2);
  @$pb.TagNumber(3)
  set groupRenameRequest(GroupRenameRequest value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasGroupRenameRequest() => $_has(2);
  @$pb.TagNumber(3)
  void clearGroupRenameRequest() => $_clearField(3);
  @$pb.TagNumber(3)
  GroupRenameRequest ensureGroupRenameRequest() => $_ensure(2);

  /// group rename response
  @$pb.TagNumber(4)
  GroupRenameResponse get groupRenameResponse => $_getN(3);
  @$pb.TagNumber(4)
  set groupRenameResponse(GroupRenameResponse value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasGroupRenameResponse() => $_has(3);
  @$pb.TagNumber(4)
  void clearGroupRenameResponse() => $_clearField(4);
  @$pb.TagNumber(4)
  GroupRenameResponse ensureGroupRenameResponse() => $_ensure(3);

  /// group invite member request
  @$pb.TagNumber(5)
  GroupInviteMemberRequest get groupInviteMemberRequest => $_getN(4);
  @$pb.TagNumber(5)
  set groupInviteMemberRequest(GroupInviteMemberRequest value) =>
      $_setField(5, value);
  @$pb.TagNumber(5)
  $core.bool hasGroupInviteMemberRequest() => $_has(4);
  @$pb.TagNumber(5)
  void clearGroupInviteMemberRequest() => $_clearField(5);
  @$pb.TagNumber(5)
  GroupInviteMemberRequest ensureGroupInviteMemberRequest() => $_ensure(4);

  /// group invite member response
  @$pb.TagNumber(6)
  GroupInviteMemberResponse get groupInviteMemberResponse => $_getN(5);
  @$pb.TagNumber(6)
  set groupInviteMemberResponse(GroupInviteMemberResponse value) =>
      $_setField(6, value);
  @$pb.TagNumber(6)
  $core.bool hasGroupInviteMemberResponse() => $_has(5);
  @$pb.TagNumber(6)
  void clearGroupInviteMemberResponse() => $_clearField(6);
  @$pb.TagNumber(6)
  GroupInviteMemberResponse ensureGroupInviteMemberResponse() => $_ensure(5);

  /// group remove member request
  @$pb.TagNumber(7)
  GroupRemoveMemberRequest get groupRemoveMemberRequest => $_getN(6);
  @$pb.TagNumber(7)
  set groupRemoveMemberRequest(GroupRemoveMemberRequest value) =>
      $_setField(7, value);
  @$pb.TagNumber(7)
  $core.bool hasGroupRemoveMemberRequest() => $_has(6);
  @$pb.TagNumber(7)
  void clearGroupRemoveMemberRequest() => $_clearField(7);
  @$pb.TagNumber(7)
  GroupRemoveMemberRequest ensureGroupRemoveMemberRequest() => $_ensure(6);

  /// group remove member response
  @$pb.TagNumber(8)
  GroupRemoveMemberResponse get groupRemoveMemberResponse => $_getN(7);
  @$pb.TagNumber(8)
  set groupRemoveMemberResponse(GroupRemoveMemberResponse value) =>
      $_setField(8, value);
  @$pb.TagNumber(8)
  $core.bool hasGroupRemoveMemberResponse() => $_has(7);
  @$pb.TagNumber(8)
  void clearGroupRemoveMemberResponse() => $_clearField(8);
  @$pb.TagNumber(8)
  GroupRemoveMemberResponse ensureGroupRemoveMemberResponse() => $_ensure(7);

  /// group info request
  @$pb.TagNumber(9)
  GroupInfoRequest get groupInfoRequest => $_getN(8);
  @$pb.TagNumber(9)
  set groupInfoRequest(GroupInfoRequest value) => $_setField(9, value);
  @$pb.TagNumber(9)
  $core.bool hasGroupInfoRequest() => $_has(8);
  @$pb.TagNumber(9)
  void clearGroupInfoRequest() => $_clearField(9);
  @$pb.TagNumber(9)
  GroupInfoRequest ensureGroupInfoRequest() => $_ensure(8);

  /// group info response
  @$pb.TagNumber(10)
  GroupInfo get groupInfoResponse => $_getN(9);
  @$pb.TagNumber(10)
  set groupInfoResponse(GroupInfo value) => $_setField(10, value);
  @$pb.TagNumber(10)
  $core.bool hasGroupInfoResponse() => $_has(9);
  @$pb.TagNumber(10)
  void clearGroupInfoResponse() => $_clearField(10);
  @$pb.TagNumber(10)
  GroupInfo ensureGroupInfoResponse() => $_ensure(9);

  /// group reply invite request
  @$pb.TagNumber(11)
  GroupReplyInviteRequest get groupReplyInviteRequest => $_getN(10);
  @$pb.TagNumber(11)
  set groupReplyInviteRequest(GroupReplyInviteRequest value) =>
      $_setField(11, value);
  @$pb.TagNumber(11)
  $core.bool hasGroupReplyInviteRequest() => $_has(10);
  @$pb.TagNumber(11)
  void clearGroupReplyInviteRequest() => $_clearField(11);
  @$pb.TagNumber(11)
  GroupReplyInviteRequest ensureGroupReplyInviteRequest() => $_ensure(10);

  /// group reply invite response
  @$pb.TagNumber(12)
  GroupReplyInviteResponse get groupReplyInviteResponse => $_getN(11);
  @$pb.TagNumber(12)
  set groupReplyInviteResponse(GroupReplyInviteResponse value) =>
      $_setField(12, value);
  @$pb.TagNumber(12)
  $core.bool hasGroupReplyInviteResponse() => $_has(11);
  @$pb.TagNumber(12)
  void clearGroupReplyInviteResponse() => $_clearField(12);
  @$pb.TagNumber(12)
  GroupReplyInviteResponse ensureGroupReplyInviteResponse() => $_ensure(11);

  /// group list request
  @$pb.TagNumber(13)
  GroupListRequest get groupListRequest => $_getN(12);
  @$pb.TagNumber(13)
  set groupListRequest(GroupListRequest value) => $_setField(13, value);
  @$pb.TagNumber(13)
  $core.bool hasGroupListRequest() => $_has(12);
  @$pb.TagNumber(13)
  void clearGroupListRequest() => $_clearField(13);
  @$pb.TagNumber(13)
  GroupListRequest ensureGroupListRequest() => $_ensure(12);

  /// group list response
  @$pb.TagNumber(14)
  GroupListResponse get groupListResponse => $_getN(13);
  @$pb.TagNumber(14)
  set groupListResponse(GroupListResponse value) => $_setField(14, value);
  @$pb.TagNumber(14)
  $core.bool hasGroupListResponse() => $_has(13);
  @$pb.TagNumber(14)
  void clearGroupListResponse() => $_clearField(14);
  @$pb.TagNumber(14)
  GroupListResponse ensureGroupListResponse() => $_ensure(13);

  /// group invited
  @$pb.TagNumber(15)
  GroupInvitedRequest get groupInvitedRequest => $_getN(14);
  @$pb.TagNumber(15)
  set groupInvitedRequest(GroupInvitedRequest value) => $_setField(15, value);
  @$pb.TagNumber(15)
  $core.bool hasGroupInvitedRequest() => $_has(14);
  @$pb.TagNumber(15)
  void clearGroupInvitedRequest() => $_clearField(15);
  @$pb.TagNumber(15)
  GroupInvitedRequest ensureGroupInvitedRequest() => $_ensure(14);

  /// group invited response
  @$pb.TagNumber(16)
  GroupInvitedResponse get groupInvitedResponse => $_getN(15);
  @$pb.TagNumber(16)
  set groupInvitedResponse(GroupInvitedResponse value) => $_setField(16, value);
  @$pb.TagNumber(16)
  $core.bool hasGroupInvitedResponse() => $_has(15);
  @$pb.TagNumber(16)
  void clearGroupInvitedResponse() => $_clearField(16);
  @$pb.TagNumber(16)
  GroupInvitedResponse ensureGroupInvitedResponse() => $_ensure(15);
}

/// Group Result
class GroupResult extends $pb.GeneratedMessage {
  factory GroupResult({
    $core.bool? status,
    $core.String? message,
  }) {
    final result = create();
    if (status != null) result.status = status;
    if (message != null) result.message = message;
    return result;
  }

  GroupResult._();

  factory GroupResult.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory GroupResult.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GroupResult',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.group'),
      createEmptyInstance: create)
    ..aOB(1, _omitFieldNames ? '' : 'status')
    ..aOS(2, _omitFieldNames ? '' : 'message')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupResult clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupResult copyWith(void Function(GroupResult) updates) =>
      super.copyWith((message) => updates(message as GroupResult))
          as GroupResult;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GroupResult create() => GroupResult._();
  @$core.override
  GroupResult createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static GroupResult getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GroupResult>(create);
  static GroupResult? _defaultInstance;

  /// status
  ///
  /// true = success
  /// false = an error happened
  ///
  /// if the result is false, the message will
  /// contain the error message.
  @$pb.TagNumber(1)
  $core.bool get status => $_getBF(0);
  @$pb.TagNumber(1)
  set status($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasStatus() => $_has(0);
  @$pb.TagNumber(1)
  void clearStatus() => $_clearField(1);

  /// message
  @$pb.TagNumber(2)
  $core.String get message => $_getSZ(1);
  @$pb.TagNumber(2)
  set message($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasMessage() => $_has(1);
  @$pb.TagNumber(2)
  void clearMessage() => $_clearField(2);
}

/// Create New Group
class GroupCreateRequest extends $pb.GeneratedMessage {
  factory GroupCreateRequest({
    $core.String? groupName,
  }) {
    final result = create();
    if (groupName != null) result.groupName = groupName;
    return result;
  }

  GroupCreateRequest._();

  factory GroupCreateRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory GroupCreateRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GroupCreateRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.group'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'groupName')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupCreateRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupCreateRequest copyWith(void Function(GroupCreateRequest) updates) =>
      super.copyWith((message) => updates(message as GroupCreateRequest))
          as GroupCreateRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GroupCreateRequest create() => GroupCreateRequest._();
  @$core.override
  GroupCreateRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static GroupCreateRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GroupCreateRequest>(create);
  static GroupCreateRequest? _defaultInstance;

  /// group name
  @$pb.TagNumber(1)
  $core.String get groupName => $_getSZ(0);
  @$pb.TagNumber(1)
  set groupName($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasGroupName() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupName() => $_clearField(1);
}

/// Group creating response
class GroupCreateResponse extends $pb.GeneratedMessage {
  factory GroupCreateResponse({
    $core.List<$core.int>? groupId,
    GroupResult? result,
  }) {
    final result$ = create();
    if (groupId != null) result$.groupId = groupId;
    if (result != null) result$.result = result;
    return result$;
  }

  GroupCreateResponse._();

  factory GroupCreateResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory GroupCreateResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GroupCreateResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.group'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'groupId', $pb.PbFieldType.OY)
    ..aOM<GroupResult>(2, _omitFieldNames ? '' : 'result',
        subBuilder: GroupResult.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupCreateResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupCreateResponse copyWith(void Function(GroupCreateResponse) updates) =>
      super.copyWith((message) => updates(message as GroupCreateResponse))
          as GroupCreateResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GroupCreateResponse create() => GroupCreateResponse._();
  @$core.override
  GroupCreateResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static GroupCreateResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GroupCreateResponse>(create);
  static GroupCreateResponse? _defaultInstance;

  /// group id
  @$pb.TagNumber(1)
  $core.List<$core.int> get groupId => $_getN(0);
  @$pb.TagNumber(1)
  set groupId($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => $_clearField(1);

  /// result
  @$pb.TagNumber(2)
  GroupResult get result => $_getN(1);
  @$pb.TagNumber(2)
  set result(GroupResult value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasResult() => $_has(1);
  @$pb.TagNumber(2)
  void clearResult() => $_clearField(2);
  @$pb.TagNumber(2)
  GroupResult ensureResult() => $_ensure(1);
}

/// Group rename request
class GroupRenameRequest extends $pb.GeneratedMessage {
  factory GroupRenameRequest({
    $core.List<$core.int>? groupId,
    $core.String? groupName,
  }) {
    final result = create();
    if (groupId != null) result.groupId = groupId;
    if (groupName != null) result.groupName = groupName;
    return result;
  }

  GroupRenameRequest._();

  factory GroupRenameRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory GroupRenameRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GroupRenameRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.group'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'groupId', $pb.PbFieldType.OY)
    ..aOS(2, _omitFieldNames ? '' : 'groupName')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupRenameRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupRenameRequest copyWith(void Function(GroupRenameRequest) updates) =>
      super.copyWith((message) => updates(message as GroupRenameRequest))
          as GroupRenameRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GroupRenameRequest create() => GroupRenameRequest._();
  @$core.override
  GroupRenameRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static GroupRenameRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GroupRenameRequest>(create);
  static GroupRenameRequest? _defaultInstance;

  /// group id
  @$pb.TagNumber(1)
  $core.List<$core.int> get groupId => $_getN(0);
  @$pb.TagNumber(1)
  set groupId($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => $_clearField(1);

  /// group name
  @$pb.TagNumber(2)
  $core.String get groupName => $_getSZ(1);
  @$pb.TagNumber(2)
  set groupName($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasGroupName() => $_has(1);
  @$pb.TagNumber(2)
  void clearGroupName() => $_clearField(2);
}

/// Group rename response
class GroupRenameResponse extends $pb.GeneratedMessage {
  factory GroupRenameResponse({
    $core.List<$core.int>? groupId,
    $core.String? groupName,
    GroupResult? result,
  }) {
    final result$ = create();
    if (groupId != null) result$.groupId = groupId;
    if (groupName != null) result$.groupName = groupName;
    if (result != null) result$.result = result;
    return result$;
  }

  GroupRenameResponse._();

  factory GroupRenameResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory GroupRenameResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GroupRenameResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.group'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'groupId', $pb.PbFieldType.OY)
    ..aOS(2, _omitFieldNames ? '' : 'groupName')
    ..aOM<GroupResult>(3, _omitFieldNames ? '' : 'result',
        subBuilder: GroupResult.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupRenameResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupRenameResponse copyWith(void Function(GroupRenameResponse) updates) =>
      super.copyWith((message) => updates(message as GroupRenameResponse))
          as GroupRenameResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GroupRenameResponse create() => GroupRenameResponse._();
  @$core.override
  GroupRenameResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static GroupRenameResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GroupRenameResponse>(create);
  static GroupRenameResponse? _defaultInstance;

  /// group id
  @$pb.TagNumber(1)
  $core.List<$core.int> get groupId => $_getN(0);
  @$pb.TagNumber(1)
  set groupId($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => $_clearField(1);

  /// group name
  @$pb.TagNumber(2)
  $core.String get groupName => $_getSZ(1);
  @$pb.TagNumber(2)
  set groupName($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasGroupName() => $_has(1);
  @$pb.TagNumber(2)
  void clearGroupName() => $_clearField(2);

  /// result
  @$pb.TagNumber(3)
  GroupResult get result => $_getN(2);
  @$pb.TagNumber(3)
  set result(GroupResult value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasResult() => $_has(2);
  @$pb.TagNumber(3)
  void clearResult() => $_clearField(3);
  @$pb.TagNumber(3)
  GroupResult ensureResult() => $_ensure(2);
}

/// Invite member
class GroupInviteMemberRequest extends $pb.GeneratedMessage {
  factory GroupInviteMemberRequest({
    $core.List<$core.int>? groupId,
    $core.List<$core.int>? userId,
  }) {
    final result = create();
    if (groupId != null) result.groupId = groupId;
    if (userId != null) result.userId = userId;
    return result;
  }

  GroupInviteMemberRequest._();

  factory GroupInviteMemberRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory GroupInviteMemberRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GroupInviteMemberRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.group'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'groupId', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(
        2, _omitFieldNames ? '' : 'userId', $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupInviteMemberRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupInviteMemberRequest copyWith(
          void Function(GroupInviteMemberRequest) updates) =>
      super.copyWith((message) => updates(message as GroupInviteMemberRequest))
          as GroupInviteMemberRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GroupInviteMemberRequest create() => GroupInviteMemberRequest._();
  @$core.override
  GroupInviteMemberRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static GroupInviteMemberRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GroupInviteMemberRequest>(create);
  static GroupInviteMemberRequest? _defaultInstance;

  /// group id
  @$pb.TagNumber(1)
  $core.List<$core.int> get groupId => $_getN(0);
  @$pb.TagNumber(1)
  set groupId($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => $_clearField(1);

  /// user id
  @$pb.TagNumber(2)
  $core.List<$core.int> get userId => $_getN(1);
  @$pb.TagNumber(2)
  set userId($core.List<$core.int> value) => $_setBytes(1, value);
  @$pb.TagNumber(2)
  $core.bool hasUserId() => $_has(1);
  @$pb.TagNumber(2)
  void clearUserId() => $_clearField(2);
}

/// Invite member response
class GroupInviteMemberResponse extends $pb.GeneratedMessage {
  factory GroupInviteMemberResponse({
    $core.List<$core.int>? groupId,
    $core.List<$core.int>? userId,
    GroupResult? result,
  }) {
    final result$ = create();
    if (groupId != null) result$.groupId = groupId;
    if (userId != null) result$.userId = userId;
    if (result != null) result$.result = result;
    return result$;
  }

  GroupInviteMemberResponse._();

  factory GroupInviteMemberResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory GroupInviteMemberResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GroupInviteMemberResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.group'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'groupId', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(
        2, _omitFieldNames ? '' : 'userId', $pb.PbFieldType.OY)
    ..aOM<GroupResult>(3, _omitFieldNames ? '' : 'result',
        subBuilder: GroupResult.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupInviteMemberResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupInviteMemberResponse copyWith(
          void Function(GroupInviteMemberResponse) updates) =>
      super.copyWith((message) => updates(message as GroupInviteMemberResponse))
          as GroupInviteMemberResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GroupInviteMemberResponse create() => GroupInviteMemberResponse._();
  @$core.override
  GroupInviteMemberResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static GroupInviteMemberResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GroupInviteMemberResponse>(create);
  static GroupInviteMemberResponse? _defaultInstance;

  /// group id
  @$pb.TagNumber(1)
  $core.List<$core.int> get groupId => $_getN(0);
  @$pb.TagNumber(1)
  set groupId($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => $_clearField(1);

  /// user id
  @$pb.TagNumber(2)
  $core.List<$core.int> get userId => $_getN(1);
  @$pb.TagNumber(2)
  set userId($core.List<$core.int> value) => $_setBytes(1, value);
  @$pb.TagNumber(2)
  $core.bool hasUserId() => $_has(1);
  @$pb.TagNumber(2)
  void clearUserId() => $_clearField(2);

  /// result
  @$pb.TagNumber(3)
  GroupResult get result => $_getN(2);
  @$pb.TagNumber(3)
  set result(GroupResult value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasResult() => $_has(2);
  @$pb.TagNumber(3)
  void clearResult() => $_clearField(3);
  @$pb.TagNumber(3)
  GroupResult ensureResult() => $_ensure(2);
}

/// Reply Invite
class GroupReplyInviteRequest extends $pb.GeneratedMessage {
  factory GroupReplyInviteRequest({
    $core.List<$core.int>? groupId,
    $core.bool? accept,
  }) {
    final result = create();
    if (groupId != null) result.groupId = groupId;
    if (accept != null) result.accept = accept;
    return result;
  }

  GroupReplyInviteRequest._();

  factory GroupReplyInviteRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory GroupReplyInviteRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GroupReplyInviteRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.group'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'groupId', $pb.PbFieldType.OY)
    ..aOB(3, _omitFieldNames ? '' : 'accept')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupReplyInviteRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupReplyInviteRequest copyWith(
          void Function(GroupReplyInviteRequest) updates) =>
      super.copyWith((message) => updates(message as GroupReplyInviteRequest))
          as GroupReplyInviteRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GroupReplyInviteRequest create() => GroupReplyInviteRequest._();
  @$core.override
  GroupReplyInviteRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static GroupReplyInviteRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GroupReplyInviteRequest>(create);
  static GroupReplyInviteRequest? _defaultInstance;

  /// group id
  @$pb.TagNumber(1)
  $core.List<$core.int> get groupId => $_getN(0);
  @$pb.TagNumber(1)
  set groupId($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => $_clearField(1);

  /// accept
  @$pb.TagNumber(3)
  $core.bool get accept => $_getBF(1);
  @$pb.TagNumber(3)
  set accept($core.bool value) => $_setBool(1, value);
  @$pb.TagNumber(3)
  $core.bool hasAccept() => $_has(1);
  @$pb.TagNumber(3)
  void clearAccept() => $_clearField(3);
}

/// Reply Invite Response
class GroupReplyInviteResponse extends $pb.GeneratedMessage {
  factory GroupReplyInviteResponse({
    $core.List<$core.int>? groupId,
    GroupResult? result,
  }) {
    final result$ = create();
    if (groupId != null) result$.groupId = groupId;
    if (result != null) result$.result = result;
    return result$;
  }

  GroupReplyInviteResponse._();

  factory GroupReplyInviteResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory GroupReplyInviteResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GroupReplyInviteResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.group'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'groupId', $pb.PbFieldType.OY)
    ..aOM<GroupResult>(3, _omitFieldNames ? '' : 'result',
        subBuilder: GroupResult.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupReplyInviteResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupReplyInviteResponse copyWith(
          void Function(GroupReplyInviteResponse) updates) =>
      super.copyWith((message) => updates(message as GroupReplyInviteResponse))
          as GroupReplyInviteResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GroupReplyInviteResponse create() => GroupReplyInviteResponse._();
  @$core.override
  GroupReplyInviteResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static GroupReplyInviteResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GroupReplyInviteResponse>(create);
  static GroupReplyInviteResponse? _defaultInstance;

  /// group id
  @$pb.TagNumber(1)
  $core.List<$core.int> get groupId => $_getN(0);
  @$pb.TagNumber(1)
  set groupId($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => $_clearField(1);

  /// result
  @$pb.TagNumber(3)
  GroupResult get result => $_getN(1);
  @$pb.TagNumber(3)
  set result(GroupResult value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasResult() => $_has(1);
  @$pb.TagNumber(3)
  void clearResult() => $_clearField(3);
  @$pb.TagNumber(3)
  GroupResult ensureResult() => $_ensure(1);
}

/// Remove member
class GroupRemoveMemberRequest extends $pb.GeneratedMessage {
  factory GroupRemoveMemberRequest({
    $core.List<$core.int>? groupId,
    $core.List<$core.int>? userId,
  }) {
    final result = create();
    if (groupId != null) result.groupId = groupId;
    if (userId != null) result.userId = userId;
    return result;
  }

  GroupRemoveMemberRequest._();

  factory GroupRemoveMemberRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory GroupRemoveMemberRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GroupRemoveMemberRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.group'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'groupId', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(
        2, _omitFieldNames ? '' : 'userId', $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupRemoveMemberRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupRemoveMemberRequest copyWith(
          void Function(GroupRemoveMemberRequest) updates) =>
      super.copyWith((message) => updates(message as GroupRemoveMemberRequest))
          as GroupRemoveMemberRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GroupRemoveMemberRequest create() => GroupRemoveMemberRequest._();
  @$core.override
  GroupRemoveMemberRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static GroupRemoveMemberRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GroupRemoveMemberRequest>(create);
  static GroupRemoveMemberRequest? _defaultInstance;

  /// group id
  @$pb.TagNumber(1)
  $core.List<$core.int> get groupId => $_getN(0);
  @$pb.TagNumber(1)
  set groupId($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => $_clearField(1);

  /// user id
  @$pb.TagNumber(2)
  $core.List<$core.int> get userId => $_getN(1);
  @$pb.TagNumber(2)
  set userId($core.List<$core.int> value) => $_setBytes(1, value);
  @$pb.TagNumber(2)
  $core.bool hasUserId() => $_has(1);
  @$pb.TagNumber(2)
  void clearUserId() => $_clearField(2);
}

/// Remove member
class GroupRemoveMemberResponse extends $pb.GeneratedMessage {
  factory GroupRemoveMemberResponse({
    $core.List<$core.int>? groupId,
    $core.List<$core.int>? userId,
    GroupResult? result,
  }) {
    final result$ = create();
    if (groupId != null) result$.groupId = groupId;
    if (userId != null) result$.userId = userId;
    if (result != null) result$.result = result;
    return result$;
  }

  GroupRemoveMemberResponse._();

  factory GroupRemoveMemberResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory GroupRemoveMemberResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GroupRemoveMemberResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.group'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'groupId', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(
        2, _omitFieldNames ? '' : 'userId', $pb.PbFieldType.OY)
    ..aOM<GroupResult>(3, _omitFieldNames ? '' : 'result',
        subBuilder: GroupResult.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupRemoveMemberResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupRemoveMemberResponse copyWith(
          void Function(GroupRemoveMemberResponse) updates) =>
      super.copyWith((message) => updates(message as GroupRemoveMemberResponse))
          as GroupRemoveMemberResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GroupRemoveMemberResponse create() => GroupRemoveMemberResponse._();
  @$core.override
  GroupRemoveMemberResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static GroupRemoveMemberResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GroupRemoveMemberResponse>(create);
  static GroupRemoveMemberResponse? _defaultInstance;

  /// group id
  @$pb.TagNumber(1)
  $core.List<$core.int> get groupId => $_getN(0);
  @$pb.TagNumber(1)
  set groupId($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => $_clearField(1);

  /// user id
  @$pb.TagNumber(2)
  $core.List<$core.int> get userId => $_getN(1);
  @$pb.TagNumber(2)
  set userId($core.List<$core.int> value) => $_setBytes(1, value);
  @$pb.TagNumber(2)
  $core.bool hasUserId() => $_has(1);
  @$pb.TagNumber(2)
  void clearUserId() => $_clearField(2);

  /// result
  @$pb.TagNumber(3)
  GroupResult get result => $_getN(2);
  @$pb.TagNumber(3)
  set result(GroupResult value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasResult() => $_has(2);
  @$pb.TagNumber(3)
  void clearResult() => $_clearField(3);
  @$pb.TagNumber(3)
  GroupResult ensureResult() => $_ensure(2);
}

/// Group info request
class GroupInfoRequest extends $pb.GeneratedMessage {
  factory GroupInfoRequest({
    $core.List<$core.int>? groupId,
  }) {
    final result = create();
    if (groupId != null) result.groupId = groupId;
    return result;
  }

  GroupInfoRequest._();

  factory GroupInfoRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory GroupInfoRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GroupInfoRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.group'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'groupId', $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupInfoRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupInfoRequest copyWith(void Function(GroupInfoRequest) updates) =>
      super.copyWith((message) => updates(message as GroupInfoRequest))
          as GroupInfoRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GroupInfoRequest create() => GroupInfoRequest._();
  @$core.override
  GroupInfoRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static GroupInfoRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GroupInfoRequest>(create);
  static GroupInfoRequest? _defaultInstance;

  /// group id
  @$pb.TagNumber(1)
  $core.List<$core.int> get groupId => $_getN(0);
  @$pb.TagNumber(1)
  set groupId($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => $_clearField(1);
}

/// Group member response
class GroupMember extends $pb.GeneratedMessage {
  factory GroupMember({
    $core.List<$core.int>? userId,
    GroupMemberRole? role,
    $fixnum.Int64? joinedAt,
    GroupMemberState? state,
    $core.int? lastMessageIndex,
  }) {
    final result = create();
    if (userId != null) result.userId = userId;
    if (role != null) result.role = role;
    if (joinedAt != null) result.joinedAt = joinedAt;
    if (state != null) result.state = state;
    if (lastMessageIndex != null) result.lastMessageIndex = lastMessageIndex;
    return result;
  }

  GroupMember._();

  factory GroupMember.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory GroupMember.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GroupMember',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.group'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'userId', $pb.PbFieldType.OY)
    ..aE<GroupMemberRole>(2, _omitFieldNames ? '' : 'role',
        enumValues: GroupMemberRole.values)
    ..a<$fixnum.Int64>(
        3, _omitFieldNames ? '' : 'joinedAt', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..aE<GroupMemberState>(4, _omitFieldNames ? '' : 'state',
        enumValues: GroupMemberState.values)
    ..aI(5, _omitFieldNames ? '' : 'lastMessageIndex',
        fieldType: $pb.PbFieldType.OU3)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupMember clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupMember copyWith(void Function(GroupMember) updates) =>
      super.copyWith((message) => updates(message as GroupMember))
          as GroupMember;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GroupMember create() => GroupMember._();
  @$core.override
  GroupMember createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static GroupMember getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GroupMember>(create);
  static GroupMember? _defaultInstance;

  /// user id
  @$pb.TagNumber(1)
  $core.List<$core.int> get userId => $_getN(0);
  @$pb.TagNumber(1)
  set userId($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasUserId() => $_has(0);
  @$pb.TagNumber(1)
  void clearUserId() => $_clearField(1);

  /// role
  @$pb.TagNumber(2)
  GroupMemberRole get role => $_getN(1);
  @$pb.TagNumber(2)
  set role(GroupMemberRole value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasRole() => $_has(1);
  @$pb.TagNumber(2)
  void clearRole() => $_clearField(2);

  /// joined at
  @$pb.TagNumber(3)
  $fixnum.Int64 get joinedAt => $_getI64(2);
  @$pb.TagNumber(3)
  set joinedAt($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasJoinedAt() => $_has(2);
  @$pb.TagNumber(3)
  void clearJoinedAt() => $_clearField(3);

  /// state
  @$pb.TagNumber(4)
  GroupMemberState get state => $_getN(3);
  @$pb.TagNumber(4)
  set state(GroupMemberState value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasState() => $_has(3);
  @$pb.TagNumber(4)
  void clearState() => $_clearField(4);

  /// last message index
  @$pb.TagNumber(5)
  $core.int get lastMessageIndex => $_getIZ(4);
  @$pb.TagNumber(5)
  set lastMessageIndex($core.int value) => $_setUnsignedInt32(4, value);
  @$pb.TagNumber(5)
  $core.bool hasLastMessageIndex() => $_has(4);
  @$pb.TagNumber(5)
  void clearLastMessageIndex() => $_clearField(5);
}

/// Group info response
class GroupInfo extends $pb.GeneratedMessage {
  factory GroupInfo({
    $core.List<$core.int>? groupId,
    $core.String? groupName,
    $fixnum.Int64? createdAt,
    GroupStatus? status,
    $core.int? revision,
    $core.bool? isDirectChat,
    $core.Iterable<GroupMember>? members,
    $core.int? unreadMessages,
    $fixnum.Int64? lastMessageAt,
    $core.List<$core.int>? lastMessage,
    $core.List<$core.int>? lastMessageSenderId,
  }) {
    final result = create();
    if (groupId != null) result.groupId = groupId;
    if (groupName != null) result.groupName = groupName;
    if (createdAt != null) result.createdAt = createdAt;
    if (status != null) result.status = status;
    if (revision != null) result.revision = revision;
    if (isDirectChat != null) result.isDirectChat = isDirectChat;
    if (members != null) result.members.addAll(members);
    if (unreadMessages != null) result.unreadMessages = unreadMessages;
    if (lastMessageAt != null) result.lastMessageAt = lastMessageAt;
    if (lastMessage != null) result.lastMessage = lastMessage;
    if (lastMessageSenderId != null)
      result.lastMessageSenderId = lastMessageSenderId;
    return result;
  }

  GroupInfo._();

  factory GroupInfo.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory GroupInfo.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GroupInfo',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.group'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'groupId', $pb.PbFieldType.OY)
    ..aOS(2, _omitFieldNames ? '' : 'groupName')
    ..a<$fixnum.Int64>(
        3, _omitFieldNames ? '' : 'createdAt', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..aE<GroupStatus>(4, _omitFieldNames ? '' : 'status',
        enumValues: GroupStatus.values)
    ..aI(5, _omitFieldNames ? '' : 'revision', fieldType: $pb.PbFieldType.OU3)
    ..aOB(6, _omitFieldNames ? '' : 'isDirectChat')
    ..pPM<GroupMember>(7, _omitFieldNames ? '' : 'members',
        subBuilder: GroupMember.create)
    ..aI(8, _omitFieldNames ? '' : 'unreadMessages',
        fieldType: $pb.PbFieldType.OU3)
    ..a<$fixnum.Int64>(
        9, _omitFieldNames ? '' : 'lastMessageAt', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.List<$core.int>>(
        10, _omitFieldNames ? '' : 'lastMessage', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(
        11, _omitFieldNames ? '' : 'lastMessageSenderId', $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupInfo clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupInfo copyWith(void Function(GroupInfo) updates) =>
      super.copyWith((message) => updates(message as GroupInfo)) as GroupInfo;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GroupInfo create() => GroupInfo._();
  @$core.override
  GroupInfo createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static GroupInfo getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupInfo>(create);
  static GroupInfo? _defaultInstance;

  /// group id
  @$pb.TagNumber(1)
  $core.List<$core.int> get groupId => $_getN(0);
  @$pb.TagNumber(1)
  set groupId($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => $_clearField(1);

  /// group name
  @$pb.TagNumber(2)
  $core.String get groupName => $_getSZ(1);
  @$pb.TagNumber(2)
  set groupName($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasGroupName() => $_has(1);
  @$pb.TagNumber(2)
  void clearGroupName() => $_clearField(2);

  /// created at
  @$pb.TagNumber(3)
  $fixnum.Int64 get createdAt => $_getI64(2);
  @$pb.TagNumber(3)
  set createdAt($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasCreatedAt() => $_has(2);
  @$pb.TagNumber(3)
  void clearCreatedAt() => $_clearField(3);

  /// group status
  @$pb.TagNumber(4)
  GroupStatus get status => $_getN(3);
  @$pb.TagNumber(4)
  set status(GroupStatus value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasStatus() => $_has(3);
  @$pb.TagNumber(4)
  void clearStatus() => $_clearField(4);

  /// group revision number
  @$pb.TagNumber(5)
  $core.int get revision => $_getIZ(4);
  @$pb.TagNumber(5)
  set revision($core.int value) => $_setUnsignedInt32(4, value);
  @$pb.TagNumber(5)
  $core.bool hasRevision() => $_has(4);
  @$pb.TagNumber(5)
  void clearRevision() => $_clearField(5);

  /// is direct chat
  @$pb.TagNumber(6)
  $core.bool get isDirectChat => $_getBF(5);
  @$pb.TagNumber(6)
  set isDirectChat($core.bool value) => $_setBool(5, value);
  @$pb.TagNumber(6)
  $core.bool hasIsDirectChat() => $_has(5);
  @$pb.TagNumber(6)
  void clearIsDirectChat() => $_clearField(6);

  /// members
  @$pb.TagNumber(7)
  $pb.PbList<GroupMember> get members => $_getList(6);

  /// unread messages
  @$pb.TagNumber(8)
  $core.int get unreadMessages => $_getIZ(7);
  @$pb.TagNumber(8)
  set unreadMessages($core.int value) => $_setUnsignedInt32(7, value);
  @$pb.TagNumber(8)
  $core.bool hasUnreadMessages() => $_has(7);
  @$pb.TagNumber(8)
  void clearUnreadMessages() => $_clearField(8);

  /// time when last message was sent
  @$pb.TagNumber(9)
  $fixnum.Int64 get lastMessageAt => $_getI64(8);
  @$pb.TagNumber(9)
  set lastMessageAt($fixnum.Int64 value) => $_setInt64(8, value);
  @$pb.TagNumber(9)
  $core.bool hasLastMessageAt() => $_has(8);
  @$pb.TagNumber(9)
  void clearLastMessageAt() => $_clearField(9);

  /// content type
  @$pb.TagNumber(10)
  $core.List<$core.int> get lastMessage => $_getN(9);
  @$pb.TagNumber(10)
  set lastMessage($core.List<$core.int> value) => $_setBytes(9, value);
  @$pb.TagNumber(10)
  $core.bool hasLastMessage() => $_has(9);
  @$pb.TagNumber(10)
  void clearLastMessage() => $_clearField(10);

  /// sender of the last message
  @$pb.TagNumber(11)
  $core.List<$core.int> get lastMessageSenderId => $_getN(10);
  @$pb.TagNumber(11)
  set lastMessageSenderId($core.List<$core.int> value) => $_setBytes(10, value);
  @$pb.TagNumber(11)
  $core.bool hasLastMessageSenderId() => $_has(10);
  @$pb.TagNumber(11)
  void clearLastMessageSenderId() => $_clearField(11);
}

/// Group list request
class GroupListRequest extends $pb.GeneratedMessage {
  factory GroupListRequest() => create();

  GroupListRequest._();

  factory GroupListRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory GroupListRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GroupListRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.group'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupListRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupListRequest copyWith(void Function(GroupListRequest) updates) =>
      super.copyWith((message) => updates(message as GroupListRequest))
          as GroupListRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GroupListRequest create() => GroupListRequest._();
  @$core.override
  GroupListRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static GroupListRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GroupListRequest>(create);
  static GroupListRequest? _defaultInstance;
}

/// Group info response
class GroupListResponse extends $pb.GeneratedMessage {
  factory GroupListResponse({
    $core.Iterable<GroupInfo>? groups,
  }) {
    final result = create();
    if (groups != null) result.groups.addAll(groups);
    return result;
  }

  GroupListResponse._();

  factory GroupListResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory GroupListResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GroupListResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.group'),
      createEmptyInstance: create)
    ..pPM<GroupInfo>(1, _omitFieldNames ? '' : 'groups',
        subBuilder: GroupInfo.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupListResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupListResponse copyWith(void Function(GroupListResponse) updates) =>
      super.copyWith((message) => updates(message as GroupListResponse))
          as GroupListResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GroupListResponse create() => GroupListResponse._();
  @$core.override
  GroupListResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static GroupListResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GroupListResponse>(create);
  static GroupListResponse? _defaultInstance;

  /// group list
  @$pb.TagNumber(1)
  $pb.PbList<GroupInfo> get groups => $_getList(0);
}

class GroupInvited extends $pb.GeneratedMessage {
  factory GroupInvited({
    $core.List<$core.int>? senderId,
    $fixnum.Int64? receivedAt,
    GroupInfo? group,
  }) {
    final result = create();
    if (senderId != null) result.senderId = senderId;
    if (receivedAt != null) result.receivedAt = receivedAt;
    if (group != null) result.group = group;
    return result;
  }

  GroupInvited._();

  factory GroupInvited.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory GroupInvited.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GroupInvited',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.group'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'senderId', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(
        2, _omitFieldNames ? '' : 'receivedAt', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOM<GroupInfo>(3, _omitFieldNames ? '' : 'group',
        subBuilder: GroupInfo.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupInvited clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupInvited copyWith(void Function(GroupInvited) updates) =>
      super.copyWith((message) => updates(message as GroupInvited))
          as GroupInvited;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GroupInvited create() => GroupInvited._();
  @$core.override
  GroupInvited createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static GroupInvited getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GroupInvited>(create);
  static GroupInvited? _defaultInstance;

  /// sender id
  @$pb.TagNumber(1)
  $core.List<$core.int> get senderId => $_getN(0);
  @$pb.TagNumber(1)
  set senderId($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSenderId() => $_has(0);
  @$pb.TagNumber(1)
  void clearSenderId() => $_clearField(1);

  /// received at
  @$pb.TagNumber(2)
  $fixnum.Int64 get receivedAt => $_getI64(1);
  @$pb.TagNumber(2)
  set receivedAt($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasReceivedAt() => $_has(1);
  @$pb.TagNumber(2)
  void clearReceivedAt() => $_clearField(2);

  /// group info
  @$pb.TagNumber(3)
  GroupInfo get group => $_getN(2);
  @$pb.TagNumber(3)
  set group(GroupInfo value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasGroup() => $_has(2);
  @$pb.TagNumber(3)
  void clearGroup() => $_clearField(3);
  @$pb.TagNumber(3)
  GroupInfo ensureGroup() => $_ensure(2);
}

/// Group list request
class GroupInvitedRequest extends $pb.GeneratedMessage {
  factory GroupInvitedRequest() => create();

  GroupInvitedRequest._();

  factory GroupInvitedRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory GroupInvitedRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GroupInvitedRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.group'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupInvitedRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupInvitedRequest copyWith(void Function(GroupInvitedRequest) updates) =>
      super.copyWith((message) => updates(message as GroupInvitedRequest))
          as GroupInvitedRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GroupInvitedRequest create() => GroupInvitedRequest._();
  @$core.override
  GroupInvitedRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static GroupInvitedRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GroupInvitedRequest>(create);
  static GroupInvitedRequest? _defaultInstance;
}

/// Group info response
class GroupInvitedResponse extends $pb.GeneratedMessage {
  factory GroupInvitedResponse({
    $core.Iterable<GroupInvited>? invited,
  }) {
    final result = create();
    if (invited != null) result.invited.addAll(invited);
    return result;
  }

  GroupInvitedResponse._();

  factory GroupInvitedResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory GroupInvitedResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GroupInvitedResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.rpc.group'),
      createEmptyInstance: create)
    ..pPM<GroupInvited>(1, _omitFieldNames ? '' : 'invited',
        subBuilder: GroupInvited.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupInvitedResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupInvitedResponse copyWith(void Function(GroupInvitedResponse) updates) =>
      super.copyWith((message) => updates(message as GroupInvitedResponse))
          as GroupInvitedResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GroupInvitedResponse create() => GroupInvitedResponse._();
  @$core.override
  GroupInvitedResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static GroupInvitedResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GroupInvitedResponse>(create);
  static GroupInvitedResponse? _defaultInstance;

  /// invited list
  @$pb.TagNumber(1)
  $pb.PbList<GroupInvited> get invited => $_getList(0);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
