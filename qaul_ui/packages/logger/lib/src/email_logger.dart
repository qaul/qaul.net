import 'package:flutter/foundation.dart';
import 'package:logger/src/models/log_event.dart';
import 'package:mailer/mailer.dart';
import 'package:mailer/smtp_server.dart';

import 'logger.dart';

class EmailLogger implements Logger {
  @override
  bool loggingEnabled = false;

  @override
  Future<void> initialize() async {}

  @override
  Future<void> logAppOpen() async {
    // TODO
  }

  @override
  Future<void> logCustomEvent(LogEvent event) async {
    // TODO
  }

  @override
  Future<void> logError(Object error, StackTrace stack, {String? message}) async {
    _sendEmail('Exception: $error', 'EXCEPTION', '$error\n$message\n', stack.toString());
  }

  @override
  Future<void> logException(Exception e, StackTrace stack, {String? message}) async {
    _sendEmail('Exception: $e', 'EXCEPTION', '$e\n$message\n', stack.toString());
  }

  Future _sendEmail(String subject, String title, String reason, String stack) async {
    var email = 'EMAIL';
    final message = Message()
      ..from = Address(email, 'NAME')
      ..recipients = ['OUR EMAIL']
      ..subject = subject
      ..html = '<h1>$title</h1><br><br><p>$reason</p>';

    final smtpServer = gmailSaslXoauth2(userEmail, accessToken);

    try {
      send(message, smtpServer);
    } on MailerException catch (e, stack) {
      debugPrint('Error sending email: $e, $stack');
    }
  }
}
