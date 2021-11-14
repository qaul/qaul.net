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
    final users =
    ref.watch(usersProvider).where((u) => !(u.isBlocked ?? false)).toList();

    useMemoized(() => refreshUsers(ref));

    return Scaffold(
      body: RefreshIndicator(
        onRefresh: () async => await refreshUsers(ref),
        child: ListView.separated(
          physics: const AlwaysScrollableScrollPhysics(),
          itemCount: users.length,
          separatorBuilder: (_, __) => const Divider(height: 12.0),
          itemBuilder: (_, i) {
            final user = users[i];
            var theme = Theme.of(context).textTheme;

            return ListTile(
              onTap: () async {
                await Navigator.push(
                  context,
                  MaterialPageRoute(
                      builder: (_) => _UserDetailsScreen(user: user)),
                );
                refreshUsers(ref);
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
                  if (user.availableTypes != null &&
                      user.availableTypes!.isNotEmpty)
                    _AvailableConnections(user: user),
                ],
              ),
            );
          },
        ),
      ),
    );
  }

  Future<void> refreshUsers(WidgetRef ref) async {
    await RpcRouter(ref.read).requestUsers();
    await Future.delayed(const Duration(seconds: 2));

    // TODO check isMounted
    final libqaul = ref.read(libqaulProvider);

    var queued = await libqaul.checkReceiveQueue();
    if (queued > 0) await libqaul.receiveRpc();

    await RpcUsers(ref.read).requestUsers();
    await Future.delayed(const Duration(seconds: 2));

    queued = await libqaul.checkReceiveQueue();
    if (queued > 0) await libqaul.receiveRpc();
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

  bool get _hasBluetooth => user.availableTypes!.contains(ConnectionType.ble);

  bool get _hasLan => user.availableTypes!.contains(ConnectionType.lan);

  bool get _hasLocal => user.availableTypes!.contains(ConnectionType.local);

  bool get _hasInternet =>
      user.availableTypes!.contains(ConnectionType.internet);
}

class _UserDetailsScreen extends HookConsumerWidget {
  const _UserDetailsScreen({Key? key, required this.user}) : super(key: key);
  final User user;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final loading = useState(false);

    var theme = Theme.of(context).textTheme;
    final l18ns = AppLocalizations.of(context);
    return LoadingDecorator(
      isLoading: loading.value,
      child: Scaffold(
        appBar: AppBar(
          leading: IconButton(
            tooltip: l18ns!.backButtonTooltip,
            icon: const Icon(Icons.arrow_back_ios_rounded),
            onPressed: () => Navigator.pop(context),
          ),
          title: Row(
            mainAxisAlignment: MainAxisAlignment.end,
            children: [
              Tooltip(
                message: l18ns.newChatTooltip,
                child: SvgPicture.asset(
                  'assets/icons/comment.svg',
                  width: 24,
                  height: 24,
                  color: Theme.of(context).appBarTheme.iconTheme?.color ??
                      Theme.of(context).iconTheme.color,
                ),
              ),
            ],
          ),
        ),
        body: SingleChildScrollView(
          child: Padding(
            padding: const EdgeInsets.symmetric(horizontal: 16.0, vertical: 32.0),
            child: Theme(
              data: Theme.of(context).copyWith(
                elevatedButtonTheme: ElevatedButtonThemeData(
                  style: ElevatedButton.styleFrom(
                    fixedSize: Size(MediaQuery.of(context).size.width * .8, 48),
                    shape: RoundedRectangleBorder(
                      borderRadius: BorderRadius.circular(24.0),
                    ),
                    textStyle: theme.headline6,
                    onSurface: Colors.white,
                  ),
                ),
              ),
              child: Builder(builder: (context) {
                return Column(
                  children: [
                    UserAvatar.large(user: user),
                    const SizedBox(height: 28.0),
                    Text(user.name, style: theme.headline3),
                    const SizedBox(height: 8.0),
                    Text('${l18ns.userID}: ${user.id?.join() ?? l18ns.unknown}',
                        style: theme.headline5),
                    const SizedBox(height: 40.0),
                    Text('${l18ns.publicKey}:\n${user.keyBase58}',
                        style: theme.headline5),
                    const SizedBox(height: 40.0),
                    if (!(user.isVerified ?? false)) ...[
                      _RoundedRectButton(
                        color: Colors.green,
                        onPressed: () async {
                          final res = await _confirmAction(context,
                              description: l18ns.verifyUserConfirmationMessage);

                          if (res is! bool || !res) return;
                          loading.value = true;

                          final libqaul = ref.read(libqaulProvider);

                          // TODO verify isMounted, block interaction while updating
                          await RpcUsers(ref.read).verifyUser(user);
                          await Future.delayed(const Duration(seconds: 2));

                          final queued = await libqaul.checkReceiveQueue();
                          if (queued > 0) await libqaul.receiveRpc();

                          loading.value = false;
                          Navigator.pop(context);
                        },
                        child: Row(
                          mainAxisAlignment: MainAxisAlignment.center,
                          children: [
                            const Icon(Icons.check, size: 32),
                            const SizedBox(width: 4),
                            Text(l18ns.verify),
                          ],
                        ),
                      ),
                      const SizedBox(height: 28.0),
                    ],
                    if (!(user.isBlocked ?? false))
                      _RoundedRectButton(
                        color: Colors.red.shade400,
                        onPressed: () async {
                          final res = await _confirmAction(context,
                              description: l18ns.blockUserConfirmationMessage);

                          if (res is! bool || !res) return;
                          loading.value = true;

                          final libqaul = ref.read(libqaulProvider);

                          // TODO verify isMounted, block interaction while updating
                          await RpcUsers(ref.read).blockUser(user);
                          await Future.delayed(const Duration(seconds: 2));

                          final queued = await libqaul.checkReceiveQueue();
                          if (queued > 0) await libqaul.receiveRpc();

                          loading.value = false;
                          Navigator.pop(context);
                        },
                        child: Text(l18ns.blockUser),
                      ),
                  ],
                );
              }),
            ),
          ),
        ),
      ),
    );
  }

  Future<bool?> _confirmAction(
    BuildContext context, {
    required String description,
  }) async {
    void pop({bool res = false}) => Navigator.pop(context, res);

    return await showDialog(
      context: context,
      builder: (c) {
        final l18ns = AppLocalizations.of(context);
        return AlertDialog(
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(20.0),
          ),
          title: Row(
            mainAxisAlignment: MainAxisAlignment.end,
            children: [
              IconButton(icon: const Icon(Icons.close), onPressed: pop),
            ],
          ),
          content: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              Text(
                description,
                style: Theme.of(context).textTheme.subtitle1,
                textAlign: TextAlign.center,
              ),
              const SizedBox(height: 24),
              _RoundedRectButton(
                color: Colors.lightBlue,
                size: const Size(280, 80),
                onPressed: () => pop(res: true),
                child: Text(l18ns!.okDialogButton),
              ),
              const SizedBox(height: 12),
              _RoundedRectButton(
                color: Colors.lightBlue,
                size: const Size(280, 80),
                onPressed: pop,
                child: Text(l18ns.cancelDialogButton),
              ),
            ],
          ),
        );
      },
    );
  }
}

class _RoundedRectButton extends StatelessWidget {
  const _RoundedRectButton({
    Key? key,
    required this.color,
    required this.onPressed,
    required this.child,
    this.size,
  }) : super(key: key);
  final Color color;
  final VoidCallback onPressed;
  final Widget child;
  final Size? size;

  @override
  Widget build(BuildContext context) {
    return ElevatedButton(
      onPressed: onPressed,
      style: Theme.of(context).elevatedButtonTheme.style!.copyWith(
            foregroundColor: MaterialStateProperty.all(Colors.white),
            backgroundColor: MaterialStateProperty.all(color),
            maximumSize: MaterialStateProperty.all(size),
          ),
      child: child,
    );
  }
}
