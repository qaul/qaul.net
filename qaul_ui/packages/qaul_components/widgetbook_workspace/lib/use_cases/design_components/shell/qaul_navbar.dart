import 'package:flutter/material.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:widgetbook_annotation/widgetbook_annotation.dart' as widgetbook;

import '../../../support/widgetbook_preview.dart';

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

  Widget _buildContentArea(BuildContext context) {
    final theme = Theme.of(context);
    return ColoredBox(
      color: widgetbookChatSurfaceColor(context),
      child: Center(
        child: Text(
          'Content preview area',
          style: theme.textTheme.bodyMedium?.copyWith(
            color: theme.colorScheme.onSurface,
          ),
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final bar = QaulNavBar(
      vertical: widget.vertical,
      onOverflowSelected: (_) {},
      selectedTab: _selectedTab,
      onTabSelected: (tab) => setState(() => _selectedTab = tab),
      publicNotificationCount: widget.vertical ? 1 : 2,
      chatNotificationCount: widget.vertical ? 2 : 3,
    );

    final sheet = widgetbookColorSheet(context);

    if (widget.vertical) {
      return Material(
        color: sheet.background,
        child: SizedBox.expand(
          child: Row(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              bar,
              Expanded(child: _buildContentArea(context)),
            ],
          ),
        ),
      );
    }

    return Material(
      color: sheet.background,
      child: SizedBox.expand(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.end,
          children: [
            Expanded(child: _buildContentArea(context)),
            bar,
          ],
        ),
      ),
    );
  }
}
