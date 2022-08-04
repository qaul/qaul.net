import 'package:hooks_riverpod/hooks_riverpod.dart';

import '../models/file_history_entity.dart';

class FileHistoryEntityNotifier extends StateNotifier<List<FileHistoryEntity>> {
  FileHistoryEntityNotifier({List<FileHistoryEntity>? files}) : super(files ?? []);

  void add(FileHistoryEntity file) => state = [file, ...state];

  void update(FileHistoryEntity file) {
    assert(contains(file), 'State does not contain file $file');
    final filtered = state.where((r) => r != file);
    state = [file, ...filtered];
  }

  bool contains(FileHistoryEntity file) => state.contains(file);
}
