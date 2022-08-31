part of 'abstract_rpc_module_translator.dart';

class ChatTranslator extends RpcModuleTranslator {
  @override
  Modules get type => Modules.CHAT;

  @override
  Future<RpcTranslatorResponse?> decodeMessageBytes(List<int> data) async {
    final message = Chat.fromBuffer(data);
    switch (message.whichMessage()) {
      case Chat_Message.overviewList:
        final rooms =
            message.ensureOverviewList().overviewList.map((e) => ChatRoom.fromOverview(e)).toList();
        return RpcTranslatorResponse(Modules.CHAT, rooms);
      case Chat_Message.conversationList:
        return RpcTranslatorResponse(Modules.CHAT, message.ensureConversationList());
      default:
        return super.decodeMessageBytes(data);
    }
  }
}
