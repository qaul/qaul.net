part of 'bubble.dart';

class BubbleStyle {
  const BubbleStyle({
    this.radius,
    this.showNip,
    this.nip,
    this.nipWidth,
    this.nipHeight,
    this.nipOffset,
    this.nipRadius,
    this.stick,
    this.color,
    this.borderColor,
    this.borderWidth,
    this.borderUp,
    this.elevation,
    this.shadowColor,
    this.padding,
    this.margin,
    this.alignment,
  });

  final Radius? radius;
  final bool? showNip;
  final BubbleNip? nip;
  final double? nipHeight;
  final double? nipWidth;
  final double? nipOffset;
  final double? nipRadius;
  final bool? stick;
  final Color? color;
  final Color? borderColor;
  final double? borderWidth;
  final bool? borderUp;
  final double? elevation;
  final Color? shadowColor;
  final BubbleEdges? padding;
  final BubbleEdges? margin;
  final AlignmentGeometry? alignment;
}
