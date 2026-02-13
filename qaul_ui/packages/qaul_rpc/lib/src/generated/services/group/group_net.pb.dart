// This is a generated file - do not edit.
//
// Generated from services/group/group_net.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'group_net.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'group_net.pbenum.dart';

enum GroupContainer_Message {
  inviteMember,
  replyInvite,
  groupInfo,
  removed,
  notSet
}

/// Group network message container
class GroupContainer extends $pb.GeneratedMessage {
  factory GroupContainer({
    InviteMember? inviteMember,
    ReplyInvite? replyInvite,
    GroupInfo? groupInfo,
    RemovedMember? removed,
  }) {
    final result = create();
    if (inviteMember != null) result.inviteMember = inviteMember;
    if (replyInvite != null) result.replyInvite = replyInvite;
    if (groupInfo != null) result.groupInfo = groupInfo;
    if (removed != null) result.removed = removed;
    return result;
  }

  GroupContainer._();

  factory GroupContainer.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory GroupContainer.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, GroupContainer_Message>
      _GroupContainer_MessageByTag = {
    1: GroupContainer_Message.inviteMember,
    2: GroupContainer_Message.replyInvite,
    3: GroupContainer_Message.groupInfo,
    4: GroupContainer_Message.removed,
    0: GroupContainer_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GroupContainer',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.group'),
      createEmptyInstance: create)
    ..oo(0, [1, 2, 3, 4])
    ..aOM<InviteMember>(1, _omitFieldNames ? '' : 'inviteMember',
        subBuilder: InviteMember.create)
    ..aOM<ReplyInvite>(2, _omitFieldNames ? '' : 'replyInvite',
        subBuilder: ReplyInvite.create)
    ..aOM<GroupInfo>(3, _omitFieldNames ? '' : 'groupInfo',
        subBuilder: GroupInfo.create)
    ..aOM<RemovedMember>(4, _omitFieldNames ? '' : 'removed',
        subBuilder: RemovedMember.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupContainer clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GroupContainer copyWith(void Function(GroupContainer) updates) =>
      super.copyWith((message) => updates(message as GroupContainer))
          as GroupContainer;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GroupContainer create() => GroupContainer._();
  @$core.override
  GroupContainer createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static GroupContainer getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GroupContainer>(create);
  static GroupContainer? _defaultInstance;

  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  GroupContainer_Message whichMessage() =>
      _GroupContainer_MessageByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  void clearMessage() => $_clearField($_whichOneof(0));

  /// group invite
  @$pb.TagNumber(1)
  InviteMember get inviteMember => $_getN(0);
  @$pb.TagNumber(1)
  set inviteMember(InviteMember value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasInviteMember() => $_has(0);
  @$pb.TagNumber(1)
  void clearInviteMember() => $_clearField(1);
  @$pb.TagNumber(1)
  InviteMember ensureInviteMember() => $_ensure(0);

  /// reply invite
  @$pb.TagNumber(2)
  ReplyInvite get replyInvite => $_getN(1);
  @$pb.TagNumber(2)
  set replyInvite(ReplyInvite value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasReplyInvite() => $_has(1);
  @$pb.TagNumber(2)
  void clearReplyInvite() => $_clearField(2);
  @$pb.TagNumber(2)
  ReplyInvite ensureReplyInvite() => $_ensure(1);

  /// group status update
  @$pb.TagNumber(3)
  GroupInfo get groupInfo => $_getN(2);
  @$pb.TagNumber(3)
  set groupInfo(GroupInfo value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasGroupInfo() => $_has(2);
  @$pb.TagNumber(3)
  void clearGroupInfo() => $_clearField(3);
  @$pb.TagNumber(3)
  GroupInfo ensureGroupInfo() => $_ensure(2);

  /// member removed
  @$pb.TagNumber(4)
  RemovedMember get removed => $_getN(3);
  @$pb.TagNumber(4)
  set removed(RemovedMember value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasRemoved() => $_has(3);
  @$pb.TagNumber(4)
  void clearRemoved() => $_clearField(4);
  @$pb.TagNumber(4)
  RemovedMember ensureRemoved() => $_ensure(3);
}

/// Invite member
class InviteMember extends $pb.GeneratedMessage {
  factory InviteMember({
    GroupInfo? group,
  }) {
    final result = create();
    if (group != null) result.group = group;
    return result;
  }

  InviteMember._();

  factory InviteMember.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory InviteMember.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'InviteMember',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.group'),
      createEmptyInstance: create)
    ..aOM<GroupInfo>(1, _omitFieldNames ? '' : 'group',
        subBuilder: GroupInfo.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  InviteMember clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  InviteMember copyWith(void Function(InviteMember) updates) =>
      super.copyWith((message) => updates(message as InviteMember))
          as InviteMember;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static InviteMember create() => InviteMember._();
  @$core.override
  InviteMember createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static InviteMember getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<InviteMember>(create);
  static InviteMember? _defaultInstance;

  /// Group Info
  @$pb.TagNumber(1)
  GroupInfo get group => $_getN(0);
  @$pb.TagNumber(1)
  set group(GroupInfo value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasGroup() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroup() => $_clearField(1);
  @$pb.TagNumber(1)
  GroupInfo ensureGroup() => $_ensure(0);
}

/// Group member
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
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.group'),
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

/// Group Info
class GroupInfo extends $pb.GeneratedMessage {
  factory GroupInfo({
    $core.List<$core.int>? groupId,
    $core.String? groupName,
    $fixnum.Int64? createdAt,
    $core.int? revision,
    $core.Iterable<GroupMember>? members,
  }) {
    final result = create();
    if (groupId != null) result.groupId = groupId;
    if (groupName != null) result.groupName = groupName;
    if (createdAt != null) result.createdAt = createdAt;
    if (revision != null) result.revision = revision;
    if (members != null) result.members.addAll(members);
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
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.group'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'groupId', $pb.PbFieldType.OY)
    ..aOS(2, _omitFieldNames ? '' : 'groupName')
    ..a<$fixnum.Int64>(
        3, _omitFieldNames ? '' : 'createdAt', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..aI(4, _omitFieldNames ? '' : 'revision', fieldType: $pb.PbFieldType.OU3)
    ..pPM<GroupMember>(5, _omitFieldNames ? '' : 'members',
        subBuilder: GroupMember.create)
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

  /// group revision
  @$pb.TagNumber(4)
  $core.int get revision => $_getIZ(3);
  @$pb.TagNumber(4)
  set revision($core.int value) => $_setUnsignedInt32(3, value);
  @$pb.TagNumber(4)
  $core.bool hasRevision() => $_has(3);
  @$pb.TagNumber(4)
  void clearRevision() => $_clearField(4);

  /// updated members
  @$pb.TagNumber(5)
  $pb.PbList<GroupMember> get members => $_getList(4);
}

/// Reply to Invite
///
/// Accept / Reject invitation
class ReplyInvite extends $pb.GeneratedMessage {
  factory ReplyInvite({
    $core.List<$core.int>? groupId,
    $core.bool? accept,
  }) {
    final result = create();
    if (groupId != null) result.groupId = groupId;
    if (accept != null) result.accept = accept;
    return result;
  }

  ReplyInvite._();

  factory ReplyInvite.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ReplyInvite.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ReplyInvite',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.group'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'groupId', $pb.PbFieldType.OY)
    ..aOB(2, _omitFieldNames ? '' : 'accept')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ReplyInvite clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ReplyInvite copyWith(void Function(ReplyInvite) updates) =>
      super.copyWith((message) => updates(message as ReplyInvite))
          as ReplyInvite;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ReplyInvite create() => ReplyInvite._();
  @$core.override
  ReplyInvite createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ReplyInvite getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ReplyInvite>(create);
  static ReplyInvite? _defaultInstance;

  /// group id
  @$pb.TagNumber(1)
  $core.List<$core.int> get groupId => $_getN(0);
  @$pb.TagNumber(1)
  set groupId($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => $_clearField(1);

  /// accept true : accept, false: decline
  @$pb.TagNumber(2)
  $core.bool get accept => $_getBF(1);
  @$pb.TagNumber(2)
  set accept($core.bool value) => $_setBool(1, value);
  @$pb.TagNumber(2)
  $core.bool hasAccept() => $_has(1);
  @$pb.TagNumber(2)
  void clearAccept() => $_clearField(2);
}

/// Removed member
class RemovedMember extends $pb.GeneratedMessage {
  factory RemovedMember({
    $core.List<$core.int>? groupId,
  }) {
    final result = create();
    if (groupId != null) result.groupId = groupId;
    return result;
  }

  RemovedMember._();

  factory RemovedMember.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory RemovedMember.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RemovedMember',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.group'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'groupId', $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemovedMember clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemovedMember copyWith(void Function(RemovedMember) updates) =>
      super.copyWith((message) => updates(message as RemovedMember))
          as RemovedMember;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RemovedMember create() => RemovedMember._();
  @$core.override
  RemovedMember createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static RemovedMember getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RemovedMember>(create);
  static RemovedMember? _defaultInstance;

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

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
