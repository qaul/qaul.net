part of 'bubble.dart';

/// Class BubbleEdges is an analog of EdgeInsets, but default values are null.
class BubbleEdges {
  const BubbleEdges.fromLTRB(
    this.left,
    this.top,
    this.right,
    this.bottom,
  );

  const BubbleEdges.all(double? value)
      : left = value,
        top = value,
        right = value,
        bottom = value;

  const BubbleEdges.only({
    this.left,
    this.top,
    this.right,
    this.bottom,
  });

  const BubbleEdges.symmetric({
    double? vertical,
    double? horizontal,
  })  : left = horizontal,
        top = vertical,
        right = horizontal,
        bottom = vertical;

  final double? left;
  final double? top;
  final double? right;
  final double? bottom;

  EdgeInsets get edgeInsets =>
      EdgeInsets.fromLTRB(left ?? 0, top ?? 0, right ?? 0, bottom ?? 0);

  @override
  String toString() => 'BubbleEdges($left, $top, $right, $bottom)';
}
