import 'package:flutter/material.dart';
import 'package:logger/logger.dart';
import 'package:qaul_ui/widgets/default_back_button.dart';

class SupportScreen extends StatelessWidget {
  const SupportScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        leading: const DefaultBackButton(),
        title: Row(
          children: const [
            Icon(Icons.contact_support_outlined),
            SizedBox(width: 8),
            Text('Support'),
          ],
        ),
      ),
      body: FutureBuilder<bool>(
          future: Logger.instance.hasLogsStored,
          builder: (context, snapshot) {
            final hasLogs = (snapshot.hasData && snapshot.data == true);
            return Column(
              mainAxisAlignment: MainAxisAlignment.center,
              crossAxisAlignment: CrossAxisAlignment.center,
              children: [
                const Text('Whenever an error occurs, a log is created.'),
                const SizedBox(height: 8, width: double.maxFinite),
                const Text('You can choose to report them or delete them.'),
                const SizedBox(height: 20, width: double.maxFinite),
                TextButton(
                  child: Text(hasLogs ? 'Send Logs' : 'No Logs Available'),
                  onPressed: hasLogs
                      ? () async {
                          await Logger.instance.sendLogs();
                          await Logger.instance.deleteLogs();
                          Navigator.pop(context);
                        }
                      : null,
                ),
                const SizedBox(height: 20, width: double.maxFinite),
                if (hasLogs)
                  TextButton(
                    child: const Text('Delete Logs'),
                    onPressed: () async {
                      await Logger.instance.deleteLogs();
                      Navigator.pop(context);
                    },
                  ),
                const SizedBox(height: 20, width: double.maxFinite),
                const Divider(),
                const Text("<Testing Only>"),
                TextButton(
                  child:
                      const Text('Throw Error (Generate Log - to see it close and reopen screen).'),
                  onPressed: () => throw FlutterError('Test error'),
                ),
              ],
            );
          }),
    );
  }
}
