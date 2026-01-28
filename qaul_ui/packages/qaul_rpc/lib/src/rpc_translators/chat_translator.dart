part of 'abstract_rpc_module_translator.dart';

class ChatTranslator extends RpcModuleTranslator {
  @override
  Modules get type => Modules.CHAT;

  @override
  Future<RpcTranslatorResponse?> decodeMessageBytes(
      List<int> data, Ref ref) async {
    final message = Chat.fromBuffer(data);
    switch (message.whichMessage()) {
      case Chat_Message.conversationList:
        return RpcTranslatorResponse(type, message.ensureConversationList());
      default:
        return super.decodeMessageBytes(data, ref);
    }
  }

  @override
  Future<void> processResponse(RpcTranslatorResponse res, Ref ref) async {
    if (res.module != type || res.data == null) return;
    if (res.data is ChatConversationList) {
      final chatRoomsNotifier = ref.read(chatRoomsProvider.notifier);
      final room = ref.read(chatRoomsProvider)
          .firstWhereOrNull((r) => r.conversationId.equals(res.data.groupId));
      final currentOpenRoom = ref.read(currentOpenChatRoom);

      if (room != null) {
        final roomWithMessages = room.mergeWithConversationList(res.data);
        chatRoomsNotifier.update(roomWithMessages);

        if (_currentOpenRoomEqualsChatConversationList(
            currentOpenRoom, res.data as ChatConversationList)) {
          ref.read(currentOpenChatRoom.notifier).setRoom(roomWithMessages);
        }
      }
    } else {
      super.processResponse(res, ref);
    }
  }

  bool _currentOpenRoomEqualsChatConversationList(
    ChatRoom? currentOpenRoom,
    ChatConversationList resData,
  ) =>
      currentOpenRoom != null &&
      currentOpenRoom.conversationId.equals(resData.groupId);
}
