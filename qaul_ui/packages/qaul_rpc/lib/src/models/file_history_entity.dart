import 'package:hooks_riverpod/hooks_riverpod.dart';

import '../../qaul_rpc.dart';
import '../generated/services/chat/chatfile_rpc.pb.dart';

class FileHistoryEntity {
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

  final int id;
  final String name;
  final String extension;
  final int size;
  final String description;
  final DateTime time;
  final String senderId;
  final String groupId;

  factory FileHistoryEntity.fromRpcEntry(FileHistoryEntry file) {
    return FileHistoryEntity(
      id: file.fileId.toInt(),
      name: file.fileName,
      extension: file.fileExtension,
      size: file.fileSize,
      description: file.fileDescription,
      time: DateTime.fromMillisecondsSinceEpoch(file.time.toInt()),
      senderId: file.senderId,
      groupId: file.groupId,
    );
  }

  String filePath(Reader read) {
    var storagePath = read(libqaulLogsStoragePath)!.replaceAll('/logs', '');
    var uuid = read(defaultUserProvider)!.idBase58;

    return '$storagePath/$uuid/files/$id.$extension';
  }
}
