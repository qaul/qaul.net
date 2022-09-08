import 'package:fluent_ui/fluent_ui.dart';

class Responsiveness {
  // Breakpoints
  static const kTabletBreakpoint = 758.0;
  static const kDesktopBreakpoint = 1440.0;

  // Constraints
  static const kNavigationSideRailWidthConstraint = 72.0;
  static const kSideMenuWidthConstraints = BoxConstraints(
    minWidth: 72.0,
    maxWidth: 300.0,
  );

  static const kMaxWidth = 1180.0;

  static bool isMobile(BuildContext context) =>
      MediaQuery.of(context).size.width < kTabletBreakpoint;

  static bool isTablet(BuildContext context) {
    var width = MediaQuery.of(context).size.width;
    return width >= kTabletBreakpoint && width < kDesktopBreakpoint;
  }

  static bool isDesktop(BuildContext context) =>
      MediaQuery.of(context).size.width >= kDesktopBreakpoint;
}
