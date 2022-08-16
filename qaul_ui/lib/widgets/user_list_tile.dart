part of 'widgets.dart';

class UserListTile extends StatelessWidget {
  const UserListTile(
    this.user, {
    Key? key,
    this.content,
    this.trailingIcon,
    this.trailingMetadata,
    this.onTap,
    this.isThreeLine = false,
    this.allowTapRouteToUserDetailsScreen = true,
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

  /// Override the behavior of tapping this tile, regardless of the value of [allowTapRouteToUserDetailsScreen]
  final VoidCallback? onTap;

  /// If set to [true], when [onTap] is [null], tapping on the [UserListTile]
  /// will open the [UserDetailsScreen] for this [user].
  ///
  /// Set to [false] to disable this behavior.
  final bool allowTapRouteToUserDetailsScreen;

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
      onTap: onTap ??
          (!allowTapRouteToUserDetailsScreen
              ? null
              : () async => await Navigator.push(
                    context,
                    MaterialPageRoute(
                        builder: (_) => UserDetailsScreen(user: user)),
                  )),
      title: title,
      subtitle: content,
      trailing: trailingIcon,
      isThreeLine: isThreeLine,
      leading: UserAvatar.small(user: user),
      visualDensity: VisualDensity.adaptivePlatformDensity,
    );
  }
}
