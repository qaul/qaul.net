import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/qaul_rpc.dart';

import '../stores/stores.dart';
import 'providers.dart';

void resetSessionScopedState(WidgetRef ref, {User? activeUser}) {
  ref.invalidate(currentOpenChatRoom);
  ref.invalidate(chatRoomsProvider);
  ref.invalidate(groupInvitesProvider);
  ref.invalidate(chatRoomsSearchProvider);
  ref.invalidate(chatRoomsStoreProvider);

  ref.invalidate(feedMessageStoreProvider);

  ref.invalidate(usersSearchProvider);
  ref.invalidate(usersStoreProvider);
  ref.invalidate(userLookupProvider);
  ref.invalidate(defaultUserProvider);

  ref.read(homeScreenControllerProvider.notifier).goToTab(TabType.public);
  ref.invalidate(nodeInfoProvider);
  ref.invalidate(dtnConfigurationProvider);
  ref.invalidate(connectedNodesProvider);
  ref.invalidate(bleStatusProvider);

  if (activeUser == null) return;

  ref.read(defaultUserProvider.notifier).state = activeUser;
  ref.read(usersStoreProvider.notifier).startOnlinePolling();
}
