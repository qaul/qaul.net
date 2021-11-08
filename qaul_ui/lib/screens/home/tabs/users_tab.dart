part of '../home_screen.dart';

class _UsersTab extends ConsumerWidget {
  const _UsersTab({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final users = ref.watch(usersProvider);
    return Scaffold(
      body: RefreshIndicator(
        onRefresh: () async => await RpcRouter(ref.read).requestUsers(),
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
                    'ID: ${user.id?.join() ?? 'Unknown'}',
                    style: theme.caption,
                  ),
                  const SizedBox(height: 4),
                  IconTheme(
                    data: Theme.of(context).iconTheme.copyWith(size: 18.0),
                    child: Row(
                      children: const [
                        Icon(CupertinoIcons.globe),
                        SizedBox(width: 4),
                        Icon(Icons.wifi),
                        SizedBox(width: 4),
                        Icon(Icons.bluetooth),
                      ],
                    ),
                  ),
                ],
              ),
            );
          },
        ),
      ),
    );
  }
}

class _UserDetailsScreen extends StatelessWidget {
  const _UserDetailsScreen({Key? key, required this.user}) : super(key: key);
  final User user;

  @override
  Widget build(BuildContext context) {
    var theme = Theme.of(context).textTheme;
    return Scaffold(
      appBar: AppBar(
        leading: IconButton(
          tooltip: 'Back',
          icon: const Icon(Icons.arrow_back_ios_rounded),
          onPressed: () => Navigator.pop(context),
        ),
        title: Row(
          mainAxisAlignment: MainAxisAlignment.end,
          children: [
            SvgPicture.asset(
              'assets/icons/comment.svg',
              width: 24,
              height: 24,
              color: Theme.of(context).appBarTheme.iconTheme?.color ??
                  Theme.of(context).iconTheme.color,
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
                  Text('User ID: ${user.id?.join() ?? 'Unkown'}',
                      style: theme.headline5),
                  const SizedBox(height: 40.0),
                  RichText(
                    text: TextSpan(children: [
                      TextSpan(text: 'Public Key:\n', style: theme.headline5),
                      TextSpan(text: '-' * 500, style: theme.bodyText2),
                    ]),
                  ),
                  const SizedBox(height: 40.0),
                  _RoundedRectButton(
                    color: Colors.green,
                    onPressed: () => _verifyUser(context),
                    child: Row(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: const [
                        Icon(Icons.check, size: 32),
                        SizedBox(width: 4),
                        Text('Verify'),
                      ],
                    ),
                  ),
                  const SizedBox(height: 28.0),
                  _RoundedRectButton(
                    color: Colors.red.shade400,
                    onPressed: () {},
                    child: const Text('Block User'),
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
      builder: (c) => Scaffold(
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
                      'Do you want to verify this user?',
                      style: Theme.of(context).textTheme.subtitle1,
                    ),
                    const SizedBox(height: 24),
                    _RoundedRectButton(
                        color: Colors.lightBlue,
                        onPressed: () => pop(res: true),
                        child: const Text('OK')),
                    const SizedBox(height: 12),
                    _RoundedRectButton(
                        color: Colors.lightBlue,
                        onPressed: pop,
                        child: const Text('Cancel')),
                  ],
                ),
              ),
            ],
          ),
        ),
      ),
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
