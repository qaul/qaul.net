part of 'widgets.dart';

class IconButtonFactory extends StatelessWidget {
  const IconButtonFactory({
    super.key,
    this.onPressed,
    this.icon = Icons.arrow_back_ios_rounded,
  });
  final VoidCallback? onPressed;
  final IconData icon;

  factory IconButtonFactory.close({Key? key, VoidCallback? onPressed}) {
    return IconButtonFactory(onPressed: onPressed, icon: Icons.close_rounded);
  }

  @override
  Widget build(BuildContext context) {
    final l18ns = AppLocalizations.of(context)!;
    return IconButton(
      splashRadius: 24,
      tooltip: l18ns.backButtonTooltip,
      icon: Icon(icon),
      onPressed:
          onPressed != null ? onPressed! : () => Navigator.maybePop(context),
    );
  }
}

class QaulButton extends StatelessWidget {
  const QaulButton({
    super.key,
    required this.label,
    this.style,
    this.onPressed,
    this.backgroundColor,
  });

  final String label;
  final TextStyle? style;
  final VoidCallback? onPressed;
  final Color? backgroundColor;

  @override
  Widget build(BuildContext context) {
    return OutlinedButton(
      onPressed: onPressed ?? () {},
      style: ButtonStyle(
        backgroundColor: WidgetStateProperty.all(
          backgroundColor ?? Colors.transparent,
        ),
      ),
      child: Padding(
        padding: const EdgeInsets.all(10.0),
        child: Text(
          label,
          style: style ?? const TextStyle(fontSize: 16),
        ),
      ),
    );
  }
}
