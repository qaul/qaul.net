import 'package:hooks_riverpod/hooks_riverpod.dart';
import '../../qaul_rpc.dart';

class ChatRoomListNotifier extends StateNotifier<List<ChatRoom>> {
  ChatRoomListNotifier({List<ChatRoom>? rooms}) : super(rooms ?? []);

  void add(ChatRoom room) => state = [room, ...state];

  void update(ChatRoom room) {
    assert(contains(room), 'State does not contain room $room');
    final filtered = state.where((r) => r != room);
    state = [room, ...filtered];
  }

  bool contains(ChatRoom room) => state.contains(room);
}
