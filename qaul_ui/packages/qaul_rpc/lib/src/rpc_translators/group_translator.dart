part of 'abstract_rpc_module_translator.dart';

class GroupTranslator extends RpcModuleTranslator {
  @override
  Modules get type => Modules.GROUP;

  @override
  Future<RpcTranslatorResponse?> decodeMessageBytes(List<int> data) async {
    final message = Group.fromBuffer(data);
    switch (message.whichMessage()) {
      case Group_Message.groupInfoResponse:
        return RpcTranslatorResponse(
          Modules.GROUP,
          GroupDetails.fromRpcGroupInfo(message.ensureGroupInfoResponse()),
        );
      case Group_Message.groupCreateResponse:
        final group = message.ensureGroupCreateResponse();
        return RpcTranslatorResponse(
          Modules.GROUP,
          GroupDetails(
            id: Uint8List.fromList(group.groupId),
            // groupName: group.groupName,
            groupName: '',
            createdAt: DateTime.now(),
            members: const [],
          ),
        );
      case Group_Message.groupRenameResponse:
      case Group_Message.groupInviteMemberResponse:
      case Group_Message.groupRemoveMemberResponse:
      case Group_Message.groupReplyInviteResponse:
        // TODO
        throw UnimplementedError('unhandled group modification message');
      case Group_Message.groupListResponse:
        final groups = message
            .ensureGroupListResponse()
            .groups
            .map((e) => GroupDetails.fromRpcGroupInfo(e))
            .toList();
        return RpcTranslatorResponse(Modules.GROUP, groups);
      case Group_Message.groupInvitedResponse:
        final invites = message
            .ensureGroupInvitedResponse()
            .invited
            .map((e) => GroupInvite.fromRpcGroupInvited(e))
            .toList();
        return RpcTranslatorResponse(Modules.GROUP, invites);
      default:
        return super.decodeMessageBytes(data);
    }
  }
}
