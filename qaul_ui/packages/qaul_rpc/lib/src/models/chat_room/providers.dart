part of 'chat_room.dart';

final chatRoomsProvider =
    NotifierProvider<ChatRoomListNotifier, List<ChatRoom>>(
        ChatRoomListNotifier.new);

final currentOpenChatRoom = StateProvider<ChatRoom?>((ref) => null);
