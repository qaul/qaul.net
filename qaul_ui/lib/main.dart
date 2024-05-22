import 'dart:async';

import 'package:adaptive_theme/adaptive_theme.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:hive_flutter/hive_flutter.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:local_notifications/local_notifications.dart';
import 'package:logging/logging.dart';
import 'package:qaul_rpc/qaul_rpc.dart';

import 'coordinators/email_logging_coordinator/email_logging_coordinator.dart';
import 'force_update.dart';
import 'helpers/user_prefs_helper.dart';
import 'qaul_app.dart';

final _container = ProviderContainer();

void main() async {
  runZonedGuarded<Future<void>>(() async {
    WidgetsFlutterBinding.ensureInitialized();
    Logger.root.level = kDebugMode ? Level.CONFIG : Level.FINE;

    final (shouldForceUpdate, previousVersion) =
        await ForceUpdateSystem.shouldForceUpdate();

    if (shouldForceUpdate) {
      runApp(MaterialApp(
        home: ForceUpdateDialog(
          required: ForceUpdateSystem.forceUpdateVersion.toString(),
          previous: previousVersion?.toString() ?? '',
          onLinkPressed: ForceUpdateSystem.openQaulRepo,
        ),
      ));
      return;
    }

    await Initializer.initialize(_container.read);

    final savedThemeMode = await AdaptiveTheme.getThemeMode();
    runApp(_CustomProviderScope(QaulApp(themeMode: savedThemeMode)));
  }, (error, stack) => Logger.root.severe(error, error, stack));
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
    _container.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return UncontrolledProviderScope(container: _container, child: widget.app);
  }
}

class Initializer {
  static Future<void> initialize(Reader read) async {
    await read(qaulWorkerProvider).initialized;
    await EmailLoggingCoordinator.instance.initialize(reader: read);

    await Hive.initFlutter();
    await Hive.openBox(UserPrefsHelper.hiveBoxName);

    await LocalNotifications.instance.initialize();
  }
}
