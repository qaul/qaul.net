// import 'dart:io';

import 'package:adaptive_theme/adaptive_theme.dart';
import 'package:flutter/material.dart';
import 'package:hive/hive.dart';
import 'package:hive_flutter/hive_flutter.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:qaul_ui/qaul_app.dart';
// import 'package:window_manager/window_manager.dart';

import 'helpers/user_prefs_helper.dart';

/// file /state/container.dart
final container = ProviderContainer();

/// file /main.dart
void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await Init.initialize(container.read);
  await Hive.initFlutter();
  await Hive.openBox(UserPrefsHelper.hiveBoxName);

  final savedThemeMode = await AdaptiveTheme.getThemeMode();
  runApp(_CustomProviderScope(QaulApp(themeMode: savedThemeMode)));
}

class _CustomProviderScope extends StatefulWidget {
  const _CustomProviderScope(this.app);

  final Widget app;

  @override
  _CustomProviderScopeState createState() => _CustomProviderScopeState();
}

class _CustomProviderScopeState extends State<_CustomProviderScope> {
  @override
  void dispose() {
    super.dispose();
    // disposing the globally self managed container.
    container.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return UncontrolledProviderScope(container: container, child: widget.app);
  }
}

class Init {
  static Future<void> initialize(Reader read) async {
    // TODO(brenodt): This package was making the application not display on Windows. Removing for now
    // if (Platform.isMacOS || Platform.isLinux) {
    //   await SystemChrome.setPreferredOrientations(
    //       [DeviceOrientation.landscapeLeft]);
    //   await initializeWindowManager();
    // }

    await read(qaulWorkerProvider).initialized;
  }

// static Future<void> initializeWindowManager() async {
//   assert(!Platform.isWindows);
//   await windowManager.ensureInitialized();
//
//   // Use it only after calling `hiddenWindowAtLaunch`
//   windowManager.waitUntilReadyToShow().then((_) async {
//     await windowManager.setSize(Size(600, 600));
//     await windowManager.setMinimumSize(const Size(512, 400));
//     await windowManager.setMaximumSize(const Size(828, 760));
//     await windowManager.setFullScreen(false);
//     await windowManager.show();
//   });
// }
}
