import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_ui/decorators/qaul_nav_bar_decorator.dart';
import 'package:qaul_ui/providers/providers.dart';

class HomeScreen extends ConsumerWidget {
  const HomeScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final tabController = ref.watch(tabControllerProvider);

    return Scaffold(
      body: QaulNavBarDecorator(
        child: Center(child: Text('home ${tabController.currentIndex}')),
      ),
    );
  }
}
