/// Added as a separate package as running tests on root project instantiates a libqaul shell for some reason, which in return fails the tests.
library color_generator;

import 'dart:math';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
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
  if (name.replaceAll(' ', '').length < 2) {
    throw ArgumentError.value(name, 'Name', 'not enough charactes to form initials string');
  }
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
  if ((clock ?? DateTime.now()).subtract(const Duration(days: 50)).isAfter(date)) {
    return DateFormat('d MMM y').format(date);
  }

  var timeSent = timeago
      .format(date, clock: clock)
      .replaceFirst(RegExp('minutes'), 'min.')
      .replaceFirst(RegExp(' ago'), '')
      .trim();

  return timeSent;
}

/// Returns a long, descriptive name for the locale (always in that language).
///
/// If [this] is not mapped, then it will return [this.toLanguageTag()]
String describeLocale(Locale l) {
  if (l.languageCode == 'en') return 'English';
  if (l.languageCode == 'ar') return 'اللغة العربية';
  if (l.languageCode == 'pt') return 'Português';
  return l.toLanguageTag();
}

bool isValidIPv4(String? s) {
  if (s == null || s.isEmpty) return false;
  return RegExp(r'^((25[0-5]|(2[0-4]|1\d|[1-9]|)\d)(\.(?!$)|$)){4}$').hasMatch(s);
}

bool isValidPort(String? s) {
  if (s == null || int.tryParse(s) == null) return false;
  return int.parse(s) >= 0 && int.parse(s) < pow(2, 16);
}

class IPv4TextInputFormatter extends TextInputFormatter {
  FilteringTextInputFormatter get _numberFilter =>
      FilteringTextInputFormatter.allow(RegExp(r'[0-9\.]'));

  @override
  TextEditingValue formatEditUpdate(TextEditingValue oldValue, TextEditingValue newValue) {
    if (newValue.text.isEmpty) return newValue;
    if (_userIsErasing(oldValue, newValue)) return newValue;

    var output = newValue.text;

    if (output.contains(',')) output = output.replaceAll(',', '.');
    if (output.endsWith('..')) {
      return TextEditingValue(
        text: output.substring(0, output.length - 1),
        selection: TextSelection.collapsed(offset: output.length - 1),
      );
    }

    output = _filterInvalidCharacters(oldValue, newValue, output);

    var sections = output.split('.');
    if (sections.length > 4) sections = sections.getRange(0, 4).toList();
    sections.last = _validateLastSection(sections);

    output = sections.join('.');
    if (output.length > 15) output = output.substring(0, 15);

    return TextEditingValue(
      text: output,
      selection: TextSelection.collapsed(offset: output.length),
    );
  }

  bool _userIsErasing(TextEditingValue oldValue, TextEditingValue newValue) =>
      oldValue.text.length > newValue.text.length;

  String _filterInvalidCharacters(
      TextEditingValue oldValue, TextEditingValue newValue, String output) {
    var value = TextEditingValue(text: output, selection: newValue.selection);
    return _numberFilter.formatEditUpdate(oldValue, value).text;
  }

  String _validateLastSection(List<String> sections) {
    var lastSection = sections.last;
    if (lastSection.length > 3) {
      lastSection = lastSection.substring(0, lastSection.length - 1);
    }
    if (lastSection.length == 3 && sections.length < 4) lastSection += '.';
    return lastSection;
  }
}
