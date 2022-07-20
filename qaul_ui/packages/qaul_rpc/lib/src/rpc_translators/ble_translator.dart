part of 'abstract_rpc_module_translator.dart';

class BleRightsRequest {}

class BleTranslator extends RpcModuleTranslator {
  @override
  Modules get type => Modules.BLE;

  @override
  Future<RpcTranslatorResponse?> decodeMessageBytes(List<int> data, Reader reader) async {
    final message = Ble.fromBuffer(data);
    switch (message.whichMessage()) {
      case Ble_Message.infoResponse:
        final msg = message.ensureInfoResponse();
        final status = BleConnectionStatus(
          bleId: Uint8List.fromList(msg.smallId),
          status: msg.status,
          deviceInfo: Uint8List.fromList(msg.deviceInfo),
        );
        return RpcTranslatorResponse(type, status);
      case Ble_Message.discoveredResponse:
        final msg = message.ensureDiscoveredResponse();
        final status = BleConnectionStatus(
          bleId: Uint8List(0),
          discoveredNodes: msg.nodesCount,
          nodesPendingConfirmation: msg.toConfirmCount,
        );
        return RpcTranslatorResponse(Modules.BLE, status);
      case Ble_Message.rightsRequest:
        return RpcTranslatorResponse(Modules.BLE, BleRightsRequest());
      default:
        return super.decodeMessageBytes(data, reader);
    }
  }

  @override
  Future<void> processResponse(RpcTranslatorResponse res, Reader reader) async {
    if (res.module != type || res.data is! BleConnectionStatus) return;
    var newStatus = res.data as BleConnectionStatus;
    _log.finer('BLE Module: received new status $newStatus');
    final currentStatus = reader(bleStatusProvider);
    if (currentStatus != null) {
      newStatus = currentStatus.copyWith(
        status: newStatus.status,
        deviceInfo: newStatus.deviceInfo,
        discoveredNodes: newStatus.discoveredNodes,
        nodesPendingConfirmation: newStatus.discoveredNodes,
      );
      _log.finest('BLE Module: Merged with previous status: $newStatus');
    }
    reader(bleStatusProvider.state).state = newStatus;
  }
}
