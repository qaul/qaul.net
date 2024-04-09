import 'dart:typed_data';

import 'package:flutter_test/flutter_test.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:qaul_ui/screens/home/dynamic_network/dynamic_network_screen.dart';
import 'package:uuid/uuid.dart';

User generateUser(String name, {Map<ConnectionType, ConnectionInfo>? connections}) {
  final id = const Uuid().v4();
  return User(
    name: name,
    id: Uint8List.fromList(id.codeUnits),
    availableTypes: connections,
  );
}

NetworkNode toNetworkNode(User u, {User? parent, Set<NetworkNode> children = const {}}) =>
    NetworkNode(
      user: u,
      parentId: parent?.id,
      children: children,
    );

void main() {
  test('Returns node with no children if no users', () async {
    var user = generateUser('user');

    final tree = NetworkNode.fromUserData(user, const [], NetworkTypeFilter.bluetooth);

    expect(tree, NetworkNode(user: user, children: const {}));
  });

  test('Returns node with no children if no users match filter', () async {
    var user = generateUser('user');
    var users = [
      generateUser('',
          connections: const {ConnectionType.internet: ConnectionInfo(ping: 110, hopCount: 1)})
    ];

    final tree = NetworkNode.fromUserData(user, users, NetworkTypeFilter.bluetooth);

    expect(tree, NetworkNode(user: user, children: const {}));
  });

  test('Node with no children if users contains only users with hops > 1', () async {
    var user = generateUser('user');
    var users = [
      generateUser('',
          connections: const {ConnectionType.internet: ConnectionInfo(ping: 110, hopCount: 2)})
    ];

    final tree = NetworkNode.fromUserData(user, users, NetworkTypeFilter.internet);

    expect(tree, NetworkNode(user: user, children: const {}));
  });

  test('Node with single child if user matches filter & one hops', () async {
    var user = generateUser('user');
    final users = [
      generateUser('child1',
          connections: const {ConnectionType.internet: ConnectionInfo(ping: 110, hopCount: 1)}),
    ];

    final tree = NetworkNode.fromUserData(user, users, NetworkTypeFilter.internet);

    expect(
      tree,
      NetworkNode(
        user: user,
        children: {
          NetworkNode(
            user: users.first,
            parentId: user.id,
            children: const {},
          ),
        },
      ),
    );
  });

  test('Node with three children', () async {
    var user = generateUser('user');
    final users = [
      generateUser('child1',
          connections: const {ConnectionType.internet: ConnectionInfo(ping: 110, hopCount: 1)}),
      generateUser('child2',
          connections: const {ConnectionType.internet: ConnectionInfo(ping: 1000000, hopCount: 1)}),
      generateUser('child3', connections: const {
        ConnectionType.internet: ConnectionInfo(ping: 109324810924, hopCount: 1)
      }),
    ];

    final tree = NetworkNode.fromUserData(user, users, NetworkTypeFilter.internet);

    expect(
      tree,
      NetworkNode(
        user: user,
        children: users
            .map((e) => NetworkNode(
                  user: e,
                  parentId: user.id,
                  children: const {},
                ))
            .toSet(),
      ),
    );
  });

  test('Node with three children and a node with 3 hops', () async {
    var user = generateUser('user');
    final users = [
      generateUser('child1',
          connections: const {ConnectionType.internet: ConnectionInfo(ping: 110, hopCount: 1)}),
      generateUser('child2',
          connections: const {ConnectionType.internet: ConnectionInfo(ping: 1000000, hopCount: 1)}),
      generateUser('child3', connections: const {
        ConnectionType.internet: ConnectionInfo(ping: 109324810924, hopCount: 1)
      }),
    ];
    final users2 = [
      ...users,
      generateUser('child3', connections: const {
        ConnectionType.internet: ConnectionInfo(ping: 109324810924, hopCount: 3)
      }),
    ];

    final tree = NetworkNode.fromUserData(user, users2, NetworkTypeFilter.internet);

    expect(
      tree,
      NetworkNode(
        user: user,
        children: users
            .map((e) => NetworkNode(
                  user: e,
                  parentId: user.id,
                  children: const {},
                ))
            .toSet(),
      ),
    );
  });

  test('Node with three children and a node with 1 hop & null ping', () async {
    var user = generateUser('user');
    final users = [
      generateUser('child1',
          connections: const {ConnectionType.internet: ConnectionInfo(ping: 110, hopCount: 1)}),
      generateUser('child2',
          connections: const {ConnectionType.internet: ConnectionInfo(ping: 1000000, hopCount: 1)}),
      generateUser('child3', connections: const {
        ConnectionType.internet: ConnectionInfo(ping: 109324810924, hopCount: 1)
      }),
    ];
    final users2 = [
      ...users,
      generateUser('child3',
          connections: const {ConnectionType.internet: ConnectionInfo(ping: null, hopCount: 1)}),
    ];

    final tree = NetworkNode.fromUserData(user, users2, NetworkTypeFilter.internet);

    expect(
      tree,
      NetworkNode(
        user: user,
        children: users
            .map((e) => NetworkNode(
                  user: e,
                  parentId: user.id,
                  children: const {},
                ))
            .toSet(),
      ),
    );
  });

  test('Node with three children and a grand-child', () async {
    var user = generateUser('user');
    var child1 = generateUser('child1',
        connections: const {ConnectionType.internet: ConnectionInfo(ping: 110, hopCount: 1)});
    var child2 = generateUser('child2',
        connections: const {ConnectionType.internet: ConnectionInfo(ping: 1000000, hopCount: 1)});
    var child3 = generateUser('child3', connections: const {
      ConnectionType.internet: ConnectionInfo(ping: 109324810924, hopCount: 1)
    });
    var grandchild11 = generateUser('grandchild11',
        connections: const {ConnectionType.internet: ConnectionInfo(ping: 111, hopCount: 2)});
    final users = [child1, child2, child3, grandchild11];

    final tree = NetworkNode.fromUserData(user, users, NetworkTypeFilter.internet);

    expect(
      tree,
      NetworkNode(
        user: user,
        children: {
          toNetworkNode(child1, parent: user, children: {
            toNetworkNode(grandchild11, parent: child1),
          }),
          toNetworkNode(child2, parent: user),
          toNetworkNode(child3, parent: user),
        },
      ),
    );
  });

  test('Node with three children and a grand-child, picks last possible parent', () async {
    var user = generateUser('user');
    var child1 = generateUser('child1',
        connections: const {ConnectionType.internet: ConnectionInfo(ping: 110, hopCount: 1)});
    var child2 = generateUser('child2',
        connections: const {ConnectionType.internet: ConnectionInfo(ping: 1000000, hopCount: 1)});
    var child3 = generateUser('child3',
        connections: const {ConnectionType.internet: ConnectionInfo(ping: 110, hopCount: 1)});
    var grandchild31 = generateUser('grandchild31',
        connections: const {ConnectionType.internet: ConnectionInfo(ping: 111, hopCount: 2)});
    final users = [child1, child2, child3, grandchild31];

    final tree = NetworkNode.fromUserData(user, users, NetworkTypeFilter.internet);

    expect(
      tree,
      NetworkNode(
        user: user,
        children: {
          toNetworkNode(child1, parent: user),
          toNetworkNode(child2, parent: user),
          toNetworkNode(child3, parent: user, children: {
            toNetworkNode(grandchild31, parent: child3),
          }),
        },
      ),
    );
  });

  test('Node with three children and three grand-children', () async {
    var user = generateUser('user');
    var child1 = generateUser('child1',
        connections: const {ConnectionType.internet: ConnectionInfo(ping: 110, hopCount: 1)});
    var child2 = generateUser('child2',
        connections: const {ConnectionType.internet: ConnectionInfo(ping: 1000000, hopCount: 1)});
    var child3 = generateUser('child3',
        connections: const {ConnectionType.internet: ConnectionInfo(ping: 129387013, hopCount: 1)});
    var gc11 = generateUser('grandchild11',
        connections: const {ConnectionType.internet: ConnectionInfo(ping: 111, hopCount: 2)});
    var gc21 = generateUser('grandchild21',
        connections: const {ConnectionType.internet: ConnectionInfo(ping: 1010000, hopCount: 2)});
    var gc31 = generateUser('grandchild31',
        connections: const {ConnectionType.internet: ConnectionInfo(ping: 129387413, hopCount: 2)});
    final users = [child1, child2, child3, gc11, gc21, gc31];

    final tree = NetworkNode.fromUserData(user, users, NetworkTypeFilter.internet);

    expect(
      tree,
      NetworkNode(
        user: user,
        children: {
          toNetworkNode(child1, parent: user, children: {
            toNetworkNode(gc11, parent: child1),
          }),
          toNetworkNode(child2, parent: user, children: {
            toNetworkNode(gc21, parent: child2),
          }),
          toNetworkNode(child3, parent: user, children: {
            toNetworkNode(gc31, parent: child3),
          }),
        },
      ),
    );
  });

  test('Complex Node Tree', () async {
    var user = generateUser('user');
    var child1 = generateUser('child1',
        connections: const {ConnectionType.internet: ConnectionInfo(ping: 110, hopCount: 1)});
    var child2 = generateUser('child2',
        connections: const {ConnectionType.internet: ConnectionInfo(ping: 1000000, hopCount: 1)});
    var child3 = generateUser('child3',
        connections: const {ConnectionType.internet: ConnectionInfo(ping: 129387013, hopCount: 1)});
    var child4 = generateUser('child4', connections: const {
      ConnectionType.internet: ConnectionInfo(ping: 1279387013, hopCount: 1)
    });
    var gc11 = generateUser('grandchild11',
        connections: const {ConnectionType.internet: ConnectionInfo(ping: 111, hopCount: 2)});
    var ggc111 = generateUser('grandgrandchild111',
        connections: const {ConnectionType.internet: ConnectionInfo(ping: 129, hopCount: 3)});
    var gggc1111 = generateUser('grandgrandgrandchild1111',
        connections: const {ConnectionType.internet: ConnectionInfo(ping: 334, hopCount: 4)});
    var gggc1112 = generateUser('grandgrandgrandchild1112',
        connections: const {ConnectionType.internet: ConnectionInfo(ping: 298, hopCount: 4)});
    var gc12 = generateUser('grandchild12',
        connections: const {ConnectionType.internet: ConnectionInfo(ping: 157, hopCount: 2)});
    var gc13 = generateUser('grandchild13',
        connections: const {ConnectionType.internet: ConnectionInfo(ping: 559, hopCount: 2)});
    var gc21 = generateUser('grandchild21',
        connections: const {ConnectionType.internet: ConnectionInfo(ping: 1010000, hopCount: 2)});
    var ggc211 = generateUser('grandgrandchild211',
        connections: const {ConnectionType.internet: ConnectionInfo(ping: null, hopCount: 3)});
    var gggc2111 = generateUser('grandgrandgrandchild2111', connections: const {
      ConnectionType.internet: ConnectionInfo(ping: 1010300, hopCount: null)
    });
    var ggc212 = generateUser('grandgrandchild212',
        connections: const {ConnectionType.internet: ConnectionInfo(ping: 1010200, hopCount: 3)});
    var gc31 = generateUser('grandchild31',
        connections: const {ConnectionType.internet: ConnectionInfo(ping: 129387413, hopCount: 2)});
    final users = [
      child1,
      gc11,
      ggc111,
      gggc1111,
      gggc1112,
      gc12,
      gc13,
      child2,
      gc21,
      ggc211,
      gggc2111,
      ggc212,
      child3,
      gc31,
      child4,
    ];

    final tree = NetworkNode.fromUserData(user, users, NetworkTypeFilter.internet);

    expect(
      tree,
      NetworkNode(
        user: user,
        children: {
          toNetworkNode(child1, parent: user, children: {
            toNetworkNode(gc11, parent: child1, children: {
              toNetworkNode(ggc111, parent: gc11, children: {
                toNetworkNode(gggc1111, parent: ggc111),
                toNetworkNode(gggc1112, parent: ggc111),
              }),
            }),
            toNetworkNode(gc12, parent: child1),
            toNetworkNode(gc13, parent: child1),
          }),
          toNetworkNode(child2, parent: user, children: {
            toNetworkNode(gc21, parent: child2, children: {
              toNetworkNode(ggc212, parent: gc21),
            }),
          }),
          toNetworkNode(child3, parent: user, children: {
            toNetworkNode(gc31, parent: child3),
          }),
          toNetworkNode(child4, parent: user),
        },
      ),
    );
  });

  test('Node with cyclic reference drops worst connection', () async {
    var user = generateUser('user');
    var child1 = generateUser('child1',
        connections: const {ConnectionType.internet: ConnectionInfo(ping: 110, hopCount: 1)});
    var grandchild11 = generateUser('grandchild11', connections: const {
      ConnectionType.internet: ConnectionInfo(ping: 111, hopCount: 2),
      ConnectionType.lan: ConnectionInfo(ping: 10, hopCount: 1),
    });
    final users = [child1, grandchild11];

    final tree = NetworkNode.fromUserData(user, users, NetworkTypeFilter.all);

    expect(
      tree,
      NetworkNode(
        user: user,
        children: {
          toNetworkNode(child1, parent: user),
          toNetworkNode(grandchild11, parent: user),
        },
      ),
    );
  });

  test('Node with cyclic reference drops worst connection 2', () async {
    var user = generateUser('a');

    var peer1 = generateUser('t v', connections: const {
      ConnectionType.ble: ConnectionInfo(ping: 3, hopCount: 2),
      ConnectionType.lan: ConnectionInfo(ping: 218, hopCount: 1),
      ConnectionType.internet: ConnectionInfo(ping: 115, hopCount: 2),
    });
    var peer2 = generateUser('b', connections: const {
      ConnectionType.ble: ConnectionInfo(ping: 0, hopCount: 1),
      ConnectionType.lan: ConnectionInfo(ping: 19, hopCount: 1),
      ConnectionType.internet: ConnectionInfo(ping: 50, hopCount: 1),
    });
    var peer3 = generateUser('c', connections: const {
      ConnectionType.ble: ConnectionInfo(ping: 312, hopCount: 4),
      ConnectionType.lan: ConnectionInfo(ping: 527, hopCount: 3),
    });
    var peer4 = generateUser('d', connections: const {
      ConnectionType.ble: ConnectionInfo(ping: 44, hopCount: 3),
      ConnectionType.lan: ConnectionInfo(ping: 259, hopCount: 2),
    });
    final users = [
      peer1,
      peer2,
      peer3,
      peer4,
    ];

    final tree = NetworkNode.fromUserData(user, users, NetworkTypeFilter.all);

    expect(
      tree,
      NetworkNode(
        user: user,
        children: {
          toNetworkNode(peer2, parent: user, children: {
            toNetworkNode(peer1, parent: peer2, children: {
              toNetworkNode(peer4, parent: peer1, children: {
                toNetworkNode(peer3, parent: peer4),
              }),
            }),
          }),
          toNetworkNode(peer1, parent: user, children: {
            toNetworkNode(peer4, parent: peer1, children: {
              toNetworkNode(peer3, parent: peer4),
            }),
          }),
        },
      ),
    );
  });
}
