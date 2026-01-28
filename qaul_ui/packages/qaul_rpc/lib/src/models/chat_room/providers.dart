part of 'chat_room.dart';

final chatRoomsProvider =
    NotifierProvider<ChatRoomListNotifier, List<ChatRoom>>(
        ChatRoomListNotifier.new);

final currentOpenChatRoom =
    NotifierProvider<CurrentOpenChatRoomNotifier, ChatRoom?>(
        CurrentOpenChatRoomNotifier.new);

class CurrentOpenChatRoomNotifier extends Notifier<ChatRoom?> {
  @override
  ChatRoom? build() => null;
}
