import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';
import 'package:mime/mime.dart';
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

  Future<AttachmentSaveOutcome> saveFile({
    required String filePath,
    required String fileName,
  });
}

enum AttachmentSaveOutcome {
  /// The attachment was written to the device.
  saved,

  /// A system export dialog took over; the user decides where it goes.
  handedToSystem,
}

/// Reduces a sender-chosen [fileName] to a bare file name, so it can't point
/// outside the folder it is saved into.
@visibleForTesting
String safeAttachmentFileName(String fileName, {required String filePath}) {
  final lastSegment = fileName.replaceAll('\\', '/').split('/').last;
  final safe = lastSegment
      .replaceAll(RegExp(r'[\x00-\x1f:*?"<>|]'), '_')
      .trim();
  if (safe.isEmpty || RegExp(r'^\.+$').hasMatch(safe)) {
    return path.basename(filePath);
  }
  return safe;
}

/// Where a saved attachment goes on mobile: the photo library or media
/// collection for its kind, or the user's downloads.
@visibleForTesting
String attachmentSaveKind(String fileName) {
  final mimeType = lookupMimeType(fileName) ?? '';
  for (final kind in ['image', 'video', 'audio']) {
    if (mimeType.startsWith('$kind/')) return kind;
  }
  return 'file';
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
  Future<AttachmentSaveOutcome> saveFile({
    required String filePath,
    required String fileName,
  }) async {
    final safeName = safeAttachmentFileName(fileName, filePath: filePath);
    if (Platform.isAndroid || Platform.isIOS) {
      final outcome = await _attachmentSaverChannel.invokeMethod<String>(
        'saveFile',
        {
          'path': filePath,
          'name': safeName,
          'kind': attachmentSaveKind(safeName),
        },
      );
      return outcome == 'handedToSystem'
          ? AttachmentSaveOutcome.handedToSystem
          : AttachmentSaveOutcome.saved;
    }

    final downloadsDirectory = await getDownloadsDirectory();
    if (downloadsDirectory == null) {
      throw const FileSystemException('Could not find the Downloads directory');
    }

    final source = File(filePath);
    if (!await source.exists()) {
      throw FileSystemException(
        'The attachment is no longer available',
        filePath,
      );
    }

    final destination = await _availableDestination(
      downloadsDirectory,
      safeName,
    );
    await source.copy(destination.path);
    return AttachmentSaveOutcome.saved;
  }

  Future<File> _availableDestination(
    Directory directory,
    String fileName,
  ) async {
    final extension = path.extension(fileName);
    final baseName = path.basenameWithoutExtension(fileName);

    for (var copyIndex = 0; ; copyIndex++) {
      final suffix = copyIndex == 0 ? '' : ' ($copyIndex)';
      final candidate = File(
        path.join(directory.path, '$baseName$suffix$extension'),
      );
      if (!await candidate.exists()) return candidate;
    }
  }
}
