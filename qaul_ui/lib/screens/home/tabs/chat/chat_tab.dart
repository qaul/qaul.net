part of '../tab.dart';

class _Chat extends BaseTab {
  const _Chat({super.key});

  @override
  _ChatState createState() => _ChatState();
}

class _ChatState extends _BaseTabState<_Chat> {
  final _log = Logger('BaseTab.chat');
  static const _pageSize = ChatRoomsStore.defaultPageSize;
  late final ScrollController _scrollController;
  bool _isLoadingMore = false;
  bool _hasMoreChatRooms = true;
  bool _hasMoreInvites = true;
  int _chatRoomsOffset = 0;
  int _invitesOffset = 0;

  @override
  void initState() {
    super.initState();
    _scrollController = ScrollController();
    _scrollController.addListener(_onScroll);
    WidgetsBinding.instance.addPostFrameCallback((_) async {
      await ref.read(chatNotificationControllerProvider).initialize();
      await _fetchFirstPage();
    });
  }

  @override
  void dispose() {
    _scrollController.removeListener(_onScroll);
    _scrollController.dispose();
    super.dispose();
  }

  void _onScroll() {
    if (_scrollController.position.pixels <
        _scrollController.position.maxScrollExtent * 0.8) {
      return;
    }
    final search = ref.read(chatRoomsSearchProvider);
    if (search.isActive) {
      _loadMoreSearchResults();
    } else {
      _loadMore();
    }
  }

  Future<void> _loadMoreSearchResults() async {
    final search = ref.read(chatRoomsSearchProvider);
    if (_isLoadingMore || !search.hasMore || search.isLoading) return;

    setState(() => _isLoadingMore = true);
    try {
      await ref.read(chatRoomsSearchProvider.notifier).loadMore();
    } finally {
      if (mounted) setState(() => _isLoadingMore = false);
    }
  }

  Future<void> _fetchFirstPage() async {
    _chatRoomsOffset = 0;
    _invitesOffset = 0;
    setState(() {
      _hasMoreChatRooms = true;
      _hasMoreInvites = true;
    });

    final groups = ref.read(chatRoomsStoreProvider.notifier);
    final results = await Future.wait([
      groups.getChatRooms(offset: 0, limit: _pageSize),
      groups.getGroupInvites(offset: 0, limit: _pageSize),
    ]);
    if (!mounted) return;

    _updatePaginationFromRoomsResult(results.first as PaginatedChatRooms?);
    _updatePaginationFromInvitesResult(results.last as PaginatedGroupInvites?);
  }

  Future<void> _refreshChatsAndInvites() async {
    if (ref.read(chatRoomsSearchProvider).isActive) {
      await ref.read(chatRoomsSearchProvider.notifier).refresh();
      return;
    }
    // Pull-to-refresh resets to a fresh first page. Clear local state so any
    // rooms/invites that no longer exist on the backend drop out of the UI —
    // the offset=0 translator path now merges instead of replacing.
    ref.read(chatRoomsProvider.notifier).clear();
    ref.read(groupInvitesProvider.notifier).clear();
    await _fetchFirstPage();
  }

  Future<void> _loadMore() async {
    if (_isLoadingMore || (!_hasMoreChatRooms && !_hasMoreInvites)) return;
    setState(() => _isLoadingMore = true);
    try {
      final groups = ref.read(chatRoomsStoreProvider.notifier);
      final futures = <Future<dynamic>>[];
      if (_hasMoreChatRooms) {
        futures.add(
          groups.getChatRooms(offset: _chatRoomsOffset, limit: _pageSize),
        );
      }
      if (_hasMoreInvites) {
        futures.add(
          groups.getGroupInvites(offset: _invitesOffset, limit: _pageSize),
        );
      }

      final results = await Future.wait(futures);
      for (final result in results) {
        if (result == null) continue;
        if (result is PaginatedChatRooms) {
          _updatePaginationFromRoomsResult(result);
        } else if (result is PaginatedGroupInvites) {
          _updatePaginationFromInvitesResult(result);
        }
      }
    } finally {
      if (mounted) setState(() => _isLoadingMore = false);
    }
  }

  void _updatePaginationFromRoomsResult(PaginatedChatRooms? result) {
    final paginationState = result?.pagination;
    if (paginationState == null) {
      // Backend omitted pagination metadata: treat as end-of-list rather than
      // optimistically advancing the offset (which would cause infinite empty
      // page fetches on scroll).
      setState(() => _hasMoreChatRooms = false);
      return;
    }
    setState(() => _hasMoreChatRooms = paginationState.hasMore);
    _chatRoomsOffset = paginationState.offset + paginationState.limit;
  }

  void _updatePaginationFromInvitesResult(PaginatedGroupInvites? result) {
    final paginationState = result?.pagination;
    if (paginationState == null) {
      setState(() => _hasMoreInvites = false);
      return;
    }
    setState(() => _hasMoreInvites = paginationState.hasMore);
    _invitesOffset = paginationState.offset + paginationState.limit;
  }

  @override
  Widget build(BuildContext context) {
    super.build(context);

    final searchController = useTextEditingController();
    useEffect(() {
      return () => ref.read(chatRoomsSearchProvider.notifier).clear();
    }, const []);

    final defaultUser = ref.watch(defaultUserProvider)!;
    final users = ref.watch(usersStoreProvider);
    final chatRooms = ref.watch(chatRoomsProvider);
    final groupInvites = ref.watch(groupInvitesProvider);
    final roomSearch = ref.watch(chatRoomsSearchProvider);
    final currentOpenChat = ref.watch(currentOpenChatRoom);

    final blockedIds = users
        .where((u) => u.isBlocked ?? false)
        .map((u) => u.conversationId);
    final filteredRooms = chatRooms
        .where((m) => !blockedIds.contains(m.conversationId))
        .toList();
    final displayRooms =
        roomSearch.isActive ? roomSearch.results : filteredRooms;
    final showInvites = !roomSearch.isActive;
    final listItemCount =
        (showInvites ? groupInvites.length : 0) + displayRooms.length;
    final isListLoading = _isLoadingMore ||
        (roomSearch.isActive &&
            roomSearch.isLoading &&
            roomSearch.results.isEmpty);

    final mobile = Responsiveness.isMobile(
      context,
    ); // MediaQuery on HookState.init throws
    final setOpenChat = useCallback((ChatRoom room, [User? otherUser]) {
      if (mobile) {
        openChat(
          room,
          ref: ref,
          context: context,
          user: defaultUser,
          otherUser: otherUser,
        );
      } else {
        ref.read(currentOpenChatRoom.notifier).state = room;
      }
    }, [mobile]);

    final onNewChatFABPressed = useCallback(() async {
      final newChat = await Navigator.push(
        context,
        MaterialPageRoute(builder: (_) => const _CreateNewRoomDialog()),
      );
      if (newChat is User) {
        final newRoom = ChatRoom.blank(otherUser: newChat);
        setOpenChat(newRoom, newChat);
      } else if (newChat is ChatRoom) {
        setOpenChat(newChat);
      }
    }, [setOpenChat]);

    final l10n = AppLocalizations.of(context);

    final chatRoomsListView = CronTaskDecorator(
      schedule: const Duration(milliseconds: 2500),
      callback: () {
        if (ref.read(chatRoomsSearchProvider).isActive) return;
        ref.read(chatRoomsStoreProvider.notifier).pollChatRoomsAndInvites();
      },
      child: LoadingDecorator(
        isLoading: isListLoading,
        child: ChatRoomList(
          scrollController: _scrollController,
          searchHint: l10n!.searchChat,
          searchController: searchController,
          onQueryChanged: ref.read(chatRoomsSearchProvider.notifier).setQuery,
          onClear: () {
            searchController.clear();
            ref.read(chatRoomsSearchProvider.notifier).clear();
          },
          onRefresh: _refreshChatsAndInvites,
          isEmpty: listItemCount == 0,
          emptyMessage: l10n.emptyChatsList,
          itemCount: listItemCount,
          itemBuilder: (_, i) {
            final theme = Theme.of(context).textTheme;

            if (showInvites && i < groupInvites.length) {
              return _GroupInviteTile(invite: groupInvites[i]);
            }

            final roomIndex = showInvites ? i - groupInvites.length : i;
            final room = displayRooms[roomIndex];
            if (room.isGroupChatRoom) {
              return QaulListTile.group(
                room,
                unreadCount: room.unreadCount,
                content: _contentFromOverview(
                  room.lastMessagePreview,
                  theme,
                  room: room,
                  l10n: l10n,
                ),
                trailingMetadata: Row(
                  children: [
                    Text(
                      room.lastMessageTime == null
                          ? ''
                          : describeFuzzyTimestamp(
                              room.lastMessageTime!,
                              locale: Locale.parse(
                                Intl.defaultLocale ?? 'en',
                              ),
                            ),
                      style: theme.bodySmall!.copyWith(
                        fontStyle: FontStyle.italic,
                      ),
                    ),
                    const Icon(Icons.chevron_right),
                  ],
                ),
                onTap: () => setOpenChat(room),
              );
            }

            final otherUser = ref
                .read(usersStoreProvider.notifier)
                .otherUserInDirectRoom(room, defaultUser);

            if (otherUser == null) {
              _log.warning('single-person room with unknown otherUser');
              return const SizedBox.shrink();
            }

            return QaulListTile.user(
              otherUser,
              unreadCount: room.unreadCount,
              content: _contentFromOverview(
                room.lastMessagePreview,
                theme,
                room: room,
                l10n: l10n,
              ),
              trailingMetadata: Row(
                children: [
                  Text(
                    room.lastMessageTime == null
                        ? ''
                        : describeFuzzyTimestamp(
                            room.lastMessageTime!,
                            locale: Locale.parse(
                              Intl.defaultLocale ?? 'en',
                            ),
                          ),
                    style: theme.bodySmall!.copyWith(
                      fontStyle: FontStyle.italic,
                    ),
                  ),
                  const Icon(Icons.chevron_right),
                ],
              ),
              onTap: () => setOpenChat(room, otherUser),
              avatarTapRoutesToDetailsScreen: false,
            );
          },
        ),
      ),
    );

    final createChatButton = QaulFAB(
      size: 48,
      heroTag: 'chatTabFAB',
      tooltip: l10n.newChatTooltip,
      onPressed: onNewChatFABPressed,
      svgAsset: 'assets/icons/comment.svg',
    );

    return ResponsiveLayout(
      mobileBody: Scaffold(
        body: chatRoomsListView,
        floatingActionButton: createChatButton,
      ),
      tabletBody: Row(
        children: [
          ConstrainedBox(
            constraints: Responsiveness.kSideMenuWidthConstraints,
            child: Stack(
              children: [
                chatRoomsListView,
                Positioned.directional(
                  textDirection: Directionality.of(context),
                  end: 0,
                  bottom: 0,
                  child: Padding(
                    padding: const EdgeInsets.all(20.0),
                    child: createChatButton,
                  ),
                ),
              ],
            ),
          ),
          const VerticalDivider(width: 1),
          Expanded(
            child: Scaffold(
              body: currentOpenChat == null
                  ? Center(child: Text(l10n.noOpenChats))
                  : ChatScreen(
                      currentOpenChat,
                      defaultUser,
                      otherUser: getOtherUser(currentOpenChat, defaultUser),
                    ),
            ),
          ),
        ],
      ),
    );
  }

  User? getOtherUser(ChatRoom chat, User defaultUser) {
    final otherUser = ref
        .read(usersStoreProvider.notifier)
        .otherUserInDirectRoom(chat, defaultUser);

    if (otherUser == null && !chat.isGroupChatRoom) {
      _log.warning('single-person room with unknown otherUser');
    }
    return otherUser;
  }

  Widget _contentFromOverview(
    MessageContent? message,
    TextTheme theme, {
    required ChatRoom room,
    required AppLocalizations l10n,
  }) {
    if (message is TextMessageContent) {
      return Text(
        (message).content,
        style: theme.bodyLarge,
        maxLines: 2,
        overflow: TextOverflow.ellipsis,
      );
    } else if (message is GroupEventContent) {
      return _contentFromGroupEvent(message, theme, room: room, l10n: l10n);
    } else if (message is FileShareContent) {
      return Text(
        '${message.fileName} · ${fileSize(message.size)}',
        maxLines: 2,
        style: theme.bodyLarge!.copyWith(fontStyle: FontStyle.italic),
        overflow: TextOverflow.ellipsis,
      );
    } else {
      _log.fine('overview type ${message.runtimeType} has not been rendered');
      return const SizedBox.shrink();
    }
  }

  Widget _contentFromGroupEvent(
    GroupEventContent message,
    TextTheme theme, {
    required ChatRoom room,
    required AppLocalizations l10n,
  }) {
    if (message.type == GroupEventContentType.none) {
      return const SizedBox.shrink();
    }

    if (message.type == GroupEventContentType.created) {
      return Text(
        l10n.groupStateEventCreated,
        style: theme.bodyLarge!.copyWith(fontStyle: FontStyle.italic),
        maxLines: 2,
        overflow: TextOverflow.ellipsis,
      );
    } else if (message.type == GroupEventContentType.closed) {
      return Text(
        l10n.groupStateEventClosed,
        style: theme.bodyLarge!.copyWith(fontStyle: FontStyle.italic),
        maxLines: 2,
        overflow: TextOverflow.ellipsis,
      );
    } else {
      final u = ref
          .read(usersStoreProvider.notifier)
          .findMemberInRoom(message.userId, room);
      if (u == null) {
        _log.warning('group event message from unknown user');
        return Text(
          l10n.unknown,
          style: theme.bodyLarge!.copyWith(fontStyle: FontStyle.italic),
          maxLines: 2,
          overflow: TextOverflow.ellipsis,
        );
      }

      String event = '';
      switch (message.type) {
        case GroupEventContentType.invited:
          event = l10n.groupEventInvited(u.name);
          break;
        case GroupEventContentType.inviteAccepted:
          event = l10n.groupEventInviteAccepted(u.name);
          break;
        case GroupEventContentType.joined:
          event = l10n.groupEventJoined(u.name);
          break;
        case GroupEventContentType.left:
          event = l10n.groupEventLeft(u.name);
          break;
        case GroupEventContentType.removed:
          event = l10n.groupEventRemoved(u.name);
          break;
        case GroupEventContentType.none:
        case GroupEventContentType.created:
        case GroupEventContentType.closed:
          break;
      }

      return Text(
        event,
        style: theme.bodyLarge!.copyWith(fontStyle: FontStyle.italic),
        maxLines: 2,
        overflow: TextOverflow.ellipsis,
      );
    }
  }
}

class _GroupInviteTile extends HookConsumerWidget {
  const _GroupInviteTile({required this.invite});

  final GroupInvite invite;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final l10n = AppLocalizations.of(context)!;
    return ListTile(
      leading: Stack(
        alignment: Alignment.topRight,
        children: [
          QaulAvatar.groupSmall(),
          const Icon(Icons.add, size: 12, color: Colors.black),
        ],
      ),
      title: Text(l10n.groupInvite),
      subtitle: Text(
        invite.groupDetails.name ?? '',
        maxLines: 1,
        overflow: TextOverflow.ellipsis,
      ),
      contentPadding: const EdgeInsets.only(left: 16, right: 8),
      trailing: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          FloatingActionButton.small(
            elevation: 0,
            focusElevation: 0,
            hoverElevation: 0,
            highlightElevation: 0,
            heroTag: 'acceptGroupInviteFAB',
            backgroundColor: Colors.green.shade300,
            foregroundColor: Colors.white,
            onPressed: () {
              final worker = ref.read(qaulWorkerProvider);
              worker.replyToGroupInvite(
                invite.groupDetails.conversationId,
                accepted: true,
              );
            },
            child: const Icon(Icons.check),
          ),
          const SizedBox(width: 4),
          FloatingActionButton.small(
            elevation: 0,
            focusElevation: 0,
            hoverElevation: 0,
            highlightElevation: 0,
            heroTag: 'groupInviteDetailsFAB',
            backgroundColor: Colors.lightBlue,
            foregroundColor: Colors.white,
            onPressed: () => showDialog(
              context: context,
              builder: (context) => _InviteDetailsDialog(invite),
            ),
            child: const Icon(Icons.more_vert),
          ),
        ],
      ),
    );
  }
}
