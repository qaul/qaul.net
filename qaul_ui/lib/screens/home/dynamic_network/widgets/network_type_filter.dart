part of '../dynamic_network_screen.dart';

enum _NetworkType {
  bluetooth,
  lan,
  internet,
}

ConnectionType _mapFilter(_NetworkType t) {
  switch (t) {
    case _NetworkType.bluetooth:
      return ConnectionType.ble;
    case _NetworkType.lan:
      return ConnectionType.lan;
    case _NetworkType.internet:
      return ConnectionType.internet;
  }
}

/// The currently active filter.
final _networkTypeFilter = StateProvider((_) => _NetworkType.bluetooth);

/// Nodes that fit the current filter criteria
final _filteredNodes = Provider<NetworkNode>((ref) {
  final filter = ref.watch(_networkTypeFilter);
  final defaultUser = ref.watch(defaultUserProvider)!;
  final users = ref
      .watch(usersProvider)
      .where((u) => !(u.isBlocked ?? false))
      .where((u) => u.idBase58 != defaultUser.idBase58)
      .where((u) => u.availableTypes?.keys.contains(_mapFilter(filter)) ?? false)
      .toList();

  return NetworkNode.fromUserData(defaultUser, users, _mapFilter(filter));
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
