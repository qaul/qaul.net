import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

import '../../decorators/qaul_nav_bar_decorator.dart';
import '../../providers/providers.dart';
import 'dynamic_network/dynamic_network_screen.dart';
import 'tabs/tab.dart';
import 'user_account_screen.dart';

class HomeScreen extends HookConsumerWidget {
  const HomeScreen() : super(key: widgetKey);

  static const widgetKey = ValueKey('HomeScreen');

  bool get _isMobile => Platform.isIOS || Platform.isAndroid;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final tabCtrl = ref.read(homeScreenControllerProvider.notifier);

    final gotoToPubTab = useCallback(() => tabCtrl.goToTab(TabType.public), []);

    final disablePageViewScroll = useState(false);

    return PopScope(
      onPopInvokedWithResult: (_, _) async {
        if (Platform.isAndroid) gotoToPubTab();
      },
      child: Scaffold(
        body: QaulNavBarDecorator(
          child: (key) => PageView(
            key: key,
            controller: tabCtrl.pageController,
            allowImplicitScrolling: true,
            physics: !disablePageViewScroll.value && _isMobile
                ? const PageScrollPhysics()
                : const NeverScrollableScrollPhysics(),
            children: [
              const UserAccountScreen(),
              BaseTab.public(disablePageViewScroll: disablePageViewScroll),
              BaseTab.users(),
              BaseTab.chat(),
              const DynamicNetworkScreen(),
              // BaseTab.network(),
            ],
          ),
        ),
      ),
    );
  }
}
