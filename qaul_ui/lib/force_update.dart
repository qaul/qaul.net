import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:logging/logging.dart';
import 'package:path_provider/path_provider.dart';
import 'package:url_launcher/url_launcher.dart';
import 'package:version/version.dart';

/// forceUpdateRequired returns true if current is not compatible with target.
bool forceUpdateRequired(Version current, Version target) {
  if (current == target) {
    return int.parse(current.build) < int.parse(target.build);
  }
  throw UnimplementedError("until now, qaul only has one version. Add logic");
}

/// Utility class to give support to the Force-update flow
class ForceUpdateSystem {
  static final _qaulRepoURL = Uri.parse("https://github.com/qaul/qaul.net");

  static final forceUpdateVersion =
      Version(2, 0, 0, preRelease: ["beta"], build: "18");

  static final _invalidSemverFormat =
      RegExp(r"[0-9]\.[0-9]\.[0-9]-beta\.[0-9]");

  static bool _isFile(FileSystemEntity e) =>
      e.statSync().type == FileSystemEntityType.file;

  static Future<(bool required, Version? version)> shouldForceUpdate() async {
    // - Android : `/data/user/0/net.qaul.qaul_app/files`
    final appDocumentDir = await getApplicationSupportDirectory();

    final entities = appDocumentDir.listSync();
    for (final e in entities) {
      if (!e.path.endsWith('version') || !_isFile(e)) {
        continue;
      }

      try {
        var versionFile = File.fromUri(e.uri).readAsStringSync();
        if (_invalidSemverFormat.hasMatch(versionFile)) {
          final index = versionFile.lastIndexOf(".");
          versionFile = versionFile.replaceRange(index, index + 1, "+");
        }

        final previous = Version.parse(versionFile);
        return (forceUpdateRequired(previous, forceUpdateVersion), previous);
      } on FormatException catch (e) {
        Logger.root.warning("unable to parse version file: ${e.toString()}");
        return (false, null);
      }
    }

    return (false, null);
  }

  static void openQaulRepo() async {
    if (await canLaunchUrl(_qaulRepoURL)) {
      launchUrl(_qaulRepoURL);
    }
  }
}

class ForceUpdateDialog extends StatelessWidget {
  const ForceUpdateDialog({
    Key? key,
    required this.previous,
    required this.required,
    this.onLinkPressed,
  }) : super(key: key);

  final String previous;
  final String required;
  final VoidCallback? onLinkPressed;

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final ttheme = Theme.of(context).textTheme;

    return Dialog(
      child: DefaultTextStyle(
        style: ttheme.bodyMedium!,
        textAlign: TextAlign.center,
        child: Padding(
          padding: const EdgeInsets.symmetric(vertical: 8, horizontal: 24),
          child: Column(
            mainAxisAlignment: MainAxisAlignment.start,
            crossAxisAlignment: CrossAxisAlignment.center,
            children: [
              Text(
                l10n.forceUpdateRequired,
                style: ttheme.displaySmall,
              ),
              const SizedBox(height: 20),

              Text(l10n.forceUpdateDescription1),
              const SizedBox(height: 8),

              Row(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  Expanded(child: Text(l10n.forceUpdateDescription2)),
                  const SizedBox(height: 8),
                  IconButton(
                    onPressed: onLinkPressed,
                    icon: const Icon(Icons.open_in_new),
                  ),
                ],
              ),
              const SizedBox(height: 8),

              Text(l10n.forceUpdateDescription3),
              const SizedBox(height: 8),
              FilledButton(
                onPressed: () {},
                child: Text(l10n.forceUpdateCreateAccount),
              ),

              const SizedBox(height: 8),
              Text(l10n.forceUpdateDisclaimer, style: ttheme.labelLarge),

              // const Expanded(child: SizedBox.shrink()),
              Expanded(
                child: SizedBox(
                  width: double.maxFinite,
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.center,
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      Text(l10n.previousVersion, style: ttheme.titleSmall),
                      Text(previous),
                      const SizedBox(height: 8),
                      Text(l10n.currentVersion, style: ttheme.titleSmall),
                      Text(required),
                    ],
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
