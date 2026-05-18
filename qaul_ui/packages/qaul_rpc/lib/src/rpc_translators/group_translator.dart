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
      // offset > 0 → user scrolled to load more; append the new page.
      // offset == 0 (or no pagination metadata) → first-page fetch from initial
      // load, pull-to-refresh, or polling. Merge instead of replacing so any
      // already-loaded later pages survive. Pull-to-refresh resets state
      // explicitly via the notifier's clear() before fetching.
      if (pagination != null && pagination.offset > 0) {
        state.append(paginated.rooms);
        return;
      }
      state.mergeOrderedFromBackend(paginated.rooms);
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
      if (pagination != null && pagination.offset > 0) {
        invites.append(paginated.invites);
        return;
      }
      for (final invite in paginated.invites) {
        if (!invites.contains(invite)) {
          invites.add(invite);
        } else {
          invites.update(invite);
        }
      }
      // Prune stale invites (e.g. one the user just accepted/declined) only
      // when the response is the complete set: legacy unpaginated path, or a
      // single first page with no further pages. With multi-page lists we
      // can't tell whether a "missing" invite was removed on the backend or
      // simply lives on a later page, so we leave state alone and rely on
      // pull-to-refresh.
      final isCompleteList =
          pagination == null || (pagination.offset == 0 && !pagination.hasMore);
      if (isCompleteList) {
        invites.retainAll(paginated.invites);
      }
    }
  }
}
