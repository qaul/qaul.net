part of 'chat_room.dart';

class ChatRoomListNotifier extends Notifier<List<ChatRoom>> {
  @override
  List<ChatRoom> build() => [];

  void clear() => state = [];

  void append(List<ChatRoom> rooms) {
    final existingIds = state.map((r) => r.idBase58).toSet();
    final newRooms =
        rooms.where((r) => !existingIds.contains(r.idBase58)).toList();
    if (newRooms.isEmpty) return;
    state = [...state, ...newRooms];
  }

  void add(ChatRoom room) => state = [room, ...state];

  void mergeOrderedFromBackend(List<ChatRoom> rooms) {
    final currentById = {for (final r in state) r.idBase58: r};
    final backendIds = rooms.map((r) => r.idBase58).toSet();
    final mergedOrdered = rooms.map((room) {
      final existing = currentById[room.idBase58];
      final isPartialUpdate = room.messages == null && existing?.messages != null;
      if (!isPartialUpdate) return room;
      return room.copyWith(
        lastMessageIndex: existing!.lastMessageIndex,
        messages: existing.messages,
      );
    }).toList();

    final remaining = state.where((r) => !backendIds.contains(r.idBase58));
    state = [...mergedOrdered, ...remaining];
  }

  void replacePreservingOrder(ChatRoom room) {
    assert(contains(room), 'State does not contain room $room');
    final existingIndex = state.indexWhere((r) => r.idBase58 == room.idBase58);
    if (existingIndex < 0) return;
    final existing = state[existingIndex];
    final isPartialUpdate = room.messages == null && existing.messages != null;
    final merged = isPartialUpdate
        ? room.copyWith(
            lastMessageIndex: existing.lastMessageIndex,
            messages: existing.messages,
          )
        : room;
    final next = [...state];
    next[existingIndex] = merged;
    state = next;
  }

  void update(ChatRoom room) {
    assert(contains(room), 'State does not contain room $room');
    final existing = state.firstWhere((r) => r.idBase58 == room.idBase58);

    final isPartialUpdate = room.messages == null && existing.messages != null;

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
