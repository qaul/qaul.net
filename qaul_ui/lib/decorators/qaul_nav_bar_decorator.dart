import 'package:badges/badges.dart';
import 'package:flutter/material.dart' hide Badge;
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:flutter_svg/flutter_svg.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:utils/utils.dart';

import '../helpers/navigation_helper.dart';
import '../l10n/app_localizations.dart';
import '../providers/providers.dart';
import '../screens/home/tabs/tab.dart';
import '../widgets/widgets.dart';

import 'qaul_nav_bar_constants.dart';

(Color, Color, Color) _navBarColors(ThemeData theme) {
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

enum NavBarOverflowOption {
  settings,
  about,
  license,
  support,
  oldNetwork,
  files,
}

class QaulNavBarDecorator extends StatefulWidget {
  const QaulNavBarDecorator({super.key, required this.child});

  /// The [pageViewKey] provided should be used in the tabs view, ensuring state is not
  /// lost when the window is resized.
  final Widget Function(GlobalKey pageViewKey) child;

  @override
  State<QaulNavBarDecorator> createState() => _QaulNavBarDecoratorState();
}

class _QaulNavBarDecoratorState extends State<QaulNavBarDecorator> {
  final _pageViewKey = GlobalKey();

  Map<NavBarOverflowOption, String> _overflowMenuLabels(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    return {
      NavBarOverflowOption.settings: l10n.settings,
      NavBarOverflowOption.about: l10n.about,
      NavBarOverflowOption.license: l10n.agplLicense,
      NavBarOverflowOption.support: l10n.support,
      NavBarOverflowOption.oldNetwork: l10n.routingDataTable,
      NavBarOverflowOption.files: l10n.fileHistory,
    };
  }

  void _handleOverflowSelected(BuildContext context, NavBarOverflowOption option) {
    switch (option) {
      case NavBarOverflowOption.settings:
        Navigator.pushNamed(context, NavigationHelper.settings);
        break;
      case NavBarOverflowOption.about:
        Navigator.pushNamed(context, NavigationHelper.about);
        break;
      case NavBarOverflowOption.license:
        Navigator.pushNamed(context, NavigationHelper.license);
        break;
      case NavBarOverflowOption.support:
        Navigator.pushNamed(context, NavigationHelper.support);
        break;
      case NavBarOverflowOption.oldNetwork:
        Navigator.push(context, MaterialPageRoute(builder: (_) {
          return Scaffold(
            appBar: AppBar(
              leading: const IconButtonFactory(),
              title: Row(
                children: [
                  const Icon(Icons.language),
                  const SizedBox(width: kNavBarHorizontalPadding),
                  Text(AppLocalizations.of(context)!.routingDataTable),
                ],
              ),
            ),
            body: BaseTab.network(),
          );
        }));
        break;
      case NavBarOverflowOption.files:
        Navigator.pushNamed(context, NavigationHelper.fileHistory);
        break;
    }
  }

  @override
  Widget build(BuildContext context) {
    return ResponsiveLayout(
      mobileBody: Column(
        children: [
          Expanded(child: widget.child(_pageViewKey)),
          QaulNavBar(
            vertical: false,
            overflowMenuLabels: _overflowMenuLabels(context),
            onOverflowSelected: (option) => _handleOverflowSelected(context, option),
          ),
        ],
      ),
      tabletBody: Row(
        children: [
          QaulNavBar(
            vertical: true,
            overflowMenuLabels: _overflowMenuLabels(context),
            onOverflowSelected: (option) => _handleOverflowSelected(context, option),
          ),
          Expanded(child: widget.child(_pageViewKey)),
        ],
      ),
    );
  }
}

class QaulNavBar extends StatelessWidget {
  const QaulNavBar({
    super.key,
    required this.vertical,
    required this.overflowMenuLabels,
    required this.onOverflowSelected,
  });

  final bool vertical;
  final Map<NavBarOverflowOption, String> overflowMenuLabels;
  final void Function(NavBarOverflowOption) onOverflowSelected;

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
              padding: const EdgeInsets.symmetric(
                  horizontal: kNavBarHorizontalPadding),
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
                  const QaulNavBarItem(TabType.account),
                  const SizedBox(height: kNavBarVerticalSpacing),
                  const QaulNavBarItem(TabType.public),
                  const SizedBox(height: kNavBarVerticalSpacing),
                  const QaulNavBarItem(TabType.users),
                  const SizedBox(height: kNavBarVerticalSpacing),
                  const QaulNavBarItem(TabType.chat),
                  const SizedBox(height: kNavBarVerticalSpacing),
                  const QaulNavBarItem(TabType.network),
                ],
              ),
              Padding(
                padding: const EdgeInsets.symmetric(
                    vertical: kNavBarVerticalMenuPadding),
                child: menuButton,
              ),
            ],
          ),
        ),
      ];
    }

    return [
      const QaulNavBarItem(TabType.account),
      Expanded(
        child: Row(
          mainAxisAlignment: MainAxisAlignment.spaceEvenly,
          children: const [
            QaulNavBarItem(TabType.public),
            QaulNavBarItem(TabType.users),
            QaulNavBarItem(TabType.chat),
            QaulNavBarItem(TabType.network),
          ],
        ),
      ),
      menuButton,
    ];
  }

  Widget _barBackground(BuildContext context, Widget child,
      {bool vertical = false}) {
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
          left: !vertical
              ? BorderSide.none
              : !ltr
                  ? side
                  : BorderSide.none,
          right: !vertical
              ? BorderSide.none
              : ltr
                  ? side
                  : BorderSide.none,
        ),
        color: backgroundColor,
      ),
      child: child,
    );
  }
}

class QaulNavBarItem extends HookConsumerWidget {
  const QaulNavBarItem(this.tab, {super.key});
  final TabType tab;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final controller = ref.read(homeScreenControllerProvider.notifier);
    var selected = useState(ref.read(homeScreenControllerProvider) == tab);

    useEffect(() {
      ref.listenManual(homeScreenControllerProvider, (previous, next) {
        selected.value = next == tab;
      });
      return null;
    }, []);

    final theme = Theme.of(context);
    final l18ns = AppLocalizations.of(context);
    final (selectedBackgroundColor, iconColor, activeColor) = _navBarColors(theme);

    switch (tab) {
      case TabType.account:
        return SizedBox(
          width: kNavBarAccountSize,
          height: kNavBarAccountSize,
          child: Tooltip(
            message: l18ns!.userAccountNavButtonTooltip,
            child: InkWell(
              onTap: () => controller.goToTab(tab),
              splashColor: Colors.transparent,
              hoverColor: Colors.transparent,
              focusColor: Colors.transparent,
              highlightColor: Colors.transparent,
              borderRadius: BorderRadius.circular(kNavBarAccountSize / 2),
              child: Center(
                child: Consumer(
                  builder: (context, ref, _) {
                    final user = ref.watch(defaultUserProvider);
                    final userColor = user != null
                        ? colorGenerationStrategy(user.idBase58)
                        : Colors.red.shade700;
                    final initialsText = user != null
                        ? initials(user.name)
                        : 'WW';
                    return CircleAvatar(
                      radius: kNavBarAccountSize / 2,
                      backgroundColor: userColor,
                      child: Text(
                        initialsText,
                        style: kNavBarAvatarTextStyle,
                      ),
                    );
                  },
                ),
              ),
            ),
          ),
        );
      case TabType.users:
      case TabType.public:
      case TabType.chat:
      case TabType.network:
        break;
    }

    final isSelected = selected.value;
    final String tooltip;
    switch (tab) {
      case TabType.users:
        tooltip = l18ns!.usersNavButtonTooltip;
        break;
      case TabType.public:
        tooltip = l18ns!.publicNavButtonTooltip;
        break;
      case TabType.chat:
        tooltip = l18ns!.chatNavButtonTooltip;
        break;
      case TabType.network:
        tooltip = l18ns!.network;
        break;
      default:
        tooltip = '';
    }
    final svgPath = navBarTabIconPath(tab, isSelected);
    final iconSize = navBarTabIconSize(tab);

    final iconWidget = SvgPicture.asset(
      svgPath,
      width: iconSize.width,
      height: iconSize.height,
      fit: BoxFit.contain,
      colorFilter: ColorFilter.mode(
        selected.value ? activeColor : iconColor,
        BlendMode.srcATop,
      ),
    );

    final squareWidget = SizedBox(
      width: kNavBarSelectedSize,
      height: kNavBarSelectedSize,
      child: Material(
        color: selected.value
            ? selectedBackgroundColor
            : Colors.transparent,
        borderRadius: BorderRadius.circular(kNavBarSelectedRadius),
        child: InkWell(
          onTap: () => controller.goToTab(tab),
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
    );

    final shouldShowLabel = tab != TabType.account && tooltip.isNotEmpty;
    final button = ValueListenableBuilder<bool>(
      valueListenable: selected,
      builder: (context, isSelected, _) {
        return Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            squareWidget,
            if (isSelected && shouldShowLabel)
              Padding(
                padding: const EdgeInsets.only(top: kNavBarLabelTopPadding),
                child: Text(
                  tooltip.toUpperCase(),
                  style: kNavBarLabelStyle.copyWith(
                    color: activeColor,
                  ),
                  maxLines: 1,
                  overflow: TextOverflow.ellipsis,
                  textAlign: TextAlign.center,
                ),
              ),
          ],
        );
      },
    );

    if (tab == TabType.public) {
      return _TabNotificationBadge(
        notificationCount:
            ref.read(publicNotificationControllerProvider).newNotificationCount,
        onPressed: () {
          controller.goToTab(tab);
          ref.read(publicNotificationControllerProvider).removeNotifications();
        },
        child: button,
      );
    } else if (tab == TabType.chat) {
      return _TabNotificationBadge(
        notificationCount:
            ref.read(chatNotificationControllerProvider).newNotificationCount,
        onPressed: () {
          controller.goToTab(tab);
          ref.read(publicNotificationControllerProvider).removeNotifications();
        },
        child: button,
      );
    }
    return button;
  }
}

class _TabNotificationBadge extends StatelessWidget {
  const _TabNotificationBadge({
    required this.notificationCount,
    required this.onPressed,
    required this.child,
  });
  final ValueNotifier<int?> notificationCount;
  final VoidCallback onPressed;
  final Widget child;

  @override
  Widget build(BuildContext context) {
    return ValueListenableBuilder<int?>(
      valueListenable: notificationCount,
      builder: (context, count, _) {
        return Stack(
          children: [
            Badge(
              showBadge: count != null,
              badgeStyle: const BadgeStyle(badgeColor: Colors.lightBlue),
              badgeContent: Text(
                '${count ?? ''}',
                style: const TextStyle(
                  fontSize: kNavBarBadgeFontSize,
                  color: Colors.white,
                  fontWeight: FontWeight.w800,
                ),
              ),
              position: BadgePosition.bottomEnd(
                  bottom: kNavBarBadgePositionOffset,
                  end: kNavBarBadgePositionOffset),
              child: child,
            ),
            Positioned.fill(
              child: GestureDetector(
                onTap: () {
                  notificationCount.value = null;
                  onPressed();
                },
              ),
            ),
          ],
        );
      },
    );
  }
}
