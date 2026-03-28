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
      final openRoom = currentOpenRoom.state;
      final isOpenRoom = openRoom != null &&
          openRoom.conversationId.equals(groupBytes);

      final source = room ?? (isOpenRoom ? openRoom : null);
      if (source == null) return;

      final merged = source.copyWithMessages(res.data);

      if (room != null) state.update(merged);

      if (isOpenRoom) {
        currentOpenRoom.state = merged;
      }
    } else {
      super.processResponse(res, ref);
    }
  }
}
