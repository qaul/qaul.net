import 'package:flutter/material.dart';
import 'package:local_notifications/local_notifications.dart';

class TestScreen extends StatelessWidget {
  const TestScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Center(
        child: TextButton(
          onPressed: () {
            const message = Message(id: 0, title: 'title', body: 'body', payload: 'payload');
            LocalNotifications.instance.displayNotification(message);
          },
          child: const Text('Show Notification'),
        ),
      ),
    );
  }
}
