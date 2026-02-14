part of 'stores.dart';

final feedMessageStoreProvider =
    NotifierProvider<FeedMessageStore, List<FeedMessage>>(FeedMessageStore.new);

class FeedMessage extends PublicPost {
  final User author;
  final String sentTimestamp;

  // TODO
  FeedMessage(PublicPost message, this.author, this.sentTimestamp) : super();
}

class FeedMessageStore extends Notifier<List<FeedMessage>> {
  @override
  build() {
    _asyncInit();
    return [];
  }

  void _asyncInit() async {
    // TODO verify if should be .listen() instead of .watch()
    final messages = ref.watch(publicMessagesProvider);
    final users = ref.watch(usersStoreProvider);
    final messagesWithUsers = messages.where(
      (m) => users.map((u) => u.idBase58).contains(m.senderIdBase58 ?? ''),
    );

    final feedMessages = <FeedMessage>[];

    for (final m in messagesWithUsers) {
      if (m.senderIdBase58 == null) continue;
      final author = await ref
          .read(usersStoreProvider.notifier)
          .getByUserID(m.senderIdBase58!);
      if (author == null) continue;
      var sentAt = describeFuzzyTimestamp(
        m.sendTime,
        locale: Locale.parse(Intl.defaultLocale ?? 'en'),
      );
      feedMessages.add(FeedMessage(m, author, sentAt));
    }
    state = feedMessages;
  }

  Future<void> refreshPublic() async {
    final worker = ref.read(qaulWorkerProvider);
    final indexes = state.map((e) => e.index ?? 1);
    await worker.requestPublicMessages(
      lastIndex: indexes.isEmpty ? null : indexes.reduce(math.max),
    );
  }

  Future<void> sendMessage(String messageText) async {
    final worker = ref.read(qaulWorkerProvider);
    await worker.sendPublicMessage(messageText);
  }
}
