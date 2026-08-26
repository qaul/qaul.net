import 'package:flutter/material.dart';

import '../../l10n/qaul_components_localizations.dart';
import '../../l10n/qaul_components_localizations_en.dart';
import 'qaul_auth_tokens.dart';

class QaulAuthWelcomeSection extends StatelessWidget {
  const QaulAuthWelcomeSection({
    super.key,
    required this.onCreateAccount,
    this.createAccountIcon,
  });

  final VoidCallback onCreateAccount;
  final Widget? createAccountIcon;

  @override
  Widget build(BuildContext context) {
    final labels =
        QaulComponentsLocalizations.of(context) ?? QaulComponentsLocalizationsEn();
    final primaryColor = qaulAuthPrimaryTextColor(context);
    final secondaryColor = qaulAuthSecondaryTextColor(context);

    return Column(
      children: [
        Text(
          labels.authWelcome,
          style: TextStyle(
            color: primaryColor,
            fontWeight: FontWeight.w800,
            letterSpacing: 1.1,
          ),
        ),
        const SizedBox(height: 12),
        InkWell(
          onTap: onCreateAccount,
          borderRadius: BorderRadius.circular(48),
          hoverColor: Colors.transparent,
          highlightColor: Colors.transparent,
          splashColor: Colors.transparent,
          child: Column(
            children: [
              createAccountIcon ??
                  Container(
                    width: 64,
                    height: 64,
                    decoration: BoxDecoration(
                      shape: BoxShape.circle,
                      border: Border.all(
                        color: primaryColor,
                        width: 2,
                      ),
                    ),
                    child: Icon(
                      Icons.accessibility_new,
                      color: primaryColor,
                      size: 40,
                    ),
                  ),
              const SizedBox(height: 12),
              Text(
                labels.accountCreateUserProfile,
                style: TextStyle(
                  color: secondaryColor,
                  fontWeight: FontWeight.w800,
                  letterSpacing: 1.1,
                ),
              ),
            ],
          ),
        ),
      ],
    );
  }
}
