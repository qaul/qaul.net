part of 'widgets.dart';

class ResponsiveLayout extends StatelessWidget {
  const ResponsiveLayout({
    super.key,
    required this.mobileBody,
    this.tabletBody,
    this.desktopBody,
  });

  final Widget mobileBody;
  final Widget? tabletBody;
  final Widget? desktopBody;

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(
      builder: (context, constraints) {
        if (Responsiveness.isTablet(context)) {
          return tabletBody ?? mobileBody;
        } else if (Responsiveness.isDesktop(context)) {
          return desktopBody ?? tabletBody ?? mobileBody;
        }

        return mobileBody;
      },
    );
  }
}

class MaxWidthContainer extends StatelessWidget {
  const MaxWidthContainer({super.key, required this.child});

  final Widget child;

  @override
  Widget build(BuildContext context) {
    return Center(
      child: ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: Responsiveness.kMaxWidth),
        child: child,
      ),
    );
  }
}
