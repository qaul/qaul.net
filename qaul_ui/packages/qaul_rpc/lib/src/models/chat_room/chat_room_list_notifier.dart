part of 'chat_room.dart';

class ChatRoomListNotifier extends Notifier<List<ChatRoom>> {
  @override
  List<ChatRoom> build() => [];

  void add(ChatRoom room) => state = [room, ...state];

  void update(ChatRoom room) {
    assert(contains(room), 'State does not contain room $room');
    final filtered = state.where((r) => r != room);
    state = [room, ...filtered];
  }

  bool contains(ChatRoom room) => state.contains(room);
}
