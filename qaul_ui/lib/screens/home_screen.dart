import 'package:flutter/material.dart';
import 'package:qaul_ui/decorators/qaul_nav_bar_decorator.dart';

class HomeScreen extends StatelessWidget {
  const HomeScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return const Scaffold(
      body: QaulNavBarDecorator(child: Center(child: Text('home'))),
    );
  }
}
