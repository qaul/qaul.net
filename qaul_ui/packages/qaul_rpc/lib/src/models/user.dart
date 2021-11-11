import 'package:equatable/equatable.dart';

enum ConnectionStatus { online, reachable, offline }

enum ConnectionType { lan, internet, ble, local }

class User extends Equatable {
  const User({
    required this.name,
    required this.idBase58,
    this.id,
    this.key,
    this.keyType,
    this.keyBase58,
    this.availableTypes,
    this.status = ConnectionStatus.offline,
  });

  final String name;
  final String idBase58;
  final List<int>? id;
  final List<int>? key;
  final String? keyType;
  final String? keyBase58;
  final List<ConnectionType>? availableTypes;
  final ConnectionStatus status;

  @override
  List<Object?> get props => [name, idBase58];
}
