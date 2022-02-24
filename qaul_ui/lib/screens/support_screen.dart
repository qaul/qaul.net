import 'package:flutter/material.dart';
import 'package:logger/logger.dart';

import '../decorators/disabled_state_decorator.dart';
import '../widgets/widgets.dart';

class SupportScreen extends StatefulWidget {
  const SupportScreen({Key? key}) : super(key: key);

  @override
  State<SupportScreen> createState() => _SupportScreenState();
}

class _SupportScreenState extends State<SupportScreen> {
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
              children: [
                Expanded(
                  child: Padding(
                    padding: const EdgeInsets.all(16),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Row(
                          crossAxisAlignment: CrossAxisAlignment.center,
                          mainAxisAlignment: MainAxisAlignment.spaceBetween,
                          children: [
                            const Text('Enable Logging:'),
                            PlatformAwareSwitch(
                              value: Logger.instance.loggingEnabled,
                              onChanged: (val) {
                                Logger.instance.loggingEnabled = val;
                                setState(() {});
                              },
                            ),
                          ],
                        ),
                        const Divider(),
                        if (hasLogs) ...[
                          Row(
                            mainAxisAlignment: MainAxisAlignment.spaceBetween,
                            children: [
                              FutureBuilder(
                                future: Logger.instance.logStorageSize,
                                builder: (context, snapshot) {
                                  final size = snapshot.data ?? '0.0 KB';
                                  return Text('Total logs size: $size');
                                },
                              ),
                              TextButton(
                                child: const Text('Delete Logs'),
                                onPressed: () async {
                                  await Logger.instance.deleteLogs();
                                  Navigator.pop(context);
                                },
                              ),
                            ],
                          ),
                        ],
                      ],
                    ),
                  ),
                ),
                Expanded(
                  child: DisabledStateDecorator(
                    isDisabled: !Logger.instance.loggingEnabled,
                    child: Column(
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
                      ],
                    ),
                  ),
                ),
                const Expanded(child: SizedBox()),
              ],
            );
          }),
    );
  }
}
