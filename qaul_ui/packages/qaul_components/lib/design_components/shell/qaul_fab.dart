import 'package:flutter/material.dart';
import 'package:flutter_svg/svg.dart';

class QaulFAB extends StatelessWidget {
  const QaulFAB({
    super.key,
    required this.svgAsset,
    required this.onPressed,
    this.heroTag,
    this.tooltip,
    this.size = 52,
    this.package,
  });

  final String svgAsset;
  final VoidCallback onPressed;
  final String? heroTag;
  final String? tooltip;
  final int size;

  /// The package to load [svgAsset] from.
  ///
  /// Pass `'qaul_components'` when the SVG lives in the component library.
  /// Omit (or pass `null`) when the asset belongs to the host app.
  final String? package;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final iconColor = theme.iconTheme.color ?? theme.colorScheme.onSurface;
    final fillColor = theme.brightness == Brightness.dark
        ? theme.colorScheme.surface
        : Colors.white;

    return FloatingActionButton.large(
      elevation: 0,
      heroTag: heroTag,
      backgroundColor: fillColor,
      tooltip: tooltip,
      shape: CircleBorder(side: BorderSide(color: iconColor)),
      onPressed: onPressed,
      child: SvgPicture.asset(
        svgAsset,
        package: package,
        width: 48,
        height: 48,
        colorFilter: ColorFilter.mode(iconColor, BlendMode.srcATop),
      ),
    );
  }
}
