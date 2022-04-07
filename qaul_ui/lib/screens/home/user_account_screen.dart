import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/qaul_rpc.dart';

import '../../decorators/cron_task_decorator.dart';
import '../../widgets/widgets.dart';

class UserAccountScreen extends HookConsumerWidget {
  const UserAccountScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final user = ref.watch(defaultUserProvider);
    final bleStatus = ref.watch(bleStatusProvider);

    final bleData = useMemoized<List<String>>(() {
      return [
        'BLE ID: ${bleStatus?.idBase58 ?? 'Unknown'}',
        'BLE Status: ${bleStatus?.status ?? 'Unknown'}',
        'Node Info ID: ${bleStatus?.deviceInfoBase58 ?? 'Unknown'}',
        'Discovered Nodes: ${bleStatus?.discoveredNodes ?? '0'}',
        'Nodes Pending Confirmation: ${bleStatus?.nodesPendingConfirmation ?? '0'}',
      ];
    }, [bleStatus]);

    final refreshBleStatus = useCallback(
        () => ref.read(qaulWorkerProvider).sendBleInfoRequest(), []);

    final theme = Theme.of(context).textTheme;
    final l18ns = AppLocalizations.of(context);
    return CronTaskDecorator(
      schedule: const Duration(milliseconds: 1500),
      callback: refreshBleStatus,
      child: Padding(
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
            const SizedBox(height: 60),
            Text('Bluetooth Connection Status', style: theme.headline5),
            const SizedBox(height: 20),
            if (Platform.isAndroid)
              ListView.separated(
                shrinkWrap: true,
                padding: EdgeInsets.zero,
                itemCount: bleData.length,
                itemBuilder: (_, i) => Text(bleData[i]),
                separatorBuilder: (_, __) => const Padding(
                  padding: EdgeInsets.symmetric(horizontal: 12),
                  child: Divider(),
                ),
              )
            else
              const Text('Currently not supported'),
          ],
        ),
      ),
    );
  }

  String _notFound(AppLocalizations localizations, String field) =>
      '$field ${localizations.notFoundErrorMessage}';
}
