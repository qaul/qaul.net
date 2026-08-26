import 'package:flutter/material.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:widgetbook_annotation/widgetbook_annotation.dart' as widgetbook;

const _kAuthPreviewIconSize = 29.0;

@widgetbook.UseCase(
  name: 'States',
  type: QaulAuthActionRow,
  path: 'design_components/auth',
)
Widget buildQaulAuthActionRowUseCase(BuildContext context) {
  final secondaryColor = qaulAuthSecondaryTextColor(context);

  return _AuthPreviewFrame(
    child: Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        QaulAuthActionRow(
          icon: Icons.translate,
          label: 'Language',
          value: 'System\'s default',
          trailing: Icon(Icons.chevron_right, color: secondaryColor),
          onTap: _noop,
        ),
        const SizedBox(height: kQaulAuthItemGap),
        QaulAuthActionRow(
          icon: Icons.open_in_new,
          label: 'Learn about qaul',
          labelColor: secondaryColor,
          trailing: Icon(Icons.open_in_new, color: secondaryColor),
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
  final primaryColor = qaulAuthPrimaryTextColor(context);

  return QaulAuthPageScaffold(
    child: ListView(
      padding: const EdgeInsets.fromLTRB(28, 32, 28, 32),
      children: [
        QaulAuthSectionTitle(
          leading: _TintedPngAssetIcon(
            'assets/icons/auth/avatar_auth.png',
            color: primaryColor,
          ),
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
      color: qaulAuthBackgroundColor(context),
      child: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 420),
          child: Padding(padding: const EdgeInsets.all(26), child: child),
        ),
      ),
    );
  }
}

class _TintedPngAssetIcon extends StatelessWidget {
  const _TintedPngAssetIcon(this.assetName, {required this.color});

  final String assetName;
  final Color color;

  @override
  Widget build(BuildContext context) {
    return ColorFiltered(
      colorFilter: ColorFilter.mode(color, BlendMode.srcIn),
      child: Image.asset(
        assetName,
        width: _kAuthPreviewIconSize,
        height: _kAuthPreviewIconSize,
        fit: BoxFit.contain,
      ),
    );
  }
}
