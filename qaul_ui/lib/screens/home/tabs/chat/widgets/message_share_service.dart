import 'dart:io';

import 'package:flutter/services.dart';
import 'package:path/path.dart' as path;
import 'package:path_provider/path_provider.dart';
import 'package:share_plus/share_plus.dart';

const _attachmentClipboardChannel = MethodChannel('qaul/attachment_clipboard');
const _attachmentSaverChannel = MethodChannel('qaul/attachment_saver');

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

  Future<void> copyFile({
    required String filePath,
    required String fileName,
  });

  Future<void> saveFile({
    required String filePath,
    required String fileName,
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

  @override
  Future<void> copyFile({
    required String filePath,
    required String fileName,
  }) {
    return _attachmentClipboardChannel.invokeMethod<void>('copyFile', {
      'path': filePath,
      'name': fileName,
    });
  }

  @override
  Future<void> saveFile({
    required String filePath,
    required String fileName,
  }) async {
    if (Platform.isAndroid || Platform.isIOS) {
      await _attachmentSaverChannel.invokeMethod<void>('saveFile', {
        'path': filePath,
        'name': fileName,
      });
      return;
    }

    final downloadsDirectory = await getDownloadsDirectory();
    if (downloadsDirectory == null) {
      throw FileSystemException('Could not find the Downloads directory');
    }

    final source = File(filePath);
    if (!await source.exists()) {
      throw FileSystemException('The attachment is no longer available', filePath);
    }

    final destination = await _availableDestination(
      downloadsDirectory,
      fileName.isEmpty ? path.basename(filePath) : fileName,
    );
    await source.copy(destination.path);
  }

  Future<File> _availableDestination(
    Directory directory,
    String fileName,
  ) async {
    final safeName = path.basename(fileName);
    final extension = path.extension(safeName);
    final baseName = path.basenameWithoutExtension(safeName);

    for (var copyIndex = 0; ; copyIndex++) {
      final suffix = copyIndex == 0 ? '' : ' ($copyIndex)';
      final candidate = File(
        path.join(directory.path, '$baseName$suffix$extension'),
      );
      if (!await candidate.exists()) return candidate;
    }
  }
}
