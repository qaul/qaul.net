import 'package:equatable/equatable.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

import '../generated/connections/connections.pb.dart';

final connectedNodesProvider = StateProvider<List<InternetNode>>((ref) => []);

class InternetNode extends Equatable {
  const InternetNode(
    this.address, {
    required this.isActive,
    required this.name,
  });

  final String address;
  final bool isActive;
  final String name;

  @override
  List<Object?> get props => [address];

  factory InternetNode.fromRpcInternetNodesEntry(InternetNodesEntry entry) {
    return InternetNode(
      entry.address,
      isActive: entry.enabled,
      name: entry.name,
    );
  }
}
