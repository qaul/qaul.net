import 'package:flutter/material.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';

class DefaultBackButton extends StatelessWidget {
  const DefaultBackButton({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final l18ns = AppLocalizations.of(context)!;
    return IconButton(
      splashRadius: 24,
      tooltip: l18ns.backButtonTooltip,
      icon: const Icon(Icons.arrow_back_ios_rounded),
      onPressed: () => Navigator.maybePop(context),
    );
  }
}
