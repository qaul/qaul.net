import 'package:hooks_riverpod/hooks_riverpod.dart';

final nodeInfoProvider =
    NotifierProvider<NodeInfoNotifier, NodeInfo?>(NodeInfoNotifier.new);

class NodeInfoNotifier extends Notifier<NodeInfo?> {
  @override
  NodeInfo? build() => null;
}

class NodeInfo {
  const NodeInfo(this.idBase58, this.knownAddresses);

  final String idBase58;
  final List<String> knownAddresses;
}
