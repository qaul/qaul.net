import 'dart:io';

import 'package:flutter/cupertino.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

import '../qaul_rpc.dart';

Future<String?> findFolderWithFilesOfExtension(
    Directory root, String extension) async {
  for (final FileSystemEntity entity in root.listSync()) {
    if (entity is File && entity.path.endsWith(extension)) {
      return root.path;
    } else if (entity is Directory) {
      final folder = await findFolderWithFilesOfExtension(entity, extension);
      if (folder != null) return folder;
    }
  }
  return null;
}

mixin FilePathResolverMixin {
  @protected
  String getFilePath(
    WidgetRef ref, {
    required String id,
    required String extension,
  }) {
    var storagePath = ref.read(libqaulLogsStoragePath)!.replaceAll('/logs', '');
    if (Platform.isWindows) {
      storagePath = ref.read(libqaulLogsStoragePath)!.replaceAll('\\logs', '');
    }
    var uuid = ref.read(defaultUserProvider)!.idBase58;

    final filePath = '$storagePath/$uuid/files/$id.$extension';
    if (Platform.isWindows) {
      return filePath.replaceAll('/', '\\');
    }
    return filePath;
  }
}
