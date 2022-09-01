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
    this.isThreeLine = false,
    this.allowTapRouteToUserDetailsScreen = true,
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
    bool isThreeLine = false,
    bool allowTapRouteToUserDetailsScreen = true,
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
    );
  }

  factory QaulListTile.group(
    ChatRoom room, {
    Key? key,
    Widget? content,
    Widget? trailingIcon,
    Widget? trailingMetadata,
    VoidCallback? onTap,
    bool isThreeLine = false,
    bool allowTapRouteToUserDetailsScreen = true,
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

    final fallback = user == null
        ? () {}
        : (!allowTapRouteToUserDetailsScreen
            ? null
            : () async => await Navigator.push(
                  context,
                  MaterialPageRoute(
                      builder: (_) => UserDetailsScreen(user: user!)),
                ));

    var leading =
        user != null ? QaulAvatar.small(user: user) : QaulAvatar.groupSmall();

    return ListTile(
      onTap: onTap ?? fallback,
      title: title,
      subtitle: content,
      trailing: trailingIcon,
      isThreeLine: isThreeLine,
      leading: leading,
      visualDensity: VisualDensity.adaptivePlatformDensity,
    );
  }
}
