import 'dart:convert';
import 'dart:io';

import 'package:archive/archive.dart';
import 'package:flutter/cupertino.dart';
import 'package:neat_periodic_task/neat_periodic_task.dart';
import 'package:path_provider/path_provider.dart';

@immutable
class LogEntry {
  const LogEntry(this.title, this.contents);

  final String title;
  final String contents;
}

class StorageManager {
  StorageManager() {
    // Create a periodic task that prints 'Hello World' every 30s
    _deleteSchedule = NeatPeriodicTaskScheduler(
      interval: const Duration(minutes: 10),
      name: 'delete-obsolete-logs',
      task: _deleteObsoleteLogs,
      timeout: const Duration(minutes: 5),
    );

    _deleteSchedule.start();
  }

  void dispose() async => await _deleteSchedule.stop();

  late NeatPeriodicTaskScheduler _deleteSchedule;

  final _staleLogPeriod = const Duration(days: 14);

  /// String identifier used to later find the application's logs.
  String get titlePrefix => 'qaul_log';

  Future<String> get _storeDirectory async {
    final dir = (Platform.isAndroid)
        ? (await getExternalStorageDirectory())!.path
        : (await getApplicationDocumentsDirectory()).path;
    return '$dir/Logs';
  }

  Future<List<FileSystemEntity>> get logs async {
    var path = Directory(await _storeDirectory);
    return path.listSync().where((file) => file.path.contains(titlePrefix)).toList();
  }

  Future<void> _deleteObsoleteLogs() async {
    final today = DateTime.now();
    final obsoleteLogs = (await logs).where((log) {
      final stat = log.statSync();
      if (stat.type != FileSystemEntityType.file) return false;
      return today.subtract(_staleLogPeriod).isAfter(_lastInteraction(stat));
    });
    await deleteLogs(obsoleteLogs);
  }

  Future<void> deleteLogs(Iterable<FileSystemEntity> logs) async {
    for (final log in logs) {
      await log.delete();
    }
  }

  DateTime _lastInteraction(FileStat stats) {
    var access = stats.accessed;
    var change = stats.changed;
    var modified = stats.modified;
    if (access.isAfter(change) && access.isAfter(modified)) return access;
    if (change.isAfter(access) && change.isAfter(modified)) return change;
    return modified;
  }

  Future<void> storeLog(LogEntry log) async {
    final logBytes = _createCompressedLog(log.contents);
    if (logBytes == null) return;
    _storeCompressedLog(logBytes, log.title);
  }

  String logContents(FileSystemEntity log) {
    var contents = GZipDecoder().decodeBytes(File(log.path).readAsBytesSync());
    return utf8.decode(contents);
  }

  List<int>? _createCompressedLog(String logContent) {
    var stringBytes = utf8.encode(logContent);
    return GZipEncoder().encode(stringBytes);
  }

  Future _storeCompressedLog(List<int> logBytes, String logTitle) async {
    final directory = await _storeDirectory;
    debugPrint('storing log in directory: $directory');
    final file = File('$directory/$logTitle.gzip');
    file.createSync(recursive: true);
    file.writeAsBytesSync(logBytes);
  }
}
