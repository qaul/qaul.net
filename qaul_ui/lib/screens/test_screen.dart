import 'package:flutter/material.dart';
import 'package:local_notifications/local_notifications.dart';

import '../widgets/widgets.dart';

class TestScreen extends StatelessWidget {
  const TestScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        leading: const DefaultBackButton(),
        title: Row(
          children: const [
            Icon(Icons.code),
            SizedBox(width: 8),
            Text('Notification Test'),
          ],
        ),
      ),
      body: Center(
        child: TextButton(
          onPressed: () {
            const message = LocalNotification(
                id: 0, title: 'title', body: 'body', payload: 'payload');
            LocalNotifications.instance.displayNotification(message);
          },
          child: const Text('Show Notification'),
        ),
      ),
    );
  }
}
