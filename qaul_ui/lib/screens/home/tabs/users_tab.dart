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
    final refreshUsers = useCallback(() async {
      final worker = ref.read(qaulWorkerProvider);
      await worker.getUsers();
    }, [UniqueKey()]);

    final l10n = AppLocalizations.of(context)!;
    return Scaffold(
      body: CronTaskDecorator(
        schedule: const Duration(milliseconds: 1000),
        callback: () async => await refreshUsers(),
        child: RefreshIndicator(
          onRefresh: () async => await refreshUsers(),
          child: SearchUserDecorator(builder: (_, users) {
            return EmptyStateTextDecorator(
              l10n.emptyUsersList,
              isEmpty: users.isEmpty,
              child: ListView.separated(
                controller: ScrollController(),
                physics: const AlwaysScrollableScrollPhysics(),
                itemCount: users.length,
                separatorBuilder: (_, __) => const Divider(height: 12.0),
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
