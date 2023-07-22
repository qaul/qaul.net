//
//  Generated code. Do not modify.
//  source: services/group/group_net.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'group_net.pbenum.dart';

export 'group_net.pbenum.dart';

enum GroupContainer_Message {
  inviteMember, 
  replyInvite, 
  groupInfo, 
  removed, 
  notSet
}

class GroupContainer extends $pb.GeneratedMessage {
  factory GroupContainer() => create();
  GroupContainer._() : super();
  factory GroupContainer.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupContainer.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static const $core.Map<$core.int, GroupContainer_Message> _GroupContainer_MessageByTag = {
    1 : GroupContainer_Message.inviteMember,
    2 : GroupContainer_Message.replyInvite,
    3 : GroupContainer_Message.groupInfo,
    4 : GroupContainer_Message.removed,
    0 : GroupContainer_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GroupContainer', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.group'), createEmptyInstance: create)
    ..oo(0, [1, 2, 3, 4])
    ..aOM<InviteMember>(1, _omitFieldNames ? '' : 'inviteMember', subBuilder: InviteMember.create)
    ..aOM<ReplyInvite>(2, _omitFieldNames ? '' : 'replyInvite', subBuilder: ReplyInvite.create)
    ..aOM<GroupInfo>(3, _omitFieldNames ? '' : 'groupInfo', subBuilder: GroupInfo.create)
    ..aOM<RemovedMember>(4, _omitFieldNames ? '' : 'removed', subBuilder: RemovedMember.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GroupContainer clone() => GroupContainer()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GroupContainer copyWith(void Function(GroupContainer) updates) => super.copyWith((message) => updates(message as GroupContainer)) as GroupContainer;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GroupContainer create() => GroupContainer._();
  GroupContainer createEmptyInstance() => create();
  static $pb.PbList<GroupContainer> createRepeated() => $pb.PbList<GroupContainer>();
  @$core.pragma('dart2js:noInline')
  static GroupContainer getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupContainer>(create);
  static GroupContainer? _defaultInstance;

  GroupContainer_Message whichMessage() => _GroupContainer_MessageByTag[$_whichOneof(0)]!;
  void clearMessage() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  InviteMember get inviteMember => $_getN(0);
  @$pb.TagNumber(1)
  set inviteMember(InviteMember v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasInviteMember() => $_has(0);
  @$pb.TagNumber(1)
  void clearInviteMember() => clearField(1);
  @$pb.TagNumber(1)
  InviteMember ensureInviteMember() => $_ensure(0);

  @$pb.TagNumber(2)
  ReplyInvite get replyInvite => $_getN(1);
  @$pb.TagNumber(2)
  set replyInvite(ReplyInvite v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasReplyInvite() => $_has(1);
  @$pb.TagNumber(2)
  void clearReplyInvite() => clearField(2);
  @$pb.TagNumber(2)
  ReplyInvite ensureReplyInvite() => $_ensure(1);

  @$pb.TagNumber(3)
  GroupInfo get groupInfo => $_getN(2);
  @$pb.TagNumber(3)
  set groupInfo(GroupInfo v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasGroupInfo() => $_has(2);
  @$pb.TagNumber(3)
  void clearGroupInfo() => clearField(3);
  @$pb.TagNumber(3)
  GroupInfo ensureGroupInfo() => $_ensure(2);

  @$pb.TagNumber(4)
  RemovedMember get removed => $_getN(3);
  @$pb.TagNumber(4)
  set removed(RemovedMember v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasRemoved() => $_has(3);
  @$pb.TagNumber(4)
  void clearRemoved() => clearField(4);
  @$pb.TagNumber(4)
  RemovedMember ensureRemoved() => $_ensure(3);
}

class InviteMember extends $pb.GeneratedMessage {
  factory InviteMember() => create();
  InviteMember._() : super();
  factory InviteMember.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory InviteMember.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'InviteMember', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.group'), createEmptyInstance: create)
    ..aOM<GroupInfo>(1, _omitFieldNames ? '' : 'group', subBuilder: GroupInfo.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  InviteMember clone() => InviteMember()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  InviteMember copyWith(void Function(InviteMember) updates) => super.copyWith((message) => updates(message as InviteMember)) as InviteMember;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static InviteMember create() => InviteMember._();
  InviteMember createEmptyInstance() => create();
  static $pb.PbList<InviteMember> createRepeated() => $pb.PbList<InviteMember>();
  @$core.pragma('dart2js:noInline')
  static InviteMember getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<InviteMember>(create);
  static InviteMember? _defaultInstance;

  @$pb.TagNumber(1)
  GroupInfo get group => $_getN(0);
  @$pb.TagNumber(1)
  set group(GroupInfo v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroup() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroup() => clearField(1);
  @$pb.TagNumber(1)
  GroupInfo ensureGroup() => $_ensure(0);
}

class GroupMember extends $pb.GeneratedMessage {
  factory GroupMember() => create();
  GroupMember._() : super();
  factory GroupMember.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupMember.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GroupMember', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.group'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'userId', $pb.PbFieldType.OY)
    ..e<GroupMemberRole>(2, _omitFieldNames ? '' : 'role', $pb.PbFieldType.OE, defaultOrMaker: GroupMemberRole.User, valueOf: GroupMemberRole.valueOf, enumValues: GroupMemberRole.values)
    ..a<$fixnum.Int64>(3, _omitFieldNames ? '' : 'joinedAt', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..e<GroupMemberState>(4, _omitFieldNames ? '' : 'state', $pb.PbFieldType.OE, defaultOrMaker: GroupMemberState.Invited, valueOf: GroupMemberState.valueOf, enumValues: GroupMemberState.values)
    ..a<$core.int>(5, _omitFieldNames ? '' : 'lastMessageIndex', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GroupMember clone() => GroupMember()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GroupMember copyWith(void Function(GroupMember) updates) => super.copyWith((message) => updates(message as GroupMember)) as GroupMember;

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
  factory GroupInfo() => create();
  GroupInfo._() : super();
  factory GroupInfo.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupInfo.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GroupInfo', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.group'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'groupId', $pb.PbFieldType.OY)
    ..aOS(2, _omitFieldNames ? '' : 'groupName')
    ..a<$fixnum.Int64>(3, _omitFieldNames ? '' : 'createdAt', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.int>(4, _omitFieldNames ? '' : 'revision', $pb.PbFieldType.OU3)
    ..pc<GroupMember>(5, _omitFieldNames ? '' : 'members', $pb.PbFieldType.PM, subBuilder: GroupMember.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GroupInfo clone() => GroupInfo()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GroupInfo copyWith(void Function(GroupInfo) updates) => super.copyWith((message) => updates(message as GroupInfo)) as GroupInfo;

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
  $core.int get revision => $_getIZ(3);
  @$pb.TagNumber(4)
  set revision($core.int v) { $_setUnsignedInt32(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasRevision() => $_has(3);
  @$pb.TagNumber(4)
  void clearRevision() => clearField(4);

  @$pb.TagNumber(5)
  $core.List<GroupMember> get members => $_getList(4);
}

class ReplyInvite extends $pb.GeneratedMessage {
  factory ReplyInvite() => create();
  ReplyInvite._() : super();
  factory ReplyInvite.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ReplyInvite.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ReplyInvite', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.group'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'groupId', $pb.PbFieldType.OY)
    ..aOB(2, _omitFieldNames ? '' : 'accept')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ReplyInvite clone() => ReplyInvite()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ReplyInvite copyWith(void Function(ReplyInvite) updates) => super.copyWith((message) => updates(message as ReplyInvite)) as ReplyInvite;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ReplyInvite create() => ReplyInvite._();
  ReplyInvite createEmptyInstance() => create();
  static $pb.PbList<ReplyInvite> createRepeated() => $pb.PbList<ReplyInvite>();
  @$core.pragma('dart2js:noInline')
  static ReplyInvite getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ReplyInvite>(create);
  static ReplyInvite? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get groupId => $_getN(0);
  @$pb.TagNumber(1)
  set groupId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => clearField(1);

  @$pb.TagNumber(2)
  $core.bool get accept => $_getBF(1);
  @$pb.TagNumber(2)
  set accept($core.bool v) { $_setBool(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasAccept() => $_has(1);
  @$pb.TagNumber(2)
  void clearAccept() => clearField(2);
}

class RemovedMember extends $pb.GeneratedMessage {
  factory RemovedMember() => create();
  RemovedMember._() : super();
  factory RemovedMember.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RemovedMember.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'RemovedMember', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.group'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'groupId', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RemovedMember clone() => RemovedMember()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RemovedMember copyWith(void Function(RemovedMember) updates) => super.copyWith((message) => updates(message as RemovedMember)) as RemovedMember;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RemovedMember create() => RemovedMember._();
  RemovedMember createEmptyInstance() => create();
  static $pb.PbList<RemovedMember> createRepeated() => $pb.PbList<RemovedMember>();
  @$core.pragma('dart2js:noInline')
  static RemovedMember getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RemovedMember>(create);
  static RemovedMember? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get groupId => $_getN(0);
  @$pb.TagNumber(1)
  set groupId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => clearField(1);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
