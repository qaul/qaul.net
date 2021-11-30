import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/src/models/feed_post.dart';
import 'package:qaul_rpc/src/models/feed_post_list_notifier.dart';
import 'package:qaul_rpc/src/models/user.dart';

import '../qaul_rpc.dart';
import 'models/internet_node.dart';
import 'models/models.dart';

final qaulWorkerProvider = Provider<LibqaulWorker>((ref) => LibqaulWorker(ref.read));

final defaultUserProvider = StateProvider<User?>((ref) => null);

final feedMessagesProvider =
    StateNotifierProvider<FeedPostListNotifier, List<FeedPost>>(
        (ref) => FeedPostListNotifier(messages: const [
              // FeedMessage(content: 'Sed porta et ligula a euismod. Quisque sit amet diam elit. Aliquam vulputate dolor elit, blandit pellentesque nibh venenatis quis. Nullam laoreet feugiat orci, at laoreet felis convallis a. Nullam semper, enim non hendrerit sollicitudin, turpis est sollicitudin mi, sit amet ultrices sapien justo ac diam. Proin vitae tempus metus. Nullam sagittis nulla ut turpis sagittis placerat. Morbi a sodales ex. Nam consectetur nunc in aliquet hendrerit.', timeSent: '2021-11-01 00:00:00.000'),
              // FeedMessage(content: 'Curabitur quis pharetra mauris. Sed et mi sit amet felis sollicitudin convallis. Ut scelerisque risus vulputate est maximus ornare. Morbi turpis velit, efficitur sit amet sodales non, semper at mauris. Integer rhoncus vulputate pellentesque. Sed rhoncus nulla erat, vel eleifend tortor faucibus in.', timeSent: '2021-11-01 00:00:00.000'),
              // FeedMessage(content: 'Interdum et malesuada fames ac ante ipsum primis in faucibus. Suspendisse fermentum gravida risus, pellentesque rhoncus ligula cursus in.', timeSent: '2021-10-15 00:00:00.000'),
              // FeedMessage(content: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nam a mauris nec mi commodo euismod id nec nunc. Aliquam quis orci vel magna convallis vestibulum. Sed sodales malesuada libero, sed euismod erat commodo vitae.', timeSent: '2021-09-20 00:00:00.000'),
              // FeedMessage(content: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nam a mauris nec mi commodo euismod id nec nunc. Aliquam quis orci vel magna convallis vestibulum. Sed sodales malesuada libero, sed euismod erat commodo vitae.', timeSent: '2021-08-20 00:00:00.000'),
            ]));

final usersProvider = StateNotifierProvider<UserListNotifier, List<User>>(
    (ref) => UserListNotifier(users: const [
        // User(name: 'John Doe', idBase58: '12D3KooWEbzJbVGua4EQNKQVUYoA46vcXnfePfm3ZL7C8pGGqddd', status: ConnectionStatus.online),
        // User(name: 'John Doe', idBase58: '12D3KooWEbzJbVGua4EQNKQVUYoA46vcXnfePfm3ZL7C8pGGqDNX', status: ConnectionStatus.offline),
        // User(name: 'John Doe', idBase58: '12D3KooWEbzJbVGua4EQNKQVUYoA46vcXnfePfm3ZL7C8rGGqDNX', status: ConnectionStatus.online),
        // User(name: 'John Doe', idBase58: '12D3KooWEbzJbVGua4EQNKQVUYoA46vcXnfePfm3ZL7C8pGGqDNX', status: ConnectionStatus.reachable),
        // User(name: 'John Doe', idBase58: '12D3KooWEbzJbVGua4EQNKQVUYoA46vcXnfePfm3ZL7C8pGG5DNX', status: ConnectionStatus.offline),
        // User(name: 'John Doe', idBase58: '12D3KooWEbzJbVGua4EQNKQVUYoA46vcXnfePfm3ZL7C8pGGqDNX', status: ConnectionStatus.online),
        // User(name: 'John Doe', idBase58: '12D3KooWEbzJbVGua4EQNKQVUYoA46vcXnfePfm3ZL7C8pGGq8NX', status: ConnectionStatus.reachable),
        // User(name: 'John Doe', idBase58: '12D3KooWEbzJbVGua4EQNKQVUYoA46vcXnfePfm3ZL7C8pGLqDNX', status: ConnectionStatus.reachable),
        // User(name: 'John Doe', idBase58: '12D3KooWEbzJbVGua4EQNKQVUYoA46vcXnfePfm3ZL7C8pGG4DNX', status: ConnectionStatus.online),
      ],
    ),
);

final connectedNodesProvider = StateProvider<List<InternetNode>>((ref) => []);
