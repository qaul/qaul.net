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
/// Opaque images are encoded as JPEG at quality 60. Transparent images remain
/// lossless RGBA PNGs, and GIFs retain their frames and timing.
/// SVGs, undecodable files, files that would grow, and images that fail to
/// process return null, so callers send the original.
Future<File?> processImage(File file, {Directory? temporaryDirectory}) async {
  assert(isImage(file.path));
  final _ProcessedImage? processed;
  try {
    final path = file.path;
    processed = await Isolate.run(() => _resizeAndCompressImage(path));
  } catch (_) {
    return null;
  }
  if (processed == null) return null;

  final tempDir = temporaryDirectory ?? await getTemporaryDirectory();
  final name = 'img_${DateTime.now().microsecondsSinceEpoch}_'
      '${math.Random.secure().nextInt(1 << 32)}${processed.extension}';
  return File('${tempDir.path}/$name')..writeAsBytesSync(processed.bytes);
}

class _ProcessedImage {
  const _ProcessedImage(this.bytes, this.extension);

  final List<int> bytes;
  final String extension;
}

_ProcessedImage? _resizeAndCompressImage(String path) {
  final sourceBytes = File(path).readAsBytesSync();
  final imageFormat = findFormatForData(sourceBytes);
  // decodeImage identifies the format and uses the appropriate decoder.
  var image = decodeImage(sourceBytes);
  if (image == null) return null;

  if (image.width > 1024 || image.height > 1024) {
    image = image.width >= image.height
        ? copyResize(image, width: 1024)
        : copyResize(image, height: 1024);
  }

  final _ProcessedImage processed;
  if (imageFormat == ImageFormat.gif) {
    processed = _ProcessedImage(encodeGif(image), '.gif');
  } else if (_hasTransparency(image)) {
    processed = _ProcessedImage(
      // Palette quantization only considers RGB. Pixels that have the same
      // RGB value but different alpha values would therefore share a palette
      // entry and have their alpha mixed. Keep RGBA pixels lossless instead.
      encodePng(image.convert(format: Format.uint8, numChannels: 4), level: 9),
      '.png',
    );
  } else {
    processed = _ProcessedImage(encodeJpg(image, quality: 60), '.jpg');
  }

  return processed.bytes.length < sourceBytes.length ? processed : null;
}

bool _hasTransparency(Image image) {
  if (!image.hasAlpha && image.palette?.numChannels != 4) return false;
  return image.any((pixel) => pixel.aNormalized < 1);
}
