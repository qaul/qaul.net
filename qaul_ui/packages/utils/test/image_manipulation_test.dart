import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:image/image.dart' as image;
import 'package:utils/utils.dart';

void main() {
  late Directory temporaryDirectory;

  setUp(() async {
    temporaryDirectory = await Directory.systemTemp.createTemp(
      'qaul_image_manipulation_test_',
    );
  });

  tearDown(() => temporaryDirectory.delete(recursive: true));

  Future<File> writeImage(String name, List<int> bytes) async {
    final file = File('${temporaryDirectory.path}/$name');
    await file.writeAsBytes(bytes);
    return file;
  }

  group('processImage', () {
    for (final dimensions
        in <
          ({
            String name,
            int width,
            int height,
            int expectedWidth,
            int expectedHeight,
          })
        >[
          (
            name: 'landscape',
            width: 1500,
            height: 800,
            expectedWidth: 1024,
            expectedHeight: 546,
          ),
          (
            name: 'portrait',
            width: 800,
            height: 1500,
            expectedWidth: 546,
            expectedHeight: 1024,
          ),
          (
            name: 'square',
            width: 1200,
            height: 1200,
            expectedWidth: 1024,
            expectedHeight: 1024,
          ),
        ]) {
      test('resizes ${dimensions.name} JPEGs to 1024px or less', () async {
        final source = await writeImage(
          '${dimensions.name}.jpg',
          image.encodeJpg(
            patternedImage(dimensions.width, dimensions.height),
            quality: 100,
          ),
        );

        final processed = await processImage(
          source,
          temporaryDirectory: temporaryDirectory,
        );

        expect(processed, isNotNull);
        final output = image.decodeJpg(await processed!.readAsBytes())!;
        expect(output.width, dimensions.expectedWidth);
        expect(output.height, dimensions.expectedHeight);
        expect(await processed.length(), lessThan(await source.length()));
      });
    }

    test('encodes opaque PNGs as JPEGs', () async {
      final source = await writeImage(
        'opaque.png',
        image.encodePng(patternedImage(1200, 900), level: 1),
      );

      final processed = await processImage(
        source,
        temporaryDirectory: temporaryDirectory,
      );

      expect(processed, isNotNull);
      expect(
        image.findFormatForData(await processed!.readAsBytes()),
        image.ImageFormat.jpg,
      );
      expect(processed.path, endsWith('.png'));
      expect(await processed.length(), lessThan(await source.length()));
    });

    test('keeps transparent PNGs transparent', () async {
      final source = await writeImage(
        'transparent.png',
        image.encodePng(patternedImage(1200, 900, transparent: true), level: 1),
      );
      final input = image.decodePng(await source.readAsBytes())!;
      expect(input.numChannels, 4);
      expect(input.any((pixel) => pixel.a < 255), isTrue);

      final processed = await processImage(
        source,
        temporaryDirectory: temporaryDirectory,
      );

      expect(processed, isNotNull);
      final output = image.decodePng(await processed!.readAsBytes())!;
      expect(output.width, 1024);
      expect(output.height, 768);
      expect(output.any((pixel) => pixel.a < 255), isTrue);
      expect(await processed.length(), lessThan(await source.length()));
    });

    test('keeps animated GIF frames and timing', () async {
      final firstFrame = patternedImage(1600, 900);
      firstFrame
        ..frameDuration = 100
        ..loopCount = 3;
      final secondFrame = patternedImage(1600, 900, offset: 80)
        ..frameDuration = 300;
      firstFrame.addFrame(secondFrame);
      final source = await writeImage(
        'animated.gif',
        image.encodeGif(firstFrame),
      );

      final processed = await processImage(
        source,
        temporaryDirectory: temporaryDirectory,
      );

      expect(processed, isNotNull);
      final output = image.decodeGif(await processed!.readAsBytes())!;
      expect(output.width, 1024);
      expect(output.height, 576);
      expect(output.numFrames, 2);
      expect(output.loopCount, 3);
      expect(output.frames.map((frame) => frame.frameDuration), [100, 300]);
      expect(await processed.length(), lessThan(await source.length()));
    });

    test(
      'falls back to the original when compression grows the file',
      () async {
        final source = await writeImage(
          'small.png',
          image.encodePng(image.Image(width: 1, height: 1)),
        );

        final processed = await processImage(
          source,
          temporaryDirectory: temporaryDirectory,
        );

        expect(processed, isNull);
      },
    );

    test('leaves SVGs unchanged', () async {
      final source = await writeImage(
        'image.svg',
        '<svg xmlns="http://www.w3.org/2000/svg" width="2000" height="2000"/>'
            .codeUnits,
      );

      final processed = await processImage(
        source,
        temporaryDirectory: temporaryDirectory,
      );

      expect(processed, isNull);
    });
  });
}

image.Image patternedImage(
  int width,
  int height, {
  bool transparent = false,
  int offset = 0,
}) {
  final result = image.Image(
    width: width,
    height: height,
    numChannels: transparent ? 4 : 3,
  );
  for (var y = 0; y < height; y++) {
    for (var x = 0; x < width; x++) {
      var value = (x * 374761393 + y * 668265263 + offset) & 0xffffffff;
      value = (value ^ (value >> 13)) * 1274126177 & 0xffffffff;
      value ^= value >> 16;
      result.setPixelRgba(
        x,
        y,
        value & 0xff,
        (value >> 8) & 0xff,
        (value >> 16) & 0xff,
        transparent ? (x + y + offset) % 256 : 255,
      );
    }
  }
  return result;
}
