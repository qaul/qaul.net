import 'package:flutter/material.dart';

import '../l10n/app_localizations.dart';

class AndroidBackgroundExecutionDialog extends StatelessWidget {
  const AndroidBackgroundExecutionDialog({super.key});

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final theme = Theme.of(context).textTheme;
    return Dialog(
      child: Padding(
        padding: const EdgeInsets.fromLTRB(16, 16, 16, 0),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Expanded(
                  child: Text(
                    l10n.aboutBackgroundExecution,
                    style: theme.titleMedium,
                  ),
                ),
                InkWell(
                  borderRadius: BorderRadius.circular(20),
                  onTap: () => Navigator.pop(context),
                  child: const Icon(Icons.close),
                ),
              ],
            ),
            const SizedBox(height: 20),
            Text(l10n.backgroundExecutionDialog1),
            const SizedBox(height: 8),
            Text(l10n.backgroundExecutionDialog2),
            const SizedBox(height: 8),
            Text(l10n.backgroundExecutionDialog3),
            Row(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                Expanded(
                  child: TextButton(
                    onPressed: () => Navigator.pop(context),
                    child: Text(l10n.backgroundExecutionDialogConfirmButton),
                  ),
                ),
              ],
            )
          ],
        ),
      ),
    );
  }
}
