/// Added as a separate package as running tests on root project instantiates a libqaul shell for some reason, which in return fails the tests.
library color_generator;

import 'dart:ui';

import 'package:flutter/material.dart';
import 'package:intl/intl.dart';
import 'package:timeago/timeago.dart' as timeago;

Color colorGenerationStrategy(String first) {
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
String initials(String name) {
  if (name.replaceAll(' ', '').length < 2)
    throw ArgumentError.value(
        name, 'Name', 'not enough charactes to form initials string');
  if (name.contains(' ')) {
    final ws = name.split(' ').where((e) => e.isNotEmpty).toList();
    if (ws.length > 1) return '${ws.first[0]}${ws.last[0]}'.toUpperCase();
  }
  return name.substring(0, 2).toUpperCase();
}

/// If [clock] is provided, timestamp is in relation to [clock] (Should only be useful for testing).
///
/// Will throw if [date] is in the future (When [clock] is provided, *future* represents time after it).
String describeFuzzyTimestamp(DateTime date, {DateTime? clock}) {
  if (date.isAfter(clock ?? DateTime.now())) throw ArgumentError.value(date);
  if ((clock ?? DateTime.now())
      .subtract(const Duration(days: 50))
      .isAfter(date)) {
    return DateFormat('d MMM y').format(date);
  }

  var timeSent = timeago
      .format(date, clock: clock)
      .replaceFirst(RegExp('minutes'), 'min.')
      .replaceFirst(RegExp(' ago'), '')
      .trim();

  return timeSent;
}

/// Returns a long, descriptive name for the locale (always in english).
///
/// If [this] is not mapped, then it will return [this.toLanguageTag()]
String describeLocale(Locale l) {
  if (l.languageCode == 'en') return 'English';
  if (l.languageCode == 'ar') return 'Arabic';
  return l.toLanguageTag();
}
