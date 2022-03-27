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
  const HomeScreen({Key? key}) : super(key: key);

  bool get _isMobile => Platform.isIOS || Platform.isAndroid;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final tabCtrl = ref.read(homeScreenControllerProvider.notifier);

    final switchToFeed = useCallback(() => tabCtrl.goToTab(TabType.feed), []);

    return WillPopScope(
      onWillPop: () async {
        if (Platform.isAndroid) switchToFeed();
        return false;
      },
      child: Scaffold(
        body: QaulNavBarDecorator(
          child: PageView(
            controller: tabCtrl.pageController,
            allowImplicitScrolling: true,
            physics: _isMobile ? const PageScrollPhysics() : const NeverScrollableScrollPhysics(),
            children: [
              const UserAccountScreen(),
              BaseTab.feed(),
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
