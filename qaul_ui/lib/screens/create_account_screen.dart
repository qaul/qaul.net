import 'package:flutter/material.dart';
import 'package:qaul_ui/helpers/navigation_helper.dart';

class CreateAccountScreen extends StatelessWidget {
  const CreateAccountScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    // TODO: Update placeholder
    return Scaffold(
        body: Column(
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        const Center(child: Text('Create account')),
        ElevatedButton(
          onPressed: () =>
              Navigator.pushReplacementNamed(context, NavigationHelper.home),
          child: const Text('Go to home'),
        ),
      ],
    ));
  }
}
