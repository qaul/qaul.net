import 'dart:typed_data';

import 'package:collection/collection.dart';
import 'package:equatable/equatable.dart';
import 'package:fast_base58/fast_base58.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:hooks_riverpod/legacy.dart';

final defaultUserProvider = StateProvider<User?>((ref) => null);

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

class PaginatedData<T> {
  const PaginatedData({
    required this.data,
    this.pagination,
  });

  final List<T> data;
  final PaginationState? pagination;
}

final usersProvider = NotifierProvider<PaginatedDataNotifier<User>, PaginatedData<User>>(
  PaginatedDataNotifier.new,
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

class PaginatedDataNotifier<T> extends Notifier<PaginatedData<T>> {
  @override
  PaginatedData<T> build() => const PaginatedData(data: []);

  void add(T item) {
    state = PaginatedData(
      data: [...state.data, item],
      pagination: state.pagination,
    );
  }

  void updateMany(List<T> items) {
    if (const ListEquality().equals(state.data, items)) {
      return;
    }

    final updatedItems = [...state.data];
    for (final item in items) {
      final idx = updatedItems.indexOf(item);
      if (idx == -1) {
        updatedItems.add(item);
        continue;
      }
      updatedItems[idx] = item;
    }
    state = PaginatedData(
      data: updatedItems,
      pagination: state.pagination,
    );
  }

  void update(T item) {
    state = PaginatedData(
      data: state.data.map((existing) => existing == item ? item : existing).toList(),
      pagination: state.pagination,
    );
  }

  bool contains(T item) => state.data.contains(item);

  void appendMany(List<T> items) {
    final existingIds = state.data.toSet();
    final newItems = items.where((item) => !existingIds.contains(item)).toList();
    if (newItems.isEmpty) return;
    state = PaginatedData(
      data: [...state.data, ...newItems],
      pagination: state.pagination,
    );
  }

  void replaceAll(List<T> items, {PaginationState? pagination}) {
    state = PaginatedData(
      data: items,
      pagination: pagination ?? state.pagination,
    );
  }

  void setPagination(PaginationState? pagination) {
    state = PaginatedData(
      data: state.data,
      pagination: pagination,
    );
  }
}

class UserListNotifier extends PaginatedDataNotifier<User> {
  @override
  void updateMany(List<User> items) {
    if (const ListEquality().equals(state.data, items)) {
      return;
    }

    final usrs = [...state.data];
    for (final u in items) {
      final idx = usrs.indexWhere((usr) => usr.id == u.id || usr.idBase58 == u.idBase58);
      if (idx == -1) {
        usrs.add(u);
        continue;
      }
      final current = usrs.elementAt(idx);
      usrs[idx] = User(
        name: current.name == 'Name Undefined' ? u.name : current.name,
        id: u.id,
        conversationId: u.conversationId ?? current.conversationId,
        status: u.status == ConnectionStatus.offline ? current.status : u.status,
        keyBase58: u.keyBase58 ?? current.keyBase58,
        isBlocked: u.isBlocked ?? current.isBlocked,
        isVerified: u.isVerified ?? current.isVerified,
        availableTypes: u.availableTypes ?? current.availableTypes,
      );
    }
    state = PaginatedData(
      data: usrs,
      pagination: state.pagination,
    );
  }

  @override
  void update(User item) {
    state = PaginatedData(
      data: state.data.map((usr) {
        if (usr.id != item.id && usr.idBase58 != item.idBase58) {
          return usr;
        }
        return User(
          name: usr.name == 'Name Undefined' ? item.name : usr.name,
          id: item.id,
          conversationId: item.conversationId ?? usr.conversationId,
          status: item.status == ConnectionStatus.offline ? usr.status : item.status,
          keyBase58: item.keyBase58 ?? usr.keyBase58,
          isBlocked: item.isBlocked ?? usr.isBlocked,
          isVerified: item.isVerified ?? usr.isVerified,
          availableTypes: item.availableTypes ?? usr.availableTypes,
        );
      }).toList(),
      pagination: state.pagination,
    );
  }

  @override
  bool contains(User item) => state.data
      .indexWhere((u) => u.id == item.id || u.idBase58 == item.idBase58)
      .isNegative == false;

  @override
  void appendMany(List<User> items) {
    final existingIds = state.data.map((u) => u.idBase58).toSet();
    final newUsers = items.where((u) => !existingIds.contains(u.idBase58)).toList();
    if (newUsers.isEmpty) return;
    state = PaginatedData(
      data: [...state.data, ...newUsers],
      pagination: state.pagination,
    );
  }
}

class PaginatedUsers {
  PaginatedUsers({
    required this.users,
    this.pagination,
  });

  final List<User> users;
  final PaginationState? pagination;
}
