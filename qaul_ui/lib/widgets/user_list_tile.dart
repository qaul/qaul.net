import 'package:flutter/material.dart';
import 'package:qaul_rpc/qaul_rpc.dart';

import 'user_avatar.dart';

class UserListTile extends StatelessWidget {
  const UserListTile(
    this.user, {
    Key? key,
    this.content,
    this.trailingIcon,
    this.trailingMetadata,
    this.onTap,
    this.isThreeLine = false,
  })  : assert(trailingIcon == null || trailingMetadata == null),
        super(key: key);
  final User user;

  final bool isThreeLine;

  /// Content displayed under the username
  final Widget? content;

  /// Center-aligned widget, displayed at the end of tile.
  final Widget? trailingIcon;

  /// Right, Baseline-aligned with username (usually a Text).
  final Widget? trailingMetadata;

  final VoidCallback? onTap;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context).textTheme;

    final username = Text(
      user.name,
      maxLines: 1,
      overflow: TextOverflow.ellipsis,
      style: theme.bodyText1!.copyWith(fontWeight: FontWeight.bold),
    );

    Widget title = trailingIcon != null
        ? username
        : Row(
            textBaseline: TextBaseline.alphabetic,
            crossAxisAlignment: CrossAxisAlignment.baseline,
            children: [
              Expanded(child: username),
              if (trailingMetadata != null) trailingMetadata!,
            ],
          );

    return ListTile(
      onTap: onTap,
      title: title,
      subtitle: content,
      trailing: trailingIcon,
      isThreeLine: isThreeLine,
      leading: UserAvatar.small(user: user),
      visualDensity: VisualDensity.adaptivePlatformDensity,
    );
  }
}
