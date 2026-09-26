import 'package:equatable/equatable.dart';

import '../generated/connections/transports.pb.dart';

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
