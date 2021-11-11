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
          if (provider.contains(Base58Encode(u.userId))) continue;
          provider.add(
            User(
              name: 'Name Undefined',
              idBase58: Base58Encode(u.userId),
              id: u.userId,
              availableTypes: _mapFromRoutingTableConnections(u.connections),
            ),
          );
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

  bool contains(String idBase58) =>
      !state.indexWhere((u) => u.idBase58 == idBase58).isNegative;
}
