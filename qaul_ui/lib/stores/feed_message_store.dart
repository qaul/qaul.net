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
    ref.listen(usersStoreProvider, (_, _) => _asyncInit());
    _asyncInit();
    return [];
  }

  void _asyncInit() async {
    final messages = ref.read(publicMessagesProvider).data;
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

  static const int _pageSize = 50;

  Future<void> refreshPublic() async {
    final worker = ref.read(qaulWorkerProvider);
    await worker.requestPublicMessages(
      offset: 0,
      limit: _pageSize,
    );
  }

  Future<void> loadMorePublic() async {
    final paginated = ref.read(publicMessagesProvider);
    final pagination = paginated.pagination;
    if (pagination != null && !pagination.hasMore) return;
    final offset = pagination != null
        ? pagination.offset + pagination.limit
        : paginated.data.length;
    final worker = ref.read(qaulWorkerProvider);
    await worker.requestPublicMessages(
      offset: offset,
      limit: _pageSize,
    );
  }

  Future<void> sendMessage(String messageText) async {
    final worker = ref.read(qaulWorkerProvider);
    await worker.sendPublicMessage(messageText);
  }
}

// 
