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
    final users = ref
        .watch(usersProvider)
        .where((u) => !(u.isBlocked ?? false))
        .where((u) => u.idBase58 != (defaultUser?.idBase58 ?? ''))
        .toList();

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
        // TODO: this should be updated when a chat module is integrated
        isEmpty: users.isEmpty,
        child: ListView.separated(
          controller: ScrollController(),
          physics: const AlwaysScrollableScrollPhysics(),
          itemCount: users.length,
          separatorBuilder: (_, __) => const Divider(height: 12.0),
          itemBuilder: (_, i) {
            var theme = Theme.of(context).textTheme;

            final user = users[i];
            return ListTile(
              leading: UserAvatar.small(user: user),
              title: Row(
                children: [
                  Text(user.name, style: theme.bodyText1!.copyWith(fontWeight: FontWeight.bold)),
                  const Expanded(child: SizedBox()),
                  Text('12:00', style: theme.caption!.copyWith(fontStyle: FontStyle.italic)),
                  const Icon(Icons.chevron_right),
                ],
              ),
              subtitle: Text(
                'Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.',
                style: theme.bodyText1,
                maxLines: 2,
                overflow: TextOverflow.ellipsis,
              ),
              onTap: defaultUser == null
                  ? null
                  : () => Navigator.push(
                        context,
                        MaterialPageRoute(
                          builder: (context) {
                            return ChatScreen(
                              initialMessages: [
                                TextMessage(
                                  idBase58: const Uuid().v4(),
                                  text: 'this is a message by another user',
                                  user: user,
                                ),
                              ],
                              user: defaultUser,
                              otherUserAvatarColor: colorGenerationStrategy(user.idBase58),
                              onSendPressed: (String rawText) {
                                return TextMessage(
                                  idBase58: const Uuid().v4(),
                                  text: rawText,
                                  user: defaultUser,
                                );
                              },
                              userAppBar: Row(
                                children: [
                                  UserAvatar.small(badgeEnabled: false, user: user),
                                  const SizedBox(width: 12),
                                  Text(user.name),
                                ],
                              ),
                            );
                          },
                        ),
                      ),
            );
          },
        ),
      ),
    );
  }
}
