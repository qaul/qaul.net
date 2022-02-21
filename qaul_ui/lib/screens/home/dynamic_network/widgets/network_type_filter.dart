part of '../dynamic_network_screen.dart';

enum _NetworkType {
  bluetooth,
  lan,
  internet,
}

/// The currently active filter.
final _networkTypeFilter = StateProvider((_) => _NetworkType.bluetooth);

/// Nodes that fit the current filter criteria
final _filteredNodes = Provider<NetworkNode>((ref) {
  final filter = ref.watch(_networkTypeFilter);

  switch (filter) {
    case _NetworkType.bluetooth:
      return root;
    case _NetworkType.lan:
      return root1;
    case _NetworkType.internet:
      return root2;
    default:
      throw FlutterError('Invalid Filter provided; Value not mapped: $filter');
  }
});

class _NetworkTypeFilterToolbar extends HookConsumerWidget {
  const _NetworkTypeFilterToolbar({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final filter = ref.watch(_networkTypeFilter);

    Color? bgColorFor(_NetworkType t) {
      return filter == t ? Colors.lightBlue : Colors.blueGrey.shade200;
    }

    return Container(
      padding: const EdgeInsets.all(4.0),
      margin: const EdgeInsets.all(16.0),
      decoration: BoxDecoration(
        color: Colors.blueGrey.withOpacity(.8),
        borderRadius: BorderRadius.circular(200.0),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          GestureDetector(
            onTap: () => ref.read(_networkTypeFilter.notifier).state = _NetworkType.bluetooth,
            child: CircleAvatar(
              backgroundColor: bgColorFor(_NetworkType.bluetooth),
              foregroundColor: Colors.white,
              child: const Icon(Icons.bluetooth),
            ),
          ),
          const SizedBox(width: 8),
          GestureDetector(
            onTap: () => ref.read(_networkTypeFilter.notifier).state = _NetworkType.lan,
            child: CircleAvatar(
              backgroundColor: bgColorFor(_NetworkType.lan),
              foregroundColor: Colors.white,
              child: const Icon(Icons.wifi),
            ),
          ),
          const SizedBox(width: 8),
          GestureDetector(
            onTap: () => ref.read(_networkTypeFilter.notifier).state = _NetworkType.internet,
            child: CircleAvatar(
              backgroundColor: bgColorFor(_NetworkType.internet),
              foregroundColor: Colors.white,
              child: const Icon(CupertinoIcons.globe),
            ),
          ),
        ],
      ),
    );
  }
}
