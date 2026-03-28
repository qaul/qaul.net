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
      final state = ref.read(chatRoomsProvider.notifier);
      final room = ref.read(chatRoomsProvider)
          .firstWhereOrNull((r) => r.conversationId.equals(res.data.groupId));
      final currentOpenRoom = ref.read(currentOpenChatRoom.notifier);

      final groupBytes = Uint8List.fromList(res.data.groupId);
      ChatRoom? roomWithMessages;
      if (room != null) {
        roomWithMessages = room.mergeWithConversationList(res.data);
        state.update(roomWithMessages);
      } else {
        final open = currentOpenRoom.state;
        if (open != null && open.conversationId.equals(groupBytes)) {
          roomWithMessages = open.mergeWithConversationList(res.data);
        }
      }

      final open = currentOpenRoom.state;
      if (open != null &&
          roomWithMessages != null &&
          open.idBase58 == roomWithMessages.idBase58) {
        currentOpenRoom.state = roomWithMessages;
      }
    } else {
      super.processResponse(res, ref);
    }
  }
}
