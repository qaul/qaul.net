import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';

import '../../l10n/qaul_components_localizations.dart';
import '../../l10n/qaul_components_localizations_en.dart';
import '../../models/account.dart';

QaulComponentsLocalizations _l10n(BuildContext context) =>
    QaulComponentsLocalizations.of(context) ?? QaulComponentsLocalizationsEn();

class QaulAccountLogo extends StatelessWidget {
  const QaulAccountLogo({super.key});

  @override
  Widget build(BuildContext context) {
    return SvgPicture.asset(
      'assets/logo/logo.svg',
      package: 'qaul_components',
      fit: BoxFit.contain,
    );
  }
}

class QaulAccountButton extends StatelessWidget {
  const QaulAccountButton({
    super.key,
    required this.label,
    this.onPressed,
    this.foregroundColor,
  });

  final String label;
  final VoidCallback? onPressed;
  final Color? foregroundColor;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final color =
        foregroundColor ??
        theme.outlinedButtonTheme.style?.foregroundColor?.resolve(
          <WidgetState>{},
        );

    return SizedBox(
      width: 216,
      child: OutlinedButton(
        onPressed: onPressed,
        child: Padding(
          padding: const EdgeInsets.all(10),
          child: Text(
            label,
            textAlign: TextAlign.center,
            style: TextStyle(fontSize: 16, color: color),
          ),
        ),
      ),
    );
  }
}

class QaulAccountLanding extends StatelessWidget {
  const QaulAccountLanding({
    super.key,
    required this.state,
    required this.logo,
    this.languageSelector,
    this.onCreateAccount,
    this.onRestoreAccount,
    this.onLogin,
    this.onLearnMore,
  });

  final QaulAccountSessionState state;
  final Widget logo;
  final Widget? languageSelector;
  final VoidCallback? onCreateAccount;
  final VoidCallback? onRestoreAccount;
  final VoidCallback? onLogin;
  final VoidCallback? onLearnMore;

  @override
  Widget build(BuildContext context) {
    final labels = _l10n(context);

    return Scaffold(
      body: SafeArea(
        child: ListView(
          padding: const EdgeInsets.symmetric(horizontal: 40, vertical: 120),
          children: [
            const SizedBox(width: double.maxFinite),
            Center(child: SizedBox(width: 320, height: 320, child: logo)),
            const SizedBox(height: 20),
            if (languageSelector != null) ...[
              ConstrainedBox(
                constraints: const BoxConstraints(maxWidth: 360),
                child: languageSelector,
              ),
              const SizedBox(height: 20),
            ],
            Center(
              child: _LandingActions(
                state: state,
                onCreateAccount: onCreateAccount,
                onRestoreAccount: onRestoreAccount,
                onLogin: onLogin,
                onLearnMore: onLearnMore,
                labels: labels,
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class _LandingActions extends StatelessWidget {
  const _LandingActions({
    required this.state,
    required this.labels,
    this.onCreateAccount,
    this.onRestoreAccount,
    this.onLogin,
    this.onLearnMore,
  });

  final QaulAccountSessionState state;
  final QaulComponentsLocalizations labels;
  final VoidCallback? onCreateAccount;
  final VoidCallback? onRestoreAccount;
  final VoidCallback? onLogin;
  final VoidCallback? onLearnMore;

  @override
  Widget build(BuildContext context) {
    final buttons = switch (state) {
      QaulAccountSessionState.noLocalAccount => [
        QaulAccountButton(
          label: labels.accountCreateUserProfile,
          onPressed: onCreateAccount,
        ),
        QaulAccountButton(
          label: labels.accountRestoreAccount,
          onPressed: onRestoreAccount,
        ),
        QaulAccountButton(
          label: labels.accountLearnMore,
          onPressed: onLearnMore,
        ),
      ],
      QaulAccountSessionState.signedOut => [
        QaulAccountButton(
          label: labels.accountCreateUserProfile,
          onPressed: onCreateAccount,
        ),
        QaulAccountButton(
          label: labels.accountLoginExistingAccount,
          onPressed: onLogin,
        ),
        QaulAccountButton(
          label: labels.accountRestoreAccount,
          onPressed: onRestoreAccount,
        ),
        QaulAccountButton(
          label: labels.accountLearnMore,
          onPressed: onLearnMore,
        ),
      ],
      QaulAccountSessionState.signedIn => [
        QaulAccountButton(
          label: labels.accountLearnMore,
          onPressed: onLearnMore,
        ),
      ],
    };

    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        for (final button in buttons) ...[button, const SizedBox(height: 20)],
      ],
    );
  }
}

class QaulAccountSettingsSection extends StatelessWidget {
  const QaulAccountSettingsSection({
    super.key,
    this.onExportAccount,
    this.onChangePassword,
    this.onLogout,
    this.onDeleteAccount,
    this.showPasswordAction = true,
  });

  final VoidCallback? onExportAccount;
  final VoidCallback? onChangePassword;
  final VoidCallback? onLogout;
  final VoidCallback? onDeleteAccount;
  final bool showPasswordAction;

  @override
  Widget build(BuildContext context) {
    final labels = _l10n(context);

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        _SettingsSectionHeader(
          icon: const Icon(Icons.supervisor_account_outlined),
          label: labels.accountManageAccount,
        ),
        const SizedBox(height: 8),
        _AccountActionTile(
          icon: Icons.logout,
          label: labels.accountLogout,
          onTap: onLogout,
        ),
        _AccountActionTile(
          icon: Icons.file_upload_outlined,
          label: labels.accountExportAccount,
          onTap: onExportAccount,
        ),
        if (showPasswordAction)
          _AccountActionTile(
            icon: Icons.password_outlined,
            label: labels.accountChangeOrRemovePassword,
            onTap: onChangePassword,
          ),
        _AccountActionTile(
          icon: Icons.person_remove_outlined,
          label: labels.accountRemoveAccount,
          onTap: onDeleteAccount,
          destructive: true,
        ),
      ],
    );
  }
}

class QaulAccountHeading extends StatelessWidget {
  const QaulAccountHeading({super.key, required this.account});

  final QaulAccountSummary account;

  @override
  Widget build(BuildContext context) {
    final labels = _l10n(context);
    final theme = Theme.of(context).textTheme;

    return Row(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        _AccountAvatar(account: account),
        Expanded(
          child: Padding(
            padding: const EdgeInsets.symmetric(horizontal: 24),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  account.name,
                  style: theme.titleLarge,
                  maxLines: 1,
                  overflow: TextOverflow.ellipsis,
                ),
                const SizedBox(height: 8),
                Text(
                  account.id,
                  style: theme.titleSmall,
                  maxLines: 3,
                  overflow: TextOverflow.ellipsis,
                ),
                const SizedBox(height: 24),
                Row(
                  children: [
                    const Icon(Icons.key, size: 16),
                    const SizedBox(width: 8),
                    Expanded(
                      child: Text(
                        labels.accountPublicKey,
                        style: theme.titleMedium,
                        maxLines: 1,
                        overflow: TextOverflow.ellipsis,
                      ),
                    ),
                  ],
                ),
                const SizedBox(height: 8),
                Text(
                  account.publicKey ?? labels.accountUnknown,
                  maxLines: 3,
                  overflow: TextOverflow.ellipsis,
                ),
              ],
            ),
          ),
        ),
      ],
    );
  }
}

class _AccountAvatar extends StatelessWidget {
  const _AccountAvatar({required this.account});

  final QaulAccountSummary account;

  @override
  Widget build(BuildContext context) {
    return CircleAvatar(
      radius: 80,
      backgroundColor: _avatarColor(account.id),
      child: Text(
        _initials(account.name),
        style: const TextStyle(
          fontSize: 68,
          color: Colors.white,
          fontWeight: FontWeight.w700,
        ),
      ),
    );
  }

  Color _avatarColor(String seed) {
    const colors = Colors.primaries;
    final index =
        seed.codeUnits.fold<int>(0, (sum, code) => sum + code) % colors.length;
    return colors[index].shade700;
  }

  String _initials(String name) {
    final words = name.trim().split(RegExp(r'\s+'));
    if (words.isEmpty || words.first.isEmpty) return 'WW';
    final chars = words
        .where((word) => word.isNotEmpty)
        .take(2)
        .map((word) => word.characters.first.toUpperCase());
    return chars.join();
  }
}

class _SettingsSectionHeader extends StatelessWidget {
  const _SettingsSectionHeader({required this.icon, required this.label});

  final Widget icon;
  final String label;

  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        icon,
        Padding(
          padding: const EdgeInsets.symmetric(horizontal: 8),
          child: Text(label),
        ),
        const Expanded(child: Divider()),
      ],
    );
  }
}

class _AccountActionTile extends StatelessWidget {
  const _AccountActionTile({
    required this.icon,
    required this.label,
    this.onTap,
    this.destructive = false,
  });

  final IconData icon;
  final String label;
  final VoidCallback? onTap;
  final bool destructive;

  @override
  Widget build(BuildContext context) {
    final color = destructive ? Theme.of(context).colorScheme.error : null;
    return InkWell(
      onTap: onTap,
      child: Padding(
        padding: const EdgeInsets.symmetric(vertical: 10),
        child: Row(
          children: [
            Icon(icon, color: color),
            const SizedBox(width: 12),
            Expanded(
              child: Text(
                label,
                style: Theme.of(
                  context,
                ).textTheme.labelLarge?.copyWith(color: color),
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class QaulRestoreAccountDialog extends StatelessWidget {
  const QaulRestoreAccountDialog({super.key, this.onCancel, this.onConfirm});

  final VoidCallback? onCancel;
  final VoidCallback? onConfirm;

  @override
  Widget build(BuildContext context) {
    final labels = _l10n(context);
    return _QaulAccountDialog(
      title: labels.accountRestoreAccount,
      message: labels.accountRestoreDescription,
      primaryLabel: labels.accountRestoreContinue,
      onPrimary: onConfirm,
      secondaryLabel: labels.accountCancel,
      onSecondary: onCancel,
    );
  }
}

class QaulRestoreFilePickerDialog extends StatelessWidget {
  const QaulRestoreFilePickerDialog({super.key, this.onCancel, this.onConfirm});

  final VoidCallback? onCancel;
  final VoidCallback? onConfirm;

  @override
  Widget build(BuildContext context) {
    final labels = _l10n(context);
    return AlertDialog(
      title: Text(labels.accountRestoreAccount),
      content: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          Text(labels.accountRestoreFilePickerDescription),
          const SizedBox(height: 16),
          OutlinedButton(
            onPressed: onConfirm,
            child: Padding(
              padding: const EdgeInsets.symmetric(vertical: 8),
              child: Text(labels.accountChooseExportFile),
            ),
          ),
          const SizedBox(height: 8),
          Text(
            labels.accountRestoreFilePickerPlaceholder,
            style: Theme.of(context).textTheme.bodySmall,
            textAlign: TextAlign.center,
          ),
        ],
      ),
      actions: [
        TextButton(onPressed: onCancel, child: Text(labels.accountCancel)),
      ],
    );
  }
}

class QaulExportAccountDialog extends StatelessWidget {
  const QaulExportAccountDialog({super.key, this.onCancel, this.onConfirm});

  final VoidCallback? onCancel;
  final VoidCallback? onConfirm;

  @override
  Widget build(BuildContext context) {
    final labels = _l10n(context);
    return _QaulAccountDialog(
      title: labels.accountExportAccount,
      message: labels.accountExportDescription,
      primaryLabel: labels.accountExportAccount,
      onPrimary: onConfirm,
      secondaryLabel: labels.accountCancel,
      onSecondary: onCancel,
    );
  }
}

class QaulDeleteAccountExportPromptDialog extends StatelessWidget {
  const QaulDeleteAccountExportPromptDialog({
    super.key,
    this.onCancel,
    this.onExportFirst,
    this.onDeleteWithoutExport,
  });

  final VoidCallback? onCancel;
  final VoidCallback? onExportFirst;
  final VoidCallback? onDeleteWithoutExport;

  @override
  Widget build(BuildContext context) {
    final labels = _l10n(context);
    return AlertDialog(
      title: Text(labels.accountRemoveAccount),
      content: Text(labels.accountDeleteExportPrompt),
      actions: [
        TextButton(onPressed: onCancel, child: Text(labels.accountCancel)),
        TextButton(
          onPressed: onDeleteWithoutExport,
          child: Text(labels.accountDeleteWithoutExport),
        ),
        TextButton(
          onPressed: onExportFirst,
          child: Text(labels.accountExportFirst),
        ),
      ],
    );
  }
}

class QaulDeleteAccountFinalDialog extends StatelessWidget {
  const QaulDeleteAccountFinalDialog({
    super.key,
    this.onCancel,
    this.onConfirm,
  });

  final VoidCallback? onCancel;
  final VoidCallback? onConfirm;

  @override
  Widget build(BuildContext context) {
    final labels = _l10n(context);
    return _QaulAccountDialog(
      title: labels.accountRemoveAccount,
      message: labels.accountDeleteFinalWarning,
      primaryLabel: labels.accountDeletePermanently,
      onPrimary: onConfirm,
      primaryIsDestructive: true,
      secondaryLabel: labels.accountCancel,
      onSecondary: onCancel,
    );
  }
}

class QaulPasswordDialog extends StatefulWidget {
  const QaulPasswordDialog({
    super.key,
    this.onCancel,
    this.onSetPassword,
    this.onRemovePassword,
  });

  final VoidCallback? onCancel;
  final ValueChanged<String>? onSetPassword;
  final VoidCallback? onRemovePassword;

  @override
  State<QaulPasswordDialog> createState() => _QaulPasswordDialogState();
}

class _QaulPasswordDialogState extends State<QaulPasswordDialog> {
  final _controller = TextEditingController();

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final labels = _l10n(context);
    return AlertDialog(
      title: Text(labels.accountChangeOrRemovePassword),
      content: TextField(
        controller: _controller,
        obscureText: true,
        decoration: InputDecoration(labelText: labels.accountNewPassword),
      ),
      actions: [
        TextButton(
          onPressed: widget.onCancel,
          child: Text(labels.accountCancel),
        ),
        TextButton(
          onPressed: widget.onRemovePassword,
          child: Text(labels.accountRemovePassword),
        ),
        TextButton(
          onPressed: () => widget.onSetPassword?.call(_controller.text),
          child: Text(labels.accountSetPassword),
        ),
      ],
    );
  }
}

class _QaulAccountDialog extends StatelessWidget {
  const _QaulAccountDialog({
    required this.title,
    required this.message,
    required this.primaryLabel,
    required this.onPrimary,
    required this.secondaryLabel,
    required this.onSecondary,
    this.primaryIsDestructive = false,
  });

  final String title;
  final String message;
  final String primaryLabel;
  final VoidCallback? onPrimary;
  final String secondaryLabel;
  final VoidCallback? onSecondary;
  final bool primaryIsDestructive;

  @override
  Widget build(BuildContext context) {
    final primaryColor = primaryIsDestructive
        ? Theme.of(context).colorScheme.error
        : null;
    return AlertDialog(
      title: Text(title),
      content: Text(message),
      actions: [
        TextButton(onPressed: onSecondary, child: Text(secondaryLabel)),
        TextButton(
          onPressed: onPrimary,
          child: Text(primaryLabel, style: TextStyle(color: primaryColor)),
        ),
      ],
    );
  }
}
