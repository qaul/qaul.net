import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/qaul_rpc.dart';

import '../helpers/navigation_helper.dart';
import '../widgets/widgets.dart';

class SplashScreen extends ConsumerWidget {
  SplashScreen() : super(key: widgetKey);

  static const widgetKey = ValueKey('SplashScreen');

  final _sendRequestProvider = FutureProvider<String>((ref) async {
    final worker = ref.read(qaulWorkerProvider);

    for (var i = 0; i < 5; i++) {
      await worker.getDefaultUserAccount();
      await Future.delayed(Duration(milliseconds: (i + 1) * 100));
      final user = ref.read(defaultUserProvider);
      if (user != null) return NavigationHelper.home;
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
