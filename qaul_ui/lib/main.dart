import 'dart:async';

import 'package:adaptive_theme/adaptive_theme.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:hive_flutter/hive_flutter.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:local_notifications/local_notifications.dart';
import 'package:logging/logging.dart';
import 'package:qaul_rpc/qaul_rpc.dart';

// import 'package:bitsdojo_window/bitsdojo_window.dart';

import 'coordinators/email_logging_coordinator/email_logging_coordinator.dart';
import 'helpers/navigation_helper.dart';
import 'helpers/user_prefs_helper.dart';
import 'qaul_app.dart';

final _container = ProviderContainer();

void main() async {
  runZonedGuarded<Future<void>>(() async {
    WidgetsFlutterBinding.ensureInitialized();
    await Initializer.initialize(_container.read);

    Logger.root.level = kDebugMode ? Level.CONFIG : Level.FINE;

    final savedThemeMode = await AdaptiveTheme.getThemeMode();
    runApp(_CustomProviderScope(QaulApp(themeMode: savedThemeMode)));
  }, (error, stack) => Logger.root.severe('', error, stack));
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
    _container.read(qaulWorkerProvider).onLibraryCrash.listen((_) {
      showDialog(
          context: context,
          barrierDismissible: false,
          builder: (_) {
            return AlertDialog(
              title: const Text('An Error occurred'),
              content: const Text('Please restart the application.'),
              actions: [
                TextButton(
                  onPressed: () => Navigator.pushNamed(context, NavigationHelper.support),
                  child: const Text('Go to support'),
                ),
              ],
            );
          });
    });
  }

  @override
  void dispose() {
    super.dispose();
    // disposing the globally self managed container.
    _container.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return UncontrolledProviderScope(container: _container, child: widget.app);
  }
}

class Initializer {
  static Future<void> initialize(Reader read) async {
    await EmailLoggingCoordinator.instance.initialize(reader: read);

    await read(qaulWorkerProvider).initialized;

    await Hive.initFlutter();
    await Hive.openBox(UserPrefsHelper.hiveBoxName);

    await LocalNotifications.instance.initialize();
  }
}
