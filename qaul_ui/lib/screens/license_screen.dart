import 'package:flutter/material.dart';
import 'package:flutter_markdown_plus/flutter_markdown_plus.dart';
import 'package:qaul_components/qaul_components.dart';

import '../l10n/app_localizations.dart';
import '../widgets/widgets.dart';

class LicenseScreen extends StatelessWidget {
  const LicenseScreen({super.key});

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
              return const QaulLoadingIndicator();
            }
            return Markdown(
              data: ss.data ?? 'An error occurred',
            );
          }),
    );
  }
}
