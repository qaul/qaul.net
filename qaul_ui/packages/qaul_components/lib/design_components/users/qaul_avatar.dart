import 'package:badges/badges.dart';
import 'package:flutter/material.dart' hide Badge;
import 'package:flutter_svg/flutter_svg.dart';
import 'package:utils/utils.dart';

const Color kQaulAvatarOnlineBadgeColor = Color(0xFF00C853);
const List<String> kQaulAvatarEmojiFontFamilyFallback = [
  'Apple Color Emoji',
  'Noto Color Emoji',
  'Segoe UI Emoji',
];

enum QaulAvatarSize {
  tiny(radius: 14, fontSize: 12, fontWeight: FontWeight.w500),
  small(radius: 20, fontSize: 16, fontWeight: FontWeight.w700),
  large(radius: 80, fontSize: 68, fontWeight: FontWeight.w700);

  const QaulAvatarSize({
    required this.radius,
    required this.fontSize,
    required this.fontWeight,
  });

  final double radius;
  final double fontSize;
  final FontWeight fontWeight;
}

class QaulAvatar extends StatelessWidget {
  const QaulAvatar({
    super.key,
    required this.name,
    required this.id,
    this.size = QaulAvatarSize.small,
  })  : isGroup = false,
        isBlank = false;

  const QaulAvatar.blank({
    super.key,
    this.size = QaulAvatarSize.large,
  })  : name = null,
        id = null,
        isGroup = false,
        isBlank = true;

  const QaulAvatar.group({
    super.key,
    this.size = QaulAvatarSize.small,
  })  : name = null,
        id = null,
        isGroup = true,
        isBlank = false;

  final String? name;
  final String? id;
  final QaulAvatarSize size;
  final bool isGroup;
  final bool isBlank;

  @override
  Widget build(BuildContext context) {
    if (isGroup || isBlank) {
      final icon = 'assets/icons/${isGroup ? 'group' : 'user'}.svg';
      return SvgPicture.asset(
        icon,
        width: size.radius * 2,
        height: size.radius * 2,
        package: 'qaul_components',
      );
    }

    final avatarInitials = initials(name!);
    return CircleAvatar(
      radius: size.radius,
      backgroundColor: colorGenerationStrategy(id!),
      child: SizedBox.square(
        dimension: size.radius * 2,
        child: Center(
          child: _AvatarInitials(
            text: avatarInitials,
            size: size,
          ),
        ),
      ),
    );
  }
}

class _AvatarInitials extends StatelessWidget {
  const _AvatarInitials({
    required this.text,
    required this.size,
  });

  final String text;
  final QaulAvatarSize size;

  @override
  Widget build(BuildContext context) {
    final isEmoji = isEmojiOnlyGrapheme(text);
    return Text(
      text,
      style: TextStyle(
        fontSize: size.fontSize,
        color: Colors.white,
        fontWeight: size.fontWeight,
        fontFamilyFallback:
            isEmoji ? kQaulAvatarEmojiFontFamilyFallback : null,
        height: isEmoji ? 1 : null,
      ),
      textAlign: TextAlign.center,
      strutStyle: isEmoji
          ? StrutStyle(
              fontSize: size.fontSize,
              height: 1,
              forceStrutHeight: true,
            )
          : null,
    );
  }
}

class QaulAvatarBadge extends StatelessWidget {
  const QaulAvatarBadge({
    super.key,
    required this.child,
    this.size = QaulAvatarSize.small,
    this.isVisible = true,
  });

  final Widget child;
  final QaulAvatarSize size;
  final bool isVisible;

  @override
  Widget build(BuildContext context) {
    if (!isVisible) return child;

    return Badge(
      position: BadgePosition.bottomEnd(bottom: 0, end: 0),
      badgeAnimation: const BadgeAnimation.slide(toAnimate: false),
      badgeStyle: BadgeStyle(
        elevation: 0.0,
        padding: EdgeInsets.all(size == QaulAvatarSize.tiny ? 4 : 6),
        badgeColor: kQaulAvatarOnlineBadgeColor,
        borderSide: BorderSide(
          color: Colors.white,
          width: size == QaulAvatarSize.tiny ? 0.5 : 1.5,
        ),
      ),
      child: child,
    );
  }
}
