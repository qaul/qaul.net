import 'package:hooks_riverpod/hooks_riverpod.dart';

import '../generated/services/chat/chatfile_rpc.pb.dart';

final fileHistoryEntitiesProvider =
    StateNotifierProvider<FileHistoryEntityNotifier, List<FileHistoryEntity>>(
  (ref) => FileHistoryEntityNotifier(files: const []),
);

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
}

class FileHistoryEntityNotifier extends StateNotifier<List<FileHistoryEntity>> {
  FileHistoryEntityNotifier({List<FileHistoryEntity>? files})
      : super(files ?? []);

  void add(FileHistoryEntity file) => state = [file, ...state];

  void update(FileHistoryEntity file) {
    assert(contains(file), 'State does not contain file $file');
    final filtered = state.where((r) => r != file);
    state = [file, ...filtered];
  }

  bool contains(FileHistoryEntity file) => state.contains(file);
}
