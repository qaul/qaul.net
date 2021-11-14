part of '../home_screen.dart';

class _ChatTab extends ConsumerWidget {
  const _ChatTab({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final users = ref.watch(usersProvider);

    final defaultUser = ref.watch(defaultUserProvider).state ??
        const User(
          name: 'Breno',
          idBase58: '12D3KooWEbzJbVGua4EQNKQVUYoA46vcXnfePfi3ZL7C8pGGqddd',
        );

    final l18ns = AppLocalizations.of(context);
    return Scaffold(
      floatingActionButton: FloatingActionButton(
        tooltip: l18ns!.newChatTooltip,
        onPressed: () {},
        child: SvgPicture.asset(
          'assets/icons/comment.svg',
          width: 24,
          height: 24,
          color: Theme.of(context).floatingActionButtonTheme.foregroundColor,
        ),
      ),
      body: ListView.separated(
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
                Text(user.name, style: theme.headline6),
                const Expanded(child: SizedBox()),
                Text('12:00', style: theme.caption),
                const Icon(Icons.chevron_right),
              ],
            ),
            subtitle: Text(
              'Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.',
              style: theme.caption!.copyWith(fontSize: 16, height: 1.4),
              maxLines: 2,
              overflow: TextOverflow.ellipsis,
            ),
            onTap: defaultUser == null
                ? null
                : () =>
                Navigator.push(
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
                        }, userAppBar: Row(
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
    );
  }
}
