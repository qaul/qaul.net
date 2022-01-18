import 'email_logger.dart';
import 'models/models.dart';

abstract class Logger {
  static final instance = EmailLogger();

  Future<void> initialize();

  set loggingEnabled(bool enabled);

  bool get loggingEnabled;

  Future<void> logAppOpen();

  Future<void> logCustomEvent(LogEvent event);

  Future<void> logException(Exception e, StackTrace stack, {String? message});

  Future<void> logError(Object error, StackTrace stack, {String? message});
}
