import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:qaul_ui/helpers/navigation_helper.dart';

class SplashScreen extends ConsumerWidget {
  SplashScreen({Key? key}) : super(key: key);

  final _sendRequestProvider = FutureProvider<String>((ref) async {
    final rpcUserAccounts = RpcUserAccounts(ref.read);

    await rpcUserAccounts.getDefaultUserAccount();
    await Future.delayed(const Duration(seconds: 7));

    final libqaul = ref.read(libqaulProvider);

    // DEBUG: how many messages are queued by libqaul
    final queued = await libqaul.checkReceiveQueue();
    // check for rpc messages
    if (queued > 0) await libqaul.receiveRpc();


    final user = ref.read(defaultUserProvider).state;
    final route =
        user == null ? NavigationHelper.createAccount : NavigationHelper.home;
    return route;
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    ref.listen(_sendRequestProvider, (AsyncValue<String> snapshot) {
      final value = snapshot.value;
      if (value is String) Navigator.pushReplacementNamed(context, value);
    });

    return const Scaffold(body: Center(child: CircularProgressIndicator()));
  }
}
