part of 'tab.dart';

class _Network extends BaseTab {
  const _Network({Key? key}) : super(key: key);

  @override
  _NetworkState createState() => _NetworkState();
}

class _NetworkState extends _BaseTabState<_Network> {
  @override
  Widget build(BuildContext context) {
    super.build(context);
    
    return Scaffold(
      body: SingleChildScrollView(
        child: Padding(
          padding: MediaQuery.of(context).viewPadding,
          child: Column(
            children: const [
              _AvailableConnectionsTable(type: _ConnectionType.bluetooth),
              SizedBox(height: 32.0),
              _AvailableConnectionsTable(type: _ConnectionType.lan),
              SizedBox(height: 32.0),
              _AvailableConnectionsTable(type: _ConnectionType.internet),
            ],
          ),
        ),
      ),
    );
  }
}

enum _ConnectionType { bluetooth, lan, internet }

class _AvailableConnectionsTable extends StatelessWidget {
  const _AvailableConnectionsTable({
    Key? key,
    required this.type,
  }) : super(key: key);
  final _ConnectionType type;

  @override
  Widget build(BuildContext context) {
    final icon = _mapIconFromType(type);

    final l18ns = AppLocalizations.of(context);
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 16.0),
      child: Column(
        children: [
          Row(
            children: [
              Icon(icon, size: 32),
              const SizedBox(width: 8),
              Text('${_buildCapitalizedEnumName()} ${l18ns!.connections}',
                  style: Theme.of(context).textTheme.headline5),
            ],
          ),
          const SizedBox(height: 12),
          Table(
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
              TableRow(children: [
                TableCell(
                  child: Container(
                    alignment: AlignmentDirectional.centerStart,
                    padding: const EdgeInsets.symmetric(vertical: 4.0),
                    child: UserAvatar.tiny(),
                  ),
                ),
                const TableCell(child: Text('103 ms')),
                const TableCell(child: Text('3')),
                const TableCell(child: Text('Node ID')),
              ]),
              TableRow(children: [
                TableCell(
                  child: Container(
                    alignment: AlignmentDirectional.centerStart,
                    padding: const EdgeInsets.symmetric(vertical: 4.0),
                    child: UserAvatar.tiny(),
                  ),
                ),
                const TableCell(child: Text('512 ms')),
                const TableCell(child: Text('2')),
                const TableCell(child: Text('Node ID')),
              ]),
            ],
          ),
        ],
      ),
    );
  }

  String _buildCapitalizedEnumName() =>
      describeEnum(type).splitMapJoin(RegExp(r'^.{1}'),
          onMatch: (m) => m[0]!.toUpperCase(), onNonMatch: (n) => n);

  IconData _mapIconFromType(_ConnectionType type) {
    switch (type) {
      case _ConnectionType.bluetooth:
        return Icons.bluetooth;
      case _ConnectionType.lan:
        return Icons.wifi;
      case _ConnectionType.internet:
        return CupertinoIcons.globe;
      default:
        throw ArgumentError.value(type, 'ConnectionType', 'value not mapped');
    }
  }
}
