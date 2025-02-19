part of 'widgets.dart';

/// If [user] is provided, it's used to populate this icon (Background color, initials, connection status).
///
/// Otherwise, the user found in [defaultUserProvider] is used.
abstract class QaulAvatar extends ConsumerWidget {
  const QaulAvatar({super.key, this.user});
  final User? user;

  factory QaulAvatar.tiny({Key? key, User? user}) =>
      _TinyQaulAvatar(key: key, user: user);

  factory QaulAvatar.small({Key? key, User? user, bool badgeEnabled = true}) =>
      _SmallQaulAvatar(key: key, user: user, badgeEnabled: badgeEnabled);

  factory QaulAvatar.large({Key? key, User? user}) =>
      _LargeQaulAvatar(key: key, user: user, isBlankUser: user == null);

  factory QaulAvatar.groupSmall({Key? key}) => const _SmallQaulAvatar(
        badgeEnabled: false,
        isGroup: true,
      );

  factory QaulAvatar.groupLarge({Key? key}) =>
      const _LargeQaulAvatar(isGroup: true);

  double get radius;

  TextStyle get initialsStyle;

  String get userInitials => initials(user!.name);

  Color get userColor => colorGenerationStrategy(user!.idBase58);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final defaultUser = ref.watch(defaultUserProvider);
    final defaultUserColor = defaultUser == null
        ? null
        : colorGenerationStrategy(defaultUser.idBase58);

    return CircleAvatar(
      radius: radius,
      backgroundColor:
          user != null ? userColor : defaultUserColor ?? Colors.red.shade700,
      child: Text(
        user != null
            ? userInitials
            : defaultUser != null
                ? initials(defaultUser.name)
                : 'WW',
        style: initialsStyle,
      ),
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

class _TinyQaulAvatar extends QaulAvatar {
  const _TinyQaulAvatar({super.key, super.user});

  @override
  double get radius => 14.0;

  @override
  TextStyle get initialsStyle => const TextStyle(
        fontSize: 12,
        color: Colors.white,
        fontWeight: FontWeight.w500,
      );
}

class _SmallQaulAvatar extends QaulAvatar {
  const _SmallQaulAvatar({
    super.key,
    super.user,
    this.badgeEnabled = true,
    this.isGroup = false,
  });
  final bool badgeEnabled;
  final bool isGroup;

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
    if (isGroup) {
      const groupIcon = 'assets/icons/group.svg';
      return SvgPicture.asset(groupIcon, width: radius * 2, height: radius * 2);
    }

    if (!badgeEnabled || userIsOffline(user)) {
      return super.build(context, ref);
    }

    return Badge(
      position: BadgePosition.bottomEnd(bottom: 0, end: 0),
      badgeAnimation: const BadgeAnimation.slide(toAnimate: false),
      badgeStyle: BadgeStyle(
        elevation: 0.0,
        padding: const EdgeInsets.all(6),
        badgeColor: Colors.greenAccent.shade700,
        borderSide: const BorderSide(color: Colors.white, width: 1.5),
      ),
      child: super.build(context, ref),
    );
  }

  bool userIsOffline(User? u) => u == null || !u.isConnected;
}

class _LargeQaulAvatar extends QaulAvatar {
  const _LargeQaulAvatar({
    super.key,
    super.user,
    this.isGroup = false,
    this.isBlankUser = false,
  });
  final bool isGroup;
  final bool isBlankUser;

  @override
  double get radius => 80.0;

  @override
  TextStyle get initialsStyle => const TextStyle(
        fontSize: 68,
        color: Colors.white,
        fontWeight: FontWeight.w700,
      );

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    if (!isGroup && !isBlankUser) return super.build(context, ref);
    final icon = 'assets/icons/${isGroup ? 'group' : 'user'}.svg';
    return SvgPicture.asset(icon, width: radius * 2, height: radius * 2);
  }
}
