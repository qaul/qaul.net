part of 'widgets.dart';

class ResponsiveScaffold extends StatelessWidget {
  const ResponsiveScaffold({
    super.key,
    required this.body,
    this.title,
    this.icon,
    this.hasAppBar = true,
    this.wrapWithScrollable = false,
  });
  final Widget body;
  final bool hasAppBar;
  final String? title;
  final IconData? icon;
  final bool wrapWithScrollable;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: !hasAppBar
          ? null
          : AppBar(
              leading: const IconButtonFactory(),
              title: Row(
                children: [
                  icon == null ? const SizedBox.shrink() : Icon(icon!),
                  const SizedBox(width: 8),
                  title == null ? const SizedBox.shrink() : Text(title!),
                ],
              ),
            ),
      body: Center(
        child: LayoutBuilder(
          builder: (context, constraints) {
            final width = constraints.constrainWidth(1200);

            if (!wrapWithScrollable) return SizedBox(width: width, child: body);

            final maxWidth = constraints.maxWidth;
            final horizontalPadding =
                EdgeInsets.symmetric(horizontal: (maxWidth - width) / 2);

            final viewPadding = MediaQuery.of(context)
                .viewPadding
                .copyWith(left: 20, right: 20, top: 20)
                .add(horizontalPadding);

            return SingleChildScrollView(padding: viewPadding, child: body);
          },
        ),
      ),
    );
  }
}
