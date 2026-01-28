import 'dart:typed_data';

import 'package:flutter_test/flutter_test.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:qaul_rpc/src/generated/services/dtn/dtn_rpc.pb.dart';
import 'package:qaul_rpc/src/models/user.dart';

class _OverrideDefaultUserNotifier extends DefaultUserNotifier {
  _OverrideDefaultUserNotifier(this._value);
  final User? _value;
  @override
  User? build() => _value;
}

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();

  late ProviderContainer container;

  setUp(() {
    container = ProviderContainer.test();
  });

  group('defaultUserProvider', () {
    test('initial state is null', () {
      expect(container.read(defaultUserProvider), isNull);
    });

    test('notifier.state can be set and read', () {
      final user = User(
        name: 'test',
        id: Uint8List.fromList([1, 2, 3]),
      );
      container.read(defaultUserProvider.notifier).state = user;
      expect(container.read(defaultUserProvider), user);
    });

    test('listeners are notified on state changes', () {
      final stateChanges = <User?>[];
      container.listen<User?>(
        defaultUserProvider,
        (previous, next) => stateChanges.add(next),
        fireImmediately: false,
      );
      final user = User(name: 'a', id: Uint8List.fromList([1]));
      container.read(defaultUserProvider.notifier).state = user;
      expect(stateChanges, [user]);
    });

    test('overrideWith allows overriding for tests (chat_tab_test pattern)', () {
      final overridden = ProviderContainer(
        overrides: [
          defaultUserProvider.overrideWith(() =>
              _OverrideDefaultUserNotifier(
                  User(name: 'overridden', id: Uint8List.fromList([9, 9, 9])))),
        ],
      );
      addTearDown(overridden.dispose);
      expect(
        overridden.read(defaultUserProvider)?.name,
        'overridden',
      );
    });
  });

  group('currentSecurityNoProvider', () {
    test('initial state is null', () {
      expect(container.read(currentSecurityNoProvider), isNull);
    });

    test('notifier.state can be set and read', () {
      final sn = SecurityNumber(
        userId: Uint8List.fromList([1]),
        securityHash: Uint8List.fromList([2]),
        securityNumberBlocks: [11111, 22222],
      );
      container.read(currentSecurityNoProvider.notifier).state = sn;
      expect(container.read(currentSecurityNoProvider), sn);
    });
  });

  group('nodeInfoProvider', () {
    test('initial state is null', () {
      expect(container.read(nodeInfoProvider), isNull);
    });

    test('notifier.state can be set and read', () {
      final info = NodeInfo('id', ['addr1']);
      container.read(nodeInfoProvider.notifier).state = info;
      expect(container.read(nodeInfoProvider), info);
    });
  });

  group('connectedNodesProvider', () {
    test('initial state is empty list', () {
      expect(container.read(connectedNodesProvider), isEmpty);
    });

    test('notifier.state can be set and read', () {
      final nodes = [
        InternetNode('/ip4/1.2.3.4/udp/1234/quic-v1',
            isActive: true, name: 'n1'),
      ];
      container.read(connectedNodesProvider.notifier).state = nodes;
      expect(container.read(connectedNodesProvider), nodes);
    });
  });

  group('dtnConfigurationProvider', () {
    test('initial state is null', () {
      expect(container.read(dtnConfigurationProvider), isNull);
    });

    test('notifier.state can be set and read', () {
      final res = DtnConfigResponse()..totalSize = 100;
      final config = DTNConfiguration.fromRpcConfigResponse(res, []);
      container.read(dtnConfigurationProvider.notifier).state = config;
      expect(container.read(dtnConfigurationProvider), config);
    });
  });

  group('currentOpenChatRoom', () {
    test('initial state is null', () {
      expect(container.read(currentOpenChatRoom), isNull);
    });

    test('notifier.state can be set and read', () {
      final room = ChatRoom(
        conversationId: Uint8List.fromList([1, 2, 3]),
      );
      container.read(currentOpenChatRoom.notifier).state = room;
      expect(container.read(currentOpenChatRoom), room);
    });
  });

  group('bleStatusProvider', () {
    test('initial state is null', () {
      expect(container.read(bleStatusProvider), isNull);
    });

    test('notifier.state can be set and read', () {
      final status = BleConnectionStatus(bleId: Uint8List.fromList([1]));
      container.read(bleStatusProvider.notifier).state = status;
      expect(container.read(bleStatusProvider), status);
    });
  });

  group('libqaulLogsStoragePath', () {
    test('initial state is null', () {
      expect(container.read(libqaulLogsStoragePath), isNull);
    });

    test('notifier.state can be set and read', () {
      container.read(libqaulLogsStoragePath.notifier).state = '/tmp/logs';
      expect(container.read(libqaulLogsStoragePath), '/tmp/logs');
    });
  });
}
