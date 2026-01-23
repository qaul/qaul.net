library;

import 'dart:math';

import 'package:flutter/material.dart';

part 'bubble_clipper.dart';
part 'bubble_edges.dart';
part 'bubble_painter.dart';

enum BubbleNip {
  no,
  leftBottom,
  rightBottom,
}

class Bubble extends StatelessWidget {
  Bubble({
    super.key,
    this.child,
    Radius? radius,
    BubbleNip? nip,
    double? nipWidth,
    double? nipHeight,
    double? nipRadius,
    Color? color,
    double? elevation,
    BubbleEdges? padding,
    BubbleEdges? margin,
  })  : color = color ?? Colors.white,
        elevation = elevation ?? 1,
        margin = EdgeInsets.only(
          left: margin?.left ?? 0,
          top: margin?.top ?? 0,
          right: margin?.right ?? 0,
          bottom: margin?.bottom ?? 0,
        ),
        bubbleClipper = BubbleClipper(
          radius: radius ?? const Radius.circular(6),
          nip: nip ?? BubbleNip.no,
          nipWidth: nipWidth ?? 8,
          nipHeight: nipHeight ?? 10,
          nipRadius: nipRadius ?? 1,
          padding: EdgeInsets.only(
            left: padding?.left ?? 8,
            top: padding?.top ?? 6,
            right: padding?.right ?? 8,
            bottom: padding?.bottom ?? 6,
          ),
        );

  final Widget? child;
  final Color color;
  final double elevation;
  final EdgeInsets margin;
  final BubbleClipper bubbleClipper;

  @override
  Widget build(BuildContext context) => Container(
        key: key,
        margin: margin,
        child: CustomPaint(
          painter: BubblePainter(
            clipper: bubbleClipper,
            color: color,
            elevation: elevation,
          ),
          child: Container(
            padding: bubbleClipper.edgeInsets,
            child: child,
          ),
        ),
      );
}
