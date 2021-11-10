part of '../home_screen.dart';

class _UserAccountTab extends ConsumerWidget {
  const _UserAccountTab({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final user = ref.watch(defaultUserProvider).state;

    var theme = Theme.of(context).textTheme;
    return Padding(
      padding: MediaQuery.of(context)
          .viewPadding
          .add(const EdgeInsets.fromLTRB(16, 8, 16, 8)),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              UserAvatar.large(),
              Expanded(
                child: Padding(
                  padding: const EdgeInsets.symmetric(horizontal: 24.0),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        user != null ? user.name : _notFound('Username'),
                        style: theme.headline6,
                      ),
                      const SizedBox(height: 24),
                      Text(
                        user != null ? user.idBase58 : _notFound('User ID'),
                        style: theme.subtitle2,
                        maxLines: 3,
                        overflow: TextOverflow.ellipsis,
                      ),
                    ],
                  ),
                ),
              ),
            ],
          ),
          const SizedBox(height: 60),
          Text('Qaul Public Key', style: theme.headline5),
          const SizedBox(height: 20),
          Text(
            user != null && user.keyBase58 != null
                ? user.keyBase58!
                : _notFound('Public Key'),
          ),
        ],
      ),
    );
  }

  String _notFound(String field) => '$field not found';
}
