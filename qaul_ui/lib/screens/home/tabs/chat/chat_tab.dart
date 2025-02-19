part of '../tab.dart';

class _Chat extends BaseTab {
  const _Chat({super.key});

  @override
  _ChatState createState() => _ChatState();
}

class _ChatState extends _BaseTabState<_Chat> {
  final _log = Logger('BaseTab.chat');

  @override
  void initState() {
    super.initState();

    ref.read(chatNotificationControllerProvider).initialize();
  }

  @override
  Widget build(BuildContext context) {
    super.build(context);

    final defaultUser = ref.watch(defaultUserProvider)!;
    final users = ref.watch(usersProvider);
    final chatRooms = ref.watch(chatRoomsProvider);
    final groupInvites = ref.watch(groupInvitesProvider);
    final currentOpenChat = ref.watch(currentOpenChatRoom);

    final blockedIds =
        users.where((u) => u.isBlocked ?? false).map((u) => u.conversationId);
    final filteredRooms = chatRooms
        .where((m) => !blockedIds.contains(m.conversationId))
        .toList()
      ..sort();

    final refreshChatsAndInvites = useCallback(() async {
      final worker = ref.read(qaulWorkerProvider);
      worker.getAllChatRooms();
      worker.getGroupInvitesReceived();
    }, []);

    final mobile =
        Responsiveness.isMobile(context); // MediaQuery on HookState.init throws
    final setOpenChat = useCallback((ChatRoom room, [User? otherUser]) {
      if (mobile) {
        openChat(room,
            ref: ref,
            context: context,
            user: defaultUser,
            otherUser: otherUser);
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
      callback: () => refreshChatsAndInvites(),
      schedule: const Duration(milliseconds: 1000),
      child: RefreshIndicator(
        onRefresh: () => refreshChatsAndInvites(),
        child: EmptyStateTextDecorator(
          l10n!.emptyChatsList,
          isEmpty: groupInvites.isEmpty && filteredRooms.isEmpty,
          child: ListView.separated(
            controller: ScrollController(),
            physics: const AlwaysScrollableScrollPhysics(),
            itemCount: groupInvites.length + filteredRooms.length,
            separatorBuilder: (_, __) => const Divider(height: 12.0),
            itemBuilder: (_, i) {
              var theme = Theme.of(context).textTheme;

              if (i < groupInvites.length) {
                return _GroupInviteTile(invite: groupInvites[i]);
              }

              final room = filteredRooms[i - groupInvites.length];
              if (room.isGroupChatRoom) {
                return QaulListTile.group(
                  room,
                  content: _contentFromOverview(
                    room.lastMessagePreview,
                    theme,
                    users: users,
                    l10n: l10n,
                  ),
                  trailingMetadata: Row(
                    children: [
                      Text(
                        room.lastMessageTime == null
                            ? ''
                            : describeFuzzyTimestamp(
                                room.lastMessageTime!,
                                locale:
                                    Locale.parse(Intl.defaultLocale ?? 'en'),
                              ),
                        style: theme.bodySmall!
                            .copyWith(fontStyle: FontStyle.italic),
                      ),
                      const Icon(Icons.chevron_right),
                    ],
                  ),
                  onTap: () => setOpenChat(room),
                );
              }

              final otherUser = users.firstWhereOrNull((u) =>
                  u.conversationId != null &&
                  u.conversationId!.equals(room.conversationId));

              if (otherUser == null) {
                _log.warning('single-person room with unknown otherUser');
                return const SizedBox.shrink();
              }

              return QaulListTile.user(
                otherUser,
                content: _contentFromOverview(
                  room.lastMessagePreview,
                  theme,
                  users: users,
                  l10n: l10n,
                ),
                trailingMetadata: Row(
                  children: [
                    Text(
                      room.lastMessageTime == null
                          ? ''
                          : describeFuzzyTimestamp(
                              room.lastMessageTime!,
                              locale: Locale.parse(Intl.defaultLocale ?? 'en'),
                            ),
                      style:
                          theme.bodySmall!.copyWith(fontStyle: FontStyle.italic),
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
                )
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
                      otherUser: getOtherUser(currentOpenChat, users),
                    ),
            ),
          ),
        ],
      ),
    );
  }

  User? getOtherUser(ChatRoom chat, List<User> users) {
    final otherUser = users.firstWhereOrNull(
        (u) => u.conversationId?.equals(chat.conversationId) ?? false);

    if (otherUser == null && !chat.isGroupChatRoom) {
      _log.warning('single-person room with unknown otherUser');
    }
    return otherUser;
  }

  Widget _contentFromOverview(
    MessageContent? message,
    TextTheme theme, {
    required List<User> users,
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
      return _contentFromGroupEvent(message, theme, users: users, l10n: l10n);
    } else if (message is FileShareContent) {
      return Text(
        '${message.fileName} · ${filesize(message.size)}',
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
    required List<User> users,
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
      final u =
          users.firstWhereOrNull((e) => e.idBase58 == message.userIdBase58);
      if (u == null) {
        _log.warning('group event message from unknown user');
        return const SizedBox.shrink();
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
  const _GroupInviteTile({
    required this.invite,
  });

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
