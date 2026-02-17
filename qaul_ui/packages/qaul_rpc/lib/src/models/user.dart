import 'dart:typed_data';

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
  static Map<String, int> _indexById(List<User> users) {
    final indexById = <String, int>{};
    for (var i = 0; i < users.length; i++) {
      indexById[users[i].idBase58] = i;
    }
    return indexById;
  }

  static User _mergeUser(User current, User incoming) {
    return User(
      name: current.name == 'Name Undefined' ? incoming.name : current.name,
      id: incoming.id,
      conversationId: incoming.conversationId ?? current.conversationId,
      status: incoming.status == ConnectionStatus.offline ? current.status : incoming.status,
      keyBase58: incoming.keyBase58 ?? current.keyBase58,
      isBlocked: incoming.isBlocked ?? current.isBlocked,
      isVerified: incoming.isVerified ?? current.isVerified,
      availableTypes: incoming.availableTypes ?? current.availableTypes,
    );
  }

  @override
  void updateMany(List<User> items) {
    final usrs = [...state.data];
    final indexById = _indexById(usrs);
    for (final u in items) {
      final idx = indexById[u.idBase58];
      if (idx == null) {
        usrs.add(u);
        indexById[u.idBase58] = usrs.length - 1;
        continue;
      }
      usrs[idx] = _mergeUser(usrs[idx], u);
    }
    state = PaginatedData(
      data: usrs,
      pagination: state.pagination,
    );
  }

  @override
  void update(User item) {
    final data = <User>[];
    for (final usr in state.data) {
      if (usr.id != item.id && usr.idBase58 != item.idBase58) {
        data.add(usr);
      } else {
        data.add(_mergeUser(usr, item));
      }
    }
    state = PaginatedData(
      data: data,
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
