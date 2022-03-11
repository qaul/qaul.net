part of '../dynamic_network_screen.dart';

enum NetworkTypeFilter {
  bluetooth,
  lan,
  internet,
  all,
}

ConnectionType _mapFilter(NetworkTypeFilter t) {
  switch (t) {
    case NetworkTypeFilter.bluetooth:
      return ConnectionType.ble;
    case NetworkTypeFilter.lan:
      return ConnectionType.lan;
    case NetworkTypeFilter.internet:
      return ConnectionType.internet;
    default:
      throw ArgumentError.value(t, '$t has no ConnectionType counterpart');
  }
}

/// The currently active filter.
final _networkTypeFilter = StateProvider((_) => NetworkTypeFilter.internet);

/// Nodes that fit the current filter criteria
final _filteredNodes = Provider<NetworkNode>((ref) {
  final filter = ref.watch(_networkTypeFilter);
  final defaultUser = ref.watch(defaultUserProvider)!;
  final users = ref
      .watch(usersProvider)
      .where((u) => !(u.isBlocked ?? false))
      .where((u) => u.idBase58 != defaultUser.idBase58)
      .toList();

  return NetworkNode.fromUserData(defaultUser, users, filter);
});

class _NetworkTypeFilterToolbar extends HookConsumerWidget {
  const _NetworkTypeFilterToolbar({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final filter = ref.watch(_networkTypeFilter);

    Color? bgColorFor(NetworkTypeFilter t) {
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
            onTap: () => ref.read(_networkTypeFilter.notifier).state = NetworkTypeFilter.bluetooth,
            child: CircleAvatar(
              backgroundColor: bgColorFor(NetworkTypeFilter.bluetooth),
              foregroundColor: Colors.white,
              child: const Icon(Icons.bluetooth),
            ),
          ),
          const SizedBox(width: 8),
          GestureDetector(
            onTap: () => ref.read(_networkTypeFilter.notifier).state = NetworkTypeFilter.lan,
            child: CircleAvatar(
              backgroundColor: bgColorFor(NetworkTypeFilter.lan),
              foregroundColor: Colors.white,
              child: const Icon(Icons.wifi),
            ),
          ),
          const SizedBox(width: 8),
          GestureDetector(
            onTap: () => ref.read(_networkTypeFilter.notifier).state = NetworkTypeFilter.internet,
            child: CircleAvatar(
              backgroundColor: bgColorFor(NetworkTypeFilter.internet),
              foregroundColor: Colors.white,
              child: const Icon(CupertinoIcons.globe),
            ),
          ),
          const SizedBox(width: 8),
          GestureDetector(
            onTap: () => ref.read(_networkTypeFilter.notifier).state = NetworkTypeFilter.all,
            child: CircleAvatar(
              backgroundColor: bgColorFor(NetworkTypeFilter.all),
              foregroundColor: Colors.white,
              child: const Icon(Icons.mediation),
            ),
          ),
        ],
      ),
    );
  }
}
