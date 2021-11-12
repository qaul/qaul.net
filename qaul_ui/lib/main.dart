import 'dart:ui';

import 'package:adaptive_theme/adaptive_theme.dart';
import 'package:flutter/material.dart';
import 'package:hive/hive.dart';
import 'package:hive_flutter/hive_flutter.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:responsive_framework/responsive_framework.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';

import 'helpers/navigation_helper.dart';
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

class QaulApp extends ConsumerWidget {
  const QaulApp({Key? key, this.themeMode}) : super(key: key);
  final AdaptiveThemeMode? themeMode;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return AdaptiveTheme(
      light: ThemeData(
        brightness: Brightness.light,
        primarySwatch: Colors.lightBlue,
        floatingActionButtonTheme: const FloatingActionButtonThemeData(
          foregroundColor: Colors.white,
        ),
        tooltipTheme:
            const TooltipThemeData(waitDuration: Duration(seconds: 1)),
        iconTheme: IconThemeData(color: Colors.grey.shade600),
        appBarTheme: AppBarTheme(
            color: Colors.transparent,
            elevation: 0.0,
            titleTextStyle: const TextStyle(
                fontSize: 16,
                fontWeight: FontWeight.bold,
                color: Colors.lightBlue),
            iconTheme: const IconThemeData(color: Colors.lightBlue),
            actionsIconTheme: const IconThemeData(color: Colors.lightBlue),
            shape: BorderDirectional(
                bottom: BorderSide(color: Colors.grey.shade300))),
      ),
      dark: ThemeData(
        brightness: Brightness.dark,
        primarySwatch: Colors.lightBlue,
        iconTheme: const IconThemeData(color: Colors.white),
        floatingActionButtonTheme: const FloatingActionButtonThemeData(
          backgroundColor: Colors.lightBlue,
          foregroundColor: Colors.black,
        ),
        tooltipTheme:
            const TooltipThemeData(waitDuration: Duration(seconds: 1)),
        appBarTheme:
            const AppBarTheme(elevation: 0.0, color: Color(0xff212121)),
      ),
      initial: themeMode ?? AdaptiveThemeMode.light,
      builder: (theme, darkTheme) {
        return ValueListenableBuilder(
          valueListenable: Hive.box(UserPrefsHelper.hiveBoxName).listenable(),
          builder: (context, box, _) {
            return MaterialApp(
              theme: theme,
              darkTheme: darkTheme,
              initialRoute: NavigationHelper.initial,
              onGenerateRoute: NavigationHelper.onGenerateRoute,
              scrollBehavior: TouchAndMouseScrollBehavior(),
              locale: UserPrefsHelper().defaultLocale,
              localizationsDelegates: AppLocalizations.localizationsDelegates,
              supportedLocales: AppLocalizations.supportedLocales,
              localeResolutionCallback: (locale, supportedLocales) {
                final defaultLocale = UserPrefsHelper().defaultLocale;
                if (defaultLocale != null) return defaultLocale;
                if (supportedLocales.contains(locale)) return locale;
                return const Locale.fromSubtags(languageCode: 'en');
              },
              builder: (context, child) {
                final mediaQuery = MediaQuery.of(context);
                return MediaQuery(
                  data: mediaQuery.copyWith(textScaleFactor: 1.0),
                  child: ResponsiveWrapper.builder(
                    child,
                    maxWidth: 828,
                    minWidth: 370,
                    breakpoints: const [
                      ResponsiveBreakpoint.resize(350.0,
                          name: 'ANDROID', scaleFactor: 0.8),
                      ResponsiveBreakpoint.resize(480, name: MOBILE),
                      ResponsiveBreakpoint.resize(680,
                          name: 'MOBILE_LANDSCAPE', scaleFactor: 0.8),
                      ResponsiveBreakpoint.autoScale(800, name: TABLET),
                      ResponsiveBreakpoint.resize(1000, name: DESKTOP),
                    ],
                  ),
                );
              },
            );
          },
        );
      },
    );
  }
}

class TouchAndMouseScrollBehavior extends MaterialScrollBehavior {
  // Override behavior methods and getters like dragDevices
  @override
  Set<PointerDeviceKind> get dragDevices => {
        PointerDeviceKind.touch,
        PointerDeviceKind.mouse,
        // etc.
      };
}

class Init {
  static Future<void> initialize(Reader read) async {
    print("initialize libqaul");
    // load libqaul
    // get it from provider
    final libqaul = read(libqaulProvider);

    print("libqaul loaded");

    // test platform function
    // final platform = await libqaul.getPlatformVersion();
    // print(platform);

    // call hello function
    final hello = await libqaul.hello();
    print(hello);

    // start libqaul
    await libqaul.start();
    print("libqaul started");

    // check if libqaul finished initializing
    //await Future.delayed(Duration(seconds: 3));
    while (libqaul.initialized() == 0) {
      await Future.delayed(Duration(milliseconds: 10));
    }

    print("libqaul initialization finished");

    // request node info
    // final rpcNode = RpcNode(read);
    // await rpcNode.getNodeInfo();

    // wait a bit
    // await Future.delayed(Duration(seconds: 1));

    // DEBUG: how many messages have been sent
    // final sent = await libqaul.checkSendCounter();
    // print("libqaul checkSendCounter: $sent");

    // DEBUG: how many messages are queued by libqaul
    // final queued = await libqaul.checkReceiveQueue();
    // print("libqaul checkReceiveQueue: $queued");

    // check for rpc messages
    // if (queued > 0) {
    //   print("libqaul receiveRpc");
    //   await libqaul.receiveRpc();
    //   print("libqaul RPC receveid");
    // }
  }
}
