// From https://github.com/vi-k/bubble
// Copyright 2019 Dunaev Victor (nashol)
// Licensed under BSD-2-Clause. See docs/licenses/THIRD_PARTY_LICENSES.md

import 'dart:math';

import 'package:flutter/material.dart';

enum BubbleNip { no, leftBottom, rightBottom }

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
    EdgeInsets? padding,
    EdgeInsets? margin,
  }) : color = color ?? Colors.white,
       elevation = elevation ?? 1,
       margin = margin ?? EdgeInsets.zero,
       _bubbleClipper = _BubbleClipper(
         radius: radius ?? const Radius.circular(6),
         nip: nip ?? BubbleNip.no,
         nipWidth: nipWidth ?? 8,
         nipHeight: nipHeight ?? 10,
         nipRadius: nipRadius ?? 1,
         padding: padding ?? const EdgeInsets.symmetric(horizontal: 8, vertical: 6),
       );

  final Widget? child;
  final Color color;
  final double elevation;
  final EdgeInsets margin;
  final _BubbleClipper _bubbleClipper;

  @override
  Widget build(BuildContext context) => Container(
    key: key,
    margin: margin,
    child: CustomPaint(
      painter: _BubblePainter(
        clipper: _bubbleClipper,
        color: color,
        elevation: elevation,
      ),
      child: Container(padding: _bubbleClipper.edgeInsets, child: child),
    ),
  );
}

class _BubbleClipper extends CustomClipper<Path> {
  _BubbleClipper({
    required this.radius,
    required this.nip,
    required this.nipWidth,
    required this.nipHeight,
    required this.nipRadius,
    required this.padding,
  }) : assert(nipWidth > 0),
       assert(nipHeight > 0),
       assert(nipRadius >= 0),
       assert(nipRadius <= nipWidth / 2 && nipRadius <= nipHeight / 2),
       super() {
    if (nip == BubbleNip.no) return;

    _startOffset = _endOffset = nipWidth;

    final k = nipHeight / nipWidth;
    final a = atan(k);

    _nipCX = (nipRadius + sqrt(nipRadius * nipRadius * (1 + k * k))) / k;
    final nipStickOffset = (_nipCX - nipRadius).floorToDouble();

    _nipCX -= nipStickOffset;
    _nipCY = nipRadius;
    _nipPX = _nipCX - nipRadius * sin(a);
    _nipPY = _nipCY + nipRadius * cos(a);
    _startOffset -= nipStickOffset;
    _endOffset -= nipStickOffset;
  }

  final Radius radius;
  final BubbleNip nip;
  final double nipHeight;
  final double nipWidth;
  final double nipRadius;
  final EdgeInsets padding;

  double _startOffset = 0;
  double _endOffset = 0;
  double _nipCX = 0;
  double _nipCY = 0;
  double _nipPX = 0;
  double _nipPY = 0;

  EdgeInsets get edgeInsets {
    switch (nip) {
      case BubbleNip.leftBottom:
        return EdgeInsets.only(
          left: _startOffset + padding.left,
          top: padding.top,
          right: _endOffset + padding.right,
          bottom: padding.bottom,
        );

      case BubbleNip.rightBottom:
        return EdgeInsets.only(
          left: _endOffset + padding.left,
          top: padding.top,
          right: _startOffset + padding.right,
          bottom: padding.bottom,
        );

      default:
        return EdgeInsets.only(
          left: _endOffset + padding.left,
          top: padding.top,
          right: _endOffset + padding.right,
          bottom: padding.bottom,
        );
    }
  }

  @override
  Path getClip(Size size) {
    var radiusX = radius.x;
    var radiusY = radius.y;
    final maxRadiusX = size.width / 2;
    final maxRadiusY = size.height / 2;

    if (radiusX > maxRadiusX) {
      radiusY *= maxRadiusX / radiusX;
      radiusX = maxRadiusX;
    }
    if (radiusY > maxRadiusY) {
      radiusX *= maxRadiusY / radiusY;
      radiusY = maxRadiusY;
    }

    var path = Path();

    switch (nip) {
      case BubbleNip.leftBottom:
        path.addRRect(
          RRect.fromLTRBR(
            _startOffset,
            0,
            size.width - _endOffset,
            size.height,
            radius,
          ),
        );
        break;

      case BubbleNip.rightBottom:
        path.addRRect(
          RRect.fromLTRBR(
            _endOffset,
            0,
            size.width - _startOffset,
            size.height,
            radius,
          ),
        );
        break;

      default:
        path.addRRect(
          RRect.fromLTRBR(
            _endOffset,
            0,
            size.width - _endOffset,
            size.height,
            radius,
          ),
        );
    }

    if (nip != BubbleNip.no) {
      switch (nip) {
        case BubbleNip.leftBottom:
          final path2 = Path()
            ..moveTo(_startOffset + radiusX, size.height)
            ..lineTo(_startOffset + radiusX, size.height - nipHeight)
            ..lineTo(_startOffset, size.height - nipHeight);

          if (nipRadius == 0) {
            path2.lineTo(0, size.height);
          } else {
            path2
              ..lineTo(_nipPX, size.height - _nipPY)
              ..arcToPoint(
                Offset(_nipCX, size.height),
                radius: Radius.circular(nipRadius),
                clockwise: false,
              );
          }

          path2.close();
          path = Path.combine(PathOperation.union, path, path2);
          break;

        case BubbleNip.rightBottom:
          final path2 = Path()
            ..moveTo(size.width - _startOffset - radiusX, size.height)
            ..lineTo(
              size.width - _startOffset - radiusX,
              size.height - nipHeight,
            )
            ..lineTo(size.width - _startOffset, size.height - nipHeight);

          if (nipRadius == 0) {
            path2.lineTo(size.width, size.height);
          } else {
            path2
              ..lineTo(size.width - _nipPX, size.height - _nipPY)
              ..arcToPoint(
                Offset(size.width - _nipCX, size.height),
                radius: Radius.circular(nipRadius),
              );
          }

          path2.close();
          path = Path.combine(PathOperation.union, path, path2);
          break;

        default:
      }
    }

    return path;
  }

  @override
  bool shouldReclip(_BubbleClipper oldClipper) => false;
}

class _BubblePainter extends CustomPainter {
  _BubblePainter({
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
  bool shouldRepaint(covariant _BubblePainter oldDelegate) => false;
}
