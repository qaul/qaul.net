import 'package:equatable/equatable.dart';
import 'package:hooks_riverpod/legacy.dart';

import '../generated/connections/transports.pb.dart';

final networkTransportsProvider =
    StateProvider<List<NetworkTransport>>((ref) => []);

/// Keeps [networkTransportsProvider] in sync after future-based RPC calls.
void syncNetworkTransports(
  StateController<List<NetworkTransport>> notifier,
  List<NetworkTransport> transports,
) {
  notifier.state = transports;
}

class NetworkTransport extends Equatable {
  const NetworkTransport({
    required this.id,
    required this.label,
    required this.status,
    required this.enabled,
    required this.supportsRuntimeToggle,
    required this.isLocalOnly,
  });

  final String id;
  final String label;
  final String status;
  final bool enabled;
  final bool supportsRuntimeToggle;
  final bool isLocalOnly;

  factory NetworkTransport.fromRpc(TransportInfo transport) {
    return NetworkTransport(
      id: transport.id,
      label: transport.label,
      status: transport.status,
      enabled: transport.enabled,
      supportsRuntimeToggle: transport.supportsRuntimeToggle,
      isLocalOnly: transport.isLocalOnly,
    );
  }

  @override
  List<Object?> get props => [
        id,
        label,
        status,
        enabled,
        supportsRuntimeToggle,
        isLocalOnly,
      ];
}
