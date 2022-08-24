part of 'abstract_rpc_module_translator.dart';

class GroupTranslator extends RpcModuleTranslator {
  @override
  Modules get type => Modules.GROUP;

  @override
  Future<RpcTranslatorResponse?> decodeMessageBytes(List<int> data) async {
    final message = Group.fromBuffer(data);
    switch (message.whichMessage()) {
      case Group_Message.groupCreateResponse:
        final group = message.ensureGroupCreateResponse();
        return RpcTranslatorResponse(
          Modules.GROUP,
          GroupInfo(
            id: Uint8List.fromList(group.groupId),
            // groupName: group.groupName,
            groupName: '',
            createdAt: DateTime.now(),
            members: const [],
          ),
        );
      case Group_Message.groupInfoResponse:
        return RpcTranslatorResponse(
          Modules.GROUP,
          GroupInfo.fromGroupInfoResponse(message.ensureGroupInfoResponse()),
        );
      case Group_Message.groupListResponse:
        final groups = message
            .ensureGroupListResponse()
            .groups
            .map((e) => GroupInfo.fromGroupInfoResponse(e))
            .toList();
        return RpcTranslatorResponse(Modules.GROUP, groups);
      default:
        return super.decodeMessageBytes(data);
    }
  }
}
