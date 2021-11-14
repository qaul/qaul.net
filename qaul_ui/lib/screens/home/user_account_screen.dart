import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:qaul_ui/widgets/user_avatar.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';

class UserAccountScreen extends ConsumerWidget {
  const UserAccountScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final user = ref.watch(defaultUserProvider).state;

    final theme = Theme.of(context).textTheme;
    final l18ns = AppLocalizations.of(context);
    return Padding(
      padding: MediaQuery.of(context)
          .viewPadding
          .add(const EdgeInsets.fromLTRB(16, 8, 16, 8)),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              UserAvatar.large(),
              Expanded(
                child: Padding(
                  padding: const EdgeInsets.symmetric(horizontal: 24.0),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        user != null
                            ? user.name
                            : _notFound(l18ns!, l18ns.username),
                        style: theme.headline6,
                      ),
                      const SizedBox(height: 24),
                      Text(
                        user != null
                            ? user.idBase58
                            : _notFound(l18ns!, l18ns.userID),
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
          Text('Qaul ${l18ns!.publicKey}', style: theme.headline5),
          const SizedBox(height: 20),
          Text(
            user != null && user.keyBase58 != null
                ? user.keyBase58!
                : _notFound(l18ns, l18ns.publicKey),
          ),
        ],
      ),
    );
  }

  String _notFound(AppLocalizations localizations, String field) =>
      '$field ${localizations.notFoundErrorMessage}';
}
