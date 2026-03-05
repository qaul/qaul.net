import 'package:flutter/material.dart';
import 'package:widgetbook_annotation/widgetbook_annotation.dart' as widgetbook;

import 'qaul_components.dart';

@widgetbook.UseCase(name: 'Horizontal (mobile)', type: QaulNavBarWidget)
Widget buildNavBarHorizontalUseCase(BuildContext context) {
  return const _NavBarUseCase(vertical: false);
}

@widgetbook.UseCase(name: 'Vertical (tablet/desktop)', type: QaulNavBarWidget)
Widget buildNavBarVerticalUseCase(BuildContext context) {
  return const _NavBarUseCase(vertical: true);
}

class _NavBarUseCase extends StatefulWidget {
  const _NavBarUseCase({required this.vertical});

  final bool vertical;

  @override
  State<_NavBarUseCase> createState() => _NavBarUseCaseState();
}

class _NavBarUseCaseState extends State<_NavBarUseCase> {
  late TabType _selectedTab;
  var _darkMode = false;

  @override
  void initState() {
    super.initState();
    _selectedTab = widget.vertical ? TabType.chat : TabType.public;
  }

  Widget _buildContentArea() {
    return Center(
      child: SwitchListTile(
        value: _darkMode,
        onChanged: (value) => setState(() => _darkMode = value),
        title: const Text('Dark mode'),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final theme = _darkMode ? ThemeData.dark() : ThemeData.light();
    final bar = Theme(
      data: theme,
      child: QaulNavBarWidget(
        vertical: widget.vertical,
        overflowMenuLabels: navBarOverflowMenuLabelsDefault(),
        onOverflowSelected: (_) {},
        selectedTab: _selectedTab,
        onTabSelected: (tab) => setState(() => _selectedTab = tab),
        tabTooltips: QaulNavBarWidget.defaultTabTooltips(),
        publicNotificationCount: widget.vertical ? null : 2,
        chatNotificationCount: widget.vertical ? 1 : null,
      ),
    );

    if (widget.vertical) {
      return Theme(
        data: theme,
        child: Material(
          child: SizedBox.expand(
            child: Row(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                bar,
                Expanded(child: _buildContentArea()),
              ],
            ),
          ),
        ),
      );
    }

    return Theme(
      data: theme,
      child: Material(
        child: SizedBox.expand(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.end,
            children: [
              Expanded(child: _buildContentArea()),
              bar,
            ],
          ),
        ),
      ),
    );
  }
}
