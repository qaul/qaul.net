part of 'chat_tab_test.dart';

final defaultUser = User(
  name: 'defaultUser',
  id: Uint8List.fromList('defaultUserid'.codeUnits),
);

final otherUser = User(
  name: 'otherUser',
  id: Uint8List.fromList('otherUserId'.codeUnits),
);

ChatRoom buildGroupChat({
  List<Message>? messages,
  ChatRoomStatus status = ChatRoomStatus.active,
}) => ChatRoom(
  name: 'Group Chat',
  messages: messages,
  conversationId: Uint8List.fromList('groupId'.codeUnits),
  isDirectChat: false,
  status: status,
  members: [
    ChatRoomUser(defaultUser, joinedAt: DateTime(2000)),
    ChatRoomUser(otherUser, joinedAt: DateTime(2000)),
  ],
);

ChatRoom buildDirectChat({
  List<Message>? messages,
  ChatRoomStatus status = ChatRoomStatus.active,
}) => ChatRoom(
  name: otherUser.name,
  messages: messages,
  conversationId: Uint8List.fromList('directId'.codeUnits),
  isDirectChat: true,
  status: status,
  members: [
    ChatRoomUser(defaultUser, joinedAt: DateTime(2000)),
    ChatRoomUser(otherUser, joinedAt: DateTime(2000)),
  ],
);
