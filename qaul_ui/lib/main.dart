import 'package:adaptive_theme/adaptive_theme.dart';
import 'package:flutter/material.dart';
import 'package:responsive_framework/responsive_framework.dart';

import 'helpers/navigation_helper.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  final savedThemeMode = await AdaptiveTheme.getThemeMode();
  runApp(QaulApp(themeMode: savedThemeMode));
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
