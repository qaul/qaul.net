import 'package:flutter/material.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:widgetbook_annotation/widgetbook_annotation.dart' as widgetbook;

@widgetbook.UseCase(
  name: 'States',
  type: QaulAuthActionRow,
  path: 'design_components/auth',
)
Widget buildQaulAuthActionRowUseCase(BuildContext context) {
  return const _AuthPreviewFrame(
    child: Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        QaulAuthActionRow(
          icon: Icons.translate,
          label: 'Language',
          value: 'System\'s default',
          trailing: Icon(
            Icons.chevron_right,
            color: kQaulAuthSecondaryTextColor,
          ),
          onTap: _noop,
        ),
        SizedBox(height: kQaulAuthItemGap),
        QaulAuthActionRow(
          icon: Icons.open_in_new,
          label: 'Learn about qaul',
          labelColor: kQaulAuthSecondaryTextColor,
          trailing: Icon(Icons.open_in_new, color: kQaulAuthSecondaryTextColor),
          onTap: _noop,
        ),
      ],
    ),
  );
}

void _noop() {}

@widgetbook.UseCase(
  name: 'Login list',
  type: QaulAuthSegmentedList,
  path: 'design_components/auth',
)
Widget buildQaulAuthSegmentedListUseCase(BuildContext context) {
  return _AuthPreviewFrame(
    child: QaulAuthSegmentedList(
      children: [
        QaulAuthAccountTile(
          avatar: const QaulAvatar(
            name: 'anonymous',
            id: '12D3KooWAnonymous',
            size: QaulAvatarSize.tiny,
          ),
          name: 'anonymous',
          onTap: () {},
        ),
        QaulAuthAccountTile(
          avatar: const QaulAvatar(
            name: 'Anna K',
            id: '12D3KooWAnnaK',
            size: QaulAvatarSize.tiny,
          ),
          name: 'Anna K',
          onTap: () {},
        ),
        QaulAuthMoreTile(onTap: () {}),
      ],
    ),
  );
}

@widgetbook.UseCase(
  name: 'Expand accounts',
  type: QaulAuthExpandTile,
  path: 'design_components/auth',
)
Widget buildQaulAuthExpandTileUseCase(BuildContext context) {
  return _AuthPreviewFrame(
    child: QaulAuthSegmentedList(
      children: [
        QaulAuthAccountTile(
          avatar: const QaulAvatar(
            name: 'anonymous',
            id: '12D3KooWAnonymous',
            size: QaulAvatarSize.tiny,
          ),
          name: 'anonymous',
          onTap: () {},
        ),
        QaulAuthAccountTile(
          avatar: const QaulAvatar(
            name: 'Anna K',
            id: '12D3KooWAnnaK',
            size: QaulAvatarSize.tiny,
          ),
          name: 'Anna K',
          onTap: () {},
        ),
        QaulAuthExpandTile(onTap: () {}),
      ],
    ),
  );
}

@widgetbook.UseCase(
  name: 'Welcome',
  type: QaulAuthWelcomeSection,
  path: 'design_components/auth',
)
Widget buildQaulAuthWelcomeSectionUseCase(BuildContext context) {
  return _AuthPreviewFrame(
    child: QaulAuthWelcomeSection(onCreateAccount: () {}),
  );
}

@widgetbook.UseCase(
  name: 'Header shell',
  type: QaulAuthPageScaffold,
  path: 'design_components/auth',
)
Widget buildQaulAuthPageScaffoldUseCase(BuildContext context) {
  return QaulAuthPageScaffold(
    child: ListView(
      padding: const EdgeInsets.fromLTRB(28, 32, 28, 32),
      children: [
        const QaulAuthSectionTitle(
          icon: Icons.accessibility_new,
          label: 'Login',
        ),
        const SizedBox(height: kQaulAuthItemGap),
        QaulAuthSegmentedList(
          children: [
            QaulAuthAccountTile(
              avatar: const QaulAvatar(
                name: 'anonymous',
                id: '12D3KooWAnonymous',
                size: QaulAvatarSize.tiny,
              ),
              name: 'anonymous',
              onTap: () {},
            ),
          ],
        ),
        const SizedBox(height: kQaulAuthItemGap),
        QaulAuthActionRow(
          icon: Icons.supervisor_account_outlined,
          label: 'Import account',
          onTap: () {},
        ),
      ],
    ),
  );
}

class _AuthPreviewFrame extends StatelessWidget {
  const _AuthPreviewFrame({required this.child});

  final Widget child;

  @override
  Widget build(BuildContext context) {
    return Material(
      color: kQaulAuthBackgroundColor,
      child: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 420),
          child: Padding(padding: const EdgeInsets.all(26), child: child),
        ),
      ),
    );
  }
}
