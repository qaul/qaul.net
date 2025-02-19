import 'package:flutter/material.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:flutter_svg/flutter_svg.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:url_launcher/url_launcher.dart';

import '../helpers/navigation_helper.dart';
import '../widgets/widgets.dart';

class SplashScreen extends HookConsumerWidget {
  SplashScreen() : super(key: widgetKey);

  static const widgetKey = ValueKey('SplashScreen');
  static const createUserButtonKey = ValueKey('createUserAccountButton');
  static const _animationDuration = Duration(milliseconds: 1500);

  final _sendRequestProvider = FutureProvider<String?>((ref) async {
    await Future.delayed(_animationDuration);

    final worker = ref.read(qaulWorkerProvider);

    for (var i = 0; i < 5; i++) {
      await worker.getDefaultUserAccount();
      await Future.delayed(Duration(milliseconds: (i + 1) * 100));
      final user = ref.read(defaultUserProvider);
      if (user != null) return NavigationHelper.home;
    }
    return null;
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    ref.listen(_sendRequestProvider,
        (AsyncValue<String?>? old, AsyncValue<String?> snapshot) {
      final value = snapshot.value;
      if (value != null) Navigator.pushReplacementNamed(context, value);
    });

    final startAnimationPlayed = useState(false);

    useEffect(() {
      const gapBetweenFetchAndRouting = Duration(milliseconds: 500);
      Future.delayed(_animationDuration + gapBetweenFetchAndRouting).then(
        (_) {
          if (!context.mounted) return;
          startAnimationPlayed.value = true;
        },
      );
      return () {};
    }, []);

    final i10n = AppLocalizations.of(context)!;

    return Scaffold(
      body: ListView(
        padding: const EdgeInsets.symmetric(horizontal: 40, vertical: 120),
        children: [
          const SizedBox(width: double.maxFinite),
          SvgPicture.asset('assets/logo/logo.svg', width: 320, height: 320),
          AnimatedOpacity(
            opacity: startAnimationPlayed.value == false ? 0 : 1,
            duration: const Duration(milliseconds: 200),
            child: Column(
              children: [
                const SizedBox(height: 20),
                const LanguageSelectDropDown(),
                const SizedBox(height: 20),
                QaulButton(
                  key: createUserButtonKey,
                  label: i10n.createUserAccount,
                  onPressed: () => Navigator.pushReplacementNamed(
                      context, NavigationHelper.createAccount),
                ),
                const SizedBox(height: 20),
                QaulButton(
                  label: i10n.learnMore,
                  onPressed: () => launchUrl(
                    Uri.parse('https://qaul.net/tutorials/onboarding/'),
                  ),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}
