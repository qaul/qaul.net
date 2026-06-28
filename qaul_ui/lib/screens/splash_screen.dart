import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:url_launcher/url_launcher.dart';

import '../coordinators/account_management_coordinator.dart';
import '../helpers/navigation_helper.dart';
import '../providers/account_session_provider.dart';
import '../widgets/widgets.dart';

class SplashScreen extends ConsumerWidget {
  const SplashScreen() : super(key: widgetKey);

  static const widgetKey = ValueKey('SplashScreen');
  static const createUserButtonKey = ValueKey('createUserAccountButton');

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    ref.listen(accountSessionProvider, (_, snapshot) {
      if (snapshot.value != QaulAccountSessionState.signedIn) return;
      WidgetsBinding.instance.addPostFrameCallback((_) {
        if (!context.mounted) return;
        Navigator.pushReplacementNamed(context, NavigationHelper.home);
      });
    });

    final session = ref.watch(accountSessionProvider);

    return session.when(
      data: (state) => QaulAccountLanding(
        state: state,
        logo: const QaulAccountLogo(),
        languageSelector: const LanguageSelectDropDown(),
        onCreateAccount: () => Navigator.pushReplacementNamed(
          context,
          NavigationHelper.createAccount,
        ),
        onRestoreAccount: () =>
            AccountManagementCoordinator.showRestoreFlow(context, ref),
        onLogin: () => AccountManagementCoordinator.showLoginFlow(context, ref),
        onLearnMore: () =>
            launchUrl(Uri.parse('https://qaul.net/tutorials/onboarding/')),
      ),
      error: (error, stackTrace) => QaulAccountLanding(
        state: QaulAccountSessionState.noLocalAccount,
        logo: const QaulAccountLogo(),
        languageSelector: const LanguageSelectDropDown(),
        onCreateAccount: () => Navigator.pushReplacementNamed(
          context,
          NavigationHelper.createAccount,
        ),
        onRestoreAccount: () =>
            AccountManagementCoordinator.showRestoreFlow(context, ref),
        onLearnMore: () =>
            launchUrl(Uri.parse('https://qaul.net/tutorials/onboarding/')),
      ),
      loading: () => const Scaffold(
        body: Center(
          child: SizedBox(width: 320, height: 320, child: QaulAccountLogo()),
        ),
      ),
    );
  }
}
