part of '../home_screen.dart';

class _UsersTab extends HookConsumerWidget {
  const _UsersTab({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final users = ref.watch(usersProvider);
    return Scaffold(
      body: RefreshIndicator(
        onRefresh: () async {
          await RpcRouter(ref.read).requestUsers();

          await Future.delayed(const Duration(seconds: 2));

          // TODO check isMounted & trigger on 1st build like FeedTab
          final libqaul = ref.read(libqaulProvider);

          // DEBUG: how many messages are queued by libqaul
          final queued = await libqaul.checkReceiveQueue();
          // check for rpc messages
          if (queued > 0) await libqaul.receiveRpc();
        },
        child: ListView.separated(
          physics: const AlwaysScrollableScrollPhysics(),
          itemCount: users.length,
          separatorBuilder: (_, __) => const Divider(height: 12.0),
          itemBuilder: (_, i) {
            final user = users[i];
            var theme = Theme.of(context).textTheme;

            return ListTile(
              onTap: () {
                Navigator.push(
                  context,
                  MaterialPageRoute(
                      builder: (_) => _UserDetailsScreen(user: user)),
                );
              },
              leading: UserAvatar.small(user: user),
              trailing: const Icon(Icons.verified_user),
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

class _UserDetailsScreen extends StatelessWidget {
  const _UserDetailsScreen({Key? key, required this.user}) : super(key: key);
  final User user;

  @override
  Widget build(BuildContext context) {
    var theme = Theme.of(context).textTheme;
    final l18ns = AppLocalizations.of(context);
    return Scaffold(
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
                  RichText(
                    text: TextSpan(children: [
                      TextSpan(
                          text: '${l18ns.publicKey}:\n',
                          style: theme.headline5),
                      TextSpan(text: '-' * 500, style: theme.bodyText2),
                    ]),
                  ),
                  const SizedBox(height: 40.0),
                  _RoundedRectButton(
                    color: Colors.green,
                    onPressed: () => _verifyUser(context),
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
                  _RoundedRectButton(
                    color: Colors.red.shade400,
                    onPressed: () {},
                    child: Text(l18ns.blockUser),
                  ),
                ],
              );
            }),
          ),
        ),
      ),
    );
  }

  Future<void> _verifyUser(BuildContext context) async {
    void pop({bool res = false}) => Navigator.pop(context, res);

    final size = MediaQuery.of(context).size;

    final res = await showDialog(
      context: context,
      builder: (c) {
        final l18ns = AppLocalizations.of(context);
        return Scaffold(
          backgroundColor: Colors.black38,
          body: Container(
            padding: const EdgeInsets.all(8.0),
            margin: EdgeInsets.symmetric(
              horizontal: size.width * .17,
              vertical: size.height * .33,
            ),
            decoration: BoxDecoration(
              color: Theme.of(context).dialogBackgroundColor,
              borderRadius: BorderRadius.circular(20),
            ),
            child: Column(
              children: [
                Row(
                  mainAxisAlignment: MainAxisAlignment.end,
                  children: [
                    IconButton(icon: const Icon(Icons.close), onPressed: pop),
                  ],
                ),
                Padding(
                  padding: const EdgeInsets.all(16.0),
                  child: Column(
                    children: [
                      Text(
                        l18ns!.verifyUserConfirmationMessage,
                        style: Theme.of(context).textTheme.subtitle1,
                      ),
                      const SizedBox(height: 24),
                      _RoundedRectButton(
                          color: Colors.lightBlue,
                          onPressed: () => pop(res: true),
                          child: Text(l18ns.okDialogButton)),
                      const SizedBox(height: 12),
                      _RoundedRectButton(
                          color: Colors.lightBlue,
                          onPressed: pop,
                          child: Text(l18ns.cancelDialogButton)),
                    ],
                  ),
                ),
              ],
            ),
          ),
        );
      },
    );

    if (res is! bool || !res) return;
    // TODO(brenodt): Add logic to verify user.
  }
}

class _RoundedRectButton extends StatelessWidget {
  const _RoundedRectButton({
    Key? key,
    required this.color,
    required this.onPressed,
    required this.child,
  }) : super(key: key);
  final Color color;
  final VoidCallback onPressed;
  final Widget child;

  @override
  Widget build(BuildContext context) {
    return ElevatedButton(
      onPressed: onPressed,
      style: Theme.of(context).elevatedButtonTheme.style!.copyWith(
          foregroundColor: MaterialStateProperty.all(Colors.white),
          backgroundColor: MaterialStateProperty.all(color)),
      child: child,
    );
  }
}
