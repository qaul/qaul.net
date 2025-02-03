part of 'abstract_rpc_module_translator.dart';

class UsersTranslator extends RpcModuleTranslator {
  @override
  Modules get type => Modules.USERS;

  @override
  Future<RpcTranslatorResponse?> decodeMessageBytes(
      List<int> data, Ref ref) async {
    final message = Users.fromBuffer(data);
    switch (message.whichMessage()) {
      case Users_Message.userList:
        final users = message
            .ensureUserList()
            .user
            .map((u) => User(
                  name: u.name,
                  id: Uint8List.fromList(u.id),
                  conversationId: Uint8List.fromList(u.groupId),
                  keyBase58: u.keyBase58,
                  isBlocked: u.blocked,
                  isVerified: u.verified,
                  status: _mapFrom(u.connectivity),
                  availableTypes: _mapFromRoutingTable(u.connections),
                ))
            .toList();

        return RpcTranslatorResponse(type, users);
      case Users_Message.securityNumberResponse:
        final res = message.ensureSecurityNumberResponse();
        final secNo = SecurityNumber(
          userId: Uint8List.fromList(res.userId),
          securityHash: Uint8List.fromList(res.securityHash),
          securityNumberBlocks: res.securityNumberBlocks,
        );
        return RpcTranslatorResponse(type, secNo);
      default:
        return super.decodeMessageBytes(data, ref);
    }
  }

  ConnectionStatus _mapFrom(Connectivity c) {
    if (c == Connectivity.Online) return ConnectionStatus.online;
    if (c == Connectivity.Reachable) return ConnectionStatus.reachable;
    return ConnectionStatus.offline;
  }

  Map<ConnectionType, ConnectionInfo> _mapFromRoutingTable(
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
  Future<void> processResponse(RpcTranslatorResponse res, Ref ref) async {
    if (res.module != type) return;
    if (res.data is List<User>) {
      final provider = ref.read(usersProvider.notifier);
      for (final user in res.data) {
        provider.contains(user) ? provider.update(user) : provider.add(user);
      }
    }
    if (res.data is SecurityNumber) {
      ref.read(currentSecurityNoProvider.notifier).state = res.data;
    }
  }
}
