import 'package:flutter_test/flutter_test.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/qaul_rpc.dart';

const _bleTransport = NetworkTransport(
  id: 'ble',
  label: 'BLE',
  status: 'running',
  enabled: true,
  supportsRuntimeToggle: true,
  isLocalOnly: true,
);

void main() {
  group('syncNetworkTransports', () {
    test('writes transport state to networkTransportsProvider', () {
      final container = ProviderContainer.test();
      addTearDown(container.dispose);

      syncNetworkTransports(
        container.read(networkTransportsProvider.notifier),
        [_bleTransport],
      );

      expect(container.read(networkTransportsProvider), [_bleTransport]);
    });

    test('replaces a previous list, including with an empty response', () {
      final container = ProviderContainer.test();
      addTearDown(container.dispose);

      syncNetworkTransports(
        container.read(networkTransportsProvider.notifier),
        [_bleTransport],
      );
      syncNetworkTransports(
        container.read(networkTransportsProvider.notifier),
        [],
      );

      expect(container.read(networkTransportsProvider), isEmpty);
    });
  });
}
