import 'dart:async';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:local_notifications/local_notifications.dart';
import 'package:logging/logging.dart';
import 'package:package_info_plus/package_info_plus.dart';
import 'package:qaul_rpc/qaul_rpc.dart';

import 'coordinators/email_logging_coordinator/email_logging_coordinator.dart';
import 'force_update.dart';
import 'helpers/user_prefs_helper.dart';
import 'l10n/app_localizations.dart';
import 'qaul_app.dart';
import 'session/session_scope.dart';
import 'stores/stores.dart';

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

  final savedThemeMode = UserPrefsHelper.instance.themeMode;
  runApp(_CustomProviderScope(QaulApp(themeMode: savedThemeMode)));
}

class _CustomProviderScope extends StatefulWidget {
  const _CustomProviderScope(this.app);

  final Widget app;

  @override
  _CustomProviderScopeState createState() => _CustomProviderScopeState();
}

class _CustomProviderScopeState extends State<_CustomProviderScope>
    with WidgetsBindingObserver {
  ProviderSubscription<String?>? _sessionSubscription;
  bool _resumed = true;

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addObserver(this);
    // The single session boundary: it discards the outgoing account's state and
    // then brings polling in line with the incoming one. A session reset takes
    // UsersStore's polling timer down with it, so polling has to be restarted
    // per session rather than once at startup.
    _sessionSubscription = listenForSessionChanges(
      _container,
      onSessionChanged: (_) => _syncOnlinePolling(),
    );
  }

  @override
  void dispose() {
    _sessionSubscription?.close();
    WidgetsBinding.instance.removeObserver(this);
    // Disposing the globally self managed container. UsersStore cancels its
    // polling timer from `ref.onDispose`.
    _container.dispose();
    super.dispose();
  }

  @override
  void didChangeAppLifecycleState(AppLifecycleState state) {
    _resumed = state == AppLifecycleState.resumed;
    _syncOnlinePolling();
  }

  /// Polling is a function of two inputs — a foregrounded app and a signed-in
  /// account — evaluated in one place so the two triggers cannot disagree.
  void _syncOnlinePolling() {
    final shouldPoll = _resumed && _container.read(sessionKeyProvider) != null;
    final store = _container.read(usersStoreProvider.notifier);
    shouldPoll ? store.startOnlinePolling() : store.stopOnlinePolling();
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
