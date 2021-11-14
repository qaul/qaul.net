import 'package:flutter/material.dart';
import 'package:flutter_markdown/flutter_markdown.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';

class AboutScreen extends StatelessWidget {
  const AboutScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final l18ns = AppLocalizations.of(context);
    var bundle = DefaultAssetBundle.of(context);

    return Scaffold(
      appBar: AppBar(
        leading: IconButton(
          tooltip: l18ns!.backButtonTooltip,
          icon: const Icon(Icons.arrow_back_ios_rounded),
          onPressed: () => Navigator.pop(context),
        ),
        title: Row(
          children: [
            const Icon(Icons.info_outline_rounded),
            const SizedBox(width: 8),
            Text(l18ns.about),
          ],
        ),
      ),
      body: FutureBuilder<String>(
          future: bundle.loadString('assets/license/agpl-3.0.md'),
          builder: (context, ss) {
            if (!ss.hasData || ss.connectionState != ConnectionState.done) {
              return const Center(child: CircularProgressIndicator());
            }
            return Markdown(
              data: ss.data ?? 'An error occurred',
            );
          }),
    );
  }
}
