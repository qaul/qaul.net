import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:utils/utils.dart';
import 'package:qaul_ui/screens/home/dynamic_network/dynamic_network_screen.dart';

import '../../decorators/qaul_nav_bar_decorator.dart';
import '../../providers/providers.dart';
import 'tabs/tab.dart';
import 'user_account_screen.dart';

class SwitchTabIntent extends Intent {
  const SwitchTabIntent._(this.switchForward);

  final bool switchForward;

  factory SwitchTabIntent.forward() => const SwitchTabIntent._(true);

  factory SwitchTabIntent.backward() => const SwitchTabIntent._(false);
}

class HomeScreen extends HookConsumerWidget {
  HomeScreen({Key? key}) : super(key: key);

  final _animatingTimer = LoopTimer(_transitionDuration);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final tabCtrl = ref.watch(selectedTabProvider);
    final pageCtrl = usePageController(initialPage: tabCtrl.initialTab);

    final updateCurrentlyVisibleTab = useCallback(() {
      if (pageCtrl.page == null) return;
      tabCtrl.updateCurrentIndexWithoutScrolling(pageCtrl.page!.round());
      _animatingTimer.cancel();
    }, [const Key('updateCurrentlyVisibleTab')]);

    final onUserSwipeAnimation = useCallback(() {
      if (_animatingTimer.isRunning) return;
      _animatingTimer.onTimeout = updateCurrentlyVisibleTab;
      _animatingTimer.start();
    }, [const Key('onUserSwipeAnimation')]);

    useEffect(() {
      pageCtrl.addListener(onUserSwipeAnimation);
      return () => pageCtrl.removeListener(onUserSwipeAnimation);
    }, [const Key('subscribeToPageCtrl')]);

    useEffect(
      () {
        final subscription = tabCtrl.stream.listen(
          (s) {
            if (!s.shouldScroll) return;
            _animatingTimer.cancel();
            pageCtrl.animateToPage(s.tab, duration: _transitionDuration, curve: Curves.decelerate);
          },
        );
        return subscription.cancel;
      },
      [tabCtrl.index],
    );

    final switchToFeed = useCallback(() => tabCtrl.goToTab(TabType.feed), []);
    final switchForward = useCallback(() => tabCtrl.goToNext(), [UniqueKey()]);
    final switchBack = useCallback(() => tabCtrl.goToPrevious(), [UniqueKey()]);

    return WillPopScope(
      onWillPop: () async {
        if (Platform.isAndroid) switchToFeed();
        return false;
      },
      child: Scaffold(
        body: Shortcuts(
          shortcuts: {
            LogicalKeySet(LogicalKeyboardKey.tab): SwitchTabIntent.forward(),
            LogicalKeySet(LogicalKeyboardKey.shift, LogicalKeyboardKey.tab):
                SwitchTabIntent.backward(),
          },
          child: Actions(
            actions: {
              SwitchTabIntent: CallbackAction<SwitchTabIntent>(
                onInvoke: (intent) => intent.switchForward ? switchForward() : switchBack(),
              ),
            },
            child: QaulNavBarDecorator(
              child: PageView(
                controller: pageCtrl,
                allowImplicitScrolling: true,
                children: [
                  const UserAccountScreen(),
                  BaseTab.feed(),
                  BaseTab.users(),
                  BaseTab.chat(),
                  // BaseTab.network(),
                  const DynamicNetworkView(),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }

  static Duration get _transitionDuration => const Duration(milliseconds: 230);
}
