import 'dart:io';

import 'package:adaptive_theme/adaptive_theme.dart';
import 'package:flutter/material.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:hive_flutter/hive_flutter.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:intl/intl.dart';
import 'package:responsive_framework/responsive_wrapper.dart';

import 'helpers/navigation_helper.dart';
import 'helpers/user_prefs_helper.dart';
import 'widgets/widgets.dart';

class QaulApp extends PlatformAwareBuilder {
  const QaulApp({Key? key, this.themeMode}) : super(key: key);
  final AdaptiveThemeMode? themeMode;

  @override
  Widget defaultBuilder(BuildContext context, WidgetRef ref) {
    return AdaptiveTheme(
      light: ThemeData(
        brightness: Brightness.light,
        primarySwatch: Colors.lightBlue,
        visualDensity: VisualDensity.adaptivePlatformDensity,
        floatingActionButtonTheme: const FloatingActionButtonThemeData(
          foregroundColor: Colors.white,
        ),
        tooltipTheme:
            const TooltipThemeData(waitDuration: Duration(seconds: 1)),
        iconTheme: IconThemeData(color: Colors.grey.shade600),
        appBarTheme: AppBarTheme(
          color: Colors.transparent,
          elevation: 0.0,
          // Shadow not used as elevation is 0.0 - using this to inject color of decorators/qaul_nav_bar_decorator.dart:157
          shadowColor: Colors.grey.shade300,
          titleTextStyle: const TextStyle(
              fontSize: 16,
              fontWeight: FontWeight.bold,
              color: Colors.lightBlue),
          iconTheme: const IconThemeData(color: Colors.lightBlue),
          actionsIconTheme: const IconThemeData(color: Colors.lightBlue),
          shape: BorderDirectional(
              bottom: BorderSide(color: Colors.grey.shade300)),
        ),
        outlinedButtonTheme: OutlinedButtonThemeData(
          style: OutlinedButton.styleFrom(
            shape: RoundedRectangleBorder(
              borderRadius: BorderRadius.circular(20.0),
            ),
            foregroundColor: Colors.black,
            // fixedSize: Size(80, 36),
            side: const BorderSide(width: 1, color: Colors.black),
          ),
        ),
        inputDecorationTheme: InputDecorationTheme(
          floatingLabelBehavior: FloatingLabelBehavior.never,
          border: OutlineInputBorder(borderRadius: BorderRadius.circular(20)),
          focusedBorder: OutlineInputBorder(
            borderRadius: BorderRadius.circular(20),
            borderSide: BorderSide(color: Colors.grey.shade600),
          ),
        ),
        textSelectionTheme: TextSelectionThemeData(
          cursorColor: Colors.grey.shade600,
        ),
      ),
      dark: ThemeData(
        brightness: Brightness.dark,
        primarySwatch: Colors.lightBlue,
        visualDensity: VisualDensity.adaptivePlatformDensity,
        iconTheme: const IconThemeData(color: Colors.white),
        floatingActionButtonTheme: const FloatingActionButtonThemeData(
          backgroundColor: Colors.lightBlue,
          foregroundColor: Colors.black,
        ),
        tooltipTheme:
            const TooltipThemeData(waitDuration: Duration(seconds: 1)),
        appBarTheme: const AppBarTheme(
          elevation: 0.0,
          color: Color(0xff212121),
          shadowColor: Color(0xff212121),
        ),
        outlinedButtonTheme: OutlinedButtonThemeData(
          style: OutlinedButton.styleFrom(
            shape: RoundedRectangleBorder(
              borderRadius: BorderRadius.circular(20.0),
            ),
            foregroundColor: Colors.white,
            // fixedSize: Size(80, 36),
            side: const BorderSide(width: 1, color: Colors.white),
          ),
        ),
        inputDecorationTheme: InputDecorationTheme(
          floatingLabelBehavior: FloatingLabelBehavior.never,
          border: OutlineInputBorder(borderRadius: BorderRadius.circular(20)),
          focusedBorder: OutlineInputBorder(
            borderRadius: BorderRadius.circular(20),
            borderSide: const BorderSide(color: Colors.white),
          ),
        ),
        textSelectionTheme: const TextSelectionThemeData(
          cursorColor: Colors.white,
        ),
      ),
      initial: themeMode ?? AdaptiveThemeMode.system,
      builder: (theme, darkTheme) {
        return ValueListenableBuilder(
          valueListenable: Hive.box(UserPrefsHelper.hiveBoxName).listenable(),
          builder: (context, box, _) {
            return MaterialApp(
              theme: theme,
              darkTheme: darkTheme,
              debugShowCheckedModeBanner: false,
              initialRoute: NavigationHelper.initial,
              onGenerateRoute: NavigationHelper.onGenerateRoute,
              locale: UserPrefsHelper().defaultLocale,
              localizationsDelegates: AppLocalizations.localizationsDelegates,
              supportedLocales: AppLocalizations.supportedLocales,
              localeResolutionCallback: (locale, supportedLocales) {
                final defaultLocale = UserPrefsHelper().defaultLocale;
                if (defaultLocale != null) {
                  Intl.defaultLocale = defaultLocale.toLanguageTag();
                  return defaultLocale;
                }
                if (locale != null && supportedLocales.contains(locale)) {
                  Intl.defaultLocale = locale.toLanguageTag();
                  return locale;
                }
                return const Locale.fromSubtags(languageCode: 'en');
              },
              builder: (context, child) {
                if (Platform.isLinux || Platform.isMacOS) {
                  return child ?? const SizedBox();
                }

                return ResponsiveWrapper.builder(
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
                );
              },
            );
          },
        );
      },
    );
  }
}
