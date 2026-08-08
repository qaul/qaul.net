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

  qc.QaulAvatarSize get componentSize;

  bool get componentBadgeEnabled => false;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final defaultUser = ref.watch(defaultUserProvider);
    final avatarUser = user ?? defaultUser;
    if (avatarUser == null) return const qc.QaulAvatar.blank();

    final avatar = qc.QaulAvatar(
      name: avatarUser.name,
      id: avatarUser.idBase58,
      size: componentSize,
    );

    return qc.QaulAvatarBadge(
      size: componentSize,
      isVisible: componentBadgeEnabled && avatarUser.isConnected,
      child: avatar,
    );
  }
}

class _TinyQaulAvatar extends QaulAvatar {
  const _TinyQaulAvatar({super.key, super.user});

  @override
  qc.QaulAvatarSize get componentSize => qc.QaulAvatarSize.tiny;
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
  qc.QaulAvatarSize get componentSize => qc.QaulAvatarSize.small;

  @override
  bool get componentBadgeEnabled => badgeEnabled;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    if (isGroup) {
      return const qc.QaulAvatar.group(size: qc.QaulAvatarSize.small);
    }

    return super.build(context, ref);
  }
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
  qc.QaulAvatarSize get componentSize => qc.QaulAvatarSize.large;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    if (!isGroup && !isBlankUser) return super.build(context, ref);
    return isGroup
        ? const qc.QaulAvatar.group(size: qc.QaulAvatarSize.large)
        : const qc.QaulAvatar.blank(size: qc.QaulAvatarSize.large);
  }
}
