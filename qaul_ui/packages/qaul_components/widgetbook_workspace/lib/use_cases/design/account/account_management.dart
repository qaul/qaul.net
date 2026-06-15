import 'package:flutter/material.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:widgetbook_annotation/widgetbook_annotation.dart' as widgetbook;

@widgetbook.UseCase(
  name: 'Account management flow',
  type: QaulAccountLanding,
  path: '[design]/account',
)
Widget buildInteractiveAccountFlowUseCase(BuildContext context) {
  return const _InteractiveAccountFlow();
}

class _InteractiveAccountFlow extends StatefulWidget {
  const _InteractiveAccountFlow();

  @override
  State<_InteractiveAccountFlow> createState() =>
      _InteractiveAccountFlowState();
}

class _InteractiveAccountFlowState extends State<_InteractiveAccountFlow> {
  QaulAccountSessionState _state = QaulAccountSessionState.signedOut;
  final List<String> _events = <String>['Signed out'];

  void _record(String event) {
    setState(() => _events.insert(0, event));
  }

  void _signIn(String event) {
    setState(() {
      _state = QaulAccountSessionState.signedIn;
      _events.insert(0, event);
    });
  }

  void _signOut() {
    setState(() {
      _state = QaulAccountSessionState.signedOut;
      _events.insert(0, 'Logout callback fired');
    });
  }

  void _showRestoreDialog() {
    showDialog<void>(
      context: context,
      builder: (dialogContext) => QaulRestoreAccountDialog(
        onCancel: () {
          Navigator.pop(dialogContext);
          _record('Restore dismissed');
        },
        onConfirm: () {
          Navigator.pop(dialogContext);
          _showRestoreFilePicker();
        },
      ),
    );
  }

  void _showRestoreFilePicker() {
    showDialog<void>(
      context: context,
      builder: (dialogContext) => QaulRestoreFilePickerDialog(
        onCancel: () {
          Navigator.pop(dialogContext);
          _record('Restore file picker dismissed');
        },
        onConfirm: () {
          Navigator.pop(dialogContext);
          _signIn('Restore file callback fired');
        },
      ),
    );
  }

  void _showExportDialog({
    String event = 'Export callback fired',
    VoidCallback? onComplete,
  }) {
    showDialog<void>(
      context: context,
      builder: (dialogContext) => QaulExportAccountDialog(
        onCancel: () {
          Navigator.pop(dialogContext);
          _record('Export dismissed');
        },
        onConfirm: () {
          Navigator.pop(dialogContext);
          _record(event);
          onComplete?.call();
        },
      ),
    );
  }

  void _showPasswordDialog() {
    showDialog<void>(
      context: context,
      builder: (dialogContext) => QaulPasswordDialog(
        onCancel: () {
          Navigator.pop(dialogContext);
          _record('Password dialog dismissed');
        },
        onRemovePassword: () {
          Navigator.pop(dialogContext);
          _record('Remove password callback fired');
        },
        onSetPassword: (password) {
          Navigator.pop(dialogContext);
          _record('Set password callback fired (${password.length} chars)');
        },
      ),
    );
  }

  void _showDeleteExportPrompt() {
    showDialog<void>(
      context: context,
      builder: (dialogContext) => QaulDeleteAccountExportPromptDialog(
        onCancel: () {
          Navigator.pop(dialogContext);
          _record('Delete dismissed');
        },
        onExportFirst: () {
          Navigator.pop(dialogContext);
          _showExportDialog(
            event: 'Export before delete callback fired',
            onComplete: _showDeleteFinalDialog,
          );
        },
        onDeleteWithoutExport: () {
          Navigator.pop(dialogContext);
          _showDeleteFinalDialog();
        },
      ),
    );
  }

  void _showDeleteFinalDialog() {
    showDialog<void>(
      context: context,
      builder: (dialogContext) => QaulDeleteAccountFinalDialog(
        onCancel: () {
          Navigator.pop(dialogContext);
          _record('Final delete dismissed');
        },
        onConfirm: () {
          Navigator.pop(dialogContext);
          setState(() {
            _state = QaulAccountSessionState.noLocalAccount;
            _events.insert(
              0,
              'Delete account callback fired → no local account',
            );
          });
        },
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    if (_state == QaulAccountSessionState.signedIn) {
      return _InteractiveShell(
        events: _events,
        child: _AccountScreenFrame(
          child: QaulAccountSettingsSection(
            account: const QaulAccountSummary(
              id: 'QmQaulAccountIdBase58Preview',
              name: 'Alice Qaul',
              publicKey: 'QmQaulPublicKeyBase58Preview',
            ),
            onLogout: _signOut,
            onExportAccount: _showExportDialog,
            onChangePassword: _showPasswordDialog,
            onDeleteAccount: _showDeleteExportPrompt,
          ),
        ),
      );
    }

    return _InteractiveShell(
      events: _events,
      child: QaulAccountLanding(
        state: _state,
        logo: const QaulAccountLogo(),
        languageSelector: const _LanguageSelectorPlaceholder(),
        onLogin: () => _signIn('Passwordless login callback fired'),
        onCreateAccount: () => _signIn('Create account callback fired'),
        onRestoreAccount: _showRestoreDialog,
        onLearnMore: () => _record('Learn more callback fired'),
      ),
    );
  }
}

class _InteractiveShell extends StatelessWidget {
  const _InteractiveShell({required this.events, required this.child});

  final List<String> events;
  final Widget child;

  @override
  Widget build(BuildContext context) {
    return Material(
      child: Row(
        children: [
          Expanded(child: child),
          SizedBox(
            width: 280,
            child: ColoredBox(
              color: Theme.of(context).colorScheme.surfaceContainerHighest,
              child: Padding(
                padding: const EdgeInsets.all(16),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      'Callback log',
                      style: Theme.of(context).textTheme.titleMedium,
                    ),
                    const SizedBox(height: 12),
                    for (final event in events.take(12))
                      Padding(
                        padding: const EdgeInsets.only(bottom: 8),
                        child: Text(event),
                      ),
                  ],
                ),
              ),
            ),
          ),
        ],
      ),
    );
  }
}

class _AccountScreenFrame extends StatelessWidget {
  const _AccountScreenFrame({required this.child});

  final Widget child;

  @override
  Widget build(BuildContext context) {
    return Material(
      child: ListView(
        padding: MediaQuery.of(
          context,
        ).viewPadding.add(const EdgeInsets.fromLTRB(16, 8, 16, 8)),
        children: [
          ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 720),
            child: child,
          ),
        ],
      ),
    );
  }
}

class _LanguageSelectorPlaceholder extends StatelessWidget {
  const _LanguageSelectorPlaceholder();

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        const Row(
          children: [
            Icon(Icons.translate),
            SizedBox(width: 8),
            Text('Language'),
            Expanded(child: Divider(indent: 8)),
          ],
        ),
        const SizedBox(height: 12),
        DropdownButtonFormField<String>(
          initialValue: 'system',
          decoration: const InputDecoration(border: OutlineInputBorder()),
          items: const [
            DropdownMenuItem(
              value: 'system',
              child: Text("Use system's default"),
            ),
          ],
          onChanged: (_) {},
        ),
      ],
    );
  }
}
