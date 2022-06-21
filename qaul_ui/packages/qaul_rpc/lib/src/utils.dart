import 'dart:io';

Future<String?> findFolderWithFilesOfExtension(Directory root, String extension) async {
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
