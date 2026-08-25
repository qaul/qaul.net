import 'package:flutter/material.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:widgetbook_annotation/widgetbook_annotation.dart' as widgetbook;

@widgetbook.UseCase(
  name: 'Settings',
  type: QaulPageHeader,
  path: 'design_components/settings',
)
Widget buildQaulSettingsHeaderUseCase(BuildContext context) {
  return Material(
    color: qaulSettingsBackgroundColor(context),
    child: Column(
      children: [
        QaulPageHeader(
          title: 'Settings',
          leadingVisual: const Icon(Icons.settings_outlined),
          actions: [
            IconButton(
              tooltip: 'More',
              onPressed: () {},
              icon: const Icon(Icons.more_vert),
            ),
          ],
          onBackPressed: () {},
        ),
        const Expanded(child: SizedBox.expand()),
      ],
    ),
  );
}

@widgetbook.UseCase(
  name: 'Overview',
  type: QaulSettingsMenuItem,
  path: 'design_components/settings',
)
Widget buildQaulSettingsComponentsOverviewUseCase(BuildContext context) {
  return const _QaulSettingsComponentsOverview();
}

@widgetbook.UseCase(
  name: 'States',
  type: QaulSettingsMenuItem,
  path: 'design_components/settings',
)
Widget buildQaulSettingsMenuItemStatesUseCase(BuildContext context) {
  return Material(
    color: qaulSettingsBackgroundColor(context),
    child: Center(
      child: ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: 720),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            QaulSettingsMenuItem(
              icon: const Icon(Icons.translate),
              title: 'Language',
              value: 'System\'s default',
              onTap: () {},
            ),
            QaulSettingsMenuItem(
              icon: const Icon(Icons.palette),
              title: 'Theme',
              value: 'Dark mode',
              onTap: () {},
            ),
            QaulSettingsMenuItem(
              icon: const Icon(Icons.notifications),
              title: 'Notifications',
              onTap: () {},
            ),
            const QaulSettingsMenuItem(
              icon: Icon(Icons.info_outline),
              title: 'Read only',
              value: 'No action',
            ),
          ],
        ),
      ),
    ),
  );
}

@widgetbook.UseCase(
  name: 'Selectable list',
  type: QaulSettingsOptionItem,
  path: 'design_components/settings',
)
Widget buildQaulSettingsOptionItemUseCase(BuildContext context) {
  return const _QaulSettingsOptionItemUseCase();
}

class _QaulSettingsComponentsOverview extends StatefulWidget {
  const _QaulSettingsComponentsOverview();

  @override
  State<_QaulSettingsComponentsOverview> createState() =>
      _QaulSettingsComponentsOverviewState();
}

class _QaulSettingsComponentsOverviewState
    extends State<_QaulSettingsComponentsOverview> {
  var _selectedLanguage = 'System\'s default';

  @override
  Widget build(BuildContext context) {
    final backgroundColor = qaulSettingsBackgroundColor(context);

    return Material(
      color: backgroundColor,
      child: SizedBox.expand(
        child: SingleChildScrollView(
          child: Center(
            child: ConstrainedBox(
              constraints: const BoxConstraints(maxWidth: 720),
              child: Padding(
                padding: const EdgeInsets.symmetric(vertical: 32),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.stretch,
                  children: [
                    Column(
                      children: [
                        QaulSettingsMenuItem(
                          icon: const Icon(Icons.translate),
                          title: 'Language',
                          value: _selectedLanguage,
                          onTap: () {},
                        ),
                        QaulSettingsMenuItem(
                          icon: const Icon(Icons.palette),
                          title: 'Theme',
                          value: 'Dark mode',
                          onTap: () {},
                        ),
                        QaulSettingsMenuItem(
                          icon: const Icon(Icons.notifications),
                          title: 'Notifications',
                          onTap: () {},
                        ),
                        QaulSettingsMenuItem(
                          icon: const Icon(Icons.account_tree),
                          title: 'Network',
                          onTap: () {},
                        ),
                        QaulSettingsMenuItem(
                          icon: const Icon(Icons.supervisor_account),
                          title: 'Account Management',
                          onTap: () {},
                        ),
                      ],
                    ),
                    const SizedBox(height: 40),
                    Padding(
                      padding: kQaulSettingsContentPadding,
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.stretch,
                        children: [
                          QaulSettingsOptionItem(
                            label: 'System\'s default',
                            selected: _selectedLanguage == 'System\'s default',
                            onTap: () => setState(
                              () => _selectedLanguage = 'System\'s default',
                            ),
                          ),
                          QaulSettingsOptionItem(
                            label: 'English',
                            selected: _selectedLanguage == 'English',
                            onTap: () =>
                                setState(() => _selectedLanguage = 'English'),
                          ),
                          QaulSettingsOptionItem(
                            label: 'Deutsch',
                            selected: _selectedLanguage == 'Deutsch',
                            onTap: () =>
                                setState(() => _selectedLanguage = 'Deutsch'),
                          ),
                        ],
                      ),
                    ),
                  ],
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}

class _QaulSettingsOptionItemUseCase extends StatefulWidget {
  const _QaulSettingsOptionItemUseCase();

  @override
  State<_QaulSettingsOptionItemUseCase> createState() =>
      _QaulSettingsOptionItemUseCaseState();
}

class _QaulSettingsOptionItemUseCaseState
    extends State<_QaulSettingsOptionItemUseCase> {
  var _selected = 'System\'s default';

  @override
  Widget build(BuildContext context) {
    return Material(
      color: qaulSettingsBackgroundColor(context),
      child: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 720),
          child: Padding(
            padding: kQaulSettingsContentPadding,
            child: Column(
              mainAxisSize: MainAxisSize.min,
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                for (final option in const [
                  'System\'s default',
                  'English',
                  'Deutsch',
                  'Italiano',
                ])
                  QaulSettingsOptionItem(
                    label: option,
                    selected: _selected == option,
                    onTap: () => setState(() => _selected = option),
                  ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
