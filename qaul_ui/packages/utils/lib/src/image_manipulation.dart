import 'dart:io';
import 'dart:math' as math;

import 'package:image/image.dart';
import 'package:mime/mime.dart';
import 'package:path_provider/path_provider.dart';


bool isImage(String filePath) {
  String? mimeStr = lookupMimeType(filePath);
  return (mimeStr != null && RegExp('image/.*').hasMatch(mimeStr));
}

Future<File?> compressImage(File file, {int quality = 70}) async {
  assert(isImage(file.path));
  final tempDir = await getTemporaryDirectory();
  final path = tempDir.path;
  int rand = math.Random().nextInt(10000);

  var image = decodeImage(file.readAsBytesSync());
  if (image == null) return null;

  return File('$path/img_$rand.jpg')
    ..writeAsBytesSync(encodeJpg(image, quality: quality.clamp(0, 100)));
}
