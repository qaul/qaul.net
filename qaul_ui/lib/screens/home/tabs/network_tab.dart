part of 'tab.dart';

class _Network extends BaseTab {
  const _Network({Key? key}) : super(key: key);

  @override
  _NetworkState createState() => _NetworkState();
}

class _NetworkState extends _BaseTabState<_Network> {
  Future<void> refreshNetwork(WidgetRef ref) async {
    if (loading.value) return;
    loading.value = true;
    final worker = ref.read(qaulWorkerProvider);
    await worker.getUsers();
    loading.value = false;
  }

  @override
  Widget build(BuildContext context) {
    super.build(context);

    return Scaffold(
      body: RefreshIndicator(
        onRefresh: () async => await refreshNetwork(ref),
        child: ListView(
          physics: const AlwaysScrollableScrollPhysics(),
          controller: ScrollController(),
          padding: MediaQuery.of(context).viewPadding.add(const EdgeInsets.symmetric(vertical: 12)),
          children: const [
            _AvailableConnectionsTable(type: ConnectionType.ble),
            SizedBox(height: 32.0),
            _AvailableConnectionsTable(type: ConnectionType.lan),
            SizedBox(height: 32.0),
            _AvailableConnectionsTable(type: ConnectionType.internet),
          ],
        ),
      ),
    );
  }
}

class _AvailableConnectionsTable extends ConsumerWidget {
  const _AvailableConnectionsTable({Key? key, required this.type}) : super(key: key);
  final ConnectionType type;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final icon = _mapIconFromType(type);

    final l18ns = AppLocalizations.of(context);
    final theme = Theme.of(context).textTheme;

    final defaultUser = ref.watch(defaultUserProvider);
    final users = ref
        .watch(usersProvider)
        .where((u) => !(u.isBlocked ?? false))
        .where((u) => u.idBase58 != (defaultUser?.idBase58 ?? ''))
        .where((u) => u.availableTypes?.keys.contains(type) ?? false)
        .toList();

    // TODO(brenodt): Should be solved programmatically, not hard-coded. Maybe make use of Intl.message (https://pub.dev/packages/intl#messages)
    final header = Text(
        (Intl.defaultLocale?.startsWith('pt') ?? false)
            ? '${l18ns!.connections} ${_buildCapitalizedEnumName()}'
            : '${_buildCapitalizedEnumName()} ${l18ns!.connections}',
        style: theme.headlineSmall);

    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 16.0),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [Icon(icon, size: 32), const SizedBox(width: 8), header],
          ),
          const SizedBox(height: 12),
          users.isEmpty
              ? Text(l18ns.noneAvailableMessage,
                  style: theme.titleMedium!.copyWith(fontStyle: FontStyle.italic))
              : Table(
                  defaultVerticalAlignment: TableCellVerticalAlignment.middle,
                  columnWidths: const {
                    0: FlexColumnWidth(.1),
                    1: FlexColumnWidth(.22),
                    2: FlexColumnWidth(.22),
                    3: FlexColumnWidth(.22),
                  },
                  children: [
                    TableRow(
                      children: [
                        const TableCell(child: SizedBox(height: 24)),
                        TableCell(child: Text(l18ns.ping)),
                        TableCell(child: Text(l18ns.hopCount)),
                        TableCell(child: Text(l18ns.via)),
                      ],
                    ),
                    ...List.generate(users.length, (index) {
                      final usr = users[index];
                      final data = usr.availableTypes![type];

                      return TableRow(children: [
                        TableCell(
                          child: Container(
                            alignment: AlignmentDirectional.centerStart,
                            padding: const EdgeInsets.symmetric(vertical: 4.0),
                            child: QaulAvatar.tiny(user: usr),
                          ),
                        ),
                        TableCell(
                            child: Text(data!.ping == null ? l18ns.unknown : '${data.ping} ms')),
                        TableCell(
                            child: Text(
                                data.hopCount == null ? l18ns.unknown : data.hopCount.toString())),
                        TableCell(
                            child: Text(
                                data.nodeIDBase58 == null ? l18ns.unknown : data.nodeIDBase58!,
                                style: theme.bodySmall)),
                      ]);
                    }),
                  ],
                ),
        ],
      ),
    );
  }

  String _buildCapitalizedEnumName() => type.name
      .splitMapJoin(RegExp(r'^.'), onMatch: (m) => m[0]!.toUpperCase(), onNonMatch: (n) => n);

  IconData _mapIconFromType(ConnectionType type) {
    switch (type) {
      case ConnectionType.ble:
        return Icons.bluetooth;
      case ConnectionType.lan:
        return Icons.wifi;
      case ConnectionType.internet:
        return CupertinoIcons.globe;
      default:
        throw ArgumentError.value(type, 'ConnectionType', 'value not mapped');
    }
  }
}
