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
      case Router_Message.connectionsList:
        final users = <ConnectionsUserEntry>[];
        users.addAll(message.ensureConnectionsList().ble);
        users.addAll(message.ensureConnectionsList().internet);
        users.addAll(message.ensureConnectionsList().local);
        users.addAll(message.ensureConnectionsList().lan);
        for (final u in users) {
          read(usersProvider).add(
            User(
              name: 'Name Undefined',
              idBase58: Base58Encode(u.userId),
              id: u.userId,
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
    final msg = Router(connectionsRequest: ConnectionsRequest());
    await encodeAndSendMessage(msg);
  }
}

class UserListNotifier extends StateNotifier<List<User>> {
  UserListNotifier({List<User>? users}) : super(users ?? []);

  void add(User u) => [...state, u];
}
