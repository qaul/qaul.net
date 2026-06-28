import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:qaul_rpc/qaul_rpc.dart';

final accountSessionProvider = FutureProvider<QaulAccountSessionState>((
  ref,
) async {
  final worker = ref.read(qaulWorkerProvider);
  final user = await worker.getDefaultUserAccount();
  if (user == null) return QaulAccountSessionState.noLocalAccount;

  final authenticated = await worker.getSessionStatus(userId: user.id);
  return authenticated
      ? QaulAccountSessionState.signedIn
      : QaulAccountSessionState.signedOut;
});
