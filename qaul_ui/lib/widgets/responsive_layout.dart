part of 'widgets.dart';

class ResponsiveLayout extends StatelessWidget {
  const ResponsiveLayout({
    Key? key,
    required this.mobileBody,
    this.tabletBody,
    this.desktopBody,
  }) : super(key: key);

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
  const MaxWidthContainer({Key? key, required this.child}) : super(key: key);

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
