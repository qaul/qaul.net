part of 'abstract_rpc_module_translator.dart';

class ChatTranslator extends RpcModuleTranslator {
  @override
  Modules get type => Modules.CHAT;

  @override
  Future<RpcTranslatorResponse?> decodeMessageBytes(List<int> data) async {
    final message = Chat.fromBuffer(data);
    switch (message.whichMessage()) {
      case Chat_Message.overviewList:
        final rooms = message
            .ensureOverviewList()
            .overviewList
            .map((e) => ChatRoom.fromOverview(e))
            .toList();
        return RpcTranslatorResponse(Modules.CHAT, rooms);
      case Chat_Message.conversationList:
        var r = ChatRoom.fromConversationList(message.ensureConversationList());
        return RpcTranslatorResponse(Modules.CHAT, r);
      default:
        return super.decodeMessageBytes(data);
    }
  }

  @override
  Future<void> processResponse(RpcTranslatorResponse res, Reader reader) async {
    if (res.module != type || res.data == null) return;
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
