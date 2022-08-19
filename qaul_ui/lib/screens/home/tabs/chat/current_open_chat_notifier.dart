import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/qaul_rpc.dart';

final uiOpenChatProvider = StateNotifierProvider<CurrentOpenChatNotifier, ChatRoom?>(
  (ref) => CurrentOpenChatNotifier(),
);

class CurrentOpenChatNotifier extends StateNotifier<ChatRoom?> {
  CurrentOpenChatNotifier({ChatRoom? room}) : super(room);

  void setCurrent(ChatRoom room) => state = room;

  void close() => state = null;
}
