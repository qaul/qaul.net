import 'dart:typed_data';

import 'package:equatable/equatable.dart';
import 'package:fast_base58/fast_base58.dart';

class LocalAccount extends Equatable {
  LocalAccount({
    required this.username,
    required this.userId,
    this.salt,
    required this.hasPassword,
  }) : userIdBase58 = Base58Encode(userId);

  final String username;
  final Uint8List userId;
  final String userIdBase58;
  final String? salt;
  final bool hasPassword;

  @override
  List<Object?> get props => [username, userIdBase58, salt, hasPassword];
}

class RestoreAccountResult extends Equatable {
  const RestoreAccountResult({
    required this.userId,
    required this.userIdBase58,
  });

  final Uint8List userId;
  final String userIdBase58;

  @override
  List<Object?> get props => [userIdBase58];
}

class RpcRequestException implements Exception {
  const RpcRequestException(this.message, {this.code, this.details});

  final String message;
  final int? code;
  final String? details;

  @override
  String toString() {
    final suffix = details == null || details!.isEmpty ? '' : ' ($details)';
    return 'RpcRequestException: $message$suffix';
  }
}
