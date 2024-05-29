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
  return false;
}

/// Utility class to give support to the Force-update flow
class ForceUpdateSystem {
  static final _qaulRepoURL =
      Uri.parse("https://github.com/qaul/qaul.net/releases/tag/v2.0.0-beta.18");

  static final forceUpdateVersion =
      Version(2, 0, 0, preRelease: ["beta"], build: "18");

  static final _invalidSemverFormat =
      RegExp(r"[0-9]\.[0-9]\.[0-9]-beta\.[0-9]");

  static bool _isFile(FileSystemEntity e) =>
      e.statSync().type == FileSystemEntityType.file;

  // This is the folder where qaul_rpc stores all user data. Deleting it erases
  // user information, which prompts for account creation.
  static Future<Directory> _qaulRpcFilesDir() async {
    if (Platform.isIOS) {
      // - iOS : /var/mobile/Containers/Data/Application/THE-DEVICE-ID/Documents
      return getApplicationDocumentsDirectory();
    }
    if (Platform.isLinux) {
      final env = Platform.environment;
      if (env["FLUTTER_ROOT"]!.contains('snap')) {
        return Directory('${env['HOME']}/snap/flutter/common');
      }
      return env.containsKey('SNAP')
          ? Directory('${env['HOME']}/snap/qaul/common')
          : Directory('${env['HOME']}/.config/qaul');
    }

    // Returns the following Path:
    // - Android : /data/user/0/net.qaul.qaul_app/files
    // - MacOS   : /Users/XYZ/Library/Containers/net.qaul.app/Data/Library/Application Support/net.qaul.app
    // - Windows : C:\Users\XYZ\AppData\Roaming\net.qaul.qaulapp\qaul
    //
    // On iOS, it's easiePr to use `getApplicationDocumentsDirectory`; however, the path returned would be:
    // /var/mobile/Containers/Data/Application/THE-DEVICE-ID/Library/Application Support
    final appDocumentDir = await getApplicationSupportDirectory();

    if (Platform.isMacOS) {
      var dir = Directory("${appDocumentDir.parent.path}/net.qaul.qaul");
      assert(dir.existsSync());
      return dir;
    }
    if (Platform.isWindows) {
      var dir =
          Directory("${appDocumentDir.parent.parent.path}\\qaul\\qaul\\config");
      assert(dir.existsSync());
      return dir;
    }

    // if Platform.isAndroid
    return appDocumentDir;
  }

  static Future<(bool required, Version? version)> shouldForceUpdate() async {
    final appDocumentDir = await _qaulRpcFilesDir();
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

  static Future<void> deleteAccount() async {
    final appDocumentDir = await _qaulRpcFilesDir();
    await appDocumentDir.delete(recursive: true);
  }
}

class ForceUpdateDialog extends StatelessWidget {
  const ForceUpdateDialog({
    Key? key,
    required this.previous,
    required this.current,
    this.onLinkPressed,
    this.onDeleteAccountPressed,
  }) : super(key: key);

  final String previous;
  final String current;
  final VoidCallback? onLinkPressed;
  final VoidCallback? onDeleteAccountPressed;

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
          child: ListView(
            children: [
              Text(
                l10n.forceUpdateRequired,
                style: ttheme.displaySmall,
              ),
              const SizedBox(height: 20),
              Text(l10n.forceUpdateDescription1),
              const SizedBox(height: 8),
              Text(l10n.forceUpdateDescription2),
              const SizedBox(height: 8),
              FilledButton(
                onPressed: onLinkPressed,
                style: FilledButton.styleFrom(
                  maximumSize: const Size.fromWidth(200),
                ),
                child: Text(
                  l10n.forceUpdateDownloadQaul18,
                  textAlign: TextAlign.center,
                ),
              ),
              const SizedBox(height: 40),
              Text(l10n.forceUpdateDescription3),
              const SizedBox(height: 8),
              FilledButton(
                onPressed: onDeleteAccountPressed,
                child: Text(l10n.forceUpdateCreateAccount),
              ),
              const SizedBox(height: 8),
              Text(l10n.forceUpdateDisclaimer, style: ttheme.labelLarge),
              const SizedBox(height: 40),
              SizedBox(
                width: double.maxFinite,
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.center,
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    Text(l10n.previousVersion, style: ttheme.titleSmall),
                    Text(previous),
                    const SizedBox(height: 8),
                    Text(l10n.currentVersion, style: ttheme.titleSmall),
                    Text(current),
                  ],
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
