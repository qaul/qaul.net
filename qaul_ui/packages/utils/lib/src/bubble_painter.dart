part of 'bubble.dart';

class BubblePainter extends CustomPainter {
  BubblePainter({
    required this.clipper,
    required Color color,
    required this.elevation,
  }) : _fillPaint = Paint()
          ..color = color
          ..style = PaintingStyle.fill;

  final CustomClipper<Path> clipper;
  final double elevation;

  final Paint _fillPaint;

  @override
  void paint(Canvas canvas, Size size) {
    final clip = clipper.getClip(size);

    if (elevation != 0.0) {
      canvas.drawShadow(clip, Colors.black, elevation, false);
    }

    canvas.drawPath(clip, _fillPaint);
  }

  @override
  bool shouldRepaint(covariant BubblePainter oldDelegate) => false;
}
