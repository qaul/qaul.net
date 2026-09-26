import 'package:hooks_riverpod/misc.dart' show ProviderOrFamily;

import 'internal/file_history.dart';
import 'models/models.dart';

/// The provider whose value defines *which* account is signed in.
///
/// The app resets session state whenever this changes account id, so it is the
/// one provider that is neither reset with the session nor app-scoped: it is
/// the key the reset is keyed on. Every entry path publishes through
/// `LibqaulWorker._setActiveUser`.
final qaulRpcSessionKeyProvider = defaultUserProvider;

/// Providers owned by this package that hold state for exactly one signed-in
/// account, and must be discarded when the account changes.
///
/// This package owns these providers, so it classifies them; the app composes
/// this list into its own (see `lib/session/session_scope.dart`). Keeping the
/// classification here is what lets `internal/` stay internal.
final qaulRpcSessionScopedProviders = <ProviderOrFamily>[
  // Chat
  chatRoomsProvider,
  currentOpenChatRoom,
  groupInvitesProvider,

  // Users
  userLookupProvider,

  // Files
  fileHistoryEntitiesProvider,

  // Node / connectivity state, reported by the daemon per signed-in account.
  bleStatusProvider,
  connectedNodesProvider,
  dtnConfigurationProvider,
  nodeInfoProvider,
];

/// Providers owned by this package that deliberately survive a session change,
/// and why.
const qaulRpcAppScopedProviders = <String, String>{
  'libqaulProvider':
      'Process-level FFI handle. The embedded node keeps running across '
          'sessions; only the account changes.',
  'qaulWorkerProvider':
      'Owns the RPC connection, its receive-queue poller and heartbeat. '
          'Recreating it would restart the node, not the session.',
  'libqaulLogsStoragePath':
      'Filesystem path resolved once at startup. Not account state.',
};
