import 'package:equatable/equatable.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

import '../generated/connections/connections.pb.dart';

final connectedNodesProvider = StateProvider<List<InternetNode>>((ref) => []);

class InternetNode extends Equatable {
  InternetNode(
    this.address, {
    required this.isActive,
    required this.name,
  })  : isIPv4 = address.contains('/ip4/'),
        isQuic = address.contains(_quicPathDescriptor),
        ip = address.replaceAll('/ip4/', '').split('/').first,
        port = _extractPortFromAddress(address);

  final String address;
  final bool isActive;
  final String name;

  final bool isIPv4;
  final String ip;
  final String port;
  final bool isQuic;

  static const _quicPathDescriptor = 'quic-v1';

  static String _extractPortFromAddress(String addr) {
    final addressSections = addr.split('/');
    if (addressSections.last != _quicPathDescriptor) {
      return addressSections.last;
    }
    addressSections.removeLast();
    return addressSections.last;
  }

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
