part of 'abstract_rpc_module_translator.dart';

class RouterTranslator extends RpcModuleTranslator {
  @override
  Modules get type => Modules.ROUTER;

  @override
  Future<RpcTranslatorResponse?> decodeMessageBytes(List<int> data) async {
    final message = Router.fromBuffer(data);
    switch (message.whichMessage()) {
      case Router_Message.routingTable:
        final data = message
            .ensureRoutingTable()
            .routingTable
            .map(
              (e) => User(
                name: 'Name Undefined',
                idBase58: Base58Encode(e.userId),
                id: Uint8List.fromList(e.userId),
                availableTypes: _mapFromRoutingTableConnections(e.connections),
                status: _isConnected(e)
                    ? ConnectionStatus.online
                    : ConnectionStatus.offline,
              ),
            )
            .toList();

        return RpcTranslatorResponse(type, data);
      default:
        return super.decodeMessageBytes(data);
    }
  }

  bool _isConnected(RoutingTableEntry e) => e.connections.isNotEmpty;

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

    ConnectionInfo toConnectionInfo(RoutingTableConnection c) => ConnectionInfo(
          ping: c.rtt ~/ 1000,
          hopCount: c.hopCount == 0 ? 1 : c.hopCount,
          nodeID: Uint8List.fromList(c.via),
          nodeIDBase58: Base58Encode(c.via),
        );

    return Map.fromEntries(connections
        .where((c) => c.module != ConnectionModule.NONE)
        .map((e) => MapEntry(toType(e), toConnectionInfo(e))));
  }

  @override
  Future<void> processResponse(RpcTranslatorResponse res, Reader reader) async {
    if (res.module != type) return;
    final provider = reader(usersProvider.notifier);
    if (res.data is List<User>) {
      for (final user in res.data) {
        provider.contains(user) ? provider.update(user) : provider.add(user);
      }
    } else if (res.data is User) {
      if (provider.contains(res.data)) provider.update(res.data);
    }
  }
}
