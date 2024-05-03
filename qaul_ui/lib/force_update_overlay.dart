import 'dart:io';

import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:logging/logging.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
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
  final forceUpdateVersion = Version(2, 0, 0, preRelease: ["beta"], build: "18");
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

        final current = Version.parse(versionFile);
        return forceUpdateRequired(current, forceUpdateVersion);
      } on FormatException catch (e) {
        Logger.root.warning("unable to parse version file: ${e.toString()}");
        return false;
      }
    }

    return false;
  }

  bool isFile(FileSystemEntity e) =>
      e.statSync().type == FileSystemEntityType.file;

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
      if (showForceUpdateDialog()) {
        // TODO show modal component
        print("show!");
      }
    });
  }

  @override
  Widget build(BuildContext context) {
    return widget.child;
  }
}
