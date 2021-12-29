import 'email_logger.dart';

abstract class Logger {
  static final instance = EmailLogger();

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