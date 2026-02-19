import 'package:flutter/material.dart';

import '../providers/providers.dart';

const String _navBarIconsPath = 'assets/icons/nav_bar';

String navBarIconPath(String name) {
  return '$_navBarIconsPath/$name.svg';
}

String navBarTabIconPath(TabType tab, bool selected) {
  final name = switch (tab) {
    TabType.users => 'people',
    TabType.public => 'public',
    TabType.chat => 'chat',
    TabType.network => 'network',
    TabType.account => null,
  };
  if (name == null) return '';
  final suffix = selected ? '-filled' : '-outlined';
  return '$_navBarIconsPath/$name$suffix.svg';
}

(Color, Color, Color) navBarColors(ThemeData theme) {
  if (theme.brightness == Brightness.dark) {
    return (
      kNavBarSelectedBackgroundDark,
      theme.iconTheme.color!,
      theme.navigationBarTheme.surfaceTintColor ?? theme.iconTheme.color!,
    );
  }
  return (
    kNavBarSelectedBackgroundLight,
    kNavBarIconColorLight,
    kNavBarIconColorLight,
  );
}

const double kNavBarSelectedSize = 45.0;
const double kNavBarSelectedRadius = 10.0;
const double kNavBarVerticalSpacing = 41.5;
const Color kNavBarSelectedBackgroundDark = Color(0xFF898989);
const Color kNavBarDarkBackground = Color(0xFF000000);
const double kNavBarMobileHeight = 100.0;
const double kNavBarMobileMargin = 16.0;
const double kNavBarHorizontalPadding = 8.0;
const double kNavBarVerticalTopSpacing = 24.0;
const double kNavBarVerticalMenuPadding = 24.0;
const double kNavBarLabelTopPadding = 4.0;
const double kNavBarVerticalWidthPercentage = 0.1;
const double kNavBarVerticalMaxWidth = 1000.0;
const double kNavBarVerticalDefaultWidth = 80.0;
const double kNavBarMenuSplashRadius = 20.0;
const double kNavBarBadgeFontSize = 10.0;
const double kNavBarBadgePositionOffset = 8.0;

const _navBarTabIconSizes = {
  TabType.chat: Size(34, 21),
  TabType.network: Size(23, 23),
  TabType.users: Size(30, 18.34),
  TabType.public: Size(31, 26),
};

Size navBarTabIconSize(TabType tab) =>
    _navBarTabIconSizes[tab] ?? (throw StateError('$tab has no icon size'));

const Size kNavBarMenuIconSize = Size(4.92, 20);
const double kNavBarAccountSize = 40.0;
const Color kNavBarSelectedBackgroundLight = Color(0xFFE5E5E5);
const Color kNavBarIconColorLight = Color(0xFF000000);

const TextStyle kNavBarLabelStyle = TextStyle(
  fontFamily: 'Roboto',
  fontSize: 8,
  fontWeight: FontWeight.w600,
);

const TextStyle kNavBarAvatarTextStyle = TextStyle(
  fontFamily: 'Roboto',
  fontSize: 20,
  fontWeight: FontWeight.w300,
  color: Color(0xFFFFFFFF),
);
