import 'package:qaul_rpc/src/generated/connections/connections.pb.dart';
import 'package:qaul_rpc/src/generated/rpc/qaul_rpc.pb.dart';
import 'package:qaul_rpc/src/models/internet_node.dart';
import 'package:qaul_rpc/src/rpc/rpc_module.dart';
import 'package:riverpod/riverpod.dart';

import '../providers.dart';

class RpcConnections extends RpcModule {
  RpcConnections(Reader read) : super(read);

  @override
  Modules get type => Modules.CONNECTIONS;

  @override
  Future<void> decodeReceivedMessage(List<int> bytes) async {
    final message = Connections.fromBuffer(bytes);

    switch (message.whichMessage()) {
      case Connections_Message.internetNodesList:
        // TODO: evaluate message info field, show message if error?
        final nodes = message.ensureInternetNodesList().nodes;

        read(connectedNodesProvider.notifier).state =
            nodes.map((e) => InternetNode(e.address)).toList();
        break;
      default:
        throw UnhandledRpcMessageException.value(
          message.whichMessage().toString(),
          runtimeType.toString(),
        );
    }
  }

  Future<void> requestNodes() async => await encodeAndSendMessage(
      Connections(internetNodesRequest: InternetNodesRequest()));

  Future<void> addNode(String address) async => await encodeAndSendMessage(
      Connections(internetNodesAdd: InternetNodesEntry(address: address)));

  Future<void> removeNode(String address) async => await encodeAndSendMessage(
      Connections(internetNodesRemove: InternetNodesEntry(address: address)));
}
