import 'package:badges/badges.dart';
import 'package:flutter/material.dart' hide Badge;
import 'package:flutter_svg/flutter_svg.dart';

import '../constants/qaul_nav_bar_constants.dart';
import '../helper/qaul_nav_bar_helper.dart';

class QaulNavBarWidget extends StatelessWidget {
  const QaulNavBarWidget({
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
                ? (constraints.maxWidth * kNavBarVerticalWidthPercentage)
                    .clamp(0.0, kNavBarVerticalMaxWidth)
                : kNavBarVerticalDefaultWidth;
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
        height: kNavBarMobileHeight,
        child: _barBackground(
          context,
          Padding(
            padding: const EdgeInsets.symmetric(horizontal: kNavBarHorizontalPadding),
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
      splashRadius: kNavBarMenuSplashRadius,
      iconSize: kNavBarMenuIconSize.height,
      icon: Builder(
        builder: (context) {
          final theme = Theme.of(context);
          final color = theme.brightness == Brightness.dark
              ? (theme.iconTheme.color ?? Colors.white)
              : kNavBarIconColorLight;
          return SizedBox(
            width: kNavBarMenuIconSize.width,
            height: kNavBarMenuIconSize.height,
            child: SvgPicture.asset(
              navBarIconPath('menu'),
              width: kNavBarMenuIconSize.width,
              height: kNavBarMenuIconSize.height,
              fit: BoxFit.contain,
              colorFilter: ColorFilter.mode(color, BlendMode.srcATop),
            ),
          );
        },
      ),
      itemBuilder: (BuildContext context) {
        return NavBarOverflowOption.values
            .map((option) => PopupMenuItem<NavBarOverflowOption>(
                  value: option,
                  child: Text(overflowMenuLabels[option]!),
                ))
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
                  const SizedBox(height: kNavBarVerticalTopSpacing),
                  _NavBarItem(
                    tab: TabType.account,
                    isSelected: selectedTab == TabType.account,
                    onTap: () => onTabSelected(TabType.account),
                    avatarChild: avatarChild,
                    tooltip: tooltips[TabType.account] ?? '',
                  ),
                  const SizedBox(height: kNavBarVerticalSpacing),
                  _NavBarItem(
                    tab: TabType.public,
                    isSelected: selectedTab == TabType.public,
                    onTap: () => onTabSelected(TabType.public),
                    tooltip: tooltips[TabType.public] ?? '',
                    badgeCount: publicNotificationCount,
                  ),
                  const SizedBox(height: kNavBarVerticalSpacing),
                  _NavBarItem(
                    tab: TabType.users,
                    isSelected: selectedTab == TabType.users,
                    onTap: () => onTabSelected(TabType.users),
                    tooltip: tooltips[TabType.users] ?? '',
                  ),
                  const SizedBox(height: kNavBarVerticalSpacing),
                  _NavBarItem(
                    tab: TabType.chat,
                    isSelected: selectedTab == TabType.chat,
                    onTap: () => onTabSelected(TabType.chat),
                    tooltip: tooltips[TabType.chat] ?? '',
                    badgeCount: chatNotificationCount,
                  ),
                  const SizedBox(height: kNavBarVerticalSpacing),
                  _NavBarItem(
                    tab: TabType.network,
                    isSelected: selectedTab == TabType.network,
                    onTap: () => onTabSelected(TabType.network),
                    tooltip: tooltips[TabType.network] ?? '',
                  ),
                ],
              ),
              Padding(
                padding: const EdgeInsets.symmetric(vertical: kNavBarVerticalMenuPadding),
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

  Widget _barBackground(BuildContext context, Widget child, {bool vertical = false}) {
    final theme = Theme.of(context);
    final ltr = Directionality.of(context) == TextDirection.ltr;
    final barTheme = theme.appBarTheme;
    final side = BorderSide(color: barTheme.shadowColor ?? Colors.transparent);
    final backgroundColor = theme.brightness == Brightness.dark
        ? kNavBarDarkBackground
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
    final (selectedBackgroundColor, iconColor, activeColor) = navBarColors(theme);

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
              child: avatarChild ??
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
          width: kNavBarSelectedSize,
          height: kNavBarSelectedSize,
          child: Material(
            color: isSelected ? selectedBackgroundColor : Colors.transparent,
            borderRadius: BorderRadius.circular(kNavBarSelectedRadius),
            child: InkWell(
              onTap: onTap,
              borderRadius: BorderRadius.circular(kNavBarSelectedRadius),
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
            padding: const EdgeInsets.only(top: kNavBarLabelTopPadding),
            child: Text(
              tooltip.toUpperCase(),
              style: kNavBarLabelStyle.copyWith(color: activeColor),
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
            fontSize: kNavBarBadgeFontSize,
            color: Colors.white,
            fontWeight: FontWeight.w800,
          ),
        ),
        position: BadgePosition.bottomEnd(
          bottom: kNavBarBadgePositionOffset,
          end: kNavBarBadgePositionOffset,
        ),
        child: content,
      );
    }
    return content;
  }
}
