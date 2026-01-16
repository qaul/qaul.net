import 'dart:async';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:local_notifications/local_notifications.dart';
import 'package:logging/logging.dart';
import 'package:package_info_plus/package_info_plus.dart';
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
      PackageInfo packageInfo = await PackageInfo.fromPlatform();

      runApp(
        MaterialApp(
          theme: QaulApp.lightTheme,
          darkTheme: QaulApp.darkTheme,
          themeMode: ThemeMode.system,
          localizationsDelegates: AppLocalizations.localizationsDelegates,
          supportedLocales: AppLocalizations.supportedLocales,
          home: ForceUpdateDialog(
            current: packageInfo.version,
            previous: previousVersion?.toString() ?? '',
            onLinkPressed: ForceUpdateSystem.openQaulRepo,
            onDeleteAccountPressed: () async {
              await ForceUpdateSystem.deleteAccount();
              await _defaultAppEntrypoint();
            },
          ),
        ),
      );
      return;
    }

    await _defaultAppEntrypoint();
  }, (error, stack) => Logger.root.severe(error, error, stack));
}

Future<void> _defaultAppEntrypoint() async {
  await Initializer.initialize(_container);

  final savedThemeMode = UserPrefsHelper().themeMode;
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
    _container.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return UncontrolledProviderScope(container: _container, child: widget.app);
  }
}

class Initializer {
  static Future<void> initialize(ProviderContainer container) async {
    await _container.read(qaulWorkerProvider).initialized;
    await EmailLoggingCoordinator.instance.initialize(container: container);

    await UserPrefsHelper.initialize();

    await LocalNotifications.instance.initialize();
  }
}
