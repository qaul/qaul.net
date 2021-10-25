import 'package:flutter/material.dart';
import 'package:responsive_framework/responsive_framework.dart';

import 'helpers/navigation_helper.dart';

void main() {
  runApp(const QaulApp());
}

class QaulApp extends StatelessWidget {
  const QaulApp({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
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
              ResponsiveBreakpoint.autoScaleDown(450.0, name: MOBILE),
              ResponsiveBreakpoint.autoScale(760.0, name: TABLET),
            ],
          ),
        );
      },
    );
  }
}
