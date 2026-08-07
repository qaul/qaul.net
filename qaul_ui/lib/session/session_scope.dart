/// The app runs on a single, long-lived `ProviderContainer` (see `main.dart`)
/// that outlives any individual account session. Signing out therefore does not
/// dispose provider state the way closing the app would: without help, the next
/// account inherits the previous one's chats, users, invites and notification
/// caches.
///
/// `listenForSessionChanges` installs the one place that notices a session
/// boundary. Every provider in the app is classified into exactly one of four
/// buckets, and `test/session_scope_test.dart` fails if a provider is missing
/// from all of them — a new provider cannot be silently forgotten here.
///
/// 1. `sessionKeyProvider` — the account identity the reset is keyed on.
/// 2. `sessionScopedProviders` — dropped whenever the account changes.
/// 3. `signOutScopedProviders` — dropped only once a session has *ended*.
/// 4. `appScopedProviders` — survive, each with a stated reason.
library;

import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:hooks_riverpod/misc.dart' show ProviderOrFamily;
import 'package:qaul_rpc/qaul_rpc.dart';

import '../providers/account_session_provider.dart';
import '../providers/providers.dart';
import '../stores/stores.dart';

/// The provider that decides which account is signed in. A session begins when
/// it gains an account id and ends when it loses one.
///
/// It is not in [sessionScopedProviders] because it is the key those providers
/// are reset against, not a member of the set being reset.
final sessionKeyProvider = qaulRpcSessionKeyProvider;

/// State belonging to exactly one signed-in account.
final sessionScopedProviders = <ProviderOrFamily>[
  ...qaulRpcSessionScopedProviders,

  // Chat
  chatRoomsSearchProvider,
  chatRoomsStoreProvider,

  // Users
  usersSearchProvider,
  usersStoreProvider,

  // Public feed
  feedMessageStoreProvider,

  // Notifications. These hold the previous account's unread caches and capture
  // `defaultUserProvider` in a `late final` at `initialize()` time, so they must
  // be replaced rather than re-initialized.
  chatNotificationControllerProvider,
  publicNotificationControllerProvider,

  // Navigation state, so a new session always lands on the default tab.
  homeScreenControllerProvider,
];

/// Session state that is only safe to discard once a session has fully ended.
///
/// [accountSessionProvider] tells the splash screen whether to auto-navigate to
/// /home. Refreshing it mid-login would race that auto-navigate against the
/// login flow's own push and land /home twice, so it is deliberately left alone
/// on the sign-in edge and refreshed on the sign-out edge, where the splash
/// screen is the destination rather than a bystander.
final signOutScopedProviders = <ProviderOrFamily>[
  accountSessionProvider,
];

/// Providers that deliberately survive a session change, and why.
///
/// Consumed by `test/session_scope_test.dart` as the documented exclusion list.
const appScopedProviders = <String, String>{
  ...qaulRpcAppScopedProviders,
  'forceSignedOutProvider':
      'A one-shot handshake between logout and the splash screen. Resetting it '
          'mid-logout would defeat its purpose.',
};

/// Watches [sessionKeyProvider] and discards the outgoing account's state at
/// every session boundary, whichever flow caused it.
///
/// This is the only place a reset is triggered. Login, account creation,
/// restore, logout and account deletion all publish through
/// [sessionKeyProvider] (see `LibqaulWorker._setActiveUser`), so a new entry
/// point cannot forget to reset — which is the failure mode the reset exists to
/// prevent in the first place.
///
/// [onSessionChanged] runs after the reset, with the incoming account (or null
/// once signed out).
ProviderSubscription<String?> listenForSessionChanges(
  ProviderContainer container, {
  required void Function(String? accountId) onSessionChanged,
}) {
  return container.listen(
    sessionKeyProvider.select((user) => user?.idBase58),
    (previousAccount, account) {
      if (previousAccount == account) return;
      if (account == null) {
        _resetOnSignOut(container);
      } else {
        resetSessionScopedState(container);
      }
      onSessionChanged(account);
    },
    fireImmediately: true,
  );
}

/// Discards every provider in [sessionScopedProviders].
void resetSessionScopedState(ProviderContainer container) {
  for (final provider in sessionScopedProviders) {
    container.invalidate(provider);
  }
}

void _resetOnSignOut(ProviderContainer container) {
  resetSessionScopedState(container);
  for (final provider in signOutScopedProviders) {
    container.invalidate(provider);
  }
}
