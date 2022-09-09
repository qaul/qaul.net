part of 'abstract_rpc_module_translator.dart';

class ChatTranslator extends RpcModuleTranslator {
  @override
  Modules get type => Modules.CHAT;

  @override
  Future<RpcTranslatorResponse?> decodeMessageBytes(List<int> data, Reader reader) async {
    final message = Chat.fromBuffer(data);
    switch (message.whichMessage()) {
      case Chat_Message.conversationList:
        return RpcTranslatorResponse(Modules.CHAT, message.ensureConversationList());
      default:
        return super.decodeMessageBytes(data, reader);
    }
  }

  @override
  Future<void> processResponse(RpcTranslatorResponse res, Reader reader) async {
    if (res.module != type || res.data == null) return;
    if (res.data is ChatConversationList) {
      final state = reader(chatRoomsProvider.notifier);
      final room = reader(chatRoomsProvider).firstWhereOrNull(
          (r) => r.conversationId.equals(res.data.groupId));

      if (room != null) {
        state.update(room.mergeWithConversationList(res.data));
      }
    } else {
      super.processResponse(res, reader);
    }
  }
}
