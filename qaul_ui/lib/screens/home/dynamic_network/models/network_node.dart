part of '../dynamic_network_screen.dart';

@immutable
class NetworkNode {
  const NetworkNode({
    required this.name,
    this.children,
    this.color = Colors.blueAccent,
  }) : assert(name.length == 2, 'Name should be user\'s initials');

  final String name;
  final Color color;
  final Set<NetworkNode>? children;
}

/// Debug only
const root = NetworkNode(
  name: 'AA',
  color: Colors.blue,
  children: {
    NetworkNode(
      name: 'BB',
      color: Colors.teal,
      children: {
        NetworkNode(name: 'CC', color: Colors.deepPurple, children: {
          NetworkNode(name: 'CC', color: Colors.deepPurple, children: {
            NetworkNode(name: 'CC', color: Colors.deepPurple, children: {
              NetworkNode(
                name: 'CC',
                color: Colors.deepPurple,
              ),
            }),
          }),
        }),
      },
    ),
    NetworkNode(name: 'DD', color: Colors.deepOrangeAccent),
    NetworkNode(
      name: 'BB',
      color: Colors.teal,
      children: {
        NetworkNode(name: 'CC', color: Colors.deepPurple),
      },
    ),
    NetworkNode(
      name: 'DD',
      color: Colors.orange,
      children: {
        NetworkNode(name: 'EE', color: Colors.redAccent),
      },
    ),
    NetworkNode(
      name: 'FF',
      color: Colors.orange,
      children: {
        NetworkNode(name: 'GG', color: Colors.redAccent),
        NetworkNode(name: 'HH', color: Colors.pinkAccent),
      },
    ),
  },
);

const root1 = NetworkNode(
  name: 'AA',
  color: Colors.pink,
);

const root2 = NetworkNode(
  name: 'AA',
  color: Colors.teal,
);
