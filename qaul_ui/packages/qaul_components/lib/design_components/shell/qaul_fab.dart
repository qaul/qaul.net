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
    return FloatingActionButton.large(
      elevation: 0,
      heroTag: heroTag,
      backgroundColor: Colors.white,
      tooltip: tooltip,
      shape: const CircleBorder(side: BorderSide(color: Colors.black)),
      onPressed: onPressed,
      child: SvgPicture.asset(
        svgAsset,
        package: package,
        width: 48,
        height: 48,
        colorFilter: ColorFilter.mode(Colors.grey.shade600, BlendMode.srcATop),
      ),
    );
  }
}
