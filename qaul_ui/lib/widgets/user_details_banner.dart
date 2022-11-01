import 'package:flutter/material.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:qaul_rpc/qaul_rpc.dart';

import 'widgets.dart';

class UserDetailsHeading extends StatelessWidget {
  const UserDetailsHeading(this.user, {Key? key}) : super(key: key);
  final User user;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context).textTheme;
    final l10n = AppLocalizations.of(context);

    return ListView(
      shrinkWrap: true,
      children: [
        Row(
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
                      style: theme.headline6,
                    ),
                    const SizedBox(height: 24),
                    Text(
                      user.idBase58,
                      style: theme.subtitle2,
                      maxLines: 3,
                      overflow: TextOverflow.ellipsis,
                    ),
                  ],
                ),
              ),
            ),
          ],
        ),
        const SizedBox(height: 60),
        Text('Qaul ${l10n!.publicKey}', style: theme.headline5),
        const SizedBox(height: 20),
        Text(user.keyBase58 ?? 'Unknown'),
        const SizedBox(height: 60),
      ],
    );
  }
}
