import 'dart:typed_data';

import 'package:fast_base58/fast_base58.dart';
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
        final provider = read(usersProvider.notifier);

        for (final u in users) {
          var map = _mapFromRoutingTableConnections(u.connections);

          final domainUser = User(
            name: 'Name Undefined',
            idBase58: Base58Encode(u.userId),
            id: Uint8List.fromList(u.userId),
            availableTypes: map,
          );

          // map.forEach((key, value) => debugPrint(
          //     'USER: ${domainUser.idBase58}\nKEY: $key, VALUE: $value'));

          if (provider.contains(domainUser)) {
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

  Future<void> requestUsers() async => await encodeAndSendMessage(
      Router(routingTableRequest: RoutingTableRequest()));

  Map<ConnectionType, ConnectionInfo> _mapFromRoutingTableConnections(
      List<RoutingTableConnection> connections) {
    ConnectionType toType(RoutingTableConnection c) {
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

    ConnectionInfo toConnectionInfo(RoutingTableConnection c) =>
        ConnectionInfo(ping: c.rtt, nodeID: Uint8List.fromList(c.via));

    return Map.fromEntries(connections
        .where((c) => c.module != ConnectionModule.NONE)
        .map((e) => MapEntry(toType(e), toConnectionInfo(e))));
  }
}

class UserListNotifier extends StateNotifier<List<User>> {
  UserListNotifier({List<User>? users}) : super(users ?? []);

  void add(User u) {
    state = [...state, u];
  }

  void update(User u) {
    state = [
      for (final usr in state)
        if (usr.id == u.id || usr.idBase58 == u.idBase58)
          User(
            name: usr.name == 'Name Undefined' ? u.name : usr.name,
            idBase58: u.idBase58,
            id: u.id,
            status:
                u.status == ConnectionStatus.offline ? usr.status : u.status,
            key: u.key ?? usr.key,
            keyType: u.keyType ?? usr.keyType,
            keyBase58: u.keyBase58 ?? usr.keyBase58,
            isBlocked: u.isBlocked ?? usr.isBlocked,
            isVerified: u.isVerified ?? usr.isVerified,
            availableTypes: u.availableTypes ?? usr.availableTypes,
          )
        else
          usr,
    ];
  }

  bool contains(User usr) => !state
      .indexWhere((u) => u.id == usr.id || u.idBase58 == usr.idBase58)
      .isNegative;
}
