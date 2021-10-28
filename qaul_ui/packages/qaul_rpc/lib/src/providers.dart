import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/src/models/user.dart';

import '../qaul_rpc.dart';

// TODO(brenodt): Hide from outside of package. No need to expose this low-level class.
final libqaulProvider = Provider<Libqaul>((ref) => Libqaul(ref.read));

final userAccountsModuleProvider = Provider<RpcUserAccounts>((ref) => RpcUserAccounts(ref.read));

final defaultUserProvider = StateProvider<User?>((ref) => null);