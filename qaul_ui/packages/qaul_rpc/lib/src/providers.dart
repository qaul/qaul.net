import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/src/models/feed_post_list_notifier.dart';

import '../qaul_rpc.dart';

final qaulWorkerProvider = Provider<LibqaulWorker>((ref) => LibqaulWorker(ref.read));

final defaultUserProvider = StateProvider<User?>((ref) => null);

final feedMessagesProvider = StateNotifierProvider<FeedPostListNotifier, List<FeedPost>>(
  (ref) => FeedPostListNotifier(messages: []),
);

final usersProvider = StateNotifierProvider<UserListNotifier, List<User>>(
  (ref) => UserListNotifier(users: const []),
);

final connectedNodesProvider = StateProvider<List<InternetNode>>((ref) => []);

final chatRoomsProvider = StateProvider<List<ChatRoom>>((ref) => []);
