import 'dart:io';
import 'dart:isolate';
import 'dart:math' as math;

import 'package:image/image.dart';
import 'package:mime/mime.dart';
import 'package:path_provider/path_provider.dart';

bool isImage(String filePath) {
  String? mimeStr = lookupMimeType(filePath);
  if (mimeStr == null) return false;
  return (RegExp('image/.*').hasMatch(mimeStr));
}

/// Resizes images so their longest side is at most 1024 pixels and returns a
/// smaller temporary file when compression saves space.
///
/// JPEGs and opaque images are encoded as JPEG at quality 60. Transparent
/// PNGs stay PNGs with a 256-colour palette, and GIFs retain their frames and
/// timing. SVGs, undecodable files, and files that would grow are unchanged.
Future<File?> processImage(File file, {Directory? temporaryDirectory}) async {
  assert(isImage(file.path));
  ReceivePort receivePort = ReceivePort();
  await Isolate.spawn(
    _resizeAndCompressImage,
    _ProcessParam(file, receivePort.sendPort),
  );

  // Get the processed image from the isolate.
  List<int>? imageBytes = await receivePort.first;

  if (imageBytes == null) return null;

  final tempDir = temporaryDirectory ?? await getTemporaryDirectory();
  final path = tempDir.path;
  int rand = math.Random().nextInt(10000);
  final extension = _fileExtension(file.path);

  return File('$path/img_$rand$extension')..writeAsBytesSync(imageBytes);
}

class _ProcessParam {
  _ProcessParam(this.file, this.sendPort);

  final File file;
  final SendPort sendPort;
}

void _resizeAndCompressImage(_ProcessParam param) {
  final sourceBytes = param.file.readAsBytesSync();
  final imageFormat = findFormatForData(sourceBytes);
  // decodeImage identifies the format and uses the appropriate decoder.
  var image = decodeImage(sourceBytes);
  if (image == null) {
    param.sendPort.send(null);
    return;
  }

  if (image.width > 1024 || image.height > 1024) {
    image = image.width >= image.height
        ? copyResize(image, width: 1024)
        : copyResize(image, height: 1024);
  }

  final List<int> compressedBytes;
  if (imageFormat == ImageFormat.gif) {
    compressedBytes = encodeGif(image);
  } else if (imageFormat == ImageFormat.png && _hasTransparency(image)) {
    compressedBytes = encodePng(_quantizeWithTransparency(image), level: 9);
  } else {
    compressedBytes = encodeJpg(image, quality: 60);
  }

  param.sendPort.send(
    compressedBytes.length < sourceBytes.length ? compressedBytes : null,
  );
}

bool _hasTransparency(Image image) {
  if (image.numChannels != 4) return false;
  return image.any((pixel) => pixel.a < 255);
}

Image _quantizeWithTransparency(Image source) {
  final indexedImage = quantize(source);
  final sourcePalette = indexedImage.palette!;
  final palette = PaletteUint8(sourcePalette.numColors, 4);
  final alphaTotals = List<int>.filled(sourcePalette.numColors, 0);
  final pixelsByIndex = List<int>.filled(sourcePalette.numColors, 0);

  for (var y = 0; y < source.height; y++) {
    for (var x = 0; x < source.width; x++) {
      final index = indexedImage.getPixelIndex(x, y);
      alphaTotals[index] += source.getPixel(x, y).a.toInt();
      pixelsByIndex[index]++;
    }
  }

  for (var index = 0; index < sourcePalette.numColors; index++) {
    palette.setRgba(
      index,
      sourcePalette.getRed(index),
      sourcePalette.getGreen(index),
      sourcePalette.getBlue(index),
      pixelsByIndex[index] == 0
          ? 255
          : (alphaTotals[index] / pixelsByIndex[index]).round(),
    );
  }

  final result = Image(
    width: source.width,
    height: source.height,
    numChannels: 1,
    palette: palette,
  );
  for (var y = 0; y < source.height; y++) {
    for (var x = 0; x < source.width; x++) {
      result.setPixelIndex(x, y, indexedImage.getPixelIndex(x, y));
    }
  }
  return result;
}

String _fileExtension(String path) {
  final fileName = Uri.file(path).pathSegments.last;
  final dotIndex = fileName.lastIndexOf('.');
  return dotIndex == -1 ? '' : fileName.substring(dotIndex);
}
