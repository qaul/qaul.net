import 'package:flutter/material.dart';
import 'package:font_awesome_flutter/font_awesome_flutter.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

import '../coordinators/email_logging_coordinator/email_logging_coordinator.dart';
import '../decorators/disabled_state_decorator.dart';
import '../widgets/widgets.dart';

class SupportScreen extends StatefulHookConsumerWidget {
  const SupportScreen({Key? key}) : super(key: key);

  @override
  ConsumerState<SupportScreen> createState() => _SupportScreenState();
}

class _SupportScreenState extends ConsumerState<SupportScreen> {
  EmailLoggingCoordinator get emailLogger => EmailLoggingCoordinator.instance;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        leading: const IconButtonFactory(),
        title: Row(
          children: const [
            FaIcon(FontAwesomeIcons.headset),
            SizedBox(width: 8),
            Text('Support'),
          ],
        ),
      ),
      body: FutureBuilder<bool>(
          future: emailLogger.hasLogsStored(reader: ref.read),
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
                              value: emailLogger.loggingEnabled,
                              onChanged: (val) {
                                emailLogger.setLoggingEnabled(val, reader: ref.read);
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
                                future: emailLogger.logStorageSize,
                                builder: (context, snapshot) {
                                  final size = snapshot.data ?? '0.0 KB';
                                  return Text('Total logs size: $size');
                                },
                              ),
                              TextButton(
                                child: const Text('Delete Logs'),
                                onPressed: () async {
                                  await emailLogger.deleteLogs();
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
                    isDisabled: !emailLogger.loggingEnabled,
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
                                  await emailLogger.sendLogs(reader: ref.read);
                                  await emailLogger.deleteLogs();
                                  Navigator.pop(context);
                                }
                              : null,
                        ),
                        const SizedBox(height: 20, width: double.maxFinite),
                      ],
                    ),
                  ),
                ),
                TextButton(onPressed: () => throw FlutterError('test'), child: const Text('Throw')),
                const Expanded(child: SizedBox()),
              ],
            );
          }),
    );
  }
}
