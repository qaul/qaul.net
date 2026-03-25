import 'package:badges/badges.dart';
import 'package:flutter/material.dart' hide Badge;
import 'package:flutter_svg/flutter_svg.dart';
import '../styles/qaul_color_sheet.dart';

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
  final colorSheet = QaulColorSheet(theme.brightness);
  final iconColor = theme.iconTheme.color ?? Colors.white;
  if (theme.brightness == Brightness.dark) {
    return (
      colorSheet.surfaceContainer,
      iconColor,
      theme.navigationBarTheme.surfaceTintColor ?? iconColor,
    );
  }
  return (
    colorSheet.surfaceContainer,
    kNavBarIconColorLight,
    kNavBarIconColorLight,
  );
}

const double _kNavBarSelectedSize = 45.0;
const double _kNavBarSelectedRadius = 10.0;
const double _kNavBarVerticalSpacing = 41.5;
const Color _kNavBarDarkBackground = Color(0xFF000000);
const double _kNavBarMobileHeight = 100.0;
const double _kNavBarHorizontalPadding = 16.0;
const double _kNavBarVerticalTopSpacing = 24.0;
const double _kNavBarVerticalMenuPadding = 24.0;
// Vertical metrics lerp: when available height is between compact and loose,
// spacing is interpolated linearly. Below compact the navbar becomes scrollable
// via SingleChildScrollView as a last resort.
// Compact ≈ iPhone SE landscape minus safe-area (~300 px visible).
// Loose ≈ iPad portrait / large phone portrait (~520 px visible).
const double _kNavBarHeightCompact = 300.0;
const double _kNavBarHeightLoose = 520.0;
// Minimum gap used at the compact end of the lerp.
const double _kNavBarCompactGap = 10.0;
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
const Color kNavBarIconColorLight = Color(0xFF000000);

const TextStyle _kNavBarLabelStyle = TextStyle(
  fontFamily: 'Roboto',
  fontSize: 8,
  fontWeight: FontWeight.w600,
);

typedef _NavBarVerticalMetrics = ({
  double topPadding,
  double gap,
  double menuPadding,
});

_NavBarVerticalMetrics _navBarVerticalMetricsForHeight(double maxHeight) {
  if (!maxHeight.isFinite || maxHeight <= 0) {
    return (
      topPadding: _kNavBarVerticalTopSpacing,
      gap: _kNavBarVerticalSpacing,
      menuPadding: _kNavBarVerticalMenuPadding,
    );
  }
  final t =
      ((maxHeight - _kNavBarHeightCompact) / (_kNavBarHeightLoose - _kNavBarHeightCompact))
          .clamp(0.0, 1.0);
  double lerpLoose(double compact, double loose) => compact + (loose - compact) * t;
  return (
    topPadding: lerpLoose(_kNavBarCompactGap, _kNavBarVerticalTopSpacing),
    gap: lerpLoose(_kNavBarCompactGap, _kNavBarVerticalSpacing),
    menuPadding: lerpLoose(_kNavBarCompactGap, _kNavBarVerticalMenuPadding),
  );
}

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

    final ltr = Directionality.of(context) == TextDirection.ltr;
    final isLandscape =
        MediaQuery.orientationOf(context) == Orientation.landscape;

    final bar = LayoutBuilder(
      builder: (context, constraints) {
        final width = constraints.maxWidth.isFinite
            ? (constraints.maxWidth * _kNavBarVerticalWidthPercentage).clamp(
                0.0,
                _kNavBarVerticalMaxWidth,
              )
            : _kNavBarVerticalDefaultWidth;
        final maxH = constraints.maxHeight;
        final metrics = _navBarVerticalMetricsForHeight(maxH);
        final hasBoundedHeight = maxH.isFinite;

        final tabList = Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            SizedBox(height: metrics.topPadding),
            _NavBarItem(
              tab: TabType.account,
              isSelected: selectedTab == TabType.account,
              onTap: () => onTabSelected(TabType.account),
              avatarChild: avatarChild,
              tooltip: tooltips[TabType.account] ?? '',
            ),
            SizedBox(height: metrics.gap),
            _NavBarItem(
              tab: TabType.public,
              isSelected: selectedTab == TabType.public,
              onTap: () => onTabSelected(TabType.public),
              tooltip: tooltips[TabType.public] ?? '',
              badgeCount: publicNotificationCount,
            ),
            SizedBox(height: metrics.gap),
            _NavBarItem(
              tab: TabType.users,
              isSelected: selectedTab == TabType.users,
              onTap: () => onTabSelected(TabType.users),
              tooltip: tooltips[TabType.users] ?? '',
            ),
            SizedBox(height: metrics.gap),
            _NavBarItem(
              tab: TabType.chat,
              isSelected: selectedTab == TabType.chat,
              onTap: () => onTabSelected(TabType.chat),
              tooltip: tooltips[TabType.chat] ?? '',
              badgeCount: chatNotificationCount,
            ),
            SizedBox(height: metrics.gap),
            _NavBarItem(
              tab: TabType.network,
              isSelected: selectedTab == TabType.network,
              onTap: () => onTabSelected(TabType.network),
              tooltip: tooltips[TabType.network] ?? '',
            ),
          ],
        );

        final menuSection = Padding(
          padding: EdgeInsets.symmetric(vertical: metrics.menuPadding),
          child: menuButton,
        );

        final barChild = hasBoundedHeight
            ? Column(
                children: [
                  Expanded(
                    child: SingleChildScrollView(
                      physics: const ClampingScrollPhysics(),
                      child: tabList,
                    ),
                  ),
                  menuSection,
                ],
              )
            : Column(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  tabList,
                  menuSection,
                ],
              );

        return ConstrainedBox(
          constraints: BoxConstraints(maxWidth: width),
          child: _BarBackground(
            vertical: true,
            child: barChild,
          ),
        );
      },
    );

    return SafeArea(
      top: !isLandscape,
      left: isLandscape ? !ltr : true,
      right: isLandscape ? ltr : true,
      bottom: false,
      child: bar,
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
        child: _BarBackground(
          vertical: false,
          child: Padding(
            padding: const EdgeInsets.symmetric(
              horizontal: _kNavBarHorizontalPadding,
            ),
            child: Row(
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

class _BarBackground extends StatelessWidget {
  const _BarBackground({required this.vertical, required this.child});

  final bool vertical;
  final Widget child;

  @override
  Widget build(BuildContext context) {
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

Widget _buildVerticalMenuButton({
  required Map<NavBarOverflowOption, String> overflowMenuLabels,
  required void Function(NavBarOverflowOption) onOverflowSelected,
}) {
  return PopupMenuButton<NavBarOverflowOption>(
    onSelected: onOverflowSelected,
    splashRadius: _kNavBarMenuSplashRadius,
    itemBuilder: (context) => NavBarOverflowOption.values
        .map(
          (option) => PopupMenuItem<NavBarOverflowOption>(
            value: option,
            child: Text(overflowMenuLabels[option]!),
          ),
        )
        .toList(),
    child: Builder(
      builder: (context) {
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
    ),
  );
}

class _NavBarOverflowMenuButton extends StatelessWidget {
  _NavBarOverflowMenuButton({
    required this.overflowMenuLabels,
    required this.onOverflowSelected,
    required this.iconBuilder,
  });

  final Map<NavBarOverflowOption, String> overflowMenuLabels;
  final void Function(NavBarOverflowOption) onOverflowSelected;
  final Widget Function(BuildContext context) iconBuilder;

  final _iconKey = GlobalKey();

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
          key: _iconKey,
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
                final renderBox =
                    _iconKey.currentContext?.findRenderObject() as RenderBox?;
                if (renderBox == null) return;

                final origin = renderBox.localToGlobal(Offset.zero);
                final size = renderBox.size;

                final selected = await showMenu<NavBarOverflowOption>(
                  context: context,
                  position: RelativeRect.fromLTRB(
                    origin.dx,
                    origin.dy,
                    origin.dx + size.width,
                    origin.dy + size.height,
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
