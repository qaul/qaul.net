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
    this.useUserColorOnName = false,
    this.tapRoutesToDetailsScreen = false,
    this.nameTapRoutesToDetailsScreen = false,
    this.avatarTapRoutesToDetailsScreen = true,
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
    bool useUserColorOnName = false,
    bool tapRoutesToDetailsScreen = false,
    bool nameTapRoutesToDetailsScreen = false,
    bool avatarTapRoutesToDetailsScreen = true,
  }) {
    return QaulListTile._(
      user: user,
      key: key,
      content: content,
      trailingIcon: trailingIcon,
      trailingMetadata: trailingMetadata,
      onTap: onTap,
      tapRoutesToDetailsScreen: tapRoutesToDetailsScreen,
      nameTapRoutesToDetailsScreen: nameTapRoutesToDetailsScreen,
      avatarTapRoutesToDetailsScreen: avatarTapRoutesToDetailsScreen,
      useUserColorOnName: useUserColorOnName,
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
    bool useUserColorOnName = false,
  }) {
    return QaulListTile._(
      room: room,
      key: key,
      content: content,
      trailingIcon: trailingIcon,
      trailingMetadata: trailingMetadata,
      onTap: onTap,
      useUserColorOnName: useUserColorOnName,
      tapRoutesToDetailsScreen: false,
      avatarTapRoutesToDetailsScreen: false,
    );
  }

  final User? user;

  final ChatRoom? room;

  final Widget? content;

  final Widget? trailingIcon;

  final Widget? trailingMetadata;

  /// Override the behavior of tapping this tile, regardless of the value of [tapRoutesToDetailsScreen]
  final VoidCallback? onTap;

  /// If set to [true], when [onTap] is [null], tapping on the [QaulListTile]
  /// will open the [UserDetailsScreen] for this [user].
  ///
  /// Set to [false] to disable this behavior.
  final bool tapRoutesToDetailsScreen;

  final bool nameTapRoutesToDetailsScreen;

  final bool avatarTapRoutesToDetailsScreen;

  final bool useUserColorOnName;

  Color get _userColor => colorGenerationStrategy(user!.idBase58);

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context).textTheme;

    final username = Text(
      user?.name ?? room?.name ?? 'Undefined Name',
      maxLines: 1,
      overflow: TextOverflow.ellipsis,
      style: theme.bodyText1!.copyWith(
        fontWeight: FontWeight.bold,
        color: useUserColorOnName ? _userColor : theme.bodyText1!.color,
      ),
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

    final onTapFallback = !tapRoutesToDetailsScreen
        ? null
        : () => _navigateToUserDetailsScreen(context, user!);

    final onNameTapFallback = !nameTapRoutesToDetailsScreen
        ? null
        : () => _navigateToUserDetailsScreen(context, user!);

    final onAvatarTapFallback = !avatarTapRoutesToDetailsScreen
        ? null
        : () => _navigateToUserDetailsScreen(context, user!);

    Widget leading =
        user != null ? QaulAvatar.small(user: user) : QaulAvatar.groupSmall();

    if (onAvatarTapFallback != null) {
      leading = GestureDetector(onTap: onAvatarTapFallback, child: leading);
    }

    Widget tileTitle = title;
    if (onNameTapFallback != null) {
      tileTitle = GestureDetector(
        onTap: onNameTapFallback,
        child: title,
      );
    }

    return InkWell(
      onTap: onTap ?? onTapFallback,
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 8),
        child: Row(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            leading,
            const SizedBox(width: 20),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  tileTitle,
                  if (content != null) ...[
                    const SizedBox(height: 4),
                    content!,
                  ]
                ],
              ),
            ),
            trailingIcon ?? const SizedBox(),
          ],
        ),
      ),
    );
  }

  void _navigateToUserDetailsScreen(BuildContext c, User? u) {
    if (u == null) return;
    Navigator.push(
        c, MaterialPageRoute(builder: (_) => UserDetailsScreen(user: u)));
  }
}
