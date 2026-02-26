import 'dart:io';

import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:intl/intl.dart';
import 'package:responsive_framework/responsive_framework.dart';

import 'helpers/navigation_helper.dart';
import 'helpers/user_prefs_helper.dart';
import 'l10n/app_localizations.dart';
import 'widgets/widgets.dart';


class QaulApp extends PlatformAwareBuilder {
  const QaulApp({super.key, this.themeMode});
  final ThemeMode? themeMode;

  static final lightTheme = ThemeData(
    useMaterial3: true,
    colorScheme: ColorScheme.fromSeed(
      seedColor: Colors.lightBlue,
      brightness: Brightness.light,
      surface: Colors.white,
    ),
    brightness: Brightness.light,
    primarySwatch: Colors.lightBlue,
    scaffoldBackgroundColor: Colors.white,
    navigationBarTheme: const NavigationBarThemeData(
      surfaceTintColor: Colors.black,
    ),
    visualDensity: VisualDensity.adaptivePlatformDensity,
    floatingActionButtonTheme: const FloatingActionButtonThemeData(
      foregroundColor: Colors.white,
    ),
    tooltipTheme: const TooltipThemeData(waitDuration: Duration(seconds: 1)),
    iconTheme: IconThemeData(color: Colors.grey.shade600),
    appBarTheme: AppBarTheme(
      toolbarHeight: 72,
      backgroundColor: Colors.transparent,
      elevation: 0.0,
      // Shadow not used as elevation is 0.0 - using this to inject color of the navbar (decorators/qaul_nav_bar_decorator + nav_bar)
      shadowColor: Colors.grey.shade300,
      titleTextStyle: const TextStyle(
          fontSize: 16, fontWeight: FontWeight.bold, color: Colors.black),
      iconTheme: IconThemeData(color: Colors.grey.shade600),
      actionsIconTheme: IconThemeData(color: Colors.grey.shade600),
      shape: BorderDirectional(bottom: BorderSide(color: Colors.grey.shade300)),
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
  );

  static final darkTheme = ThemeData(
    useMaterial3: true,
    colorScheme: ColorScheme.fromSeed(
      seedColor: Colors.lightBlue,
      brightness: Brightness.dark,
    ),
    brightness: Brightness.dark,
    primarySwatch: Colors.lightBlue,
    visualDensity: VisualDensity.adaptivePlatformDensity,
    iconTheme: const IconThemeData(color: Colors.white),
    navigationBarTheme: const NavigationBarThemeData(
      surfaceTintColor: Colors.white,
    ),
    floatingActionButtonTheme: const FloatingActionButtonThemeData(
      backgroundColor: Colors.lightBlue,
      foregroundColor: Colors.black,
    ),
    tooltipTheme: const TooltipThemeData(waitDuration: Duration(seconds: 1)),
    appBarTheme: const AppBarTheme(
      toolbarHeight: 72,
      elevation: 0.0,
      backgroundColor: Color(0xff212121),
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
  );

  @override
  Widget defaultBuilder(BuildContext context, WidgetRef ref) {
    return ValueListenableBuilder<ThemeMode>(
      valueListenable: UserPrefsHelper.instance.themeModeNotifier,
      builder: (context, currentThemeMode, _) {
        return ValueListenableBuilder<Locale?>(
          valueListenable: UserPrefsHelper.instance.localeNotifier,
          builder: (context, currentLocale, _) {
            return MaterialApp(
              theme: lightTheme,
              darkTheme: darkTheme,
              themeMode: currentThemeMode,
                  debugShowCheckedModeBanner: false,
                  initialRoute: NavigationHelper.initial,
                  onGenerateRoute: NavigationHelper.onGenerateRoute,
                  locale: currentLocale,
                  localizationsDelegates: AppLocalizations.localizationsDelegates,
                  supportedLocales: AppLocalizations.supportedLocales,
                  localeResolutionCallback: (locale, supportedLocales) {
                    if (currentLocale != null) {
                      Intl.defaultLocale = currentLocale.toLanguageTag();
                      return currentLocale;
                    }
                    if (locale != null && supportedLocales.contains(locale)) {
                      Intl.defaultLocale = locale.toLanguageTag();
                      return locale;
                    }

                    final List<Locale> systemLocales =
                        View.of(context).platformDispatcher.locales;
                    for (final systemLocale in systemLocales) {
                      final lang = Locale.fromSubtags(
                          languageCode: systemLocale.languageCode);
                      if (supportedLocales.contains(systemLocale)) {
                        Intl.defaultLocale = systemLocale.toLanguageTag();
                        return systemLocale;
                      } else if (supportedLocales.contains(lang)) {
                        Intl.defaultLocale = lang.toLanguageTag();
                        return lang;
                      }
                    }

                    return const Locale.fromSubtags(languageCode: 'en');
                  },
                  builder: (context, child) {
                    if (Platform.isLinux || Platform.isMacOS) {
                      return child ?? const SizedBox();
                    }

                    return ResponsiveBreakpoints.builder(
                      child: child!,
                      breakpoints: const [
                        Breakpoint(start: 0, end: 350, name: "ANDROID"),
                        Breakpoint(start: 351, end: 480, name: MOBILE),
                        Breakpoint(start: 481, end: 680, name: "MOBILE_LANDSCAPE"),
                        Breakpoint(start: 681, end: 800, name: TABLET),
                        Breakpoint(start: 801, end: 1920, name: DESKTOP),
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
