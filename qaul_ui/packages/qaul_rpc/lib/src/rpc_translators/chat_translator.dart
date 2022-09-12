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
    // TODO: not handling the proper data coming from [decodeMessageBytes]
    if (res.data is List<ChatRoom>) {
      final state = reader(chatRoomsProvider.notifier);
      for (final room in res.data) {
        if (!state.contains(room)) {
          state.add(room);
        } else {
          state.update(room);
        }
      }
    }
    if (res.data is ChatRoom) {
      final currentRoom = reader(currentOpenChatRoom);
      if (currentRoom != null &&
          currentRoom.conversationId.equals(res.data.conversationId)) {
        reader(currentOpenChatRoom.notifier).state = res.data;
      }
    }
  }
}
