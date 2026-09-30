import 'dart:ui' show Rect;

import 'package:share_plus/share_plus.dart';

abstract interface class MessageShareService {
  Future<void> shareText({
    required String text,
    required Rect? sharePositionOrigin,
  });

  Future<void> shareFile({
    required String filePath,
    required String fileName,
    required Rect? sharePositionOrigin,
  });
}

class SystemMessageShareService implements MessageShareService {
  const SystemMessageShareService();

  @override
  Future<void> shareText({
    required String text,
    required Rect? sharePositionOrigin,
  }) async {
    await SharePlus.instance.share(
      ShareParams(text: text, sharePositionOrigin: sharePositionOrigin),
    );
  }

  @override
  Future<void> shareFile({
    required String filePath,
    required String fileName,
    required Rect? sharePositionOrigin,
  }) async {
    await SharePlus.instance.share(
      ShareParams(
        files: [XFile(filePath)],
        fileNameOverrides: [fileName],
        sharePositionOrigin: sharePositionOrigin,
      ),
    );
  }
}
