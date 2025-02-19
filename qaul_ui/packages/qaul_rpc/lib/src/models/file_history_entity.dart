import 'package:hooks_riverpod/hooks_riverpod.dart';

import '../generated/services/chat/chatfile_rpc.pb.dart';
import '../utils.dart';

class FileHistoryEntity with FilePathResolverMixin {
  FileHistoryEntity({
    required this.id,
    required this.name,
    required this.extension,
    required this.size,
    required this.description,
    required this.time,
    required this.senderId,
    required this.groupId,
  });

  final String id;
  final String name;
  final String extension;
  final int size;
  final String description;
  final DateTime time;
  final String senderId;
  final String groupId;

  factory FileHistoryEntity.fromRpcEntry(FileHistoryEntry file) {
    return FileHistoryEntity(
      id: file.fileId.toStringUnsigned(),
      name: file.fileName,
      extension: file.fileExtension,
      size: file.fileSize,
      description: file.fileDescription,
      time: DateTime.fromMillisecondsSinceEpoch(file.time.toInt()),
      senderId: file.senderId,
      groupId: file.groupId,
    );
  }

  String filePath(WidgetRef ref) =>
      getFilePath(ref, id: id, extension: extension);
}
