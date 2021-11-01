import 'package:adaptive_theme/adaptive_theme.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_ui/helpers/navigation_helper.dart';
import 'package:qaul_ui/providers/providers.dart';
import 'package:qaul_ui/widgets/user_avatar.dart';

class QaulNavBarDecorator extends StatefulWidget {
  const QaulNavBarDecorator({Key? key, required this.child}) : super(key: key);
  final Widget child;

  @override
  State<QaulNavBarDecorator> createState() => _QaulNavBarDecoratorState();
}

class _QaulNavBarDecoratorState extends State<QaulNavBarDecorator> {
  static final _overflowMenuOptions = {'Settings', 'About'};

  void _handleClick(String value) {
    switch (value) {
      case 'Settings':
        Navigator.pushNamed(context, NavigationHelper.settings);
        break;
      case 'About':
        Navigator.pushNamed(context, NavigationHelper.about);
        break;
    }
  }

  @override
  Widget build(BuildContext context) {
    return ValueListenableBuilder<AdaptiveThemeMode>(
      valueListenable: AdaptiveTheme.of(context).modeChangeNotifier,
      builder: (_, mode, child) {
        var isDark = mode == AdaptiveThemeMode.dark;

        return DefaultSvgTheme(
          theme: SvgTheme(currentColor: isDark ? Colors.white : Colors.black),
          child: OrientationBuilder(
            builder: (context, orientation) {
              return Stack(
                alignment: orientation == Orientation.portrait
                    ? AlignmentDirectional.topCenter
                    : AlignmentDirectional.topStart,
                children: [
                  widget.child,
                  orientation == Orientation.portrait
                      ? _buildHorizontalBar(context)
                      : _buildVerticalBar(context),
                ],
              );
            },
          ),
        );
      },
    );
  }

  List<Widget> get _tabBarContent {
    return [
      const QaulNavBarItem(TabType.account),
      const QaulNavBarItem(TabType.feed),
      const QaulNavBarItem(TabType.users),
      const QaulNavBarItem(TabType.chat),
      const QaulNavBarItem(TabType.network),
      const SizedBox(width: 40, height: 40),
      PopupMenuButton<String>(
        onSelected: _handleClick,
        iconSize: 30,
        itemBuilder: (BuildContext context) {
          return _overflowMenuOptions.map((String choice) {
            return PopupMenuItem<String>(
              value: choice,
              child: Text(choice),
            );
          }).toList();
        },
      ),
    ];
  }

  Widget _buildHorizontalBar(BuildContext context) {
    return FractionallySizedBox(
      heightFactor: 0.12,
      child: _barBackground(
        context,
        Row(
          mainAxisAlignment: MainAxisAlignment.spaceEvenly,
          children: _tabBarContent,
        ),
      ),
    );
  }

  Widget _buildVerticalBar(BuildContext context) {
    return FractionallySizedBox(
      widthFactor: 0.1,
      child: _barBackground(
        context,
        Column(
          mainAxisAlignment: MainAxisAlignment.spaceEvenly,
          children: [
            SizedBox(height: MediaQuery.of(context).viewPadding.top),
            ..._tabBarContent,
          ],
        ),
      ),
    );
  }

  Widget _barBackground(BuildContext context, Widget child) => Container(
        alignment: Alignment.bottomCenter,
        color: Theme.of(context).primaryColor,
        padding: const EdgeInsets.all(8.0),
        child: child,
      );
}

class QaulNavBarItem extends ConsumerWidget {
  const QaulNavBarItem(this.tab, {Key? key}) : super(key: key);
  final TabType tab;

  final double _iconSize = 24.0;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final controller = ref.watch(selectedTabProvider);

    String svgPath;
    switch (tab) {
      case TabType.account:
        return UserAvatar(controller: controller, tab: tab);
      case TabType.users:
        return IconButton(
          icon: const Icon(Icons.group),
          onPressed: () => controller.goToTab(tab),
        );
      case TabType.feed:
        svgPath = 'assets/icons/hashtag.svg';
        break;
      case TabType.chat:
        svgPath = 'assets/icons/comments.svg';
        break;
      case TabType.network:
        svgPath = 'assets/icons/network.svg';
        break;
    }

    return ValueListenableBuilder<AdaptiveThemeMode>(
        valueListenable: AdaptiveTheme.of(context).modeChangeNotifier,
        builder: (_, mode, child) {
          var isDark = mode == AdaptiveThemeMode.dark;

          return IconButton(
            icon: SvgPicture.asset(
              svgPath,
              width: _iconSize,
              height: _iconSize,
              color: isDark ? Colors.white : Colors.black,
            ),
            onPressed: () => controller.goToTab(tab),
          );
        });
  }
}
