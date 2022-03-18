// import 'dart:io';

import 'dart:async';

import 'package:adaptive_theme/adaptive_theme.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:hive_flutter/hive_flutter.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:local_notifications/local_notifications.dart';
import 'package:logger/logger.dart';
import 'package:logging/logging.dart' as logging;
import 'package:qaul_rpc/qaul_rpc.dart';

// import 'package:bitsdojo_window/bitsdojo_window.dart';

import 'helpers/user_prefs_helper.dart';
import 'qaul_app.dart';

final container = ProviderContainer();

void main() async {
  runZonedGuarded<Future<void>>(() async {
    WidgetsFlutterBinding.ensureInitialized();
    await Init.initialize(container.read);
    await Hive.initFlutter();
    await Hive.openBox(UserPrefsHelper.hiveBoxName);
    await LocalNotifications.instance.initialize();

    logging.Logger.root.level = kDebugMode ? logging.Level.CONFIG : logging.Level.FINE;
    logging.Logger.root.onRecord.listen((record) {
      debugPrint('[${record.level.name}] ${record.loggerName} (${record.time}): ${record.message}');
    });

    await Logger.instance.initialize();

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
  void initState() {
    super.initState();
    container.read(qaulWorkerProvider).onLibraryCrash.listen((_) {
      showDialog(
          context: context,
          barrierDismissible: false,
          builder: (_) {
            return const AlertDialog(
              title: Text('An Error occurred'),
              content: Text('Please restart the application.'),
            );
          });
    });
  }

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
