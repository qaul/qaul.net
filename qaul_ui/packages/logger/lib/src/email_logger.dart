import 'dart:convert';
import 'dart:io';
import 'dart:isolate';

import 'package:archive/archive.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter_email_sender/flutter_email_sender.dart';
import 'package:logger/src/info_provider.dart';
import 'package:mailto/mailto.dart';
import 'package:path_provider/path_provider.dart';
import 'package:url_launcher/url_launcher.dart';

import 'logger.dart';

class EmailLogger implements Logger {
  @override
  bool loggingEnabled = true;

  @override
  Future<void> initialize() async {
    FlutterError.onError = (details) => logError(details.exception, details.stack!);
    Isolate.current.addErrorListener(RawReceivePort((pair) async {
      final List<dynamic> errorAndStacktrace = pair;
      await logError(errorAndStacktrace.first, errorAndStacktrace.last);
    }).sendPort);
  }

  DateTime get now => DateTime.now();

  Future<String> get _storeDirectory async {
    final dir = (Platform.isAndroid)
        ? (await getExternalStorageDirectory())!.path
        : (await getApplicationDocumentsDirectory()).path;
    return '$dir/Logs';
  }

  Future<List<FileSystemEntity>> get _logs async {
    var path = Directory(await _storeDirectory);
    return path.listSync().where((element) => element.path.contains(_titlePrefix)).toList();
  }

  @override
  Future<bool> get hasLogsStored async => (await _logs).map((e) => e.path).isNotEmpty;

  @override
  Future<void> logError(Object e, StackTrace s, {String? message}) async {
    if (!loggingEnabled) return;
    final log = await _buildLogContent(e, s, 'ERROR', message ?? '-');
    final bytes = _createCompressedLog(log);
    if (bytes != null) _storeCompressedLog(bytes, _buildTitle(e));
  }

  @override
  Future<void> logException(Exception e, StackTrace s, {String? message}) async {
    if (!loggingEnabled) return;
    final log = await _buildLogContent(e, s, 'EXCEPTION', message ?? '-');
    final bytes = _createCompressedLog(log);
    if (bytes != null) _storeCompressedLog(bytes, _buildTitle(e));
  }

  String get _titlePrefix => 'qaul_log';

  String _buildTitle(Object e) =>
      '$_titlePrefix-${e.runtimeType.toString().trim().replaceAll(' ', '_')}-${now.millisecondsSinceEpoch}';

  Future<String> _buildLogContent(Object e, StackTrace stack, String type, String msg) async {
    return '''
Log Type: $type

Error/Exception: $e

Message: $msg

App Details:
${await InfoProvider.getPackageInfo()}

Device Details:
${await InfoProvider.getDeviceInfo()}

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
    final file = File('$directory/$logTitle.gzip');
    file.createSync(recursive: true);
    file.writeAsBytesSync(logBytes);
  }

  @override
  Future<void> sendLogs() async {
    if (!loggingEnabled) return;
    if (Platform.isAndroid || Platform.isIOS) await _sendMobileLogs();
    await _sendDesktopLogs();
  }

  Future<void> _sendMobileLogs() async {
    final email = Email(
      body: 'Customer Feedback - Error/Exception Logs',
      subject: 'Customer Feedback - Error/Exception Logs',
      recipients: ['qaul.service@gmail.com'],
      // recipients: ['debug@qaul.net'],
      attachmentPaths: (await _logs).map((e) => e.path).toList(),
      isHTML: false,
    );
    await FlutterEmailSender.send(email);
  }

  Future<void> _sendDesktopLogs() async {
    final mailtoLink = Mailto(
      to: ['qaul.service@gmail.com'],
      body: await _buildDesktopEmail(),
      subject: 'Customer Feedback - Error/Exception Logs',
    );
    await launch('$mailtoLink');
  }

  Future<String> _buildDesktopEmail() async {
    var body = 'Customer Feedback - Error/Exception Logs\n\n\n${'#' * 100}\n\n';
    for (final log in (await _logs).map((e) => e.path)) {
      var logContent = GZipDecoder().decodeBytes(File(log).readAsBytesSync());
      body += utf8.decode(logContent) + '\n${'#' * 100}\n\n';
    }
    return body;
  }

  @override
  Future<void> deleteLogs() async {
    for (var log in (await _logs)) {
      log.deleteSync();
    }
  }
}
