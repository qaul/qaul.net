import 'package:badges/badges.dart';
import 'package:flutter/material.dart' hide Badge;
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:flutter_svg/flutter_svg.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

import '../helpers/navigation_helper.dart';
import '../l10n/app_localizations.dart';
import '../providers/providers.dart';
import '../screens/home/tabs/tab.dart';
import '../widgets/widgets.dart';

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
                  const SizedBox(width: 8),
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
          QaulNavBar(
            vertical: false,
            overflowMenuLabels: _overflowMenuLabels(context),
            onOverflowSelected: (option) => _handleOverflowSelected(context, option),
          ),
          Expanded(child: widget.child(_pageViewKey)),
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
    return SafeArea(
      child: LayoutBuilder(
        builder: (context, constraints) {
          if (vertical) {
            final width = constraints.maxWidth.isFinite
                ? (constraints.maxWidth * 0.1).clamp(0.0, 1000.0)
                : 80.0;
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
          }
          final height = constraints.maxHeight.isFinite
              ? (constraints.maxHeight * 0.13).clamp(0.0, 600.0)
              : 104.0;
          final width = constraints.maxWidth.isFinite
              ? constraints.maxWidth
              : 400.0;
          return SizedBox(
            width: width,
            height: height,
            child: _barBackground(
              context,
              Padding(
                padding: const EdgeInsets.symmetric(horizontal: 8),
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                  children: _tabBarContent(context, vertical: false),
                ),
              ),
              vertical: false,
            ),
          );
        },
      ),
    );
  }

  List<Widget> _tabBarContent(BuildContext context, {required bool vertical}) {
    return [
      const QaulNavBarItem(TabType.account),
      Expanded(
        child: vertical
            ? const Column(
                mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                children: [
                  QaulNavBarItem(TabType.public),
                  QaulNavBarItem(TabType.users),
                  QaulNavBarItem(TabType.chat),
                  QaulNavBarItem(TabType.network),
                ],
              )
            : const Row(
                mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                children: [
                  QaulNavBarItem(TabType.public),
                  QaulNavBarItem(TabType.users),
                  QaulNavBarItem(TabType.chat),
                  QaulNavBarItem(TabType.network),
                ],
              ),
      ),
      PopupMenuButton<NavBarOverflowOption>(
        onSelected: onOverflowSelected,
        splashRadius: 20,
        iconSize: 32,
        icon: Icon(vertical ? Icons.more_horiz : Icons.more_vert),
        itemBuilder: (BuildContext context) {
          return NavBarOverflowOption.values
              .map((option) => PopupMenuItem<NavBarOverflowOption>(
                    value: option,
                    child: Text(overflowMenuLabels[option]!),
                  ))
              .toList();
        },
      ),
    ];
  }

  Widget _barBackground(BuildContext context, Widget child,
      {bool vertical = false}) {
    final ltr = Directionality.of(context) == TextDirection.ltr;
    final barTheme = Theme.of(context).appBarTheme;
    final side = BorderSide(color: barTheme.shadowColor ?? Colors.transparent);
    return Container(
      alignment: Alignment.bottomCenter,
      decoration: BoxDecoration(
        border: Border(
          bottom: vertical ? BorderSide.none : side,
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
        color: barTheme.backgroundColor,
      ),
      child: child,
    );
  }
}

class QaulNavBarItem extends HookConsumerWidget {
  const QaulNavBarItem(this.tab, {super.key});
  final TabType tab;

  final double _iconSize = 32.0;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final controller = ref.read(homeScreenControllerProvider.notifier);
    var selected = useState(false);

    useEffect(() {
      ref.listenManual(homeScreenControllerProvider, (previous, next) {
        selected.value = next == tab;
      });
      return null;
    }, []);

    var theme = Theme.of(context);
    final l18ns = AppLocalizations.of(context);

    String svgPath;
    String tooltip;
    double sizeFactor = 1.0;
    switch (tab) {
      case TabType.account:
        return Padding(
          padding: const EdgeInsets.all(8.0),
          child: Tooltip(
            message: l18ns!.userAccountNavButtonTooltip,
            child: InkWell(
              onTap: () => controller.goToTab(tab),
              splashColor: Colors.transparent,
              hoverColor: Colors.transparent,
              focusColor: Colors.transparent,
              highlightColor: Colors.transparent,
              child: QaulAvatar.small(badgeEnabled: false),
            ),
          ),
        );
      case TabType.users:
        svgPath = 'assets/icons/people.svg';
        tooltip = l18ns!.usersNavButtonTooltip;
        sizeFactor = 1.45;
        break;
      case TabType.public:
        svgPath = 'assets/icons/public.svg';
        tooltip = l18ns!.publicNavButtonTooltip;
        sizeFactor = 1.5;
        break;
      case TabType.chat:
        svgPath = 'assets/icons/chat.svg';
        tooltip = l18ns!.chatNavButtonTooltip;
        sizeFactor = 1.45;
        break;
      case TabType.network:
        svgPath = 'assets/icons/network.svg';
        tooltip = l18ns!.network;
        sizeFactor = 1.3;
        break;
    }

    final activeColor = Theme.of(context).navigationBarTheme.surfaceTintColor!;
    final button = _SelectedIndicatorDecorator(
      isSelected: selected,
      label: tooltip,
      selectedColor: activeColor,
      child: SizedBox(
        width: _iconSize * sizeFactor,
        height: _iconSize * sizeFactor,
        child: IconButton(
          tooltip: tooltip,
          splashRadius: 0.01,
          icon: SvgPicture.asset(
            svgPath,
            // fit: BoxFit.cover,
            fit: BoxFit.contain,
            colorFilter: ColorFilter.mode(
              selected.value ? activeColor : theme.iconTheme.color!,
              BlendMode.srcATop,
            ),
          ),
          onPressed: () => controller.goToTab(tab),
        ),
      ),
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

class _SelectedIndicatorDecorator extends StatelessWidget {
  const _SelectedIndicatorDecorator({
    required this.isSelected,
    required this.label,
    required this.selectedColor,
    required this.child,
  });

  final ValueNotifier<bool> isSelected;
  final Widget child;
  final String label;
  final Color selectedColor;

  @override
  Widget build(BuildContext context) {
    return OrientationBuilder(builder: (context, orientation) {
      if (orientation != Orientation.landscape) return child;

      var indicatorLength = (24.0 + 8.0 + 8.0) * 1.5;

      return Column(
        mainAxisAlignment: MainAxisAlignment.end,
        children: [
          child,
          Container(
              width: indicatorLength,
              margin: const EdgeInsets.only(bottom: 4),
              child: Text(
                label.toUpperCase(),
                textAlign: TextAlign.center,
                style: TextStyle(
                  fontSize: 8,
                  color: isSelected.value ? selectedColor : Colors.transparent,
                  fontWeight: FontWeight.bold,
                ),
              )),
        ],
      );
    });
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
                  fontSize: 10,
                  color: Colors.white,
                  fontWeight: FontWeight.w800,
                ),
              ),
              position: BadgePosition.bottomEnd(bottom: 8, end: 8),
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
