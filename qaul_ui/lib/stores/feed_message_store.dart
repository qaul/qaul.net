part of 'stores.dart';

final feedMessageStoreProvider =
    NotifierProvider<FeedMessageStore, List<FeedMessage>>(FeedMessageStore.new);

class FeedMessage extends PublicPost {
  final User author;
  final String sentTimestamp;

  FeedMessage(PublicPost message, this.author, this.sentTimestamp)
      : super(
          senderId: message.senderId,
          index: message.index,
          senderIdBase58: message.senderIdBase58,
          messageId: message.messageId,
          messageIdBase58: message.messageIdBase58,
          content: message.content,
          sendTime: message.sendTime,
          receiveTime: message.receiveTime,
        );
}

class FeedMessageStore extends Notifier<List<FeedMessage>> {
  @override
  build() {
    ref.listen(publicMessagesProvider, (_, _) => _asyncInit());
    _asyncInit();
    return [];
  }

  void _asyncInit() async {
    final messages = ref.read(publicMessagesProvider);
    final knownUsers = ref.read(usersStoreProvider);
    final authorById = <String, User>{
      for (final u in knownUsers) u.idBase58: u,
    };
    final usersStore = ref.read(usersStoreProvider.notifier);
    final feedMessages = <FeedMessage>[];

    for (final m in messages) {
      if (m.senderIdBase58 == null) continue;
      User? author = authorById[m.senderIdBase58];
      if (author == null) {
        author = await usersStore.getByUserID(m.senderIdBase58!);
        if (author != null) authorById[m.senderIdBase58!] = author;
      }
      if (author == null) continue;
      final sentAt = describeFuzzyTimestamp(
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

// 
