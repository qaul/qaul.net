part of '../home_screen.dart';

class _UserAccountTab extends StatelessWidget {
  const _UserAccountTab({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    var theme = Theme.of(context).textTheme;
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 16.0),
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              UserAvatar.large(),
              const SizedBox(width: 24),
              Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text('User name', style: theme.headline6),
                  const SizedBox(height: 24),
                  Text('User ID', style: theme.subtitle1),
                ],
              ),
            ],
          ),
          const SizedBox(height: 60),
          Text('Qaul Public Key', style: theme.headline5),
          const SizedBox(height: 20),
          Text('-' * 500),
        ],
      ),
    );
  }
}
