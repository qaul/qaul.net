import 'package:flutter/material.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';

class AndroidBackgroundExecutionDialog extends StatelessWidget {
  const AndroidBackgroundExecutionDialog({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final theme = Theme.of(context).textTheme;
    return Dialog(
      child: Padding(
        padding: const EdgeInsets.all(8.0),
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
                IconButton(
                  onPressed: () => Navigator.pop(context),
                  icon: const Icon(Icons.close),
                  splashRadius: 12,
                ),
              ],
            ),
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
