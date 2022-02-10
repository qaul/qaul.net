import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:qaul_ui/helpers/navigation_helper.dart';
import 'package:qaul_ui/widgets/loading_indicator.dart';

class SplashScreen extends ConsumerWidget {
  SplashScreen({Key? key}) : super(key: key);

  final _sendRequestProvider = FutureProvider<String>((ref) async {
    final worker = ref.read(qaulWorkerProvider);

    for (var i = 0; i < 5; i++) {
      await worker.getDefaultUserAccount();
      await Future.delayed(Duration(milliseconds: (i + 1) * 100));
      // final user = ref.read(defaultUserProvider);
      return NavigationHelper.home;
    }
    return NavigationHelper.createAccount;
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    ref.listen(_sendRequestProvider, (AsyncValue<String>? old, AsyncValue<String> snapshot) {
      final value = snapshot.value;
      if (value != null) Navigator.pushReplacementNamed(context, value);
    });

    return const Scaffold(body: LoadingIndicator());
  }
}
