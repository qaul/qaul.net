import 'dart:isolate';

import 'package:firebase_analytics/firebase_analytics.dart';
import 'package:firebase_core/firebase_core.dart';
import 'package:firebase_crashlytics/firebase_crashlytics.dart';
import 'package:flutter/foundation.dart';

import '../logger.dart';

class FirebaseAnalyticsLogger implements Logger {
  FirebaseAnalyticsLogger({FirebaseAnalytics? analytics})
      : _analytics = analytics ?? FirebaseAnalytics.instance;

  final FirebaseAnalytics _analytics;

  @override
  Future<void> initialize() async {
    await Firebase.initializeApp();
    FlutterError.onError = FirebaseCrashlytics.instance.recordFlutterError;
    Isolate.current.addErrorListener(RawReceivePort((pair) async {
      final List<dynamic> errorAndStacktrace = pair;
      await FirebaseCrashlytics.instance.recordError(
        errorAndStacktrace.first,
        errorAndStacktrace.last,
      );
    }).sendPort);
  }

  @override
  set loggingEnabled(bool enabled) {
    FirebaseCrashlytics.instance.setCrashlyticsCollectionEnabled(enabled);
  }

  @override
  bool get loggingEnabled => FirebaseCrashlytics.instance.isCrashlyticsCollectionEnabled;

  @override
  Future<void> logAppOpen() async {
    try {
      await _analytics.logAppOpen();
    } on Exception catch (e, stack) {
      logException(e, stack, message: 'Exception while logging app open');
    }
  }

  @override
  Future<void> logCustomEvent(LogEvent event) async {
    try {
      await _analytics.logEvent(name: event.name, parameters: event.parameters);
    } on Exception catch (e, stack) {
      logException(
        e,
        stack,
        message:
            'Exception while attempting to log event:\nName: ${event.name}\nParameters: ${event.parameters}',
      );
    }
  }

  @override
  Future<void> logError(Object e, StackTrace stack, {String? message}) async {
    FirebaseCrashlytics.instance
        .recordError(e, stack, reason: 'Event Type: ERROR - Message: ${message ?? 'none'}');
  }

  @override
  Future<void> logException(Exception e, StackTrace stack, {String? message}) async {
    FirebaseCrashlytics.instance
        .recordError(e, stack, reason: 'Event Type: EXCEPTION - Message: ${message ?? 'none'}');
  }
}
