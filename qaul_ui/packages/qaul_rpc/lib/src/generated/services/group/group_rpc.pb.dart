///
//  Generated code. Do not modify.
//  source: services/group/group_rpc.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'group_rpc.pbenum.dart';

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

class Group extends $pb.GeneratedMessage {
  static const $core.Map<$core.int, Group_Message> _Group_MessageByTag = {
    1 : Group_Message.groupCreateRequest,
    2 : Group_Message.groupCreateResponse,
    3 : Group_Message.groupRenameRequest,
    4 : Group_Message.groupRenameResponse,
    5 : Group_Message.groupInviteMemberRequest,
    6 : Group_Message.groupInviteMemberResponse,
    7 : Group_Message.groupRemoveMemberRequest,
    8 : Group_Message.groupRemoveMemberResponse,
    9 : Group_Message.groupInfoRequest,
    10 : Group_Message.groupInfoResponse,
    11 : Group_Message.groupReplyInviteRequest,
    12 : Group_Message.groupReplyInviteResponse,
    13 : Group_Message.groupListRequest,
    14 : Group_Message.groupListResponse,
    15 : Group_Message.groupInvitedRequest,
    16 : Group_Message.groupInvitedResponse,
    0 : Group_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Group', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.group'), createEmptyInstance: create)
    ..oo(0, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16])
    ..aOM<GroupCreateRequest>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupCreateRequest', subBuilder: GroupCreateRequest.create)
    ..aOM<GroupCreateResponse>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupCreateResponse', subBuilder: GroupCreateResponse.create)
    ..aOM<GroupRenameRequest>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupRenameRequest', subBuilder: GroupRenameRequest.create)
    ..aOM<GroupRenameResponse>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupRenameResponse', subBuilder: GroupRenameResponse.create)
    ..aOM<GroupInviteMemberRequest>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupInviteMemberRequest', subBuilder: GroupInviteMemberRequest.create)
    ..aOM<GroupInviteMemberResponse>(6, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupInviteMemberResponse', subBuilder: GroupInviteMemberResponse.create)
    ..aOM<GroupRemoveMemberRequest>(7, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupRemoveMemberRequest', subBuilder: GroupRemoveMemberRequest.create)
    ..aOM<GroupRemoveMemberResponse>(8, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupRemoveMemberResponse', subBuilder: GroupRemoveMemberResponse.create)
    ..aOM<GroupInfoRequest>(9, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupInfoRequest', subBuilder: GroupInfoRequest.create)
    ..aOM<GroupInfo>(10, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupInfoResponse', subBuilder: GroupInfo.create)
    ..aOM<GroupReplyInviteRequest>(11, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupReplyInviteRequest', subBuilder: GroupReplyInviteRequest.create)
    ..aOM<GroupReplyInviteResponse>(12, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupReplyInviteResponse', subBuilder: GroupReplyInviteResponse.create)
    ..aOM<GroupListRequest>(13, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupListRequest', subBuilder: GroupListRequest.create)
    ..aOM<GroupListResponse>(14, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupListResponse', subBuilder: GroupListResponse.create)
    ..aOM<GroupInvitedRequest>(15, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupInvitedRequest', subBuilder: GroupInvitedRequest.create)
    ..aOM<GroupInvitedResponse>(16, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupInvitedResponse', subBuilder: GroupInvitedResponse.create)
    ..hasRequiredFields = false
  ;

  Group._() : super();
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
    final _result = create();
    if (groupCreateRequest != null) {
      _result.groupCreateRequest = groupCreateRequest;
    }
    if (groupCreateResponse != null) {
      _result.groupCreateResponse = groupCreateResponse;
    }
    if (groupRenameRequest != null) {
      _result.groupRenameRequest = groupRenameRequest;
    }
    if (groupRenameResponse != null) {
      _result.groupRenameResponse = groupRenameResponse;
    }
    if (groupInviteMemberRequest != null) {
      _result.groupInviteMemberRequest = groupInviteMemberRequest;
    }
    if (groupInviteMemberResponse != null) {
      _result.groupInviteMemberResponse = groupInviteMemberResponse;
    }
    if (groupRemoveMemberRequest != null) {
      _result.groupRemoveMemberRequest = groupRemoveMemberRequest;
    }
    if (groupRemoveMemberResponse != null) {
      _result.groupRemoveMemberResponse = groupRemoveMemberResponse;
    }
    if (groupInfoRequest != null) {
      _result.groupInfoRequest = groupInfoRequest;
    }
    if (groupInfoResponse != null) {
      _result.groupInfoResponse = groupInfoResponse;
    }
    if (groupReplyInviteRequest != null) {
      _result.groupReplyInviteRequest = groupReplyInviteRequest;
    }
    if (groupReplyInviteResponse != null) {
      _result.groupReplyInviteResponse = groupReplyInviteResponse;
    }
    if (groupListRequest != null) {
      _result.groupListRequest = groupListRequest;
    }
    if (groupListResponse != null) {
      _result.groupListResponse = groupListResponse;
    }
    if (groupInvitedRequest != null) {
      _result.groupInvitedRequest = groupInvitedRequest;
    }
    if (groupInvitedResponse != null) {
      _result.groupInvitedResponse = groupInvitedResponse;
    }
    return _result;
  }
  factory Group.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Group.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Group clone() => Group()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Group copyWith(void Function(Group) updates) => super.copyWith((message) => updates(message as Group)) as Group; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Group create() => Group._();
  Group createEmptyInstance() => create();
  static $pb.PbList<Group> createRepeated() => $pb.PbList<Group>();
  @$core.pragma('dart2js:noInline')
  static Group getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Group>(create);
  static Group? _defaultInstance;

  Group_Message whichMessage() => _Group_MessageByTag[$_whichOneof(0)]!;
  void clearMessage() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  GroupCreateRequest get groupCreateRequest => $_getN(0);
  @$pb.TagNumber(1)
  set groupCreateRequest(GroupCreateRequest v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupCreateRequest() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupCreateRequest() => clearField(1);
  @$pb.TagNumber(1)
  GroupCreateRequest ensureGroupCreateRequest() => $_ensure(0);

  @$pb.TagNumber(2)
  GroupCreateResponse get groupCreateResponse => $_getN(1);
  @$pb.TagNumber(2)
  set groupCreateResponse(GroupCreateResponse v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasGroupCreateResponse() => $_has(1);
  @$pb.TagNumber(2)
  void clearGroupCreateResponse() => clearField(2);
  @$pb.TagNumber(2)
  GroupCreateResponse ensureGroupCreateResponse() => $_ensure(1);

  @$pb.TagNumber(3)
  GroupRenameRequest get groupRenameRequest => $_getN(2);
  @$pb.TagNumber(3)
  set groupRenameRequest(GroupRenameRequest v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasGroupRenameRequest() => $_has(2);
  @$pb.TagNumber(3)
  void clearGroupRenameRequest() => clearField(3);
  @$pb.TagNumber(3)
  GroupRenameRequest ensureGroupRenameRequest() => $_ensure(2);

  @$pb.TagNumber(4)
  GroupRenameResponse get groupRenameResponse => $_getN(3);
  @$pb.TagNumber(4)
  set groupRenameResponse(GroupRenameResponse v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasGroupRenameResponse() => $_has(3);
  @$pb.TagNumber(4)
  void clearGroupRenameResponse() => clearField(4);
  @$pb.TagNumber(4)
  GroupRenameResponse ensureGroupRenameResponse() => $_ensure(3);

  @$pb.TagNumber(5)
  GroupInviteMemberRequest get groupInviteMemberRequest => $_getN(4);
  @$pb.TagNumber(5)
  set groupInviteMemberRequest(GroupInviteMemberRequest v) { setField(5, v); }
  @$pb.TagNumber(5)
  $core.bool hasGroupInviteMemberRequest() => $_has(4);
  @$pb.TagNumber(5)
  void clearGroupInviteMemberRequest() => clearField(5);
  @$pb.TagNumber(5)
  GroupInviteMemberRequest ensureGroupInviteMemberRequest() => $_ensure(4);

  @$pb.TagNumber(6)
  GroupInviteMemberResponse get groupInviteMemberResponse => $_getN(5);
  @$pb.TagNumber(6)
  set groupInviteMemberResponse(GroupInviteMemberResponse v) { setField(6, v); }
  @$pb.TagNumber(6)
  $core.bool hasGroupInviteMemberResponse() => $_has(5);
  @$pb.TagNumber(6)
  void clearGroupInviteMemberResponse() => clearField(6);
  @$pb.TagNumber(6)
  GroupInviteMemberResponse ensureGroupInviteMemberResponse() => $_ensure(5);

  @$pb.TagNumber(7)
  GroupRemoveMemberRequest get groupRemoveMemberRequest => $_getN(6);
  @$pb.TagNumber(7)
  set groupRemoveMemberRequest(GroupRemoveMemberRequest v) { setField(7, v); }
  @$pb.TagNumber(7)
  $core.bool hasGroupRemoveMemberRequest() => $_has(6);
  @$pb.TagNumber(7)
  void clearGroupRemoveMemberRequest() => clearField(7);
  @$pb.TagNumber(7)
  GroupRemoveMemberRequest ensureGroupRemoveMemberRequest() => $_ensure(6);

  @$pb.TagNumber(8)
  GroupRemoveMemberResponse get groupRemoveMemberResponse => $_getN(7);
  @$pb.TagNumber(8)
  set groupRemoveMemberResponse(GroupRemoveMemberResponse v) { setField(8, v); }
  @$pb.TagNumber(8)
  $core.bool hasGroupRemoveMemberResponse() => $_has(7);
  @$pb.TagNumber(8)
  void clearGroupRemoveMemberResponse() => clearField(8);
  @$pb.TagNumber(8)
  GroupRemoveMemberResponse ensureGroupRemoveMemberResponse() => $_ensure(7);

  @$pb.TagNumber(9)
  GroupInfoRequest get groupInfoRequest => $_getN(8);
  @$pb.TagNumber(9)
  set groupInfoRequest(GroupInfoRequest v) { setField(9, v); }
  @$pb.TagNumber(9)
  $core.bool hasGroupInfoRequest() => $_has(8);
  @$pb.TagNumber(9)
  void clearGroupInfoRequest() => clearField(9);
  @$pb.TagNumber(9)
  GroupInfoRequest ensureGroupInfoRequest() => $_ensure(8);

  @$pb.TagNumber(10)
  GroupInfo get groupInfoResponse => $_getN(9);
  @$pb.TagNumber(10)
  set groupInfoResponse(GroupInfo v) { setField(10, v); }
  @$pb.TagNumber(10)
  $core.bool hasGroupInfoResponse() => $_has(9);
  @$pb.TagNumber(10)
  void clearGroupInfoResponse() => clearField(10);
  @$pb.TagNumber(10)
  GroupInfo ensureGroupInfoResponse() => $_ensure(9);

  @$pb.TagNumber(11)
  GroupReplyInviteRequest get groupReplyInviteRequest => $_getN(10);
  @$pb.TagNumber(11)
  set groupReplyInviteRequest(GroupReplyInviteRequest v) { setField(11, v); }
  @$pb.TagNumber(11)
  $core.bool hasGroupReplyInviteRequest() => $_has(10);
  @$pb.TagNumber(11)
  void clearGroupReplyInviteRequest() => clearField(11);
  @$pb.TagNumber(11)
  GroupReplyInviteRequest ensureGroupReplyInviteRequest() => $_ensure(10);

  @$pb.TagNumber(12)
  GroupReplyInviteResponse get groupReplyInviteResponse => $_getN(11);
  @$pb.TagNumber(12)
  set groupReplyInviteResponse(GroupReplyInviteResponse v) { setField(12, v); }
  @$pb.TagNumber(12)
  $core.bool hasGroupReplyInviteResponse() => $_has(11);
  @$pb.TagNumber(12)
  void clearGroupReplyInviteResponse() => clearField(12);
  @$pb.TagNumber(12)
  GroupReplyInviteResponse ensureGroupReplyInviteResponse() => $_ensure(11);

  @$pb.TagNumber(13)
  GroupListRequest get groupListRequest => $_getN(12);
  @$pb.TagNumber(13)
  set groupListRequest(GroupListRequest v) { setField(13, v); }
  @$pb.TagNumber(13)
  $core.bool hasGroupListRequest() => $_has(12);
  @$pb.TagNumber(13)
  void clearGroupListRequest() => clearField(13);
  @$pb.TagNumber(13)
  GroupListRequest ensureGroupListRequest() => $_ensure(12);

  @$pb.TagNumber(14)
  GroupListResponse get groupListResponse => $_getN(13);
  @$pb.TagNumber(14)
  set groupListResponse(GroupListResponse v) { setField(14, v); }
  @$pb.TagNumber(14)
  $core.bool hasGroupListResponse() => $_has(13);
  @$pb.TagNumber(14)
  void clearGroupListResponse() => clearField(14);
  @$pb.TagNumber(14)
  GroupListResponse ensureGroupListResponse() => $_ensure(13);

  @$pb.TagNumber(15)
  GroupInvitedRequest get groupInvitedRequest => $_getN(14);
  @$pb.TagNumber(15)
  set groupInvitedRequest(GroupInvitedRequest v) { setField(15, v); }
  @$pb.TagNumber(15)
  $core.bool hasGroupInvitedRequest() => $_has(14);
  @$pb.TagNumber(15)
  void clearGroupInvitedRequest() => clearField(15);
  @$pb.TagNumber(15)
  GroupInvitedRequest ensureGroupInvitedRequest() => $_ensure(14);

  @$pb.TagNumber(16)
  GroupInvitedResponse get groupInvitedResponse => $_getN(15);
  @$pb.TagNumber(16)
  set groupInvitedResponse(GroupInvitedResponse v) { setField(16, v); }
  @$pb.TagNumber(16)
  $core.bool hasGroupInvitedResponse() => $_has(15);
  @$pb.TagNumber(16)
  void clearGroupInvitedResponse() => clearField(16);
  @$pb.TagNumber(16)
  GroupInvitedResponse ensureGroupInvitedResponse() => $_ensure(15);
}

class GroupResult extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GroupResult', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.group'), createEmptyInstance: create)
    ..aOB(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'status')
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'message')
    ..hasRequiredFields = false
  ;

  GroupResult._() : super();
  factory GroupResult({
    $core.bool? status,
    $core.String? message,
  }) {
    final _result = create();
    if (status != null) {
      _result.status = status;
    }
    if (message != null) {
      _result.message = message;
    }
    return _result;
  }
  factory GroupResult.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupResult.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GroupResult clone() => GroupResult()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GroupResult copyWith(void Function(GroupResult) updates) => super.copyWith((message) => updates(message as GroupResult)) as GroupResult; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupResult create() => GroupResult._();
  GroupResult createEmptyInstance() => create();
  static $pb.PbList<GroupResult> createRepeated() => $pb.PbList<GroupResult>();
  @$core.pragma('dart2js:noInline')
  static GroupResult getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupResult>(create);
  static GroupResult? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get status => $_getBF(0);
  @$pb.TagNumber(1)
  set status($core.bool v) { $_setBool(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasStatus() => $_has(0);
  @$pb.TagNumber(1)
  void clearStatus() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get message => $_getSZ(1);
  @$pb.TagNumber(2)
  set message($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasMessage() => $_has(1);
  @$pb.TagNumber(2)
  void clearMessage() => clearField(2);
}

class GroupCreateRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GroupCreateRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.group'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupName')
    ..hasRequiredFields = false
  ;

  GroupCreateRequest._() : super();
  factory GroupCreateRequest({
    $core.String? groupName,
  }) {
    final _result = create();
    if (groupName != null) {
      _result.groupName = groupName;
    }
    return _result;
  }
  factory GroupCreateRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupCreateRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GroupCreateRequest clone() => GroupCreateRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GroupCreateRequest copyWith(void Function(GroupCreateRequest) updates) => super.copyWith((message) => updates(message as GroupCreateRequest)) as GroupCreateRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupCreateRequest create() => GroupCreateRequest._();
  GroupCreateRequest createEmptyInstance() => create();
  static $pb.PbList<GroupCreateRequest> createRepeated() => $pb.PbList<GroupCreateRequest>();
  @$core.pragma('dart2js:noInline')
  static GroupCreateRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupCreateRequest>(create);
  static GroupCreateRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get groupName => $_getSZ(0);
  @$pb.TagNumber(1)
  set groupName($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupName() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupName() => clearField(1);
}

class GroupCreateResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GroupCreateResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.group'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupId', $pb.PbFieldType.OY)
    ..aOM<GroupResult>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'result', subBuilder: GroupResult.create)
    ..hasRequiredFields = false
  ;

  GroupCreateResponse._() : super();
  factory GroupCreateResponse({
    $core.List<$core.int>? groupId,
    GroupResult? result,
  }) {
    final _result = create();
    if (groupId != null) {
      _result.groupId = groupId;
    }
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory GroupCreateResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupCreateResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GroupCreateResponse clone() => GroupCreateResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GroupCreateResponse copyWith(void Function(GroupCreateResponse) updates) => super.copyWith((message) => updates(message as GroupCreateResponse)) as GroupCreateResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupCreateResponse create() => GroupCreateResponse._();
  GroupCreateResponse createEmptyInstance() => create();
  static $pb.PbList<GroupCreateResponse> createRepeated() => $pb.PbList<GroupCreateResponse>();
  @$core.pragma('dart2js:noInline')
  static GroupCreateResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupCreateResponse>(create);
  static GroupCreateResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get groupId => $_getN(0);
  @$pb.TagNumber(1)
  set groupId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => clearField(1);

  @$pb.TagNumber(2)
  GroupResult get result => $_getN(1);
  @$pb.TagNumber(2)
  set result(GroupResult v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasResult() => $_has(1);
  @$pb.TagNumber(2)
  void clearResult() => clearField(2);
  @$pb.TagNumber(2)
  GroupResult ensureResult() => $_ensure(1);
}

class GroupRenameRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GroupRenameRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.group'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupId', $pb.PbFieldType.OY)
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupName')
    ..hasRequiredFields = false
  ;

  GroupRenameRequest._() : super();
  factory GroupRenameRequest({
    $core.List<$core.int>? groupId,
    $core.String? groupName,
  }) {
    final _result = create();
    if (groupId != null) {
      _result.groupId = groupId;
    }
    if (groupName != null) {
      _result.groupName = groupName;
    }
    return _result;
  }
  factory GroupRenameRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupRenameRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GroupRenameRequest clone() => GroupRenameRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GroupRenameRequest copyWith(void Function(GroupRenameRequest) updates) => super.copyWith((message) => updates(message as GroupRenameRequest)) as GroupRenameRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupRenameRequest create() => GroupRenameRequest._();
  GroupRenameRequest createEmptyInstance() => create();
  static $pb.PbList<GroupRenameRequest> createRepeated() => $pb.PbList<GroupRenameRequest>();
  @$core.pragma('dart2js:noInline')
  static GroupRenameRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupRenameRequest>(create);
  static GroupRenameRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get groupId => $_getN(0);
  @$pb.TagNumber(1)
  set groupId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get groupName => $_getSZ(1);
  @$pb.TagNumber(2)
  set groupName($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasGroupName() => $_has(1);
  @$pb.TagNumber(2)
  void clearGroupName() => clearField(2);
}

class GroupRenameResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GroupRenameResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.group'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupId', $pb.PbFieldType.OY)
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupName')
    ..aOM<GroupResult>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'result', subBuilder: GroupResult.create)
    ..hasRequiredFields = false
  ;

  GroupRenameResponse._() : super();
  factory GroupRenameResponse({
    $core.List<$core.int>? groupId,
    $core.String? groupName,
    GroupResult? result,
  }) {
    final _result = create();
    if (groupId != null) {
      _result.groupId = groupId;
    }
    if (groupName != null) {
      _result.groupName = groupName;
    }
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory GroupRenameResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupRenameResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GroupRenameResponse clone() => GroupRenameResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GroupRenameResponse copyWith(void Function(GroupRenameResponse) updates) => super.copyWith((message) => updates(message as GroupRenameResponse)) as GroupRenameResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupRenameResponse create() => GroupRenameResponse._();
  GroupRenameResponse createEmptyInstance() => create();
  static $pb.PbList<GroupRenameResponse> createRepeated() => $pb.PbList<GroupRenameResponse>();
  @$core.pragma('dart2js:noInline')
  static GroupRenameResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupRenameResponse>(create);
  static GroupRenameResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get groupId => $_getN(0);
  @$pb.TagNumber(1)
  set groupId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get groupName => $_getSZ(1);
  @$pb.TagNumber(2)
  set groupName($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasGroupName() => $_has(1);
  @$pb.TagNumber(2)
  void clearGroupName() => clearField(2);

  @$pb.TagNumber(3)
  GroupResult get result => $_getN(2);
  @$pb.TagNumber(3)
  set result(GroupResult v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasResult() => $_has(2);
  @$pb.TagNumber(3)
  void clearResult() => clearField(3);
  @$pb.TagNumber(3)
  GroupResult ensureResult() => $_ensure(2);
}

class GroupInviteMemberRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GroupInviteMemberRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.group'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupId', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'userId', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  GroupInviteMemberRequest._() : super();
  factory GroupInviteMemberRequest({
    $core.List<$core.int>? groupId,
    $core.List<$core.int>? userId,
  }) {
    final _result = create();
    if (groupId != null) {
      _result.groupId = groupId;
    }
    if (userId != null) {
      _result.userId = userId;
    }
    return _result;
  }
  factory GroupInviteMemberRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupInviteMemberRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GroupInviteMemberRequest clone() => GroupInviteMemberRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GroupInviteMemberRequest copyWith(void Function(GroupInviteMemberRequest) updates) => super.copyWith((message) => updates(message as GroupInviteMemberRequest)) as GroupInviteMemberRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupInviteMemberRequest create() => GroupInviteMemberRequest._();
  GroupInviteMemberRequest createEmptyInstance() => create();
  static $pb.PbList<GroupInviteMemberRequest> createRepeated() => $pb.PbList<GroupInviteMemberRequest>();
  @$core.pragma('dart2js:noInline')
  static GroupInviteMemberRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupInviteMemberRequest>(create);
  static GroupInviteMemberRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get groupId => $_getN(0);
  @$pb.TagNumber(1)
  set groupId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get userId => $_getN(1);
  @$pb.TagNumber(2)
  set userId($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasUserId() => $_has(1);
  @$pb.TagNumber(2)
  void clearUserId() => clearField(2);
}

class GroupInviteMemberResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GroupInviteMemberResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.group'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupId', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'userId', $pb.PbFieldType.OY)
    ..aOM<GroupResult>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'result', subBuilder: GroupResult.create)
    ..hasRequiredFields = false
  ;

  GroupInviteMemberResponse._() : super();
  factory GroupInviteMemberResponse({
    $core.List<$core.int>? groupId,
    $core.List<$core.int>? userId,
    GroupResult? result,
  }) {
    final _result = create();
    if (groupId != null) {
      _result.groupId = groupId;
    }
    if (userId != null) {
      _result.userId = userId;
    }
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory GroupInviteMemberResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupInviteMemberResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GroupInviteMemberResponse clone() => GroupInviteMemberResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GroupInviteMemberResponse copyWith(void Function(GroupInviteMemberResponse) updates) => super.copyWith((message) => updates(message as GroupInviteMemberResponse)) as GroupInviteMemberResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupInviteMemberResponse create() => GroupInviteMemberResponse._();
  GroupInviteMemberResponse createEmptyInstance() => create();
  static $pb.PbList<GroupInviteMemberResponse> createRepeated() => $pb.PbList<GroupInviteMemberResponse>();
  @$core.pragma('dart2js:noInline')
  static GroupInviteMemberResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupInviteMemberResponse>(create);
  static GroupInviteMemberResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get groupId => $_getN(0);
  @$pb.TagNumber(1)
  set groupId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get userId => $_getN(1);
  @$pb.TagNumber(2)
  set userId($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasUserId() => $_has(1);
  @$pb.TagNumber(2)
  void clearUserId() => clearField(2);

  @$pb.TagNumber(3)
  GroupResult get result => $_getN(2);
  @$pb.TagNumber(3)
  set result(GroupResult v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasResult() => $_has(2);
  @$pb.TagNumber(3)
  void clearResult() => clearField(3);
  @$pb.TagNumber(3)
  GroupResult ensureResult() => $_ensure(2);
}

class GroupReplyInviteRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GroupReplyInviteRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.group'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupId', $pb.PbFieldType.OY)
    ..aOB(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'accept')
    ..hasRequiredFields = false
  ;

  GroupReplyInviteRequest._() : super();
  factory GroupReplyInviteRequest({
    $core.List<$core.int>? groupId,
    $core.bool? accept,
  }) {
    final _result = create();
    if (groupId != null) {
      _result.groupId = groupId;
    }
    if (accept != null) {
      _result.accept = accept;
    }
    return _result;
  }
  factory GroupReplyInviteRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupReplyInviteRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GroupReplyInviteRequest clone() => GroupReplyInviteRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GroupReplyInviteRequest copyWith(void Function(GroupReplyInviteRequest) updates) => super.copyWith((message) => updates(message as GroupReplyInviteRequest)) as GroupReplyInviteRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupReplyInviteRequest create() => GroupReplyInviteRequest._();
  GroupReplyInviteRequest createEmptyInstance() => create();
  static $pb.PbList<GroupReplyInviteRequest> createRepeated() => $pb.PbList<GroupReplyInviteRequest>();
  @$core.pragma('dart2js:noInline')
  static GroupReplyInviteRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupReplyInviteRequest>(create);
  static GroupReplyInviteRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get groupId => $_getN(0);
  @$pb.TagNumber(1)
  set groupId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => clearField(1);

  @$pb.TagNumber(3)
  $core.bool get accept => $_getBF(1);
  @$pb.TagNumber(3)
  set accept($core.bool v) { $_setBool(1, v); }
  @$pb.TagNumber(3)
  $core.bool hasAccept() => $_has(1);
  @$pb.TagNumber(3)
  void clearAccept() => clearField(3);
}

class GroupReplyInviteResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GroupReplyInviteResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.group'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupId', $pb.PbFieldType.OY)
    ..aOM<GroupResult>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'result', subBuilder: GroupResult.create)
    ..hasRequiredFields = false
  ;

  GroupReplyInviteResponse._() : super();
  factory GroupReplyInviteResponse({
    $core.List<$core.int>? groupId,
    GroupResult? result,
  }) {
    final _result = create();
    if (groupId != null) {
      _result.groupId = groupId;
    }
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory GroupReplyInviteResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupReplyInviteResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GroupReplyInviteResponse clone() => GroupReplyInviteResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GroupReplyInviteResponse copyWith(void Function(GroupReplyInviteResponse) updates) => super.copyWith((message) => updates(message as GroupReplyInviteResponse)) as GroupReplyInviteResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupReplyInviteResponse create() => GroupReplyInviteResponse._();
  GroupReplyInviteResponse createEmptyInstance() => create();
  static $pb.PbList<GroupReplyInviteResponse> createRepeated() => $pb.PbList<GroupReplyInviteResponse>();
  @$core.pragma('dart2js:noInline')
  static GroupReplyInviteResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupReplyInviteResponse>(create);
  static GroupReplyInviteResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get groupId => $_getN(0);
  @$pb.TagNumber(1)
  set groupId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => clearField(1);

  @$pb.TagNumber(3)
  GroupResult get result => $_getN(1);
  @$pb.TagNumber(3)
  set result(GroupResult v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasResult() => $_has(1);
  @$pb.TagNumber(3)
  void clearResult() => clearField(3);
  @$pb.TagNumber(3)
  GroupResult ensureResult() => $_ensure(1);
}

class GroupRemoveMemberRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GroupRemoveMemberRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.group'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupId', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'userId', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  GroupRemoveMemberRequest._() : super();
  factory GroupRemoveMemberRequest({
    $core.List<$core.int>? groupId,
    $core.List<$core.int>? userId,
  }) {
    final _result = create();
    if (groupId != null) {
      _result.groupId = groupId;
    }
    if (userId != null) {
      _result.userId = userId;
    }
    return _result;
  }
  factory GroupRemoveMemberRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupRemoveMemberRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GroupRemoveMemberRequest clone() => GroupRemoveMemberRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GroupRemoveMemberRequest copyWith(void Function(GroupRemoveMemberRequest) updates) => super.copyWith((message) => updates(message as GroupRemoveMemberRequest)) as GroupRemoveMemberRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupRemoveMemberRequest create() => GroupRemoveMemberRequest._();
  GroupRemoveMemberRequest createEmptyInstance() => create();
  static $pb.PbList<GroupRemoveMemberRequest> createRepeated() => $pb.PbList<GroupRemoveMemberRequest>();
  @$core.pragma('dart2js:noInline')
  static GroupRemoveMemberRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupRemoveMemberRequest>(create);
  static GroupRemoveMemberRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get groupId => $_getN(0);
  @$pb.TagNumber(1)
  set groupId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get userId => $_getN(1);
  @$pb.TagNumber(2)
  set userId($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasUserId() => $_has(1);
  @$pb.TagNumber(2)
  void clearUserId() => clearField(2);
}

class GroupRemoveMemberResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GroupRemoveMemberResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.group'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupId', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'userId', $pb.PbFieldType.OY)
    ..aOM<GroupResult>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'result', subBuilder: GroupResult.create)
    ..hasRequiredFields = false
  ;

  GroupRemoveMemberResponse._() : super();
  factory GroupRemoveMemberResponse({
    $core.List<$core.int>? groupId,
    $core.List<$core.int>? userId,
    GroupResult? result,
  }) {
    final _result = create();
    if (groupId != null) {
      _result.groupId = groupId;
    }
    if (userId != null) {
      _result.userId = userId;
    }
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory GroupRemoveMemberResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupRemoveMemberResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GroupRemoveMemberResponse clone() => GroupRemoveMemberResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GroupRemoveMemberResponse copyWith(void Function(GroupRemoveMemberResponse) updates) => super.copyWith((message) => updates(message as GroupRemoveMemberResponse)) as GroupRemoveMemberResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupRemoveMemberResponse create() => GroupRemoveMemberResponse._();
  GroupRemoveMemberResponse createEmptyInstance() => create();
  static $pb.PbList<GroupRemoveMemberResponse> createRepeated() => $pb.PbList<GroupRemoveMemberResponse>();
  @$core.pragma('dart2js:noInline')
  static GroupRemoveMemberResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupRemoveMemberResponse>(create);
  static GroupRemoveMemberResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get groupId => $_getN(0);
  @$pb.TagNumber(1)
  set groupId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get userId => $_getN(1);
  @$pb.TagNumber(2)
  set userId($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasUserId() => $_has(1);
  @$pb.TagNumber(2)
  void clearUserId() => clearField(2);

  @$pb.TagNumber(3)
  GroupResult get result => $_getN(2);
  @$pb.TagNumber(3)
  set result(GroupResult v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasResult() => $_has(2);
  @$pb.TagNumber(3)
  void clearResult() => clearField(3);
  @$pb.TagNumber(3)
  GroupResult ensureResult() => $_ensure(2);
}

class GroupInfoRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GroupInfoRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.group'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupId', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  GroupInfoRequest._() : super();
  factory GroupInfoRequest({
    $core.List<$core.int>? groupId,
  }) {
    final _result = create();
    if (groupId != null) {
      _result.groupId = groupId;
    }
    return _result;
  }
  factory GroupInfoRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupInfoRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GroupInfoRequest clone() => GroupInfoRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GroupInfoRequest copyWith(void Function(GroupInfoRequest) updates) => super.copyWith((message) => updates(message as GroupInfoRequest)) as GroupInfoRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupInfoRequest create() => GroupInfoRequest._();
  GroupInfoRequest createEmptyInstance() => create();
  static $pb.PbList<GroupInfoRequest> createRepeated() => $pb.PbList<GroupInfoRequest>();
  @$core.pragma('dart2js:noInline')
  static GroupInfoRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupInfoRequest>(create);
  static GroupInfoRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get groupId => $_getN(0);
  @$pb.TagNumber(1)
  set groupId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => clearField(1);
}

class GroupMember extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GroupMember', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.group'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'userId', $pb.PbFieldType.OY)
    ..e<GroupMemberRole>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'role', $pb.PbFieldType.OE, defaultOrMaker: GroupMemberRole.User, valueOf: GroupMemberRole.valueOf, enumValues: GroupMemberRole.values)
    ..a<$fixnum.Int64>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'joinedAt', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..e<GroupMemberState>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'state', $pb.PbFieldType.OE, defaultOrMaker: GroupMemberState.Invited, valueOf: GroupMemberState.valueOf, enumValues: GroupMemberState.values)
    ..a<$core.int>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'lastMessageIndex', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  GroupMember._() : super();
  factory GroupMember({
    $core.List<$core.int>? userId,
    GroupMemberRole? role,
    $fixnum.Int64? joinedAt,
    GroupMemberState? state,
    $core.int? lastMessageIndex,
  }) {
    final _result = create();
    if (userId != null) {
      _result.userId = userId;
    }
    if (role != null) {
      _result.role = role;
    }
    if (joinedAt != null) {
      _result.joinedAt = joinedAt;
    }
    if (state != null) {
      _result.state = state;
    }
    if (lastMessageIndex != null) {
      _result.lastMessageIndex = lastMessageIndex;
    }
    return _result;
  }
  factory GroupMember.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupMember.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GroupMember clone() => GroupMember()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GroupMember copyWith(void Function(GroupMember) updates) => super.copyWith((message) => updates(message as GroupMember)) as GroupMember; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupMember create() => GroupMember._();
  GroupMember createEmptyInstance() => create();
  static $pb.PbList<GroupMember> createRepeated() => $pb.PbList<GroupMember>();
  @$core.pragma('dart2js:noInline')
  static GroupMember getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupMember>(create);
  static GroupMember? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get userId => $_getN(0);
  @$pb.TagNumber(1)
  set userId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasUserId() => $_has(0);
  @$pb.TagNumber(1)
  void clearUserId() => clearField(1);

  @$pb.TagNumber(2)
  GroupMemberRole get role => $_getN(1);
  @$pb.TagNumber(2)
  set role(GroupMemberRole v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasRole() => $_has(1);
  @$pb.TagNumber(2)
  void clearRole() => clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get joinedAt => $_getI64(2);
  @$pb.TagNumber(3)
  set joinedAt($fixnum.Int64 v) { $_setInt64(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasJoinedAt() => $_has(2);
  @$pb.TagNumber(3)
  void clearJoinedAt() => clearField(3);

  @$pb.TagNumber(4)
  GroupMemberState get state => $_getN(3);
  @$pb.TagNumber(4)
  set state(GroupMemberState v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasState() => $_has(3);
  @$pb.TagNumber(4)
  void clearState() => clearField(4);

  @$pb.TagNumber(5)
  $core.int get lastMessageIndex => $_getIZ(4);
  @$pb.TagNumber(5)
  set lastMessageIndex($core.int v) { $_setUnsignedInt32(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasLastMessageIndex() => $_has(4);
  @$pb.TagNumber(5)
  void clearLastMessageIndex() => clearField(5);
}

class GroupInfo extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GroupInfo', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.group'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupId', $pb.PbFieldType.OY)
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupName')
    ..a<$fixnum.Int64>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'createdAt', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..e<GroupStatus>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'status', $pb.PbFieldType.OE, defaultOrMaker: GroupStatus.ACTIVE, valueOf: GroupStatus.valueOf, enumValues: GroupStatus.values)
    ..a<$core.int>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'revision', $pb.PbFieldType.OU3)
    ..aOB(6, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'isDirectChat')
    ..pc<GroupMember>(7, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'members', $pb.PbFieldType.PM, subBuilder: GroupMember.create)
    ..a<$core.int>(8, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'unreadMessages', $pb.PbFieldType.OU3)
    ..a<$fixnum.Int64>(9, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'lastMessageAt', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.List<$core.int>>(10, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'lastMessage', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(11, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'lastMessageSenderId', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  GroupInfo._() : super();
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
    final _result = create();
    if (groupId != null) {
      _result.groupId = groupId;
    }
    if (groupName != null) {
      _result.groupName = groupName;
    }
    if (createdAt != null) {
      _result.createdAt = createdAt;
    }
    if (status != null) {
      _result.status = status;
    }
    if (revision != null) {
      _result.revision = revision;
    }
    if (isDirectChat != null) {
      _result.isDirectChat = isDirectChat;
    }
    if (members != null) {
      _result.members.addAll(members);
    }
    if (unreadMessages != null) {
      _result.unreadMessages = unreadMessages;
    }
    if (lastMessageAt != null) {
      _result.lastMessageAt = lastMessageAt;
    }
    if (lastMessage != null) {
      _result.lastMessage = lastMessage;
    }
    if (lastMessageSenderId != null) {
      _result.lastMessageSenderId = lastMessageSenderId;
    }
    return _result;
  }
  factory GroupInfo.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupInfo.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GroupInfo clone() => GroupInfo()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GroupInfo copyWith(void Function(GroupInfo) updates) => super.copyWith((message) => updates(message as GroupInfo)) as GroupInfo; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupInfo create() => GroupInfo._();
  GroupInfo createEmptyInstance() => create();
  static $pb.PbList<GroupInfo> createRepeated() => $pb.PbList<GroupInfo>();
  @$core.pragma('dart2js:noInline')
  static GroupInfo getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupInfo>(create);
  static GroupInfo? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get groupId => $_getN(0);
  @$pb.TagNumber(1)
  set groupId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get groupName => $_getSZ(1);
  @$pb.TagNumber(2)
  set groupName($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasGroupName() => $_has(1);
  @$pb.TagNumber(2)
  void clearGroupName() => clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get createdAt => $_getI64(2);
  @$pb.TagNumber(3)
  set createdAt($fixnum.Int64 v) { $_setInt64(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasCreatedAt() => $_has(2);
  @$pb.TagNumber(3)
  void clearCreatedAt() => clearField(3);

  @$pb.TagNumber(4)
  GroupStatus get status => $_getN(3);
  @$pb.TagNumber(4)
  set status(GroupStatus v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasStatus() => $_has(3);
  @$pb.TagNumber(4)
  void clearStatus() => clearField(4);

  @$pb.TagNumber(5)
  $core.int get revision => $_getIZ(4);
  @$pb.TagNumber(5)
  set revision($core.int v) { $_setUnsignedInt32(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasRevision() => $_has(4);
  @$pb.TagNumber(5)
  void clearRevision() => clearField(5);

  @$pb.TagNumber(6)
  $core.bool get isDirectChat => $_getBF(5);
  @$pb.TagNumber(6)
  set isDirectChat($core.bool v) { $_setBool(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasIsDirectChat() => $_has(5);
  @$pb.TagNumber(6)
  void clearIsDirectChat() => clearField(6);

  @$pb.TagNumber(7)
  $core.List<GroupMember> get members => $_getList(6);

  @$pb.TagNumber(8)
  $core.int get unreadMessages => $_getIZ(7);
  @$pb.TagNumber(8)
  set unreadMessages($core.int v) { $_setUnsignedInt32(7, v); }
  @$pb.TagNumber(8)
  $core.bool hasUnreadMessages() => $_has(7);
  @$pb.TagNumber(8)
  void clearUnreadMessages() => clearField(8);

  @$pb.TagNumber(9)
  $fixnum.Int64 get lastMessageAt => $_getI64(8);
  @$pb.TagNumber(9)
  set lastMessageAt($fixnum.Int64 v) { $_setInt64(8, v); }
  @$pb.TagNumber(9)
  $core.bool hasLastMessageAt() => $_has(8);
  @$pb.TagNumber(9)
  void clearLastMessageAt() => clearField(9);

  @$pb.TagNumber(10)
  $core.List<$core.int> get lastMessage => $_getN(9);
  @$pb.TagNumber(10)
  set lastMessage($core.List<$core.int> v) { $_setBytes(9, v); }
  @$pb.TagNumber(10)
  $core.bool hasLastMessage() => $_has(9);
  @$pb.TagNumber(10)
  void clearLastMessage() => clearField(10);

  @$pb.TagNumber(11)
  $core.List<$core.int> get lastMessageSenderId => $_getN(10);
  @$pb.TagNumber(11)
  set lastMessageSenderId($core.List<$core.int> v) { $_setBytes(10, v); }
  @$pb.TagNumber(11)
  $core.bool hasLastMessageSenderId() => $_has(10);
  @$pb.TagNumber(11)
  void clearLastMessageSenderId() => clearField(11);
}

class GroupListRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GroupListRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.group'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupListRequest._() : super();
  factory GroupListRequest() => create();
  factory GroupListRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupListRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GroupListRequest clone() => GroupListRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GroupListRequest copyWith(void Function(GroupListRequest) updates) => super.copyWith((message) => updates(message as GroupListRequest)) as GroupListRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupListRequest create() => GroupListRequest._();
  GroupListRequest createEmptyInstance() => create();
  static $pb.PbList<GroupListRequest> createRepeated() => $pb.PbList<GroupListRequest>();
  @$core.pragma('dart2js:noInline')
  static GroupListRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupListRequest>(create);
  static GroupListRequest? _defaultInstance;
}

class GroupListResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GroupListResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.group'), createEmptyInstance: create)
    ..pc<GroupInfo>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groups', $pb.PbFieldType.PM, subBuilder: GroupInfo.create)
    ..hasRequiredFields = false
  ;

  GroupListResponse._() : super();
  factory GroupListResponse({
    $core.Iterable<GroupInfo>? groups,
  }) {
    final _result = create();
    if (groups != null) {
      _result.groups.addAll(groups);
    }
    return _result;
  }
  factory GroupListResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupListResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GroupListResponse clone() => GroupListResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GroupListResponse copyWith(void Function(GroupListResponse) updates) => super.copyWith((message) => updates(message as GroupListResponse)) as GroupListResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupListResponse create() => GroupListResponse._();
  GroupListResponse createEmptyInstance() => create();
  static $pb.PbList<GroupListResponse> createRepeated() => $pb.PbList<GroupListResponse>();
  @$core.pragma('dart2js:noInline')
  static GroupListResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupListResponse>(create);
  static GroupListResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<GroupInfo> get groups => $_getList(0);
}

class GroupInvited extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GroupInvited', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.group'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'senderId', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'receivedAt', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOM<GroupInfo>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'group', subBuilder: GroupInfo.create)
    ..hasRequiredFields = false
  ;

  GroupInvited._() : super();
  factory GroupInvited({
    $core.List<$core.int>? senderId,
    $fixnum.Int64? receivedAt,
    GroupInfo? group,
  }) {
    final _result = create();
    if (senderId != null) {
      _result.senderId = senderId;
    }
    if (receivedAt != null) {
      _result.receivedAt = receivedAt;
    }
    if (group != null) {
      _result.group = group;
    }
    return _result;
  }
  factory GroupInvited.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupInvited.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GroupInvited clone() => GroupInvited()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GroupInvited copyWith(void Function(GroupInvited) updates) => super.copyWith((message) => updates(message as GroupInvited)) as GroupInvited; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupInvited create() => GroupInvited._();
  GroupInvited createEmptyInstance() => create();
  static $pb.PbList<GroupInvited> createRepeated() => $pb.PbList<GroupInvited>();
  @$core.pragma('dart2js:noInline')
  static GroupInvited getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupInvited>(create);
  static GroupInvited? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get senderId => $_getN(0);
  @$pb.TagNumber(1)
  set senderId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasSenderId() => $_has(0);
  @$pb.TagNumber(1)
  void clearSenderId() => clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get receivedAt => $_getI64(1);
  @$pb.TagNumber(2)
  set receivedAt($fixnum.Int64 v) { $_setInt64(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasReceivedAt() => $_has(1);
  @$pb.TagNumber(2)
  void clearReceivedAt() => clearField(2);

  @$pb.TagNumber(3)
  GroupInfo get group => $_getN(2);
  @$pb.TagNumber(3)
  set group(GroupInfo v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasGroup() => $_has(2);
  @$pb.TagNumber(3)
  void clearGroup() => clearField(3);
  @$pb.TagNumber(3)
  GroupInfo ensureGroup() => $_ensure(2);
}

class GroupInvitedRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GroupInvitedRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.group'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupInvitedRequest._() : super();
  factory GroupInvitedRequest() => create();
  factory GroupInvitedRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupInvitedRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GroupInvitedRequest clone() => GroupInvitedRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GroupInvitedRequest copyWith(void Function(GroupInvitedRequest) updates) => super.copyWith((message) => updates(message as GroupInvitedRequest)) as GroupInvitedRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupInvitedRequest create() => GroupInvitedRequest._();
  GroupInvitedRequest createEmptyInstance() => create();
  static $pb.PbList<GroupInvitedRequest> createRepeated() => $pb.PbList<GroupInvitedRequest>();
  @$core.pragma('dart2js:noInline')
  static GroupInvitedRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupInvitedRequest>(create);
  static GroupInvitedRequest? _defaultInstance;
}

class GroupInvitedResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GroupInvitedResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.group'), createEmptyInstance: create)
    ..pc<GroupInvited>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'invited', $pb.PbFieldType.PM, subBuilder: GroupInvited.create)
    ..hasRequiredFields = false
  ;

  GroupInvitedResponse._() : super();
  factory GroupInvitedResponse({
    $core.Iterable<GroupInvited>? invited,
  }) {
    final _result = create();
    if (invited != null) {
      _result.invited.addAll(invited);
    }
    return _result;
  }
  factory GroupInvitedResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupInvitedResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GroupInvitedResponse clone() => GroupInvitedResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GroupInvitedResponse copyWith(void Function(GroupInvitedResponse) updates) => super.copyWith((message) => updates(message as GroupInvitedResponse)) as GroupInvitedResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupInvitedResponse create() => GroupInvitedResponse._();
  GroupInvitedResponse createEmptyInstance() => create();
  static $pb.PbList<GroupInvitedResponse> createRepeated() => $pb.PbList<GroupInvitedResponse>();
  @$core.pragma('dart2js:noInline')
  static GroupInvitedResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupInvitedResponse>(create);
  static GroupInvitedResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<GroupInvited> get invited => $_getList(0);
}

