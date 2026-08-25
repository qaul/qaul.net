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
    return InkWell(
      onTap: onTap,
      child: SizedBox(
        height: kQaulAuthMoreRowHeight,
        child: Padding(
          padding: const EdgeInsets.symmetric(horizontal: 12),
          child: Row(
            children: [
              const Icon(
                Icons.add_circle_outline,
                color: kQaulAuthSecondaryTextColor,
                size: kQaulAuthIconSize,
              ),
              const SizedBox(width: kQaulAuthAvatarTextGap),
              Expanded(
                child: Text(
                  labels.authMoreAccounts,
                  style: kQaulAuthAccountTextStyle,
                ),
              ),
              const Icon(
                Icons.keyboard_arrow_down,
                color: kQaulAuthSecondaryTextColor,
              ),
            ],
          ),
        ),
      ),
    );
  }
}
