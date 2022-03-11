import 'package:flutter/material.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:flutter_svg/flutter_svg.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

import '../helpers/navigation_helper.dart';
import '../providers/providers.dart';
import '../screens/home/tabs/tab.dart';
import '../screens/test_screen.dart';
import '../widgets/widgets.dart';

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
        // 'support': AppLocalizations.of(context)!.support,
        'support': 'Support',
        'old-network': 'Classic Network View',
        'test': "TEST",
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
              leading: const DefaultBackButton(),
              title: Row(
                children: const [
                  Icon(Icons.language),
                  SizedBox(width: 8),
                  Text('Classic Network View'),
                ],
              ),
            ),
            body: BaseTab.network(),
          );
        }));
        break;
      case 'test':
        Navigator.push(context, MaterialPageRoute(builder: (_) => const TestScreen()));
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
            orientation == Orientation.portrait ? _buildHorizontalBody() : _buildVerticalBody(),
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

  Widget _buildHorizontalBody() {
    final top = MediaQuery.of(context).size.height * .12;
    return Padding(
      padding: EdgeInsets.only(top: top),
      child: widget.child,
    );
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

  Widget _buildVerticalBody() {
    final spacing = MediaQuery.of(context).size.width * .12;
    final pad = Directionality.of(context) == TextDirection.rtl
        ? EdgeInsets.only(right: spacing)
        : EdgeInsets.only(left: spacing);

    return Padding(padding: pad, child: widget.child);
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

  Widget _barBackground(BuildContext context, Widget child, {bool vertical = false}) {
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
    final controller = ref.watch(selectedTabProvider);
    var selected = useState(false);

    useEffect(() {
      void updateSelected(SelectedTabStatus s) => selected.value = TabType.values[s.tab] == tab;
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
              splashColor: Colors.transparent,
              hoverColor: Colors.transparent,
              focusColor: Colors.transparent,
              highlightColor: Colors.transparent,
              child: UserAvatar.small(badgeEnabled: false),
            ),
          ),
        );
      case TabType.users:
        svgPath = 'assets/icons/people.svg';
        tooltip = l18ns!.feedNavButtonTooltip;
        break;
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
            color: selected.value ? theme.colorScheme.primary : theme.iconTheme.color,
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
