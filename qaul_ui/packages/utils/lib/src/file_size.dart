import 'package:flutter/widgets.dart';

String fileSize(num size) {
  const divider = 1024;

  if (size < divider) {
    return '$size B';
  }

  if (size < divider * divider) {
    return '${(size / divider).toStringAsFixed(2)} KB';
  }

  if (size < divider * divider * divider) {
    return '${(size / (divider * divider)).toStringAsFixed(2)} MB';
  }

  return '${(size / (divider * divider * divider)).toStringAsFixed(2)} GB';
}

/// Fits a file name on one line, preserving its extension and both stem ends.
///
/// If even the extension cannot fit, prioritizes its trailing characters.
/// Grapheme clusters are kept intact, including emoji and combining marks.
String shortenFileName(
  String name, {
  required double maxWidth,
  required TextStyle style,
  TextScaler textScaler = TextScaler.noScaling,
  TextDirection textDirection = TextDirection.ltr,
  Locale? locale,
}) {
  if (name.isEmpty || maxWidth <= 0 || maxWidth.isNaN) return '';
  final painter = TextPainter(
    textDirection: textDirection,
    textScaler: textScaler,
    locale: locale,
    maxLines: 1,
  );
  bool fits(String value) {
    painter.text = TextSpan(text: value, style: style);
    painter.layout();
    return painter.width <= maxWidth;
  }

  // Find the longest fitting candidate without repeatedly laying out every size.
  String longestFit(int maxCount, String Function(int) candidate) {
    var low = 0;
    var high = maxCount;
    var result = '';
    while (low <= high) {
      final count = (low + high) ~/ 2;
      final value = candidate(count);
      if (fits(value)) {
        result = value;
        low = count + 1;
      } else {
        high = count - 1;
      }
    }
    return result;
  }

  try {
    if (fits(name)) return name;
    final dot = name.lastIndexOf('.');
    final hasExtension = dot > 0 && dot < name.length - 1;
    final extension = hasExtension ? name.substring(dot) : '';
    final stem = (hasExtension ? name.substring(0, dot) : name).characters
        .toList();

    if (!fits('…$extension')) {
      if (extension.isNotEmpty && fits(extension)) return extension;
      final tail = extension.characters.toList();
      return longestFit(
        tail.length,
        (count) => '…${tail.skip(tail.length - count).join()}',
      );
    }

    return longestFit(stem.length - 1, (count) {
      final startCount = (count * 2 / 3).ceil();
      final endCount = count - startCount;
      return '${stem.take(startCount).join()}…'
          '${stem.skip(stem.length - endCount).join()}$extension';
    });
  } finally {
    painter.dispose();
  }
}
