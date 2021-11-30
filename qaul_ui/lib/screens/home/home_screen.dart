import 'package:flutter/cupertino.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_ui/decorators/qaul_nav_bar_decorator.dart';
import 'package:qaul_ui/providers/providers.dart';
import 'package:qaul_ui/screens/home/tabs/tab.dart';
import 'package:qaul_ui/screens/home/user_account_screen.dart';

class SwitchTabIntent extends Intent {
  const SwitchTabIntent._(this.switchForward);

  final bool switchForward;

  factory SwitchTabIntent.forward() => const SwitchTabIntent._(true);

  factory SwitchTabIntent.backward() => const SwitchTabIntent._(false);
}

class HomeScreen extends HookConsumerWidget {
  const HomeScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final tabCtrl = ref.watch(selectedTabProvider);
    final pageCtrl = usePageController(initialPage: tabCtrl.initialTab);

    useEffect(
      () {
        final subscription = tabCtrl.stream.listen(
          (i) => pageCtrl.animateToPage(i,
              duration: _pageTransitionDuration, curve: Curves.decelerate),
        );
        return subscription.cancel;
      },
      [tabCtrl.index],
    );

    final switchForward = useCallback(() => tabCtrl.goToNext(), [UniqueKey()]);
    final switchBack = useCallback(() => tabCtrl.goToPrevious(), [UniqueKey()]);

    return Scaffold(
      body: Shortcuts(
        shortcuts: {
          LogicalKeySet(LogicalKeyboardKey.tab): SwitchTabIntent.forward(),
          LogicalKeySet(LogicalKeyboardKey.shift, LogicalKeyboardKey.tab):
              SwitchTabIntent.backward(),
        },
        child: Actions(
          actions: {
            SwitchTabIntent: CallbackAction<SwitchTabIntent>(
              onInvoke: (intent) =>
                  intent.switchForward ? switchForward() : switchBack(),
            ),
          },
          child: QaulNavBarDecorator(
            child: PageView(
              controller: pageCtrl,
              allowImplicitScrolling: true,
              physics: const NeverScrollableScrollPhysics(),
              children: [
                const UserAccountScreen(),
                BaseTab.feed(),
                BaseTab.users(),
                BaseTab.chat(),
                BaseTab.network(),
              ],
            ),
          ),
        ),
      ),
    );
  }

  Duration get _pageTransitionDuration => const Duration(milliseconds: 230);
}
