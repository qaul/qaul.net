import 'package:flutter/material.dart';
import 'package:font_awesome_flutter/font_awesome_flutter.dart';
import 'package:qaul_rpc/qaul_rpc.dart';

import '../l10n/app_localizations.dart';

import 'widgets.dart';

class UserDetailsHeading extends StatelessWidget {
  const UserDetailsHeading(this.user, {super.key});
  final User user;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context).textTheme;
    final l10n = AppLocalizations.of(context);

    return ListView(
      shrinkWrap: true,
      children: [
        Row(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            QaulAvatar.large(user: user),
            Expanded(
              child: Padding(
                padding: const EdgeInsets.symmetric(horizontal: 24.0),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      user.name,
                      style: theme.titleLarge,
                      maxLines: 1,
                      overflow: TextOverflow.ellipsis,
                    ),
                    const SizedBox(height: 8),
                    Text(
                      user.idBase58,
                      style: theme.titleSmall,
                      maxLines: 3,
                      overflow: TextOverflow.ellipsis,
                    ),
                    const SizedBox(height: 24),
                    Row(
                      children: [
                        const FaIcon(FontAwesomeIcons.key, size: 16),
                        const SizedBox(width: 8),
                        Expanded(
                          child: Text(
                            l10n!.publicKey,
                            style: theme.titleMedium,
                            maxLines: 1,
                            overflow: TextOverflow.ellipsis,
                          ),
                        ),
                      ],
                    ),
                    const SizedBox(height: 8),
                    Text(user.keyBase58 ?? 'Unknown'),
                  ],
                ),
              ),
            ),
          ],
        ),
        const SizedBox(height: 60),
      ],
    );
  }
}
