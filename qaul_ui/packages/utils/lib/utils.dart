// ignore_for_file: depend_on_referenced_packages
/// Added as a separate package as running tests on root project instantiates a libqaul shell for some reason, which in return fails the tests.
library color_generator;

import 'dart:async';

import 'package:flutter/material.dart';
import 'package:intl/intl.dart';
import 'package:intl/locale.dart';
import 'package:timeago/timeago.dart' as timeago;

import 'src/emoji_string_manipulator.dart';

export 'src/file_size_descriptor.dart';
export 'src/image_manipulation.dart';
export 'src/ip_utils.dart';

Color colorGenerationStrategy(String first) {
  // defined using --dart-define=testing_mode=true when running tests
  if (const bool.fromEnvironment('testing_mode', defaultValue: false)) {
    return Colors.green;
  }
  final colors = <Color>[
    ...Colors.primaries.map((e) => e.shade700),
    ...Colors.accents.map((e) => e.shade700)
  ];
  return colors[first.hashCode % colors.length];
}

/// Given a string containing values separated by space (" "), yields a string of length 2
/// containing the first letter of the first and last word, respectively, in uppercase.
///
/// If the provided string has no spaces, returns its first two letters - also uppercase.
///
/// Note: Filters out emojis, so as not to cause malformed UTF-16 issues. See more here:
/// * https://github.com/dart-lang/sdk/issues/35798
/// * https://github.com/flutter/flutter/issues/52306
/// * https://github.com/flutter/flutter/issues/43302
String initials(String name) {
  assert(name.isNotEmpty, 'name should have at least one character');
  if (hasEmojis(name)) {
    final emoji = retrieveFirstEmoji(name);
    if (emoji != null) return emoji;
    name = removeEmoji(name);
  }
  if (name.replaceAll(' ', '').isEmpty) {
    throw ArgumentError.value(
        name, 'Name', 'not enough characters to form initials string');
  }
  if (name.contains(' ')) {
    final ws = name.split(' ').where((e) => e.isNotEmpty).toList();
    if (ws.length > 1) return '${ws.first[0]}${ws.last[0]}'.toUpperCase();
  }
  return name.substring(0, 1).toUpperCase();
}

/// If [clock] is provided, timestamp is in relation to [clock] (Should only be useful for testing).
///
/// Will throw if [date] is in the future (When [clock] is provided, *future* represents time after it).
String describeFuzzyTimestamp(DateTime date,
    {DateTime? clock, Locale? locale}) {
  if (date.isAfter(clock ?? DateTime.now())) throw ArgumentError.value(date);
  if ((clock ?? DateTime.now())
      .subtract(const Duration(days: 50))
      .isAfter(date)) {
    return DateFormat('d MMM y').format(date);
  }

  if (locale != null) {
    timeago.setLocaleMessages(locale.languageCode, _fromLocale(locale));
    timeago.setDefaultLocale(locale.languageCode);
  }
  var timeSent = timeago
      .format(date, clock: clock)
      .replaceFirst(RegExp('minutes'), 'min.')
      .replaceFirst(RegExp(' ago'), '')
      .trim();

  return timeSent;
}

/// Helper function to enable dynamic loading of prebuilt lookup messages.
///
/// Defaults to English. Add more cases as necessary.
timeago.LookupMessages _fromLocale(Locale l) {
  switch (l.languageCode) {
    case 'ar':
      return timeago.ArShortMessages();
    case 'id':
      return timeago.IdMessages();
    case 'pt':
      return timeago.PtBrShortMessages();
    case 'uk':
      return timeago.UkMessages();
    case 'en':
    default:
      return timeago.EnShortMessages();
  }
}

/// A substitute to [Timer] that allows cancelling & restarting with no need to
/// re-instantiate the timer. Ticks occur every 10ms.
///
/// Example:
/// ```dart
/// final timer = LoopTimer(const Duration(milliseconds: 500));
/// timer.onTick = () => print('tick');
/// timer.onTimeout = () {
///   print('timeout');
///   timer.cancel();
/// };
/// timer.start();
/// ```
class LoopTimer {
  LoopTimer(Duration duration)
      : _stopwatch = Stopwatch(),
        _loopDuration = duration,
        _zone = Zone.current;

  final Stopwatch _stopwatch;

  final Zone _zone;
  Timer? _zoneTick;
  Timer? _zoneTimeout;

  Duration get tenMs => const Duration(milliseconds: 10);

  Duration get target => _loopDuration;
  final Duration _loopDuration;

  Duration get elapsed => (_loopDuration - _stopwatch.elapsed);

  bool get isRunning => _stopwatch.isRunning;

  void start() {
    _stopwatch.start();
    if (_onTick != null) _scheduleTickCallback();
    if (_onTimeout != null) _scheduleTimeoutCallback();
  }

  void pause() {
    _stopwatch.stop();
    _cancelZoneTimers();
  }

  void cancel() {
    _stopwatch
      ..stop()
      ..reset();
    _cancelZoneTimers();
    _deleteAssignedCallbacks();
  }

  void _cancelZoneTimers() {
    _zoneTick?.cancel();
    _zoneTimeout?.cancel();
  }

  void _deleteAssignedCallbacks() {
    _onTick = null;
    _onTimeout = null;
  }

  void _invoke(VoidCallback? callback) {
    if (callback != null) _zone.run(callback);
  }

  void Function()? _onTimeout;

  set onTimeout(void Function()? callback) {
    _onTimeout = callback == null ? null : _zone.bindCallback(callback);
    if (isRunning) _scheduleTimeoutCallback();
  }

  void _scheduleTimeoutCallback() {
    _zoneTimeout = _zone.createTimer(elapsed, () => _invoke(_onTimeout));
  }

  void Function()? _onTick;

  set onTick(void Function()? callback) {
    _onTick = callback == null ? null : _zone.bindCallback(callback);
    if (isRunning) _scheduleTickCallback();
  }

  void cancelOnTickCallback() {
    _zoneTick?.cancel();
    _onTick = null;
  }

  void _scheduleTickCallback() {
    _zoneTick = _zone.createPeriodicTimer(tenMs, (_) => _invoke(_onTick));
  }
}
