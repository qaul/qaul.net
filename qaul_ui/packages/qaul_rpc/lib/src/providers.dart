import 'package:hooks_riverpod/hooks_riverpod.dart';

import '../qaul_rpc.dart';
import 'models/chat_room_list_notifier.dart';
import 'models/feed_post_list_notifier.dart';
import 'models/file_history_entity_notifier.dart';
import 'models/group_invite.dart';

final qaulWorkerProvider =
    Provider<LibqaulWorker>((ref) => LibqaulWorker(ref.read));

final nodeInfoProvider = StateProvider<NodeInfo?>((_) => null);

final defaultUserProvider = StateProvider<User?>((ref) => null);

final feedMessagesProvider =
    StateNotifierProvider<FeedPostListNotifier, List<FeedPost>>(
  (ref) => FeedPostListNotifier(messages: []),
);

final usersProvider = StateNotifierProvider<UserListNotifier, List<User>>(
  (ref) => UserListNotifier(users: const []),
);

final connectedNodesProvider = StateProvider<List<InternetNode>>((ref) => []);

final chatRoomsProvider =
    StateNotifierProvider<ChatRoomListNotifier, List<ChatRoom>>(
        (ref) => ChatRoomListNotifier());

final currentOpenChatRoom = StateProvider<ChatRoom?>((ref) => null);

final libqaulLogsStoragePath = StateProvider<String?>((ref) => null);

final bleStatusProvider = StateProvider<BleConnectionStatus?>((_) => null);

final fileHistoryEntitiesProvider =
    StateNotifierProvider<FileHistoryEntityNotifier, List<FileHistoryEntity>>(
  (ref) => FileHistoryEntityNotifier(files: const []),
);

final groupInvitesProvider =
    StateNotifierProvider<GroupInviteListNotifier, List<GroupInvite>>(
        (_) => GroupInviteListNotifier());
