import 'dart:typed_data';

import 'package:flutter_test/flutter_test.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/qaul_rpc.dart';

void main() {
  final conversationId = Uint8List.fromList('room1'.codeUnits);

  Message message(int index) => Message(
        senderId: Uint8List.fromList('sender'.codeUnits),
        messageId: Uint8List.fromList('msg$index'.codeUnits),
        content: const TextMessageContent('text'),
        index: index,
        sentAt: DateTime(2020, 1, 1),
        receivedAt: DateTime(2020, 1, 1),
      );

  ChatRoom room({
    List<Message>? messages,
    int? lastMessageIndex,
  }) =>
      ChatRoom(
        conversationId: conversationId,
        name: 'Room',
        messages: messages,
        lastMessageIndex: lastMessageIndex,
      );

  group('ChatRoom Equatable props (idBase58, lastMessageIndex)', () {
    test('rooms with same idBase58 and same lastMessageIndex are equal', () {
      final a = room(messages: [message(1)], lastMessageIndex: 1);
      final b = room(messages: [message(1)], lastMessageIndex: 1);
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });

    test('rooms with same idBase58 but different lastMessageIndex are not equal', () {
      final a = room(messages: [message(1)], lastMessageIndex: 1);
      final b = room(messages: [message(1), message(2)], lastMessageIndex: 2);
      expect(a, isNot(equals(b)));
    });

    test('room with lastMessageIndex null vs set triggers inequality so UI can rebuild', () {
      final withoutMessages = room(messages: null, lastMessageIndex: null);
      final withMessages = room(messages: [message(0)], lastMessageIndex: 0);
      expect(withoutMessages, isNot(equals(withMessages)));
    });
  });

  group('ChatRoomListNotifier identify by idBase58', () {
    test('contains returns true when list has room with same idBase58', () {
      final container = ProviderContainer();
      addTearDown(container.dispose);
      final notifier = container.read(chatRoomsProvider.notifier);

      final roomA = room(messages: [message(1)], lastMessageIndex: 1);
      notifier.add(roomA);

      final roomB = room(messages: [message(1), message(2)], lastMessageIndex: 2);
      expect(notifier.contains(roomB), isTrue);
    });

    test('update replaces room with same idBase58', () {
      final container = ProviderContainer();
      addTearDown(container.dispose);
      final notifier = container.read(chatRoomsProvider.notifier);

      final roomA = room(messages: [message(1)], lastMessageIndex: 1);
      notifier.add(roomA);
      expect(container.read(chatRoomsProvider).single.messages!.length, 1);

      final roomB = room(messages: [message(1), message(2)], lastMessageIndex: 2);
      notifier.update(roomB);

      final list = container.read(chatRoomsProvider);
      expect(list.length, 1);
      expect(list.single.idBase58, roomA.idBase58);
      expect(list.single.messages!.length, 2);
      expect(list.single.lastMessageIndex, 2);
    });

    test('contains returns false when no room has that idBase58', () {
      final container = ProviderContainer();
      addTearDown(container.dispose);
      final notifier = container.read(chatRoomsProvider.notifier);

      notifier.add(room(messages: [message(1)], lastMessageIndex: 1));

      final otherRoom = ChatRoom(
        conversationId: Uint8List.fromList('other'.codeUnits),
        name: 'Other',
      );
      expect(notifier.contains(otherRoom), isFalse);
    });
  });
}
