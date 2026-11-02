// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

import 'dart:ui' as ui;

import 'package:flutter/material.dart';
import 'package:flutter/rendering.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:qaul_components/qaul_components.dart';

/// Measures actual thumb pixels in the rightmost 10pt of the composer.
Future<Rect?> paintedThumb(WidgetTester tester) async {
  final boundaryFinder = find.byKey(const ValueKey('footer-capture'));
  final boundary = tester.renderObject<RenderRepaintBoundary>(boundaryFinder);
  final field = tester.getRect(find.byType(TextField)).shift(
    -tester.getTopLeft(boundaryFinder),
  );
  return tester.runAsync<Rect?>(() async {
    final image = await boundary.toImage(pixelRatio: 3);
    final data = (await image.toByteData(format: ui.ImageByteFormat.rawRgba))!;
    Rect? bounds;
    for (var y = (field.top * 3).ceil(); y < (field.bottom * 3).floor(); y++) {
      for (var x = ((field.right - 10) * 3).ceil();
          x < (field.right * 3).floor(); x++) {
        final offset = (y * image.width + x) * 4;
        if (data.getUint8(offset) == 0xB1 &&
            data.getUint8(offset + 1) == 0xB2 &&
            data.getUint8(offset + 2) == 0xB5 &&
            data.getUint8(offset + 3) == 255) {
          final pixel = Rect.fromLTWH(x / 3, y / 3, 1 / 3, 1 / 3);
          bounds = bounds?.expandToInclude(pixel) ?? pixel;
        }
      }
    }
    image.dispose();
    return bounds?.shift(-field.topLeft);
  });
}

void main() {
  for (final platform in [TargetPlatform.android, TargetPlatform.macOS]) {
    for (final brightness in Brightness.values) {
      testWidgets('composer scrollbar on ${platform.name} ${brightness.name}', (
        tester,
      ) async {
        tester.view.physicalSize = const Size(1170, 2532);
        tester.view.devicePixelRatio = 3;
        addTearDown(tester.view.resetPhysicalSize);
        addTearDown(tester.view.resetDevicePixelRatio);
        final controller = TextEditingController();
        addTearDown(controller.dispose);

        await tester.pumpWidget(MaterialApp(
          theme: ThemeData(brightness: brightness, platform: platform),
          home: Scaffold(
            body: Align(
              alignment: Alignment.bottomCenter,
              child: RepaintBoundary(
                key: const ValueKey('footer-capture'),
                child: ChatFooter(
                  placeholder: 'Secure private message',
                  controller: controller,
                  applyBottomSafeArea: false,
                ),
              ),
            ),
          ),
        ));

        for (final lines in [3, 5, 6, 10]) {
          controller.text = List.generate(lines, (i) => 'Line $i').join('\n');
          await tester.pumpAndSettle();
          final field = tester.widget<TextField>(find.byType(TextField));
          final scrollController = field.scrollController!;
          expect(field.maxLines, 5);
          expect(find.byType(RawScrollbar), findsOneWidget);
          expect(find.byType(Scrollbar), findsNothing);
          expect(tester.widget<RawScrollbar>(find.byType(RawScrollbar)).controller,
              same(scrollController));

          final thumb = await paintedThumb(tester);
          if (lines <= 5) {
            expect(scrollController.position.maxScrollExtent, 0);
            expect(thumb, isNull);
          } else {
            expect(scrollController.position.maxScrollExtent, greaterThan(0));
            expect(thumb, isNotNull);
            expect(thumb!.width, closeTo(5, 0.5));
            expect(tester.getSize(find.byType(TextField)).width - thumb.right,
                closeTo(3, 0.5));
            expect(thumb.top, closeTo(10, 1));

            scrollController.jumpTo(scrollController.position.maxScrollExtent);
            await tester.pumpAndSettle();
            final scrolledThumb = await paintedThumb(tester);
            expect(scrolledThumb!.top, greaterThan(thumb.top));
            await tester.pump(const Duration(seconds: 2));
            expect(await paintedThumb(tester), scrolledThumb);
            scrollController.jumpTo(0);
            await tester.pumpAndSettle();
          }
        }

        controller.text = 'Short again';
        await tester.pumpAndSettle();
        expect(await paintedThumb(tester), isNull);
        expect(tester.takeException(), isNull);
        await tester.pumpWidget(const SizedBox.shrink());
      });
    }
  }
}
