part of 'chat_room.dart';

final chatRoomsProvider =
    StateNotifierProvider<ChatRoomListNotifier, List<ChatRoom>>(
        (ref) => ChatRoomListNotifier());

final currentOpenChatRoom = StateProvider<ChatRoom?>((ref) => null);
