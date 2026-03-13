import 'package:badges/badges.dart';
import 'package:flutter/material.dart' hide Badge;
import 'package:flutter_svg/flutter_svg.dart';

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

enum TabType { account, public, users, chat, network }

enum NavBarOverflowOption {
  settings,
  about,
  license,
  support,
  oldNetwork,
  files,
}

// ---------------------------------------------------------------------------
// Public constants & helpers
// ---------------------------------------------------------------------------

const double kNavBarAccountSize = 40.0;
const double kNavBarMobileMargin = 16.0;

const TextStyle kNavBarAvatarTextStyle = TextStyle(
  fontFamily: 'Roboto',
  fontSize: 20,
  fontWeight: FontWeight.w300,
  color: Color(0xFFFFFFFF),
);

Map<NavBarOverflowOption, String> navBarOverflowMenuLabelsDefault() =>
    Map.from(_kNavBarOverflowMenuLabelsEn);

// ---------------------------------------------------------------------------
// Private constants & helpers
// ---------------------------------------------------------------------------

const String _kNavBarIconsPath = 'assets/icons';

@visibleForTesting
String navBarIconPath(String name) => '$_kNavBarIconsPath/$name.svg';

@visibleForTesting
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
  return '$_kNavBarIconsPath/$name$suffix.svg';
}

@visibleForTesting
(Color, Color, Color) navBarColors(ThemeData theme) {
  final iconColor = theme.iconTheme.color ?? Colors.white;
  if (theme.brightness == Brightness.dark) {
    return (
      kNavBarSelectedBackgroundDark,
      iconColor,
      theme.navigationBarTheme.surfaceTintColor ?? iconColor,
    );
  }
  return (
    kNavBarSelectedBackgroundLight,
    kNavBarIconColorLight,
    kNavBarIconColorLight,
  );
}

const double _kNavBarSelectedSize = 45.0;
const double _kNavBarSelectedRadius = 10.0;
const double _kNavBarVerticalSpacing = 41.5;
@visibleForTesting
const Color kNavBarSelectedBackgroundDark = Color(0xFF898989);
const Color _kNavBarDarkBackground = Color(0xFF000000);
const double _kNavBarMobileHeight = 100.0;
const double _kNavBarHorizontalPadding = 8.0;
const double _kNavBarVerticalTopSpacing = 24.0;
const double _kNavBarVerticalMenuPadding = 24.0;
const double _kNavBarLabelTopPadding = 4.0;
const double _kNavBarVerticalWidthPercentage = 0.1;
const double _kNavBarVerticalMaxWidth = 1000.0;
const double _kNavBarVerticalDefaultWidth = 80.0;
const double _kNavBarMenuSplashRadius = 20.0;
const double _kNavBarBadgeFontSize = 10.0;
const double _kNavBarBadgePositionOffset = 8.0;

const Map<TabType, Size> _kNavBarTabIconSizes = {
  TabType.chat: Size(34, 21),
  TabType.network: Size(23, 23),
  TabType.users: Size(30, 18.34),
  TabType.public: Size(31, 26),
};

@visibleForTesting
Size navBarTabIconSize(TabType tab) =>
    _kNavBarTabIconSizes[tab] ?? (throw StateError('$tab has no icon size'));

const Size _kNavBarMenuIconSize = Size(4.92, 20);
@visibleForTesting
const Color kNavBarSelectedBackgroundLight = Color(0xFFE5E5E5);
@visibleForTesting
const Color kNavBarIconColorLight = Color(0xFF000000);

const TextStyle _kNavBarLabelStyle = TextStyle(
  fontFamily: 'Roboto',
  fontSize: 8,
  fontWeight: FontWeight.w600,
);

const Map<NavBarOverflowOption, String> _kNavBarOverflowMenuLabelsEn = {
  NavBarOverflowOption.settings: 'Settings',
  NavBarOverflowOption.about: 'About',
  NavBarOverflowOption.license: 'AGPL License',
  NavBarOverflowOption.support: 'Support',
  NavBarOverflowOption.oldNetwork: 'Routing table',
  NavBarOverflowOption.files: 'File history',
};

// ---------------------------------------------------------------------------
// QaulNavBar widget
// ---------------------------------------------------------------------------

class QaulNavBar extends StatelessWidget {
  const QaulNavBar({
    super.key,
    required this.vertical,
    required this.overflowMenuLabels,
    required this.onOverflowSelected,
    required this.selectedTab,
    required this.onTabSelected,
    this.avatarChild,
    this.publicNotificationCount,
    this.chatNotificationCount,
    this.tabTooltips,
  });

  final bool vertical;
  final Map<NavBarOverflowOption, String> overflowMenuLabels;
  final void Function(NavBarOverflowOption) onOverflowSelected;
  final TabType selectedTab;
  final void Function(TabType) onTabSelected;
  final Widget? avatarChild;
  final int? publicNotificationCount;
  final int? chatNotificationCount;
  final Map<TabType, String>? tabTooltips;

  static Map<TabType, String> defaultTabTooltips() => {
    TabType.account: 'Account',
    TabType.public: 'Public',
    TabType.users: 'Users',
    TabType.chat: 'Chat',
    TabType.network: 'Network',
  };

  @override
  Widget build(BuildContext context) {
    if (vertical) {
      return SafeArea(
        child: LayoutBuilder(
          builder: (context, constraints) {
            final width = constraints.maxWidth.isFinite
                ? (constraints.maxWidth * _kNavBarVerticalWidthPercentage)
                      .clamp(0.0, _kNavBarVerticalMaxWidth)
                : _kNavBarVerticalDefaultWidth;
            return ConstrainedBox(
              constraints: BoxConstraints(maxWidth: width),
              child: _barBackground(
                context,
                Column(
                  mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                  children: _tabBarContent(context, vertical: true),
                ),
                vertical: true,
              ),
            );
          },
        ),
      );
    }
    return SafeArea(
      top: false,
      bottom: false,
      child: SizedBox(
        height: _kNavBarMobileHeight,
        child: _barBackground(
          context,
          Padding(
            padding: const EdgeInsets.symmetric(
              horizontal: _kNavBarHorizontalPadding,
            ),
            child: Row(
              mainAxisAlignment: MainAxisAlignment.spaceEvenly,
              children: _tabBarContent(context, vertical: false),
            ),
          ),
          vertical: false,
        ),
      ),
    );
  }

  List<Widget> _tabBarContent(BuildContext context, {required bool vertical}) {
    final menuButton = PopupMenuButton<NavBarOverflowOption>(
      onSelected: onOverflowSelected,
      splashRadius: _kNavBarMenuSplashRadius,
      iconSize: _kNavBarMenuIconSize.height,
      icon: Builder(
        builder: (context) {
          final theme = Theme.of(context);
          final color = theme.brightness == Brightness.dark
              ? (theme.iconTheme.color ?? Colors.white)
              : kNavBarIconColorLight;
          return SizedBox(
            width: _kNavBarMenuIconSize.width,
            height: _kNavBarMenuIconSize.height,
            child: SvgPicture.asset(
              navBarIconPath('menu'),
              package: 'qaul_components',
              width: _kNavBarMenuIconSize.width,
              height: _kNavBarMenuIconSize.height,
              fit: BoxFit.contain,
              colorFilter: ColorFilter.mode(color, BlendMode.srcATop),
            ),
          );
        },
      ),
      itemBuilder: (BuildContext context) {
        return NavBarOverflowOption.values
            .map(
              (option) => PopupMenuItem<NavBarOverflowOption>(
                value: option,
                child: Text(overflowMenuLabels[option]!),
              ),
            )
            .toList();
      },
    );

    final tooltips = tabTooltips ?? defaultTabTooltips();

    if (vertical) {
      return [
        Expanded(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  const SizedBox(height: _kNavBarVerticalTopSpacing),
                  _NavBarItem(
                    tab: TabType.account,
                    isSelected: selectedTab == TabType.account,
                    onTap: () => onTabSelected(TabType.account),
                    avatarChild: avatarChild,
                    tooltip: tooltips[TabType.account] ?? '',
                  ),
                  const SizedBox(height: _kNavBarVerticalSpacing),
                  _NavBarItem(
                    tab: TabType.public,
                    isSelected: selectedTab == TabType.public,
                    onTap: () => onTabSelected(TabType.public),
                    tooltip: tooltips[TabType.public] ?? '',
                    badgeCount: publicNotificationCount,
                  ),
                  const SizedBox(height: _kNavBarVerticalSpacing),
                  _NavBarItem(
                    tab: TabType.users,
                    isSelected: selectedTab == TabType.users,
                    onTap: () => onTabSelected(TabType.users),
                    tooltip: tooltips[TabType.users] ?? '',
                  ),
                  const SizedBox(height: _kNavBarVerticalSpacing),
                  _NavBarItem(
                    tab: TabType.chat,
                    isSelected: selectedTab == TabType.chat,
                    onTap: () => onTabSelected(TabType.chat),
                    tooltip: tooltips[TabType.chat] ?? '',
                    badgeCount: chatNotificationCount,
                  ),
                  const SizedBox(height: _kNavBarVerticalSpacing),
                  _NavBarItem(
                    tab: TabType.network,
                    isSelected: selectedTab == TabType.network,
                    onTap: () => onTabSelected(TabType.network),
                    tooltip: tooltips[TabType.network] ?? '',
                  ),
                ],
              ),
              Padding(
                padding: const EdgeInsets.symmetric(
                  vertical: _kNavBarVerticalMenuPadding,
                ),
                child: menuButton,
              ),
            ],
          ),
        ),
      ];
    }

    return [
      _NavBarItem(
        tab: TabType.account,
        isSelected: selectedTab == TabType.account,
        onTap: () => onTabSelected(TabType.account),
        avatarChild: avatarChild,
        tooltip: tooltips[TabType.account] ?? '',
      ),
      Expanded(
        child: Row(
          mainAxisAlignment: MainAxisAlignment.spaceEvenly,
          children: [
            _NavBarItem(
              tab: TabType.public,
              isSelected: selectedTab == TabType.public,
              onTap: () => onTabSelected(TabType.public),
              tooltip: tooltips[TabType.public] ?? '',
              badgeCount: publicNotificationCount,
            ),
            _NavBarItem(
              tab: TabType.users,
              isSelected: selectedTab == TabType.users,
              onTap: () => onTabSelected(TabType.users),
              tooltip: tooltips[TabType.users] ?? '',
            ),
            _NavBarItem(
              tab: TabType.chat,
              isSelected: selectedTab == TabType.chat,
              onTap: () => onTabSelected(TabType.chat),
              tooltip: tooltips[TabType.chat] ?? '',
              badgeCount: chatNotificationCount,
            ),
            _NavBarItem(
              tab: TabType.network,
              isSelected: selectedTab == TabType.network,
              onTap: () => onTabSelected(TabType.network),
              tooltip: tooltips[TabType.network] ?? '',
            ),
          ],
        ),
      ),
      menuButton,
    ];
  }

  Widget _barBackground(
    BuildContext context,
    Widget child, {
    bool vertical = false,
  }) {
    final theme = Theme.of(context);
    final ltr = Directionality.of(context) == TextDirection.ltr;
    final barTheme = theme.appBarTheme;
    final side = BorderSide(color: barTheme.shadowColor ?? Colors.transparent);
    final backgroundColor = theme.brightness == Brightness.dark
        ? _kNavBarDarkBackground
        : (barTheme.backgroundColor ?? Colors.transparent);
    return Container(
      alignment: Alignment.center,
      decoration: BoxDecoration(
        border: Border(
          bottom: BorderSide.none,
          left: !vertical ? BorderSide.none : (!ltr ? side : BorderSide.none),
          right: !vertical ? BorderSide.none : (ltr ? side : BorderSide.none),
        ),
        color: backgroundColor,
      ),
      child: child,
    );
  }
}

// ---------------------------------------------------------------------------
// _NavBarItem (private)
// ---------------------------------------------------------------------------

class _NavBarItem extends StatelessWidget {
  const _NavBarItem({
    required this.tab,
    required this.isSelected,
    required this.onTap,
    required this.tooltip,
    this.avatarChild,
    this.badgeCount,
  });

  final TabType tab;
  final bool isSelected;
  final VoidCallback onTap;
  final String tooltip;
  final Widget? avatarChild;
  final int? badgeCount;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final (selectedBackgroundColor, iconColor, activeColor) = navBarColors(
      theme,
    );

    if (tab == TabType.account) {
      return SizedBox(
        width: kNavBarAccountSize,
        height: kNavBarAccountSize,
        child: Tooltip(
          message: tooltip,
          child: InkWell(
            onTap: onTap,
            splashColor: Colors.transparent,
            hoverColor: Colors.transparent,
            focusColor: Colors.transparent,
            highlightColor: Colors.transparent,
            borderRadius: BorderRadius.circular(kNavBarAccountSize / 2),
            child: Center(
              child:
                  avatarChild ??
                  CircleAvatar(
                    radius: kNavBarAccountSize / 2,
                    backgroundColor: Colors.grey.shade700,
                    child: const Text('WW', style: kNavBarAvatarTextStyle),
                  ),
            ),
          ),
        ),
      );
    }

    final svgPath = navBarTabIconPath(tab, isSelected);
    final iconSize = navBarTabIconSize(tab);

    final iconWidget = SvgPicture.asset(
      svgPath,
      package: 'qaul_components',
      width: iconSize.width,
      height: iconSize.height,
      fit: BoxFit.contain,
      colorFilter: ColorFilter.mode(
        isSelected ? activeColor : iconColor,
        BlendMode.srcATop,
      ),
    );

    final content = Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        SizedBox(
          width: _kNavBarSelectedSize,
          height: _kNavBarSelectedSize,
          child: Material(
            color: isSelected ? selectedBackgroundColor : Colors.transparent,
            borderRadius: BorderRadius.circular(_kNavBarSelectedRadius),
            child: InkWell(
              onTap: onTap,
              borderRadius: BorderRadius.circular(_kNavBarSelectedRadius),
              splashColor: Colors.transparent,
              highlightColor: Colors.transparent,
              child: Tooltip(
                message: tooltip,
                child: Center(
                  child: SizedBox(
                    width: iconSize.width,
                    height: iconSize.height,
                    child: iconWidget,
                  ),
                ),
              ),
            ),
          ),
        ),
        if (isSelected && tooltip.isNotEmpty)
          Padding(
            padding: const EdgeInsets.only(top: _kNavBarLabelTopPadding),
            child: Text(
              tooltip.toUpperCase(),
              style: _kNavBarLabelStyle.copyWith(color: activeColor),
              maxLines: 1,
              overflow: TextOverflow.ellipsis,
              textAlign: TextAlign.center,
            ),
          ),
      ],
    );

    if (badgeCount != null && badgeCount! > 0) {
      return Badge(
        showBadge: true,
        badgeStyle: const BadgeStyle(badgeColor: Colors.lightBlue),
        badgeContent: Text(
          '$badgeCount',
          style: const TextStyle(
            fontSize: _kNavBarBadgeFontSize,
            color: Colors.white,
            fontWeight: FontWeight.w800,
          ),
        ),
        position: BadgePosition.bottomEnd(
          bottom: _kNavBarBadgePositionOffset,
          end: _kNavBarBadgePositionOffset,
        ),
        child: content,
      );
    }
    return content;
  }
}
