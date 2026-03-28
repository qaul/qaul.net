part of 'chat_room.dart';

class ChatRoomListNotifier extends Notifier<List<ChatRoom>> {
  @override
  List<ChatRoom> build() => [];

  void add(ChatRoom room) => state = [room, ...state];

  void update(ChatRoom room) {
    assert(contains(room), 'State does not contain room $room');
    final existing = state.firstWhere((r) => r.idBase58 == room.idBase58);

    final isPartialUpdate = room.messages == null && existing.messages != null;

    // if room only has metadata updates, keep the existing messages
    final merged = isPartialUpdate
        ? room.copyWith(
            lastMessageIndex: existing.lastMessageIndex,
            messages: existing.messages,
          )
        : room;
    final filtered = state.where((r) => r.idBase58 != room.idBase58);
    state = [merged, ...filtered];
  }

  bool contains(ChatRoom room) => state.any((r) => r.idBase58 == room.idBase58);
}
