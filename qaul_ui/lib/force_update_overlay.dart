import 'dart:io';

import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:logging/logging.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:url_launcher/url_launcher.dart';
import 'package:version/version.dart';

/// forceUpdateRequired returns true if current is not compatible with target.
bool forceUpdateRequired(Version current, Version target) {
  if (current == target) {
    return int.parse(current.build) < int.parse(target.build);
  }
  throw UnimplementedError("until now, qaul only has one version. Add logic");
}

class ForceUpdateOverlay extends StatefulHookConsumerWidget {
  const ForceUpdateOverlay({Key? key, required this.child}) : super(key: key);
  final Widget child;

  @override
  ConsumerState<ForceUpdateOverlay> createState() => _ForceUpdateOverlayState();
}

class _ForceUpdateOverlayState extends ConsumerState<ForceUpdateOverlay> {
  final qaulRepoURL = Uri.parse("https://github.com/qaul/qaul.net");

  final forceUpdateVersion =
      Version(2, 0, 0, preRelease: ["beta"], build: "18");

  Version? previousVersion;

  final invalidSemverFormat = RegExp(r"[0-9]\.[0-9]\.[0-9]-beta\.[0-9]");

  bool showForceUpdateDialog() {
    final path = ref.read(libqaulLogsStoragePath);
    if (path == null) return false;

    final entities = Directory(path).parent.listSync();
    for (final e in entities) {
      if (!e.path.endsWith('version') || !isFile(e)) {
        continue;
      }

      try {
        var versionFile = File.fromUri(e.uri).readAsStringSync();
        if (invalidSemverFormat.hasMatch(versionFile)) {
          final index = versionFile.lastIndexOf(".");
          versionFile = versionFile.replaceRange(index, index + 1, "+");
        }

        final previous = Version.parse(versionFile);
        setState(() => previousVersion = previous);
        return forceUpdateRequired(previous, forceUpdateVersion);
      } on FormatException catch (e) {
        Logger.root.warning("unable to parse version file: ${e.toString()}");
        return false;
      }
    }

    return false;
  }

  bool isFile(FileSystemEntity e) =>
      e.statSync().type == FileSystemEntityType.file;

  void openQaulRepo() async {
    if (await canLaunchUrl(qaulRepoURL)) {
      launchUrl(qaulRepoURL);
    }
  }

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((timeStamp) async {
      await ref.read(qaulWorkerProvider).initialized;
      var attempts = 0;
      while (ref.read(libqaulLogsStoragePath) == null) {
        await Future.delayed(Duration(milliseconds: 10 * (attempts + 1)));
        attempts++;
        if (attempts == 20) {
          Logger.root.warning("giving up force-update check after 20 attempts");
          return;
        }
      }
      if (mounted && showForceUpdateDialog() && previousVersion != null) {
        showDialog(
          context: context,
          barrierDismissible: false,
          builder: (context) {
            return ForceUpdateDialog(
              previous: previousVersion!.toString(),
              required: forceUpdateVersion.toString(),
              onLinkPressed: openQaulRepo,
            );
          },
        );
      }
    });
  }

  @override
  Widget build(BuildContext context) {
    return widget.child;
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
    final ttheme = Theme.of(context).textTheme;

    return Dialog(
      child: Padding(
        padding: const EdgeInsets.all(8.0),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.start,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            SizedBox(
              width: double.maxFinite,
              child: Text(
                "Update Required",
                style: ttheme.displaySmall,
                textAlign: TextAlign.center,
              ),
            ),
            const SizedBox(height: 20),

            Text("What's wrong", style: ttheme.titleLarge),
            const SizedBox(height: 4),
            const Text(
              "You're updating to a major release, which is incompatible with the last detected version.",
            ),
            const SizedBox(height: 8),

            Text("What should I do", style: ttheme.titleLarge),
            const SizedBox(height: 4),
            const Text(
              "Update to the previous version before updating to this one.",
            ),
            const SizedBox(height: 8),

            Text("Why it's necessary", style: ttheme.titleLarge),
            const SizedBox(height: 4),
            const Text(
              "This ensures qaul.net can migrate you to the latest major release without data loss.",
            ),

            // const Expanded(child: SizedBox.shrink()),
            Expanded(
              child: SizedBox(
                width: double.maxFinite,
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.center,
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    Text("Last detected version", style: ttheme.titleLarge),
                    Text(previous, style: ttheme.labelLarge),
                    const SizedBox(height: 8),
                    Text("Version required for update",
                        style: ttheme.titleLarge),
                    Text(required, style: ttheme.labelLarge),
                  ],
                ),
              ),
            ),

            Align(
              alignment: Alignment.bottomCenter,
              child: TextButton(
                onPressed: onLinkPressed,
                child: const Row(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    Text("qaul.net project website"),
                    SizedBox(width: 8),
                    Icon(
                      Icons.open_in_new,
                      size: 16,
                    )
                  ],
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
