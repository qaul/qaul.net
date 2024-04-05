import 'package:flutter/material.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
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
    final l10n = AppLocalizations.of(context)!;
    return ResponsiveScaffold(
      icon: FontAwesomeIcons.headset,
      title: l10n.support,
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
                            Text(l10n.enableLogging),
                            PlatformAwareSwitch(
                              value: emailLogger.loggingEnabled,
                              onChanged: (val) {
                                emailLogger.setLoggingEnabled(val,
                                    reader: ref.read);
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
                              Expanded(
                                child: FutureBuilder(
                                  future: emailLogger.logStorageSize,
                                  builder: (context, snapshot) {
                                    final size = snapshot.data ?? '0.0 KB';
                                    return Text(
                                      '${l10n.totalLogsSize} $size',
                                      maxLines: 2,
                                      overflow: TextOverflow.ellipsis,
                                    );
                                  },
                                ),
                              ),
                              TextButton(
                                child: Text(l10n.deleteLogs),
                                onPressed: () async {
                                  await emailLogger.deleteLogs();
                                  if (!context.mounted) return;
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
                    child: Padding(
                      padding: const EdgeInsets.all(20),
                      child: Column(
                        mainAxisAlignment: MainAxisAlignment.center,
                        crossAxisAlignment: CrossAxisAlignment.center,
                        children: [
                          Text(l10n.logsDescription1),
                          const SizedBox(height: 8),
                          Text(l10n.logsDescription2),
                          const SizedBox(height: 20),
                          TextButton(
                            onPressed: hasLogs
                                ? () async {
                                    await emailLogger.sendLogs(
                                        reader: ref.read);
                                    await emailLogger.deleteLogs();
                                    if (!context.mounted) return;
                                    Navigator.pop(context);
                                  }
                                : null,
                            child: Text(
                              hasLogs ? l10n.sendLogs : l10n.noLogsAvailable,
                            ),
                          ),
                          const SizedBox(height: 20, width: double.maxFinite),
                        ],
                      ),
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
