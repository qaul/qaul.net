import 'package:hooks_riverpod/hooks_riverpod.dart';

import '../models/feed_post.dart';

// Maybe using a Stream would be simpler. Just creating this class to facilitate manipulating StateNotifierProvider
class FeedPostListNotifier extends StateNotifier<List<FeedPost>> {
  FeedPostListNotifier({List<FeedPost>? messages}) : super(messages ?? []);

  void add(FeedPost message) {
    state = [message, ...state];
  }

  bool contains(FeedPost message) {
    return !state
        .indexWhere(
          (m) =>
              m.senderIdBase58 == message.senderIdBase58 &&
              m.messageIdBase58 == message.messageIdBase58,
        )
        .isNegative;
  }
}
