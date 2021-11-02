import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/src/models/feed_message.dart';
import 'package:qaul_rpc/src/models/user.dart';

import '../qaul_rpc.dart';

// TODO(brenodt): Hide from outside of package. No need to expose this low-level class.
final libqaulProvider = Provider<Libqaul>((ref) => Libqaul(ref.read));

final userAccountsModuleProvider =
    Provider<RpcUserAccounts>((ref) => RpcUserAccounts(ref.read));

final defaultUserProvider = StateProvider<User?>((ref) => null);

final feedMessagesProvider =
    StateNotifierProvider<FeedMessages, List<FeedMessage>>(
        (ref) => FeedMessages(messages: const [
              FeedMessage(content: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nam a mauris nec mi commodo euismod id nec nunc. Aliquam quis orci vel magna convallis vestibulum. Sed sodales malesuada libero, sed euismod erat commodo vitae.', timeSent: '2021-09-20 00:00:00.000'),
              FeedMessage(content: 'Interdum et malesuada fames ac ante ipsum primis in faucibus. Suspendisse fermentum gravida risus, pellentesque rhoncus ligula cursus in.', timeSent: '2021-10-15 00:00:00.000'),
              FeedMessage(content: 'Curabitur quis pharetra mauris. Sed et mi sit amet felis sollicitudin convallis. Ut scelerisque risus vulputate est maximus ornare. Morbi turpis velit, efficitur sit amet sodales non, semper at mauris. Integer rhoncus vulputate pellentesque. Sed rhoncus nulla erat, vel eleifend tortor faucibus in.', timeSent: '2021-11-01 00:00:00.000'),
              FeedMessage(content: 'Sed porta et ligula a euismod. Quisque sit amet diam elit. Aliquam vulputate dolor elit, blandit pellentesque nibh venenatis quis. Nullam laoreet feugiat orci, at laoreet felis convallis a. Nullam semper, enim non hendrerit sollicitudin, turpis est sollicitudin mi, sit amet ultrices sapien justo ac diam. Proin vitae tempus metus. Nullam sagittis nulla ut turpis sagittis placerat. Morbi a sodales ex. Nam consectetur nunc in aliquet hendrerit.', timeSent: '2021-11-01 00:00:00.000'),
            ]));
