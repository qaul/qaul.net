part of '../dynamic_network_screen.dart';

@immutable
class NetworkNode extends Equatable {
  const NetworkNode({
    required this.user,
    this.parentId,
    this.children,
  });

  final User user;
  final Uint8List? parentId;
  final Set<NetworkNode>? children;

  @override
  List<Object?> get props => [user, children];

  @override
  String toString() => 'NetworkNode(${user.name}, children: $children)';

  Uint8List get id => user.id;

  factory NetworkNode.fromUserData(
    User root,
    List<User> users,
    NetworkTypeFilter filter,
  ) {
    var rootNode = NetworkNode(user: root, children: const {});
    if (users.isEmpty) return rootNode;

    final options = filter != NetworkTypeFilter.all
        ? [filter]
        : [
            NetworkTypeFilter.lan,
            NetworkTypeFilter.bluetooth,
            NetworkTypeFilter.internet,
          ];

    Set<NetworkNode> nodes = {};
    for (final opt in options) {
      final a = _assignParentToChildNodes(root, users, [opt]);
      nodes = nodes.union(a);

    }

    return _buildNetworkNodeListRecursively(rootNode, allNodes: nodes.toList());
  }

  static Set<NetworkNode> _assignParentToChildNodes(
    User root,
    List<User> users,
    List<NetworkTypeFilter> options,
  ) {
    bool hasValidConnectionData(User u, ConnectionType type) =>
        (u.availableTypes?.containsKey(type) ?? false) &&
        u.availableTypes![type]!.hopCount != null &&
        u.availableTypes![type]!.ping != null;

    final unstructured = <NetworkNode>{};
    for (final type in options) {
      final flt = _mapFilter(type);
      final fltrd = users.where((u) => hasValidConnectionData(u, flt)).toList();
      unstructured.addAll(_prepareUnstructuredNetworkNodes(root, fltrd, flt));
    }
    return unstructured;
  }

  static List<NetworkNode> _prepareUnstructuredNetworkNodes(
    User root,
    List<User> users,
    ConnectionType filter,
  ) {
    final immediateChildren = users
        .where((element) => element.availableTypes![filter]!.hopCount! == 1);
    final out = immediateChildren
        .map((e) => NetworkNode(user: e, parentId: root.id))
        .toList();
    if (out.isEmpty) return out;

    final remainingUsers = [...users]
      ..removeWhere((element) => immediateChildren.contains(element));
    var hops = remainingUsers.map((e) => e.availableTypes![filter]!.hopCount!);
    if (hops.isEmpty) return out;

    final maxHops = hops.reduce(math.max);
    for (var hops = 2; hops <= maxHops; hops++) {
      hopCountMatchesHops(e) => e.availableTypes?[filter]?.hopCount == hops;
      final usersWithNHops = remainingUsers.where(hopCountMatchesHops).toList();
      if (usersWithNHops.isEmpty) continue;

      remainingUsers.removeWhere(hopCountMatchesHops);

      final possibleParents =
          users.where((e) => e.availableTypes![filter]!.hopCount! == hops - 1);
      for (final user in usersWithNHops) {
        num smallestDistance = double.infinity;
        User? mostProbableParent;

        for (final parent in possibleParents) {
          final parentRtt = parent.availableTypes![filter]!.ping!;
          var diff = (user.availableTypes![filter]!.ping! - parentRtt).abs();

          if (diff <= smallestDistance) {
            smallestDistance = diff;
            mostProbableParent = parent;
          }
        }

        if (mostProbableParent != null) {
          out.add(NetworkNode(user: user, parentId: mostProbableParent.id));
        }
      }
    }

    return out;
  }

  static NetworkNode _buildNetworkNodeListRecursively(
    NetworkNode node, {
    List<NetworkNode>? allNodes,
  }) {
    return NetworkNode(
      user: node.user,
      parentId: node.parentId,
      children: allNodes
          ?.where((element) => node.user.id == element.parentId)
          .map((e) => _buildNetworkNodeListRecursively(
              NetworkNode(user: e.user, parentId: e.parentId),
              allNodes: allNodes))
          .toSet(),
    );
  }
}
