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
    final defaultUser = ref.watch(defaultUserProvider);
    final chatRooms = ref.watch(chatRoomsProvider);

    final l18ns = AppLocalizations.of(context);
    return Scaffold(
      floatingActionButton: FloatingActionButton(
        heroTag: 'chatTabFAB',
        tooltip: l18ns!.newChatTooltip,
        onPressed: () {},
        child: SvgPicture.asset(
          'assets/icons/comment.svg',
          width: 24,
          height: 24,
          color: Theme.of(context).floatingActionButtonTheme.foregroundColor,
        ),
      ),
      body: EmptyStateTextDecorator(
        l18ns.emptyChatsList,
        isEmpty: chatRooms.isEmpty,
        child: ListView.separated(
          controller: ScrollController(),
          physics: const AlwaysScrollableScrollPhysics(),
          itemCount: chatRooms.length,
          separatorBuilder: (_, __) => const Divider(height: 12.0),
          itemBuilder: (_, i) {
            var theme = Theme.of(context).textTheme;
            final room = chatRooms[i];

            return ListTile(
              // TODO Must match with other user's id
              // leading: UserAvatar.small(user: user),
              leading: const CircleAvatar(),
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
              onTap: defaultUser == null
                  ? null
                  : () {
                      Navigator.push(
                        context,
                        MaterialPageRoute(
                          builder: (context) {
                            // TODO: MUST BE MAPPED
                            final otherUser = defaultUser;
                            return ChatScreen(
                              initialMessages: room.messages!,
                              user: defaultUser,
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
                    },
            );
          },
        ),
      ),
    );
  }
}
