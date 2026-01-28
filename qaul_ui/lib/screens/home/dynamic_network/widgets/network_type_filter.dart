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

class _NetworkTypeFilterNotifier extends Notifier<NetworkTypeFilter> {
  @override
  NetworkTypeFilter build() => NetworkTypeFilter.all;

  void select(NetworkTypeFilter value) => state = value;
}

final _networkTypeFilter =
    NotifierProvider<_NetworkTypeFilterNotifier, NetworkTypeFilter>(
        _NetworkTypeFilterNotifier.new);

class _FilteredNodesNotifier extends Notifier<NetworkNode> {
  @override
  NetworkNode build() {
    final filter = ref.watch(_networkTypeFilter);
    final defaultUser = ref.watch(defaultUserProvider)!;
    final users = ref
        .watch(usersProvider)
        .where((u) => !(u.isBlocked ?? false))
        .where((u) => u.idBase58 != defaultUser.idBase58)
        .toList();
    return NetworkNode.fromUserData(defaultUser, users, filter);
  }
}

final _filteredNodes =
    NotifierProvider<_FilteredNodesNotifier, NetworkNode>(_FilteredNodesNotifier.new);

class _NetworkTypeFilterToolbar extends HookConsumerWidget {
  const _NetworkTypeFilterToolbar();

  static final List<NetworkTypeFilter> availableFilters = [
    if (Platform.isAndroid) NetworkTypeFilter.bluetooth,
    NetworkTypeFilter.lan,
    NetworkTypeFilter.internet,
    NetworkTypeFilter.all,
  ];

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final filter = ref.watch(_networkTypeFilter);

    Color bgColorFor(NetworkTypeFilter t) {
      return filter == t ? Colors.lightBlue : Colors.blueGrey.shade200;
    }

    final buttons = List.generate(availableFilters.length, (i) {
      final filter = availableFilters[i];
      return filterButton(
        context,
        filter: filter,
        backgroundColor: bgColorFor(filter),
        onTap: () => ref.read(_networkTypeFilter.notifier).select(filter),
      );
    }).intersperse(const SizedBox(width: 8)).toList();

    return Container(
      padding: const EdgeInsets.all(4.0),
      margin: const EdgeInsets.all(16.0),
      decoration: BoxDecoration(
        color: Colors.blueGrey.withValues(alpha: .8),
        borderRadius: BorderRadius.circular(200.0),
      ),
      child: Row(mainAxisSize: MainAxisSize.min, children: buttons),
    );
  }

  Widget filterButton(
    BuildContext context, {
    required NetworkTypeFilter filter,
    required Color backgroundColor,
    required VoidCallback? onTap,
  }) {
    return GestureDetector(
      onTap: onTap,
      child: Tooltip(
        message: labelFor(filter, context: context),
        child: CircleAvatar(
          foregroundColor: Colors.white,
          backgroundColor: backgroundColor,
          child: iconFor(filter, context: context),
        ),
      ),
    );
  }

  Widget iconFor(NetworkTypeFilter filter, {required BuildContext context}) {
    switch (filter) {
      case NetworkTypeFilter.bluetooth:
        return const Icon(Icons.bluetooth);
      case NetworkTypeFilter.lan:
        return const Icon(Icons.wifi);
      case NetworkTypeFilter.internet:
        return const Icon(CupertinoIcons.globe);
      case NetworkTypeFilter.all:
        return SvgPicture.asset(
          'assets/icons/network.svg',
          colorFilter: const ColorFilter.mode(Colors.white, BlendMode.srcATop),
        );
    }
  }

  String labelFor(NetworkTypeFilter filter, {required BuildContext context}) {
    switch (filter) {
      case NetworkTypeFilter.bluetooth:
        return 'Bluetooth';
      case NetworkTypeFilter.lan:
        return 'LAN';
      case NetworkTypeFilter.internet:
        return 'Internet';
      case NetworkTypeFilter.all:
        return AppLocalizations.of(context)!.allConnectionsFilterLabel;
    }
  }
}
