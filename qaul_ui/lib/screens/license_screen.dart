import 'package:flutter/material.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:flutter_markdown/flutter_markdown.dart';

import '../widgets/widgets.dart';

class LicenseScreen extends StatelessWidget {
  const LicenseScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    var bundle = DefaultAssetBundle.of(context);

    return ResponsiveScaffold(
      icon: Icons.info_outline_rounded,
      title: l10n.agplLicense,
      body: FutureBuilder<String>(
          future: bundle.loadString('assets/license/agpl-3.0.md'),
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
