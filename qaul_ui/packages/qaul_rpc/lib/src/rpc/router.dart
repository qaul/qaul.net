import 'package:fast_base58/fast_base58.dart';
import 'package:flutter/foundation.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:qaul_rpc/src/generated/router/router.pb.dart';
import 'package:qaul_rpc/src/generated/rpc/qaul_rpc.pb.dart';
import 'package:qaul_rpc/src/models/user.dart';
import 'package:qaul_rpc/src/rpc/rpc_module.dart';
import 'package:riverpod/riverpod.dart';

class RpcRouter extends RpcModule {
  RpcRouter(Reader read) : super(read);

  @override
  Modules get type => Modules.ROUTER;

  @override
  Future<void> decodeReceivedMessage(List<int> bytes) async {
    // decode bytes to message
    Router message = Router.fromBuffer(bytes);

    // send message to the appropriate module
    switch (message.whichMessage()) {
      case Router_Message.routingTable:
        final users = message.ensureRoutingTable().routingTable;
        debugPrint('Router received Users: $users');

        final provider = read(usersProvider.notifier);

        for (final u in users) {
          final domainUser = User(
            name: 'Name Undefined',
            idBase58: Base58Encode(u.userId),
            id: u.userId,
            availableTypes: _mapFromRoutingTableConnections(u.connections),
          );

          if (provider.contains(domainUser.idBase58)) {
            provider.update(domainUser);
            continue;
          }
          provider.add(domainUser);
        }

        break;
      default:
        throw UnhandledRpcMessageException.value(
          message.whichMessage().toString(),
          runtimeType.toString(),
        );
    }
  }

  Future<void> requestUsers() async {
    final msg = Router(routingTableRequest: RoutingTableRequest());
    await encodeAndSendMessage(msg);
  }

  List<ConnectionType> _mapFromRoutingTableConnections(
      List<RoutingTableConnection> connections) {
    ConnectionType _toType(RoutingTableConnection c) {
      if (c.module == ConnectionModule.LOCAL) {
        return ConnectionType.local;
      }
      if (c.module == ConnectionModule.LAN) {
        return ConnectionType.lan;
      }
      if (c.module == ConnectionModule.INTERNET) {
        return ConnectionType.internet;
      }
      if (c.module == ConnectionModule.BLE) {
        return ConnectionType.ble;
      }
      throw ArgumentError.value(c, 'ConnectionModule', 'value not mapped');
    }

    return connections
        .where((c) => c.module != ConnectionModule.NONE)
        .map(_toType)
        .toList();
  }
}

class UserListNotifier extends StateNotifier<List<User>> {
  UserListNotifier({List<User>? users}) : super(users ?? []);

  void add(User u) {
    state = [...state, u];
  }

  void update(User u) {
    assert(u.id != null);
    final i = state.indexWhere((e) => u.idBase58 == e.idBase58 || e.id == u.id);
    if (i.isNegative) throw ArgumentError.value(u, 'User', 'user not in state');
    final beforeUpdate = state[i];
    state[i] = User(
      name: beforeUpdate.name == 'Name Undefined' ? u.name : beforeUpdate.name,
      idBase58: u.idBase58,
      id: u.id,
      status: u.status,
      key: u.key ?? beforeUpdate.key,
      keyType: u.keyType ?? beforeUpdate.keyType,
      keyBase58: u.keyBase58 ?? beforeUpdate.keyBase58,
      isBlocked: u.isBlocked ?? beforeUpdate.isBlocked,
      isVerified: u.isVerified ?? beforeUpdate.isVerified,
      availableTypes: u.availableTypes ?? beforeUpdate.availableTypes,
    );
  }

  bool contains(String idBase58) =>
      !state.indexWhere((u) => u.idBase58 == idBase58).isNegative;
}
