part of 'chat_tab_test.dart';

final defaultUser = User(
  name: 'defaultUser',
  id: Uint8List.fromList('defaultUserid'.codeUnits),
);

final otherUser = User(
  name: 'otherUser',
  id: Uint8List.fromList('otherUserId'.codeUnits),
);

ChatRoom buildGroupChat({List<Message>? messages}) => ChatRoom(
  name: 'Group Chat',
  messages: messages,
  conversationId: Uint8List.fromList('groupId'.codeUnits),
  isDirectChat: false,
  members: [
    ChatRoomUser(defaultUser, joinedAt: DateTime(2000)),
    ChatRoomUser(defaultUser, joinedAt: DateTime(2000)),
  ],
);
