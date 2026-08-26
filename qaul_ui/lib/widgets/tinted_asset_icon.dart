part of 'widgets.dart';

class QaulTintedAssetIcon extends StatelessWidget {
  const QaulTintedAssetIcon({
    super.key,
    required this.assetName,
    required this.width,
    required this.height,
    this.color,
    this.svgPadding = 0,
  });

  final String assetName;
  final double width;
  final double height;
  final Color? color;
  final double svgPadding;

  @override
  Widget build(BuildContext context) {
    final isSvg = assetName.toLowerCase().endsWith('.svg');
    final icon = isSvg ? _buildSvgIcon() : _buildPngIcon();

    return SizedBox(
      width: width,
      height: height,
      child: isSvg && svgPadding > 0
          ? Padding(padding: EdgeInsets.all(svgPadding), child: icon)
          : icon,
    );
  }

  Widget _buildSvgIcon() {
    return SvgPicture.asset(
      assetName,
      width: width - svgPadding * 2,
      height: height - svgPadding * 2,
      colorFilter: color == null
          ? null
          : ColorFilter.mode(color!, BlendMode.srcIn),
    );
  }

  Widget _buildPngIcon() {
    final image = Image.asset(
      assetName,
      width: width,
      height: height,
      fit: BoxFit.contain,
    );

    if (color == null) return image;

    return ColorFiltered(
      colorFilter: ColorFilter.mode(color!, BlendMode.srcIn),
      child: image,
    );
  }
}
