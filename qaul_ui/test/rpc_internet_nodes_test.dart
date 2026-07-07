import 'package:flutter_test/flutter_test.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/qaul_rpc.dart';

InternetNode _communityNode({required bool active}) => InternetNode(
      '/ip4/144.91.74.192/udp/9229/quic-v1',
      isActive: active,
      name: 'qaul Community Node [IPv4]',
    );

void main() {
  group('syncConnectedInternetNodes', () {
    test('writes community nodes to connectedNodesProvider', () {
      final container = ProviderContainer.test();
      addTearDown(container.dispose);

      final nodes = [_communityNode(active: false)];
      syncConnectedInternetNodes(container.read(connectedNodesProvider.notifier), nodes);

      expect(container.read(connectedNodesProvider), nodes);
    });

    test('replaces a previous list (including empty)', () {
      final container = ProviderContainer.test();
      addTearDown(container.dispose);

      syncConnectedInternetNodes(
        container.read(connectedNodesProvider.notifier),
        [_communityNode(active: true)],
      );
      syncConnectedInternetNodes(
        container.read(connectedNodesProvider.notifier),
        [],
      );

      expect(container.read(connectedNodesProvider), isEmpty);
    });
  });
}
