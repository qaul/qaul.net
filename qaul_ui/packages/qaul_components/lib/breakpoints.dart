import 'package:flutter/material.dart';

const String kBreakpointIphone16 = 'iPhone 16';
const String kBreakpointIphone16Pro = 'iPhone 16 Pro';
const String kBreakpointLaptop = 'Laptop';
const String kBreakpointMacbook = 'MacBook';
const String kBreakpointFullHd = 'Full HD';

const double kTabletBreakpoint = 758.0;
const double kDesktopBreakpoint = 1440.0;

const List<DesignerViewport> kDesignerBreakpoints = [
  DesignerViewport(name: kBreakpointIphone16, width: 393, height: 852),
  DesignerViewport(name: kBreakpointIphone16Pro, width: 402, height: 874),
  DesignerViewport(name: kBreakpointLaptop, width: 1366, height: 768),
  DesignerViewport(name: kBreakpointMacbook, width: 1440, height: 900),
  DesignerViewport(name: kBreakpointFullHd, width: 1920, height: 1080),
];

class DesignerViewport {
  const DesignerViewport({
    required this.name,
    required this.width,
    required this.height,
  });
  final String name;
  final double width;
  final double height;
}

class QaulBreakpoints {
  QaulBreakpoints._();

  static bool isMobile(BuildContext context) =>
      MediaQuery.sizeOf(context).width < kTabletBreakpoint;

  static bool isTablet(BuildContext context) {
    final w = MediaQuery.sizeOf(context).width;
    return w >= kTabletBreakpoint && w < kDesktopBreakpoint;
  }

  static bool isDesktop(BuildContext context) =>
      MediaQuery.sizeOf(context).width >= kDesktopBreakpoint;
}
