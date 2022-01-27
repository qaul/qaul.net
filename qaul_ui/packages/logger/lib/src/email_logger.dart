import 'dart:io';
import 'dart:isolate';

import 'package:filesize/filesize.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter_email_sender/flutter_email_sender.dart';
import 'package:logger/src/info_provider.dart';
import 'package:logger/src/storage_manager.dart';
import 'package:mailto/mailto.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:url_launcher/url_launcher.dart';

import 'logger.dart';

class EmailLogger implements Logger {
  @override
  Future<void> initialize() async {
    SharedPreferences prefs = await SharedPreferences.getInstance();
    if (prefs.containsKey(_loggingEnabledKey)) {
      _enabled = prefs.getBool(_loggingEnabledKey) ?? true;
    }

    FlutterError.onError = (details) => logError(details.exception, details.stack!);
    Isolate.current.addErrorListener(RawReceivePort((pair) async {
      final List<dynamic> errorAndStacktrace = pair;
      await logError(errorAndStacktrace.first, errorAndStacktrace.last);
    }).sendPort);
  }

  static const _loggingEnabledKey = 'loggingEnabledKey';

  @override
  bool get loggingEnabled => _enabled;
  bool _enabled = true;

  @override
  set loggingEnabled(bool enabled) {
    _enabled = enabled;
    _storeLoggingOption();
  }

  void _storeLoggingOption() async {
    SharedPreferences prefs = await SharedPreferences.getInstance();
    prefs.setBool(_loggingEnabledKey, _enabled);
  }

  final _storageManager = StorageManager();

  @override
  Future<bool> get hasLogsStored async =>
      (await _storageManager.logs).map((e) => e.path).isNotEmpty;

  @override
  Future<String> get logStorageSize async {
    final size = await _storageManager.logFolderSize;
    return filesize(size);
  }

  @override
  Future<void> logError(Object e, StackTrace s, {String? message}) async {
    if (!loggingEnabled) return;
    final log = await _buildLogContent(e, s, 'ERROR', message ?? '-');
    _storageManager.storeLog(LogEntry(_buildTitle(e), log));
  }

  @override
  Future<void> logException(Exception e, StackTrace s, {String? message}) async {
    if (!loggingEnabled) return;
    final log = await _buildLogContent(e, s, 'EXCEPTION', message ?? '-');
    _storageManager.storeLog(LogEntry(_buildTitle(e), log));
  }

  String _buildTitle(Object e) =>
      '${_storageManager.titlePrefix}-${e.runtimeType.toString().trim().replaceAll(' ', '_')}-${DateTime.now().millisecondsSinceEpoch}';

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

  @override
  Future<void> sendLogs() async {
    if (!loggingEnabled) return;
    (Platform.isAndroid || Platform.isIOS) ? await _sendMobileLogs() : await _sendDesktopLogs();
  }

  Future<void> _sendMobileLogs() async {
    final email = Email(
      body: 'Customer Feedback - Error/Exception Logs',
      subject: 'Customer Feedback - Error/Exception Logs',
      recipients: ['qaul.service@gmail.com'],
      // recipients: ['debug@qaul.net'],
      attachmentPaths: (await _storageManager.logs).map((e) => e.path).toList(),
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
    for (final log in (await _storageManager.logs)) {
      body += _storageManager.logContents(log) + '\n${'#' * 100}\n\n';
    }
    return body;
  }

  @override
  Future<void> deleteLogs() async => _storageManager.deleteLogs(await _storageManager.logs);
}
