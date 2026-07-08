import 'package:collection/collection.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

import '../generated/services/chat/chat.pb.dart';
import '../models/models.dart';

const _byteListEquality = ListEquality<int>();

void applyChatConversationList(
  Ref ref,
  ChatConversationList conversation,
) {
  final state = ref.read(chatRoomsProvider.notifier);
  final room = ref.read(chatRoomsProvider).firstWhereOrNull(
        (r) => _byteListEquality.equals(
          r.conversationId,
          conversation.groupId,
        ),
      );
  final currentOpenRoom = ref.read(currentOpenChatRoom.notifier);

  final openRoom = currentOpenRoom.state;
  final isOpenRoom = openRoom != null &&
      _byteListEquality.equals(openRoom.conversationId, conversation.groupId);

  final source = room ?? (isOpenRoom ? openRoom : null);
  if (source == null) return;

  final merged = source.copyWithMessages(conversation);

  if (room != null) {
    state.replacePreservingOrder(merged);
  }

  if (isOpenRoom) {
    currentOpenRoom.state = merged;
  }
}
