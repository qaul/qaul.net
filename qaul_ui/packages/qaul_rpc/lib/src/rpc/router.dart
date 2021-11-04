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
      case Router_Message.userList:
        final users = message.ensureUserList().user;
        for (final u in users) {
          read(usersProvider).add(
            User(
              name: u.name,
              idBase58: u.idBase58,
              id: u.id,
              key: u.key,
              keyBase58: u.keyBase58,
              keyType: u.keyType,
              status: _mapConnectionStatusFrom(u.connectivity),
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
    final msg = Router(userRequest: UserRequest());
    await encodeAndSendMessage(msg);
  }

  ConnectionStatus _mapConnectionStatusFrom(Connectivity c) {
    if (c == Connectivity.Online) return ConnectionStatus.online;
    if (c == Connectivity.Offline) return ConnectionStatus.offline;
    if (c == Connectivity.Reachable) return ConnectionStatus.reachable;
    throw 'Unhandled value of Connectivity: $c';
  }
}

class UserListNotifier extends StateNotifier<List<User>> {
  UserListNotifier({List<User>? users}) : super(users ?? []);

  void add(User u) => [...state, u];
}
