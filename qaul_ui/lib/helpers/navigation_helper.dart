import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';

import '../screens/splash_screen.dart';

class NavigationHelper {
  static const initial = '/';

  static Route<T> _buildRoute<T>(
          final RouteSettings settings, final WidgetBuilder page) =>
      CupertinoPageRoute(builder: page, settings: settings);

  static Route<dynamic> onGenerateRoute(final RouteSettings _settings) {
    switch (_settings.name) {
      case initial:
        return _buildRoute(
            _settings,
            (context) => WillPopScope(
                  onWillPop: () async => false,
                  child: const SplashScreen(),
                ));
      default:
        throw ArgumentError.value(_settings.name, 'Route name',
            'Handle this route in NavigationHelper.');
    }
  }
}
