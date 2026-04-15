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
  List<PublicPost> _posts = [];
  PaginationState? _pagination;

  PaginationState? get pagination => _pagination;

  @override
  List<FeedMessage> build() {
    ref.listen(usersStoreProvider, (_, __) {
      Future.microtask(_rebuildFeedMessages);
    });
    return [];
  }

  /// Applies a feed RPC response: merges [data.posts] into [_posts] and
  /// rebuilds [state] with resolved authors.
  Future<void> applyPaginatedPosts(PaginatedPosts data) async {
    final posts = data.posts;
    final pagination = data.pagination;
    if (pagination != null) {
      _pagination = pagination;
      if (pagination.offset == 0) {
        _posts = List<PublicPost>.from(posts);
      } else {
        final existingIds = _posts.map((m) => m.messageIdBase58).toSet();
        final newPosts = posts
            .where((p) => !existingIds.contains(p.messageIdBase58))
            .toList();
        if (newPosts.isNotEmpty) {
          _posts = [..._posts, ...newPosts];
        }
      }
    } else {
      final existingIds = _posts.map((m) => m.messageIdBase58).toSet();
      final newPosts = posts
          .where((p) => !existingIds.contains(p.messageIdBase58))
          .toList();
      if (newPosts.isNotEmpty) {
        _posts = [...newPosts, ..._posts];
      }
    }
    await _rebuildFeedMessages();
  }

  Future<void> _rebuildFeedMessages() async {
    final knownUsers = ref.read(usersStoreProvider);
    final authorById = <String, User>{
      for (final u in knownUsers) u.idBase58: u,
    };
    final usersStore = ref.read(usersStoreProvider.notifier);
    final feedMessages = <FeedMessage>[];

    for (final m in _posts) {
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
    final result = await worker.requestPublicMessages(
      lastIndex: indexes.isEmpty ? null : indexes.reduce(math.max),
    );
    if (result != null) await applyPaginatedPosts(result);
  }

  Future<PaginatedPosts?> loadMore(int offset, {int limit = 50}) async {
    final worker = ref.read(qaulWorkerProvider);
    final result = await worker.requestPublicMessages(offset: offset, limit: limit);
    if (result != null) await applyPaginatedPosts(result);
    return result;
  }

  Future<bool> sendMessage(String messageText) async {
    final worker = ref.read(qaulWorkerProvider);
    return worker.sendPublicMessage(messageText);
  }
}
