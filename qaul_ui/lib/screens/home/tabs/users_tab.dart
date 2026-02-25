part of 'tab.dart';

class _Users extends BaseTab {
  const _Users({super.key});

  @override
  _UsersState createState() => _UsersState();
}

class _UsersState extends _BaseTabState<_Users> {
  static const _pageSize = 50;
  late final ScrollController _scrollController;
  bool _isLoadingMore = false;
  bool _hasMore = true;
  int _currentOffset = 0;

  @override
  void initState() {
    super.initState();
    _scrollController = ScrollController();
    _scrollController.addListener(_onScroll);
    WidgetsBinding.instance.addPostFrameCallback((_) {
      _refreshUsers();
    });
  }

  @override
  void dispose() {
    _scrollController.removeListener(_onScroll);
    _scrollController.dispose();
    super.dispose();
  }

  void _onScroll() {
    if (_scrollController.position.pixels >=
        _scrollController.position.maxScrollExtent * 0.8) {
      _loadMoreUsers();
    }
  }

  void _updatePaginationState() {
    final paginationState = ref.read(usersProvider).pagination;
    if (paginationState != null) {
      setState(() => _hasMore = paginationState.hasMore);
      _currentOffset = paginationState.offset + paginationState.limit;
      return;
    }
    _currentOffset += _pageSize;
  }

  Future<void> _loadMoreUsers() async {
    if (_isLoadingMore || !_hasMore) return;

    setState(() => _isLoadingMore = true);
    try {
      final worker = ref.read(qaulWorkerProvider);
      await worker.getUsers(offset: _currentOffset, limit: _pageSize);
      _updatePaginationState();
    } finally {
      if (mounted) setState(() => _isLoadingMore = false);
    }
  }

  Future<void> _refreshUsers() async {
    _currentOffset = 0;
    setState(() => _hasMore = true);
    final worker = ref.read(qaulWorkerProvider);
    await worker.getUsers(offset: 0, limit: _pageSize);
    _updatePaginationState();
  }

  @override
  Widget build(BuildContext context) {
    super.build(context);

    ref.listen(usersProvider, (previous, next) {
      if (next.pagination != null && !next.pagination!.hasMore) {
        setState(() => _hasMore = false);
      }
    });

    final l10n = AppLocalizations.of(context)!;
    return Scaffold(
      body: LoadingDecorator(
        isLoading: _isLoadingMore,
        child: RefreshIndicator(
            onRefresh: () async => await _refreshUsers(),
            child: SearchUserDecorator(builder: (_, users) {
              return EmptyStateTextDecorator(
                l10n.emptyUsersList,
                isEmpty: users.isEmpty,
                child: ListView.separated(
                  controller: _scrollController,
                  physics: const AlwaysScrollableScrollPhysics(),
                  itemCount: users.length,
                  separatorBuilder: (_, _) => const Divider(height: 12.0),
                  itemBuilder: (_, i) {
                    final user = users[i];
                  var theme = Theme.of(context).textTheme;
                  var hasConnections =
                      user.availableTypes != null && user.availableTypes!.isNotEmpty;

                  var userId = Text(
                    'ID: ${user.idBase58}',
                    style: theme.bodySmall!.copyWith(fontSize: 10),
                  );
                  var content = !hasConnections
                      ? userId
                      : Column(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            userId,
                            if (hasConnections) ...[
                              const SizedBox(height: 8),
                              _AvailableConnections(user: user),
                            ],
                          ],
                        );

                  return DisabledStateDecorator(
                    isDisabled: user.isBlocked ?? false,
                    ignorePointer: false,
                    child: QaulListTile.user(
                      user,
                      content: content,
                      isThreeLine: hasConnections,
                      trailingIcon: (user.isVerified ?? false)
                          ? const Icon(Icons.verified_user)
                          : const SizedBox(),
                      tapRoutesToDetailsScreen: true,
                    ),
                  );
                  },
                ),
              );
            }),
          ),
        ),
      ),
    );
  }
}

class _AvailableConnections extends StatelessWidget {
  const _AvailableConnections({
    required this.user,
  });

  final User user;

  @override
  Widget build(BuildContext context) {
    const space = SizedBox(width: 4);
    return IconTheme(
      data: Theme.of(context).iconTheme.copyWith(size: 18.0),
      child: Row(
        children: [
          if (_hasInternet) ...[const Icon(CupertinoIcons.globe), space],
          if (_hasLan) ...[const Icon(Icons.wifi), space],
          if (_hasLocal) ...[const Icon(Icons.cable), space],
          if (_hasBluetooth) ...[const Icon(Icons.bluetooth)],
        ],
      ),
    );
  }

  bool get _hasBluetooth => user.availableTypes!.keys.contains(ConnectionType.ble);

  bool get _hasLan => user.availableTypes!.keys.contains(ConnectionType.lan);

  bool get _hasLocal => user.availableTypes!.keys.contains(ConnectionType.local);

  bool get _hasInternet => user.availableTypes!.keys.contains(ConnectionType.internet);
}
