part of 'chat_room.dart';

class ChatRoomListNotifier extends Notifier<List<ChatRoom>> {
  @override
  List<ChatRoom> build() => [];

  void add(ChatRoom room) => state = [room, ...state];

  void update(ChatRoom room) {
    assert(contains(room), 'State does not contain room $room');
    final existing = state.firstWhere((r) => r.idBase58 == room.idBase58);

    final merged = (room.messages == null && existing.messages != null)
        ? ChatRoom(
            conversationId: room.conversationId,
            lastMessageSenderId: room.lastMessageSenderId,
            lastMessageIndex: existing.lastMessageIndex,
            name: room.name,
            lastMessageTime: room.lastMessageTime,
            lastMessagePreview: room.lastMessagePreview,
            messages: existing.messages,
            unreadCount: room.unreadCount,
            createdAt: room.createdAt,
            isDirectChat: room.isDirectChat,
            members: room.members,
            revisionNumber: room.revisionNumber,
            status: room.status,
          )
        : room;
    final filtered = state.where((r) => r.idBase58 != room.idBase58);
    state = [merged, ...filtered];
  }

  bool contains(ChatRoom room) =>
      state.any((r) => r.idBase58 == room.idBase58);
}
