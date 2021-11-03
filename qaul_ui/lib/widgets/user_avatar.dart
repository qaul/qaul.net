import 'dart:math';

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

  @protected
  String generateRandomInitials() {
    int charA = "a".codeUnitAt(0);
    int charZ = "z".codeUnitAt(0);
    var out = '';
    for (var i = 0; i < 2; i++) {
      out += String.fromCharCode(charA + Random().nextInt(charZ - charA));
    }
    return out.toUpperCase();
  }
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
        elevation: 0.0,
        padding: const EdgeInsets.all(6),
        borderSide: const BorderSide(color: Colors.white, width: 1.5),
        position: BadgePosition.bottomEnd(bottom: 0, end: 0),
        badgeColor: Colors.greenAccent.shade700,
        child: CircleAvatar(
          child: Text(
            user != null ? initials(user.name) : generateRandomInitials(),
            style: Theme.of(context)
                .textTheme
                .bodyText2!
                .copyWith(color: Colors.white, fontWeight: FontWeight.w600),
          ),
          backgroundColor: color ?? Colors.red.shade100,
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
          user != null ? initials(user.name) : generateRandomInitials(),
          style: Theme.of(context)
              .textTheme
              .headline2!
              .copyWith(color: Colors.white),
        ),
        backgroundColor: color ?? Colors.red.shade100,
      ),
    );
  }
}
