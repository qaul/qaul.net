import 'package:hooks_riverpod/hooks_riverpod.dart';

final nodeInfoProvider = StateProvider<NodeInfo?>((_) => null);

class NodeInfo {
  const NodeInfo(this.idBase58, this.knownAddresses);

  final String idBase58;
  final List<String> knownAddresses;
}
