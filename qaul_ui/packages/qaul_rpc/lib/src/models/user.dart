import 'dart:typed_data';

import 'package:equatable/equatable.dart';

enum ConnectionStatus { online, reachable, offline }

enum ConnectionType { lan, internet, ble, local }

class User with EquatableMixin implements Comparable<User> {
  const User({
    required this.name,
    required this.id,
    required this.idBase58,
    this.key,
    this.keyType,
    this.keyBase58,
    this.availableTypes,
    this.isBlocked,
    this.isVerified,
    this.status = ConnectionStatus.offline,
  });

  final String name;
  final String idBase58;
  final Uint8List id;
  final Uint8List? key;
  final String? keyType;
  final String? keyBase58;
  final Map<ConnectionType, ConnectionInfo>? availableTypes;
  final bool? isBlocked;
  final bool? isVerified;
  final ConnectionStatus status;

  @override
  int compareTo(dynamic other) {
    assert(
      runtimeType == other.runtimeType,
      "The sorting algorithm must not compare incomparable keys, since they don't "
      'know how to order themselves relative to each other. Comparing $this with $other',
    );
    // If blocked, always order after other. If other is connected, go after other. Otherwise, go before other.
    return (isBlocked ?? false)
        ? 1
        : (other as User).isConnected
            ? 1
            : -1;
  }

  @override
  List<Object?> get props => [name, idBase58];

  bool get isConnected => _statusIsNotOffline && _typesAreNotEmpty;

  bool get _statusIsNotOffline => status != ConnectionStatus.offline;

  bool get _typesAreNotEmpty => (availableTypes?.isNotEmpty ?? false);
}

class ConnectionInfo extends Equatable {
  const ConnectionInfo({this.ping, this.hopCount, this.nodeID, this.nodeIDBase58});

  final int? ping;
  final int? hopCount;
  final Uint8List? nodeID;
  final String? nodeIDBase58;

  @override
  List<Object?> get props => [ping, hopCount, nodeID, nodeIDBase58];
}
