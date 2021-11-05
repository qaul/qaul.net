part of '../home_screen.dart';

enum _ConnectionType { bluetooth, lan, internet }

class _NetworkTab extends StatelessWidget {
  const _NetworkTab({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: SingleChildScrollView(
        child: Padding(
          padding: EdgeInsets.symmetric(
            horizontal: 16.0,
            vertical: MediaQuery.of(context).size.height * .12,
          ),
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

class _AvailableConnectionsTable extends StatelessWidget {
  const _AvailableConnectionsTable({
    Key? key,
    required this.type,
  }) : super(key: key);
  final _ConnectionType type;

  @override
  Widget build(BuildContext context) {
    final icon = _mapIconFromType(type);

    return Column(
      children: [
        Row(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Icon(icon, size: 32),
            const SizedBox(width: 8),
            Text('${_buildCapitalizedEnumName()} Connections',
                style: Theme.of(context).textTheme.headline4),
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
            const TableRow(
              children: [
                TableCell(child: SizedBox(height: 24)),
                TableCell(child: Text('Ping')),
                TableCell(child: Text('Hop Count')),
                TableCell(child: Text('Via')),
              ],
            ),
            TableRow(children: [
              TableCell(
                  child: Padding(
                padding: const EdgeInsets.symmetric(vertical: 4.0),
                child: UserAvatar.tiny(),
              )),
              const TableCell(child: Text('103 ms')),
              const TableCell(child: Text('3')),
              const TableCell(child: Text('Node ID')),
            ]),
            TableRow(children: [
              TableCell(
                  child: Padding(
                padding: const EdgeInsets.symmetric(vertical: 4.0),
                child: UserAvatar.tiny(),
              )),
              const TableCell(child: Text('512 ms')),
              const TableCell(child: Text('2')),
              const TableCell(child: Text('Node ID')),
            ]),
          ],
        ),
      ],
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
