part of 'tab.dart';

class _Users extends BaseTab {
  const _Users({Key? key}) : super(key: key);

  @override
  _UsersState createState() => _UsersState();
}

class _UsersState extends _BaseTabState<_Users> {
  @override
  Widget build(BuildContext context) {
    super.build(context);
    final defaultUser = ref.watch(defaultUserProvider);
    final users =
        ref.watch(usersProvider).where((u) => u.idBase58 != (defaultUser.idBase58 ?? '')).toList();

    users.sort((u1, u2) => u2.isConnected ? 1 : 0);
    users.sort((u1, u2) => (u1.isBlocked ?? false) ? 1 : 0);

    final refreshUsers = useCallback(() async {
      final worker = ref.read(qaulWorkerProvider);
      await worker.getUsers();
    }, [UniqueKey()]);

    final l18ns = AppLocalizations.of(context)!;
    return Scaffold(
      body: CronTaskDecorator(
        schedule: const Duration(milliseconds: 1000),
        callback: () async => await refreshUsers(),
        child: RefreshIndicator(
          onRefresh: () async => await refreshUsers(),
          child: EmptyStateTextDecorator(
            l18ns.emptyUsersList,
            isEmpty: users.isEmpty,
            child: ListView.separated(
              controller: ScrollController(),
              physics: const AlwaysScrollableScrollPhysics(),
              itemCount: users.length,
              separatorBuilder: (_, __) => const Divider(height: 12.0),
              itemBuilder: (_, i) {
                final user = users[i];
                var theme = Theme.of(context).textTheme;

                return DisabledStateDecorator(
                  isDisabled: user.isBlocked ?? false,
                  child: ListTile(
                    onTap: () async {
                      await Navigator.push(
                        context,
                        MaterialPageRoute(builder: (_) => UserDetailsScreen(user: user)),
                      );
                      refreshUsers();
                    },
                    leading: UserAvatar.small(user: user),
                    trailing: (user.isVerified ?? false)
                        ? const Icon(Icons.verified_user)
                        : const SizedBox(),
                    visualDensity: VisualDensity.adaptivePlatformDensity,
                    title: Padding(
                      padding: const EdgeInsets.only(bottom: 4.0),
                      child: Text(user.name, style: theme.headline6),
                    ),
                    subtitle: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          'ID: ${user.idBase58}',
                          style: theme.caption!.copyWith(fontSize: 10),
                        ),
                        const SizedBox(height: 4),
                        if (user.availableTypes != null && user.availableTypes!.isNotEmpty)
                          _AvailableConnections(user: user),
                      ],
                    ),
                  ),
                );
              },
            ),
          ),
        ),
      ),
    );
  }
}

class _AvailableConnections extends StatelessWidget {
  const _AvailableConnections({
    Key? key,
    required this.user,
  }) : super(key: key);

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
          if (_hasBluetooth) const Icon(Icons.bluetooth),
        ],
      ),
    );
  }

  bool get _hasBluetooth => user.availableTypes!.keys.contains(ConnectionType.ble);

  bool get _hasLan => user.availableTypes!.keys.contains(ConnectionType.lan);

  bool get _hasLocal => user.availableTypes!.keys.contains(ConnectionType.local);

  bool get _hasInternet => user.availableTypes!.keys.contains(ConnectionType.internet);
}
