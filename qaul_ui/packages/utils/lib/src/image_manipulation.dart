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

/// Resizes image to Full HD (maintaining aspect ratio) if width is greater than
/// 1920, and compresses it to a JPEG 85% quality.
Future<File?> processImage(File file) async {
  assert(isImage(file.path));
  ReceivePort receivePort = ReceivePort();
  await Isolate.spawn(_resizeAndCompressImage, _ProcessParam(file, receivePort.sendPort));

  // Get the processed image from the isolate.
  List<int>? imageBytes = await receivePort.first;

  if (imageBytes == null) return null;

  final tempDir = await getTemporaryDirectory();
  final path = tempDir.path;
  int rand = math.Random().nextInt(10000);

  return File('$path/img_$rand.jpg')..writeAsBytesSync(imageBytes);
}

class _ProcessParam {
  _ProcessParam(this.file, this.sendPort);

  final File file;
  final SendPort sendPort;
}

void _resizeAndCompressImage(_ProcessParam param) {
  // Read an image from file.
  // decodeImage will identify the format of the image and use the appropriate
  // decoder.
  var image = decodeImage(param.file.readAsBytesSync());
  if (image == null) {
    param.sendPort.send(null);
    return;
  }

  if (image.width > 1920) {
    // Resize the image to Full HD 1920x? (maintaining the aspect ratio).
    image = copyResize(image, width: 1920);
  }

  // Send back encoded jpg image
  param.sendPort.send(encodeJpg(image, quality: 85));
}
