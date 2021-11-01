import 'package:badges/badges.dart';
import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:qaul_ui/providers/providers.dart';
import 'package:qaul_ui/providers/user_color_provider.dart';
import 'package:utils/utils.dart';

class UserAvatar extends ConsumerWidget {
  const UserAvatar({
    Key? key,
    required this.controller,
    required this.tab,
  }) : super(key: key);

  final SelectedTab controller;
  final TabType tab;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final color = ref.watch(userColorProvider);
    final user = ref.watch(defaultUserProvider).state;

    return GestureDetector(
      onTap: () => controller.goToTab(tab),
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
