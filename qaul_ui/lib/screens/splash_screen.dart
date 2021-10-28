import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:qaul_ui/helpers/navigation_helper.dart';

class SplashScreen extends ConsumerWidget {
  SplashScreen({Key? key}) : super(key: key);

  final _sendRequestProvider = FutureProvider<String>((ref) async {
    final rpcUserAccounts = RpcUserAccounts(ref.read);
    await rpcUserAccounts.getDefaultUserAccount();

    await Future.delayed(const Duration(seconds: 5));
    final user = ref.read(defaultUserProvider).state;
    final route = user == null
        ? NavigationHelper.createAccount
        : NavigationHelper.home;
    return route;
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final defaultUser = ref.watch(_sendRequestProvider);

    Widget child = const SizedBox.shrink();
    defaultUser.when(
      error: (_, s, sync) => child = const Scaffold(
        body: Center(child: Text('An Error Occurred.')),
      ),
      loading: (_) => child = const Scaffold(
        body: Center(child: CircularProgressIndicator()),
      ),
      // TODO(brenodt): Must not be called from within the build method.
      data: (route) => Navigator.pushNamed(context, route),
    );

    return child;
  }
}
