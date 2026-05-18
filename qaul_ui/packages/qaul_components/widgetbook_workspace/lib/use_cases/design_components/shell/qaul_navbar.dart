import 'package:flutter/material.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:widgetbook_annotation/widgetbook_annotation.dart' as widgetbook;

@widgetbook.UseCase(name: 'Horizontal (mobile)', type: QaulNavBar)
Widget buildNavBarHorizontalUseCase(BuildContext context) {
  return const _NavBarUseCase(vertical: false);
}

@widgetbook.UseCase(name: 'Vertical (tablet/desktop)', type: QaulNavBar)
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

  @override
  void initState() {
    super.initState();
    _selectedTab = widget.vertical ? TabType.chat : TabType.public;
  }

  Widget _buildContentArea() {
    return const Center(child: Text('Content preview area'));
  }

  @override
  Widget build(BuildContext context) {
    final bar = QaulNavBar(
      vertical: widget.vertical,
      overflowMenuLabels: navBarOverflowMenuLabelsDefault(),
      onOverflowSelected: (_) {},
      selectedTab: _selectedTab,
      onTabSelected: (tab) => setState(() => _selectedTab = tab),
      tabTooltips: QaulNavBar.defaultTabTooltips(),
      publicNotificationCount: widget.vertical ? 1 : 2,
      chatNotificationCount: widget.vertical ? 2 : 3,
    );

    if (widget.vertical) {
      return Material(
        child: SizedBox.expand(
          child: Row(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              bar,
              Expanded(child: _buildContentArea()),
            ],
          ),
        ),
      );
    }

    return Material(
      child: SizedBox.expand(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.end,
          children: [
            Expanded(child: _buildContentArea()),
            bar,
          ],
        ),
      ),
    );
  }
}
