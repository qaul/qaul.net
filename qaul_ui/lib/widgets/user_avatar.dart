part of 'widgets.dart';

/// If [user] is provided, it's used to populate this icon (Background color, initials, connection status).
///
/// Otherwise, the user found in [defaultUserProvider] is used.
abstract class UserAvatar extends ConsumerWidget {
  const UserAvatar({Key? key, this.user}) : super(key: key);
  final User? user;

  factory UserAvatar.tiny({Key? key, User? user}) => _TinyUserAvatar(key: key, user: user);

  factory UserAvatar.small({Key? key, User? user, bool badgeEnabled = true}) =>
      _SmallUserAvatar(key: key, user: user, badgeEnabled: badgeEnabled);

  factory UserAvatar.large({Key? key, User? user}) => _LargeUserAvatar(key: key, user: user);

  double get radius;

  TextStyle get initialsStyle;

  String get userInitials => initials(user!.name);

  Color get userColor => colorGenerationStrategy(user!.idBase58);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final defaultUser = ref.watch(defaultUserProvider);
    final defaultUserColor =
        defaultUser == null ? null : colorGenerationStrategy(defaultUser.idBase58);

    return CircleAvatar(
      radius: radius,
      child: Text(
        user != null
            ? userInitials
            : defaultUser != null
                ? initials(defaultUser.name)
                : 'WW',
        style: initialsStyle,
      ),
      backgroundColor: user != null ? userColor : defaultUserColor ?? Colors.red.shade700,
    );
  }

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

class _TinyUserAvatar extends UserAvatar {
  const _TinyUserAvatar({Key? key, User? user}) : super(key: key, user: user);

  @override
  double get radius => 14.0;

  @override
  TextStyle get initialsStyle => const TextStyle(
        fontSize: 12,
        color: Colors.white,
        fontWeight: FontWeight.w500,
      );
}

class _SmallUserAvatar extends UserAvatar {
  const _SmallUserAvatar({
    Key? key,
    User? user,
    this.badgeEnabled = true,
  }) : super(key: key, user: user);
  final bool badgeEnabled;

  @override
  double get radius => 20.0;

  @override
  TextStyle get initialsStyle => const TextStyle(
        fontSize: 16,
        color: Colors.white,
        fontWeight: FontWeight.w700,
      );

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    if (!badgeEnabled || userIsOffline(user)) {
      return super.build(context, ref);
    }

    return Badge(
      elevation: 0.0,
      toAnimate: false,
      padding: const EdgeInsets.all(6),
      borderSide: const BorderSide(color: Colors.white, width: 1.5),
      position: BadgePosition.bottomEnd(bottom: 0, end: 0),
      badgeColor: Colors.greenAccent.shade700,
      child: super.build(context, ref),
    );
  }

  bool userIsOffline(User? u) => u == null || !u.isConnected;
}

class _LargeUserAvatar extends UserAvatar {
  const _LargeUserAvatar({Key? key, User? user}) : super(key: key, user: user);

  @override
  double get radius => 80.0;

  @override
  TextStyle get initialsStyle => const TextStyle(
        fontSize: 68,
        color: Colors.white,
        fontWeight: FontWeight.w700,
      );
}
