import 'dart:async';

import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:qaul_rpc/qaul_rpc.dart';

import '../helpers/navigation_helper.dart';
import '../l10n/app_localizations.dart';
import '../providers/account_session_provider.dart';

class AccountManagementCoordinator {
  const AccountManagementCoordinator._();

  static Future<void> showRestoreFlow(
    BuildContext context,
    WidgetRef ref,
  ) async {
    final confirmed = await showDialog<bool>(
      context: context,
      builder: (dialogContext) => QaulRestoreAccountDialog(
        onCancel: () => Navigator.pop(dialogContext, false),
        onConfirm: () => Navigator.pop(dialogContext, true),
      ),
    );
    if (confirmed != true || !context.mounted) return;

    final chooseFile = await showDialog<bool>(
      context: context,
      builder: (dialogContext) => QaulRestoreFilePickerDialog(
        onCancel: () => Navigator.pop(dialogContext, false),
        onConfirm: () => Navigator.pop(dialogContext, true),
      ),
    );
    if (chooseFile != true || !context.mounted) return;

    final picked = await FilePicker.platform.pickFiles(
      type: FileType.custom,
      allowedExtensions: const ['qaul_export'],
    );
    final path = picked?.files.single.path;
    if (path == null || !context.mounted) return;

    await _runWithProgress(context, () async {
      final worker = ref.read(qaulWorkerProvider);
      final restored = await ref.read(qaulWorkerProvider).restoreAccount(path);
      if (restored == null) throw const RpcRequestException('Restore failed');

      final accounts = await worker.getLocalAccounts();
      final restoredAccount = _findAccount(accounts, restored.userId);
      if (restoredAccount == null) {
        throw const RpcRequestException('Restored account was not found');
      }

      if (restoredAccount.hasPassword) {
        ref.read(defaultUserProvider.notifier).state = User(
          name: restoredAccount.username,
          id: restoredAccount.userId,
          hasPassword: restoredAccount.hasPassword,
        );
        throw const RpcRequestException(
          'Account restored. Please log in with its password.',
        );
      }

      final loggedIn = await worker.loginLocalAccount(restoredAccount);
      if (!loggedIn) throw const RpcRequestException('Restore login failed');
    });
    if (!context.mounted) return;
    Navigator.pushReplacementNamed(context, NavigationHelper.home);
  }

  static Future<void> showLoginFlow(BuildContext context, WidgetRef ref) async {
    final accounts = await _runWithProgress<List<LocalAccount>>(
      context,
      () => ref.read(qaulWorkerProvider).getLocalAccounts(),
    );
    if (!context.mounted || accounts == null || accounts.isEmpty) return;

    final account = await showDialog<LocalAccount>(
      context: context,
      builder: (dialogContext) => SimpleDialog(
        title: const Text('Select account'),
        children: [
          for (final account in accounts)
            SimpleDialogOption(
              onPressed: () => Navigator.pop(dialogContext, account),
              child: Text(account.username),
            ),
        ],
      ),
    );
    if (!context.mounted || account == null) return;

    String? password;
    if (account.hasPassword) {
      password = await _askPassword(context);
      if (!context.mounted || password == null) return;
    }

    final loggedIn = await _runWithProgress<bool>(
      context,
      () => ref
          .read(qaulWorkerProvider)
          .loginLocalAccount(account, password: password),
    );
    if (!context.mounted || loggedIn != true) return;

    Navigator.pushReplacementNamed(context, NavigationHelper.home);
  }

  static Future<void> showExportFlow(
    BuildContext context,
    WidgetRef ref, {
    VoidCallback? onComplete,
  }) async {
    final confirmed = await showDialog<bool>(
      context: context,
      builder: (dialogContext) => QaulExportAccountDialog(
        onCancel: () => Navigator.pop(dialogContext, false),
        onConfirm: () => Navigator.pop(dialogContext, true),
      ),
    );
    if (!context.mounted || confirmed != true) return;

    final outputPath = await FilePicker.platform.getDirectoryPath();
    if (!context.mounted || outputPath == null) return;

    final path = await _runWithProgress<String?>(
      context,
      () => ref.read(qaulWorkerProvider).exportAccount(outputPath: outputPath),
    );
    if (!context.mounted || path == null) return;

    _showMessage(context, 'Account exported to $path');
    onComplete?.call();
  }

  static Future<void> showPasswordFlow(
    BuildContext context,
    WidgetRef ref,
  ) async {
    await showDialog<void>(
      context: context,
      builder: (dialogContext) => QaulPasswordDialog(
        onCancel: () => Navigator.pop(dialogContext),
        onRemovePassword: () async {
          Navigator.pop(dialogContext);
          await _setPassword(context, ref, null);
        },
        onSetPassword: (password) async {
          Navigator.pop(dialogContext);
          await _setPassword(context, ref, password);
        },
      ),
    );
  }

  static Future<void> logout(BuildContext context, WidgetRef ref) async {
    final user = ref.read(defaultUserProvider);
    if (user == null) return;

    final navigator = Navigator.of(context, rootNavigator: true);
    ref.read(forceSignedOutProvider.notifier).state = true;
    if (!navigator.mounted) return;
    _returnToInitial(navigator, ref);

    unawaited(
      ref.read(qaulWorkerProvider).logout(userId: user.id).catchError((_) {
        return false;
      }),
    );
  }

  static Future<void> showDeleteFlow(
    BuildContext context,
    WidgetRef ref,
  ) async {
    final exportFirst = await showDialog<bool>(
      context: context,
      builder: (dialogContext) => QaulDeleteAccountExportPromptDialog(
        onCancel: () => Navigator.pop(dialogContext),
        onExportFirst: () => Navigator.pop(dialogContext, true),
        onDeleteWithoutExport: () => Navigator.pop(dialogContext, false),
      ),
    );
    if (!context.mounted || exportFirst == null) return;

    if (exportFirst) {
      await showExportFlow(context, ref);
      if (!context.mounted) return;
    }

    final confirmed = await showDialog<bool>(
      context: context,
      builder: (dialogContext) => QaulDeleteAccountFinalDialog(
        onCancel: () => Navigator.pop(dialogContext, false),
        onConfirm: () => Navigator.pop(dialogContext, true),
      ),
    );
    if (!context.mounted || confirmed != true) return;

    final user = ref.read(defaultUserProvider);
    if (user == null) return;
    final navigator = Navigator.of(context, rootNavigator: true);
    final deleted = await _runWithProgress<bool>(
      context,
      () => ref.read(qaulWorkerProvider).deleteAccount(userId: user.id),
    );
    if (!context.mounted || deleted != true) return;

    ref.read(forceSignedOutProvider.notifier).state = true;
    if (!navigator.mounted) return;
    _returnToInitial(navigator, ref);
  }

  static Future<void> _setPassword(
    BuildContext context,
    WidgetRef ref,
    String? password,
  ) async {
    final changed = await _runWithProgress<bool>(
      context,
      () => ref.read(qaulWorkerProvider).setAccountPassword(password),
    );
    if (!context.mounted || changed != true) return;
    _showMessage(context, 'Password updated');
  }

  static Future<String?> _askPassword(BuildContext context) {
    final controller = TextEditingController();
    return showDialog<String>(
      context: context,
      builder: (dialogContext) => AlertDialog(
        title: const Text('Password'),
        content: TextField(
          controller: controller,
          obscureText: true,
          autofocus: true,
          onSubmitted: (_) => Navigator.pop(dialogContext, controller.text),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(dialogContext),
            child: Text(AppLocalizations.of(context)!.cancelDialogButton),
          ),
          TextButton(
            onPressed: () => Navigator.pop(dialogContext, controller.text),
            child: Text(AppLocalizations.of(context)!.okDialogButton),
          ),
        ],
      ),
    ).whenComplete(controller.dispose);
  }

  static Future<T?> _runWithProgress<T>(
    BuildContext context,
    Future<T> Function() operation,
  ) async {
    final navigator = Navigator.of(context, rootNavigator: true);
    var dialogShown = false;

    showDialog<void>(
      context: context,
      useRootNavigator: true,
      barrierDismissible: false,
      builder: (_) {
        dialogShown = true;
        return const PopScope(
          canPop: false,
          child: Center(child: CircularProgressIndicator()),
        );
      },
    );

    await WidgetsBinding.instance.endOfFrame;

    try {
      return await operation();
    } catch (error) {
      if (context.mounted) _showMessage(context, error.toString());
      return null;
    } finally {
      if (dialogShown && navigator.mounted && navigator.canPop()) {
        navigator.pop();
      }
    }
  }

  static void _showMessage(BuildContext context, String message) {
    ScaffoldMessenger.of(
      context,
    ).showSnackBar(SnackBar(content: Text(message)));
  }

  static void _returnToInitial(NavigatorState navigator, WidgetRef ref) {
    navigator.pushNamedAndRemoveUntil(NavigationHelper.initial, (_) => false);
    WidgetsBinding.instance.addPostFrameCallback((_) {
      ref.read(defaultUserProvider.notifier).state = null;
      ref.invalidate(accountSessionProvider);
    });
  }

  static LocalAccount? _findAccount(
    List<LocalAccount> accounts,
    List<int> userId,
  ) {
    for (final account in accounts) {
      if (_sameBytes(account.userId, userId)) return account;
    }
    return null;
  }

  static bool _sameBytes(List<int> a, List<int> b) {
    if (a.length != b.length) return false;
    for (var i = 0; i < a.length; i++) {
      if (a[i] != b[i]) return false;
    }
    return true;
  }
}
