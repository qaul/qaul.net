import 'dart:convert';
import 'dart:io' show Directory, File, FileSystemEntity, Platform;
import 'dart:isolate';

import 'package:archive/archive.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter_email_sender/flutter_email_sender.dart';
import 'package:path_provider/path_provider.dart';

import 'logger.dart';

class EmailLogger implements Logger {
  @override
  bool loggingEnabled = Platform.isIOS || Platform.isAndroid;

  @override
  Future<void> initialize() async {
    FlutterError.onError = (details) => logError(details.exception, details.stack!);
    Isolate.current.addErrorListener(RawReceivePort((pair) async {
      final List<dynamic> errorAndStacktrace = pair;
      await logError(errorAndStacktrace.first, errorAndStacktrace.last);
    }).sendPort);
  }

  DateTime get now => DateTime.now();

  Future<String> get _storeDirectory async => (await getApplicationDocumentsDirectory()).path;

  Future<List<FileSystemEntity>> get _logs async =>
      Directory('${await _storeDirectory}/Logs').listSync();

  @override
  Future<bool> get hasLogsStored async => (await _logs).map((e) => e.path).isNotEmpty;

  @override
  Future<void> logError(Object e, StackTrace s, {String? message}) async {
    if (!loggingEnabled) return;
    final log = _buildLogContent(e, s, 'ERROR', message ?? '-');
    final bytes = _createCompressedLog(log);
    if (bytes != null) _storeCompressedLog(bytes, _buildTitle(e));
  }

  @override
  Future<void> logException(Exception e, StackTrace s, {String? message}) async {
    if (!loggingEnabled) return;
    final log = _buildLogContent(e, s, 'EXCEPTION', message ?? '-');
    final bytes = _createCompressedLog(log);
    if (bytes != null) _storeCompressedLog(bytes, _buildTitle(e));
  }

  String get _titlePrefix => 'qaul_log';

  String _buildTitle(Object e) =>
      '$_titlePrefix-${e.toString().trim().replaceAll(' ', '_')}-${now.millisecondsSinceEpoch}';

  String _buildLogContent(Object e, StackTrace stack, String type, String msg) {
    return '''
Log Type: $type

Error/Exception: $e

Message: $msg

Stack Trace:
$stack
''';
  }

  List<int>? _createCompressedLog(String logContent) {
    var stringBytes = utf8.encode(logContent);
    return GZipEncoder().encode(stringBytes);
  }

  Future _storeCompressedLog(List<int> logBytes, String logTitle) async {
    final directory = await _storeDirectory;
    debugPrint('storing log in directory: $directory');
    final file = File('$directory/Logs/$logTitle.gzip');
    file.createSync(recursive: true);
    file.writeAsBytesSync(logBytes);
  }

  @override
  Future<void> sendLogs() async {
    if (!loggingEnabled) return;

    final email = Email(
      body: 'Customer Feedback - Error/Exception Logs',
      subject: 'Customer Feedback - Error/Exception Logs',
      recipients: ['qaul.service@gmail.com'],
      attachmentPaths: (await _logs).map((e) => e.path).toList(),
      isHTML: false,
    );

    await FlutterEmailSender.send(email);
  }

  @override
  Future<void> deleteLogs() async {
    for (var log in (await _logs)) {
      log.deleteSync();
    }
  }
}
