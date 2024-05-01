part of '../dynamic_network_screen.dart';

class _NetworkNodeInfoBottomSheet extends StatelessWidget {
  const _NetworkNodeInfoBottomSheet({
    Key? key,
    required this.node, this.onClosePressed,
  }) : super(key: key);
  final NetworkNode node;
  final VoidCallback? onClosePressed;

  List<ConnectionType> get _supportedTypes =>
      [ConnectionType.ble, ConnectionType.lan, ConnectionType.internet];

  IconData _toIconData(ConnectionType t) {
    switch (t) {
      case ConnectionType.lan:
        return Icons.wifi;
      case ConnectionType.internet:
        return CupertinoIcons.globe;
      case ConnectionType.ble:
        return Icons.bluetooth;
      case ConnectionType.local:
        return Icons.cable_outlined;
    }
  }

  @override
  Widget build(BuildContext context) {
    final l18ns = AppLocalizations.of(context)!;
    final theme = Theme.of(context).textTheme;

    const start = Alignment.centerLeft;
    const end = Alignment.centerRight;

    return GestureDetector(
      child: Container(
        margin: const EdgeInsets.symmetric(horizontal: 20.0),
        padding: const EdgeInsets.fromLTRB(20.0, 16.0, 20.0, 16.0),
        decoration: BoxDecoration(
          color: Theme.of(context).colorScheme.surface,
          borderRadius: const BorderRadius.vertical(
            top: Radius.circular(20.0),
          ),
        ),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.end,
          mainAxisAlignment: MainAxisAlignment.start,
          children: <Widget>[
            Align(
              alignment: Alignment.topLeft,
              child: GestureDetector(
                onTap: onClosePressed,
                child: const Icon(Icons.close_rounded),
              ),
            ),
            QaulListTile.user(
              node.user,
              content: Text(
                'ID: ${node.user.idBase58}',
                style: theme.bodySmall!.copyWith(fontSize: 10),
              ),
            ),
            const SizedBox(height: 12),
            Table(
              defaultVerticalAlignment: TableCellVerticalAlignment.middle,
              columnWidths: const {
                0: FlexColumnWidth(.05),
                1: FlexColumnWidth(.3),
                2: FlexColumnWidth(.3),
                3: FlexColumnWidth(.3),
              },
              children: [
                TableRow(
                  children: [
                    const TableCell(child: SizedBox.shrink()),
                    TableCell(child: Text(l18ns.ping)),
                    TableCell(child: Text(l18ns.hopCount)),
                    TableCell(child: Text(l18ns.via)),
                  ],
                ),
                ...List<TableRow>.generate(_supportedTypes.length, (index) {
                  var type = _supportedTypes[index];
                  var info = node.user.availableTypes?[type];
                  var ping = info?.ping == null ? '-' : '${info!.ping!} ms';
                  return TableRow(children: [
                    TableCell(
                      child: Container(
                        padding: const EdgeInsets.symmetric(vertical: 2),
                        alignment: start,
                        child: Icon(_toIconData(type)),
                      ),
                    ),
                    TableCell(
                      child: Container(
                        alignment: end,
                        padding: const EdgeInsets.only(right: 12),
                        child: Text(ping),
                      ),
                    ),
                    TableCell(child: Text(info?.hopCount?.toString() ?? '-')),
                    TableCell(
                      child: Text(
                        info?.nodeIDBase58 ?? '-',
                        style: theme.bodySmall!.copyWith(fontSize: 10),
                      ),
                    ),
                  ]);
                }),
              ],
            ),
          ],
        ),
      ),
    );
  }
}
