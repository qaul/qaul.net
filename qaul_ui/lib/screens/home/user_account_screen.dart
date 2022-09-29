import 'package:flutter/cupertino.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/qaul_rpc.dart';

import '../../decorators/cron_task_decorator.dart';
import '../../decorators/search_user_decorator.dart';
import '../../widgets/widgets.dart';

class UserAccountScreen extends HookConsumerWidget {
  const UserAccountScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final user = ref.watch(defaultUserProvider);
    final nodeInfo = ref.watch(nodeInfoProvider);

    final refreshConnectionData = useCallback(() {
      ref.read(qaulWorkerProvider).sendBleInfoRequest();
      ref.read(qaulWorkerProvider).getNodeInfo();
    }, []);

    final theme = Theme.of(context).textTheme;
    final l10n = AppLocalizations.of(context);
    return CronTaskDecorator(
      schedule: const Duration(milliseconds: 1500),
      callback: refreshConnectionData,
      child: ListView(
        padding: MediaQuery.of(context)
            .viewPadding
            .add(const EdgeInsets.fromLTRB(16, 8, 16, 8)),
        children: [
          Row(
            children: [
              QaulAvatar.large(),
              Expanded(
                child: Padding(
                  padding: const EdgeInsets.symmetric(horizontal: 24.0),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        user != null
                            ? user.name
                            : _notFound(l10n!, l10n.username),
                        style: theme.headline6,
                      ),
                      const SizedBox(height: 24),
                      Text(
                        user != null
                            ? user.idBase58
                            : _notFound(l10n!, l10n.userID),
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
          Text(
            user != null && user.keyBase58 != null
                ? user.keyBase58!
                : _notFound(l10n, l10n.publicKey),
          ),
          const SizedBox(height: 60),
          const _DTNNodesList(),
          const SizedBox(height: 60),
          Text('Node Info', style: theme.headline4),
          const SizedBox(height: 20),
          Text('Node ID', style: theme.headline6),
          const SizedBox(height: 8),
          Text(nodeInfo?.idBase58 ?? 'Unknown',
              style: theme.bodyText2!.copyWith(fontSize: 12)),
          const SizedBox(height: 20),
          Text(l10n.knownAddresses, style: theme.headline6),
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

  String _notFound(AppLocalizations localizations, String field) =>
      '$field ${localizations.notFoundErrorMessage}';
}

class _DTNNodesList extends HookConsumerWidget {
  const _DTNNodesList();

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
        title: l10n.dtnNodes,
        addRowLabel: l10n.addUserNode,
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
          );
        },
      ),
    );
  }
}

class _AddUserDialog extends HookConsumerWidget {
  const _AddUserDialog({Key? key}) : super(key: key);

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
