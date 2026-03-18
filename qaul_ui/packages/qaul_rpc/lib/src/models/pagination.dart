class PaginationState {
  const PaginationState({
    required this.hasMore,
    required this.total,
    required this.offset,
    required this.limit,
  });

  final bool hasMore;
  final int total;
  final int offset;
  final int limit;
}

class PaginatedData<T> {
  const PaginatedData({
    required this.data,
    required this.pagination,
  });

  final List<T> data;
  final PaginationState? pagination;
}

