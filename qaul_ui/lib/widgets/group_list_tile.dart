part of 'widgets.dart';

class GroupListTile extends StatelessWidget {
  const GroupListTile(
    this.room, {
    Key? key,
    this.content,
    this.trailingIcon,
    this.trailingMetadata,
    this.onTap,
    this.isThreeLine = false,
  })  : assert(trailingIcon == null || trailingMetadata == null),
        super(key: key);
  final ChatRoom room;

  final bool isThreeLine;

  /// Content displayed under the username
  final Widget? content;

  /// Center-aligned widget, displayed at the end of tile.
  final Widget? trailingIcon;

  /// Right, Baseline-aligned with username (usually a Text).
  final Widget? trailingMetadata;

  final VoidCallback? onTap;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context).textTheme;

    final username = Text(
      room.name!,
      maxLines: 1,
      overflow: TextOverflow.ellipsis,
      style: theme.bodyText1!.copyWith(fontWeight: FontWeight.bold),
    );

    Widget title = trailingIcon != null
        ? username
        : Row(
            textBaseline: TextBaseline.alphabetic,
            crossAxisAlignment: CrossAxisAlignment.baseline,
            children: [
              Expanded(child: username),
              if (trailingMetadata != null) trailingMetadata!,
            ],
          );

    return ListTile(
      onTap: onTap,
      title: title,
      subtitle: content,
      trailing: trailingIcon,
      isThreeLine: isThreeLine,
      leading: CircleAvatar(
        radius: 20.0,
        child: Text(
          initials(room.name!),
          style: const TextStyle(
            fontSize: 16,
            color: Colors.white,
            fontWeight: FontWeight.w700,
          ),
        ),
        backgroundColor: Colors.red.shade700,
      ),
      visualDensity: VisualDensity.adaptivePlatformDensity,
    );
  }
}
