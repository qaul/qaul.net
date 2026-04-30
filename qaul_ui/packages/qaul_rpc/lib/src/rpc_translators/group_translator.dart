part of 'abstract_rpc_module_translator.dart';

class GroupTranslator extends RpcModuleTranslator {
  @override
  Modules get type => Modules.GROUP;

  @override
  Future<RpcTranslatorResponse?> decodeMessageBytes(
      List<int> data, Ref ref) async {
    final knownUsers = ref.read(userLookupProvider);
    final message = Group.fromBuffer(data);

    switch (message.whichMessage()) {
      case Group_Message.groupInfoResponse:
        final info = message.ensureGroupInfoResponse();
        return RpcTranslatorResponse(
          type,
          ChatRoom.fromRpcGroupInfo(info, knownUsers),
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
        final groupResponse = message.ensureGroupListResponse();
        final groupsPb = groupResponse.groups;
        final rooms = groupsPb
            .map((g) => ChatRoom.fromRpcGroupInfo(g, knownUsers))
            .toList();
        PaginationState? pagination;
        if (groupResponse.hasPagination()) {
          final meta = groupResponse.pagination;
          pagination = PaginationState(
            hasMore: meta.hasMore,
            total: meta.total,
            offset: meta.offset,
            limit: meta.limit,
          );
        }
        return RpcTranslatorResponse(
          type,
          PaginatedChatRooms(rooms: rooms, pagination: pagination),
        );
      case Group_Message.groupInvitedResponse:
        final invitedResponse = message.ensureGroupInvitedResponse();
        final invited = invitedResponse.invited;
        final invites = invited
            .map((e) => GroupInvite.fromRpcGroupInvited(e, knownUsers))
            .toList();
        PaginationState? pagination;
        if (invitedResponse.hasPagination()) {
          final meta = invitedResponse.pagination;
          pagination = PaginationState(
            hasMore: meta.hasMore,
            total: meta.total,
            offset: meta.offset,
            limit: meta.limit,
          );
        }
        return RpcTranslatorResponse(
          type,
          PaginatedGroupInvites(invites: invites, pagination: pagination),
        );
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
    if (res.data is PaginatedChatRooms) {
      final paginated = res.data as PaginatedChatRooms;
      final pagination = paginated.pagination;
      if (pagination != null) {
        if (pagination.offset == 0) {
          state.setAll(paginated.rooms);
        } else {
          state.append(paginated.rooms);
        }
        return;
      }
      for (final room in paginated.rooms) {
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
    } else if (res.data is PaginatedGroupInvites) {
      final paginated = res.data as PaginatedGroupInvites;
      final invites = ref.read(groupInvitesProvider.notifier);
      final pagination = paginated.pagination;
      if (pagination != null) {
        if (pagination.offset == 0) {
          invites.setAll(paginated.invites);
        } else {
          invites.append(paginated.invites);
        }
        return;
      }

      for (final invite in paginated.invites) {
        if (!invites.contains(invite)) {
          invites.add(invite);
        } else {
          invites.update(invite);
        }
      }
      invites.filterInvitesNotIn(paginated.invites);
    }
  }
}
