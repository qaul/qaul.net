import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_ui/decorators/qaul_nav_bar_decorator.dart';
import 'package:qaul_ui/providers/providers.dart';

class HomeScreen extends HookConsumerWidget {
  const HomeScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final tabCtrl = ref.watch(selectedTabProvider);
    final pageCtrl = usePageController();

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
          controller: pageCtrl,
          children: [
            Center(child: Text('home 😀')),
            Center(child: Text('home 😁')),
            Center(child: Text('home 😆')),
            Center(child: Text('home 🤩')),
            Center(child: Text('home 🥸')),
          ],
        ),
      ),
    );
  }

  Duration get _pageTransitionDuration => const Duration(milliseconds: 230);
}
