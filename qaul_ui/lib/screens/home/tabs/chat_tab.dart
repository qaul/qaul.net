part of '../home_screen.dart';

class _ChatTab extends ConsumerWidget {
  const _ChatTab({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final users = ref.watch(usersProvider);

    final defaultUser = ref.watch(defaultUserProvider).state ??
        User(
          name: 'Breno',
          idBase58: '12D3KooWEbzJbVGua4EQNKQVUYoA46vcXnfePfi3ZL7C8pGGqddd',
        );

    return Scaffold(
      floatingActionButton: FloatingActionButton(
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
        itemCount: 4,
        separatorBuilder: (_, __) => const Divider(height: 12.0),
        itemBuilder: (_, i) {
          var theme = Theme.of(context).textTheme;

          return ListTile(
            leading: UserAvatar.small(user: users[i]),
            title: Row(
              children: [
                Text(users[i].name, style: theme.headline6),
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
                : () => showChat(
                      context: context,
                      messages: [
                        TextMessage(
                          idBase58: const Uuid().v4(),
                          text: 'this is a message by another user',
                          user: users[i],
                        ),
                      ],
                      user: defaultUser,
                      otherUserAvatarColor: colorGenerationStrategy(users[i].idBase58),
                      userAppBarAvatar: Row(
                        children: [
                          UserAvatar.small(badgeEnabled: false),
                          const SizedBox(width: 12),
                          Text(defaultUser.name),
                        ],
                      ),
                      onSendPressed: (String rawText) {
                        return TextMessage(
                          idBase58: const Uuid().v4(),
                          text: rawText,
                          user: defaultUser,
                        );
                      },
                    ),
          );
        },
      ),
    );
  }
}
