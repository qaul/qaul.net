part of 'abstract_rpc_module_translator.dart';

class GroupTranslator extends RpcModuleTranslator {
  @override
  Modules get type => Modules.GROUP;

  @override
  Future<RpcTranslatorResponse?> decodeMessageBytes(List<int> data, Ref ref) async {
    final users = ref.read(usersProvider).data;
    final message = Group.fromBuffer(data);

    switch (message.whichMessage()) {
      case Group_Message.groupInfoResponse:
        return RpcTranslatorResponse(
          type,
          ChatRoom.fromRpcGroupInfo(message.ensureGroupInfoResponse(), users),
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
        final groups = message
            .ensureGroupListResponse()
            .groups
            .map((g) => ChatRoom.fromRpcGroupInfo(g, users))
            .toList();
        return RpcTranslatorResponse(type, groups);
      case Group_Message.groupInvitedResponse:
        final invites = message
            .ensureGroupInvitedResponse()
            .invited
            .map((e) => GroupInvite.fromRpcGroupInvited(e, users))
            .toList();
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
