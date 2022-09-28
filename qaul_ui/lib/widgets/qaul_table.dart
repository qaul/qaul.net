part of 'widgets.dart';

class QaulTable extends StatelessWidget {
  const QaulTable({
    Key? key,
    required this.titleIcon,
    required this.title,
    required this.rowCount,
    required this.rowBuilder,
    required this.addRowLabel,
    required this.onAddRowPressed,
    this.emptyStateWidget,
  }) : super(key: key);
  final IconData titleIcon;
  final String title;
  final int rowCount;
  final Widget Function(BuildContext, int) rowBuilder;
  final String addRowLabel;
  final VoidCallback onAddRowPressed;
  final Widget? emptyStateWidget;

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    return Column(
      children: [
        Row(
          children: [
            Icon(titleIcon),
            const SizedBox(width: 8.0),
            Text(title),
          ],
        ),
        const SizedBox(height: 8.0),
        if (rowCount == 0)
          emptyStateWidget ?? Text(l10n.genericEmptyState)
        else
          Container(
            padding: const EdgeInsets.symmetric(vertical: 4),
            decoration: BoxDecoration(
              border: Border.symmetric(
                horizontal: BorderSide(color: Theme.of(context).dividerColor),
              ),
            ),
            child: ListView.separated(
              shrinkWrap: true,
              physics: const NeverScrollableScrollPhysics(),
              itemCount: rowCount,
              separatorBuilder: (_, __) => const Divider(height: 12.0),
              itemBuilder: rowBuilder,
            ),
          ),
        const SizedBox(height: 12.0),
        Row(
          children: [
            IconButton(
              icon: const Icon(Icons.add),
              splashRadius: 24,
              onPressed: onAddRowPressed,
            ),
            const SizedBox(width: 12.0),
            Text(addRowLabel),
          ],
        ),
      ],
    );
  }
}
