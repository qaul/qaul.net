part of 'abstract_rpc_module_translator.dart';

class FeedTranslator extends RpcModuleTranslator {
  @override
  Modules get type => Modules.FEED;

  @override
  Future<RpcTranslatorResponse?> decodeMessageBytes(List<int> data, Ref ref) async {
    final message = Feed.fromBuffer(data);
    switch (message.whichMessage()) {
      case Feed_Message.received:
        final received = message.ensureReceived();
        final posts = received.feedMessage.map((msg) => msg.toModelMessage).toList();
        PaginationState? pagination;
        if (received.hasPagination()) {
          final p = received.pagination;
          pagination = PaginationState(
            hasMore: p.hasMore,
            total: p.total,
            offset: p.offset,
            limit: p.limit,
          );
        }
        return RpcTranslatorResponse(
          type,
          PaginatedData<PublicPost>(data: posts, pagination: pagination),
        );
      default:
        return super.decodeMessageBytes(data, ref);
    }
  }

  @override
  Future<void> processResponse(RpcTranslatorResponse res, Ref ref) async {
    if (res.module != type || res.data is! PaginatedData<PublicPost>) return;
    final paginated = res.data as PaginatedData<PublicPost>;
    final notifier = ref.read(publicMessagesProvider.notifier);
    final isFirstPage = paginated.pagination?.offset == 0;
    if (isFirstPage) {
      notifier.replaceAll(paginated.data, pagination: paginated.pagination);
    } else {
      notifier.appendMany(paginated.data);
      notifier.setPagination(paginated.pagination);
    }
  }
}
