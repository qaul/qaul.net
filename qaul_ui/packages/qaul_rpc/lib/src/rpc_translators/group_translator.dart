part of 'abstract_rpc_module_translator.dart';

Future<List<User>> _resolveUsersForGroupInfo(
  GroupInfo g,
  List<User> knownUsers,
  Ref ref,
) async {
  final fetch = ref.read(fetchUserByIdForRpcProvider);
  final onResolved = ref.read(onGroupMemberUserResolvedProvider);
  final memberIdSet =
      g.members.map((m) => Base58Encode(Uint8List.fromList(m.userId))).toSet();
  final result = <User>[];
  for (final user in knownUsers) {
    if (memberIdSet.contains(user.idBase58)) {
      result.add(user);
    }
  }
  if (fetch == null) return result;

  final resolvedIds = result.map((u) => u.idBase58).toSet();
  final toFetch = <Uint8List>[];
  for (final m in g.members) {
    final idBytes = Uint8List.fromList(m.userId);
    if (resolvedIds.contains(Base58Encode(idBytes))) continue;
    resolvedIds.add(Base58Encode(idBytes));
    toFetch.add(idBytes);
  }
  if (toFetch.isEmpty) return result;

  final fetched = await Future.wait(toFetch.map(fetch));
  for (var i = 0; i < toFetch.length; i++) {
    final u = fetched[i];
    if (u != null) {
      result.add(u);
      onResolved?.call(u);
    }
  }
  return result;
}

class GroupTranslator extends RpcModuleTranslator {
  @override
  Modules get type => Modules.GROUP;

  @override
  Future<RpcTranslatorResponse?> decodeMessageBytes(List<int> data, Ref ref) async {
    final knownUsers = ref.read(userLookupProvider);
    final message = Group.fromBuffer(data);

    switch (message.whichMessage()) {
      case Group_Message.groupInfoResponse:
        final info = message.ensureGroupInfoResponse();
        final users = await _resolveUsersForGroupInfo(info, knownUsers, ref);
        return RpcTranslatorResponse(
          type,
          ChatRoom.fromRpcGroupInfo(info, users),
        );
      case Group_Message.groupCreateResponse:
        final createResult = message.ensureGroupCreateResponse().result;
        return _receiveGroupResultResponse(createResult);
      case Group_Message.groupRenameResponse:
        final renameResult = message.ensureGroupRenameResponse().result;
        return _receiveGroupResultResponse(renameResult);
      case Group_Message.groupInviteMemberResponse:
        final inviteResult = message.ensureGroupInviteMemberResponse().result;
        return _receiveGroupResultResponse(inviteResult);
      case Group_Message.groupRemoveMemberResponse:
        final removeResult = message.ensureGroupRemoveMemberResponse().result;
        return _receiveGroupResultResponse(removeResult);
      case Group_Message.groupReplyInviteResponse:
        final replyResult = message.ensureGroupReplyInviteResponse().result;
        return _receiveGroupResultResponse(replyResult);
      case Group_Message.groupListResponse:
        final groupsPb = message.ensureGroupListResponse().groups;
        final rooms = <ChatRoom>[];
        for (final g in groupsPb) {
          final users = await _resolveUsersForGroupInfo(g, knownUsers, ref);
          rooms.add(ChatRoom.fromRpcGroupInfo(g, users));
        }
        return RpcTranslatorResponse(type, rooms);
      case Group_Message.groupInvitedResponse:
        final invited = message.ensureGroupInvitedResponse().invited;
        final invites = <GroupInvite>[];
        for (final e in invited) {
          final users = await _resolveUsersForGroupInfo(e.group, knownUsers, ref);
          invites.add(GroupInvite.fromRpcGroupInvited(e, users));
        }
        return RpcTranslatorResponse(type, invites);
      default:
        return super.decodeMessageBytes(data, ref);
    }
  }

  RpcTranslatorResponse _receiveGroupResultResponse(GroupResult res) {
    if (res.status == true) return RpcTranslatorResponse(type, true);
    throw ArgumentError.value(res.message, 'GroupTranslator');
  }

  @override
  Future<void> processResponse(RpcTranslatorResponse res, Ref ref) async {
    if (res.module != type || res.data == null) return;

    // Means GroupResult yielded a success message.
    if (res.data is bool && res.data == true) return;

    final state = ref.read(chatRoomsProvider.notifier);
    if (res.data is List<ChatRoom>) {
      for (final room in res.data) {
        if (!state.contains(room)) {
          state.add(room);
        } else {
          state.update(room);
        }
      }
      return;
    } else if (res.data is ChatRoom) {
      if (!state.contains(res.data)) {
        state.add(res.data);
      } else {
        state.update(res.data);
      }
    } else if (res.data is List<GroupInvite>) {
      final invites = ref.read(groupInvitesProvider.notifier);
      for (final invite in res.data) {
        if (!invites.contains(invite)) {
          invites.add(invite);
        } else {
          invites.update(invite);
        }
      }
      invites.filterInvitesNotIn(res.data);
    }
  }
}
