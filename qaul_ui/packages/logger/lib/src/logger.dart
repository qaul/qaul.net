import 'package:flutter/foundation.dart';

import 'email_logger.dart';

abstract class Logger {
  static final instance = kDebugMode ? _VoidLogger() : EmailLogger();

  Future<void> initialize();

  set loggingEnabled(bool enabled);

  bool get loggingEnabled;

  Future<void> logException(Exception e, StackTrace stack, {String? message});

  Future<void> logError(Object error, StackTrace stack, {String? message});

  Future<bool> get hasLogsStored;

  Future<String> get logStorageSize;

  Future<void> sendLogs();

  Future<void> deleteLogs();
}

class _VoidLogger implements Logger {
  @override
  bool loggingEnabled = false;

  @override
  Future<void> deleteLogs() async {}

  @override
  Future<bool> get hasLogsStored async => false;

  @override
  Future<void> initialize() async {}

  @override
  Future<void> logError(Object error, StackTrace stack,
      {String? message}) async {}

  @override
  Future<void> logException(Exception e, StackTrace stack,
      {String? message}) async {}

  @override
  Future<String> get logStorageSize async => '';

  @override
  Future<void> sendLogs() async {}
}