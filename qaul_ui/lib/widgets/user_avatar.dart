import 'dart:math';

import 'package:badges/badges.dart';
import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:qaul_ui/providers/providers.dart';
import 'package:qaul_ui/providers/user_color_provider.dart';
import 'package:utils/utils.dart';

/// If [user] is provided, it's used to populate this icon (Background color, initials, connection status).
///
/// Otherwise, the user found in [defaultUserProvider] is used.
abstract class UserAvatar extends ConsumerWidget {
  const UserAvatar({Key? key, this.user}) : super(key: key);
  final User? user;

  factory UserAvatar.small({Key? key, User? user}) =>
      _SmallUserAvatar(key: key, user: user);

  factory UserAvatar.large({Key? key, User? user}) =>
      _LargeUserAvatar(key: key, user: user);

  String get userInitials => initials(user!.name);

  Color get userColor => colorGenerationStrategy(user!.idBase58);

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
  const _SmallUserAvatar({Key? key, User? user}) : super(key: key, user: user);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final controller = ref.watch(selectedTabProvider);
    final defaultUserColor = ref.watch(userColorProvider);
    final defaultUser = ref.watch(defaultUserProvider).state;

    final badgeColor = mapConnectionStatus(user != null
        ? user!.status
        : (defaultUser?.status ?? ConnectionStatus.offline));

    return GestureDetector(
      onTap: () => controller.goToTab(TabType.account),
      child: Badge(
        elevation: 0.0,
        padding: const EdgeInsets.all(6),
        borderSide: const BorderSide(color: Colors.white, width: 1.5),
        position: BadgePosition.bottomEnd(bottom: 0, end: 0),
        badgeColor: badgeColor,
        child: CircleAvatar(
          child: Text(
            user != null
                ? userInitials
                : defaultUser != null
                    ? initials(defaultUser.name)
                    : generateRandomInitials(),
            style: Theme.of(context)
                .textTheme
                .bodyText2!
                .copyWith(color: Colors.white, fontWeight: FontWeight.w600),
          ),
          backgroundColor: user != null
              ? userColor
              : defaultUserColor ?? Colors.red.shade100,
        ),
      ),
    );
  }

  Color mapConnectionStatus(ConnectionStatus s) {
    switch (s) {
      case ConnectionStatus.online:
        return Colors.greenAccent.shade700;
      case ConnectionStatus.reachable:
        return Colors.yellow.shade800;
      case ConnectionStatus.offline:
        return Colors.grey.shade500;
      default:
        throw ArgumentError.value(s, 'ConnectionStatus', 'value not mapped');
    }
  }
}

class _LargeUserAvatar extends UserAvatar {
  const _LargeUserAvatar({Key? key, User? user}) : super(key: key, user: user);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final controller = ref.watch(selectedTabProvider);
    final defaultUserColor = ref.watch(userColorProvider);
    final defaultUser = ref.watch(defaultUserProvider).state;

    return GestureDetector(
      onTap: () => controller.goToTab(TabType.account),
      child: CircleAvatar(
        minRadius: 60.0,
        maxRadius: 80.0,
        child: Text(
          user != null
              ? userInitials
              : defaultUser != null
                  ? initials(defaultUser.name)
                  : generateRandomInitials(),
          style: Theme.of(context)
              .textTheme
              .headline2!
              .copyWith(color: Colors.white),
        ),
        backgroundColor:
            user != null ? userColor : defaultUserColor ?? Colors.red.shade100,
      ),
    );
  }
}
