// ignore_for_file: depend_on_referenced_packages
/// Added as a separate package as running tests on root project instantiates a libqaul shell for some reason, which in return fails the tests.
library;

import 'dart:async';

import 'package:flutter/material.dart';
import 'package:intl/intl.dart';
import 'package:intl/locale.dart';
import 'package:timeago/timeago.dart' as timeago;

import 'src/emoji_string_manipulator.dart';

export 'src/bubble.dart';
export 'src/file_size.dart';
export 'src/image_manipulation.dart';
export 'src/intersperse.dart';
export 'src/ip_utils.dart';
export 'src/noise.dart';
export 'src/version.dart';

Color colorGenerationStrategy(String first) {
  // defined using --dart-define=testing_mode=true when running tests
  if (const bool.fromEnvironment('testing_mode', defaultValue: false)) {
    return Colors.green;
  }
  final colors = <Color>[
    ...Colors.primaries.map((e) => e.shade700),
    ...Colors.accents.map((e) => e.shade700),
  ];
  return colors[first.hashCode % colors.length];
}

/// If the string contains an emoji, returns the first emoji grapheme found.
///
/// Otherwise, yields the first letter of the first two words, in uppercase.
/// Single-word names return their first grapheme in uppercase.
///
/// Note: Uses grapheme-aware extraction to avoid malformed UTF-16 issues. See:
/// * https://github.com/dart-lang/sdk/issues/35798
/// * https://github.com/flutter/flutter/issues/52306
/// * https://github.com/flutter/flutter/issues/43302
String initials(String name) {
  assert(name.isNotEmpty, 'name should have at least one character');
  if (name.replaceAll(' ', '').isEmpty) {
    throw ArgumentError.value(
      name,
      'Name',
      'not enough characters to form initials string',
    );
  }

  final emoji = _firstEmojiGrapheme(name);
  if (emoji != null) return emoji;

  final words = name.trim().split(RegExp(r'\s+')).where((e) => e.isNotEmpty).toList();
  if (words.length > 1) {
    return '${words.first.characters.first}${words[1].characters.first}'
        .toUpperCase();
  }
  return words.first.characters.first.toUpperCase();
}

bool isEmojiOnlyGrapheme(String text) {
  if (text.isEmpty) return false;
  final graphemes = text.characters.toList(growable: false);
  return graphemes.length == 1 && _looksLikeEmojiGrapheme(graphemes.first);
}

String? _firstEmojiGrapheme(String text) {
  for (final grapheme in text.characters) {
    if (_looksLikeEmojiGrapheme(grapheme)) return grapheme;
  }
  return null;
}

bool _looksLikeEmojiGrapheme(String grapheme) {
  if (hasEmojis(grapheme)) return true;
  if (grapheme.contains('\u200D') || grapheme.contains('\uFE0F')) return true;
  final rune = grapheme.runes.first;
  return (rune >= 0x1F000 && rune <= 0x1FAFF) ||
      (rune >= 0x2600 && rune <= 0x27BF);
}

/// If [clock] is provided, timestamp is in relation to [clock] (Should only be useful for testing).
///
/// Will throw if [date] is in the future and [allowFutureDate] is true.
/// Otherwise, replaces date with `DateTime.now()` for convenience.
///
/// Defaults [allowFutureDate] to `false`.
String describeFuzzyTimestamp(
  DateTime date, {
  DateTime? clock,
  Locale? locale,
  bool allowFutureDate = false,
}) {
  if (date.isAfter(clock ?? DateTime.now())) {
    if (allowFutureDate) {
      throw ArgumentError.value(date);
    }
    return timeago.format(DateTime.now(), clock: null);
  }
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

/// Encodes a mailto URI according to RFC 6068.
///
/// Creates a properly encoded mailto: URI string from the provided parameters.
///
/// Example:
/// ```dart
/// final uri = encodeMailto(
///   to: ['user@example.com'],
///   subject: 'Hello World',
///   body: 'This is a test email',
/// );
/// // Returns: "mailto:user@example.com?subject=Hello%20World&body=This%20is%20a%20test%20email"
/// ```
///
/// The email address part before the '@' is percent-encoded, while the domain
/// part remains unencoded. All query parameters (subject, body, cc, bcc) are
/// percent-encoded.
String encodeMailto({
  List<String>? to,
  List<String>? cc,
  List<String>? bcc,
  String? subject,
  String? body,
}) {
  String encodeEmail(String e) => Uri.encodeComponent(e).replaceAll('%40', '@');

  var result = 'mailto:';

  if (to != null && to.isNotEmpty) {
    result += to.map(encodeEmail).join(',');
  }

  final queryParts = <String>[];

  if (subject != null && subject.isNotEmpty) {
    queryParts.add('subject=${Uri.encodeComponent(subject)}');
  }
  if (body != null && body.isNotEmpty) {
    queryParts.add('body=${Uri.encodeComponent(body)}');
  }
  if (cc != null && cc.isNotEmpty) {
    queryParts.add('cc=${cc.map(encodeEmail).join(',')}');
  }
  if (bcc != null && bcc.isNotEmpty) {
    queryParts.add('bcc=${bcc.map(encodeEmail).join(',')}');
  }

  if (queryParts.isNotEmpty) {
    result += '?${queryParts.join('&')}';
  }

  return result;
}
