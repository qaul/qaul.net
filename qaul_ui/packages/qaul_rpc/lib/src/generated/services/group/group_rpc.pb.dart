///
//  Generated code. Do not modify.
//  source: services/group/group_rpc.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

enum Group_Message {
  groupCreateRequest, 
  groupCreateResponse, 
  groupRenameRequest, 
  groupInviteMemberRequest, 
  groupRemoveMemberRequest, 
  groupInfoRequest, 
  groupInfoResponse, 
  groupReplyInviteRequest, 
  groupListRequest, 
  groupListResponse, 
  groupSendRequest, 
  notSet
}

class Group extends $pb.GeneratedMessage {
  static const $core.Map<$core.int, Group_Message> _Group_MessageByTag = {
    1 : Group_Message.groupCreateRequest,
    2 : Group_Message.groupCreateResponse,
    3 : Group_Message.groupRenameRequest,
    4 : Group_Message.groupInviteMemberRequest,
    5 : Group_Message.groupRemoveMemberRequest,
    6 : Group_Message.groupInfoRequest,
    7 : Group_Message.groupInfoResponse,
    8 : Group_Message.groupReplyInviteRequest,
    9 : Group_Message.groupListRequest,
    10 : Group_Message.groupListResponse,
    11 : Group_Message.groupSendRequest,
    0 : Group_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Group', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.group'), createEmptyInstance: create)
    ..oo(0, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11])
    ..aOM<GroupCreateRequest>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupCreateRequest', subBuilder: GroupCreateRequest.create)
    ..aOM<GroupCreateResponse>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupCreateResponse', subBuilder: GroupCreateResponse.create)
    ..aOM<GroupRenameRequest>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupRenameRequest', subBuilder: GroupRenameRequest.create)
    ..aOM<GroupInviteMemberRequest>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupInviteMemberRequest', subBuilder: GroupInviteMemberRequest.create)
    ..aOM<GroupRemoveMemberRequest>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupRemoveMemberRequest', subBuilder: GroupRemoveMemberRequest.create)
    ..aOM<GroupInfoRequest>(6, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupInfoRequest', subBuilder: GroupInfoRequest.create)
    ..aOM<GroupInfoResponse>(7, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupInfoResponse', subBuilder: GroupInfoResponse.create)
    ..aOM<GroupReplyInviteRequest>(8, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupReplyInviteRequest', subBuilder: GroupReplyInviteRequest.create)
    ..aOM<GroupListRequest>(9, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupListRequest', subBuilder: GroupListRequest.create)
    ..aOM<GroupListResponse>(10, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupListResponse', subBuilder: GroupListResponse.create)
    ..aOM<GroupSendRequest>(11, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupSendRequest', subBuilder: GroupSendRequest.create)
    ..hasRequiredFields = false
  ;

  Group._() : super();
  factory Group({
    GroupCreateRequest? groupCreateRequest,
    GroupCreateResponse? groupCreateResponse,
    GroupRenameRequest? groupRenameRequest,
    GroupInviteMemberRequest? groupInviteMemberRequest,
    GroupRemoveMemberRequest? groupRemoveMemberRequest,
    GroupInfoRequest? groupInfoRequest,
    GroupInfoResponse? groupInfoResponse,
    GroupReplyInviteRequest? groupReplyInviteRequest,
    GroupListRequest? groupListRequest,
    GroupListResponse? groupListResponse,
    GroupSendRequest? groupSendRequest,
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
    if (groupInviteMemberRequest != null) {
      _result.groupInviteMemberRequest = groupInviteMemberRequest;
    }
    if (groupRemoveMemberRequest != null) {
      _result.groupRemoveMemberRequest = groupRemoveMemberRequest;
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
    if (groupListRequest != null) {
      _result.groupListRequest = groupListRequest;
    }
    if (groupListResponse != null) {
      _result.groupListResponse = groupListResponse;
    }
    if (groupSendRequest != null) {
      _result.groupSendRequest = groupSendRequest;
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
  GroupInviteMemberRequest get groupInviteMemberRequest => $_getN(3);
  @$pb.TagNumber(4)
  set groupInviteMemberRequest(GroupInviteMemberRequest v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasGroupInviteMemberRequest() => $_has(3);
  @$pb.TagNumber(4)
  void clearGroupInviteMemberRequest() => clearField(4);
  @$pb.TagNumber(4)
  GroupInviteMemberRequest ensureGroupInviteMemberRequest() => $_ensure(3);

  @$pb.TagNumber(5)
  GroupRemoveMemberRequest get groupRemoveMemberRequest => $_getN(4);
  @$pb.TagNumber(5)
  set groupRemoveMemberRequest(GroupRemoveMemberRequest v) { setField(5, v); }
  @$pb.TagNumber(5)
  $core.bool hasGroupRemoveMemberRequest() => $_has(4);
  @$pb.TagNumber(5)
  void clearGroupRemoveMemberRequest() => clearField(5);
  @$pb.TagNumber(5)
  GroupRemoveMemberRequest ensureGroupRemoveMemberRequest() => $_ensure(4);

  @$pb.TagNumber(6)
  GroupInfoRequest get groupInfoRequest => $_getN(5);
  @$pb.TagNumber(6)
  set groupInfoRequest(GroupInfoRequest v) { setField(6, v); }
  @$pb.TagNumber(6)
  $core.bool hasGroupInfoRequest() => $_has(5);
  @$pb.TagNumber(6)
  void clearGroupInfoRequest() => clearField(6);
  @$pb.TagNumber(6)
  GroupInfoRequest ensureGroupInfoRequest() => $_ensure(5);

  @$pb.TagNumber(7)
  GroupInfoResponse get groupInfoResponse => $_getN(6);
  @$pb.TagNumber(7)
  set groupInfoResponse(GroupInfoResponse v) { setField(7, v); }
  @$pb.TagNumber(7)
  $core.bool hasGroupInfoResponse() => $_has(6);
  @$pb.TagNumber(7)
  void clearGroupInfoResponse() => clearField(7);
  @$pb.TagNumber(7)
  GroupInfoResponse ensureGroupInfoResponse() => $_ensure(6);

  @$pb.TagNumber(8)
  GroupReplyInviteRequest get groupReplyInviteRequest => $_getN(7);
  @$pb.TagNumber(8)
  set groupReplyInviteRequest(GroupReplyInviteRequest v) { setField(8, v); }
  @$pb.TagNumber(8)
  $core.bool hasGroupReplyInviteRequest() => $_has(7);
  @$pb.TagNumber(8)
  void clearGroupReplyInviteRequest() => clearField(8);
  @$pb.TagNumber(8)
  GroupReplyInviteRequest ensureGroupReplyInviteRequest() => $_ensure(7);

  @$pb.TagNumber(9)
  GroupListRequest get groupListRequest => $_getN(8);
  @$pb.TagNumber(9)
  set groupListRequest(GroupListRequest v) { setField(9, v); }
  @$pb.TagNumber(9)
  $core.bool hasGroupListRequest() => $_has(8);
  @$pb.TagNumber(9)
  void clearGroupListRequest() => clearField(9);
  @$pb.TagNumber(9)
  GroupListRequest ensureGroupListRequest() => $_ensure(8);

  @$pb.TagNumber(10)
  GroupListResponse get groupListResponse => $_getN(9);
  @$pb.TagNumber(10)
  set groupListResponse(GroupListResponse v) { setField(10, v); }
  @$pb.TagNumber(10)
  $core.bool hasGroupListResponse() => $_has(9);
  @$pb.TagNumber(10)
  void clearGroupListResponse() => clearField(10);
  @$pb.TagNumber(10)
  GroupListResponse ensureGroupListResponse() => $_ensure(9);

  @$pb.TagNumber(11)
  GroupSendRequest get groupSendRequest => $_getN(10);
  @$pb.TagNumber(11)
  set groupSendRequest(GroupSendRequest v) { setField(11, v); }
  @$pb.TagNumber(11)
  $core.bool hasGroupSendRequest() => $_has(10);
  @$pb.TagNumber(11)
  void clearGroupSendRequest() => clearField(11);
  @$pb.TagNumber(11)
  GroupSendRequest ensureGroupSendRequest() => $_ensure(10);
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
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupName')
    ..a<$core.List<$core.int>>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupId', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  GroupCreateResponse._() : super();
  factory GroupCreateResponse({
    $core.String? groupName,
    $core.List<$core.int>? groupId,
  }) {
    final _result = create();
    if (groupName != null) {
      _result.groupName = groupName;
    }
    if (groupId != null) {
      _result.groupId = groupId;
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
  $core.String get groupName => $_getSZ(0);
  @$pb.TagNumber(1)
  set groupName($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupName() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupName() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get groupId => $_getN(1);
  @$pb.TagNumber(2)
  set groupId($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasGroupId() => $_has(1);
  @$pb.TagNumber(2)
  void clearGroupId() => clearField(2);
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

class GroupReplyInviteRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GroupReplyInviteRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.group'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupId', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'userId', $pb.PbFieldType.OY)
    ..aOB(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'accept')
    ..hasRequiredFields = false
  ;

  GroupReplyInviteRequest._() : super();
  factory GroupReplyInviteRequest({
    $core.List<$core.int>? groupId,
    $core.List<$core.int>? userId,
    $core.bool? accept,
  }) {
    final _result = create();
    if (groupId != null) {
      _result.groupId = groupId;
    }
    if (userId != null) {
      _result.userId = userId;
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

  @$pb.TagNumber(2)
  $core.List<$core.int> get userId => $_getN(1);
  @$pb.TagNumber(2)
  set userId($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasUserId() => $_has(1);
  @$pb.TagNumber(2)
  void clearUserId() => clearField(2);

  @$pb.TagNumber(3)
  $core.bool get accept => $_getBF(2);
  @$pb.TagNumber(3)
  set accept($core.bool v) { $_setBool(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasAccept() => $_has(2);
  @$pb.TagNumber(3)
  void clearAccept() => clearField(3);
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
    ..a<$core.int>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'role', $pb.PbFieldType.OU3)
    ..a<$fixnum.Int64>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'joinedAt', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.int>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'state', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  GroupMember._() : super();
  factory GroupMember({
    $core.List<$core.int>? userId,
    $core.int? role,
    $fixnum.Int64? joinedAt,
    $core.int? state,
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
  $core.int get role => $_getIZ(1);
  @$pb.TagNumber(2)
  set role($core.int v) { $_setUnsignedInt32(1, v); }
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
  $core.int get state => $_getIZ(3);
  @$pb.TagNumber(4)
  set state($core.int v) { $_setUnsignedInt32(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasState() => $_has(3);
  @$pb.TagNumber(4)
  void clearState() => clearField(4);
}

class GroupInfoResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GroupInfoResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.group'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupId', $pb.PbFieldType.OY)
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupName')
    ..a<$fixnum.Int64>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'createdAt', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..pc<GroupMember>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'members', $pb.PbFieldType.PM, subBuilder: GroupMember.create)
    ..hasRequiredFields = false
  ;

  GroupInfoResponse._() : super();
  factory GroupInfoResponse({
    $core.List<$core.int>? groupId,
    $core.String? groupName,
    $fixnum.Int64? createdAt,
    $core.Iterable<GroupMember>? members,
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
    if (members != null) {
      _result.members.addAll(members);
    }
    return _result;
  }
  factory GroupInfoResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupInfoResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GroupInfoResponse clone() => GroupInfoResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GroupInfoResponse copyWith(void Function(GroupInfoResponse) updates) => super.copyWith((message) => updates(message as GroupInfoResponse)) as GroupInfoResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupInfoResponse create() => GroupInfoResponse._();
  GroupInfoResponse createEmptyInstance() => create();
  static $pb.PbList<GroupInfoResponse> createRepeated() => $pb.PbList<GroupInfoResponse>();
  @$core.pragma('dart2js:noInline')
  static GroupInfoResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupInfoResponse>(create);
  static GroupInfoResponse? _defaultInstance;

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
  $core.List<GroupMember> get members => $_getList(3);
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
    ..pc<GroupInfoResponse>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groups', $pb.PbFieldType.PM, subBuilder: GroupInfoResponse.create)
    ..hasRequiredFields = false
  ;

  GroupListResponse._() : super();
  factory GroupListResponse({
    $core.Iterable<GroupInfoResponse>? groups,
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
  $core.List<GroupInfoResponse> get groups => $_getList(0);
}

class GroupSendRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GroupSendRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.group'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupId', $pb.PbFieldType.OY)
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'message')
    ..hasRequiredFields = false
  ;

  GroupSendRequest._() : super();
  factory GroupSendRequest({
    $core.List<$core.int>? groupId,
    $core.String? message,
  }) {
    final _result = create();
    if (groupId != null) {
      _result.groupId = groupId;
    }
    if (message != null) {
      _result.message = message;
    }
    return _result;
  }
  factory GroupSendRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupSendRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GroupSendRequest clone() => GroupSendRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GroupSendRequest copyWith(void Function(GroupSendRequest) updates) => super.copyWith((message) => updates(message as GroupSendRequest)) as GroupSendRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupSendRequest create() => GroupSendRequest._();
  GroupSendRequest createEmptyInstance() => create();
  static $pb.PbList<GroupSendRequest> createRepeated() => $pb.PbList<GroupSendRequest>();
  @$core.pragma('dart2js:noInline')
  static GroupSendRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupSendRequest>(create);
  static GroupSendRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get groupId => $_getN(0);
  @$pb.TagNumber(1)
  set groupId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get message => $_getSZ(1);
  @$pb.TagNumber(2)
  set message($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasMessage() => $_has(1);
  @$pb.TagNumber(2)
  void clearMessage() => clearField(2);
}

class GroupConversationRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GroupConversationRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'qaul.rpc.group'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupId', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  GroupConversationRequest._() : super();
  factory GroupConversationRequest({
    $core.List<$core.int>? groupId,
  }) {
    final _result = create();
    if (groupId != null) {
      _result.groupId = groupId;
    }
    return _result;
  }
  factory GroupConversationRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupConversationRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GroupConversationRequest clone() => GroupConversationRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GroupConversationRequest copyWith(void Function(GroupConversationRequest) updates) => super.copyWith((message) => updates(message as GroupConversationRequest)) as GroupConversationRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupConversationRequest create() => GroupConversationRequest._();
  GroupConversationRequest createEmptyInstance() => create();
  static $pb.PbList<GroupConversationRequest> createRepeated() => $pb.PbList<GroupConversationRequest>();
  @$core.pragma('dart2js:noInline')
  static GroupConversationRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupConversationRequest>(create);
  static GroupConversationRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get groupId => $_getN(0);
  @$pb.TagNumber(1)
  set groupId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => clearField(1);
}

