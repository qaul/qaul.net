part of 'widgets.dart';

class ResponsiveScaffold extends StatelessWidget {
  const ResponsiveScaffold({
    super.key,
    required this.body,
    this.title,
    this.icon,
    this.titleIcon,
    this.actions,
    this.backgroundColor,
    this.bodyAlignment = Alignment.center,
    this.scrollHorizontalPadding = 20,
    this.scrollTopPadding = 20,
    this.hasAppBar = true,
    this.wrapWithScrollable = false,
  });
  final Widget body;
  final bool hasAppBar;
  final String? title;
  final IconData? icon;
  final Widget? titleIcon;
  final List<Widget>? actions;
  final Color? backgroundColor;
  final AlignmentGeometry bodyAlignment;
  final double scrollHorizontalPadding;
  final double scrollTopPadding;
  final bool wrapWithScrollable;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: backgroundColor,
      appBar: !hasAppBar
          ? null
          : qc.QaulPageHeader(
              title: title ?? '',
              leadingVisual: titleIcon ??
                  (icon == null
                      ? null
                      : Icon(
                          icon,
                          color: qc.kQaulPageHeaderControlColor,
                        )),
              actions: actions ?? const [],
            ),
      body: Align(
        alignment: bodyAlignment,
        child: LayoutBuilder(
          builder: (context, constraints) {
            final width = constraints.constrainWidth(1200);

            if (!wrapWithScrollable) return SizedBox(width: width, child: body);

            final maxWidth = constraints.maxWidth;
            final horizontalPadding =
                EdgeInsets.symmetric(horizontal: (maxWidth - width) / 2);

            final viewPadding = MediaQuery.of(context)
                .viewPadding
                .copyWith(
                  left: scrollHorizontalPadding,
                  right: scrollHorizontalPadding,
                  top: scrollTopPadding,
                )
                .add(horizontalPadding);

            return SingleChildScrollView(padding: viewPadding, child: body);
          },
        ),
      ),
    );
  }
}
