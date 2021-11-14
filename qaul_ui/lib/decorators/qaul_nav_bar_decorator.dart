import 'package:adaptive_theme/adaptive_theme.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:flutter_svg/flutter_svg.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_ui/helpers/navigation_helper.dart';
import 'package:qaul_ui/providers/providers.dart';
import 'package:qaul_ui/widgets/user_avatar.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';

class QaulNavBarDecorator extends StatefulWidget {
  const QaulNavBarDecorator({Key? key, required this.child}) : super(key: key);
  final Widget child;

  @override
  State<QaulNavBarDecorator> createState() => _QaulNavBarDecoratorState();
}

class _QaulNavBarDecoratorState extends State<QaulNavBarDecorator> {
  Map<String, String> get _overflowMenuOptions => {
        'settings': AppLocalizations.of(context)!.settings,
        'about': AppLocalizations.of(context)!.about,
      };

  void _handleClick(String value) {
    switch (value) {
      case 'settings':
        Navigator.pushNamed(context, NavigationHelper.settings);
        break;
      case 'about':
        Navigator.pushNamed(context, NavigationHelper.about);
        break;
    }
  }

  @override
  Widget build(BuildContext context) {
    return OrientationBuilder(
      builder: (context, orientation) {
        return Stack(
          alignment: orientation == Orientation.portrait
              ? AlignmentDirectional.topCenter
              : AlignmentDirectional.topStart,
          children: [
            orientation == Orientation.portrait
                ? _buildHorizontalBody()
                : _buildVerticalBody(),
            orientation == Orientation.portrait
                ? _buildHorizontalBar(context)
                : _buildVerticalBar(context),
          ],
        );
      },
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
                  QaulNavBarItem(TabType.feed),
                  QaulNavBarItem(TabType.users),
                  QaulNavBarItem(TabType.chat),
                  QaulNavBarItem(TabType.network),
                ],
              )
            : Row(
                mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                children: const [
                  QaulNavBarItem(TabType.feed),
                  QaulNavBarItem(TabType.users),
                  QaulNavBarItem(TabType.chat),
                  QaulNavBarItem(TabType.network),
                ],
              ),
      ),
      PopupMenuButton<String>(
        onSelected: _handleClick,
        iconSize: 36,
        icon: const Icon(Icons.more_horiz),
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

  Widget _buildHorizontalBody() {
    final top = MediaQuery.of(context).size.height * .12;
    return Padding(
      padding: EdgeInsets.only(top: top),
      child: widget.child,
    );
  }

  Widget _buildHorizontalBar(BuildContext context) {
    return FractionallySizedBox(
      heightFactor: 0.12,
      child: _barBackground(
        context,
        Padding(
          padding: const EdgeInsets.symmetric(horizontal: 8.0),
          child: Row(
            mainAxisAlignment: MainAxisAlignment.spaceEvenly,
            children: _tabBarContent(),
          ),
        ),
      ),
    );
  }

  Widget _buildVerticalBody() {
    final spacing = MediaQuery.of(context).size.width * .12;
    final pad = Directionality.of(context) == TextDirection.rtl
        ? EdgeInsets.only(right: spacing)
        : EdgeInsets.only(left: spacing);

    return Padding(padding: pad, child: widget.child);
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
            ..._tabBarContent(vertical: true),
          ],
        ),
        vertical: true,
      ),
    );
  }

  Widget _barBackground(BuildContext context, Widget child,
      {bool vertical = false}) {
    return ValueListenableBuilder<AdaptiveThemeMode>(
        valueListenable: AdaptiveTheme.of(context).modeChangeNotifier,
        builder: (_, mode, __) {
          var isDark = mode == AdaptiveThemeMode.dark;

          final ltr = Directionality.of(context) == TextDirection.ltr;
          var side = BorderSide(
            color:
                isDark ? Theme.of(context).primaryColor : Colors.grey.shade300,
          );
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
              color:
                  !isDark ? Colors.transparent : Theme.of(context).primaryColor,
            ),
            child: child,
          );
        });
  }
}

class QaulNavBarItem extends HookConsumerWidget {
  const QaulNavBarItem(this.tab, {Key? key}) : super(key: key);
  final TabType tab;

  final double _iconSize = 32.0;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final controller = ref.watch(selectedTabProvider);
    var selected = useState(false);

    useEffect(() {
      void updateSelected(int i) => selected.value = TabType.values[i] == tab;
      return controller.addListener(updateSelected);
    });

    var theme = Theme.of(context);
    final l18ns = AppLocalizations.of(context);

    String svgPath;
    String tooltip;
    switch (tab) {
      case TabType.account:
        return Padding(
          padding: const EdgeInsets.all(8.0),
          child: Tooltip(
            message: l18ns!.userAccountNavButtonTooltip,
            child: InkWell(
              onTap: () => controller.goToTab(tab),
              child: UserAvatar.small(badgeEnabled: false),
            ),
          ),
        );
      case TabType.users:
        return _SelectedIndicatorDecorator(
          selected: selected,
          selectedColor: theme.colorScheme.primary,
          child: Padding(
            padding: const EdgeInsets.all(8.0),
            child: IconButton(
              padding: const EdgeInsets.all(0.0),
              splashRadius: 0.01,
              tooltip: l18ns!.usersNavButtonTooltip,
              icon: Icon(Icons.group,
                  size: _iconSize,
                  color: selected.value
                      ? theme.colorScheme.primary
                      : theme.iconTheme.color),
              onPressed: () => controller.goToTab(tab),
            ),
          ),
        );
      case TabType.feed:
        svgPath = 'assets/icons/hashtag.svg';
        tooltip = l18ns!.feedNavButtonTooltip;
        break;
      case TabType.chat:
        svgPath = 'assets/icons/comments.svg';
        tooltip = l18ns!.chatNavButtonTooltip;
        break;
      case TabType.network:
        svgPath = 'assets/icons/network.svg';
        tooltip = l18ns!.netNavButtonTooltip;
        break;
    }

    return _SelectedIndicatorDecorator(
      selected: selected,
      selectedColor: theme.colorScheme.primary,
      child: Padding(
        padding: const EdgeInsets.all(8.0),
        child: IconButton(
          tooltip: tooltip,
          splashRadius: 0.01,
          padding: const EdgeInsets.all(8.0),
          icon: SvgPicture.asset(
            svgPath,
            width: _iconSize,
            height: _iconSize,
            color: selected.value
                ? theme.colorScheme.primary
                : theme.iconTheme.color,
          ),
          onPressed: () => controller.goToTab(tab),
        ),
      ),
    );
  }
}

class _SelectedIndicatorDecorator extends StatelessWidget {
  const _SelectedIndicatorDecorator({
    Key? key,
    required this.selected,
    required this.selectedColor,
    required this.child,
  }) : super(key: key);

  final ValueNotifier<bool> selected;
  final Color selectedColor;
  final Widget child;

  @override
  Widget build(BuildContext context) {
    return OrientationBuilder(builder: (context, orientation) {
      if (orientation != Orientation.landscape) return child;

      var indicatorLength = (24.0 + 8.0 + 8.0) * 1.5;
      var side = BorderSide(
        width: 2.0,
        color: selected.value ? selectedColor : Colors.transparent,
      );
      return Stack(
        alignment: AlignmentDirectional.bottomCenter,
        children: [
          child,
          Container(
            height: 2.0,
            width: indicatorLength,
            alignment: Alignment.bottomCenter,
            decoration: BoxDecoration(
              border: Border(bottom: side),
            ),
          ),
        ],
      );
    });
  }
}
