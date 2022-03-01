part of '../dynamic_network_screen.dart';

class _NetworkNodeInfoBottomSheet extends StatelessWidget {
  const _NetworkNodeInfoBottomSheet({
    Key? key,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final l18ns = AppLocalizations.of(context)!;
    final theme = Theme.of(context).textTheme;

    const start = Alignment.centerLeft;
    const end = Alignment.centerRight;

    return GestureDetector(
      child: Container(
        margin: const EdgeInsets.symmetric(horizontal: 20.0),
        padding: const EdgeInsets.fromLTRB(20.0, 16.0, 10.0, 16.0),
        decoration: BoxDecoration(
          color: Theme.of(context).appBarTheme.backgroundColor,
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
                onTap: () => Navigator.pop(context),
                child: const Icon(Icons.close_rounded),
              ),
            ),
            ListTile(
              leading: UserAvatar.small(),
              visualDensity: VisualDensity.adaptivePlatformDensity,
              contentPadding: EdgeInsets.zero,
              title: Padding(
                padding: const EdgeInsets.only(bottom: 4.0),
                child: Text('Name name', style: theme.headline6),
              ),
              subtitle: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    'ID: --------------------------------------',
                    style: theme.caption!.copyWith(fontSize: 10),
                  ),
                ],
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
                TableRow(children: [
                  const TableCell(
                    child: Align(
                      alignment: start,
                      child: Icon(Icons.bluetooth),
                    ),
                  ),
                  TableCell(
                    child: Container(
                      alignment: end,
                      padding: const EdgeInsets.only(right: 12),
                      child: const Text('200 ms'),
                    ),
                  ),
                  const TableCell(child: Text('HC')),
                  const TableCell(child: Text('Via')),
                ]),
                TableRow(children: [
                  const TableCell(
                    child: Align(
                      alignment: start,
                      child: Icon(Icons.wifi),
                    ),
                  ),
                  TableCell(
                    child: Container(
                      alignment: end,
                      padding: const EdgeInsets.only(right: 12),
                      child: const Text('200 ms'),
                    ),
                  ),
                  const TableCell(child: Text('HC')),
                  const TableCell(child: Text('Via')),
                ]),
                TableRow(children: [
                  const TableCell(
                    child: Align(
                      alignment: start,
                      child: Icon(CupertinoIcons.globe),
                    ),
                  ),
                  TableCell(
                    child: Container(
                      alignment: end,
                      padding: const EdgeInsets.only(right: 12),
                      child: const Text('200 ms'),
                    ),
                  ),
                  const TableCell(child: Text('HC')),
                  const TableCell(child: Text('Via')),
                ]),
              ],
            ),
          ],
        ),
      ),
    );
  }
}
