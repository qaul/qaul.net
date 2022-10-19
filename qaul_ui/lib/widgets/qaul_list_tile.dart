part of 'widgets.dart';

class QaulListTile extends StatelessWidget {
  const QaulListTile._({
    Key? key,
    this.user,
    this.room,
    this.content,
    this.trailingIcon,
    this.trailingMetadata,
    this.onTap,
    this.onAvatarTap,
    this.isThreeLine = false,
    this.allowTapRouteToUserDetailsScreen = false,
    this.allowAvatarTapRouteToUserDetailsScreen = true,
  })  : assert(trailingIcon == null || trailingMetadata == null),
        assert(user != null || room != null),
        super(key: key);

  factory QaulListTile.user(
    User user, {
    Key? key,
    Widget? content,
    Widget? trailingIcon,
    Widget? trailingMetadata,
    VoidCallback? onTap,
    VoidCallback? onAvatarTap,
    bool isThreeLine = false,
    bool allowTapRouteToUserDetailsScreen = false,
    bool allowAvatarTapRouteToUserDetailsScreen = true,
  }) {
    return QaulListTile._(
      user: user,
      key: key,
      content: content,
      trailingIcon: trailingIcon,
      trailingMetadata: trailingMetadata,
      onTap: onTap,
      isThreeLine: isThreeLine,
      allowTapRouteToUserDetailsScreen: allowTapRouteToUserDetailsScreen,
      onAvatarTap: onAvatarTap,
      allowAvatarTapRouteToUserDetailsScreen:
          allowAvatarTapRouteToUserDetailsScreen,
    );
  }

  factory QaulListTile.group(
    ChatRoom room, {
    Key? key,
    Widget? content,
    Widget? trailingIcon,
    Widget? trailingMetadata,
    VoidCallback? onTap,
    VoidCallback? onAvatarTap,
    bool isThreeLine = false,
    bool allowTapRouteToUserDetailsScreen = false,
    bool allowAvatarTapRouteToUserDetailsScreen = true,
  }) {
    return QaulListTile._(
      room: room,
      key: key,
      content: content,
      trailingIcon: trailingIcon,
      trailingMetadata: trailingMetadata,
      onTap: onTap,
      isThreeLine: isThreeLine,
      allowTapRouteToUserDetailsScreen: allowTapRouteToUserDetailsScreen,
      onAvatarTap: onAvatarTap,
      allowAvatarTapRouteToUserDetailsScreen:
          allowAvatarTapRouteToUserDetailsScreen,
    );
  }

  final User? user;

  final ChatRoom? room;

  final bool isThreeLine;

  /// Content displayed under the username
  final Widget? content;

  /// Center-aligned widget, displayed at the end of tile.
  final Widget? trailingIcon;

  /// Right, Baseline-aligned with username (usually a Text).
  final Widget? trailingMetadata;

  /// Override the behavior of tapping this tile, regardless of the value of [allowTapRouteToUserDetailsScreen]
  final VoidCallback? onTap;

  /// If set to [true], when [onTap] is [null], tapping on the [QaulListTile]
  /// will open the [UserDetailsScreen] for this [user].
  ///
  /// Set to [false] to disable this behavior.
  final bool allowTapRouteToUserDetailsScreen;

  final VoidCallback? onAvatarTap;

  final bool allowAvatarTapRouteToUserDetailsScreen;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context).textTheme;

    final username = Text(
      user?.name ?? room?.name ?? 'Undefined Name',
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

    final onTapFallback = !allowTapRouteToUserDetailsScreen
        ? null
        : () => _navigateToUserDetailsScreen(context, user!);

    final onAvatarTapFallback = !allowAvatarTapRouteToUserDetailsScreen
        ? null
        : () => _navigateToUserDetailsScreen(context, user!);

    Widget leading =
        user != null ? QaulAvatar.small(user: user) : QaulAvatar.groupSmall();

    if (onAvatarTap != null || onAvatarTapFallback != null) {
      leading = GestureDetector(
          onTap: onAvatarTap ?? onAvatarTapFallback, child: leading);
    }

    Widget tileTitle = title;
    if (onAvatarTapFallback != null) {
      tileTitle = GestureDetector(
        onTap: onAvatarTapFallback,
        child: title,
      );
    }

    return ListTile(
      onTap: onTap ?? onTapFallback,
      title: tileTitle,
      subtitle: content,
      trailing: trailingIcon,
      isThreeLine: isThreeLine,
      leading: leading,
      visualDensity: VisualDensity.adaptivePlatformDensity,
    );
  }

  void _navigateToUserDetailsScreen(BuildContext c, User? u) {
    if (u == null) return;
    Navigator.push(
        c, MaterialPageRoute(builder: (_) => UserDetailsScreen(user: u)));
  }
}
