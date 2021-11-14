import 'package:flutter/cupertino.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_ui/decorators/qaul_nav_bar_decorator.dart';
import 'package:qaul_ui/providers/providers.dart';
import 'package:qaul_ui/screens/home/tabs/tab.dart';
import 'package:qaul_ui/screens/home/user_account_screen.dart';

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

    return Scaffold(
      body: QaulNavBarDecorator(
        child: PageView(
          physics: const NeverScrollableScrollPhysics(),
          controller: pageCtrl,
          children: [
            const UserAccountScreen(),
            BaseTab.feed(),
            BaseTab.users(),
            BaseTab.chat(),
            BaseTab.network(),
          ],
        ),
      ),
    );
  }

  Duration get _pageTransitionDuration => const Duration(milliseconds: 230);
}
