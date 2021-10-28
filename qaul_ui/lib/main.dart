import 'package:adaptive_theme/adaptive_theme.dart';
import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:responsive_framework/responsive_framework.dart';

import 'helpers/navigation_helper.dart';

// void main() async {
//   WidgetsFlutterBinding.ensureInitialized();
//
//   await Init.initialize();
//
//   final savedThemeMode = await AdaptiveTheme.getThemeMode();
//   runApp(ProviderScope(
//       observers: [Logger()], child: QaulApp(themeMode: savedThemeMode)));
// }

/// file /state/container.dart
final container = ProviderContainer();

/// file /main.dart
void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await Init.initialize(container.read);

  final savedThemeMode = await AdaptiveTheme.getThemeMode();
  runApp(MyApp(QaulApp(themeMode: savedThemeMode)));
}

class MyApp extends StatefulWidget {
  MyApp(this.app);

  final Widget app;

  @override
  _MyApp createState() => _MyApp();
}

class _MyApp extends State<MyApp> {
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

class QaulApp extends StatelessWidget {
  const QaulApp({Key? key, this.themeMode}) : super(key: key);
  final AdaptiveThemeMode? themeMode;

  @override
  Widget build(BuildContext context) {
    return AdaptiveTheme(
      light: ThemeData(
        brightness: Brightness.light,
        primarySwatch: Colors.lightBlue,
      ),
      dark: ThemeData(
        brightness: Brightness.dark,
        primarySwatch: Colors.lightBlue,
      ),
      initial: themeMode ?? AdaptiveThemeMode.light,
      builder: (theme, darkTheme) => MaterialApp(
        theme: theme,
        darkTheme: darkTheme,
        initialRoute: NavigationHelper.initial,
        onGenerateRoute: NavigationHelper.onGenerateRoute,
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
                ResponsiveBreakpoint.autoScale(800, name: TABLET),
                ResponsiveBreakpoint.resize(1000, name: DESKTOP),
              ],
            ),
          );
        },
      ),
    );
  }
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
    final rpcNode = RpcNode(read);
    await rpcNode.getNodeInfo();

    // wait a bit
    await Future.delayed(Duration(seconds: 1));

    // DEBUG: how many messages have been sent
    final sent = await libqaul.checkSendCounter();
    print("libqaul checkSendCounter: $sent");

    // DEBUG: how many messages are queued by libqaul
    final queued = await libqaul.checkReceiveQueue();
    print("libqaul checkReceiveQueue: $queued");

    // check for rpc messages
    if (queued > 0) {
      print("libqaul receiveRpc");
      await libqaul.receiveRpc();
      print("libqaul RPC receveid");
    }
  }
}
