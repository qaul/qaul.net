part of 'widgets.dart';

class ResponsiveScaffold extends StatelessWidget {
  const ResponsiveScaffold({
    Key? key,
    required this.body,
    this.title,
    this.icon,
    this.hasAppBar = true,
  }) : super(key: key);
  final Widget body;
  final bool hasAppBar;
  final String? title;
  final IconData? icon;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: !hasAppBar ? null: AppBar(
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
            return SizedBox(
                width: constraints.constrainWidth(1200), child: body);
          },
        ),
      ),
    );
  }
}
