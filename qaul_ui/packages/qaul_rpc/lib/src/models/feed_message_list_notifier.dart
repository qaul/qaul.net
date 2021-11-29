import 'package:hooks_riverpod/hooks_riverpod.dart';

import '../generated/services/feed/feed.pb.dart' as pb;
import '../models/feed_message.dart' as models;

// Maybe using a Stream would be simpler. Just creating this class to facilitate manipulating StateNotifierProvider
class FeedMessageListNotifier extends StateNotifier<List<models.FeedMessage>> {
  FeedMessageListNotifier({List<models.FeedMessage>? messages})
      : super(messages ?? []);

  void add(models.FeedMessage message) {
    state = [message, ...state];
  }

  bool contains(pb.FeedMessage message) {
    return !state
        .indexWhere(
          (m) =>
              m.senderIdBase58 == message.senderIdBase58 &&
              m.messageIdBase58 == message.messageIdBase58,
        )
        .isNegative;
  }
}
