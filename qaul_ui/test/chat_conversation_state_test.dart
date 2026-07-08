import 'dart:typed_data';

import 'package:fixnum/fixnum.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:qaul_rpc/src/generated/services/chat/chat.pb.dart';
import 'package:qaul_rpc/src/internal/chat_conversation_state.dart';

final _conversationApplierProvider =
    NotifierProvider<_ConversationApplier, void>(_ConversationApplier.new);

class _ConversationApplier extends Notifier<void> {
  @override
  void build() {}

  void apply(ChatConversationList conversation) {
    applyChatConversationList(ref, conversation);
  }
}

void main() {
  final conversationId = Uint8List.fromList('group-1'.codeUnits);

  ChatRoom room(String name) =>
      ChatRoom(conversationId: conversationId, name: name, isDirectChat: false);

  ChatConversationList conversation(int index) => ChatConversationList(
    groupId: conversationId,
    messageList: [
      ChatMessage(
        index: Int64(index),
        senderId: Uint8List.fromList('sender'.codeUnits),
        messageId: Uint8List.fromList('message-$index'.codeUnits),
      ),
    ],
  );

  test('applies fetched messages to chatRoomsProvider and open room', () {
    final container = ProviderContainer();
    addTearDown(container.dispose);

    final storedRoom = room('Stored group');
    final openRoom = room('Open group');
    container.read(chatRoomsProvider.notifier).add(storedRoom);
    container.read(currentOpenChatRoom.notifier).state = openRoom;

    container
        .read(_conversationApplierProvider.notifier)
        .apply(conversation(7));

    final updatedStoredRoom = container.read(chatRoomsProvider).single;
    final updatedOpenRoom = container.read(currentOpenChatRoom)!;

    expect(updatedStoredRoom.idBase58, storedRoom.idBase58);
    expect(updatedStoredRoom.messages, hasLength(1));
    expect(updatedStoredRoom.lastMessageIndex, 7);
    expect(updatedOpenRoom.idBase58, openRoom.idBase58);
    expect(updatedOpenRoom.messages, hasLength(1));
    expect(updatedOpenRoom.lastMessageIndex, 7);
  });
}
