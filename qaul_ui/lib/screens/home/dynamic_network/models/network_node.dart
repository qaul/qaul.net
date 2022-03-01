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

  Uint8List get id => user.id;

  factory NetworkNode.fromUserData(User defaultUser, List<User> users, ConnectionType filter) {
    var root = NetworkNode(user: defaultUser, children: const {});
    if (users.isEmpty) return root;

    bool hasValidConnectionData(User u) =>
        (u.availableTypes?.containsKey(filter) ?? false) &&
        u.availableTypes![filter]!.hopCount != null &&
        u.availableTypes![filter]!.ping != null;
    final filtered = users.where(hasValidConnectionData).toList();

    final unstructured = _prepareUnstructuredNetworkNodes(defaultUser, filtered, filter);
    return _buildNetworkNodeListRecursively(root, allNodes: unstructured);
  }

  static List<NetworkNode> _prepareUnstructuredNetworkNodes(
      User defaultUser, List<User> users, ConnectionType filter) {
    final immediateChildren =
        users.where((element) => element.availableTypes![filter]!.hopCount! == 1);
    final out =
        immediateChildren.map((e) => NetworkNode(user: e, parentId: defaultUser.id)).toList();
    if (out.isEmpty) return out;

    final remainingUsers = [...users]
      ..removeWhere((element) => immediateChildren.contains(element));
    var hops = remainingUsers.map((e) => e.availableTypes![filter]!.hopCount!);
    if (hops.isEmpty) return out;

    final maxHops = hops.reduce(max);
    for (var hops = 2; hops <= maxHops; hops++) {
      hopCountMatchesHops(e) => e.availableTypes?[filter]?.hopCount == hops;
      final usersWithNHops = remainingUsers.where(hopCountMatchesHops).toList();
      if (usersWithNHops.isEmpty) continue;

      remainingUsers.removeWhere(hopCountMatchesHops);

      final possibleParents = users.where((e) => e.availableTypes![filter]!.hopCount! == hops - 1);
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

  static NetworkNode _buildNetworkNodeListRecursively(NetworkNode node,
      {List<NetworkNode>? allNodes}) {
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

  @override
  List<Object?> get props => [user, parentId, children];

  @override
  String toString() => 'NetworkNode(${user.name}, children: $children)';
}
