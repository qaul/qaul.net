part of 'abstract_rpc_module_translator.dart';

class BleTranslator extends RpcModuleTranslator {
  @override
  Modules get type => Modules.BLE;

  @override
  Future<RpcTranslatorResponse?> decodeMessageBytes(List<int> data) async {
    final message = Ble.fromBuffer(data);
    switch (message.whichMessage()) {
      case Ble_Message.infoResponse:
        final msg = message.ensureInfoResponse();
        final status = BleConnectionStatus(
          bleId: Uint8List.fromList(msg.smallId),
          status: msg.status,
          deviceInfo: Uint8List.fromList(msg.deviceInfo),
        );
        return RpcTranslatorResponse(Modules.BLE, status);
      case Ble_Message.discoveredResponse:
        final msg = message.ensureDiscoveredResponse();
        final status = BleConnectionStatus(
          bleId: Uint8List(0),
          discoveredNodes: msg.nodesCount,
          nodesPendingConfirmation: msg.toConfirmCount,
        );
        return RpcTranslatorResponse(Modules.BLE, status);
      default:
        return super.decodeMessageBytes(data);
    }
  }
}
