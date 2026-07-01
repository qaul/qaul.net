import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:hooks_riverpod/legacy.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:qaul_rpc/qaul_rpc.dart';

/// Set before navigating to splash after an explicit user sign-out so the
/// landing screen is not immediately redirected back to home while the daemon
/// session is still winding down (common on iOS where RPC can lag).
final forceSignedOutProvider = StateProvider<bool>((ref) => false);

final accountSessionProvider = FutureProvider<QaulAccountSessionState>((
  ref,
) async {
  if (ref.read(forceSignedOutProvider)) {
    ref.read(forceSignedOutProvider.notifier).state = false;
    return QaulAccountSessionState.signedOut;
  }

  final worker = ref.read(qaulWorkerProvider);
  final user = await worker.getDefaultUserAccount();
  if (user == null) return QaulAccountSessionState.noLocalAccount;

  final authenticated = await worker.getSessionStatus(userId: user.id);
  return authenticated
      ? QaulAccountSessionState.signedIn
      : QaulAccountSessionState.signedOut;
});
