part of 'tab.dart';

class _Chat extends BaseTab {
  const _Chat({Key? key}) : super(key: key);

  @override
  _ChatState createState() => _ChatState();
}

class _ChatState extends _BaseTabState<_Chat> {
  @override
  Widget build(BuildContext context) {
    super.build(context);
    final defaultUser = ref.watch(defaultUserProvider)!;
    final users = ref.watch(usersProvider);
    final chatRooms = ref.watch(chatRoomsProvider);

    final blockedIds = users.where((u) => u.isBlocked ?? false).map((u) => u.id);
    final filteredRooms = chatRooms.where((m) => !blockedIds.contains(m.conversationId)).toList();

    final refreshChats = useCallback(() async {
      final worker = ref.read(qaulWorkerProvider);
      worker.getAllChatRooms();
    }, [UniqueKey()]);

    final l18ns = AppLocalizations.of(context);
    return Scaffold(
      floatingActionButton: FloatingActionButton(
        heroTag: 'chatTabFAB',
        tooltip: l18ns!.newChatTooltip,
        onPressed: () async {
          final availableUsers = users
              .where((u) =>
                  !(u.isBlocked ?? false) &&
                  chatRooms.indexWhere((c) => c.conversationId == u.id).isNegative)
              .toList();
          final user = await showDialog(
            context: context,
            builder: (_) => _CreateNewRoomDialog(availableUsers),
          );
          if (user is User) {
            final newRoom = ChatRoom.blank(user: defaultUser, otherUser: user);
            _openChat(newRoom, defaultUser, user);
          }
        },
        child: SvgPicture.asset(
          'assets/icons/comment.svg',
          width: 24,
          height: 24,
          color: Theme.of(context).floatingActionButtonTheme.foregroundColor,
        ),
      ),
      body: CronTaskDecorator(
        callback: () => refreshChats(),
        schedule: const Duration(milliseconds: 500),
        child: RefreshIndicator(
          onRefresh: () => refreshChats(),
          child: EmptyStateTextDecorator(
            l18ns.emptyChatsList,
            isEmpty: filteredRooms.isEmpty,
            child: ListView.separated(
              controller: ScrollController(),
              physics: const AlwaysScrollableScrollPhysics(),
              itemCount: filteredRooms.length,
              separatorBuilder: (_, __) => const Divider(height: 12.0),
              itemBuilder: (_, i) {
                var theme = Theme.of(context).textTheme;
                final room = filteredRooms[i];
                final otherUser = users.firstWhereOrNull((u) => u.id.equals(room.conversationId));

                // TODO error handling?
                if (otherUser == null) return const SizedBox.shrink();

                return ListTile(
                  leading: UserAvatar.small(user: otherUser),
                  title: Row(
                    children: [
                      Text(room.name ?? '',
                          style: theme.bodyText1!.copyWith(fontWeight: FontWeight.bold)),
                      const Expanded(child: SizedBox()),
                      Text(
                        room.lastMessageTime == null
                            ? ''
                            : describeFuzzyTimestamp(room.lastMessageTime!),
                        style: theme.caption!.copyWith(fontStyle: FontStyle.italic),
                      ),
                      const Icon(Icons.chevron_right),
                    ],
                  ),
                  subtitle: Text(
                    room.lastMessagePreview ?? '',
                    style: theme.bodyText1,
                    maxLines: 2,
                    overflow: TextOverflow.ellipsis,
                  ),
                  onTap: () => _openChat(room, defaultUser, otherUser),
                );
              },
            ),
          ),
        ),
      ),
    );
  }

  void _openChat(ChatRoom room, User user, User otherUser) {
    ref.read(currentOpenChatRoom.notifier).state = room;
    ref.read(qaulWorkerProvider).getChatRoomMessages(room.conversationId);

    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (context) {
          return ChatScreen(
            initialMessages: room.messages ?? [],
            user: user,
            otherUser: otherUser,
            onSendPressed: (msg) {
              final worker = ref.read(qaulWorkerProvider);
              worker.sendMessage(room.conversationId, msg);
            },
            userAppBar: Row(
              children: [
                UserAvatar.small(badgeEnabled: false, user: otherUser),
                const SizedBox(width: 12),
                Text(otherUser.name),
              ],
            ),
          );
        },
      ),
    );
  }
}

class _CreateNewRoomDialog extends StatelessWidget {
  const _CreateNewRoomDialog(this.availableUsers, {Key? key})
      : assert(availableUsers.length > 0),
        super(key: key);
  final List<User> availableUsers;

  @override
  Widget build(BuildContext context) {
    var theme = Theme.of(context).textTheme;

    return Scaffold(
      body: Container(
        decoration: BoxDecoration(
          borderRadius: BorderRadius.circular(20),
        ),
        child: ListView.builder(
          itemCount: availableUsers.length,
          itemBuilder: (context, i) {
            final user = availableUsers[i];
            return ListTile(
              onTap: () => Navigator.pop(context, user),
              leading: UserAvatar.small(user: user),
              title: Text(user.name, style: theme.bodyText1!.copyWith(fontWeight: FontWeight.bold)),
            );
          },
        ),
      ),
    );
  }
}
