import 'package:flutter/cupertino.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/qaul_rpc.dart';

import '../../decorators/cron_task_decorator.dart';
import '../../decorators/search_user_decorator.dart';
import '../../l10n/app_localizations.dart';
import '../../widgets/user_details_banner.dart';
import '../../widgets/widgets.dart';

class UserAccountScreen extends HookConsumerWidget {
  const UserAccountScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final user = ref.watch(defaultUserProvider)!;
    final nodeInfo = ref.watch(nodeInfoProvider);

    final refreshConnectionData = useCallback(() {
      ref.read(qaulWorkerProvider).sendBleInfoRequest();
      ref.read(qaulWorkerProvider).getNodeInfo();
    }, []);

    final theme = Theme.of(context).textTheme;
    final l10n = AppLocalizations.of(context)!;
    return CronTaskDecorator(
      schedule: const Duration(milliseconds: 1500),
      callback: refreshConnectionData,
      child: ListView(
        padding: MediaQuery.of(context)
            .viewPadding
            .add(const EdgeInsets.fromLTRB(16, 8, 16, 8)),
        children: [
          UserDetailsHeading(user),
          const _StorageUsersList(),
          const SizedBox(height: 60),
          Text('Node Info', style: theme.headlineMedium),
          const SizedBox(height: 20),
          Text('Node ID', style: theme.titleLarge),
          const SizedBox(height: 8),
          Text(nodeInfo?.idBase58 ?? 'Unknown',
              style: theme.bodyMedium!.copyWith(fontSize: 12)),
          const SizedBox(height: 20),
          Text(l10n.knownAddresses, style: theme.titleLarge),
          const SizedBox(height: 8),
          Table(
            border: TableBorder.all(),
            defaultVerticalAlignment: TableCellVerticalAlignment.middle,
            children: List.generate(
              nodeInfo?.knownAddresses.length ?? 0,
              (index) => TableRow(
                children: [
                  TableCell(
                      child: Padding(
                    padding: const EdgeInsets.all(8.0),
                    child: Text(nodeInfo!.knownAddresses[index]),
                  )),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }
}

class _StorageUsersList extends HookConsumerWidget {
  const _StorageUsersList();

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final config = ref.watch(dtnConfigurationProvider);

    final refreshDTN = useCallback(() {
      final worker = ref.read(qaulWorkerProvider);
      worker.getDTNConfiguration();
    }, []);

    final removeUser = useCallback((Uint8List userId) async {
      final worker = ref.read(qaulWorkerProvider);
      worker.removeDTNUser(userId);
    }, []);

    final addUser = useCallback((Uint8List userId) async {
      final worker = ref.read(qaulWorkerProvider);
      worker.addDTNUser(userId);
    }, []);

    useMemoized(() async {
      final worker = ref.read(qaulWorkerProvider);
      worker.getDTNConfiguration();
    });

    final l10n = AppLocalizations.of(context)!;
    return CronTaskDecorator(
      callback: refreshDTN,
      schedule: const Duration(milliseconds: 200),
      child: QaulTable(
        titleIcon: Icons.storage,
        title: l10n.storageUsers,
        addRowLabel: l10n.addStorageUser,
        emptyStateWidget: Text(l10n.emptyUsersList),
        rowCount: config == null ? 0 : config.users.length,
        onAddRowPressed: () async {
          final res = await Navigator.push(context,
              MaterialPageRoute(builder: (_) => const _AddUserDialog()));
          if (res is! User) return;
          addUser(res.id);
        },
        rowBuilder: (context, i) {
          var user = config!.users[i];
          return QaulListTile.user(
            user,
            trailingIcon: IconButton(
              splashRadius: 20,
              icon: const Icon(CupertinoIcons.delete),
              onPressed: () => removeUser(user.id),
            ),
            nameTapRoutesToDetailsScreen: true,
          );
        },
      ),
    );
  }
}

class _AddUserDialog extends HookConsumerWidget {
  const _AddUserDialog();

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final config = ref.watch(dtnConfigurationProvider);

    return SearchUserDecorator(
        title: 'Select user to create DTN node',
        builder: (context, users) {
          final eligibleUsers = users
              .where((u) => !(config?.users.contains(u) ?? false))
              .toList();

          return ListView.builder(
            itemCount: eligibleUsers.length,
            itemBuilder: (context, i) => QaulListTile.user(
              eligibleUsers[i],
              onTap: () => Navigator.pop(context, eligibleUsers[i]),
            ),
          );
        });
  }
}
