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
const double _kNavBarHorizontalPadding = 16.0;
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
      return _QaulNavBarVerticalLayout(
        overflowMenuLabels: overflowMenuLabels,
        onOverflowSelected: onOverflowSelected,
        selectedTab: selectedTab,
        onTabSelected: onTabSelected,
        avatarChild: avatarChild,
        publicNotificationCount: publicNotificationCount,
        chatNotificationCount: chatNotificationCount,
        tabTooltips: tabTooltips,
      );
    }
    return _QaulNavBarHorizontalLayout(
      overflowMenuLabels: overflowMenuLabels,
      onOverflowSelected: onOverflowSelected,
      selectedTab: selectedTab,
      onTabSelected: onTabSelected,
      avatarChild: avatarChild,
      publicNotificationCount: publicNotificationCount,
      chatNotificationCount: chatNotificationCount,
      tabTooltips: tabTooltips,
    );
  }
}

class _QaulNavBarVerticalLayout extends StatelessWidget {
  const _QaulNavBarVerticalLayout({
    required this.overflowMenuLabels,
    required this.onOverflowSelected,
    required this.selectedTab,
    required this.onTabSelected,
    required this.avatarChild,
    required this.publicNotificationCount,
    required this.chatNotificationCount,
    required this.tabTooltips,
  });

  final Map<NavBarOverflowOption, String> overflowMenuLabels;
  final void Function(NavBarOverflowOption) onOverflowSelected;
  final TabType selectedTab;
  final void Function(TabType) onTabSelected;
  final Widget? avatarChild;
  final int? publicNotificationCount;
  final int? chatNotificationCount;
  final Map<TabType, String>? tabTooltips;

  @override
  Widget build(BuildContext context) {
    final tooltips = tabTooltips ?? QaulNavBar.defaultTabTooltips();
    final menuButton = _buildVerticalMenuButton(
      overflowMenuLabels: overflowMenuLabels,
      onOverflowSelected: onOverflowSelected,
    );

    return SafeArea(
      child: LayoutBuilder(
        builder: (context, constraints) {
          final width = constraints.maxWidth.isFinite
              ? (constraints.maxWidth * _kNavBarVerticalWidthPercentage).clamp(
                  0.0,
                  _kNavBarVerticalMaxWidth,
                )
              : _kNavBarVerticalDefaultWidth;
          return ConstrainedBox(
            constraints: BoxConstraints(maxWidth: width),
            child: _barBackground(
              context,
              vertical: true,
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
          );
        },
      ),
    );
  }
}

class _QaulNavBarHorizontalLayout extends StatelessWidget {
  const _QaulNavBarHorizontalLayout({
    required this.overflowMenuLabels,
    required this.onOverflowSelected,
    required this.selectedTab,
    required this.onTabSelected,
    required this.avatarChild,
    required this.publicNotificationCount,
    required this.chatNotificationCount,
    required this.tabTooltips,
  });

  final Map<NavBarOverflowOption, String> overflowMenuLabels;
  final void Function(NavBarOverflowOption) onOverflowSelected;
  final TabType selectedTab;
  final void Function(TabType) onTabSelected;
  final Widget? avatarChild;
  final int? publicNotificationCount;
  final int? chatNotificationCount;
  final Map<TabType, String>? tabTooltips;

  @override
  Widget build(BuildContext context) {
    final tooltips = tabTooltips ?? QaulNavBar.defaultTabTooltips();
    final menuButton = _buildHorizontalMenuButton(
      overflowMenuLabels: overflowMenuLabels,
      onOverflowSelected: onOverflowSelected,
    );

    return SafeArea(
      top: false,
      child: SizedBox(
        height: _kNavBarMobileHeight,
        child: _barBackground(
          context,
          vertical: false,
          child: Padding(
            padding: const EdgeInsets.symmetric(
              horizontal: _kNavBarHorizontalPadding,
            ),
            child: Row(
              crossAxisAlignment: CrossAxisAlignment.center,
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                _NavBarItem(
                  tab: TabType.account,
                  isSelected: selectedTab == TabType.account,
                  onTap: () => onTabSelected(TabType.account),
                  avatarChild: avatarChild,
                  tooltip: tooltips[TabType.account] ?? '',
                ),
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
                menuButton,
              ],
            ),
          ),
        ),
      ),
    );
  }
}

Widget _barBackground(
  BuildContext context, {
  required Widget child,
  required bool vertical,
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

Widget _buildMenuButtonBase({
  required Map<NavBarOverflowOption, String> overflowMenuLabels,
  required void Function(NavBarOverflowOption) onOverflowSelected,
  required EdgeInsetsGeometry? padding,
  required Widget Function(BuildContext context) iconBuilder,
}) {
  final icon = Builder(builder: (context) => iconBuilder(context));
  List<PopupMenuEntry<NavBarOverflowOption>> itemBuilderFn(
    BuildContext context,
  ) {
    return NavBarOverflowOption.values
        .map(
          (option) => PopupMenuItem<NavBarOverflowOption>(
            value: option,
            child: Text(overflowMenuLabels[option]!),
          ),
        )
        .toList();
  }

  if (padding == null) {
    return PopupMenuButton<NavBarOverflowOption>(
      onSelected: onOverflowSelected,
      splashRadius: _kNavBarMenuSplashRadius,
      itemBuilder: itemBuilderFn,
      child: icon,
    );
  }

  return PopupMenuButton<NavBarOverflowOption>(
    onSelected: onOverflowSelected,
    splashRadius: _kNavBarMenuSplashRadius,
    itemBuilder: itemBuilderFn,
    padding: padding,
    child: icon,
  );
}

Widget _buildVerticalMenuButton({
  required Map<NavBarOverflowOption, String> overflowMenuLabels,
  required void Function(NavBarOverflowOption) onOverflowSelected,
}) {
  return _buildMenuButtonBase(
    overflowMenuLabels: overflowMenuLabels,
    onOverflowSelected: onOverflowSelected,
    padding: null,
    iconBuilder: (context) {
      final theme = Theme.of(context);
      final color = theme.brightness == Brightness.dark
          ? (theme.iconTheme.color ?? Colors.white)
          : kNavBarIconColorLight;

      return SvgPicture.asset(
        navBarIconPath('menu'),
        package: 'qaul_components',
        fit: BoxFit.contain,
        colorFilter: ColorFilter.mode(color, BlendMode.srcATop),
      );
    },
  );
}

class _NavBarOverflowMenuButton extends StatelessWidget {
  const _NavBarOverflowMenuButton({
    required this.overflowMenuLabels,
    required this.onOverflowSelected,
    required this.iconBuilder,
  });

  final Map<NavBarOverflowOption, String> overflowMenuLabels;
  final void Function(NavBarOverflowOption) onOverflowSelected;
  final Widget Function(BuildContext context) iconBuilder;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final hoverColor = theme.brightness == Brightness.dark
        ? Colors.white.withValues(alpha: 0.10)
        : Colors.black.withValues(alpha: 0.06);

    const hitSize = 40.0;
    final iconW = _kNavBarMenuIconSize.width;
    final iconH = _kNavBarMenuIconSize.height;

    return Stack(
      clipBehavior: Clip.none,
      children: [
        SizedBox(
          width: iconW,
          height: iconH,
          child: iconBuilder(context),
        ),
        Positioned(
          left: (iconW - hitSize) / 2,
          top: (iconH - hitSize) / 2,
          width: hitSize,
          height: hitSize,
          child: Material(
            color: Colors.transparent,
            child: InkWell(
              borderRadius: BorderRadius.circular(hitSize / 2),
              hoverColor: hoverColor,
              splashColor: hoverColor,
              focusColor: Colors.transparent,
              onTapDown: (details) async {
                final renderBox = context.findRenderObject() as RenderBox?;
                if (renderBox == null) return;

                final origin = renderBox.localToGlobal(Offset.zero);

                final anchorLeft = origin.dx + (hitSize - iconW) / 2;
                final anchorTop = origin.dy + (hitSize - iconH) / 2;

                final selected = await showMenu<NavBarOverflowOption>(
                  context: context,
                  position: RelativeRect.fromLTRB(
                    anchorLeft,
                    anchorTop,
                    anchorLeft + iconW,
                    anchorTop + iconH,
                  ),
                  items: NavBarOverflowOption.values
                      .map(
                        (option) => PopupMenuItem<NavBarOverflowOption>(
                          value: option,
                          child: Text(overflowMenuLabels[option]!),
                        ),
                      )
                      .toList(),
                );

                if (selected != null) {
                  onOverflowSelected(selected);
                }
              },
            ),
          ),
        ),
      ],
    );
  }
}

Widget _buildHorizontalMenuButton({
  required Map<NavBarOverflowOption, String> overflowMenuLabels,
  required void Function(NavBarOverflowOption) onOverflowSelected,
}) {
  return _NavBarOverflowMenuButton(
    overflowMenuLabels: overflowMenuLabels,
    onOverflowSelected: onOverflowSelected,
    iconBuilder: (context) {
      final theme = Theme.of(context);
      final color = theme.brightness == Brightness.dark
          ? (theme.iconTheme.color ?? Colors.white)
          : kNavBarIconColorLight;

      return SvgPicture.asset(
        navBarIconPath('menu'),
        package: 'qaul_components',
        width: _kNavBarMenuIconSize.width,
        height: _kNavBarMenuIconSize.height,
        fit: BoxFit.contain,
        colorFilter: ColorFilter.mode(color, BlendMode.srcATop),
      );
    },
  );
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
            child: avatarChild ??
                CircleAvatar(
                  radius: kNavBarAccountSize / 2,
                  backgroundColor: Colors.grey.shade700,
                  child: const Text('WW', style: kNavBarAvatarTextStyle),
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

    final content = SizedBox(
      width: _kNavBarSelectedSize,
      height: _kNavBarSelectedSize,
      child: Stack(
        clipBehavior: Clip.none,
        alignment: Alignment.topCenter,
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
            Positioned(
              top: _kNavBarSelectedSize + _kNavBarLabelTopPadding,
              child: IgnorePointer(
                child: Text(
                  tooltip.toUpperCase(),
                  style: _kNavBarLabelStyle.copyWith(color: activeColor),
                  maxLines: 1,
                  overflow: TextOverflow.ellipsis,
                  textAlign: TextAlign.center,
                ),
              ),
            ),
        ],
      ),
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
