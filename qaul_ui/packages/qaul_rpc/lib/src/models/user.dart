import 'dart:typed_data';

import 'package:equatable/equatable.dart';
import 'package:fast_base58/fast_base58.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:hooks_riverpod/legacy.dart';

final defaultUserProvider = StateProvider<User?>((ref) => null);

/// Read-only mirror of user data for the RPC layer.
///
/// Populated by `UsersStore` after every state change so that translators
/// (which live in the `qaul_rpc` package and cannot import app-layer stores)
/// can look up users during message decoding.
final userLookupProvider = StateProvider<List<User>>((ref) => []);

/// Resolves a user by raw id bytes when they are missing from [userLookupProvider].
///
/// The app must override this (typically to call [LibqaulWorker.getUserById]) so group
/// and chat decoding can load members that are not in the paginated user list.
final fetchUserByIdForRpcProvider =
    Provider<Future<User?> Function(Uint8List userId)?>((ref) => null);

/// When a user is fetched during group decoding, the app may merge them into [UsersStore].
final onGroupMemberUserResolvedProvider = Provider<void Function(User)?>((ref) => null);

class PaginationState {
  const PaginationState({
    required this.hasMore,
    required this.total,
    required this.offset,
    required this.limit,
  });

  final bool hasMore;
  final int total;
  final int offset;
  final int limit;
}

enum ConnectionStatus { online, reachable, offline }

enum ConnectionType { lan, internet, ble, local }

class User with EquatableMixin implements Comparable<User> {
  User({
    required this.name,
    required this.id,
    this.conversationId,
    this.keyBase58,
    this.availableTypes,
    this.isBlocked,
    this.isVerified,
    this.status = ConnectionStatus.offline,
  }) : idBase58 = Base58Encode(id);

  final String name;
  final Uint8List id;
  final Uint8List? conversationId;
  final String? keyBase58;
  final Map<ConnectionType, ConnectionInfo>? availableTypes;
  final bool? isBlocked;
  final bool? isVerified;
  final ConnectionStatus status;

  final String idBase58;

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

  bool get isConnected =>
      availableTypes?.isNotEmpty ?? status == ConnectionStatus.online;

  User copyWith({required Map<ConnectionType, ConnectionInfo> availableTypes}) {
    return User(
      name: name,
      id: id,
      conversationId: conversationId,
      keyBase58: keyBase58,
      availableTypes: availableTypes,
      isBlocked: isBlocked,
      isVerified: isVerified,
      status: status,
    );
  }
}

class ConnectionInfo extends Equatable {
  const ConnectionInfo(
      {this.ping, this.hopCount, this.nodeID, this.nodeIDBase58});

  final int? ping;
  final int? hopCount;
  final Uint8List? nodeID;
  final String? nodeIDBase58;

  @override
  List<Object?> get props => [ping, hopCount, nodeID, nodeIDBase58];
}

class PaginatedUsers {
  PaginatedUsers({
    required this.users,
    this.pagination,
  });

  final List<User> users;
  final PaginationState? pagination;
}
