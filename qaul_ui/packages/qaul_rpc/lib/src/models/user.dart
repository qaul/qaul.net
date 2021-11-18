import 'dart:typed_data';

import 'package:equatable/equatable.dart';

enum ConnectionStatus { online, reachable, offline }

enum ConnectionType { lan, internet, ble, local }

class User extends Equatable {
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
  List<Object?> get props => [name, idBase58];
}

class ConnectionInfo extends Equatable {
  const ConnectionInfo({this.ping, this.hopCount, this.nodeID});

  final int? ping;
  final int? hopCount;
  final Uint8List? nodeID;

  @override
  List<Object?> get props => [ping, hopCount, nodeID];
}
