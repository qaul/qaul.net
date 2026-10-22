// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

import 'package:flutter/widgets.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:utils/utils.dart';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();
  const style = TextStyle(fontSize: 16);

  double width(String text, {TextScaler scaler = TextScaler.noScaling}) {
    final painter = TextPainter(
      text: TextSpan(text: text, style: style),
      textDirection: TextDirection.ltr,
      textScaler: scaler,
    )..layout();
    final result = painter.width;
    painter.dispose();
    return result;
  }

  test('leaves short names unchanged, including an exact fit', () {
    for (final name in [
      'report.pdf',
      'README',
      '.gitignore',
      'archive.tar.gz',
      'a.',
    ]) {
      expect(shortenFileName(name, maxWidth: width(name), style: style), name);
      expect(
        shortenFileName(name, maxWidth: double.infinity, style: style),
        name,
      );
    }
  });

  test(
    'fits 120-character names while retaining the extension and both ends',
    () {
      final name = '${List.filled(116, 'a').join()}.pdf';
      for (final available in [100.0, 180.0, 320.0]) {
        final result = shortenFileName(name, maxWidth: available, style: style);
        expect(result, contains('…'));
        expect(result, startsWith('a'));
        expect(result, endsWith('.pdf'));
        if (available >= 180) expect(result, endsWith('a.pdf'));
        expect(width(result), lessThanOrEqualTo(available));
        expect(result.length, lessThan(name.length));
      }
    },
  );

  test('uses actual glyph widths and text scale', () {
    final wide = '${List.filled(60, 'W').join()}.pdf';
    final narrow = '${List.filled(60, 'i').join()}.pdf';
    final wideStyle = const TextStyle(fontSize: 16, fontFamily: 'Roboto');
    for (final name in [wide, narrow]) {
      for (final scale in [0.6, 1.0, 2.5]) {
        final scaler = TextScaler.linear(scale);
        final result = shortenFileName(
          name,
          maxWidth: 240,
          style: wideStyle,
          textScaler: scaler,
        );
        final painter = TextPainter(
          text: TextSpan(text: result, style: wideStyle),
          textDirection: TextDirection.ltr,
          textScaler: scaler,
        )..layout();
        expect(result, endsWith('.pdf'));
        expect(painter.width, lessThanOrEqualTo(240));
        painter.dispose();
      }
    }
  });

  test('does not split emoji or combining character clusters', () {
    for (final cluster in ['👩🏽‍💻', 'e\u0301']) {
      final name = '${List.filled(30, cluster).join()}.pdf';
      final result = shortenFileName(name, maxWidth: 240, style: style);
      expect(result, endsWith('.pdf'));
      final remaining = result
          .substring(0, result.length - 4)
          .replaceAll('…', '');
      expect(remaining.characters.every((value) => value == cluster), isTrue);
      expect(width(result), lessThanOrEqualTo(240));
    }
  });

  test('handles names without an extension and hidden files', () {
    for (final name in ['README' * 20, '.hidden' * 20]) {
      final result = shortenFileName(name, maxWidth: 180, style: style);
      expect(result, contains('…'));
      expect(width(result), lessThanOrEqualTo(180));
    }
  });

  test('handles empty or extremely narrow bounds without overflowing', () {
    expect(shortenFileName('', maxWidth: 100, style: style), '');
    for (final available in [0.0, 1.0, 20.0, 50.0]) {
      final result = shortenFileName(
        'long_name.pdf',
        maxWidth: available,
        style: style,
      );
      expect(width(result), lessThanOrEqualTo(available));
    }
    final extensionWidth = width('.pdf');
    expect(
      shortenFileName('long_name.pdf', maxWidth: extensionWidth, style: style),
      '.pdf',
    );
  });
}
