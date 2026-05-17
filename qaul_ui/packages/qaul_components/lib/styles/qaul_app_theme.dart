import 'package:flutter/material.dart';

/// Light/dark [ThemeData] aligned with [QaulApp] for Widgetbook and the host app.
abstract final class QaulAppTheme {
  static final ThemeData light = ThemeData(
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
    iconTheme: IconThemeData(color: Colors.grey),
    appBarTheme: AppBarTheme(
      toolbarHeight: 72,
      backgroundColor: Colors.transparent,
      elevation: 0.0,
      shadowColor: Colors.grey.shade300,
      titleTextStyle: const TextStyle(
        fontSize: 16,
        fontWeight: FontWeight.bold,
        color: Colors.black,
      ),
      iconTheme: IconThemeData(color: Colors.grey.shade600),
      actionsIconTheme: IconThemeData(color: Colors.grey.shade600),
      shape: BorderDirectional(
        bottom: BorderSide(color: Colors.grey.shade300),
      ),
    ),
    outlinedButtonTheme: OutlinedButtonThemeData(
      style: OutlinedButton.styleFrom(
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(20.0),
        ),
        foregroundColor: Colors.black,
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

  static final ThemeData dark = ThemeData(
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
        side: const BorderSide(width: 1, color: Colors.white),
      ),
    ),
    inputDecorationTheme: const InputDecorationTheme(
      floatingLabelBehavior: FloatingLabelBehavior.never,
      border: OutlineInputBorder(
        borderRadius: BorderRadius.all(Radius.circular(20)),
      ),
      focusedBorder: OutlineInputBorder(
        borderRadius: BorderRadius.all(Radius.circular(20)),
        borderSide: BorderSide(color: Colors.white),
      ),
    ),
    textSelectionTheme: const TextSelectionThemeData(
      cursorColor: Colors.white,
    ),
  );
}
