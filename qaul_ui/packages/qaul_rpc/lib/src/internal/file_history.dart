import 'package:hooks_riverpod/hooks_riverpod.dart';

import '../../qaul_rpc.dart';

final fileHistoryEntitiesProvider =
NotifierProvider<FileHistoryEntityNotifier, List<FileHistoryEntity>>(
      FileHistoryEntityNotifier.new,
);

class FileHistoryEntityNotifier extends Notifier<List<FileHistoryEntity>> {
  @override
  List<FileHistoryEntity> build() => [];

  void add(FileHistoryEntity file) => state = [file, ...state];

  void update(FileHistoryEntity file) {
    assert(contains(file), 'State does not contain file $file');
    final filtered = state.where((r) => r != file);
    state = [file, ...filtered];
  }

  void clear() => state = [];

  bool contains(FileHistoryEntity file) => state.contains(file);
}
