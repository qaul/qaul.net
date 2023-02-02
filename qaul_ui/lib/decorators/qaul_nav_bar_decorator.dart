import 'package:badges/badges.dart';
import 'package:flutter/material.dart' hide Badge;
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:flutter_svg/flutter_svg.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

import '../helpers/navigation_helper.dart';
import '../providers/providers.dart';
import '../screens/home/tabs/tab.dart';
import '../widgets/widgets.dart';

class QaulNavBarDecorator extends StatefulWidget {
  const QaulNavBarDecorator({Key? key, required this.child}) : super(key: key);

  /// The [pageViewKey] provided should be used in the tabs view, ensuring state is not
  /// lost when the window is resized.
  final Widget Function(GlobalKey pageViewKey) child;

  @override
  State<QaulNavBarDecorator> createState() => _QaulNavBarDecoratorState();
}

class _QaulNavBarDecoratorState extends State<QaulNavBarDecorator> {
  final _pageViewKey = GlobalKey();

  Map<String, String> get _overflowMenuOptions => {
        'settings': AppLocalizations.of(context)!.settings,
        'about': AppLocalizations.of(context)!.about,
        'support': AppLocalizations.of(context)!.support,
        'old-network': AppLocalizations.of(context)!.routingDataTable,
        'files': AppLocalizations.of(context)!.fileHistory,
      };

  void _handleClick(String value) {
    switch (value) {
      case 'settings':
        Navigator.pushNamed(context, NavigationHelper.settings);
        break;
      case 'about':
        Navigator.pushNamed(context, NavigationHelper.about);
        break;
      case 'support':
        Navigator.pushNamed(context, NavigationHelper.support);
        break;
      case 'old-network':
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
      case 'files':
        Navigator.pushNamed(context, NavigationHelper.fileHistory);
        break;
    }
  }

  @override
  Widget build(BuildContext context) {
    return ResponsiveLayout(
      mobileBody: Column(
        children: [
          _buildHorizontalBar(context),
          Expanded(child: widget.child(_pageViewKey)),
        ],
      ),
      tabletBody: Row(
        children: [
          _buildVerticalBar(context),
          Expanded(child: widget.child(_pageViewKey)),
        ],
      ),
    );
  }

  List<Widget> _tabBarContent({bool vertical = false}) {
    return [
      const QaulNavBarItem(TabType.account),
      Expanded(
        child: vertical
            ? Column(
                mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                children: const [
                  QaulNavBarItem(TabType.public),
                  QaulNavBarItem(TabType.users),
                  QaulNavBarItem(TabType.chat),
                  QaulNavBarItem(TabType.network),
                ],
              )
            : Row(
                mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                children: const [
                  QaulNavBarItem(TabType.public),
                  QaulNavBarItem(TabType.users),
                  QaulNavBarItem(TabType.chat),
                  QaulNavBarItem(TabType.network),
                ],
              ),
      ),
      PopupMenuButton<String>(
        onSelected: _handleClick,
        splashRadius: 20,
        iconSize: 36,
        icon: Icon(vertical ? Icons.more_horiz : Icons.more_vert),
        itemBuilder: (BuildContext context) {
          return _overflowMenuOptions.keys.map((String key) {
            return PopupMenuItem<String>(
              value: key,
              child: Text(_overflowMenuOptions[key]!),
            );
          }).toList();
        },
      ),
    ];
  }

  Widget _buildHorizontalBar(BuildContext context) {
    final safePadding = MediaQuery.of(context).padding.top;
    final safeFraction = safePadding / MediaQuery.of(context).size.height;

    return ConstrainedBox(
      constraints: BoxConstraints(maxHeight: 600 + safePadding),
      child: FractionallySizedBox(
        heightFactor: 0.12 + safeFraction,
        child: _barBackground(
          context,
          Padding(
            padding: EdgeInsets.only(left: 8, right: 8, top: safePadding),
            child: Row(
              mainAxisAlignment: MainAxisAlignment.spaceEvenly,
              children: _tabBarContent(),
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildVerticalBar(BuildContext context) {
    return ConstrainedBox(
      constraints: const BoxConstraints(maxWidth: 1000),
      child: FractionallySizedBox(
        widthFactor: 0.1,
        child: _barBackground(
          context,
          Column(
            mainAxisAlignment: MainAxisAlignment.spaceEvenly,
            children: [
              SizedBox(height: MediaQuery.of(context).viewPadding.top),
              ..._tabBarContent(vertical: true),
            ],
          ),
          vertical: true,
        ),
      ),
    );
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
  const QaulNavBarItem(this.tab, {Key? key}) : super(key: key);
  final TabType tab;

  final double _iconSize = 32.0;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final controller = ref.read(homeScreenControllerProvider.notifier);
    var selected = useState(false);

    useEffect(() {
      void updateSelected(int i) => selected.value = TabType.values[i] == tab;
      return controller.addListener(updateSelected);
    });

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
    Key? key,
    required this.isSelected,
    required this.label,
    required this.selectedColor,
    required this.child,
  }) : super(key: key);

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
    Key? key,
    required this.notificationCount,
    required this.onPressed,
    required this.child,
  }) : super(key: key);
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
            badges.Badge(
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
              position: badges.BadgePosition.bottomEnd(bottom: 8, end: 8),
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
