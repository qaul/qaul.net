// import 'dart:io';

import 'dart:async';

import 'package:adaptive_theme/adaptive_theme.dart';
import 'package:flutter/material.dart';
import 'package:hive_flutter/hive_flutter.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:logger/logger.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:qaul_ui/qaul_app.dart';
// import 'package:bitsdojo_window/bitsdojo_window.dart';

import 'helpers/user_prefs_helper.dart';

final container = ProviderContainer();

void main() async {
  runZonedGuarded<Future<void>>(() async {
    WidgetsFlutterBinding.ensureInitialized();
    await Init.initialize(container.read);
    await Hive.initFlutter();
    await Hive.openBox(UserPrefsHelper.hiveBoxName);

    await Logger.instance.initialize();

    throw FlutterError('a custom error');

    final savedThemeMode = await AdaptiveTheme.getThemeMode();
    runApp(_CustomProviderScope(QaulApp(themeMode: savedThemeMode)));

    // if (Platform.isLinux || Platform.isMacOS) {
    //   doWhenWindowReady(() {
    //     const initialSize = Size(1920, 1080);
    //     appWindow.minSize = const Size(800, 600);
    //     appWindow.size = initialSize;
    //     appWindow.alignment = Alignment.center;
    //     appWindow.show();
    //   });
    // }
  }, (error, stack) => Logger.instance.logError(error, stack));
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
  static Future<void> initialize(Reader read) async => await read(qaulWorkerProvider).initialized;
}
