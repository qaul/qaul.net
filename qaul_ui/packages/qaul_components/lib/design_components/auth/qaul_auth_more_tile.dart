import 'package:flutter/material.dart';

import '../../l10n/qaul_components_localizations.dart';
import '../../l10n/qaul_components_localizations_en.dart';
import 'qaul_auth_tokens.dart';

class QaulAuthMoreTile extends StatelessWidget {
  const QaulAuthMoreTile({super.key, required this.onTap});

  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    final labels =
        QaulComponentsLocalizations.of(context) ?? QaulComponentsLocalizationsEn();
    final secondaryColor = qaulAuthSecondaryTextColor(context);

    return InkWell(
      onTap: onTap,
      child: SizedBox(
        height: kQaulAuthMoreRowHeight,
        child: Padding(
          padding: const EdgeInsets.symmetric(horizontal: 12),
          child: Row(
            children: [
              Icon(
                Icons.add_circle_outline,
                color: secondaryColor,
                size: kQaulAuthIconSize,
              ),
              const SizedBox(width: kQaulAuthAvatarTextGap),
              Expanded(
                child: Text(
                  labels.authMoreAccounts,
                  style: qaulAuthAccountTextStyle(context),
                ),
              ),
              Icon(
                Icons.keyboard_arrow_down,
                color: secondaryColor,
              ),
            ],
          ),
        ),
      ),
    );
  }
}
