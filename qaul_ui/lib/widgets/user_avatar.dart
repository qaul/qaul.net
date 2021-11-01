import 'package:badges/badges.dart';
import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:qaul_ui/providers/providers.dart';
import 'package:qaul_ui/providers/user_color_provider.dart';
import 'package:utils/utils.dart';

abstract class UserAvatar extends ConsumerWidget {
  const UserAvatar({Key? key}) : super(key: key);

  factory UserAvatar.small({Key? key}) => _SmallUserAvatar(key: key);

  factory UserAvatar.large({Key? key}) => _LargeUserAvatar(key: key);
}

class _SmallUserAvatar extends UserAvatar {
  const _SmallUserAvatar({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final controller = ref.watch(selectedTabProvider);
    final color = ref.watch(userColorProvider);
    final user = ref.watch(defaultUserProvider).state;

    return GestureDetector(
      onTap: () => controller.goToTab(TabType.account),
      // TODO(brenodt): Change badge status depending on user connectivity
      child: Badge(
        position: BadgePosition.bottomEnd(bottom: 0, end: 0),
        badgeColor: Colors.greenAccent.shade700,
        child: CircleAvatar(
          child: Text(user != null ? initials(user.name) : ''),
          backgroundColor: color ?? Colors.limeAccent.shade100,
        ),
      ),
    );
  }
}

class _LargeUserAvatar extends UserAvatar {
  const _LargeUserAvatar({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final controller = ref.watch(selectedTabProvider);
    final color = ref.watch(userColorProvider);
    final user = ref.watch(defaultUserProvider).state;

    return GestureDetector(
      onTap: () => controller.goToTab(TabType.account),
      child: CircleAvatar(
        minRadius: 60.0,
        maxRadius: 80.0,
        child: Text(
          user != null ? initials(user.name) : '',
          style: Theme.of(context).textTheme.headline2,
        ),
        backgroundColor: color ?? Colors.limeAccent.shade100,
      ),
    );
  }
}
