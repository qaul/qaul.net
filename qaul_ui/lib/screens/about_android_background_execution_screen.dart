import 'package:flutter/material.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:flutter_markdown/flutter_markdown.dart';

import '../widgets/widgets.dart';

class AboutAndroidBackgroundExecutionScreen extends StatelessWidget {
  const AboutAndroidBackgroundExecutionScreen({Key? key}) : super(key: key);

  final _filename = 'assets/about/android_background_execution.md';

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    var bundle = DefaultAssetBundle.of(context);

    return ResponsiveScaffold(
      icon: Icons.info_outline_rounded,
      title: l10n.backgroundExecution,
      body: FutureBuilder<String>(
          future: bundle.loadString(_filename),
          builder: (context, ss) {
            if (!ss.hasData || ss.connectionState != ConnectionState.done) {
              return const LoadingIndicator();
            }
            return Markdown(
              data: ss.data ?? 'An error occurred',
            );
          }),
    );
  }
}
