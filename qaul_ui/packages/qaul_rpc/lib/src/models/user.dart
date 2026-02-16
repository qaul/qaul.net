import 'dart:typed_data';

import 'package:collection/collection.dart';
import 'package:equatable/equatable.dart';
import 'package:fast_base58/fast_base58.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:hooks_riverpod/legacy.dart';

final defaultUserProvider = StateProvider<User?>((ref) => null);

final usersProvider = NotifierProvider<UserListNotifier, List<User>>(
  UserListNotifier.new,
);

class UsersPaginationStateNotifier extends Notifier<UsersPaginationState?> {
  @override
  UsersPaginationState? build() => null;

  void setPagination(UsersPaginationState? value) {
    state = value;
  }
}

final usersPaginationStateProvider =
    NotifierProvider<UsersPaginationStateNotifier, UsersPaginationState?>(
  UsersPaginationStateNotifier.new,
);

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

class UserListNotifier extends Notifier<List<User>> {
  @override
  List<User> build() => [];

  void add(User u) {
    state = [...state, u];
  }

  /// [updateMany] safely assigns [users] to this notifier's state.
  ///
  /// If [users] and [state] are deeply equal, will do nothing. As a result, it
  /// avoids re-rendering UI code that depends on the [List<User>] that this
  /// notifier exposes.
  ///
  /// New users get appended to the list, whilst existing ones get their data
  /// updated.
  void updateMany(List<User> users) {
    if (const ListEquality().equals(state, users)) {
      return;
    }

    final usrs = [...state];
    for (final u in users) {
      final idx = usrs.indexOf(u);
      if (idx == -1) {
        usrs.add(u);
        continue;
      }
      final current = usrs.elementAt(idx);
      usrs[idx] = User(
        name: current.name == 'Name Undefined' ? u.name : current.name,
        id: u.id,
        conversationId: u.conversationId ?? current.conversationId,
        status:
            u.status == ConnectionStatus.offline ? current.status : u.status,
        keyBase58: u.keyBase58 ?? current.keyBase58,
        isBlocked: u.isBlocked ?? current.isBlocked,
        isVerified: u.isVerified ?? current.isVerified,
        availableTypes: u.availableTypes ?? current.availableTypes,
      );
    }
    state = usrs;
  }

  void update(User u) {
    state = state.map((usr) {
      if (usr.id != u.id && usr.idBase58 != u.idBase58) {
        return usr;
      }
      return User(
        name: usr.name == 'Name Undefined' ? u.name : usr.name,
        id: u.id,
        conversationId: u.conversationId ?? usr.conversationId,
        status: u.status == ConnectionStatus.offline ? usr.status : u.status,
        keyBase58: u.keyBase58 ?? usr.keyBase58,
        isBlocked: u.isBlocked ?? usr.isBlocked,
        isVerified: u.isVerified ?? usr.isVerified,
        availableTypes: u.availableTypes ?? usr.availableTypes,
      );
    }).toList();
  }

  bool contains(User usr) => !state
      .indexWhere((u) => u.id == usr.id || u.idBase58 == usr.idBase58)
      .isNegative;

  void appendMany(List<User> users) {
    final existingIds = state.map((u) => u.idBase58).toSet();
    final newUsers = users.where((u) => !existingIds.contains(u.idBase58)).toList();
    if (newUsers.isEmpty) return;
    state = [...state, ...newUsers];
  }

  void replaceAll(List<User> users) {
    state = users;
  }
}

class PaginatedUsers {
  PaginatedUsers({
    required this.users,
    this.pagination,
  });

  final List<User> users;
  final UsersPaginationState? pagination;
}

class UsersPaginationState {
  const UsersPaginationState({
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
